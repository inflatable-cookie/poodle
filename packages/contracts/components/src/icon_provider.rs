#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IconProviderSpec {
    pub icon_set_name: Option<String>,
}

impl IconProviderSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_icon_set_name(mut self, icon_set_name: impl Into<String>) -> Self {
        self.icon_set_name = Some(icon_set_name.into());
        self
    }
}
