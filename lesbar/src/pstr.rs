//! A printable and non-empty [`str`][prim@str].

#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;
use core::fmt::{self, Debug, Display, Formatter};
use core::iter::Peekable;
use core::mem;
use core::num::NonZeroUsize;
use core::ops::{Deref, DerefMut};
use mitsein::iter1::Iterator1;
use mitsein::str1::Str1;
use unicode_segmentation::{GraphemeIndices, Graphemes, UnicodeSegmentation};
use unicode_width::UnicodeWidthStr;

#[cfg(feature = "alloc")]
use crate::pstring::PString;
use crate::{Printable, StrExt as _};

pub type PStr = Printable<str>;

impl PStr {
    pub const fn from_str1_unchecked(text: &Str1) -> &Self {
        // SAFETY:
        unsafe { mem::transmute::<&'_ Str1, &'_ PStr>(text) }
    }

    pub const fn from_mut_str1_unchecked(text: &mut Str1) -> &mut Self {
        // SAFETY:
        unsafe { mem::transmute::<&'_ mut Str1, &'_ mut PStr>(text) }
    }

    pub fn try_from_str(text: &str) -> Result<&Self, &str> {
        Str1::try_from_str(text).and_then(|text| PStr::try_from_str1(text).map_err(Str1::as_str))
    }

    pub fn try_from_mut_str(text: &mut str) -> Result<&mut Self, &mut str> {
        Str1::try_from_mut_str(text)
            .and_then(|text| PStr::try_from_mut_str1(text).map_err(Str1::as_mut_str))
    }

    pub fn try_from_str1(text: &Str1) -> Result<&Self, &Str1> {
        if text.has_printable_text() {
            Ok(PStr::from_str1_unchecked(text))
        }
        else {
            Err(text)
        }
    }

    pub fn try_from_mut_str1(text: &mut Str1) -> Result<&mut Self, &mut Str1> {
        if text.has_printable_text() {
            Ok(PStr::from_mut_str1_unchecked(text))
        }
        else {
            Err(text)
        }
    }

    #[cfg(feature = "alloc")]
    pub fn into_pstring(&self) -> PString {
        PString::from(self)
    }

    pub fn graphemes1(&self) -> Iterator1<Peekable<Graphemes<'_>>> {
        Iterator1::try_from_iter(self.graphemes(true))
            .expect("printable string has no grapheme clusters")
    }

    pub fn graphemes_indices1(&self) -> Iterator1<Peekable<GraphemeIndices<'_>>> {
        Iterator1::try_from_iter(self.grapheme_indices(true))
            .expect("printable string has no grapheme clusters")
    }

    pub fn width(&self) -> NonZeroUsize {
        NonZeroUsize::new(self.as_str().width()).expect("printable string width is zero")
    }

    pub fn width_cjk(&self) -> NonZeroUsize {
        NonZeroUsize::new(self.as_str().width_cjk()).expect("printable string width is zero")
    }

    pub fn as_str1(&self) -> &Str1 {
        &self.text
    }

    pub fn as_mut_str1(&mut self) -> &mut Str1 {
        &mut self.text
    }

    pub fn as_str(&self) -> &str {
        self.as_str1().as_str()
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        self.as_mut_str1().as_mut_str()
    }
}

impl AsMut<str> for PStr {
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl AsMut<Str1> for PStr {
    fn as_mut(&mut self) -> &mut Str1 {
        self.as_mut_str1()
    }
}

impl AsRef<str> for PStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<Str1> for PStr {
    fn as_ref(&self) -> &Str1 {
        self.as_str1()
    }
}

impl Debug for PStr {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.as_str())
    }
}

impl Deref for PStr {
    type Target = Str1;

    fn deref(&self) -> &Self::Target {
        self.as_str1()
    }
}

impl DerefMut for PStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str1()
    }
}

impl Display for PStr {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl<'a> From<&'a PStr> for &'a str {
    fn from(text: &'a PStr) -> Self {
        text.as_str()
    }
}

impl<'a> From<&'a mut PStr> for &'a mut str {
    fn from(text: &'a mut PStr) -> Self {
        text.as_mut_str()
    }
}

impl<'a> From<&'a PStr> for &'a Str1 {
    fn from(text: &'a PStr) -> Self {
        text.as_str1()
    }
}

impl<'a> From<&'a mut PStr> for &'a mut Str1 {
    fn from(text: &'a mut PStr) -> Self {
        text.as_mut_str1()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl ToOwned for PStr {
    type Owned = PString;

    fn to_owned(&self) -> Self::Owned {
        PString::from(self)
    }
}

impl<'a> TryFrom<&'a str> for &'a PStr {
    type Error = &'a str;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        PStr::try_from_str(text)
    }
}

impl<'a> TryFrom<&'a mut str> for &'a mut PStr {
    type Error = &'a mut str;

    fn try_from(text: &'a mut str) -> Result<Self, Self::Error> {
        PStr::try_from_mut_str(text)
    }
}

impl<'a> TryFrom<&'a Str1> for &'a PStr {
    type Error = &'a Str1;

    fn try_from(text: &'a Str1) -> Result<Self, Self::Error> {
        PStr::try_from_str1(text)
    }
}

impl<'a> TryFrom<&'a mut Str1> for &'a mut PStr {
    type Error = &'a mut Str1;

    fn try_from(text: &'a mut Str1) -> Result<Self, Self::Error> {
        PStr::try_from_mut_str1(text)
    }
}

#[cfg(test)]
mod tests {}
