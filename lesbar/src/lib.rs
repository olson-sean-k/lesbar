//! Lesbar provides string types that must encode legible text.
//!
//! At time of writing, `rustdoc` ignores input type parameters in the "Methods from
//! `Deref<Target = _>`" section. For types that implement `Deref<Target = NonEmpty<_>>`, **the API
//! documentation may be misleading** and list all methods of [`NonEmpty`] regardless of its input
//! type parameter. This is mostly a problem for types that dereference to [`Str1`], such as
//! [`TStr`]. See [this `rustdoc` bug](https://github.com/rust-lang/rust/issues/24686).
//!
//! # Integrations and Cargo Features
//!
//! Lesbar supports `no_std` environments and provides features for integrating as needed with
//! [`alloc`]. By default, the `alloc` feature is enabled for complete support of the standard
//! library.
//!
//! The following table summarizes supported Cargo features and integrations.
//!
//! | Feature     | Default | Primary Dependency | Description                                         |
//! |-------------|---------|--------------------|-----------------------------------------------------|
//! | `alloc`     | No      | [`alloc`]          | Legible string buffers, like [`TString`].           |
//! | `serde`     | No      | [`serde`]          | De/serialization of legible strings with [`serde`]. |
//!
//! [`TStr`]: crate::tstr::TStr
//! [`TString`]: crate::tstring::TString
//! [`serde`]: https://crates.io/crates/serde
//! [`Str1`]: mitsein::str1::Str1

#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate self as lesbar;

pub mod tstr;
pub mod tstring;

pub mod prelude {
    //! Re-exports of recommended APIs and extension traits.

    pub use crate::tstr::TStr;
    #[cfg(feature = "alloc")]
    pub use crate::tstring::{CowTStrExt as _, TString};
    pub use crate::StrExt as _;
}

use mitsein::NonEmpty;

pub use lesbar_macros::{str1, tstr};
pub use lesbar_text::{iter, Grapheme, StrExt};

// Though this type contains a `NonEmpty<T>` rather than a `T`, this `derive` is correct, since
// `NonEmpty` also implements these traits with bounds on `T`.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Text<T>
where
    T: ?Sized,
{
    text: NonEmpty<T>,
}

impl<T> AsRef<T> for Text<T> {
    fn as_ref(&self) -> &T {
        self.text.as_ref()
    }
}

#[cfg(test)]
mod tests {}
