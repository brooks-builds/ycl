use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Eq, Debug, Deserialize, Serialize)]
pub enum BBRole {
    Learner,
    Author,
}

impl ToString for BBRole {
    fn to_string(&self) -> String {
        match self {
            Self::Author => "Author",
            Self::Learner => "Learner",
        }.to_owned()
    }
}

