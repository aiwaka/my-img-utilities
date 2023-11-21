use std::fmt::Display;

use image::{GenericImageView, ImageBuffer};
use num_traits::Zero;

use crate::{
    arithmetic::TripleNums,
    process::{FilterProcessor, FilterProcessorOptions},
};

// fn gaussian<T: Float + FloatConst + Copy>(sigma: T, x: T, y: T) -> T {
//     let half = T::from(0.5).unwrap();
//     // 1 / \sqrt{2\pi}.
//     let coeff = half * T::FRAC_2_SQRT_PI() * T::FRAC_1_SQRT_2();
//     (-(x * x + y * y) * half / sigma / sigma).exp() * coeff / sigma
// }
const GAUSSIAN_COEFF: f64 = std::f64::consts::FRAC_1_PI * 0.5;
fn gaussian_dist(sigma: f64, dist: u32) -> f64 {
    let x = dist as f64;
    (-(x * x) * 0.5 / sigma / sigma).exp() * GAUSSIAN_COEFF / sigma / sigma
}

#[derive(Debug, Clone)]
pub struct GaussianFilterOption {
    pub window_size: u32,
    pub sigma: f64,
}
impl GaussianFilterOption {
    pub fn new(window_size: u32, sigma: f64) -> Self {
        Self { window_size, sigma }
    }
}
impl Default for GaussianFilterOption {
    fn default() -> Self {
        Self {
            window_size: 10,
            sigma: 5.0,
        }
    }
}
impl Display for GaussianFilterOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(window_size={}, sigma={})",
            self.window_size, self.sigma
        )
    }
}
impl FilterProcessorOptions for GaussianFilterOption {}

/// ガウスぼかしフィルタ
#[derive(Debug, Clone)]
pub struct GaussianFilter {
    pub option: GaussianFilterOption,
}

impl GaussianFilter {
    pub fn new(option: GaussianFilterOption) -> Self {
        Self { option }
    }
}
impl Display for GaussianFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gaussian {}", self.option)
    }
}
impl FilterProcessor for GaussianFilter {
    type OptionsType = GaussianFilterOption;
    fn process(
        &self,
        buf: &ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let GaussianFilterOption { window_size, sigma } = self.option;
        let (buf_width, buf_height) = buf.dimensions();
        let mut result_buf = buf.clone();
        for j in 0..buf_height {
            for i in 0..buf_width {
                let window_edge_size = 2 * window_size + 1;
                let view_left = i.saturating_sub(window_size);
                let view_top = j.saturating_sub(window_size);
                let view_width = buf_width.min(i + window_size + 1) - view_left;
                let view_height = buf_height.min(j + window_size + 1) - view_top;
                let sub_view = buf.view(view_left, view_top, view_width, view_height);
                let mut coeff_sum = 0f64;
                let mut color_sum = TripleNums::<f64>::zero();
                // 距離ごとにガウス関数の値を計算しておく
                let coeff_array = (0..(window_edge_size))
                    .map(|dist| gaussian_dist(sigma, dist))
                    .collect::<Vec<f64>>();
                // 実際に使われた窓の重み付き平均を取るため都度計算している
                for (x, y, color) in sub_view.pixels() {
                    // (x, y)が(0, 0), (1, 0)...と取得される。
                    // 実際の座標は(view_left + x, view_top + y)になる。
                    let &coeff = coeff_array
                        .get(((view_left + x).abs_diff(i) + (view_top + y).abs_diff(j)) as usize)
                        .unwrap();
                    coeff_sum += coeff;
                    color_sum = color_sum + TripleNums(color.0).to_f64() * coeff;
                }
                result_buf.put_pixel(i, j, image::Rgb::from((color_sum / coeff_sum).to_u8().0));
            }
        }
        result_buf
    }
    fn get_option(&self) -> Self::OptionsType {
        self.option.clone()
    }
}
