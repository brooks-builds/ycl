#[derive(PartialEq, Clone, Copy)]
pub enum AlignText {
    Left,
    Center,
    Right,
}

impl AlignText {
    pub fn class(&self) -> &'static str {
        match self {
            AlignText::Left => "bb-text-start",
            AlignText::Center => "text-center",
            AlignText::Right => "text-end",
        }
    }
}

impl Default for AlignText {
    fn default() -> Self {
        Self::Left
    }
}
