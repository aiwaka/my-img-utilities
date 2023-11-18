use inquire::autocompletion::Replacement;
use inquire::{error::InquireResult, CustomType, Select, Text};
use inquire::{Autocomplete, Confirm, CustomUserError};
use std::fmt::Display;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::str::FromStr;

use crate::cli::clap_parser::parser::AppArgs;
use crate::filter::prelude::*;
use crate::filter::{AppFilter, AppFilterType};

#[derive(Debug, Clone)]
pub struct FilterProcess {
    pub filter: AppFilter,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
impl FilterProcess {
    pub fn new(filter: AppFilter, rect_info: RectInfo) -> Self {
        let rect = rect_info.0;
        FilterProcess {
            filter,
            x: rect.0,
            y: rect.1,
            width: rect.2,
            height: rect.3,
        }
    }
}

#[derive(Debug)]
pub struct AppParams {
    pub filepath: PathBuf,
    pub output: PathBuf,
    pub processes: Vec<FilterProcess>,
}

/// 矩形情報を表す。(x, y, width, height)で、(x, y)は矩形のtop-leftを配置する。
#[derive(Debug, Clone, Copy)]
pub struct RectInfo(pub (u32, u32, u32, u32));
impl FromStr for RectInfo {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace();
        let mut vec = Vec::<u32>::new();
        for s in split.into_iter() {
            match s.parse::<u32>() {
                Ok(value) => vec.push(value),
                Err(err) => return Err(format!("{:?}", err)),
            }
        }
        if vec.len() == 4 {
            Ok(RectInfo((vec[0], vec[1], vec[2], vec[3])))
        } else {
            Err(String::from("the number of numbers must be 4."))
        }
    }
}
impl Display for RectInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.0;
        write!(f, "{} {} {} {}", value.0, value.1, value.2, value.3)
    }
}

fn simple_param_input<T: FromStr>(message: &str, default: &str) -> InquireResult<T>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    loop {
        match Text::new(message)
            .with_default(default)
            .prompt()?
            .parse::<T>()
        {
            Ok(sigma) => break Ok(sigma),
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    }
}

pub fn input_in_console(app_args: &AppArgs) -> InquireResult<AppParams> {
    // コマンドライン引数に存在しない場合はプロンプトを用いて決定させる。
    let filepath = match &app_args.filepath {
        Some(filepath) => {
            let canononicalized = fs::canonicalize(filepath).unwrap();
            println!("filepath: {}", canononicalized.to_str().unwrap());
            canononicalized
        }
        None => loop {
            let filepath = Text::new("file path:")
                .with_autocomplete(FilePathCompleter::default())
                .prompt()
                .unwrap_or_else(|err| {
                    println!("{}", err);
                    std::process::exit(1)
                });
            let pathbuf = PathBuf::from(filepath);
            if pathbuf.exists() {
                break fs::canonicalize(pathbuf).unwrap();
            } else {
                println!("path does not exist.");
            }
        },
    };
    let output = match &app_args.output {
        Some(output) => {
            let canononicalized = fs::canonicalize(output).unwrap();
            println!("filepath: {}", canononicalized.to_str().unwrap());
            canononicalized
        }
        None => {
            let default_path = {
                let file_stem = filepath.file_stem().unwrap();
                format!("./{}_filtered.jpg", file_stem.to_str().unwrap())
            };
            let filepath = Text::new("output path:")
                .with_default(&default_path)
                .prompt()
                .unwrap_or_else(|err| {
                    println!("{}", err);
                    std::process::exit(1)
                });
            PathBuf::from(filepath)
        }
    };
    let mut processes = Vec::<FilterProcess>::new();
    let filter_vec = AppFilterType::create_vec();
    loop {
        let filter_type = Select::new("filter type:", filter_vec.clone()).prompt()?;
        let rect_info = CustomType::new("specify x, y of top-left, and width and height:")
            .with_formatter(&|rect_info: RectInfo| {
                let (x, y, width, height) = rect_info.0;
                format!("x={} y={} width={} height={}", x, y, width, height)
            })
            .with_error_message("Please type a valid number")
            .with_help_message("format: x y width height")
            .prompt()?;
        // TODO: 数値をmatch arm内で入力させる
        let filter = match filter_type {
            AppFilterType::Gaussian => {
                let window_size = simple_param_input("input window size (positive integer)", "10")?;
                let sigma = simple_param_input("input parameter sigma (float)", "4.0")?;

                AppFilter::Gaussian(GaussianFilter::new(GaussianFilterOption::new(
                    window_size,
                    sigma,
                )))
            }

            AppFilterType::GrayScale => AppFilter::GrayScale(GrayscaleFilter::new()),
            AppFilterType::Kuwahara => {
                let window_size = simple_param_input("input window size (positive integer)", "3")?;
                AppFilter::Kuwahara(KuwaharaFilter::new(KuwaharaFilterOptions::new(window_size)))
            }
            AppFilterType::Mosaic => {
                let size = simple_param_input("input window size (positive integer)", "50")?;
                AppFilter::Mosaic(MosaicFilter::new(MosaicFilterOption::new(size)))
            }
            AppFilterType::Truncate => {
                let component =
                    Select::new("select rgb component to truncate", TruncateComponent::vec())
                        .prompt()?;

                AppFilter::Truncate(TruncateColorFilter::new(TruncateColorFilterOption::new(
                    component,
                )))
            }
        };
        processes.push(FilterProcess::new(filter, rect_info));

        match Confirm::new("add another filter ?")
            .with_default(false)
            .prompt()
        {
            Ok(true) => continue,
            Ok(false) => break,
            Err(err) => {
                println!("{}", err);
                std::process::exit(1);
            }
        }
    }

    let app_params = AppParams {
        filepath,
        output,
        processes,
    };
    Ok(app_params)
}

