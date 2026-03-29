//! Decoding and Encoding of WebP Images

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
// Enable nightly benchmark functionality if "_benchmarks" feature is enabled.
#![cfg_attr(all(test, feature = "_benchmarks"), feature(test))]
#[cfg(all(test, feature = "_benchmarks"))]
extern crate test;

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use self::decoder::{
    DecodingError, LoopCount, UpsamplingMethod, WebPDecodeOptions, WebPDecoder,
};
pub use self::encoder::{ColorType, EncoderParams, EncodingError, WebPEncoder};

mod alpha_blending;
mod byteorder_ext;
mod decoder;
mod encoder;
mod extended;
mod huffman;
mod loop_filter;
mod lossless;
mod lossless_transform;
mod transform;
mod vp8_arithmetic_decoder;
mod yuv;

pub mod vp8;
