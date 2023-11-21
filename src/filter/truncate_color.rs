use std::fmt::Display;

use image::ImageBuffer;

use crate::process::{FilterProcessor, FilterProcessorOptions};

#[derive(Debug, Clone, Copy)]
pub enum TruncateComponent {
    R,
    G,
    B,
}
impl TruncateComponent {
    pub fn vec() -> Vec<Self> {
        vec![Self::R, Self::G, Self::B]
    }
}
impl Display for TruncateComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::R => "Red",
                Self::G => "Green",
                Self::B => "Blue",
            }
        )
    }
}
#[derive(Debug, Clone)]
pub struct TruncateColorFilterOption {
    pub component: TruncateComponent,
}
impl TruncateColorFilterOption {
    pub fn new(component: TruncateComponent) -> Self {
        Self { component }
    }
}
impl Display for TruncateColorFilterOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.component)
    }
}
impl Default for TruncateColorFilterOption {
    fn default() -> Self {
        Self {
            component: TruncateComponent::R,
        }
    }
}
impl FilterProcessorOptions for TruncateColorFilterOption {}
/// RGBのいずれかを0にするフィルタ。
#[derive(Debug, Clone)]
pub struct TruncateColorFilter {
    pub option: TruncateColorFilterOption,
}

impl TruncateColorFilter {
    pub fn new(option: TruncateColorFilterOption) -> Self {
        Self { option }
    }
}
impl Display for TruncateColorFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TruncateColor {}", self.option)
    }
}
impl FilterProcessor for TruncateColorFilter {
    type OptionsType = TruncateColorFilterOption;
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
    fn get_option(&self) -> Self::OptionsType {
        self.option.clone()
    }
}
