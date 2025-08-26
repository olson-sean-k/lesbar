#![cfg(feature = "alloc")]
#![cfg_attr(docsrs, doc(cfg(feature = "alloc")))]

use alloc::borrow::{Borrow, Cow, ToOwned};
use alloc::string::String;
use core::ops::Deref;
use mitsein::borrow1::CowStr1;
use mitsein::str1::Str1;
use mitsein::string1::String1;

use crate::grapheme::Grapheme;
use crate::RuneError;

pub type CowGrapheme<'t> = Cow<'t, Grapheme>;

pub trait CowGraphemeExt<'t> {
    fn into_cow_str1(self) -> CowStr1<'t>;
}

impl<'t> CowGraphemeExt<'t> for CowGrapheme<'t> {
    fn into_cow_str1(self) -> CowStr1<'t> {
        match self {
            Cow::Borrowed(borrowed) => Cow::Borrowed(borrowed.as_str1()),
            Cow::Owned(owned) => Cow::Owned(owned.into()),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GraphemeBuf(String1);

impl GraphemeBuf {
    /// # Safety
    ///
    /// The given string buffer `text` must be non-empty.
    pub unsafe fn from_string_unchecked(text: String) -> Self {
        GraphemeBuf(String1::from_string_unchecked(text))
    }

    pub const fn from_string1_unchecked(text: String1) -> Self {
        GraphemeBuf(text)
    }

    /// # Safety
    ///
    /// The given string slice `text` must be non-empty.
    pub unsafe fn from_str_unchecked(text: &str) -> Self {
        GraphemeBuf::from(Grapheme::from_str_unchecked(text))
    }

    pub fn from_str1_unchecked(text: &Str1) -> Self {
        GraphemeBuf::from(Grapheme::from_str1_unchecked(text))
    }

    pub fn as_grapheme(&self) -> &Grapheme {
        Grapheme::from_str1_unchecked(self.0.as_str1())
    }

    pub fn as_string1(&self) -> &String1 {
        &self.0
    }
}

impl AsRef<Grapheme> for GraphemeBuf {
    fn as_ref(&self) -> &Grapheme {
        self.as_grapheme()
    }
}

impl AsRef<String> for GraphemeBuf {
    fn as_ref(&self) -> &String {
        self.0.as_string()
    }
}

impl AsRef<String1> for GraphemeBuf {
    fn as_ref(&self) -> &String1 {
        &self.0
    }
}

impl Borrow<Grapheme> for GraphemeBuf {
    fn borrow(&self) -> &Grapheme {
        Grapheme::from_str1_unchecked(self.0.as_str1())
    }
}

impl Deref for GraphemeBuf {
    type Target = Grapheme;

    fn deref(&self) -> &Self::Target {
        self.as_grapheme()
    }
}

impl<'t> From<CowGrapheme<'t>> for GraphemeBuf {
    fn from(grapheme: CowGrapheme<'t>) -> Self {
        grapheme.into_owned()
    }
}

impl<'t> From<&'t Grapheme> for GraphemeBuf {
    fn from(grapheme: &'t Grapheme) -> Self {
        grapheme.to_owned()
    }
}

impl From<GraphemeBuf> for String {
    fn from(grapheme: GraphemeBuf) -> Self {
        grapheme.0.into()
    }
}

impl From<GraphemeBuf> for String1 {
    fn from(grapheme: GraphemeBuf) -> Self {
        grapheme.0
    }
}

impl TryFrom<String> for GraphemeBuf {
    type Error = RuneError<String>;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        String1::try_from(text)
            .map_err(RuneError::from_invalid)
            .and_then(|text| {
                GraphemeBuf::try_from(text).map_err(|error| error.map(String1::into_string))
            })
    }
}

impl TryFrom<String1> for GraphemeBuf {
    type Error = RuneError<String1>;

    fn try_from(text: String1) -> Result<Self, Self::Error> {
        if Grapheme::try_from_str1(text.as_str1()).is_ok() {
            Ok(GraphemeBuf(text))
        }
        else {
            Err(RuneError::from_invalid(text))
        }
    }
}
