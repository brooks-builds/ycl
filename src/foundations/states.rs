#[derive(PartialEq, Debug)]
pub enum BBLoadingState {
    Initialized,
    Loading,
    Loaded,
}

impl BBLoadingState {
    pub fn is_loaded(&self) -> bool {
        *self == Self::Loaded
    }
}
