//! Model-connection display types and pure helpers. Mirror of core
//! `model-connection.ts`.
//!
//! Poodle receives presentation records with opaque ids and safe labels. No
//! type here carries a credential, credential reference, raw probe output,
//! filesystem evidence, executable handle, target, or account identifier —
//! the same rule the TypeScript owner states, restated where the Rust targets
//! read it.
//!
//! Contracts: `docs/contracts/components/model-connection-picker.md`,
//! `model-connection-setup.md`, `model-connection-card.md`,
//! `model-catalogue-editor.md`.
//!
//! The vectors below are owner-local: each Rust case names the same inputs and
//! observable outputs as the matching case in
//! `packages/core/test/model-connection.test.ts`. Duplicated explicit cases
//! are the point — there is no shared corpus, schema, or comparator.

// ── Shared display vocabulary ────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelConnectionBadgeTone {
    #[default]
    Neutral,
    Info,
    Success,
    Warning,
    Danger,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModelConnectionBadge {
    pub label: String,
    pub tone: ModelConnectionBadgeTone,
}

impl ModelConnectionBadge {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            tone: ModelConnectionBadgeTone::Neutral,
        }
    }

    pub fn with_tone(mut self, tone: ModelConnectionBadgeTone) -> Self {
        self.tone = tone;
        self
    }
}

/// Host-supplied classification. Poodle groups and disables by it; it never
/// probes, detects, or derives it.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelConnectionAvailability {
    #[default]
    Available,
    Checking,
    Unavailable,
    Unsupported,
}

/// Catalogue posture supplied by the host.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelConnectionPickerState {
    #[default]
    Ready,
    Loading,
    Error,
    Empty,
    NoResults,
}

/// PickerShell browse-state spelling for `NoResults`. Kept distinct from
/// [`ModelConnectionPickerState`] because the resolution collapses supplied
/// posture, source count, match count, and query into one shell state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelConnectionPickerShellState {
    #[default]
    Ready,
    Loading,
    Error,
    Empty,
    NoResults,
}

/// One exact connection route. `id` is opaque: it is the only thing a
/// selection ever emits.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModelConnectionOption {
    pub id: String,
    pub provider_label: String,
    pub route_label: Option<String>,
    pub description: Option<String>,
    pub group: String,
    pub keywords: Vec<String>,
    pub availability: ModelConnectionAvailability,
    pub availability_label: String,
    pub is_disabled: bool,
    pub requires_configuration: bool,
}

