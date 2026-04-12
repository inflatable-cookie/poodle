use poodle_tokens::semantic;
use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileUploadSpec {
    pub accept: Option<String>,
    pub max_size: Option<u64>,
    pub is_multiple: bool,
    pub is_disabled: bool,
    pub is_dragging: bool,
    /// Maximum number of files that can be uploaded in a multi-file
    /// session. Only meaningful when `is_multiple` is true. When None
    /// there is no cap.
    pub max_files: Option<u32>,
    /// When true the component advertises that images will be
    /// compressed client-side before upload. The compression itself
    /// is consumer-owned; this flag only drives the "Will compress
    /// images" helper copy.
    /// Show image thumbnails for uploaded files.
    pub show_preview: bool,
    pub compress: bool,
    /// Pre-computed validation error to display beneath the drop
    /// zone. Mirrors Svelte's `validate` closure returning a
    /// non-null string — here the caller runs their validator and
    /// passes the resolved error (if any).
    pub validation_error: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for FileUploadSpec {
    fn default() -> Self {
        Self {
            accept: None,
            max_size: None,
            is_multiple: false,
            is_disabled: false,
            is_dragging: false,
            max_files: None,
            show_preview: true,
            compress: false,
            validation_error: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
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

    pub fn with_max_files(mut self, max_files: u32) -> Self {
        self.max_files = Some(max_files);
        self
    }

    pub fn with_show_preview(mut self, show_preview: bool) -> Self {
        self.show_preview = show_preview;
        self
    }

    pub fn with_compress(mut self, compress: bool) -> Self {
        self.compress = compress;
        self
    }

    pub fn with_validation_error(mut self, error: impl Into<String>) -> Self {
        self.validation_error = Some(error.into());
        self
    }

    pub fn has_validation_error(&self) -> bool {
        self.validation_error.is_some()
    }

    pub fn error_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
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

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
