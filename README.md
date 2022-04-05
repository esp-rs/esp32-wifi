# esp32-wifi

## NOTE: Superceded by [esp-wifi](https://github.com/esp-rs/esp-wifi).

A experimental wifi driver for the [esp32](https://en.wikipedia.org/wiki/ESP32) written in Rust.

Contributions are welcome :)

Join in on the discussion: https://matrix.to/#/#esp-rs:matrix.org!

## Building

This crate uses the esp-idf binary blobs for wifi functionality. The C foreign function interface is generated with bindgen. This is done using the generate/bindgen.sh script, but only needs to be done again when the version of the binary blobs is changed.

The generate/bindgen.sh can be called from the root directory to create the various files in 
src/binary. The version of esp-idf used should match the version of the binary blobs in the 
esp32-wifi-lib sub-repository. (Currently version v4.1 of the esp-idf is used.)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