impl ModelConnectionOption {
    pub fn new(
        id: impl Into<String>,
        provider_label: impl Into<String>,
        group: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            provider_label: provider_label.into(),
            route_label: None,
            description: None,
            group: group.into(),
            keywords: Vec::new(),
            availability: ModelConnectionAvailability::Available,
            availability_label: "Available".to_string(),
            is_disabled: false,
            requires_configuration: false,
        }
    }

    pub fn with_route_label(mut self, route_label: impl Into<String>) -> Self {
        self.route_label = Some(route_label.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_keywords<I, S>(mut self, keywords: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_availability(
        mut self,
        availability: ModelConnectionAvailability,
        availability_label: impl Into<String>,
    ) -> Self {
        self.availability = availability;
        self.availability_label = availability_label.into();
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_requires_configuration(mut self, requires_configuration: bool) -> Self {
        self.requires_configuration = requires_configuration;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelConnectionSetupStage {
    #[default]
    Choose,
    Configure,
}

/// Display posture of one configured connection. Poodle never derives it.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelConnectionReadiness {
    Ready,
    Checking,
    Attention,
    Unavailable,
    #[default]
    Unknown,
    Error,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelCatalogueState {
    #[default]
    Ready,
    Loading,
    Unavailable,
    Empty,
    Error,
    SessionNegotiated,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModelCatalogueItem {
    pub id: String,
    pub label: String,
    pub provider_label: Option<String>,
    pub description: Option<String>,
    pub badges: Vec<ModelConnectionBadge>,
    pub visible: bool,
    pub is_disabled: bool,
}

impl ModelCatalogueItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            provider_label: None,
            description: None,
            badges: Vec::new(),
            visible: true,
            is_disabled: false,
        }
    }

    pub fn with_provider_label(mut self, provider_label: impl Into<String>) -> Self {
        self.provider_label = Some(provider_label.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_badge(mut self, badge: ModelConnectionBadge) -> Self {
        self.badges.push(badge);
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

/// The complete visibility request payload: an exact id and the requested
/// state. Nothing else crosses the boundary, and nothing is applied locally.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalogueVisibilityChange {
    pub id: String,
    pub visible: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModelConnectionStatusTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModelConnectionOptionGroup {
    pub group: String,
    pub options: Vec<ModelConnectionOption>,
}

/// Title plus supporting message for a non-ready posture. `message` is empty
/// for the ready posture, exactly as the web owner returns it.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModelConnectionStateCopy {
    pub title: String,
    pub message: String,
}

// ── Picker filtering ─────────────────────────────────────────────────────

fn case_fold(value: &str) -> String {
    value.to_lowercase()
}

fn option_matches_query(option: &ModelConnectionOption, folded_query: &str) -> bool {
    if folded_query.is_empty() {
        return true;
    }

    if case_fold(&option.provider_label).contains(folded_query)
        || case_fold(option.route_label.as_deref().unwrap_or("")).contains(folded_query)
        || case_fold(option.description.as_deref().unwrap_or("")).contains(folded_query)
        || case_fold(&option.group).contains(folded_query)
    {
        return true;
    }

    option
        .keywords
        .iter()
        .any(|keyword| case_fold(keyword).contains(folded_query))
}

/// Case-folded filter across provider, route, description, group, and
/// keywords. Retains host source order — ranking belongs to the host.
pub fn filter_model_connection_options(
    options: &[ModelConnectionOption],
    query: &str,
) -> Vec<ModelConnectionOption> {
    let folded = case_fold(query.trim());
    if folded.is_empty() {
        return options.to_vec();
    }
    options
        .iter()
        .filter(|option| option_matches_query(option, &folded))
        .cloned()
        .collect()
}

/// Group filtered options while retaining first-seen group order and source
/// order within each group.
pub fn group_model_connection_options(
    options: &[ModelConnectionOption],
) -> Vec<ModelConnectionOptionGroup> {
    let mut groups: Vec<ModelConnectionOptionGroup> = Vec::new();

    for option in options {
        match groups.iter_mut().find(|entry| entry.group == option.group) {
            Some(entry) => entry.options.push(option.clone()),
            None => groups.push(ModelConnectionOptionGroup {
                group: option.group.clone(),
                options: vec![option.clone()],
            }),
        }
    }

    groups
}

pub fn model_connection_option_selectable(option: &ModelConnectionOption) -> bool {
    !option.is_disabled && option.availability == ModelConnectionAvailability::Available
}

pub fn resolve_model_connection_picker_shell_state(
    state: ModelConnectionPickerState,
    source_count: usize,
    match_count: usize,
    query: &str,
) -> ModelConnectionPickerShellState {
    match state {
        ModelConnectionPickerState::Loading => return ModelConnectionPickerShellState::Loading,
        ModelConnectionPickerState::Error => return ModelConnectionPickerShellState::Error,
        _ => {}
    }
    if state == ModelConnectionPickerState::Empty || source_count == 0 {
        return ModelConnectionPickerShellState::Empty;
    }
    if state == ModelConnectionPickerState::NoResults
        || (!query.trim().is_empty() && match_count == 0)
    {
        return ModelConnectionPickerShellState::NoResults;
    }
    ModelConnectionPickerShellState::Ready
}

pub fn model_connection_picker_result_announcement(match_count: usize, query: &str) -> String {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return if match_count == 1 {
            "1 connection".to_string()
        } else {
            format!("{match_count} connections")
        };
    }
    if match_count == 0 {
        return format!("No connections match \u{201c}{trimmed}\u{201d}");
    }
    if match_count == 1 {
        format!("1 connection matches \u{201c}{trimmed}\u{201d}")
    } else {
        format!("{match_count} connections match \u{201c}{trimmed}\u{201d}")
    }
}

pub fn model_connection_picker_state_copy(
    state: ModelConnectionPickerShellState,
    query: &str,
) -> ModelConnectionStateCopy {
    let copy = |title: &str, message: String| ModelConnectionStateCopy {
        title: title.to_string(),
        message,
    };

    match state {
        ModelConnectionPickerShellState::Loading => copy(
            "Loading connections",
            "Checking the available connection routes.".to_string(),
        ),
        ModelConnectionPickerShellState::Error => copy(
            "Could not load connections",
            "Try again from the host application.".to_string(),
        ),
        ModelConnectionPickerShellState::Empty => copy(
            "No connections available",
            "Add or detect a supported connection route first.".to_string(),
        ),
        ModelConnectionPickerShellState::NoResults => {
            let trimmed = query.trim();
            copy(
                "No matching connections",
                if trimmed.is_empty() {
                    "Try a different search.".to_string()
                } else {
                    format!("No connections match \u{201c}{trimmed}\u{201d}.")
                },
            )
        }
        ModelConnectionPickerShellState::Ready => copy("Choose a connection", String::new()),
    }
}

// ── Setup stage guards ───────────────────────────────────────────────────

/// The controlled inputs a setup transition reads. The host owns every field
/// and replaces them after acting on the emitted effects.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModelConnectionSetupContext {
    pub stage: ModelConnectionSetupStage,
    pub value: Option<String>,
    pub query: String,
    pub options: Vec<ModelConnectionOption>,
    pub can_submit: bool,
    pub is_pending: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModelConnectionSetupEvent {
    Select { id: String },
    SetValue { id: Option<String> },
    SetQuery { query: String },
    SetStage { stage: ModelConnectionSetupStage },
    Continue,
    Back,
    Submit,
    Cancel,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModelConnectionSetupEffect {
    EmitValueChange { id: String },
    EmitQueryChange { query: String },
    EmitStageChange { stage: ModelConnectionSetupStage },
    EmitSubmit { id: String },
    EmitCancel,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelConnectionSetupResult {
    pub context: ModelConnectionSetupContext,
    pub effects: Vec<ModelConnectionSetupEffect>,
}

fn find_option<'a>(
    options: &'a [ModelConnectionOption],
    id: Option<&str>,
) -> Option<&'a ModelConnectionOption> {
    let id = id?;
    options.iter().find(|option| option.id == id)
}

/// The option the current selection names, if the host still supplies it.
pub fn model_connection_setup_selected_option(
    context: &ModelConnectionSetupContext,
) -> Option<&ModelConnectionOption> {
    find_option(&context.options, context.value.as_deref())
}

pub fn model_connection_setup_can_continue(context: &ModelConnectionSetupContext) -> bool {
    if context.is_pending || context.value.is_none() {
        return false;
    }
    find_option(&context.options, context.value.as_deref())
        .is_some_and(model_connection_option_selectable)
}

pub fn model_connection_setup_can_submit(context: &ModelConnectionSetupContext) -> bool {
    if context.is_pending || !context.can_submit {
        return false;
    }
    let Some(option) = find_option(&context.options, context.value.as_deref()) else {
        return false;
    };
    if context.stage == ModelConnectionSetupStage::Choose {
        return !option.requires_configuration && model_connection_option_selectable(option);
    }
    true
}

pub fn model_connection_setup_transition(
    context: ModelConnectionSetupContext,
    event: ModelConnectionSetupEvent,
) -> ModelConnectionSetupResult {
    let stay = |context: ModelConnectionSetupContext| ModelConnectionSetupResult {
        context,
        effects: Vec::new(),
    };

    match event {
        // Programmatic replacement is not a workflow event: it stays open
        // while pending, exactly like the web owner.
        ModelConnectionSetupEvent::SetValue { id } => stay(ModelConnectionSetupContext {
            value: id,
            ..context
        }),

        ModelConnectionSetupEvent::SetStage { stage } => {
            stay(ModelConnectionSetupContext { stage, ..context })
        }

        ModelConnectionSetupEvent::SetQuery { query } => {
            if context.is_pending {
                return stay(context);
            }
            ModelConnectionSetupResult {
                effects: vec![ModelConnectionSetupEffect::EmitQueryChange {
                    query: query.clone(),
                }],
                context: ModelConnectionSetupContext { query, ..context },
            }
        }

        ModelConnectionSetupEvent::Select { id } => {
            if context.is_pending {
                return stay(context);
            }
            match find_option(&context.options, Some(id.as_str())) {
                Some(option) if model_connection_option_selectable(option) => {}
                _ => return stay(context),
            }
            if context.value.as_deref() == Some(id.as_str()) {
                return stay(context);
            }
            ModelConnectionSetupResult {
                effects: vec![ModelConnectionSetupEffect::EmitValueChange { id: id.clone() }],
                context: ModelConnectionSetupContext {
                    value: Some(id),
                    ..context
                },
            }
        }

        ModelConnectionSetupEvent::Continue => {
            if !model_connection_setup_can_continue(&context) {
                return stay(context);
            }
            ModelConnectionSetupResult {
                context: ModelConnectionSetupContext {
                    stage: ModelConnectionSetupStage::Configure,
                    ..context
                },
                effects: vec![ModelConnectionSetupEffect::EmitStageChange {
                    stage: ModelConnectionSetupStage::Configure,
                }],
            }
        }

        ModelConnectionSetupEvent::Back => {
            if context.is_pending || context.stage != ModelConnectionSetupStage::Configure {
                return stay(context);
            }
            ModelConnectionSetupResult {
                context: ModelConnectionSetupContext {
                    stage: ModelConnectionSetupStage::Choose,
                    ..context
                },
                effects: vec![ModelConnectionSetupEffect::EmitStageChange {
                    stage: ModelConnectionSetupStage::Choose,
                }],
            }
        }

        ModelConnectionSetupEvent::Submit => {
            if !model_connection_setup_can_submit(&context) {
                return stay(context);
            }
            let Some(id) = context.value.clone() else {
                return stay(context);
            };
            ModelConnectionSetupResult {
                context,
                effects: vec![ModelConnectionSetupEffect::EmitSubmit { id }],
            }
        }

        ModelConnectionSetupEvent::Cancel => {
            if context.is_pending {
                return stay(context);
            }
            ModelConnectionSetupResult {
                context,
                effects: vec![ModelConnectionSetupEffect::EmitCancel],
            }
        }
    }
}

/// The safe summary the configure stage shows for the selected route. It
/// repeats supplied labels; it resolves nothing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelConnectionSelectedSummary {
    pub id: String,
    pub title: String,
    pub provider_label: String,
    pub route_label: Option<String>,
}

pub fn model_connection_selected_summary(
    option: Option<&ModelConnectionOption>,
) -> Option<ModelConnectionSelectedSummary> {
    option.map(|option| ModelConnectionSelectedSummary {
        id: option.id.clone(),
        title: option.provider_label.clone(),
        provider_label: option.provider_label.clone(),
        route_label: option.route_label.clone(),
    })
}

// ── Catalogue order and visibility ───────────────────────────────────────

pub fn shown_model_catalogue_items(items: &[ModelCatalogueItem]) -> Vec<ModelCatalogueItem> {
    items.iter().filter(|item| item.visible).cloned().collect()
}

pub fn hidden_model_catalogue_items(items: &[ModelCatalogueItem]) -> Vec<ModelCatalogueItem> {
    items.iter().filter(|item| !item.visible).cloned().collect()
}

/// Complete shown-id order after moving one shown index to another, or `None`
/// when the move is a no-op or out of range. The payload is always the whole
/// order — a delta would make the host reconstruct state Poodle already has.
pub fn request_model_catalogue_order(
    shown_ids: &[String],
    from_index: usize,
    to_index: usize,
) -> Option<Vec<String>> {
    if from_index >= shown_ids.len() || to_index >= shown_ids.len() || from_index == to_index {
        return None;
    }

    let mut next = shown_ids.to_vec();
    let moved = next.remove(from_index);
    next.insert(to_index, moved);
    Some(next)
}

pub fn request_model_catalogue_visibility(
    id: &str,
    visible: bool,
) -> ModelCatalogueVisibilityChange {
    ModelCatalogueVisibilityChange {
        id: id.to_string(),
        visible,
    }
}

/// Where focus goes after a shown model is hidden: the next shown model, else
/// the previous one, else the hidden-section disclosure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModelCatalogueFocusAfterHide {
    Shown { id: String },
    HiddenSection,
}

pub fn model_catalogue_focus_after_hide(
    shown_ids: &[String],
    hidden_id: &str,
) -> ModelCatalogueFocusAfterHide {
    let Some(index) = shown_ids.iter().position(|id| id == hidden_id) else {
        return ModelCatalogueFocusAfterHide::HiddenSection;
    };
    let next = shown_ids
        .get(index + 1)
        .or_else(|| index.checked_sub(1).and_then(|prev| shown_ids.get(prev)));
    match next {
        Some(id) => ModelCatalogueFocusAfterHide::Shown { id: id.clone() },
        None => ModelCatalogueFocusAfterHide::HiddenSection,
    }
}

pub fn model_catalogue_reorder_announcement(label: &str, position: usize, total: usize) -> String {
    format!("Moved {label} to position {position} of {total}.")
}

pub fn model_catalogue_visibility_announcement(label: &str, visible: bool) -> String {
    if visible {
        format!("Restored {label}.")
    } else {
        format!("Hid {label}.")
    }
}

/// Accessible-reorder keydown resolution. Mirror of core `listReorderKeyIntent`
/// (`packages/core/src/edit.ts`), which the ModelCatalogueEditor contract names
/// as a machinery dependency.
///
/// The Rust core has no shared `edit` module yet, so the mirror lives with its
/// only Rust consumer rather than inventing a home for one caller. Move it out
/// when a second Rust component needs it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelCatalogueKeyIntent {
    Grab,
    Drop,
    CancelGrab,
    Move { from: usize, to: usize },
    Boundary,
}

pub fn model_catalogue_reorder_key_intent(
    key: &str,
    index: usize,
    grabbed_index: Option<usize>,
    item_count: usize,
) -> Option<ModelCatalogueKeyIntent> {
    if key == " " || key == "Enter" {
        return Some(if grabbed_index == Some(index) {
            ModelCatalogueKeyIntent::Drop
        } else {
            ModelCatalogueKeyIntent::Grab
        });
    }

    if key == "Escape" {
        return grabbed_index.map(|_| ModelCatalogueKeyIntent::CancelGrab);
    }

    if key != "ArrowUp" && key != "ArrowDown" {
        return None;
    }

    let active_index = grabbed_index.unwrap_or(index);
    let target_index = if key == "ArrowUp" {
        match active_index.checked_sub(1) {
            Some(target) => target,
            None => return Some(ModelCatalogueKeyIntent::Boundary),
        }
    } else {
        active_index + 1
    };

    if target_index >= item_count {
        return Some(ModelCatalogueKeyIntent::Boundary);
    }

    Some(ModelCatalogueKeyIntent::Move {
        from: active_index,
        to: target_index,
    })
}

/// Grab announcement copy, shared by the keyboard intent and the handle's
/// pointer activation so both routes say the same thing.
pub fn model_catalogue_grab_announcement(label: &str) -> String {
    format!("Grabbed {label}. Use arrow keys to move, Escape to cancel.")
}

pub const MODEL_CATALOGUE_DROP_ANNOUNCEMENT: &str = "Dropped item.";
pub const MODEL_CATALOGUE_CANCEL_GRAB_ANNOUNCEMENT: &str = "Cancelled keyboard move.";
pub const MODEL_CATALOGUE_BOUNDARY_ANNOUNCEMENT: &str = "Reached list boundary.";

// ── Status tone mapping ──────────────────────────────────────────────────

pub fn model_connection_availability_tone(
    availability: ModelConnectionAvailability,
) -> ModelConnectionStatusTone {
    match availability {
        ModelConnectionAvailability::Available => ModelConnectionStatusTone::Success,
        ModelConnectionAvailability::Checking => ModelConnectionStatusTone::Info,
        ModelConnectionAvailability::Unavailable => ModelConnectionStatusTone::Warning,
        ModelConnectionAvailability::Unsupported => ModelConnectionStatusTone::Neutral,
    }
}

/// The compact label the picker shows. The host's own `availability_label`
/// stays the full reason and reaches assistive technology unchanged.
pub fn model_connection_availability_label(
    availability: ModelConnectionAvailability,
) -> &'static str {
    match availability {
        ModelConnectionAvailability::Available => "Available",
        ModelConnectionAvailability::Checking => "Checking",
        ModelConnectionAvailability::Unavailable => "Unavailable",
        ModelConnectionAvailability::Unsupported => "Unsupported",
    }
}

pub fn model_connection_readiness_tone(
    readiness: ModelConnectionReadiness,
) -> ModelConnectionStatusTone {
    match readiness {
        ModelConnectionReadiness::Ready => ModelConnectionStatusTone::Success,
        ModelConnectionReadiness::Checking => ModelConnectionStatusTone::Info,
        ModelConnectionReadiness::Attention => ModelConnectionStatusTone::Warning,
        ModelConnectionReadiness::Unavailable => ModelConnectionStatusTone::Warning,
        ModelConnectionReadiness::Unknown => ModelConnectionStatusTone::Neutral,
        ModelConnectionReadiness::Error => ModelConnectionStatusTone::Danger,
    }
}

pub fn model_catalogue_state_copy(state: ModelCatalogueState) -> ModelConnectionStateCopy {
    let copy = |title: &str, message: &str| ModelConnectionStateCopy {
        title: title.to_string(),
        message: message.to_string(),
    };

    match state {
        ModelCatalogueState::Loading => copy("Loading models", "Waiting for the connection catalogue."),
        ModelCatalogueState::Unavailable => copy(
            "Models unavailable",
            "This connection does not expose a model catalogue.",
        ),
        ModelCatalogueState::Empty => copy(
            "No models",
            "The catalogue returned successfully with no entries.",
        ),
        ModelCatalogueState::Error => copy(
            "Could not load models",
            "The catalogue request failed. Try again from the host.",
        ),
        ModelCatalogueState::SessionNegotiated => copy(
            "Models after session",
            "Models for this connection are negotiated after a session starts.",
        ),
        ModelCatalogueState::Ready => copy("Models", ""),
    }
}

/// The card's contextual status label: a ready connection shows its supplied
/// access summary, every other readiness shows its readiness label.
pub fn model_connection_card_status_label<'a>(
    readiness: ModelConnectionReadiness,
    access_summary: Option<&'a str>,
    readiness_label: &'a str,
) -> &'a str {
    match (readiness, access_summary) {
        (ModelConnectionReadiness::Ready, Some(summary)) if !summary.is_empty() => summary,
        _ => readiness_label,
    }
}

