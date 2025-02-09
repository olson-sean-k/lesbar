**Lesbar** (ˈleːsbaːɐ̯ | _laze-bahr_) is a Rust library that provides strongly
typed APIs for printable and non-empty strings. These strings represent some
amount of legible text. Lesbar extends and is implemented with
[Mitsein][`mitsein`].

[![GitHub](https://img.shields.io/badge/GitHub-olson--sean--k/lesbar-8da0cb?logo=github&style=for-the-badge)](https://github.com/olson-sean-k/lesbar)
[![docs.rs](https://img.shields.io/badge/docs.rs-lesbar-66c2a5?logo=rust&style=for-the-badge)](https://docs.rs/lesbar)
[![crates.io](https://img.shields.io/crates/v/lesbar.svg?logo=rust&style=for-the-badge)](https://crates.io/crates/lesbar)

## Basic Usage

Allocating a `PString` (printable string) from a string literal:

```rust
use lesbar::prelude::*;

let text = PString::try_from("Servus!").unwrap();
let error = PString::try_from("\u{FEFF}").unwrap_err();
```

Constructing a `PStr` (printable string slice) with the `pstr!` macro:

```rust
let text = lesbar::pstr!("Macros sind der Hammer!");

// This does not build.
//
// let text = lesbar::pstr!("\u{200B}\u{200E}");
```

## Features and Comparisons

The [`mitsein`] crate provides the non-empty string types `String1` and `Str1`,
which represent non-empty strings. Similarly, the [`non-empty-string`] crate
provides the `NonEmptyString` type. However, these types only guarantee that
strings are comprised of one or more Unicode code points or bytes of UTF-8.
**Lesbar implements types with more strict requirements: printable strings that
must encode legible text.**

**Lesbar implements both printable strings and printable string slices
(`PString` and `PStr`),** which are analogous to standard Rust string types.
These types also support conversions into printable container types like `Box`.
The [`non-empty-string`] crate does not make this distinction, and only
implements owned string buffers.

Lesbar is implemented with the [`mitsein`] crate, which provides non-empty
collections, slices, and iterators. **Printable string types provide strongly
typed APIs for slicing and iteration that reflect the non-empty and printable
guarantee** with conversions into and from non-empty types. The
[`non-empty-string`] crate, for example, provides no conversions or iteration
mechanism that consider this property.

Lesbar is a `no_std` library and `alloc` is optional. **Printable string slices
can be used in contexts where OS features or allocation are not available.**

## Integrations and Cargo Features

Lesbar provides some optional features and integrations via the following Cargo
features.

| Feature     | Default | Primary Dependency | Description                                             |
|-------------|---------|--------------------|---------------------------------------------------------|
| `alloc`     | No      | `alloc`            | Printable and non-empty string buffers, like `PString`. |
| `serde`     | No      | [`serde`]          | De/serialization of printable strings with [`serde`].   |
| `std`       | Yes     | `std`              | Integrations with `std::io`.                            |

[`mitsein`]: https://crates.io/crates/mitsein
[`non-empty-string`]: https://crates.io/crates/non-empty-string
[`serde`]: https://crates.io/crates/serde
