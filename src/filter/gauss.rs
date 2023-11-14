use image::ImageBuffer;

use crate::process::FilterProcessor;

pub struct GaussianFilterOption {
    pub sigma: f64,
}
impl GaussianFilterOption {
    pub fn new(sigma: f64) -> Self {
        Self { sigma }
    }
}
/// ガウスぼかしフィルタ
pub struct GaussianFilter {
    pub option: GaussianFilterOption,
}

impl GaussianFilter {
    pub fn new(option: GaussianFilterOption) -> Self {
        Self { option }
    }
}
impl FilterProcessor for GaussianFilter {
    fn process(
        &self,
        buf: &ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        buf.to_owned()
        // TODO:
    }
}
