//! A non-empty [`String`][`string`] that represents legible text.
//!
//! [`string`]: alloc::string

#![cfg(feature = "alloc")]
#![cfg_attr(docsrs, doc(cfg(feature = "alloc")))]

use alloc::borrow::{Borrow, BorrowMut, Cow};
use alloc::boxed::Box;
use alloc::string::String;
use core::fmt::{self, Debug, Display, Formatter};
use core::ops::{Deref, DerefMut, RangeTo};
use core::slice::SliceIndex;
use mitsein::borrow1::CowStr1;
use mitsein::boxed1::BoxedStr1;
use mitsein::str1::Str1;
use mitsein::string1::String1;
use mitsein::Segmentation;

use crate::grapheme::GraphemeBuf;
use crate::text::Text;
use crate::{IllegibleError, Legible, StrExt as _};

pub type BoxedText = Box<Text>;

pub trait BoxedTextExt {
    fn from_boxed_str1_unchecked(text: BoxedStr1) -> Self;

    fn into_boxed_str1(self) -> BoxedStr1;
}

impl BoxedTextExt for BoxedText {
    fn from_boxed_str1_unchecked(text: BoxedStr1) -> Self {
        let text = Box::into_raw(text);
        // SAFETY: Client code is responsible for asserting that the input string has legible text.
        //         This transmutation is safe, because `Str1` and `Text` have the same
        //         representation (`Text` is `repr(transparent)`). Moreover, the allocator only
        //         requires that the memory location and layout are the same when deallocating, so
        //         dropping the transmuted `Box` is sound.
        unsafe { Box::from_raw(text as *mut Text) }
    }

    fn into_boxed_str1(self) -> BoxedStr1 {
        let text = Box::into_raw(self);
        // SAFETY: This transmutation is safe, because `Str1` and `Text` have the same
        //         representation (`Text` is `repr(transparent)`). Moreover, the allocator only
        //         requires that the memory location and layout are the same when deallocating, so
        //         dropping the transmuted `Box` is sound.
        unsafe { Box::from_raw(text as *mut Str1) }
    }
}

pub type CowText<'a> = Cow<'a, Text>;

pub trait CowTextExt<'a> {}

impl<'a> CowTextExt<'a> for CowText<'a> {}

pub type Pop<'t, T> = Take<'t, T, RangeTo<usize>>;

#[derive(Debug)]
pub struct Take<'t, T, N = ()> {
    text: &'t mut TextBuf,
    remainder: N,
    many: fn(&'t mut TextBuf, N) -> T,
}

impl<'t, T, N> Take<'t, T, N> {
    const fn with(text: &'t mut TextBuf, remainder: N, many: fn(&mut TextBuf, N) -> T) -> Self {
        Take {
            text,
            remainder,
            many,
        }
    }
}

impl<'t, T, N> Take<'t, T, N>
where
    N: Clone + SliceIndex<str, Output = str>,
{
    fn take_or_else<E, F>(self, one: F) -> Result<T, E>
    where
        F: FnOnce(&'t mut TextBuf, N) -> E,
    {
        let Take {
            text,
            remainder,
            many,
        } = self;
        if text
            .get(remainder.clone())
            .expect("string slice out of bounds or not on code point boundary")
            .has_legible_text()
        {
            Ok(many(text, remainder))
        }
        else {
            Err(one(text, remainder))
        }
    }

    pub fn or_else<E, F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        self.take_or_else(|_, _| f())
    }

    pub fn or_none(self) -> Option<T> {
        self.take_or_else(|_, _| ()).ok()
    }

    pub fn or_false(self) -> bool {
        self.or_none().is_some()
    }
}

impl<'t, T> Take<'t, T, RangeTo<usize>> {
    pub fn or_get(self) -> Result<T, &'t str> {
        self.take_or_else(|text, remainder| {
            // `take_or_else` attempts to slice the string, so `[]` is used here instead of `get`
            // and `expect`.
            &text[remainder.end..]
        })
    }
}

pub type TextBuf = Legible<String>;

impl TextBuf {
    pub const fn from_string1_unchecked(text: String1) -> Self {
        TextBuf { text }
    }

    pub fn into_string1(self) -> String1 {
        self.text
    }

