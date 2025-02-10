//! A non-empty [`String`][`string`] that represents legible text.
//!
//! [`string`]: alloc::string

#![cfg(feature = "alloc")]

use alloc::borrow::{Borrow, BorrowMut, Cow};
use alloc::boxed::Box;
use alloc::string::String;
use core::fmt::{self, Debug, Display, Formatter};
use core::ops::{Deref, DerefMut};
use mitsein::string1::String1;

use crate::tstr::TStr;
use crate::{StrExt as _, Text};

pub type BoxedTStr = Box<TStr>;

pub trait BoxedTStrExt {}

impl BoxedTStrExt for BoxedTStr {}

pub type CowTStr<'a> = Cow<'a, TStr>;

pub trait CowTStrExt<'a> {}

impl<'a> CowTStrExt<'a> for CowTStr<'a> {}

pub type TString = Text<String>;

impl TString {
    pub const fn from_string1_unchecked(text: String1) -> Self {
        TString { text }
    }

    pub fn into_string1(self) -> String1 {
        self.text
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
}

impl AsMut<TStr> for TString {
    fn as_mut(&mut self) -> &mut TStr {
        self.as_mut_tstr()
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
mod tests {}
