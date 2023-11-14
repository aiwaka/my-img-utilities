use image::{GenericImage, GenericImageView, ImageBuffer, Rgb};
use itertools::iproduct;
use num_traits::Zero;

use crate::{arithmetic::TripleNums, process::FilterProcessor};

#[derive(Debug, Clone)]
pub struct KuwaharaFilterOptions {
    /// Kuwahara filterの平均化する近傍窓サイズ。デフォルトは3。
    pub window_size: u32,
}
impl Default for KuwaharaFilterOptions {
    fn default() -> Self {
        Self { window_size: 3 }
    }
}

pub struct KuwaharaFilter {
    pub option: KuwaharaFilterOptions,
}
impl KuwaharaFilter {
    pub fn new(option: KuwaharaFilterOptions) -> Self {
        Self { option }
    }
}
impl FilterProcessor for KuwaharaFilter {
    fn process(&self, buf: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let window_size = self.option.window_size;
        let (buf_width, buf_height) = buf.dimensions();
        let mut buf = buf.clone();
        let mut result_buf = buf.clone();
        for i in 0..buf_width {
            for j in 0..buf_height {
                // 分散の和とRGB平均を4近傍ごとに保存する。 index: [x][y]
                let mut var_sum_array = [[0f64; 2]; 2];
                let mut mean_rgb_array = [[[0u8; 3]; 2]; 2];
                // 4近傍の端にあたるピクセル番号を計算する（3点の直積で4近傍を表現できる）
                let neighbour_edge_x = [
                    (i + 1).saturating_sub(window_size),
                    i,
                    buf_width.min(i + window_size) - 1,
                ];
                let neighbour_edge_y = [
                    (j + 1).saturating_sub(window_size),
                    j,
                    buf_height.min(j + window_size) - 1,
                ];

                for (block_x, block_y) in iproduct!(0..=1, 0..=1) {
                    // 各近傍について演算
                    let block_min_x = neighbour_edge_x[block_x];
                    let block_min_y = neighbour_edge_y[block_y];
                    let block_max_x = neighbour_edge_x[block_x + 1];
                    let block_max_y = neighbour_edge_y[block_y + 1];
                    let sub_width = block_max_x + 1 - block_min_x;
                    let sub_height = block_max_y + 1 - block_min_y;

                    let sub_image = buf.sub_image(block_min_x, block_min_y, sub_width, sub_height);
                    let (sum, double_sum) = sub_image
                        .pixels()
                        .map(|(_, _, color)| TripleNums(color.0).to_f64())
                        .fold(
                            (TripleNums::<f64>::zero(), TripleNums::<f64>::zero()),
                            |(prev_single, prev_double), curr| {
                                (prev_single + curr, prev_double + curr * curr)
                            },
                        );
                    let pix_num_f = sub_image.pixels().count() as f64;
                    let variances = double_sum / pix_num_f - sum * sum / pix_num_f / pix_num_f;
                    var_sum_array[block_x][block_y] = variances.iter().sum::<f64>();
                    // RGB平均は後で選べるように保存しておく
                    mean_rgb_array[block_x][block_y] = (sum / pix_num_f).to_u8().0;
                }

                // 各ブロックの値を比較して最も小さい領域の平均RGBをとる。
                let (min_block_index_x, min_block_index_y, _) = iproduct!(0..=1, 0..=1).fold(
                    (0, 0, f64::MAX),
                    |(prev_x, prev_y, prev_val), (block_x, block_y)| {
                        if var_sum_array[block_x][block_y] < prev_val {
                            (block_x, block_y, var_sum_array[block_x][block_y])
                        } else {
                            (prev_x, prev_y, prev_val)
                        }
                    },
                );
                result_buf.put_pixel(
                    i,
                    j,
                    Rgb::<u8>(mean_rgb_array[min_block_index_x][min_block_index_y]),
                );
            }
        }
        result_buf
    }
}
