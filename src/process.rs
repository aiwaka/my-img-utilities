use anyhow::Result;
use image::{GenericImage, ImageBuffer, Rgb};

/// FilterProcessorの設定オプションであることを示す。
pub trait FilterProcessorOptions: std::fmt::Debug + std::fmt::Display + Clone + Default {}
#[derive(Default, Clone, Debug)]
pub struct EmptyOption;
impl std::fmt::Display for EmptyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
impl FilterProcessorOptions for EmptyOption {}

/// 画像処理フィルタであることを示す。
pub trait FilterProcessor: std::fmt::Debug + std::fmt::Display {
    type OptionsType: FilterProcessorOptions;
    /// ピクセルバッファを受け取り処理後のバッファを返す。
    fn process(&self, buf: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>;
    fn get_option(&self) -> Self::OptionsType;
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
    let (img_width, img_height) = img.dimensions();
    let (x, y, width, height) = if x > img_width || y > img_height {
        println!("x or y value exceeds the bound of image. process stop.");
        std::process::exit(2);
    } else {
        let width = if x + width > img_width {
            let width = img_width.saturating_sub(x);
            println!(
                "the value of (x + width) exceeds the width of image ({} px). width clamped to {}",
                img_width, width
            );
            width
        } else {
            width
        };
        let height = if y + height > img_height {
            let height = img_height.saturating_sub(y);
            println!(
                "the value of (y + height) exceeds the height of image ({} px). height clamped to {}",
                img_height, height
            );
            height
        } else {
            height
        };
        (x, y, width, height)
    };
    let cropped = img.sub_image(x, y, width, height);
    let processed = processor.process(&cropped.to_image());
    img.copy_from(&processed, x, y)?;
    Ok(img)
}
