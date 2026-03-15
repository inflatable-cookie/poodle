use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiscoveryState {
    Ready,
    Loading,
    Error,
    Empty,
    NoResults,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkspaceShellState {
    Ready,
    Loading,
    Empty,
    Offline,
    Disconnected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SplitOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DockEdge {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandActionItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub group: Option<String>,
    pub shortcut: Option<String>,
    pub keywords: Vec<String>,
    pub badge: Option<String>,
    pub is_disabled: bool,
}

impl CommandActionItem {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            group: None,
            shortcut: None,
            keywords: Vec::new(),
            badge: None,
            is_disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionDiscoverySection {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub actions: Vec<CommandActionItem>,
}

impl ActionDiscoverySection {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        actions: Vec<CommandActionItem>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            actions,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PanelTabItem {
    pub value: String,
    pub label: String,
    pub icon: Option<String>,
    pub is_closable: bool,
}

impl PanelTabItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            is_closable: false,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_closable(mut self, is_closable: bool) -> Self {
        self.is_closable = is_closable;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceTabItem {
    pub value: String,
    pub label: String,
    pub is_closable: bool,
}

impl SurfaceTabItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_closable: false,
        }
    }

    pub fn with_closable(mut self, is_closable: bool) -> Self {
        self.is_closable = is_closable;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DockRegionSnapshot {
    pub edge: DockEdge,
    #[serde(rename = "isCollapsed")]
    pub is_collapsed: bool,
    #[serde(rename = "activePanel")]
    pub active_panel: Option<String>,
    pub order: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceLayoutSnapshot {
    pub version: u8,
    #[serde(rename = "activeSurface")]
    pub active_surface: String,
    #[serde(rename = "surfaceOrder")]
    pub surface_order: Vec<String>,
    #[serde(rename = "primarySplitRatio")]
    pub primary_split_ratio: f32,
    #[serde(rename = "secondarySplitRatio")]
    pub secondary_split_ratio: f32,
    #[serde(rename = "leftDock")]
    pub left_dock: DockRegionSnapshot,
    #[serde(rename = "rightDock")]
    pub right_dock: DockRegionSnapshot,
}
