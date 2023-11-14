use image::{ImageBuffer, Rgb};

use crate::filter::kuwahara::KuwaharaFilterOptions;

mod filter;
mod io;

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

fn main() {
    // let path = std::fs::canonicalize("./IMG.JPG").unwrap();
    let path = std::fs::canonicalize("./cropped.png").unwrap();
    let mut img = image::open(path).unwrap();
    let width = img.width();
    let height = img.height();
    println!("image size: ({}, {})", width, height);

    // 一部を切り出して処理してあとで戻す処理をしている。
    // let cropped_img = img.crop_imm(0, 0, 1000, 1000);
    // let mut cropped_img = cropped_img.into_rgb8();
    // let mut img = img.into_rgb8();
    // modify(&mut cropped_img);
    // img.copy_from(&cropped_img, 0, 0).unwrap();

    let img = img.into_rgb8();
    let img = filter::kuwahara::kuwahara_filter(&img, KuwaharaFilterOptions::default());
    img.save("./operated.png").unwrap();
}
