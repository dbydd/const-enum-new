# const-enum

[![GitHub](https://img.shields.io/static/v1?label=GitHub&message=const-enum&style=for-the-badge&logo=github&logoColor=white&color=informational)](https://github.com/ppmathis/const-enum)
[![Crates.io](https://img.shields.io/crates/v/const-enum?style=for-the-badge&logo=rust&logoColor=white&color=informational)](https://crates.io/crates/const-enum)
[![docs.rs](https://img.shields.io/docsrs/const-enum?style=for-the-badge&label=docs.rs&logo=rust&logoColor=white&color=informational)](https://docs.rs/const-enum)
[![GitHub Actions Status](https://img.shields.io/github/workflow/status/ppmathis/const-enum/CI/main?style=for-the-badge&logo=githubactions&logoColor=white)](https://github.com/ppmathis/const-enum/actions)

This crate providers a procedural derive macro `ConstEnum`, which will provide a `const` implementation of the `From`
trait for converting an `enum` based on their `repr` type.

Unfortunately Rust Stable does not currently contain all required features for implementing this crate.
To use of this library, you must use a recent Rust Nightly release and add the following feature flags to your crate root:

```rust
#![feature(const_trait_impl)]   // always required
```

Here is a simple example of how this library can be used:

```rust
#![feature(const_trait_impl)]

use const_enum::ConstEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(u8)]
enum Test {
    A = 0b010,
    B = 0b100,
    C = 0b001
}

pub fn main() {
    println!("{:?}", Test::from(0b010 as u8));
    println!("{:?}", u8::from(Test::A));
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
