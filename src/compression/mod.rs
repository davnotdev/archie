use super::*;

mod xz_compress;

pub struct Compression;

impl Compression {
    pub fn compress(_: &Config, input: &[u8]) -> Result<Vec<u8>> {
        Ok(xz_compress::compress_xz(input))
    }

    pub fn decompress(_: &Config, input: &[u8]) -> Result<Vec<u8>> {
        Ok(xz_compress::decompress_xz(input))
    }
}
