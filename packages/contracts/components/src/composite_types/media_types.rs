//! Media / picker / misc composite types. Split out of
//! `composite_types/mod.rs`.

//! Shared composite types used across multiple component specs — not a
//! component spec itself. No corresponding contract file or Svelte component.

use crate::StatusTone;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MediaKind {
    Image,
    Audio,
    Video,
    Document,
    Embed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AspectRatio {
    /// Frame respects the media's intrinsic ratio (contract `data-aspect-ratio="auto"`).
    /// Web uses `aspect-ratio: auto`; with no real media in the Rust targets the frame
    /// falls back to the landscape default for placeholder sizing — see `is_auto()`.
    Auto,
    Square,
    Landscape,
    Portrait,
    Video,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EmptyStateVariant {
    #[default]
    Neutral,
    Search,
    FirstRun,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScrollOwner {
    Shell,
    Content,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableColumnSpec {
    pub id: String,
    pub label: String,
    pub align_end: bool,
    pub is_sortable: bool,
    /// When false the column cannot be hidden via the column-visibility
    /// menu. Matches Svelte `hideable` prop (inverted default).
    pub is_hideable: bool,
    /// Optional explicit width in rem. When None the column flex-grows.
    pub width_rem: Option<f32>,
}

impl TableColumnSpec {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            align_end: false,
            is_sortable: false,
            is_hideable: true,
            width_rem: None,
        }
    }

    pub fn with_align_end(mut self, align_end: bool) -> Self {
        self.align_end = align_end;
        self
    }

    pub fn with_sortable(mut self, is_sortable: bool) -> Self {
        self.is_sortable = is_sortable;
        self
    }

    pub fn with_hideable(mut self, is_hideable: bool) -> Self {
        self.is_hideable = is_hideable;
        self
    }

    pub fn with_width_rem(mut self, width_rem: f32) -> Self {
        self.width_rem = Some(width_rem);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableRowSpec {
    pub id: String,
    pub cells: Vec<(String, String)>,
    pub summary: Option<String>,
    /// When set, the cell value is rendered as a status Pill with the
    /// specified tone instead of plain text. Keyed by column id.
    /// This lets specimens exercise the "custom cell" pattern without
    /// a full slot API.
    pub cell_tones: Vec<(String, StatusTone)>,
}

impl TableRowSpec {
    pub fn new(id: impl Into<String>, cells: Vec<(String, String)>) -> Self {
        Self {
            id: id.into(),
            cells,
            summary: None,
            cell_tones: Vec::new(),
        }
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn with_cell_tone(mut self, column_id: impl Into<String>, tone: StatusTone) -> Self {
        self.cell_tones.push((column_id.into(), tone));
        self
    }

    pub fn cell_tone_for(&self, column_id: &str) -> Option<StatusTone> {
        self.cell_tones
            .iter()
            .find(|(id, _)| id == column_id)
            .map(|(_, tone)| *tone)
    }
}

/// Pagination state for a DataTable — matches Svelte TablePagination.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TablePagination {
    pub page: u32,
    pub limit: u32,
    pub total: u32,
}

impl TablePagination {
    pub fn new(page: u32, limit: u32, total: u32) -> Self {
        Self { page, limit, total }
    }

    /// Total number of pages given the current `limit` and `total`.
    pub fn total_pages(&self) -> u32 {
        if self.limit == 0 {
            0
        } else {
            self.total.div_ceil(self.limit)
        }
    }

    /// 1-based first item index on the current page.
    pub fn first_item(&self) -> u32 {
        if self.total == 0 {
            0
        } else {
            (self.page - 1) * self.limit + 1
        }
    }

    /// 1-based last item index on the current page.
    pub fn last_item(&self) -> u32 {
        (self.page * self.limit).min(self.total)
    }
}

/// Filter state — keyed by column id.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableFilter {
    pub column_id: String,
    pub value: String,
}

impl TableFilter {
    pub fn new(column_id: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            column_id: column_id.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PickerItemSpec {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub meta: Option<String>,
}

impl PickerItemSpec {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            meta: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSummaryItem {
    pub id: String,
    pub label: String,
    pub meta: Option<String>,
}

impl SelectionSummaryItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            meta: None,
        }
    }

    pub fn with_meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiscoveryState {
    Ready,
    Loading,
    Error,
    Empty,
    NoResults,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SplitOrientation {
    Horizontal,
    Vertical,
}

/// When a `SplitView` shows its collapse-toggle pill.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SplitToggleVisibility {
    /// The pill is always on screen.
    #[default]
    Always,
    /// The pill is revealed only while the pointer is on the seam. A collapsed
    /// pane's expand toggle stays visible either way — hiding the only way
    /// back would strand the pane.
    Hover,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
