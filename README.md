# MTGJSON (Rust)

[![Crate](https://img.shields.io/crates/v/mtgjson)](https://crates.io/crates/mtgjson)
[![Docs](https://img.shields.io/docsrs/mtgjson)](https://docs.rs/mtgjson)

Unofficially provided lightweight models of the data provided by MTGJSON.

## Features

-   `unknown_variants` — Off by default. When enabled, every string enum that can
    grow over time (rarity, layout, promo type, frame effect, language, etc.)
    gains an `Unknown(UnknownStr)` variant, so deserialization tolerates values
    added to MTGJSON after this crate was published instead of failing. The value
    is captured in [`UnknownStr`], a `Copy` wrapper around an interned `&'static
    str` that derefs to `str`, so the enums stay `Copy`. Enabling this is a mild
    breaking change for code that matches these enums exhaustively.

## License

Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
