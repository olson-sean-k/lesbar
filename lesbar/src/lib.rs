//! Lesbar provides printable string types that must encode legible text.
//!
//! # Integrations and Cargo Features
//!
//! Lesbar supports `no_std` environments and provides features for integrating as needed with
//! [`alloc`] and [`std`]. By default, the `std` feature is enabled for complete support of the
//! standard library.
//!
//! The following table summarizes supported Cargo features and integrations.
//!
//! | Feature     | Default | Primary Dependency | Description                                               |
//! |-------------|---------|--------------------|-----------------------------------------------------------|
//! | `alloc`     | No      | [`alloc`]          | Printable and non-empty string buffers, like [`PString`]. |
//! | `serde`     | No      | [`serde`]          | De/serialization of printable string with [`serde`].      |
//! | `std`       | Yes     | [`std`]            | Integrations with `std::io`.                              |
//!
//! [`PString`]: crate::pstring::PString
//! [`serde`]: https://crates.io/crates/serde

#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

extern crate self as lesbar;

pub mod pstr;
pub mod pstring;

pub mod prelude {
    //! Re-exports of recommended APIs and extension traits.

    pub use crate::pstr::PStr;
    #[cfg(feature = "alloc")]
    pub use crate::pstring::{CowPStrExt as _, PString};
    pub use crate::StrExt as _;
}

use mitsein::NonEmpty;

pub use lesbar_macros::{pstr, str1};
pub use lesbar_text::StrExt;

// Though this type contains a `NonEmpty<T>` rather than a `T`, this `derive` is correct, since
// `NonEmpty` also implements these traits with bounds on `T`.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Printable<T>
where
    T: ?Sized,
{
    text: NonEmpty<T>,
}

impl<T> AsRef<T> for Printable<T> {
    fn as_ref(&self) -> &T {
        self.text.as_ref()
    }
}

#[cfg(test)]
mod tests {}
