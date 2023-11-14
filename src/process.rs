use anyhow::Result;
use image::{DynamicImage, GenericImage, ImageBuffer, Rgb};

/// (x, y)の座標をtop-leftとして(width, height)の大きさの矩形を取り扱い、任意の変更を行う。
/// 結果をImageBufferとして返す。
pub fn modify_part_of_img<F>(
    img: DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    processor: F,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>>
where
    F: FnOnce(&ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>,
{
    let cropped = img.crop_imm(x, y, width, height).into_rgb8();
    let processed = processor(&cropped);
    let mut img = img.into_rgb8();
    img.copy_from(&processed, x, y).unwrap();
    Ok(img)
}
