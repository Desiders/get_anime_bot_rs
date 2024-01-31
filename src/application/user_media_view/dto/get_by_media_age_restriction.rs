#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUserMediaViewByMediaAgeRestriction {
    is_sfw: Option<bool>,
}

impl GetUserMediaViewByMediaAgeRestriction {
    pub const fn new(is_sfw: Option<bool>) -> Self {
        Self { is_sfw }
    }

    pub const fn is_sfw(&self) -> Option<bool> {
        self.is_sfw
    }
}
