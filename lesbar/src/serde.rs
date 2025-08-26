#![cfg(feature = "serde")]

use mitsein::NonEmpty;
use serde_derive::{Deserialize, Serialize};

use crate::{IllegibleError, Legible};

#[derive(Debug, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Serde<T> {
    text: T,
}

impl<T> From<Legible<T>> for Serde<NonEmpty<T>> {
    fn from(text: Legible<T>) -> Self {
        Serde { text: text.text }
    }
}

impl<T, U> TryFrom<Serde<U>> for Legible<T>
where
    Legible<T>: TryFrom<U, Error = IllegibleError<U>>,
{
    type Error = IllegibleError<U>;

    fn try_from(text: Serde<U>) -> Result<Self, Self::Error> {
        Legible::try_from(text.text)
    }
}

#[cfg(all(test, feature = "alloc"))]
pub mod harness {
    use core::fmt::Debug;
    use rstest::fixture;
    use serde::{Deserialize, Serialize};
    use serde_test::{self, Token};

    use crate::ILLEGIBLE_ERROR_MESSAGE;

    #[fixture]
    pub fn legible() -> impl Iterator<Item = Token> {
        Some(Token::BorrowedStr("legible")).into_iter()
    }

    #[fixture]
    pub fn illegible() -> impl Iterator<Item = Token> {
        Some(Token::BorrowedStr("\u{FEFF}")).into_iter()
    }

    pub fn assert_into_and_from_tokens_eq<T, N>(text: T, tokens: impl IntoIterator<Item = Token>)
    where
        for<'de> T: Debug + Deserialize<'de> + PartialEq + Serialize,
        N: AsRef<[Token]> + FromIterator<Token>,
    {
        let tokens: N = tokens.into_iter().collect();
        serde_test::assert_tokens(&text, tokens.as_ref());
    }

    pub fn assert_deserialize_error_eq_illegible_error<T, N>(
        tokens: impl IntoIterator<Item = Token>,
    ) where
        for<'de> T: Debug + Deserialize<'de> + PartialEq + Serialize,
        N: AsRef<[Token]> + FromIterator<Token>,
    {
        let tokens: N = tokens.into_iter().collect();
        serde_test::assert_de_tokens_error::<T>(tokens.as_ref(), ILLEGIBLE_ERROR_MESSAGE);
    }
}
