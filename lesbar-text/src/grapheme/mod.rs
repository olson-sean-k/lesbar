mod buf;

#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;
use core::mem;
use core::ops::Deref;
use mitsein::str1::Str1;
use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};
use unicode_width::UnicodeWidthStr;

use crate::RuneError;

#[cfg(feature = "alloc")]
pub use crate::grapheme::buf::*;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Grapheme(Str1);

impl Grapheme {
    /// # Safety
    ///
    /// The given string slice `text` must be non-empty.
    pub const unsafe fn from_str_unchecked(text: &str) -> &Self {
        Grapheme::from_str1_unchecked(Str1::from_str_unchecked(text))
    }

    pub const fn from_str1_unchecked(text: &Str1) -> &Self {
        // SAFETY: `Grapheme` is `repr(transparent)`, so the representations of `Str1` and
        //         `Grapheme` are the same.
        unsafe { mem::transmute::<&'_ Str1, &'_ Grapheme>(text) }
    }

    pub fn try_from_str(text: &str) -> Result<&Self, RuneError<&str>> {
        Str1::try_from_str(text)
            .map_err(RuneError::from_invalid)
            .and_then(|text| Grapheme::try_from_str1(text).map_err(|error| error.map(Str1::as_str)))
    }

    pub fn try_from_str1(text: &Str1) -> Result<&Self, RuneError<&Str1>> {
        if crate::is_grapheme(text) {
            Ok(Grapheme::from_str1_unchecked(text))
        }
        else {
            Err(RuneError::from_invalid(text))
        }
    }
}

impl Grapheme {
    pub fn to_char(&self) -> Option<char> {
        match self.as_str1().chars1().enumerate().last() {
            (0, point) => Some(point),
            _ => None,
        }
    }

    pub fn as_str1(&self) -> &Str1 {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        self.as_str1().as_str()
    }

    // TODO: The claim below that private-use characters have no display width may not be true.
    //       `unicode-width` reports a non-zero display width for the private-use characters in
    //       this module's unit tests! Investigate this and simplify the `is_text` predicate if
    //       possible.
    // A grapheme cluster is considered text if it is a private-use character or its display width
    // is non-zero per UCS and UAX11. Private-use characters have no display width, but are
    // typically rendered as a replacement glyph when not assigned or recognized. When used as
    // intended, private-use characters represent a glyph with some non-zero rendered width.
    pub fn is_legible_text(&self) -> bool {
        self.is_private_use_character() || self.width() != 0
    }

    pub fn is_private_use_character(&self) -> bool {
        self.to_char()
            .map(UnicodeGeneralCategory::general_category)
            .is_some_and(|category| matches!(category, GeneralCategory::PrivateUse))
    }
}

impl AsRef<Grapheme> for Grapheme {
    fn as_ref(&self) -> &Grapheme {
        self
    }
}

impl AsRef<str> for Grapheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<Str1> for Grapheme {
    fn as_ref(&self) -> &Str1 {
        self.as_str1()
    }
}

impl Deref for Grapheme {
    type Target = Str1;

    fn deref(&self) -> &Self::Target {
        self.as_str1()
    }
}

impl PartialEq<str> for Grapheme {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl PartialEq<Str1> for Grapheme {
    fn eq(&self, other: &Str1) -> bool {
        self.as_str1().eq(other)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl ToOwned for Grapheme {
    type Owned = GraphemeBuf;

    fn to_owned(&self) -> Self::Owned {
        GraphemeBuf::from_string1_unchecked(self.0.to_owned())
    }
}

impl<'t> TryFrom<&'t str> for &'t Grapheme {
    type Error = RuneError<&'t str>;

    fn try_from(text: &'t str) -> Result<Self, Self::Error> {
        Grapheme::try_from_str(text)
    }
}

impl<'t> TryFrom<&'t Str1> for &'t Grapheme {
    type Error = RuneError<&'t Str1>;

    fn try_from(text: &'t Str1) -> Result<Self, Self::Error> {
        Grapheme::try_from_str1(text)
    }
}
