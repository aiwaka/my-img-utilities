use std::fmt::Display;

use image::{DynamicImage, ImageBuffer};

use crate::process::{EmptyOption, FilterProcessor};

/// グレイスケールにするフィルタ
#[derive(Debug, Clone)]
pub struct GrayscaleFilter;

impl GrayscaleFilter {
    pub fn new() -> Self {
        Self
    }
}
impl Display for GrayscaleFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grayscale")
    }
}
impl FilterProcessor for GrayscaleFilter {
    type OptionsType = EmptyOption;
    fn process(
        &self,
        buf: &ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let image = DynamicImage::from(buf.clone());
        let gray = image.grayscale();
        gray.into_rgb8()
        // TODO:
    }
    fn get_option(&self) -> Self::OptionsType {
        EmptyOption
    }
}
