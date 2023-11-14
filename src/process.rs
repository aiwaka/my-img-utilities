use anyhow::Result;
use image::{DynamicImage, GenericImage, ImageBuffer, Rgb};

/// 画像処理フィルタであることを示す。
pub trait FilterProcessor {
    /// ピクセルバッファを受け取り処理後のバッファを返す。
    fn process(&self, buf: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>;
}

/// (x, y)の座標をtop-leftとして(width, height)の大きさの矩形を取り扱い、その部分のみにフィルタを適用する。
/// 適用後の結果をImageBufferとして返す。
pub fn modify_part_of_img<F>(
    img: DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    processor: &F,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>>
where
    F: FilterProcessor,
{
    let cropped = img.crop_imm(x, y, width, height).into_rgb8();
    let processed = processor.process(&cropped);
    let mut img = img.into_rgb8();
    img.copy_from(&processed, x, y).unwrap();
    Ok(img)
}
