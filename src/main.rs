use std::fs::File;

use anyhow::Result;
use clap::Parser;
use cli::{clap_parser::parser::AppArgs, interactive::input::input_on_console};
use image::codecs::jpeg::JpegEncoder;
use img_parts::{jpeg::Jpeg, ImageICC};

use crate::{
    cli::interactive::input::{AppParams, FilterProcess},
    io::{read_image, ImageData},
    process::modify_part_of_img,
};

mod arithmetic;
mod cli;
mod filter;
mod io;
mod my_magick;
mod process;

fn main() -> Result<()> {
    let app_args = AppArgs::parse();
    let app_params = match input_on_console(&app_args) {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };
    println!("applying following filters");
    println!("{}", app_params);

    let AppParams {
        filepath,
        output,
        processes,
    } = app_params;
    let ImageData {
        buffer: mut img,
        icc,
    } = read_image(&filepath)?;

    // フィルタをピクセル列に繰り返し適用
    for filter_process in processes.iter() {
        let FilterProcess {
            filter,
            x,
            y,
            width,
            height,
        } = filter_process;
        img = modify_part_of_img(img, *x, *y, *width, *height, filter).unwrap();
    }
    // icc profileを引き継ぎながらファイルに書き出す
    let mut jpeg_buf = Vec::<u8>::new();
    img.write_with_encoder(JpegEncoder::new_with_quality(&mut jpeg_buf, 85))?;
    let mut jpeg = Jpeg::from_bytes(jpeg_buf.into())?;
    jpeg.set_icc_profile(icc);
    jpeg.encoder().write_to(File::create(output)?)?;
    Ok(())
}
