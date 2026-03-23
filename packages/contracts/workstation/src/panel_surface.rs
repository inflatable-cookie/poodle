use flint_primitives::PaddingScale;
use flint_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PanelBodyPadding {
    None,
    Sm,
    Md,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PanelScrollMode {
    Panel,
    Content,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PanelSurfaceSpec {
    pub title: Option<String>,
    pub is_active: bool,
    pub is_elevated: bool,
    pub has_header: bool,
    pub body_padding: PanelBodyPadding,
    pub scroll_mode: PanelScrollMode,
}

impl Default for PanelSurfaceSpec {
    fn default() -> Self {
        Self {
            title: None,
            is_active: false,
            is_elevated: false,
            has_header: true,
            body_padding: PanelBodyPadding::Md,
            scroll_mode: PanelScrollMode::Panel,
        }
    }
}

impl PanelSurfaceSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    pub fn with_elevated(mut self, is_elevated: bool) -> Self {
        self.is_elevated = is_elevated;
        self
    }

    pub fn with_header(mut self, has_header: bool) -> Self {
        self.has_header = has_header;
        self
    }

    pub fn with_body_padding(mut self, body_padding: PanelBodyPadding) -> Self {
        self.body_padding = body_padding;
        self
    }

    pub fn with_scroll_mode(mut self, scroll_mode: PanelScrollMode) -> Self {
        self.scroll_mode = scroll_mode;
        self
    }

    pub fn resolved_padding_scale(&self) -> PaddingScale {
        match self.body_padding {
            PanelBodyPadding::None => PaddingScale::None,
            PanelBodyPadding::Sm => PaddingScale::Sm,
            PanelBodyPadding::Md => PaddingScale::Md,
        }
    }

    pub fn background_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn elevation_token(&self) -> &'static str {
        if self.is_elevated {
            semantic::ELEVATION_OVERLAY
        } else {
            semantic::ELEVATION_SURFACE
        }
    }
}
