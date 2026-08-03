//! Form / validation / table composite types. Split out of
//! `composite_types/mod.rs` (god-file decomposition).

//! Shared composite types used across multiple component specs — not a
//! component spec itself. No corresponding contract file or Svelte component.

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
            let width =
                extract_attribute(trimmed, "width").and_then(|value| value.parse::<u32>().ok());
            let height =
                extract_attribute(trimmed, "height").and_then(|value| value.parse::<u32>().ok());

            return Some(
                Self::new(
                    "generic",
                    src.clone().unwrap_or_else(|| trimmed.to_string()),
                )
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

    if id.is_empty() {
        None
    } else {
        Some(id)
    }
}

fn extract_digits_after(input: &str, needle: &str) -> Option<String> {
    let start = input.find(needle)? + needle.len();
    let suffix = &input[start..];
    let id: String = suffix
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect();

    if id.is_empty() {
        None
    } else {
        Some(id)
    }
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
    (input.starts_with("http://") || input.starts_with("https://"))
        && !input.contains(char::is_whitespace)
}
