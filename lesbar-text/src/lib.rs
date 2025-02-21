#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod iter;

use core::ops::Deref;
use mitsein::str1::Str1;
use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};
use unicode_width::UnicodeWidthStr;

use crate::iter::{GraphemeIndices, Graphemes};

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
        self.width() != 0 && self.graphemes().filter(Grapheme::is_text).take(1).count() != 0
    }
}

// TODO: `Grapheme` is meant as more of an "ephemeral" type, so it does not have explicit slice vs.
//       buffer types. This keeps its API minimal, but also inflexible. Note that enabling the
//       `alloc` feature changes `Grapheme`'s representation to support buffering but at a cost. A
//       `Grapheme` slice type, `GraphemeBuf` buffer type, and `CowGrapheme` (`Cow<'_, Grapheme>`)
//       copy-on-write type are probably a better solution for this.

#[cfg(not(feature = "alloc"))]
mod grapheme {
    use mitsein::str1::Str1;

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Grapheme<'t>(&'t Str1);

    impl<'t> Grapheme<'t> {
        /// # Safety
        ///
        /// The given string slice `text` must be non-empty.
        pub const unsafe fn from_str_unchecked(text: &'t str) -> Self {
            Grapheme(Str1::from_str_unchecked(text))
        }

        pub const fn from_str1_unchecked(text: &'t Str1) -> Self {
            Grapheme(text)
        }
    }

    impl Grapheme<'_> {
        pub fn as_str1(&self) -> &Str1 {
            self.0
        }
    }
}

#[cfg(feature = "alloc")]
mod grapheme {
    use alloc::borrow::Cow;
    use alloc::string::String;
    use mitsein::borrow1::CowStr1;
    use mitsein::str1::Str1;
    use mitsein::string1::String1;

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Grapheme<'t>(CowStr1<'t>);

    impl<'t> Grapheme<'t> {
        /// # Safety
        ///
        /// The given string buffer `text` must be non-empty.
        #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
        pub unsafe fn from_string_unchecked(text: String) -> Self {
            Grapheme(CowStr1::Owned(String1::from_string_unchecked(text)))
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
        pub fn from_string1_unchecked(text: String1) -> Self {
            Grapheme(CowStr1::Owned(text))
        }

        /// # Safety
        ///
        /// The given string slice `text` must be non-empty.
        pub const unsafe fn from_str_unchecked(text: &'t str) -> Self {
            Grapheme(CowStr1::Borrowed(Str1::from_str_unchecked(text)))
        }

        pub const fn from_str1_unchecked(text: &'t Str1) -> Self {
            Grapheme(CowStr1::Borrowed(text))
        }
    }

    impl Grapheme<'_> {
        #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
        pub fn into_owned(self) -> Grapheme<'static> {
            Grapheme(CowStr1::Owned(self.0.into_owned()))
        }

        pub fn to_string1(&self) -> String1 {
            self.0.to_string1()
        }

        pub fn as_str1(&self) -> &Str1 {
            self.0.as_ref()
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<'t> From<Grapheme<'t>> for Cow<'t, str> {
        fn from(grapheme: Grapheme<'t>) -> Self {
            match grapheme.0 {
                CowStr1::Borrowed(text) => Cow::Borrowed(text.as_str()),
                CowStr1::Owned(text) => Cow::Owned(text.into_string()),
            }
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<'t> From<Grapheme<'t>> for CowStr1<'t> {
        fn from(grapheme: Grapheme<'t>) -> Self {
            grapheme.0
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<'t> From<Grapheme<'t>> for String {
        fn from(grapheme: Grapheme<'t>) -> Self {
            grapheme.to_string1().into_string()
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<'t> From<Grapheme<'t>> for String1 {
        fn from(grapheme: Grapheme<'t>) -> Self {
            grapheme.to_string1()
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl TryFrom<String> for Grapheme<'_> {
        type Error = String;

        fn try_from(text: String) -> Result<Self, Self::Error> {
            String1::try_from(text).and_then(|text| Grapheme::try_from(text).map_err(String::from))
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl TryFrom<String1> for Grapheme<'_> {
        type Error = String1;

        fn try_from(text: String1) -> Result<Self, Self::Error> {
            if crate::is_grapheme(&text) {
                Ok(Grapheme::from_string1_unchecked(text))
            }
            else {
                Err(text)
            }
        }
    }
}

pub use grapheme::Grapheme;

impl Grapheme<'_> {
    pub fn to_char(&self) -> Option<char> {
        match self.as_str1().chars1().enumerate().last() {
            (0, point) => Some(point),
            _ => None,
        }
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
    pub fn is_text(&self) -> bool {
        self.is_private_use_character() || self.width() != 0
    }

    pub fn is_private_use_character(&self) -> bool {
        self.to_char()
            .map(UnicodeGeneralCategory::general_category)
            .is_some_and(|category| matches!(category, GeneralCategory::PrivateUse))
    }
}

impl AsRef<str> for Grapheme<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<Str1> for Grapheme<'_> {
    fn as_ref(&self) -> &Str1 {
        self.as_str1()
    }
}

impl Deref for Grapheme<'_> {
    type Target = Str1;

    fn deref(&self) -> &Self::Target {
        self.as_str1()
    }
}

impl<'t, T> PartialEq<&'_ T> for Grapheme<'t>
where
    Grapheme<'t>: PartialEq<T>,
    T: ?Sized,
{
    fn eq(&self, other: &&'_ T) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<str> for Grapheme<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl PartialEq<Str1> for Grapheme<'_> {
    fn eq(&self, other: &Str1) -> bool {
        self.as_str1().eq(other)
    }
}

impl<'t> TryFrom<&'t str> for Grapheme<'t> {
    type Error = &'t str;

    fn try_from(text: &'t str) -> Result<Self, Self::Error> {
        Str1::try_from_str(text)
            .and_then(|text| Grapheme::try_from(text).map_err(|text| text.as_str()))
    }
}

impl<'t> TryFrom<&'t Str1> for Grapheme<'t> {
    type Error = &'t Str1;

    fn try_from(text: &'t Str1) -> Result<Self, Self::Error> {
        if crate::is_grapheme(text) {
            Ok(Grapheme::from_str1_unchecked(text))
        }
        else {
            Err(text)
        }
    }
}

fn is_grapheme(text: &str) -> bool {
    matches!(text.graphemes().enumerate().last(), Some((0, _)))
}

#[cfg(test)]
mod tests {
    extern crate alloc;
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
