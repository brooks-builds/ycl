use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Eq, Debug, Deserialize, Serialize, Default)]
pub enum BBRole {
    Learner,
    Author,
    #[default]
    Public,
}

impl ToString for BBRole {
    fn to_string(&self) -> String {
        match self {
            Self::Author => "Author",
            Self::Learner => "Learner",
            Self::Public => "public",
        }
        .to_owned()
    }
}

impl From<String> for BBRole {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "author" => Self::Author,
            "learner" => Self::Learner,
            _ => Self::Public,
        }
    }
}
