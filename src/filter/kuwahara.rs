use image::{ImageBuffer, Rgb};
use itertools::iproduct;

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

fn variance(vec: &[u8]) -> f64 {
    let n = vec.len() as f64;
    let mean = vec.iter().map(|&x| x as f64).sum::<f64>() / n;
    vec.iter().map(|&x| x as f64 * x as f64).sum::<f64>() / n - mean * mean
}

/// ピクセルの参照を受け取ってその領域のみで計算する。
pub fn kuwahara_filter(
    buf: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    options: KuwaharaFilterOptions,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let window_size = options.window_size;
    let (buf_width, buf_height) = buf.dimensions();
    let mut result_buf = buf.clone();
    for i in 0..buf_width {
        for j in 0..buf_height {
            // 分散の和を4近傍ごとに保存する。 index: [row][column]
            let mut var_sum_array = [[0f64; 2]; 2];
            // 近傍を示すインデックスの区間を作成する
            let neighbour_indices_x = [
                (0.max(i as i32 + 1 - window_size as i32) as u32)..(i + 1),
                i..(buf_width.min(i + window_size)),
            ];
            let neighbour_indices_y = [
                (0.max(j as i32 + 1 - window_size as i32) as u32)..(j + 1),
                j..(buf_height.min(j + window_size)),
            ];
            for (block_x, block_y) in iproduct!(0..=1, 0..=1) {
                // 各近傍について演算
                // 近傍ブロック内のピクセルをすべて保存するため、矩形の範囲を計算する
                // 各ピクセルのRGBの値をそれぞれベクトルで保存する. index: [rgb][pix]
                let mut pixel_colors_rgb: [Vec<u8>; 3] = [vec![], vec![], vec![]];
                for i_in_block in neighbour_indices_x[block_x].clone() {
                    for j_in_block in neighbour_indices_y[block_y].clone() {
                        (0usize..3).for_each(|index| {
                            pixel_colors_rgb[index]
                                .push(buf.get_pixel(i_in_block, j_in_block).0[index])
                        });
                    }
                }
                // RGBごとに全ピクセルの分散を計算して足し合わせる
                for pixel_colors in pixel_colors_rgb.iter() {
                    var_sum_array[block_x][block_y] += variance(pixel_colors);
                }
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
            let mut pixel_colors_rgb: [Vec<u8>; 3] = [vec![], vec![], vec![]];
            for i_in_block in neighbour_indices_x[min_block_index_x].clone() {
                for j_in_block in neighbour_indices_y[min_block_index_y].clone() {
                    (0usize..3).for_each(|index| {
                        pixel_colors_rgb[index].push(buf.get_pixel(i_in_block, j_in_block).0[index])
                    });
                }
            }
            let mut result_rgb = [0u8; 3];
            for (rgb_index, pixel_colors) in pixel_colors_rgb.iter().enumerate() {
                result_rgb[rgb_index] = (pixel_colors.iter().map(|&x| x as f64).sum::<f64>()
                    / pixel_colors.len() as f64) as u8;
            }
            result_buf.put_pixel(i, j, Rgb::<u8>(result_rgb));
        }
    }
    result_buf
}
