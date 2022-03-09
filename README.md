# iunorm

[![Crate](https://img.shields.io/crates/v/iunorm.svg)](https://crates.io/crates/iunorm)
[![API](https://docs.rs/iunorm/badge.svg)](https://docs.rs/iunorm/)
[![Lines Of Code](https://tokei.rs/b1/github/joseluis/iunorm?category=code)](https://github.com/joseluis/iunorm)

This library defines multiple types of sizes ranging between 8 and 128-bit,
that can represent a normalized value (between 0 and 1 if unsigned, or -1
and 1 if signed), mapping the range to its `::MIN` & `::MAX` boundaries.

They can be created from f32 & f64.
