use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// MediaBrowsePanel — grid of selectable media items with loading/error/empty states.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MediaBrowseItem {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub meta: Option<String>,
}

impl MediaBrowseItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, kind: impl Into<String>) -> Self {
        Self { id: id.into(), label: label.into(), kind: kind.into(), meta: None }
    }

    pub fn with_meta(mut self, meta: impl Into<String>) -> Self { self.meta = Some(meta.into()); self }
}

#[derive(Clone, Debug, Default)]
pub struct MediaBrowsePanelSpec {
    pub items: Vec<MediaBrowseItem>,
    pub loading: bool,
    pub error: Option<String>,
    pub has_more: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl MediaBrowsePanelSpec {
    pub fn new() -> Self { Self::default() }

    pub fn with_items(mut self, items: Vec<MediaBrowseItem>) -> Self { self.items = items; self }
    pub fn with_loading(mut self, v: bool) -> Self { self.loading = v; self }
    pub fn with_error(mut self, v: impl Into<String>) -> Self { self.error = Some(v.into()); self }
    pub fn with_has_more(mut self, v: bool) -> Self { self.has_more = v; self }

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
