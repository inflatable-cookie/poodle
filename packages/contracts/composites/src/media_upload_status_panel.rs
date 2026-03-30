use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// MediaUploadStatusPanel — workflow status panel for media upload with step transitions.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MediaUploadStep {
    #[default]
    Checking,
    Duplicate,
    Uploading,
    Finalising,
    Complete,
    Error,
}

#[derive(Clone, Debug, Default)]
pub struct MediaUploadStatusPanelSpec {
    pub step: MediaUploadStep,
    pub duplicate_label: Option<String>,
    pub upload_progress: f32,
    pub upload_error: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl MediaUploadStatusPanelSpec {
    pub fn new() -> Self { Self::default() }

    pub fn with_step(mut self, step: MediaUploadStep) -> Self { self.step = step; self }
    pub fn with_duplicate_label(mut self, v: impl Into<String>) -> Self { self.duplicate_label = Some(v.into()); self }
    pub fn with_upload_progress(mut self, v: f32) -> Self { self.upload_progress = v; self }
    pub fn with_upload_error(mut self, v: impl Into<String>) -> Self { self.upload_error = Some(v.into()); self }

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
