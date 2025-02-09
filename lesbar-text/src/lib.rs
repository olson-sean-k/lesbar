#![no_std]

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub trait StrExt {
    fn has_printable_text(&self) -> bool;
}

impl StrExt for str {
    // TODO: The definition of "printable" or "legible" is critical to the purpose of these crates.
    //       This must be as well-defined as possible and documented accordingly. This function
    //       implements this important predicate and so should probably provide this definition in
    //       its API documentation.
    // TODO: Consider particular Unicode code points that may thwart this implementation or present
    //       some ambiguity. For example, private-use code points report zero-width in
    //       `unicode-width` and have no established properties, because they are user-defined. Are
    //       these code points and grapheme clusters "printable"? Note that software tends to
    //       render these with the replacement glyph (which is distinct from the replacement
    //       **character**). U+E064 renders a replacement glyph in GNOME Terminal and Termius, for
    //       example.
    fn has_printable_text(&self) -> bool {
        self.width() != 0
            && self
                .graphemes(true)
                .map(Grapheme)
                .filter(Grapheme::is_printable_text)
                .take(1)
                .count()
                != 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
struct Grapheme<'t>(&'t str);

impl Grapheme<'_> {
    pub fn is_printable_text(&self) -> bool {
        self.0.width() != 0
    }
}

#[cfg(test)]
mod tests {}
