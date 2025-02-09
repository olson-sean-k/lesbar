#![no_std]

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub trait StrExt {
    fn has_printable_text(&self) -> bool;
}

impl StrExt for str {
    fn has_printable_text(&self) -> bool {
        self.width() != 0
            && self
                .graphemes(true)
                .filter(|grapheme| grapheme.width() != 0)
                .take(1)
                .count()
                != 0
    }
}

#[cfg(test)]
mod tests {}
