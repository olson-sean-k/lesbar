#![cfg(feature = "serde")]

use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};
use mitsein::NonEmpty;
use serde_derive::{Deserialize, Serialize};

use crate::Text;

const NON_TEXT_ERROR_MESSAGE: &str =
    "failed to deserialize textual string: no legible text content";

#[derive(Debug, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Serde<T> {
    text: T,
}

impl<T> From<Text<T>> for Serde<NonEmpty<T>> {
    fn from(text: Text<T>) -> Self {
        Serde { text: text.text }
    }
}

impl<T, U> TryFrom<Serde<U>> for Text<T>
where
    Text<T>: TryFrom<U>,
{
    type Error = NonTextError;

    fn try_from(text: Serde<U>) -> Result<Self, Self::Error> {
        Text::try_from(text.text).map_err(|_| NonTextError)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NonTextError;

impl Display for NonTextError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{NON_TEXT_ERROR_MESSAGE}")
    }
}

impl Error for NonTextError {}
