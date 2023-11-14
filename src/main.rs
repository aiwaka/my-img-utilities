use anyhow::Result;
use image::{ImageBuffer, Rgb};

use crate::{
    filter::kuwahara::{KuwaharaFilter, KuwaharaFilterOptions},
    process::modify_part_of_img,
};

mod filter;
mod io;
mod process;

fn modify(buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let width = buf.width();
    let height = buf.height();
    for y in 0..height {
        for x in 0..width {
            let pixel = buf.get_pixel_mut(x, y);
            pixel.0[0] = 0;
        }
    }
}

fn main() -> Result<()> {
    let path = std::fs::canonicalize("./IMG_6388.jpg").unwrap();
    let path_str = path.to_str().unwrap().to_owned();
    let img = image::open(path).unwrap();
    let img_width = img.width();
    let img_height = img.height();
    println!(
        "path: {}\nimage size: ({}, {})",
        path_str, img_width, img_height
    );

    let kuwahara_config = KuwaharaFilterOptions { window_size: 15 };
    let processor = KuwaharaFilter::new(kuwahara_config);
    // 一部を切り出して処理してあとで戻す処理をしている。
    let img = modify_part_of_img(img, 1000, 1000, 1000, 1000, &processor)?;

    img.save("./operated.png").unwrap();
    Ok(())
}
