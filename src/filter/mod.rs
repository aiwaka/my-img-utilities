use std::fmt::Display;

use crate::process::{EmptyOption, FilterProcessor};

use self::{
    gaussian::GaussianFilter, grayscale::GrayscaleFilter, kuwahara::KuwaharaFilter,
    mosaic::MosaicFilter, truncate_color::TruncateColorFilter,
};

pub mod gaussian;
pub mod grayscale;
pub mod kuwahara;
pub mod mosaic;
pub mod truncate_color;

pub mod prelude {
    pub use super::gaussian::{GaussianFilter, GaussianFilterOption};
    pub use super::grayscale::GrayscaleFilter;
    pub use super::kuwahara::{KuwaharaFilter, KuwaharaFilterOptions};
    pub use super::mosaic::{MosaicFilter, MosaicFilterOption};
    pub use super::truncate_color::{
        TruncateColorFilter, TruncateColorFilterOption, TruncateComponent,
    };
}

#[derive(Clone, Copy, Debug)]
pub enum AppFilterType {
    Gaussian,
    GrayScale,
    Kuwahara,
    Mosaic,
    Truncate,
}
impl std::fmt::Display for AppFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl AppFilterType {
    pub fn create_vec() -> Vec<Self> {
        vec![
            Self::Gaussian,
            Self::GrayScale,
            Self::Kuwahara,
            Self::Mosaic,
            Self::Truncate,
        ]
    }
}

#[derive(Clone, Debug)]
pub enum AppFilter {
    Gaussian(GaussianFilter),
    GrayScale(GrayscaleFilter),
    Kuwahara(KuwaharaFilter),
    Mosaic(MosaicFilter),
    Truncate(TruncateColorFilter),
}
impl Display for AppFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gaussian(filter) => filter.fmt(f),
            Self::GrayScale(filter) => filter.fmt(f),
            Self::Kuwahara(filter) => filter.fmt(f),
            Self::Mosaic(filter) => filter.fmt(f),
            Self::Truncate(filter) => filter.fmt(f),
        }
    }
}
impl FilterProcessor for AppFilter {
    type OptionsType = EmptyOption;
    fn process(
        &self,
        buf: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        match self {
            Self::Gaussian(filter) => filter.process(buf),
            Self::GrayScale(filter) => filter.process(buf),
            Self::Kuwahara(filter) => filter.process(buf),
            Self::Mosaic(filter) => filter.process(buf),
            Self::Truncate(filter) => filter.process(buf),
        }
    }
    fn get_option(&self) -> Self::OptionsType {
        EmptyOption
    }
}