// ── Specimen fixtures (inert placeholders only) ──────────────────────────

/// The picker fixtures the web specimens use, restated for the Rust targets.
/// Inert placeholder routes: no provider registry, no probe, no credential.
pub fn model_connection_picker_fixtures() -> Vec<ModelConnectionOption> {
    vec![
        ModelConnectionOption::new("openai-responses", "OpenAI", "Hosted")
            .with_route_label("Responses API")
            .with_description("Hosted Responses route for chat and tools.")
            .with_keywords(["openai", "responses", "api"])
            .with_requires_configuration(true),
        ModelConnectionOption::new("openai-completions", "OpenAI", "Hosted")
            .with_route_label("Chat Completions")
            .with_description("Legacy chat-completions route.")
            .with_keywords(["openai", "completions"])
            .with_requires_configuration(true),
        ModelConnectionOption::new("anthropic-messages", "Anthropic", "Hosted")
            .with_route_label("Messages API")
            .with_description("Hosted Messages route.")
            .with_keywords(["anthropic", "messages"])
            .with_requires_configuration(true),
        ModelConnectionOption::new("codex-app", "Codex", "Installed")
            .with_route_label("App install")
            .with_description("Installed local harness.")
            .with_keywords(["codex", "local", "harness"])
            .with_availability(ModelConnectionAvailability::Checking, "Checking install")
            .with_disabled(true),
        ModelConnectionOption::new("ollama-local", "Ollama", "Local runtime")
            .with_route_label("Local runtime")
            .with_description("Local OpenAI-compatible endpoint.")
            .with_keywords(["ollama", "local", "endpoint"])
            .with_requires_configuration(true),
        ModelConnectionOption::new("lmstudio-local", "LM Studio", "Local runtime")
            .with_route_label("Local endpoint")
            .with_description("Local OpenAI-compatible server.")
            .with_keywords(["lmstudio", "local"])
            .with_availability(
                ModelConnectionAvailability::Unavailable,
                "Runtime not detected",
            )
            .with_disabled(true)
            .with_requires_configuration(true),
        ModelConnectionOption::new("vendor-legacy", "Legacy Vendor", "Hosted")
            .with_route_label("SDK v1")
            .with_description("Unsupported on this machine.")
            .with_keywords(["legacy"])
            .with_availability(
                ModelConnectionAvailability::Unsupported,
                "Unsupported on this platform",
            )
            .with_disabled(true)
            .with_requires_configuration(true),
    ]
}

