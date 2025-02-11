**Lesbar** (ˈleːsbaːɐ̯ | _laze-bahr_) is a Rust library that provides strongly
typed APIs for strings that represent legible text. Lesbar extends and is
implemented with [Mitsein][`mitsein`].

[![GitHub](https://img.shields.io/badge/GitHub-olson--sean--k/lesbar-8da0cb?logo=github&style=for-the-badge)](https://github.com/olson-sean-k/lesbar)
[![docs.rs](https://img.shields.io/badge/docs.rs-lesbar-66c2a5?logo=rust&style=for-the-badge)](https://docs.rs/lesbar)
[![crates.io](https://img.shields.io/crates/v/lesbar.svg?logo=rust&style=for-the-badge)](https://crates.io/crates/lesbar)

## Basic Usage

Allocating a `TString` (textual string) from a string literal:

```rust
use lesbar::prelude::*;

let text = TString::try_from("Servus!").unwrap();
let error = TString::try_from("\u{FEFF}").unwrap_err();
```

Constructing a `TStr` (textual string slice) with the `tstr!` macro:

```rust
let text = lesbar::tstr!("Macros sind der Hammer!");

// This does not build.
//
// let text = lesbar::tstr!("\u{200B}\u{200E}");
```

Removing text from a `TString`:

```rust
use lesbar::prelude::*;

let mut text = TString::from(lesbar::tstr!("Raus damit."));
let grapheme = text.pop_grapheme_or().none().unwrap();

assert_eq!(grapheme, ".");
```

## Features and Comparisons

The [`mitsein`] crate provides the non-empty string types `String1` and `Str1`,
which represent non-empty strings. Similarly, the [`non-empty-string`] crate
provides the `NonEmptyString` type. However, these types only guarantee that
strings are comprised of one or more Unicode code points or bytes of UTF-8.
**Lesbar implements types with more strict requirements: textual strings that
must encode some amount of legible (visible) text.**

**Lesbar implements both textual strings and textual string slices (`TString`
and `TStr`),** which are analogous to standard Rust string types. These types
also support conversions into textual container types like `Box`. The
[`non-empty-string`] crate does not make this distinction, only implements owned
string buffers, and does not preserve the non-empty property when converting
into containers like `Box`.

Lesbar is implemented with the [`mitsein`] crate, which provides non-empty
collections, slices, and iterators. **Textual string types provide strongly
typed APIs for slicing and iteration that reflect the non-empty and legible
guarantee** with conversions into and from non-empty types. The
[`non-empty-string`] crate, for example, provides no conversions or iteration
mechanism that consider this property.

Lesbar is a `no_std` library and `alloc` is optional. **Textual string slices
can be used in contexts where OS features or allocation are not available.**

## Integrations and Cargo Features

Lesbar provides some optional features and integrations via the following Cargo
features.

| Feature     | Default | Primary Dependency | Description                                         |
|-------------|---------|--------------------|-----------------------------------------------------|
| `alloc`     | No      | `alloc`            | Legible string buffers, like `TString`.             |
| `serde`     | No      | [`serde`]          | De/serialization of legible strings with [`serde`]. |

[`mitsein`]: https://crates.io/crates/mitsein
[`non-empty-string`]: https://crates.io/crates/non-empty-string
[`serde`]: https://crates.io/crates/serde
