use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// Upload lifecycle of a single file in the list. Mirrors the Svelte
/// `FileUploadItem.status` union (`"pending" | "uploading" | "complete"
/// | "error"`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileUploadStatus {
    #[default]
    Pending,
    Uploading,
    Complete,
    Error,
}

/// A single file row rendered in the upload list. Mirrors the Svelte
/// `FileUploadItem` interface — the Rust side carries the
/// already-resolved display fields (name, byte size, progress, status,
/// preview flag, error) rather than a live `File` handle, since the
/// component is render-only and the host owns the actual file bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileUploadItem {
    /// Stable identity (host-assigned) used as the row key.
    pub id: String,
    /// Display file name (shown in `.file-upload__name`, truncated).
    pub name: String,
    /// File size in bytes (formatted for `.file-upload__size`).
    pub size: u64,
    /// Upload progress 0..=100. Only meaningful while `Uploading`.
    pub progress: u8,
    pub status: FileUploadStatus,
    /// When true the row is an image with an available preview thumbnail
    /// (`.file-upload__preview`); when false it renders the generic file
    /// icon (`.file-upload__file-icon`). The actual bitmap is host-owned
    /// (preview-loop) — this flag only drives which anatomy part shows.
    pub has_preview: bool,
    /// Per-file error message shown in `.file-upload__error-text` when
    /// `status == Error`.
    pub error: Option<String>,
}

impl FileUploadItem {
    pub fn new(id: impl Into<String>, name: impl Into<String>, size: u64) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            size,
            progress: 0,
            status: FileUploadStatus::Pending,
            has_preview: false,
            error: None,
        }
    }

    pub fn with_progress(mut self, progress: u8) -> Self {
        self.progress = progress.min(100);
        self.status = if self.progress >= 100 {
            FileUploadStatus::Complete
        } else {
            FileUploadStatus::Uploading
        };
        self
    }

    pub fn with_status(mut self, status: FileUploadStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_preview(mut self, has_preview: bool) -> Self {
        self.has_preview = has_preview;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self.status = FileUploadStatus::Error;
        self
    }

    pub fn is_error(&self) -> bool {
        self.status == FileUploadStatus::Error
    }

    pub fn is_uploading(&self) -> bool {
        self.status == FileUploadStatus::Uploading
    }

    pub fn is_complete(&self) -> bool {
        self.status == FileUploadStatus::Complete
    }

    /// Human-readable byte size (`bytes` / `KB` / `MB`) matching Svelte's
    /// `formatFileSize` thresholds.
    pub fn formatted_size(&self) -> String {
        format_file_size(self.size)
    }
}

/// Format a byte count as `bytes` / `KB` / `MB`, matching the Svelte
/// `formatFileSize` helper used by both the file list and the dropzone
/// hint.
pub fn format_file_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    let b = bytes as f64;
    if b < KB {
        format!("{bytes} bytes")
    } else if b < MB {
        format!("{:.1} KB", b / KB)
    } else {
        format!("{:.1} MB", b / MB)
    }
}

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
    /// Files currently in the upload list. Drives the `.file-upload__list`
    /// anatomy (File Item / Preview / Meta / Progress / Remove). Empty by
    /// default — the list is conditional on `files.len() > 0`.
    pub files: Vec<FileUploadItem>,
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
            files: Vec::new(),
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

    pub fn with_files(mut self, files: Vec<FileUploadItem>) -> Self {
        self.files = files;
        self
    }

    pub fn with_file(mut self, file: FileUploadItem) -> Self {
        self.files.push(file);
        self
    }

    pub fn has_files(&self) -> bool {
        !self.files.is_empty()
    }

    pub fn error_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    // ── File-list item token targets (contract §8) ───────────────

    /// File item row background (`.file-upload__item`).
    pub fn item_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// Danger base used for the error-item background mix and error text
    /// (`.file-upload__item--error`, `.file-upload__error-text`).
    pub fn item_error_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    /// Surface base mixed at 82% for the file icon / progress track /
    /// remove-hover backgrounds.
    pub fn item_surface_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    /// File-icon glyph + remove-button rest color.
    pub fn item_icon_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// File name color (`.file-upload__name`).
    pub fn item_name_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// File size / hint color (`.file-upload__size`).
    pub fn item_size_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }

    /// Progress-bar fill (`.file-upload__progress-bar`).
    pub fn progress_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn item_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
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
