//! Non-empty [string][prim@str] types that represent legible text.

mod buf;

#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;
use core::fmt::{self, Debug, Display, Formatter};
use core::iter::Peekable;
use core::mem;
use core::ops::{Deref, DerefMut};
use mitsein::iter1::Iterator1;
use mitsein::str1::Str1;

use crate::iter::{GraphemeIndices, Graphemes};
use crate::{IllegibleError, Legible, StrExt as _};

#[cfg(feature = "alloc")]
pub use crate::text::buf::*;

pub type Text = Legible<str>;

impl Text {
    pub const fn from_str1_unchecked(text: &Str1) -> &Self {
        // SAFETY: `Legible` is `repr(transparent)`: `Str1` and `Text` have the same
        //         representation.
        unsafe { mem::transmute::<&'_ Str1, &'_ Text>(text) }
    }

    pub const fn from_mut_str1_unchecked(text: &mut Str1) -> &mut Self {
        // SAFETY: `Legible` is `repr(transparent)`: `Str1` and `Text` have the same
        //         representation.
        unsafe { mem::transmute::<&'_ mut Str1, &'_ mut Text>(text) }
    }

    pub fn try_from_str(text: &str) -> Result<&Self, IllegibleError<&str>> {
        Str1::try_from_str(text)
            .map_err(IllegibleError::from_illegible)
            .and_then(|text1| Text::try_from_str1(text1).map_err(|error| error.map(Str1::as_str)))
    }

    pub fn try_from_mut_str(text: &mut str) -> Result<&mut Self, IllegibleError<&mut str>> {
        Str1::try_from_mut_str(text)
            .map_err(IllegibleError::from_illegible)
            .and_then(|text| {
                Text::try_from_mut_str1(text).map_err(|error| error.map(Str1::as_mut_str))
            })
    }

    pub fn try_from_str1(text: &Str1) -> Result<&Self, IllegibleError<&Str1>> {
        if text.has_text() {
            Ok(Text::from_str1_unchecked(text))
        }
        else {
            Err(IllegibleError::from_illegible(text))
        }
    }

    pub fn try_from_mut_str1(text: &mut Str1) -> Result<&mut Self, IllegibleError<&mut Str1>> {
        if text.has_text() {
            Ok(Text::from_mut_str1_unchecked(text))
        }
        else {
            Err(IllegibleError::from_illegible(text))
        }
    }

    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn into_text_buf(self: BoxedText) -> TextBuf {
        TextBuf::from(self)
    }

    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn to_text_buf(&self) -> TextBuf {
        TextBuf::from(self)
    }

    pub fn graphemes1(&self) -> Iterator1<Peekable<Graphemes<'_>>> {
        Iterator1::try_from_iter(self.graphemes()).expect("text has no grapheme clusters")
    }

    pub fn grapheme_indices1(&self) -> Iterator1<Peekable<GraphemeIndices<'_>>> {
        Iterator1::try_from_iter(self.grapheme_indices()).expect("text has no grapheme clusters")
    }

    pub const fn as_str1(&self) -> &Str1 {
        &self.text
    }

    pub const fn as_mut_str1(&mut self) -> &mut Str1 {
        &mut self.text
    }

    pub const fn as_str(&self) -> &str {
        self.as_str1().as_str()
    }

    pub const fn as_mut_str(&mut self) -> &mut str {
        self.as_mut_str1().as_mut_str()
    }
}

impl AsMut<str> for Text {
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl AsMut<Str1> for Text {
    fn as_mut(&mut self) -> &mut Str1 {
        self.as_mut_str1()
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<Str1> for Text {
    fn as_ref(&self) -> &Str1 {
        self.as_str1()
    }
}

impl Debug for Text {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.as_str())
    }
}

impl Deref for Text {
    type Target = Str1;

    fn deref(&self) -> &Self::Target {
        self.as_str1()
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str1()
    }
}

impl Display for Text {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl<'a> From<&'a Text> for &'a str {
    fn from(text: &'a Text) -> Self {
        text.as_str()
    }
}

impl<'a> From<&'a mut Text> for &'a mut str {
    fn from(text: &'a mut Text) -> Self {
        text.as_mut_str()
    }
}

impl<'a> From<&'a Text> for &'a Str1 {
    fn from(text: &'a Text) -> Self {
        text.as_str1()
    }
}

impl<'a> From<&'a mut Text> for &'a mut Str1 {
    fn from(text: &'a mut Text) -> Self {
        text.as_mut_str1()
    }
}

impl<T> PartialEq<&'_ T> for Text
where
    Text: PartialEq<T>,
    T: ?Sized,
{
    fn eq(&self, other: &&'_ T) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl PartialEq<Str1> for Text {
    fn eq(&self, other: &Str1) -> bool {
        self.as_str1().eq(other)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl ToOwned for Text {
    type Owned = TextBuf;

    fn to_owned(&self) -> Self::Owned {
        TextBuf::from(self)
    }
}

impl<'a> TryFrom<&'a str> for &'a Text {
    type Error = IllegibleError<&'a str>;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        Text::try_from_str(text)
    }
}

impl<'a> TryFrom<&'a mut str> for &'a mut Text {
    type Error = IllegibleError<&'a mut str>;

    fn try_from(text: &'a mut str) -> Result<Self, Self::Error> {
        Text::try_from_mut_str(text)
    }
}

impl<'a> TryFrom<&'a Str1> for &'a Text {
    type Error = IllegibleError<&'a Str1>;

    fn try_from(text: &'a Str1) -> Result<Self, Self::Error> {
        Text::try_from_str1(text)
    }
}

impl<'a> TryFrom<&'a mut Str1> for &'a mut Text {
    type Error = IllegibleError<&'a mut Str1>;

    fn try_from(text: &'a mut Str1) -> Result<Self, Self::Error> {
        Text::try_from_mut_str1(text)
    }
}

#[cfg(test)]
mod tests {}
