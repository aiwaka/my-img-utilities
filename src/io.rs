use anyhow::{ensure, Result};
use image::{ImageBuffer, ImageResult, Rgb};
use img_parts::jpeg::Jpeg;
use img_parts::{Bytes, ImageICC};
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

pub struct ImageData {
    pub buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub icc: Option<Bytes>,
}
impl ImageData {
    pub fn new(buffer: ImageBuffer<Rgb<u8>, Vec<u8>>, icc: Option<Bytes>) -> Self {
        Self { buffer, icc }
    }
}

/// 受け取ったパスのファイルを読んで画像データとして返す。
/// HEIC形式はimageクレートで読めないため、その場合はMagickが必要となる。
/// TODO: featureとして分離すれば依存するmagickがなくてもコンパイルできそう
pub fn read_image<P: AsRef<Path>>(path: P) -> ImageResult<ImageData>
where
    PathBuf: std::convert::From<P>,
{
    let pathbuf = PathBuf::from(path);
    let ext = pathbuf.extension().unwrap_or_else(|| {
        println!("error: cannot get extension (as format).");
        std::process::exit(1);
    });
    if ext.to_ascii_lowercase() == "heic" {
        // heicフォーマットはjpegに変換してからImageBufferにデコードする。
        let file_buf = read_binary(pathbuf).unwrap();
        let jpeg_binary = convert_to_jpeg_binary(&file_buf).unwrap();
        let icc = Jpeg::from_bytes(jpeg_binary.clone().into())
            .unwrap()
            .icc_profile();
        let buffer = image::load_from_memory_with_format(&jpeg_binary, image::ImageFormat::Jpeg)?
            .into_rgb8();
        Ok(ImageData::new(buffer, icc))
    } else {
        let dyn_image = image::open(pathbuf)?;
        let icc = Jpeg::from_bytes(dyn_image.as_bytes().to_owned().into())
            .unwrap()
            .icc_profile();
        let buffer = dyn_image.into_rgb8();
        Ok(ImageData::new(buffer, icc))
    }
}
