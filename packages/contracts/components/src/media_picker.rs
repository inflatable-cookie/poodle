use crate::{ControlDensity, ControlSize, MediaKind, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// Active tab in the media picker (contract §4 `activeTab`). The picker
/// switches between the browse grid and the upload dropzone.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MediaPickerTab {
    #[default]
    Browse,
    Upload,
}

/// A single selectable media item in the browse grid. Mirrors the Svelte
/// `MediaPickerItem` type (`id`, `label`, `thumbnailUrl`, `kind`). The
/// thumbnail bitmap itself is host-owned (preview-loop); `has_thumbnail`
/// drives the placeholder-vs-image anatomy split per contract §2.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPickerItem {
    pub id: String,
    pub label: String,
    /// Whether a real thumbnail bitmap exists. When false the grid renders
    /// the placeholder SVG surface (`.media-picker__thumb--placeholder`).
    pub has_thumbnail: bool,
    pub kind: MediaKind,
}

impl MediaPickerItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, kind: MediaKind) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            has_thumbnail: false,
            kind,
        }
    }

    pub fn with_thumbnail(mut self, has_thumbnail: bool) -> Self {
        self.has_thumbnail = has_thumbnail;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPickerSpec {
    pub title: String,
    pub is_open: bool,
    pub active_tab: MediaPickerTab,
    /// Media items shown in the browse grid (contract `items` prop).
    pub items: Vec<MediaPickerItem>,
    /// File-type filter forwarded to the upload-tab FileUpload (`accept`).
    pub accept: Option<String>,
    /// Max upload file size in bytes forwarded to FileUpload (`maxFileSize`).
    pub max_file_size: Option<u64>,
    /// Message shown in the browse grid when no items are available.
    /// Defaults to `"No media items found."` when None.
    pub empty_message: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl MediaPickerSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            is_open: false,
            active_tab: MediaPickerTab::Browse,
            items: Vec::new(),
            accept: None,
            max_file_size: None,
            empty_message: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_active_tab(mut self, tab: MediaPickerTab) -> Self {
        self.active_tab = tab;
        self
    }

    pub fn with_items(mut self, items: Vec<MediaPickerItem>) -> Self {
        self.items = items;
        self
    }

    pub fn with_item(mut self, item: MediaPickerItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn with_accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    pub fn with_max_file_size(mut self, max_file_size: u64) -> Self {
        self.max_file_size = Some(max_file_size);
        self
    }

    pub fn is_browsing(&self) -> bool {
        self.active_tab == MediaPickerTab::Browse
    }

    pub fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn with_empty_message(mut self, empty_message: impl Into<String>) -> Self {
        self.empty_message = Some(empty_message.into());
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_DIALOG
    }

    // ── Browse-grid token targets (contract §8) ──────────────────

    /// Grid item rest border (transparent) → focus/hover border-focus.
    /// Contract `--poodle-color-border-focus`; resolved via the accent
    /// focus-ring token (matching `media_browse_panel`).
    pub fn item_border_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    /// Grid item hover/focus background and placeholder thumb fill.
    pub fn item_hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn item_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    /// Item label color (`.media-picker__label`).
    pub fn label_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Placeholder SVG glyph color.
    pub fn placeholder_icon_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
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
