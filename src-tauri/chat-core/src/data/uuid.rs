use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UUID(String);

impl UUID {
    #[must_use]
    pub fn random() -> UUID {
        uuid::Uuid::new_v4().into()
    }
}

impl From<String> for UUID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for UUID {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<UUID> for String {
    fn from(value: UUID) -> String {
        value.0
    }
}

impl From<uuid::Uuid> for UUID {
    fn from(value: uuid::Uuid) -> Self {
        Self(value.as_hyphenated().to_string())
    }
}

impl Deref for UUID {
    type Target = String;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for UUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
