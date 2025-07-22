#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod grapheme;
pub mod iter;

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
        self.width() != 0
            && self
                .graphemes()
                .filter(|grapheme| grapheme.is_text())
                .take(1)
                .count()
                != 0
    }
}

fn is_grapheme(text: &str) -> bool {
    matches!(text.graphemes().take(2).enumerate().last(), Some((0, _)))
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
