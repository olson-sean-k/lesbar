use unicode_segmentation::UnicodeSegmentation;

use crate::grapheme::Grapheme;

#[derive(Clone, Debug)]
pub struct Graphemes<'t> {
    input: unicode_segmentation::Graphemes<'t>,
}

impl<'t> Graphemes<'t> {
    pub(crate) fn from_str(text: &'t str) -> Self {
        Graphemes {
            input: text.graphemes(true),
        }
    }
}

impl DoubleEndedIterator for Graphemes<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // SAFETY: The `UnicodeSegmentation` implementation never emits empty `str`s as graphemes.
        //         Moreover, emitting an empty `str` here would most certrainly be considered a bug
        //         upstream.
        self.input
            .next_back()
            .map(|grapheme| unsafe { Grapheme::from_str_unchecked(grapheme) })
    }
}

impl<'t> Iterator for Graphemes<'t> {
    type Item = &'t Grapheme;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: The `UnicodeSegmentation` implementation never emits empty `str`s as graphemes.
        //         Moreover, emitting an empty `str` here would most certrainly be considered a bug
        //         upstream.
        self.input
            .next()
            .map(|grapheme| unsafe { Grapheme::from_str_unchecked(grapheme) })
    }
}

#[derive(Clone, Debug)]
pub struct GraphemeIndices<'t> {
    input: unicode_segmentation::GraphemeIndices<'t>,
}

impl<'t> GraphemeIndices<'t> {
    pub(crate) fn from_str(text: &'t str) -> Self {
        GraphemeIndices {
            input: text.grapheme_indices(true),
        }
    }
}

impl DoubleEndedIterator for GraphemeIndices<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // SAFETY: The `UnicodeSegmentation` implementation never emits empty `str`s as graphemes.
        //         Moreover, emitting an empty `str` here would most certrainly be considered a bug
        //         upstream.
        self.input
            .next_back()
            .map(|(index, grapheme)| unsafe { (index, Grapheme::from_str_unchecked(grapheme)) })
    }
}

impl<'t> Iterator for GraphemeIndices<'t> {
    type Item = (usize, &'t Grapheme);

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: The `UnicodeSegmentation` implementation never emits empty `str`s as graphemes.
        //         Moreover, emitting an empty `str` here would most certrainly be considered a bug
        //         upstream.
        self.input
            .next()
            .map(|(index, grapheme)| unsafe { (index, Grapheme::from_str_unchecked(grapheme)) })
    }
}
