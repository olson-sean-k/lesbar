#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod grapheme;
pub mod iter;

#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;
use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};
use unicode_width::UnicodeWidthStr;

use crate::iter::{GraphemeIndices, Graphemes};

const RUNE_ERROR_MESSAGE: &str =
    "encountered an invalid code point, character, or grapheme cluster";

pub trait StrExt {
    fn graphemes(&self) -> Graphemes<'_>;

    fn grapheme_indices(&self) -> GraphemeIndices<'_>;

    fn has_text(&self) -> bool;
}

impl StrExt for str {
    fn graphemes(&self) -> Graphemes<'_> {
        Graphemes::from_str(self)
    }

    fn grapheme_indices(&self) -> GraphemeIndices<'_> {
        GraphemeIndices::from_str(self)
    }

    // TODO: The definition of "text" is critical to the purpose of these crates. This must be as
    //       well-defined as possible and documented accordingly. This function implements this
    //       important predicate and so should probably provide this definition in its API
    //       documentation.
    fn has_text(&self) -> bool {
        self.width() != 0
            && self
                .graphemes()
                .filter(|grapheme| grapheme.is_text())
                .take(1)
                .count()
                != 0
    }
}

// TODO: Implement `From<mitsein::EmptyError<_>>`.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct RuneError<T> {
    invalid: T,
}

impl<T> RuneError<T> {
    fn from_invalid(invalid: T) -> Self {
        RuneError { invalid }
    }

    pub fn into_invalid(self) -> T {
        self.invalid
    }

    fn map<U, F>(self, f: F) -> RuneError<U>
    where
        F: FnOnce(T) -> U,
    {
        RuneError {
            invalid: f(self.invalid),
        }
    }

    pub fn take(self) -> (T, RuneError<()>) {
        (self.invalid, RuneError::from_invalid(()))
    }

    pub fn take_and_drop(self) -> RuneError<()> {
        self.take().1
    }

    pub fn as_invalid(&self) -> &T {
        &self.invalid
    }
}

impl<T> RuneError<&'_ T> {
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn into_owning(self) -> RuneError<T::Owned>
    where
        T: ToOwned,
    {
        RuneError::from_invalid(self.invalid.to_owned())
    }
}

impl<T> Debug for RuneError<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("RuneError").finish_non_exhaustive()
    }
}

impl<T> Display for RuneError<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{RUNE_ERROR_MESSAGE}")
    }
}

impl<T> Error for RuneError<T> {}

fn is_grapheme(text: &str) -> bool {
    matches!(text.graphemes().take(2).enumerate().last(), Some((0, _)))
}

#[cfg(test)]
mod tests {
    extern crate std;

    use rstest::rstest;

    use crate::StrExt as _;

    #[rstest]
    fn empty_str_has_no_text() {
        assert!(!"".has_text());
    }

    #[rstest]
    #[case::one_from_plane_basic_multilingual("\u{E064}")]
    #[case::one_from_plane_15("\u{F00FF}")]
    #[case::one_from_plane_16("\u{100000}")]
    #[case::one_with_non_text("\u{200B}\u{E064}")]
    #[case::many_from_each_plane("\u{E000}\u{F0000}\u{10FFFD}")]
    #[case::many_with_non_text("\u{E000}\u{200B}\u{E001}")]
    fn str_with_private_use_characters_has_text(#[case] text: &str) {
        assert!(text.has_text())
    }
}
