use crate::{ButtonVariant, FormActionAlign, StatusTone, ValidationState};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedEmbed {
    pub provider: String,
    pub id: String,
    pub original_url: Option<String>,
    pub original_embed: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl ParsedEmbed {
    pub fn new(provider: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            id: id.into(),
            original_url: None,
            original_embed: None,
            width: None,
            height: None,
        }
    }

    pub fn with_original_url(mut self, original_url: impl Into<String>) -> Self {
        self.original_url = Some(original_url.into());
        self
    }

    pub fn with_original_embed(mut self, original_embed: impl Into<String>) -> Self {
        self.original_embed = Some(original_embed.into());
        self
    }

    pub fn with_dimensions(mut self, width: Option<u32>, height: Option<u32>) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn detect(input: &str) -> Option<Self> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return None;
        }

        if let Some(id) = extract_after(trimmed, "youtu.be/") {
            return Some(Self::new("youtube", id).with_original_url(trimmed));
        }

        if let Some(id) = extract_after(trimmed, "youtube.com/watch?v=") {
            return Some(Self::new("youtube", id).with_original_url(trimmed));
        }

        if let Some(id) = extract_after(trimmed, "youtube.com/embed/") {
            return Some(Self::new("youtube", id).with_original_url(trimmed));
        }

        if let Some(id) = extract_digits_after(trimmed, "vimeo.com/") {
            return Some(Self::new("vimeo", id).with_original_url(trimmed));
        }

        if trimmed.starts_with('<') && trimmed.contains("iframe") {
            let src = extract_attribute(trimmed, "src");
            let width = extract_attribute(trimmed, "width").and_then(|value| value.parse::<u32>().ok());
            let height = extract_attribute(trimmed, "height").and_then(|value| value.parse::<u32>().ok());

            return Some(
                Self::new("generic", src.clone().unwrap_or_else(|| trimmed.to_string()))
                    .with_dimensions(width, height)
                    .with_original_embed(trimmed)
                    .with_original_url_opt(src),
            );
        }

        if is_probably_url(trimmed) {
            return Some(Self::new("generic", trimmed).with_original_url(trimmed));
        }

        None
    }

    fn with_original_url_opt(mut self, original_url: Option<String>) -> Self {
        self.original_url = original_url;
        self
    }
}

fn extract_after(input: &str, needle: &str) -> Option<String> {
    let start = input.find(needle)? + needle.len();
    let suffix = &input[start..];
    let id: String = suffix
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == '_')
        .collect();

    if id.is_empty() { None } else { Some(id) }
}

fn extract_digits_after(input: &str, needle: &str) -> Option<String> {
    let start = input.find(needle)? + needle.len();
    let suffix = &input[start..];
    let id: String = suffix.chars().take_while(|ch| ch.is_ascii_digit()).collect();

    if id.is_empty() { None } else { Some(id) }
}

fn extract_attribute(input: &str, attribute: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let pattern = format!("{attribute}={quote}");
        if let Some(start) = input.find(&pattern) {
            let value_start = start + pattern.len();
            let suffix = &input[value_start..];
            if let Some(end) = suffix.find(quote) {
                return Some(suffix[..end].to_string());
            }
        }
    }

    None
}

fn is_probably_url(input: &str) -> bool {
    (input.starts_with("http://") || input.starts_with("https://")) && !input.contains(char::is_whitespace)
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
            (self.total + self.limit - 1) / self.limit
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