    pub fn pop_char(&mut self) -> Pop<'_, char> {
        let (index, _) = self.char_indices1().rev().first();
        // `TakeOr` only calls this function if the range has text. Since `index` demarks the last
        // code point and the exclusive end of the range, there must be a terminating code point
        // that is unnecessary for `self` to remain textual.
        Take::with(self, ..index, |text, _| {
            text.as_mut_string1()
                .pop_or()
                .none()
                .expect("expected code point following textual sub-string")
        })
    }

    pub fn pop_grapheme(&mut self) -> Pop<'_, GraphemeBuf> {
        let (index, _) = self.grapheme_indices1().rev().first();
        // SAFETY: `index` demarks a grapheme and `TakeOr` only calls this function if the
        //         range is a valid string slice and has text, so splitting off the grapheme
        //         produces non-empty and valid UTF-8 on both sides and `self` remains textual.
        Take::with(self, ..index, |text, remainder| unsafe {
            GraphemeBuf::from_string_unchecked(String::from_utf8_unchecked(
                text.as_mut_string1()
                    .as_mut_vec1()
                    .segment(remainder.end..)
                    .split_off(0),
            ))
        })
    }

    pub fn leak<'a>(self) -> &'a Text {
        Text::from_str1_unchecked(self.text.leak())
    }

    pub fn as_text(&self) -> &Text {
        Text::from_str1_unchecked(self.text.as_str1())
    }

    pub fn as_mut_text(&mut self) -> &mut Text {
        Text::from_mut_str1_unchecked(self.text.as_mut_str1())
    }

    const fn as_string1(&self) -> &String1 {
        &self.text
    }

    const fn as_mut_string1(&mut self) -> &mut String1 {
        &mut self.text
    }
}

impl AsMut<Str1> for TextBuf {
    fn as_mut(&mut self) -> &mut Str1 {
        self.as_mut_text().as_mut_str1()
    }
}

impl AsMut<Text> for TextBuf {
    fn as_mut(&mut self) -> &mut Text {
        self.as_mut_text()
    }
}

impl AsRef<Str1> for TextBuf {
    fn as_ref(&self) -> &Str1 {
        self.as_text().as_str1()
    }
}

impl AsRef<Text> for TextBuf {
    fn as_ref(&self) -> &Text {
        self.as_text()
    }
}

impl Borrow<Text> for TextBuf {
    fn borrow(&self) -> &Text {
        self.as_text()
    }
}

impl BorrowMut<Text> for TextBuf {
    fn borrow_mut(&mut self) -> &mut Text {
        self.as_mut_text()
    }
}

impl Debug for TextBuf {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.as_str())
    }
}

impl Deref for TextBuf {
    type Target = Text;

    fn deref(&self) -> &Self::Target {
        self.as_text()
    }
}

impl DerefMut for TextBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_text()
    }
}

impl Display for TextBuf {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl From<BoxedText> for TextBuf {
    fn from(text: BoxedText) -> Self {
        TextBuf::from_string1_unchecked(String1::from(text.into_boxed_str1()))
    }
}

impl<'a> From<&'a Text> for TextBuf {
    fn from(text: &'a Text) -> Self {
        TextBuf::from_string1_unchecked(String1::from(text.as_str1()))
    }
}

impl<T> PartialEq<&'_ T> for TextBuf
where
    TextBuf: PartialEq<T>,
    T: ?Sized,
{
    fn eq(&self, other: &&'_ T) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<Cow<'_, str>> for TextBuf {
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        self.as_str().eq(other.as_ref())
    }
}

impl PartialEq<CowStr1<'_>> for TextBuf {
    fn eq(&self, other: &CowStr1<'_>) -> bool {
        self.as_str1().eq(other.as_ref())
    }
}

impl PartialEq<CowText<'_>> for TextBuf {
    fn eq(&self, other: &CowText<'_>) -> bool {
        self.as_text().eq(other.as_ref())
    }
}

impl PartialEq<str> for TextBuf {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl PartialEq<Str1> for TextBuf {
    fn eq(&self, other: &Str1) -> bool {
        self.as_str1().eq(other)
    }
}

impl PartialEq<String1> for TextBuf {
    fn eq(&self, other: &String1) -> bool {
        self.as_string1().eq(other)
    }
}

impl PartialEq<Text> for TextBuf {
    fn eq(&self, other: &Text) -> bool {
        self.as_text().eq(other)
    }
}

impl<'a> TryFrom<&'a str> for TextBuf {
    type Error = IllegibleError<&'a str>;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        String1::try_from(text)
            .map_err(IllegibleError::from_illegible)
            .and_then(|text1| TextBuf::try_from(text1).map_err(|error| error.map(|_| text)))
    }
}

