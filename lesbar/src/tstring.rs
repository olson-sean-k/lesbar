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
use mitsein::str1::Str1;
use mitsein::string1::{CowStr1, String1};
use mitsein::Segmentation;

use crate::tstr::TStr;
use crate::{Grapheme, StrExt as _, Text};

pub type BoxedTStr = Box<TStr>;

pub trait BoxedTStrExt {}

impl BoxedTStrExt for BoxedTStr {}

pub type CowTStr<'a> = Cow<'a, TStr>;

pub trait CowTStrExt<'a> {}

impl<'a> CowTStrExt<'a> for CowTStr<'a> {}

pub type PopOr<'t, T> = TakeOr<'t, T, RangeTo<usize>>;

#[derive(Debug)]
pub struct TakeOr<'t, T, N = ()> {
    text: &'t mut TString,
    remainder: N,
    many: fn(&'t mut TString, N) -> T,
}

impl<'t, T, N> TakeOr<'t, T, N> {
    const fn with(text: &'t mut TString, remainder: N, many: fn(&mut TString, N) -> T) -> Self {
        TakeOr {
            text,
            remainder,
            many,
        }
    }
}

impl<'t, T, N> TakeOr<'t, T, N>
where
    N: Clone + SliceIndex<str, Output = str>,
{
    fn take_or_else<E, F>(self, one: F) -> Result<T, E>
    where
        F: FnOnce(&'t mut TString, N) -> E,
    {
        let TakeOr {
            text,
            remainder,
            many,
        } = self;
        if text
            .get(remainder.clone())
            .expect("string slice out of bounds or not on code point boundary")
            .has_text()
        {
            Ok(many(text, remainder))
        }
        else {
            Err(one(text, remainder))
        }
    }

    pub fn none(self) -> Option<T> {
        self.take_or_else(|_, _| ()).ok()
    }
}

impl<'t, T> TakeOr<'t, T, RangeTo<usize>> {
    pub fn get(self) -> Result<T, &'t str> {
        self.take_or_else(|text, remainder| {
            // `take_or_else` attempts to slice the string, so `[]` is used here instead of `get`
            // and `expect`.
            &text[remainder.end..]
        })
    }
}

pub type TString = Text<String>;

impl TString {
    pub const fn from_string1_unchecked(text: String1) -> Self {
        TString { text }
    }

    pub fn into_string1(self) -> String1 {
        self.text
    }

    pub fn pop_grapheme_or(&mut self) -> PopOr<'_, Grapheme<'_>> {
        let (index, _) = self.grapheme_indices1().rev().first();
        // SAFETY: `index` demarks a grapheme and `TakeOr` only calls this function if the
        //         remainder is a valid string slice and has text, so splitting off the grapheme
        //         produces non-empty and valid UTF-8 on both sides and `self` remains textual.
        TakeOr::with(self, ..index, |text, remainder| unsafe {
            Grapheme::from_string_unchecked(String::from_utf8_unchecked(
                text.as_mut_string1()
                    .as_mut_vec1()
                    .segment(remainder.end..)
                    .split_off(0),
            ))
        })
    }

    pub fn leak<'a>(self) -> &'a TStr {
        TStr::from_str1_unchecked(self.text.leak())
    }

    pub fn as_tstr(&self) -> &TStr {
        TStr::from_str1_unchecked(self.text.as_str1())
    }

    pub fn as_mut_tstr(&mut self) -> &mut TStr {
        TStr::from_mut_str1_unchecked(self.text.as_mut_str1())
    }

    const fn as_string1(&self) -> &String1 {
        &self.text
    }

    const fn as_mut_string1(&mut self) -> &mut String1 {
        &mut self.text
    }
}

impl AsMut<Str1> for TString {
    fn as_mut(&mut self) -> &mut Str1 {
        self.as_mut_tstr().as_mut_str1()
    }
}

impl AsMut<TStr> for TString {
    fn as_mut(&mut self) -> &mut TStr {
        self.as_mut_tstr()
    }
}

impl AsRef<Str1> for TString {
    fn as_ref(&self) -> &Str1 {
        self.as_tstr().as_str1()
    }
}

impl AsRef<TStr> for TString {
    fn as_ref(&self) -> &TStr {
        self.as_tstr()
    }
}

impl Borrow<TStr> for TString {
    fn borrow(&self) -> &TStr {
        self.as_tstr()
    }
}

impl BorrowMut<TStr> for TString {
    fn borrow_mut(&mut self) -> &mut TStr {
        self.as_mut_tstr()
    }
}

impl Debug for TString {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.as_str())
    }
}

impl Deref for TString {
    type Target = TStr;

    fn deref(&self) -> &Self::Target {
        self.as_tstr()
    }
}

impl DerefMut for TString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_tstr()
    }
}

impl Display for TString {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl<'a> From<&'a TStr> for TString {
    fn from(text: &'a TStr) -> Self {
        TString::from_string1_unchecked(String1::from(text.as_str1()))
    }
}

impl<T> PartialEq<&'_ T> for TString
where
    TString: PartialEq<T>,
    T: ?Sized,
{
    fn eq(&self, other: &&'_ T) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<Cow<'_, str>> for TString {
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        self.as_str().eq(other.as_ref())
    }
}

impl PartialEq<CowStr1<'_>> for TString {
    fn eq(&self, other: &CowStr1<'_>) -> bool {
        self.as_str1().eq(other.as_ref())
    }
}

impl PartialEq<CowTStr<'_>> for TString {
    fn eq(&self, other: &CowTStr<'_>) -> bool {
        self.as_tstr().eq(other.as_ref())
    }
}

impl PartialEq<str> for TString {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl PartialEq<Str1> for TString {
    fn eq(&self, other: &Str1) -> bool {
        self.as_str1().eq(other)
    }
}

impl PartialEq<String1> for TString {
    fn eq(&self, other: &String1) -> bool {
        self.as_string1().eq(other)
    }
}

impl PartialEq<TStr> for TString {
    fn eq(&self, other: &TStr) -> bool {
        self.as_tstr().eq(other)
    }
}

impl<'a> TryFrom<&'a str> for TString {
    type Error = &'a str;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        String1::try_from(text).and_then(|text1| TString::try_from(text1).map_err(|_| text))
    }
}

impl TryFrom<String> for TString {
    type Error = String;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        String1::try_from(text)
            .and_then(|text| TString::try_from(text).map_err(String1::into_string))
    }
}

impl TryFrom<String1> for TString {
    type Error = String1;

    fn try_from(text: String1) -> Result<Self, Self::Error> {
        if text.has_text() {
            Ok(TString::from_string1_unchecked(text))
        }
        else {
            Err(text)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;

    use rstest::rstest;

    use crate::tstr::TStr;
    use crate::tstring::TString;

    #[rstest]
    #[case::only_one_grapheme("A", "A")]
    #[case::only_one_grapheme("あ", "あ")]
    #[case::alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "A")]
    #[case::whitespace("   ", " ")]
    #[case::whitespace("\tend", "\t")]
    #[case::cjk("練習しなくてはいけないと思う", "練")]
    #[case::non_text_cluster_prefix("\u{200B}\u{E064}ZWSP+PUC", "\u{200B}\u{E064}")]
    fn pop_grapheme_or_from_tstring_until_exhausted_then_tstring_eq(
        #[case] text: &str,
        #[case] expected: &str,
    ) {
        let mut text = TString::try_from(text).unwrap();
        let expected = TStr::try_from_str(expected).unwrap();
        while text.pop_grapheme_or().none().is_some() {}
        assert_eq!(text, expected);
    }
}
