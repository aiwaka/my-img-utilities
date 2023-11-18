use anyhow::Result;
use clap::Parser;
use cli::{clap_parser::parser::AppArgs, interactive::input::input_in_console};

use crate::{
    cli::interactive::input::{AppParams, FilterProcess},
    process::modify_part_of_img,
};

mod arithmetic;
mod cli;
mod filter;
mod io;
mod process;

fn main() -> Result<()> {
    let app_args = AppArgs::parse();
    let app_params = match input_in_console(&app_args) {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };
    println!("{:?}", app_params);

    let AppParams {
        filepath,
        output,
        processes,
    } = app_params;
    let mut img = image::open(filepath).unwrap().into_rgb8();
    // let img_width = img.width();
    // let img_height = img.height();

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
    img.save(output).unwrap();
    Ok(())
}
