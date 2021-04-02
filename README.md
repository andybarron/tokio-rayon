# tokio-rayon

_Mix async code with CPU-heavy thread pools using [Tokio][tokio-url] + [Rayon][rayon-url]_

[tokio-url]: https://docs.rs/tokio
[rayon-url]: https://docs.rs/rayon

[![crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![Build status][build-badge]][build-url]
[![Test coverage][coverage-badge]][coverage-url]
[![MIT license][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/tokio-rayon.svg
[crates-url]: https://crates.io/crates/tokio-rayon
[docs-badge]: https://docs.rs/tokio-rayon/badge.svg
[docs-url]: https://docs.rs/tokio-rayon
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/andybarron/tokio-rayon/blob/master/LICENSE
[build-badge]: https://github.com/andybarron/tokio-rayon/actions/workflows/ci.yaml/badge.svg
[build-url]: https://github.com/andybarron/tokio-rayon/actions
[coverage-badge]: https://codecov.io/gh/andybarron/tokio-rayon/branch/main/graph/badge.svg
[coverage-url]: https://codecov.io/gh/andybarron/tokio-rayon

## [API documentation][docs-url]

## TL;DR

Sometimes, you're doing async stuff, and you also need to do CPU-heavy
stuff. This library will help!

```rust
let nft = tokio_rayon::spawn_async(|| {
  do_some_crypto_stuff()
}).await?;

assert_eq!(nft, ExpensiveNft);
```
