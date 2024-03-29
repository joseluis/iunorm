# iunorm

[![Crate](https://img.shields.io/crates/v/iunorm.svg)](https://crates.io/crates/iunorm)
[![API](https://docs.rs/iunorm/badge.svg)](https://docs.rs/iunorm/)
[![MSRV: 1.43.1](https://flat.badgen.net/badge/MSRV/1.43.1/purple)](https://releases.rs/docs/1.43.1/)

This library facilitates mapping between floating-point numbers, normalized
to some range, and (un)signed integers.

- `Inorm` types are a new-type over **signed** integers that *by default*
map to floating-point range of `-1..1`.
- `Unorm` types are a new-type over **unsigned** integers that *by default*
map to floating-point range of `0..1`.

The *by default* mapping is used by the `From`/`Into` traits and the
`from_f32`/`to_f32` methods, and the equivalent `f64` versions.

Additionally both `Inorm` & `Unorm` types can map from/to any custom
floating-point range by using the `from_f32_minmax`/`to_f32_minmax` methods,
and the equivalent `f64` versions.

## Status

This library is considered stable.

## License
This project is dual licensed under either [MIT](LICENSE-MIT)
or [Apache-2.0](LICENSE-APACHE) at your option.
