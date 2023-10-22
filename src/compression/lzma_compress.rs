use super::*;

pub fn compress_lzma(input: &[u8]) -> Result<Vec<u8>> {
    Ok(lzma::compress(input, 6)?)
}

pub fn decompress_lzma(input: &[u8]) -> Result<Vec<u8>> {
    Ok(lzma::decompress(input)?)
}
