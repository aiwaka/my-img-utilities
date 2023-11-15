use image::ImageBuffer;

use crate::process::FilterProcessor;

#[derive(Debug, Clone)]
pub struct MosaicFilterOption {
    pub size: u32,
}
impl MosaicFilterOption {
    pub fn new(size: u32) -> Self {
        Self { size }
    }
}
/// モザイクフィルタ
#[derive(Debug, Clone)]
pub struct MosaicFilter {
    pub option: MosaicFilterOption,
}

impl MosaicFilter {
    pub fn new(option: MosaicFilterOption) -> Self {
        Self { option }
    }
}
impl FilterProcessor for MosaicFilter {
    fn process(
        &self,
        buf: &ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        buf.to_owned()
        // TODO:
    }
}
