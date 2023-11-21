use std::fmt::Display;

use image::ImageBuffer;

use crate::process::{FilterProcessor, FilterProcessorOptions};

#[derive(Debug, Clone)]
pub struct MosaicFilterOption {
    pub size: usize,
}
impl MosaicFilterOption {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}
impl Default for MosaicFilterOption {
    fn default() -> Self {
        Self { size: 50 }
    }
}
impl Display for MosaicFilterOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(size={})", self.size)
    }
}
impl FilterProcessorOptions for MosaicFilterOption {}
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
impl Display for MosaicFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mosaic {}", self.option)
    }
}
impl FilterProcessor for MosaicFilter {
    type OptionsType = MosaicFilterOption;
    fn process(
        &self,
        buf: &ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let MosaicFilterOption { size } = self.option;
        let (buf_width, buf_height) = buf.dimensions();
        let mut result_buf = buf.clone();
        for block_y in (0..buf_height).collect::<Vec<u32>>().chunks(size) {
            for block_x in (0..buf_width).collect::<Vec<u32>>().chunks(size) {
                let &block_pix = buf.get_pixel(block_x[0], block_y[0]);
                for &y in block_y {
                    for &x in block_x {
                        result_buf.put_pixel(x, y, block_pix);
                    }
                }
            }
        }
        result_buf
    }
    fn get_option(&self) -> Self::OptionsType {
        self.option.clone()
    }
}
