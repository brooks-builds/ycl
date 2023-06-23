#[derive(PartialEq, Debug, Default, Clone)]
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

#[derive(Default, Debug, Clone, PartialEq)]
pub enum BBValidationState {
    #[default]
    Initialized,
    Valid,
    NotValid,
}

impl From<bool> for BBValidationState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Valid,
            false => Self::NotValid,
        }
    }
}

impl BBValidationState {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Initialized => true,
            Self::Valid => true,
            Self::NotValid => false,
        }
    }

    pub fn not_valid(&self) -> bool {
        match self {
            Self::Initialized => true,
            Self::Valid => false,
            Self::NotValid => true,
        }
    }
}
