use super::*;

mod lzma_compress;

pub struct Compression;

impl Compression {
    pub fn compress(_: &Config, input: &[u8]) -> Result<Vec<u8>> {
        lzma_compress::compress_lzma(input)
    }

    pub fn decompress(_: &Config, input: &[u8]) -> Result<Vec<u8>> {
        lzma_compress::decompress_lzma(input)
    }
}
