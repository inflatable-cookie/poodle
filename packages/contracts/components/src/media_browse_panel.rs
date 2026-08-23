use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// MediaBrowsePanel — grid of selectable media items with loading/error/empty states.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MediaBrowseItem {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub meta: Option<String>,
    pub thumbnail_url: Option<String>,
}

impl MediaBrowseItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: kind.into(),
            meta: None,
            thumbnail_url: None,
        }
    }

    pub fn with_meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn with_thumbnail_url(mut self, thumbnail_url: impl Into<String>) -> Self {
        self.thumbnail_url = Some(thumbnail_url.into());
        self
    }
}

#[derive(Clone, Debug)]
pub struct MediaBrowsePanelSpec {
    pub items: Vec<MediaBrowseItem>,
    pub loading: bool,
    pub error: Option<String>,
    pub has_more: bool,
    pub empty_message: String,
    pub load_more_label: String,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for MediaBrowsePanelSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            loading: false,
            error: None,
            has_more: false,
            empty_message: String::from("No media found"),
            load_more_label: String::from("Load more"),
            size: None,
            size_role: SemanticControlSizeRole::default(),
            density: None,
        }
    }
}

impl MediaBrowsePanelSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_items(mut self, items: Vec<MediaBrowseItem>) -> Self {
        self.items = items;
        self
    }
    pub fn with_loading(mut self, v: bool) -> Self {
        self.loading = v;
        self
    }
    pub fn with_error(mut self, v: impl Into<String>) -> Self {
        self.error = Some(v.into());
        self
    }
    pub fn with_has_more(mut self, v: bool) -> Self {
        self.has_more = v;
        self
    }

    pub fn with_empty_message(mut self, v: impl Into<String>) -> Self {
        self.empty_message = v.into();
        self
    }

    pub fn with_load_more_label(mut self, v: impl Into<String>) -> Self {
        self.load_more_label = v.into();
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    // ── Token methods ──────────────────────────────────────────

    /// Meta and state-copy font-size token. Contract §8 Meta / State `p`:
    /// `0.8125rem` (13px) — the `typography.label.size` token, not caption (11px).
    pub fn meta_font_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }

    /// Item border color token. Contract §8 Item: `0.0625rem solid border-subtle`.
    pub fn item_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// Item radius token. Contract §8 Item: `radius-surface`.
    pub fn item_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    /// Item background base. Contract §8 Item background mixes
    /// `background-panel 92%` with transparent.
    pub fn item_bg_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// Item hover/focus background base. Contract §8 Item `:hover`/`:focus-visible`
    /// mixes `background-elevated 90%` with transparent.
    pub fn item_hover_bg_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    /// Item hover/focus border. Contract §8 Item `:hover`/`:focus-visible`:
    /// `border-color: border-focus`.
    pub fn item_focus_border_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
}