/// The catalogue fixtures the web specimens use, restated for the Rust
/// targets. Two rows deliberately share a label: identity is the opaque id.
pub fn model_catalogue_fixtures() -> Vec<ModelCatalogueItem> {
    vec![
        ModelCatalogueItem::new("model-alpha", "Frontier Alpha")
            .with_provider_label("OpenAI")
            .with_badge(ModelConnectionBadge::new("Default").with_tone(ModelConnectionBadgeTone::Info)),
        ModelCatalogueItem::new("model-beta", "Frontier Beta").with_provider_label("OpenAI"),
        ModelCatalogueItem::new("model-gamma", "Gateway Gamma")
            .with_provider_label("Anthropic")
            .with_description("Mixed gateway entry.")
            .with_badge(ModelConnectionBadge::new("Gateway")),
        ModelCatalogueItem::new("model-dup-a", "Shared Label").with_provider_label("OpenAI"),
        ModelCatalogueItem::new("model-dup-b", "Shared Label")
            .with_provider_label("Anthropic")
            .with_visible(false),
        ModelCatalogueItem::new("model-hidden", "Archive Delta")
            .with_description("Recoverable hidden model.")
            .with_visible(false),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> Vec<ModelConnectionOption> {
        model_connection_picker_fixtures()
    }

    /// The `codex-app` route made available, so the direct-add path has a
    /// route that does not require configuration. Same override as the TS
    /// owner's `directOptions`.
    fn direct_options() -> Vec<ModelConnectionOption> {
        options()
            .into_iter()
            .map(|option| {
                if option.id == "codex-app" {
                    option
                        .with_availability(ModelConnectionAvailability::Available, "Available")
                        .with_disabled(false)
                } else {
                    option
                }
            })
            .collect()
    }

    fn ids(options: &[ModelConnectionOption]) -> Vec<&str> {
        options.iter().map(|option| option.id.as_str()).collect()
    }

    fn setup() -> ModelConnectionSetupContext {
        ModelConnectionSetupContext {
            stage: ModelConnectionSetupStage::Choose,
            value: None,
            query: String::new(),
            options: options(),
            can_submit: false,
            is_pending: false,
        }
    }

    // ── filter_model_connection_options ──

    #[test]
    fn filter_retains_source_order_across_every_searchable_field() {
        let filtered = filter_model_connection_options(&options(), "LOCAL");
        assert_eq!(
            ids(&filtered),
            ["codex-app", "ollama-local", "lmstudio-local"]
        );
    }

    #[test]
    fn filter_with_an_empty_query_returns_the_full_source_order() {
        let all = options();
        let filtered = filter_model_connection_options(&all, "  ");
        assert_eq!(ids(&filtered), ids(&all));
    }

    #[test]
    fn filter_case_folds_keywords_without_reshuffling_groups() {
        let filtered = filter_model_connection_options(&options(), "responses");
        assert_eq!(ids(&filtered), ["openai-responses"]);
        assert_eq!(
            group_model_connection_options(&filtered)
                .iter()
                .map(|group| group.group.as_str())
                .collect::<Vec<_>>(),
            ["Hosted"]
        );
    }

    // ── group_model_connection_options ──

    #[test]
    fn grouping_preserves_first_seen_group_order() {
        assert_eq!(
            group_model_connection_options(&options())
                .iter()
                .map(|group| group.group.as_str())
                .collect::<Vec<_>>(),
            ["Hosted", "Installed", "Local runtime"]
        );
    }

    // ── model_connection_option_selectable ──

    #[test]
    fn only_available_non_disabled_options_are_selectable() {
        let options = options();
        let by_id = |id: &str| {
            options
                .iter()
                .find(|option| option.id == id)
                .expect("fixture option")
                .clone()
        };
        assert!(model_connection_option_selectable(&by_id(
            "openai-responses"
        )));
        assert!(!model_connection_option_selectable(&by_id("codex-app")));
        assert!(!model_connection_option_selectable(&by_id("lmstudio-local")));
        assert!(!model_connection_option_selectable(&by_id("vendor-legacy")));
    }

    // ── resolve_model_connection_picker_shell_state ──

    #[test]
    fn shell_state_maps_postures_and_derived_empty_or_no_results() {
        use ModelConnectionPickerShellState as Shell;
        use ModelConnectionPickerState as State;

        assert_eq!(
            resolve_model_connection_picker_shell_state(State::Loading, 3, 0, ""),
            Shell::Loading
        );
        assert_eq!(
            resolve_model_connection_picker_shell_state(State::Error, 3, 0, ""),
            Shell::Error
        );
        assert_eq!(
            resolve_model_connection_picker_shell_state(State::Ready, 0, 0, ""),
            Shell::Empty
        );
        assert_eq!(
            resolve_model_connection_picker_shell_state(State::Empty, 4, 4, ""),
            Shell::Empty
        );
        assert_eq!(
            resolve_model_connection_picker_shell_state(State::Ready, 4, 0, "zzz"),
            Shell::NoResults
        );
        assert_eq!(
            resolve_model_connection_picker_shell_state(State::NoResults, 4, 0, ""),
            Shell::NoResults
        );
        assert_eq!(
            resolve_model_connection_picker_shell_state(State::Ready, 4, 2, "open"),
            Shell::Ready
        );
    }

    // ── model_connection_picker_result_announcement ──

    #[test]
    fn result_announcement_names_match_counts() {
        assert!(model_connection_picker_result_announcement(0, "zzz").contains("No connections"));
        assert!(
            model_connection_picker_result_announcement(1, "a").contains("1 connection matches")
        );
        assert_eq!(
            model_connection_picker_result_announcement(3, ""),
            "3 connections"
        );
        assert_eq!(
            model_connection_picker_result_announcement(1, ""),
            "1 connection"
        );
    }

    // ── model_connection_picker_state_copy ──

    #[test]
    fn state_copy_names_every_non_ready_posture() {
        use ModelConnectionPickerShellState as Shell;

        assert_eq!(
            model_connection_picker_state_copy(Shell::Loading, "").title,
            "Loading connections"
        );
        assert_eq!(
            model_connection_picker_state_copy(Shell::Error, "").title,
            "Could not load connections"
        );
        assert_eq!(
            model_connection_picker_state_copy(Shell::Empty, "").title,
            "No connections available"
        );
        assert_eq!(
            model_connection_picker_state_copy(Shell::NoResults, "local"),
            ModelConnectionStateCopy {
                title: "No matching connections".to_string(),
                message: "No connections match \u{201c}local\u{201d}.".to_string(),
            }
        );
    }

    // ── model_connection_setup_transition ──

    #[test]
    fn continue_requires_a_selectable_exact_id() {
        assert!(!model_connection_setup_can_continue(&setup()));
        assert!(!model_connection_setup_can_continue(
            &ModelConnectionSetupContext {
                value: Some("codex-app".to_string()),
                ..setup()
            }
        ));
        assert!(model_connection_setup_can_continue(
            &ModelConnectionSetupContext {
                value: Some("openai-responses".to_string()),
                ..setup()
            }
        ));

        let blocked =
            model_connection_setup_transition(setup(), ModelConnectionSetupEvent::Continue);
        assert!(blocked.effects.is_empty());

        let ok = model_connection_setup_transition(
            ModelConnectionSetupContext {
                value: Some("openai-responses".to_string()),
                ..setup()
            },
            ModelConnectionSetupEvent::Continue,
        );
        assert_eq!(ok.context.stage, ModelConnectionSetupStage::Configure);
        assert_eq!(
            ok.effects,
            [ModelConnectionSetupEffect::EmitStageChange {
                stage: ModelConnectionSetupStage::Configure
            }]
        );
    }

    #[test]
    fn submit_requires_configure_can_submit_and_an_exact_id() {
        let denied_context = ModelConnectionSetupContext {
            stage: ModelConnectionSetupStage::Configure,
            value: Some("openai-responses".to_string()),
            can_submit: false,
            ..setup()
        };
        assert!(!model_connection_setup_can_submit(&denied_context));
        assert!(model_connection_setup_transition(
            denied_context,
            ModelConnectionSetupEvent::Submit
        )
        .effects
        .is_empty());

        let accepted = model_connection_setup_transition(
            ModelConnectionSetupContext {
                stage: ModelConnectionSetupStage::Configure,
                value: Some("openai-responses".to_string()),
                can_submit: true,
                ..setup()
            },
            ModelConnectionSetupEvent::Submit,
        );
        assert_eq!(
            accepted.effects,
            [ModelConnectionSetupEffect::EmitSubmit {
                id: "openai-responses".to_string()
            }]
        );
    }

    #[test]
    fn direct_routes_submit_from_choose_without_entering_configure() {
        let context = ModelConnectionSetupContext {
            value: Some("codex-app".to_string()),
            options: direct_options(),
            can_submit: true,
            ..setup()
        };

        assert!(model_connection_setup_can_continue(&context));
        assert!(model_connection_setup_can_submit(&context));

        let result =
            model_connection_setup_transition(context.clone(), ModelConnectionSetupEvent::Submit);
        assert_eq!(result.context, context, "submitting changes no state");
        assert_eq!(
            result.effects,
            [ModelConnectionSetupEffect::EmitSubmit {
                id: "codex-app".to_string()
            }],
            "no stage change is emitted for a direct route"
        );
    }

    #[test]
    fn pending_guards_workflow_events() {
        let pending = ModelConnectionSetupContext {
            stage: ModelConnectionSetupStage::Configure,
            value: Some("openai-responses".to_string()),
            can_submit: true,
            is_pending: true,
            ..setup()
        };
        for event in [
            ModelConnectionSetupEvent::Submit,
            ModelConnectionSetupEvent::Back,
            ModelConnectionSetupEvent::Cancel,
            ModelConnectionSetupEvent::Select {
                id: "anthropic-messages".to_string(),
            },
            ModelConnectionSetupEvent::SetQuery {
                query: "o".to_string(),
            },
        ] {
            assert!(
                model_connection_setup_transition(pending.clone(), event.clone())
                    .effects
                    .is_empty(),
                "pending blocks {event:?}"
            );
        }
    }

    #[test]
    fn back_returns_to_choose_without_clearing_selection() {
        let result = model_connection_setup_transition(
            ModelConnectionSetupContext {
                stage: ModelConnectionSetupStage::Configure,
                value: Some("openai-responses".to_string()),
                ..setup()
            },
            ModelConnectionSetupEvent::Back,
        );
        assert_eq!(result.context.stage, ModelConnectionSetupStage::Choose);
        assert_eq!(result.context.value.as_deref(), Some("openai-responses"));
        assert_eq!(
            result.effects,
            [ModelConnectionSetupEffect::EmitStageChange {
                stage: ModelConnectionSetupStage::Choose
            }]
        );
    }

    #[test]
    fn select_emits_exact_opaque_ids_only_for_selectable_options() {
        let accepted = model_connection_setup_transition(
            setup(),
            ModelConnectionSetupEvent::Select {
                id: "openai-responses".to_string(),
            },
        );
        assert_eq!(
            accepted.effects,
            [ModelConnectionSetupEffect::EmitValueChange {
                id: "openai-responses".to_string()
            }]
        );

        let rejected = model_connection_setup_transition(
            setup(),
            ModelConnectionSetupEvent::Select {
                id: "vendor-legacy".to_string(),
            },
        );
        assert!(rejected.effects.is_empty());
    }

    #[test]
    fn reselecting_the_current_value_emits_nothing() {
        let result = model_connection_setup_transition(
            ModelConnectionSetupContext {
                value: Some("openai-responses".to_string()),
                ..setup()
            },
            ModelConnectionSetupEvent::Select {
                id: "openai-responses".to_string(),
            },
        );
        assert!(result.effects.is_empty());
    }

    #[test]
    fn the_selected_summary_repeats_supplied_labels_only() {
        let context = ModelConnectionSetupContext {
            value: Some("openai-responses".to_string()),
            ..setup()
        };
        let option = model_connection_setup_selected_option(&context).cloned();
        assert_eq!(
            model_connection_selected_summary(option.as_ref()),
            Some(ModelConnectionSelectedSummary {
                id: "openai-responses".to_string(),
                title: "OpenAI".to_string(),
                provider_label: "OpenAI".to_string(),
                route_label: Some("Responses API".to_string()),
            })
        );
        assert_eq!(model_connection_selected_summary(None), None);
    }

    // ── catalogue order and visibility ──

    #[test]
    fn shown_and_hidden_partitions_ignore_hidden_order_meaning() {
        let items = model_catalogue_fixtures();
        assert_eq!(
            shown_model_catalogue_items(&items)
                .iter()
                .map(|item| item.id.clone())
                .collect::<Vec<_>>(),
            ["model-alpha", "model-beta", "model-gamma", "model-dup-a"]
        );
        assert_eq!(
            hidden_model_catalogue_items(&items)
                .iter()
                .map(|item| item.id.clone())
                .collect::<Vec<_>>(),
            ["model-dup-b", "model-hidden"]
        );
    }

    #[test]
    fn order_requests_emit_the_complete_shown_id_order() {
        let shown = shown_model_catalogue_items(&model_catalogue_fixtures())
            .iter()
            .map(|item| item.id.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            request_model_catalogue_order(&shown, 0, 2),
            Some(vec![
                "model-beta".to_string(),
                "model-gamma".to_string(),
                "model-alpha".to_string(),
                "model-dup-a".to_string(),
            ])
        );
        assert_eq!(request_model_catalogue_order(&shown, 0, 0), None);
        assert_eq!(request_model_catalogue_order(&shown, 0, 9), None);
    }

    #[test]
    fn visibility_requests_only_carry_id_and_visible() {
        assert_eq!(
            request_model_catalogue_visibility("model-alpha", false),
            ModelCatalogueVisibilityChange {
                id: "model-alpha".to_string(),
                visible: false,
            }
        );
    }

    #[test]
    fn focus_after_hide_follows_next_previous_or_hidden_section() {
        let ids = |list: [&str; 3]| list.map(str::to_string).to_vec();
        assert_eq!(
            model_catalogue_focus_after_hide(&ids(["a", "b", "c"]), "b"),
            ModelCatalogueFocusAfterHide::Shown {
                id: "c".to_string()
            }
        );
        assert_eq!(
            model_catalogue_focus_after_hide(&ids(["a", "b", "c"]), "c"),
            ModelCatalogueFocusAfterHide::Shown {
                id: "b".to_string()
            }
        );
        assert_eq!(
            model_catalogue_focus_after_hide(&["a".to_string()], "a"),
            ModelCatalogueFocusAfterHide::HiddenSection
        );
        assert_eq!(
            model_catalogue_focus_after_hide(&["a".to_string()], "zz"),
            ModelCatalogueFocusAfterHide::HiddenSection
        );
    }

    #[test]
    fn announcements_name_the_model_and_outcome() {
        assert_eq!(
            model_catalogue_reorder_announcement("Frontier Alpha", 2, 4),
            "Moved Frontier Alpha to position 2 of 4."
        );
        assert_eq!(
            model_catalogue_visibility_announcement("Archive Delta", false),
            "Hid Archive Delta."
        );
        assert_eq!(
            model_catalogue_visibility_announcement("Archive Delta", true),
            "Restored Archive Delta."
        );
    }

    #[test]
    fn catalogue_state_copy_keeps_every_posture_distinct() {
        use ModelCatalogueState as State;
        let titles = [
            State::Ready,
            State::Loading,
            State::Unavailable,
            State::Empty,
            State::Error,
            State::SessionNegotiated,
        ]
        .map(|state| model_catalogue_state_copy(state).title);
        let mut unique = titles.to_vec();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), titles.len(), "no posture borrows another's copy");
        assert_eq!(model_catalogue_state_copy(State::Empty).title, "No models");
        assert_eq!(
            model_catalogue_state_copy(State::SessionNegotiated).message,
            "Models for this connection are negotiated after a session starts."
        );
        assert_eq!(model_catalogue_state_copy(State::Ready).message, "");
    }

    // ── reorder key intents ──

    #[test]
    fn reorder_key_intent_toggles_grab_and_cancels() {
        assert_eq!(
            model_catalogue_reorder_key_intent(" ", 2, None, 5),
            Some(ModelCatalogueKeyIntent::Grab)
        );
        assert_eq!(
            model_catalogue_reorder_key_intent("Enter", 2, Some(2), 5),
            Some(ModelCatalogueKeyIntent::Drop)
        );
        assert_eq!(
            model_catalogue_reorder_key_intent("Escape", 2, Some(2), 5),
            Some(ModelCatalogueKeyIntent::CancelGrab)
        );
        assert_eq!(model_catalogue_reorder_key_intent("Escape", 2, None, 5), None);
        assert_eq!(model_catalogue_reorder_key_intent("x", 2, None, 5), None);
    }

    #[test]
    fn reorder_key_intent_moves_the_active_row_and_reports_boundaries() {
        assert_eq!(
            model_catalogue_reorder_key_intent("ArrowDown", 1, None, 5),
            Some(ModelCatalogueKeyIntent::Move { from: 1, to: 2 })
        );
        assert_eq!(
            model_catalogue_reorder_key_intent("ArrowUp", 1, Some(3), 5),
            Some(ModelCatalogueKeyIntent::Move { from: 3, to: 2 })
        );
        assert_eq!(
            model_catalogue_reorder_key_intent("ArrowUp", 0, None, 5),
            Some(ModelCatalogueKeyIntent::Boundary)
        );
        assert_eq!(
            model_catalogue_reorder_key_intent("ArrowDown", 4, None, 5),
            Some(ModelCatalogueKeyIntent::Boundary)
        );
    }

    // ── status tones ──

    #[test]
    fn tones_map_availability_and_readiness_without_collapsing_dimensions() {
        assert_eq!(
            model_connection_availability_tone(ModelConnectionAvailability::Available),
            ModelConnectionStatusTone::Success
        );
        assert_eq!(
            model_connection_availability_tone(ModelConnectionAvailability::Unsupported),
            ModelConnectionStatusTone::Neutral
        );
        assert_eq!(
            model_connection_readiness_tone(ModelConnectionReadiness::Attention),
            ModelConnectionStatusTone::Warning
        );
        assert_eq!(
            model_connection_readiness_tone(ModelConnectionReadiness::Error),
            ModelConnectionStatusTone::Danger
        );
    }

    #[test]
    fn picker_uses_the_compact_availability_labels() {
        assert_eq!(
            model_connection_availability_label(ModelConnectionAvailability::Available),
            "Available"
        );
        assert_eq!(
            model_connection_availability_label(ModelConnectionAvailability::Checking),
            "Checking"
        );
        assert_eq!(
            model_connection_availability_label(ModelConnectionAvailability::Unavailable),
            "Unavailable"
        );
        assert_eq!(
            model_connection_availability_label(ModelConnectionAvailability::Unsupported),
            "Unsupported"
        );
    }

    #[test]
    fn the_card_status_label_prefers_access_summary_only_when_ready() {
        assert_eq!(
            model_connection_card_status_label(
                ModelConnectionReadiness::Ready,
                Some("API key on file"),
                "Ready"
            ),
            "API key on file"
        );
        assert_eq!(
            model_connection_card_status_label(ModelConnectionReadiness::Ready, None, "Ready"),
            "Ready"
        );
        assert_eq!(
            model_connection_card_status_label(
                ModelConnectionReadiness::Checking,
                Some("API key on file"),
                "Checking install"
            ),
            "Checking install",
            "readiness overrides the access summary once it is not ready"
        );
    }
}