/// [https://docs.rs/inquire/0.6.2/src/complex_autocompletion/complex_autocompletion.rs.html]より.
#[derive(Clone, Default)]
pub struct FilePathCompleter {
    input: String,
    paths: Vec<String>,
    lcp: String,
}

impl FilePathCompleter {
    fn update_input(&mut self, input: &str) -> Result<(), CustomUserError> {
        if input == self.input {
            return Ok(());
        }

        self.input = input.to_owned();
        self.paths.clear();

        let input_path = std::path::PathBuf::from(input);

        let fallback_parent = input_path
            .parent()
            .map(|p| {
                if p.to_string_lossy() == "" {
                    std::path::PathBuf::from(".")
                } else {
                    p.to_owned()
                }
            })
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        let scan_dir = if input.ends_with('/') {
            input_path
        } else {
            fallback_parent.clone()
        };

        let entries = match std::fs::read_dir(scan_dir) {
            Ok(read_dir) => Ok(read_dir),
            Err(err) if err.kind() == ErrorKind::NotFound => std::fs::read_dir(fallback_parent),
            Err(err) => Err(err),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        let mut idx = 0;
        let limit = 15;

        while idx < entries.len() && self.paths.len() < limit {
            let entry = entries.get(idx).unwrap();

            let path = entry.path();
            let path_str = if path.is_dir() {
                format!("{}/", path.to_string_lossy())
            } else {
                path.to_string_lossy().to_string()
            };

            if path_str.starts_with(&self.input) && path_str.len() != self.input.len() {
                self.paths.push(path_str);
            }

            idx = idx.saturating_add(1);
        }

        self.lcp = self.longest_common_prefix();

        Ok(())
    }

    fn longest_common_prefix(&self) -> String {
        let mut ret: String = String::new();

        let mut sorted = self.paths.clone();
        sorted.sort();
        if sorted.is_empty() {
            return ret;
        }

        let mut first_word = sorted.first().unwrap().chars();
        let mut last_word = sorted.last().unwrap().chars();

        loop {
            match (first_word.next(), last_word.next()) {
                (Some(c1), Some(c2)) if c1 == c2 => {
                    ret.push(c1);
                }
                _ => return ret,
            }
        }
    }
}

impl Autocomplete for FilePathCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update_input(input)?;

        Ok(self.paths.clone())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        self.update_input(input)?;

        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => match self.lcp.is_empty() {
                true => Replacement::None,
                false => Replacement::Some(self.lcp.clone()),
            },
        })
    }
}
