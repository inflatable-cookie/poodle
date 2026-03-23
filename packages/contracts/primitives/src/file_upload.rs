use flint_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileUploadSpec {
    pub accept: Option<String>,
    pub max_size: Option<u64>,
    pub is_multiple: bool,
    pub is_disabled: bool,
    pub is_dragging: bool,
}

impl Default for FileUploadSpec {
    fn default() -> Self {
        Self {
            accept: None,
            max_size: None,
            is_multiple: false,
            is_disabled: false,
            is_dragging: false,
        }
    }
}

impl FileUploadSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    pub fn with_max_size(mut self, max_size: u64) -> Self {
        self.max_size = Some(max_size);
        self
    }

    pub fn with_multiple(mut self, is_multiple: bool) -> Self {
        self.is_multiple = is_multiple;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_dragging(mut self, is_dragging: bool) -> Self {
        self.is_dragging = is_dragging;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        if self.is_dragging {
            semantic::COLOR_ACCENT_BASE
        } else {
            semantic::COLOR_BACKGROUND_SURFACE
        }
    }

    pub fn border_token(&self) -> &'static str {
        if self.is_dragging {
            semantic::COLOR_ACCENT_BASE
        } else {
            semantic::COLOR_BORDER_DEFAULT
        }
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_border_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
}
