#[derive(PartialEq, Clone, Copy, Eq, Debug)]
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

impl From<&str> for BBRole {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "author" => Self::Author,
            "learner" => Self::Learner,
            _ => Self::Learner,
        }
    }
}
