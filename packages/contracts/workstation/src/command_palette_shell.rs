use flint_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandPaletteShellSpec {
    pub open: Option<bool>,
    pub default_open: bool,
    pub aria_label: Option<String>,
}

impl Default for CommandPaletteShellSpec {
    fn default() -> Self {
        Self {
            open: None,
            default_open: false,
            aria_label: None,
        }
    }
}

impl CommandPaletteShellSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn backdrop_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_OVERLAY
    }
}
