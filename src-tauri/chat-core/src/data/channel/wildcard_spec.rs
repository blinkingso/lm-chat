use crate::data::channel;
use std::str::FromStr;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct WildcardSpec(String);

/// Currently you can have up to three levels deep with your channel segment
/// hierarchies, `a.b.c`, for example.
impl WildcardSpec {
    fn is_valid(s: &str) -> bool {
        if s.starts_with('.') {
            return false;
        }

        // A simple state machine
        {
            let mut was_dot = false;
            let mut was_asterisk = false;
            let mut dots_count = 0;
            for c in s.chars() {
                if was_asterisk {
                    return false;
                }

                was_asterisk = false;
                if was_dot {
                    if c == '*' {
                        was_asterisk = true;
                    }
                } else if c == '*' {
                    return false;
                }
                let is_dot = c == '.';
                if is_dot {
                    dots_count += 1;
                    if dots_count > 2 {
                        return false;
                    }
                }

                was_dot = is_dot;
            }

            if was_dot {
                return false;
            }
        }

        true
    }

    #[must_use]
    pub fn from_string_unchecked(s: String) -> Self {
        Self(s)
    }
}

impl TryFrom<String> for WildcardSpec {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !Self::is_valid(&value) {
            return Err(value);
        }

        Ok(Self(value))
    }
}

impl FromStr for WildcardSpec {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !Self::is_valid(s) {
            return Err(());
        }

        Ok(Self(s.to_owned()))
    }
}

impl AsRef<String> for WildcardSpec {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsRef<str> for WildcardSpec {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for WildcardSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<channel::Name> for WildcardSpec {
    fn from(name: channel::Name) -> Self {
        Self::from_string_unchecked(name.into())
    }
}
