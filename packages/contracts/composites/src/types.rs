use poodle_primitives::{ButtonVariant, FormActionAlign, StatusTone, ValidationState};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnnouncementMode {
    None,
    Polite,
    Assertive,
}

impl AnnouncementMode {
    pub fn accessibility_role(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Polite => Some("status"),
            Self::Assertive => Some("alert"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormFieldState {
    pub id: String,
    pub label: String,
    pub validation_state: ValidationState,
    pub message: Option<String>,
    pub is_required: bool,
    pub is_disabled: bool,
}

impl FormFieldState {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            validation_state: ValidationState::None,
            message: None,
            is_required: false,
            is_disabled: false,
        }
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn is_blocking(&self) -> bool {
        self.validation_state == ValidationState::Invalid
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormSectionSpec {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub field_ids: Vec<String>,
}

impl FormSectionSpec {
    pub fn new(id: impl Into<String>, title: impl Into<String>, field_ids: Vec<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            field_ids,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationSummaryEntry {
    pub field_id: String,
    pub label: String,
    pub message: String,
    pub validation_state: ValidationState,
}

impl ValidationSummaryEntry {
    pub fn new(
        field_id: impl Into<String>,
        label: impl Into<String>,
        message: impl Into<String>,
        validation_state: ValidationState,
    ) -> Self {
        Self {
            field_id: field_id.into(),
            label: label.into(),
            message: message.into(),
            validation_state,
        }
    }

    pub fn is_blocking(&self) -> bool {
        self.validation_state == ValidationState::Invalid
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemediationAction {
    pub id: String,
    pub label: String,
    pub variant: ButtonVariant,
    pub is_disabled: bool,
}

impl RemediationAction {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Secondary,
            is_disabled: false,
        }
    }

    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormStatusSummary {
    pub tone: StatusTone,
    pub message: String,
}

impl FormStatusSummary {
    pub fn new(tone: StatusTone, message: impl Into<String>) -> Self {
        Self {
            tone,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormActionLayout {
    pub align: FormActionAlign,
    pub action_count: usize,
}

impl FormActionLayout {
    pub fn new(align: FormActionAlign, action_count: usize) -> Self {
        Self {
            align,
            action_count,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TableSortDirection {
    Asc,
    Desc,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrowseState {
    Ready,
    Empty,
    Loading,
    Error,
    NoResults,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MinColumnWidth {
    Sm,
    Md,
    Lg,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PickerVariant {
    Inline,
    Popover,
    Modal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionMode {
    Single,
    Multiple,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MediaState {
    Ready,
    Loading,
    Error,
    Empty,
}

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
    Square,
    Landscape,
    Portrait,
    Video,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmptyStateVariant {
    Neutral,
    Search,
    FirstRun,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScrollOwner {
    Shell,
    Content,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableColumnSpec {
    pub id: String,
    pub label: String,
    pub align_end: bool,
    pub is_sortable: bool,
}

impl TableColumnSpec {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            align_end: false,
            is_sortable: false,
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableRowSpec {
    pub id: String,
    pub cells: Vec<(String, String)>,
    pub summary: Option<String>,
}

impl TableRowSpec {
    pub fn new(id: impl Into<String>, cells: Vec<(String, String)>) -> Self {
        Self {
            id: id.into(),
            cells,
            summary: None,
        }
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
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

