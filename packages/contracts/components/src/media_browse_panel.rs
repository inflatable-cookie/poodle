use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

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
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
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
            size: ControlSize::default(),
            size_role: SemanticControlSizeRole::default(),
            density: ControlDensity::default(),
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
