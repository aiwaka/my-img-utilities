use image::ImageBuffer;

use crate::process::FilterProcessor;

pub enum TruncateComponent {
    R,
    G,
    B,
}
pub struct TruncateColorFilterOption {
    pub component: TruncateComponent,
}
impl TruncateColorFilterOption {
    pub fn new(component: TruncateComponent) -> Self {
        Self { component }
    }
}
/// RGBのいずれかを0にするフィルタ。
pub struct TruncateColorFilter {
    pub option: TruncateColorFilterOption,
}

impl TruncateColorFilter {
    pub fn new(option: TruncateColorFilterOption) -> Self {
        Self { option }
    }
}
impl FilterProcessor for TruncateColorFilter {
    fn process(
        &self,
        buf: &ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut buf = buf.to_owned();
        let width = buf.width();
        let height = buf.height();
        for x in 0..width {
            for y in 0..height {
                let pixel = buf.get_pixel_mut(x, y);
                let index = match self.option.component {
                    TruncateComponent::R => 0,
                    TruncateComponent::G => 1,
                    TruncateComponent::B => 2,
                };
                pixel.0[index] = 0;
            }
        }
        buf
    }
}
