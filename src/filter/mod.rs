pub mod gaussian;
pub mod grayscale;
pub mod kuwahara;
pub mod mosaic;
pub mod truncate_color;

#[derive(Clone, Debug)]
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
