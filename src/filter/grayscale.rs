use image::{DynamicImage, ImageBuffer};

use crate::process::FilterProcessor;

/// グレイスケールにするフィルタ
#[derive(Debug, Clone)]
pub struct GrayscaleFilter;

impl GrayscaleFilter {
    pub fn new() -> Self {
        Self
    }
}
impl FilterProcessor for GrayscaleFilter {
    fn process(
        &self,
        buf: &ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let image = DynamicImage::from(buf.clone());
        let gray = image.grayscale();
        gray.into_rgb8()
        // TODO:
    }
}
