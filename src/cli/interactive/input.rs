use inquire::{error::InquireResult, Confirm, CustomType, Select, Text};
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use crate::cli::clap_parser::parser::AppArgs;
use crate::filter::prelude::*;
use crate::filter::{AppFilter, AppFilterType};

use super::autocompleter::FilePathCompleter;

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
impl std::fmt::Display for FilterProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.filter {
            AppFilter::Gaussian(filter) => filter.fmt(f),
            AppFilter::GrayScale(filter) => filter.fmt(f),
            AppFilter::Kuwahara(filter) => filter.fmt(f),
            AppFilter::Mosaic(filter) => filter.fmt(f),
            AppFilter::Truncate(filter) => filter.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct AppParams {
    pub filepath: PathBuf,
    pub output: PathBuf,
    pub processes: Vec<FilterProcess>,
}
impl std::fmt::Display for AppParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filter_str = self
            .processes
            .iter()
            .enumerate()
            .fold("".to_string(), |prev, (idx, next)| {
                format!("{}    {:>2}. {}\n", prev, idx + 1, next)
            });
        write!(
            f,
            "> filepath: {}\n> output  : {}\n> filters :\n{}",
            self.filepath.to_string_lossy(),
            self.output.to_string_lossy(),
            filter_str
        )
    }
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
        let rect_info = CustomType::new(
            "specify x, y of top-left, and width and height (format: x y width height):",
        )
        .with_formatter(&|rect_info: RectInfo| {
            let (x, y, width, height) = rect_info.0;
            format!("x={} y={} width={} height={}", x, y, width, height)
        })
        .with_error_message("Please type a valid number")
        .with_help_message("")
        .with_help_message("if the input exceeds max width of height, it clamped automatically.")
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
