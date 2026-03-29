This is a fork of the [image-webp](https://crates.io/crates/image-webp) crate. The git repository is located at https://github.com/image-rs/image-webp.

# ai-image-webp

[![crates.io](https://img.shields.io/crates/v/ai-image-webp.svg)](https://crates.io/crates/ai-image-webp)
[![Documentation](https://docs.rs/ai-image-webp/badge.svg)](https://docs.rs/ai-image-webp)
[![Build Status](https://github.com/atom-planet-embrace/ai-image-webp/workflows/Rust%20CI/badge.svg)](https://github.com/atom-planet-embrace/ai-image-webp/actions)

This crate is an independent implementation of the WebP image format, written so
that the `image` crate can have a pure-Rust WebP backend for both encoding and
decoding.

## Current Status

* **Decoder:** Supports all WebP format features including both lossless and
  lossy compression, alpha channel, and animation. Both the "simple" and
  "extended" formats are handled, and it exposes methods to extract ICC, EXIF,
  and XMP chunks. Decoding speed is generally in the range of **70-100%** of the
  speed of libwebp.

* **Encoder:** This crate only supports lossless encoding. The encoder
  implementation is relatively basic which makes it very fast, but it doesn't
  get as good compression ratios as libwebp can. Nonetheless, it often produces
  smaller files than PNG, even when compared against the slowest/highest
  compression options of PNG encoders.

## Future possibilities

* We continue to be interested in **optimizations** and **bug fixes** and hope
  the bring the decoder closer to parity with libwebp.

* Another potential area is **animation encoding**. Much of the groundwork is in
  place for this, but it will require some additional work to implement.

* We would like to add **lossy encoding** support, but this is a non-trivial
  task and would require a lot of work. If you are interested in helping with
  this, please get in touch!

