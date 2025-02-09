//! A printable and non-empty [`String`][`string`].
//!
//! [`string`]: alloc::string

#![cfg(feature = "alloc")]

use alloc::borrow::{Borrow, BorrowMut, Cow};
use alloc::boxed::Box;
use alloc::string::String;
use core::fmt::{self, Debug, Display, Formatter};
use core::ops::{Deref, DerefMut};
use mitsein::string1::String1;

use crate::pstr::PStr;
use crate::{Printable, StrExt as _};

pub type BoxedPStr = Box<PStr>;

pub trait BoxedPStrExt {}

impl BoxedPStrExt for BoxedPStr {}

pub type CowPStr<'a> = Cow<'a, PStr>;

pub trait CowPStrExt<'a> {}

impl<'a> CowPStrExt<'a> for CowPStr<'a> {}

pub type PString = Printable<String>;

impl PString {
    pub const fn from_string1_unchecked(text: String1) -> Self {
        PString { text }
    }

    pub fn into_string1(self) -> String1 {
        self.text
    }

    pub fn leak<'a>(self) -> &'a PStr {
        PStr::from_str1_unchecked(self.text.leak())
    }

    pub fn as_pstr(&self) -> &PStr {
        PStr::from_str1_unchecked(self.text.as_str1())
    }

    pub fn as_mut_pstr(&mut self) -> &mut PStr {
        PStr::from_mut_str1_unchecked(self.text.as_mut_str1())
    }
}

impl AsMut<PStr> for PString {
    fn as_mut(&mut self) -> &mut PStr {
        self.as_mut_pstr()
    }
}

impl AsRef<PStr> for PString {
    fn as_ref(&self) -> &PStr {
        self.as_pstr()
    }
}

impl Borrow<PStr> for PString {
    fn borrow(&self) -> &PStr {
        self.as_pstr()
    }
}

impl BorrowMut<PStr> for PString {
    fn borrow_mut(&mut self) -> &mut PStr {
        self.as_mut_pstr()
    }
}

impl Debug for PString {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.as_str())
    }
}

impl Deref for PString {
    type Target = PStr;

    fn deref(&self) -> &Self::Target {
        self.as_pstr()
    }
}

impl DerefMut for PString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_pstr()
    }
}

impl Display for PString {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl<'a> From<&'a PStr> for PString {
    fn from(text: &'a PStr) -> Self {
        PString::from_string1_unchecked(String1::from(text.as_str1()))
    }
}

impl<'a> TryFrom<&'a str> for PString {
    type Error = &'a str;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        String1::try_from(text).and_then(|text1| PString::try_from(text1).map_err(|_| text))
    }
}

impl TryFrom<String> for PString {
    type Error = String;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        String1::try_from(text)
            .and_then(|text| PString::try_from(text).map_err(String1::into_string))
    }
}

impl TryFrom<String1> for PString {
    type Error = String1;

    fn try_from(text: String1) -> Result<Self, Self::Error> {
        if text.has_printable_text() {
            Ok(PString::from_string1_unchecked(text))
        }
        else {
            Err(text)
        }
    }
}

#[cfg(test)]
mod tests {}