impl TryFrom<String> for TextBuf {
    type Error = IllegibleError<String>;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        String1::try_from(text)
            .map_err(IllegibleError::from_illegible)
            .and_then(|text1| {
                TextBuf::try_from(text1).map_err(|error| error.map(String1::into_string))
            })
    }
}

impl<'a> TryFrom<&'a Str1> for TextBuf {
    type Error = IllegibleError<&'a Str1>;

    fn try_from(text: &'a Str1) -> Result<Self, Self::Error> {
        TextBuf::try_from(String1::from(text)).map_err(|error| error.map(|_| text))
    }
}

impl TryFrom<String1> for TextBuf {
    type Error = IllegibleError<String1>;

    fn try_from(text: String1) -> Result<Self, Self::Error> {
        if text.has_legible_text() {
            Ok(TextBuf::from_string1_unchecked(text))
        }
        else {
            Err(IllegibleError::from_illegible(text))
        }
    }
}

#[cfg(test)]
pub mod harness {
    use rstest::fixture;

    use crate::text::TextBuf;

    #[fixture]
    pub fn text() -> TextBuf {
        TextBuf::try_from("legible").unwrap()
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use rstest::rstest;
    #[cfg(feature = "serde")]
    use {alloc::vec::Vec, serde_test::Token};

    use crate::text::{Text, TextBuf};
    #[cfg(feature = "serde")]
    use {crate::serde, crate::serde::harness::legible, crate::text::buf::harness::text};

    #[rstest]
    #[case::only_one_char("A", "A")]
    #[case::only_one_char("あ", "あ")]
    #[case::alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "A")]
    #[case::japanese("練習しなくてはいけないと思う", "練")]
    #[case::whitespace("   ", " ")]
    #[case::whitespace("\tend", "\t")]
    #[case::non_text_prefix("\u{200B}ZWSP", "\u{200B}Z")]
    #[case::non_text_prefix("\u{200B}\u{E064}ZWSP+PUC", "\u{200B}\u{E064}")]
    #[case::combining("ä", "ä")]
    #[case::combining("\u{1F3F3}\u{FE0F}\u{200D}\u{1F308}", "\u{1F3F3}")]
    fn pop_char_from_text_buf_until_exhausted_then_text_buf_eq(
        #[case] text: &str,
        #[case] expected: &str,
    ) {
        let mut text = TextBuf::try_from(text).unwrap();
        let expected = Text::try_from_str(expected).unwrap();
        while text.pop_char().or_false() {}
        assert_eq!(text, expected);
    }

    #[rstest]
    #[case::only_one_grapheme("A", "A")]
    #[case::only_one_grapheme("あ", "あ")]
    #[case::alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "A")]
    #[case::japanese("練習しなくてはいけないと思う", "練")]
    #[case::whitespace("   ", " ")]
    #[case::whitespace("\tend", "\t")]
    #[case::non_text_prefix("\u{200B}ZWSP", "\u{200B}Z")]
    #[case::non_text_prefix("\u{200B}\u{E064}ZWSP+PUC", "\u{200B}\u{E064}")]
    #[case::combining("ä", "ä")]
    #[case::combining(
        "\u{1F3F3}\u{FE0F}\u{200D}\u{1F308}",
        "\u{1F3F3}\u{FE0F}\u{200D}\u{1F308}"
    )]
    fn pop_grapheme_from_text_buf_until_exhausted_then_text_buf_eq(
        #[case] text: &str,
        #[case] expected: &str,
    ) {
        let mut text = TextBuf::try_from(text).unwrap();
        let expected = Text::try_from_str(expected).unwrap();
        while text.pop_grapheme().or_false() {}
        assert_eq!(text, expected);
    }

    #[cfg(feature = "serde")]
    #[rstest]
    fn de_serialize_text_buf_into_and_from_tokens_eq(
        text: TextBuf,
        legible: impl Iterator<Item = Token>,
    ) {
        serde::harness::assert_into_and_from_tokens_eq::<_, Vec<_>>(text, legible);
    }

    #[cfg(feature = "serde")]
    #[rstest]
    #[case::empty(serde::harness::borrowed_str_token(""))]
    #[case::non_empty(serde::harness::borrowed_str_token("\u{FEFF}"))]
    fn deserialize_text_buf_from_illegible_tokens_then_illegible_error(
        #[case] tokens: impl Iterator<Item = Token>,
    ) {
        serde::harness::assert_deserialize_error_eq_illegible_error::<TextBuf, Vec<_>>(tokens);
    }
}
