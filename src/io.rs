use anyhow::{ensure, Result};
use image::{ImageBuffer, ImageResult, Rgb};
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use crate::my_magick::convert_to_jpeg_binary;

pub fn read_binary<P>(path: P) -> Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn write_binary<P>(path: P, data: &[u8]) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut file = File::create(path)?;
    let buf = BufReader::new(data)
        .bytes()
        .collect::<io::Result<Vec<u8>>>()?;
    file.write_all(&buf)?;
    file.flush()?;
    Ok(())
}

pub fn change_extention(filepath: &PathBuf, to_ext: &str) -> Result<PathBuf> {
    let mut path = std::fs::canonicalize(filepath).unwrap();

    ensure!(path.is_file(), "the path is not a file.");
    path.set_extension(to_ext);
    Ok(path)
}

/// 受け取ったパスのファイルを読んで画像データとして返す。
/// HEIC形式はimageクレートで読めないため、その場合はMagickが必要となる。
/// TODO: featureとして分離すれば依存するmagickがなくてもコンパイルできそう
pub fn read_image<P: AsRef<Path>>(path: P) -> ImageResult<ImageBuffer<Rgb<u8>, Vec<u8>>>
where
    PathBuf: std::convert::From<P>,
{
    let pathbuf = PathBuf::from(path);
    let ext = pathbuf.extension().unwrap_or_else(|| {
        println!("error: cannot get extension (as format).");
        std::process::exit(1);
    });
    if ext.to_ascii_lowercase() == "heic" {
        let file_buf = read_binary(pathbuf).unwrap();
        let jpeg_binary = convert_to_jpeg_binary(&file_buf).unwrap();
        Ok(
            image::load_from_memory_with_format(&jpeg_binary, image::ImageFormat::Jpeg)?
                .into_rgb8(),
        )
    } else {
        Ok(image::open(pathbuf)?.into_rgb8())
    }
}
