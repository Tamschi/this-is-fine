# this-is-fine

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/this-is-fine)
[![Crates.io](https://img.shields.io/crates/v/this-is-fine)](https://crates.io/crates/this-is-fine)
[![Docs.rs](https://docs.rs/this-is-fine/badge.svg)](https://docs.rs/this-is-fine)

![Rust 1.51](https://img.shields.io/static/v1?logo=Rust&label=&message=1.51&color=grey)
[![CI](https://github.com/Tamschi/this-is-fine/workflows/CI/badge.svg?branch=develop)](https://github.com/Tamschi/this-is-fine/actions?query=workflow%3ACI+branch%3Adevelop)
![Crates.io - License](https://img.shields.io/crates/l/this-is-fine/0.0.1)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/this-is-fine)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/this-is-fine)](https://github.com/Tamschi/this-is-fine/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/this-is-fine)](https://github.com/Tamschi/this-is-fine/pulls)
[![good first issues](https://img.shields.io/github/issues-raw/Tamschi/this-is-fine/good%20first%20issue?label=good+first+issues)](https://github.com/Tamschi/this-is-fine/contribute)

[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/this-is-fine.svg)](https://web.crev.dev/rust-reviews/crate/this-is-fine/)

☕🐕

Utilities for working with `type Fine<T, E> = (T, Result<(), E>)`.

It's a useful return type for map and set insertions.

The API is mostly a port of `Result<T, E>`'s.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add this-is-fine
```

## Example

This is a slightly incomplete API preview.

```rust
use this_is_fine::{Fine, prelude::*};

let fine: Fine<_, ()> = ((), Ok(()));

fine.fine(); // Discard any error.
fine.not_fine(); // Convert to `Result<T, E>`.
fine.into_result(); // Convert to `Result<T, (T, E)>`.
fine.ok(); // `.not_fine().ok()`
fine.err(); // `.1.err()`
fine.is_ok(); // `.1.is_ok()`
fine.is_err(); // `.1.is_err()`
fine.as_ref(); // Like `Result::as_ref`.
fine.map(|t| t); // Like `Result::map`.
fine.map_err(|e| e); // Like `Result::map_err`.
fine.expect("message"); // Like `.fine`, but can panic.
fine.unwrap(); // "

let mut fine = fine;
fine.as_mut(); // Like `Result::as_mut`.

// `Some` error is still fine.
let fine = this_is_fine::from_inverse(((), Some(())));
fine.expect_err("message");
fine.unwrap_err();
fine.fine();

// Twice as fine.
let fine_fine: Fine<Fine<_, ()>, ()> = (((), Ok(())), Ok(()));
fine_fine.transpose(); // Exchange `Result`s.
```

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING](CONTRIBUTING.md) for more information.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`this-is-fine` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

- The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
- The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
