use std::str::FromStr;

/// A list of the symbols, prohibited for use in the channel name.
pub const PROHIBITED_SYMBOLS: &[char] = &[','];

/// A channel name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Name(String);

impl Name {
    fn is_valid(s: &str) -> bool {
        !s.contains(PROHIBITED_SYMBOLS)
    }

    #[must_use]
    pub fn from_string_unchecked(s: String) -> Self {
        Self(s)
    }
}

impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !Self::is_valid(&value) {
            Err(value)
        } else {
            Ok(Self(value))
        }
    }
}

impl FromStr for Name {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !Self::is_valid(s) {
            Err(())
        } else {
            Ok(Self(s.to_owned()))
        }
    }
}

impl AsRef<String> for Name {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Name> for String {
    fn from(name: Name) -> Self {
        name.0
    }
}
