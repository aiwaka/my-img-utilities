use anyhow::Result;
use magick_rust::{magick_wand_genesis, MagickWand};
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

/// 指定したパスのファイルをImageMagickを用いてJPGに変換する。変換後のバイナリを返す。
pub(crate) fn convert_to_jpeg_binary(buf: &[u8]) -> Result<Vec<u8>> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    wand.set_compression_quality(90)?;
    wand.read_image_blob(buf)?;
    wand.strip_image()?;
    let res = wand.write_image_blob("jpeg")?;
    Ok(res)
}
