#![no_std]

use unicode_properties::{GeneralCategory, UnicodeGeneralCategory};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub trait StrExt {
    fn has_text(&self) -> bool;
}

impl StrExt for str {
    // TODO: The definition of "text" is critical to the purpose of these crates. This must be as
    //       well-defined as possible and documented accordingly. This function implements this
    //       important predicate and so should probably provide this definition in its API
    //       documentation.
    fn has_text(&self) -> bool {
        self.width() != 0
            && self
                .graphemes(true)
                .map(Grapheme)
                .filter(Grapheme::is_text)
                .take(1)
                .count()
                != 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
struct Grapheme<'t>(&'t str);

impl Grapheme<'_> {
    fn as_singular_code_point(&self) -> Option<char> {
        match self
            .0
            .chars()
            .enumerate()
            .last()
            .expect("grapheme cluster has no code points")
        {
            (0, point) => Some(point),
            _ => None,
        }
    }

    // A grapheme cluster is considered text if it is a private-use character or its display width
    // is non-zero per UCS and UAX11. Private-use characters have no display width, but are
    // typically rendered as a replacement glyph when not assigned or recognized. When used as
    // intended, private-use characters represent a glyph with some non-zero rendered width.
    pub fn is_text(&self) -> bool {
        self.is_private_use_character() || self.0.width() != 0
    }

    fn is_private_use_character(&self) -> bool {
        self.as_singular_code_point()
            .map(UnicodeGeneralCategory::general_category)
            .map_or(false, |category| {
                matches!(category, GeneralCategory::PrivateUse)
            })
    }
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
    #[case::one_plane_basic_multilingual("\u{E064}")]
    #[case::one_plane_15("\u{F00FF}")]
    #[case::one_plane_16("\u{100000}")]
    #[case::one_with_non_printable("\u{200B}\u{E064}")]
    #[case::many_from_each_plane("\u{E000}\u{F0000}\u{10FFFD}")]
    #[case::many_with_non_printable("\u{E000}\u{200B}\u{E001}")]
    fn str_with_private_use_chars_has_text(#[case] text: &str) {
        assert!(text.has_text())
    }
}
