//! Url Encoded List.

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

/// NewType for an encoded list of channels.
///
/// Immutable
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlEncodedList(String);

impl UrlEncodedList {
    /// Create a new [`UrlEncodedList`] from an iterator of [`String`]
    /// values
    pub fn from_str_iter<T, I>(iter: I) -> Self
    where
        T: AsRef<str>,
        I: IntoIterator<Item = T>,
    {
        let iter = iter
            .into_iter()
            .map(|item| utf8_percent_encode(item.as_ref(), NON_ALPHANUMERIC).to_string())
            .collect::<Vec<_>>();
        Self(iter.as_slice().join("%2C"))
    }

    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<T: AsRef<str>, I: IntoIterator<Item = T>> From<I> for UrlEncodedList {
    fn from(vec: I) -> Self {
        Self::from_str_iter(vec.into_iter())
    }
}
