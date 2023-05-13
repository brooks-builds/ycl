#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BBTag {
    Unknown,
    Axum,
    NodeJS,
    Yew,
    Rust,
}

impl From<&str> for BBTag {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "axum" => Self::Axum,
            "nodejs" => Self::NodeJS,
            "yew" => Self::Yew,
            "rust" => Self::Rust,
            _ => Self::Unknown,
        }
    }
}
