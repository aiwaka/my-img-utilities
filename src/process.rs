use anyhow::Result;
use image::{GenericImage, ImageBuffer, Rgb};

/// 画像処理フィルタであることを示す。
pub trait FilterProcessor: std::fmt::Debug {
    /// ピクセルバッファを受け取り処理後のバッファを返す。
    fn process(&self, buf: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>;

    fn display(&self) {
        println!("{:?}", &self);
    }
}

pub fn modify_whole_img<F>(
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    processor: &F,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>>
where
    F: FilterProcessor,
{
    let (width, height) = img.dimensions();
    modify_part_of_img(img, 0, 0, width, height, processor)
}

/// (x, y)の座標をtop-leftとして(width, height)の大きさの矩形を取り扱い、その部分のみにフィルタを適用する。
/// 適用後の結果をImageBufferとして返す。
pub fn modify_part_of_img<F>(
    mut img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    processor: &F,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>>
where
    F: FilterProcessor,
{
    let cropped = img.sub_image(x, y, width, height);
    let processed = processor.process(&cropped.to_image());
    img.copy_from(&processed, x, y)?;
    Ok(img)
}
