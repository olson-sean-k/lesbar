**Lesbar** (ˈleːsbaːɐ̯ | _laze-bahr_) is a Rust library that provides strongly
typed APIs for non-empty strings that represent **legible** text. Lesbar extends
and is implemented with [Mitsein][`mitsein`].

[![GitHub](https://img.shields.io/badge/GitHub-olson--sean--k/lesbar-8da0cb?logo=github&style=for-the-badge)](https://github.com/olson-sean-k/lesbar)
[![docs.rs](https://img.shields.io/badge/docs.rs-lesbar-66c2a5?logo=rust&style=for-the-badge)](https://docs.rs/lesbar)
[![crates.io](https://img.shields.io/crates/v/lesbar.svg?logo=rust&style=for-the-badge)](https://crates.io/crates/lesbar)

## Basic Usage

Allocating a `TextBuf` (textual string buffer) from a string literal:

```rust
use lesbar::prelude::*;

let text = TextBuf::try_from("Servus!").unwrap();
let error = TextBuf::try_from("\u{FEFF}").unwrap_err();
```

Constructing a `Text` (textual string slice) with the `text!` macro:

```rust
let text = lesbar::text!("Macros sind der Hammer!");

// This does not build.
//
// let text = lesbar::text!("\u{200B}\u{200E}");
```

Removing text from a `TextBuf`:

```rust
use lesbar::prelude::*;

let mut text = TextBuf::from(lesbar::text!("Raus damit."));
let grapheme = text.pop_grapheme().or_none().unwrap();

assert_eq!(grapheme, ".");
```

## Legibility

Legible string types encode some non-zero amount of Unicode with a **specified**
non-zero column width or code points and grapheme clusters that **specify** a
visual presentation. Note that blank non-empty space is considered legible. This
is based only on the Unicode specification and its interpretations. Fonts,
glyphs, and other rendering elements are not considered at all, for example.

Some elements of Unicode are ambiguous regarding this notion of legibility, and
Lesbar attempts reasonable compromise that errs on the conservative side
(considering Unicode **illegible** when ambiguous).

Text rendering software has far more context when presenting text and can
interpret Unicode arbitrarily. There is no guarantee that the contents of a
legible string type in Lesbar will actually present as non-empty when rendered.
However, legible text is very unlikely to render this way.

## Features and Comparisons

The [`mitsein`] crate provides the non-empty string types `String1` and `Str1`,
which represent non-empty strings. Similarly, the [`non-empty-string`] crate
provides the `NonEmptyString` type. However, these types only guarantee that
strings are comprised of one or more Unicode code points or bytes of UTF-8.
**Lesbar implements types with more strict requirements: textual strings that
must encode some non-zero amount of legible (visible) text.**

**Lesbar implements both textual string slices and textual string buffers
(`Text` and `TextBuf`),** which are analogous to standard Rust string types.
These types also support conversions into textual container types like `Box`.
The [`non-empty-string`] crate does not make this distinction, only implements
string buffers, and does not preserve the non-empty property when converting
into iterators and containers like `Box`.

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
| `alloc`     | Yes     | `alloc`            | Legible string buffer types like `TextBuf`.         |
| `serde`     | No      | [`serde`]          | De/serialization of legible strings with [`serde`]. |

[`mitsein`]: https://crates.io/crates/mitsein
[`non-empty-string`]: https://crates.io/crates/non-empty-string
[`serde`]: https://crates.io/crates/serde
