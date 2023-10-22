use xz::read::{XzDecoder, XzEncoder};

pub fn compress_xz(input: &[u8]) -> Vec<u8> {
    XzEncoder::new(input, 9).into_inner().to_vec()
}

pub fn decompress_xz(input: &[u8]) -> Vec<u8> {
    XzDecoder::new(input).into_inner().to_vec()
}
