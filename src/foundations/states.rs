#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub enum BBLoadingState {
    #[default]
    Initialized,
    Loading,
    Loaded,
}

impl BBLoadingState {
    pub fn is_loaded(&self) -> bool {
        *self == Self::Loaded
    }
}
