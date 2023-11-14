use anyhow::Result;
use image::DynamicImage;

use crate::{
    filter::{
        kuwahara::{KuwaharaFilter, KuwaharaFilterOptions},
        truncate_color::{TruncateColorFilter, TruncateColorFilterOption},
    },
    process::modify_part_of_img,
};

mod arithmetic;
mod filter;
mod io;
mod process;

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
    let img = modify_part_of_img(img, 1000, 1000, 1000, 1000, &processor)?;

    // let truncate_option =
    //     TruncateColorFilterOption::new(filter::truncate_color::TruncateComponent::R);
    // let r_processor = TruncateColorFilter::new(truncate_option);
    // let truncate_option =
    //     TruncateColorFilterOption::new(filter::truncate_color::TruncateComponent::G);
    // let g_processor = TruncateColorFilter::new(truncate_option);
    // let img = DynamicImage::from(modify_part_of_img(
    //     img,
    //     1000,
    //     1000,
    //     1000,
    //     1000,
    //     &r_processor,
    // )?);
    // let img = modify_part_of_img(img, 1500, 1800, 1000, 1000, &g_processor)?;

    img.save("./operated.png").unwrap();
    Ok(())
}
