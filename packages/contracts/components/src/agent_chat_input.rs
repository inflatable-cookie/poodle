//! AgentChatInputSpec — the agent composer model, Rust mirror of the
//! `@inflatable-cookie/poodle-svelte` AgentChatInput type model and `agent-chat-input-model.ts`
//! logic.
//!
//! Contract: `docs/contracts/components/agent-chat-input.md`.
//!
//! Poodle owns the composer's shape, submit gating and status semantics. The
//! host owns the transcript, the transport and whatever controls it drops into
//! the toolbar region.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AgentChatStatus {
    #[default]
    Idle,
    Busy,
    Questioning,
    ReviewingPlan,
}

impl AgentChatStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            AgentChatStatus::Idle => "idle",
            AgentChatStatus::Busy => "busy",
            AgentChatStatus::Questioning => "questioning",
            AgentChatStatus::ReviewingPlan => "reviewing-plan",
        }
    }

    pub fn is_busy(self) -> bool {
        matches!(self, AgentChatStatus::Busy)
    }
}

/// What a key gesture means in the composer (contract §4 Submit Intent
/// Resolution). The renderer maps its own key events onto `resolve_submit_intent`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubmitIntent {
    Submit,
    Newline,
    Stop,
    None,
}

/// The keys the composer reacts to. Everything else resolves to `None`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComposerKey {
    Enter,
    Escape,
    Other,
}

/// Modifier/IME state accompanying a key press.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ComposerKeyModifiers {
    pub shift: bool,
    /// Cmd on macOS, Ctrl elsewhere.
    pub command: bool,
    pub is_composing: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AgentChatAttachment {
    pub id: String,
    pub label: String,
    /// Host-defined kind, surfaced as a styling hook.
    pub kind: Option<String>,
    pub icon: Option<String>,
    /// Image source for a visual attachment. When set the chip is replaced by a
    /// thumbnail tile — an image says more than its filename does.
    pub thumbnail_url: Option<String>,
    pub is_disabled: bool,
}

impl AgentChatAttachment {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: None,
            icon: None,
            thumbnail_url: None,
            is_disabled: false,
        }
    }

    /// Render this attachment as a thumbnail tile instead of a labelled chip.
    pub fn with_thumbnail(mut self, url: impl Into<String>) -> Self {
        self.thumbnail_url = Some(url.into());
        self
    }

    /// Whether this attachment renders as a tile rather than a chip.
    pub fn is_thumbnail(&self) -> bool {
        self.thumbnail_url.is_some()
    }

    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = Some(kind.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AgentChatInputSpec {
    pub value: String,
    pub placeholder: String,
    pub question_placeholder: String,
    pub question_can_submit: bool,
    pub plan_placeholder: String,
    pub status: AgentChatStatus,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub aria_label: String,
    pub submit_label: String,
    pub stop_label: String,
    pub submit_on_enter: bool,
    pub min_rows: usize,
    pub max_rows: usize,
    pub max_length: Option<usize>,
    pub allow_empty_submit: bool,
    pub attachments: Vec<AgentChatAttachment>,
    pub context_used: Option<f64>,
    pub context_limit: Option<f64>,
    pub context_warn_at: f64,
    pub context_label: String,
    pub toolbar_dividers: bool,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
}

impl Default for AgentChatInputSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentChatInputSpec {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: "Send a message".to_string(),
            question_placeholder:
                "Type your own answer, or leave this blank to use the selected option".to_string(),
            question_can_submit: false,
            plan_placeholder: "Describe what to change, or decide the plan above".to_string(),
            status: AgentChatStatus::Idle,
            is_disabled: false,
            is_read_only: false,
            aria_label: "Message".to_string(),
            submit_label: "Send".to_string(),
            stop_label: "Stop".to_string(),
            submit_on_enter: true,
            min_rows: 2,
            max_rows: 12,
            max_length: None,
            allow_empty_submit: false,
            attachments: Vec::new(),
            context_used: None,
            context_limit: None,
            context_warn_at: 0.8,
            context_label: "Context used".to_string(),
            toolbar_dividers: true,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_question_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.question_placeholder = placeholder.into();
        self
    }

    pub fn with_question_can_submit(mut self, can_submit: bool) -> Self {
        self.question_can_submit = can_submit;
        self
    }

    pub fn with_plan_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.plan_placeholder = placeholder.into();
        self
    }

    pub fn with_status(mut self, status: AgentChatStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.is_read_only = read_only;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    pub fn with_submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit_label = label.into();
        self
    }

    pub fn with_stop_label(mut self, label: impl Into<String>) -> Self {
        self.stop_label = label.into();
        self
    }

    pub fn with_submit_on_enter(mut self, submit_on_enter: bool) -> Self {
        self.submit_on_enter = submit_on_enter;
        self
    }

    pub fn with_rows(mut self, min_rows: usize, max_rows: usize) -> Self {
        self.min_rows = min_rows;
        self.max_rows = max_rows;
        self
    }

    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn with_allow_empty_submit(mut self, allow: bool) -> Self {
        self.allow_empty_submit = allow;
        self
    }

    pub fn with_attachments(mut self, attachments: Vec<AgentChatAttachment>) -> Self {
        self.attachments = attachments;
        self
    }

    pub fn with_context(mut self, used: f64, limit: f64) -> Self {
        self.context_used = Some(used);
        self.context_limit = Some(limit);
        self
    }

    pub fn with_context_warn_at(mut self, warn_at: f64) -> Self {
        self.context_warn_at = warn_at;
        self
    }

    pub fn with_context_label(mut self, label: impl Into<String>) -> Self {
        self.context_label = label.into();
        self
    }

    pub fn with_toolbar_dividers(mut self, dividers: bool) -> Self {
        self.toolbar_dividers = dividers;
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

    // ── Derived state (contract §3 Computed Values) ───────────────────────

    pub fn is_busy(&self) -> bool {
        self.status.is_busy()
    }

    pub fn has_text(&self) -> bool {
        !self.value.trim().is_empty()
    }

    pub fn effective_placeholder(&self) -> &str {
        match self.status {
            AgentChatStatus::Questioning => &self.question_placeholder,
            AgentChatStatus::ReviewingPlan => &self.plan_placeholder,
            AgentChatStatus::Idle | AgentChatStatus::Busy => &self.placeholder,
        }
    }

    /// Whether the action button is enabled. While busy it always is — stopping
    /// must never be blocked by an empty editor.
    pub fn can_submit(&self) -> bool {
        !self.is_disabled
            && (self.is_busy()
                || self.has_text()
                || (self.status == AgentChatStatus::Questioning && self.question_can_submit)
                || (self.status != AgentChatStatus::Questioning && self.allow_empty_submit))
    }

    /// Glyph for the action button.
    pub fn action_icon(&self) -> &'static str {
        if self.is_busy() {
            "square"
        } else {
            "arrow-up"
        }
    }

    /// Accessible name for the action button.
    pub fn action_aria_label(&self) -> &str {
        if self.is_busy() {
            &self.stop_label
        } else {
            &self.submit_label
        }
    }

    pub fn action_state(&self) -> &'static str {
        if self.is_busy() {
            "stop"
        } else {
            "submit"
        }
    }

    pub fn show_context(&self) -> bool {
        self.context_limit.is_some_and(|limit| limit > 0.0)
    }

    /// Context usage as a percentage of the limit, or `None` when no limit is set.
    pub fn context_percentage(&self) -> Option<f64> {
        let limit = self.context_limit.filter(|limit| *limit > 0.0)?;
        let used = self.context_used.unwrap_or(0.0).clamp(0.0, limit);
        Some(used / limit * 100.0)
    }

    /// The `high` threshold handed to the context Meter.
    pub fn context_high(&self) -> Option<f64> {
        self.context_limit.map(|limit| limit * self.context_warn_at)
    }

    /// Accessible name for the context ring, carrying the percentage.
    pub fn context_aria_label(&self) -> String {
        match self.context_percentage() {
            Some(percentage) => format!("{}, {}%", self.context_label, percentage.round() as i64),
            None => self.context_label.clone(),
        }
    }

    /// Editor height in rows: the value's line count clamped between the floor
    /// and ceiling. Native targets use this in place of text measurement
    /// (contract §12).
    pub fn visible_rows(&self) -> usize {
        let lines = self.value.lines().count().max(1);
        lines.clamp(
            self.min_rows.max(1),
            self.max_rows.max(self.min_rows.max(1)),
        )
    }

    /// Contract §4: map a key gesture to composer intent.
    pub fn resolve_submit_intent(
        &self,
        key: ComposerKey,
        modifiers: ComposerKeyModifiers,
    ) -> SubmitIntent {
        match key {
            ComposerKey::Escape => {
                if self.is_busy() {
                    SubmitIntent::Stop
                } else {
                    SubmitIntent::None
                }
            }
            ComposerKey::Enter => {
                // IME composition never submits — the Enter belongs to the IME.
                if modifiers.is_composing {
                    return SubmitIntent::Newline;
                }
                if modifiers.command {
                    return SubmitIntent::Submit;
                }
                if modifiers.shift || !self.submit_on_enter {
                    return SubmitIntent::Newline;
                }
                SubmitIntent::Submit
            }
            ComposerKey::Other => SubmitIntent::None,
        }
    }

    /// Whether a resolved intent should actually fire `on_submit`. Stop is
    /// deliberate: a keyboard submit while busy is dropped, not converted.
    pub fn accepts_submit(&self, intent: SubmitIntent) -> bool {
        intent == SubmitIntent::Submit && !self.is_busy() && self.can_submit()
    }

    // ── Size ladder (contract §8) ─────────────────────────────────────────

    /// Field padding: (vertical, horizontal) in rem.
    pub fn field_padding_rem(&self, size: ControlSize) -> (f32, f32) {
        match size {
            ControlSize::Xs => (0.375, 0.5),
            ControlSize::Sm => (0.5, 0.625),
            ControlSize::Md => (0.625, 0.75),
            ControlSize::Lg => (0.75, 1.0),
            ControlSize::Xl => (0.875, 1.125),
        }
    }

    pub fn editor_font_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.75,
            ControlSize::Sm => 0.8125,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 0.9375,
            ControlSize::Xl => 1.0,
        }
    }

    pub fn action_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 1.5,
            ControlSize::Sm => 1.75,
            ControlSize::Md => 2.0,
            ControlSize::Lg => 2.375,
            ControlSize::Xl => 2.75,
        }
    }

    /// Hairline height for the dividers between leading toolbar children. The
    /// web draws these as a border on the child, so they take the child's full
    /// height; natives have no such hook and use one line-height (contract §12).
    pub fn toolbar_divider_height_rem(&self, size: ControlSize) -> f32 {
        self.editor_font_rem(size) * 1.5
    }

    /// Thumbnail tile edge in rem, per size (contract §8).
    pub fn attachment_thumb_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 2.0,
            ControlSize::Sm => 2.5,
            ControlSize::Md => 3.0,
            ControlSize::Lg => 3.5,
            ControlSize::Xl => 4.0,
        }
    }

    pub fn toolbar_gap_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.25,
            ControlSize::Sm => 0.375,
            ControlSize::Md => 0.5,
            ControlSize::Lg => 0.625,
            ControlSize::Xl => 0.75,
        }
    }

    /// Density scales only sibling spacing — never the field's vertical padding
    /// or the action box (contract §8). Takes the resolved density; omission is
    /// resolved by the render context.
    pub fn density_gap_scale(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.75,
            ControlDensity::Default => 1.0,
            ControlDensity::Comfortable => 1.25,
        }
    }

    // ── Token accessors (shared by GPUI + Jetstream) ──────────────────────

    /// Two steps up the background ladder (canvas → surface → panel): one step is
    /// too close to the page for the composer to read as a distinct block.
    pub fn field_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn field_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn field_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Placeholder text colour. `color.text.placeholder` is not a token any theme
    /// defines — the placeholder is secondary text held at
    /// `placeholder_opacity_ratio` of the muted opacity.
    pub fn placeholder_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn placeholder_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_MUTED
    }

    /// The composer's placeholder is a standing hint, not a value, so it sits
    /// dimmer than the shared input convention (contract §8).
    pub fn placeholder_opacity_ratio(&self) -> f32 {
        0.62
    }

    pub fn secondary_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn divider_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn attachment_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn attachment_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    /// Action fill: accent while idle, danger while busy (contract §8).
    pub fn action_fill_token(&self) -> &'static str {
        if self.is_busy() {
            semantic::COLOR_STATUS_DANGER
        } else {
            semantic::COLOR_ACCENT_BASE
        }
    }

    pub fn action_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_INVERSE
    }

    /// Below the field on the ladder, so the footer reads as a secondary bar
    /// tucked under the composer rather than another block on top of it.
    pub fn footer_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enter(shift: bool, command: bool) -> ComposerKeyModifiers {
        ComposerKeyModifiers {
            shift,
            command,
            is_composing: false,
        }
    }

    #[test]
    fn submit_gating_matches_the_contract_table() {
        let empty = AgentChatInputSpec::new();
        assert!(!empty.can_submit());

        assert!(AgentChatInputSpec::new().with_value("hi").can_submit());
        // Whitespace is not text.
        assert!(!AgentChatInputSpec::new().with_value("   \n ").can_submit());
        assert!(AgentChatInputSpec::new()
            .with_allow_empty_submit(true)
            .can_submit());
        // Busy always allows the action — stopping must not be blocked.
        assert!(AgentChatInputSpec::new()
            .with_status(AgentChatStatus::Busy)
            .can_submit());
        // Disabled beats everything.
        assert!(!AgentChatInputSpec::new()
            .with_value("hi")
            .with_disabled(true)
            .can_submit());
        assert!(AgentChatInputSpec::new()
            .with_status(AgentChatStatus::Questioning)
            .with_question_can_submit(true)
            .can_submit());
        assert!(!AgentChatInputSpec::new()
            .with_status(AgentChatStatus::Questioning)
            .with_allow_empty_submit(true)
            .can_submit());
    }

    #[test]
    fn status_selects_the_editor_placeholder() {
        let spec = AgentChatInputSpec::new()
            .with_question_placeholder("Answer the question")
            .with_plan_placeholder("Revise the plan");

        assert_eq!(spec.effective_placeholder(), "Send a message");
        assert_eq!(
            spec.clone()
                .with_status(AgentChatStatus::Questioning)
                .effective_placeholder(),
            "Answer the question"
        );
        assert_eq!(
            spec.with_status(AgentChatStatus::ReviewingPlan)
                .effective_placeholder(),
            "Revise the plan"
        );
    }

    #[test]
    fn action_flips_with_status() {
        let idle = AgentChatInputSpec::new().with_value("hi");
        assert_eq!(idle.action_icon(), "arrow-up");
        assert_eq!(idle.action_aria_label(), "Send");
        assert_eq!(idle.action_state(), "submit");
        assert_eq!(idle.action_fill_token(), semantic::COLOR_ACCENT_BASE);

        let busy = idle.clone().with_status(AgentChatStatus::Busy);
        assert_eq!(busy.action_icon(), "square");
        assert_eq!(busy.action_aria_label(), "Stop");
        assert_eq!(busy.action_state(), "stop");
        assert_eq!(busy.action_fill_token(), semantic::COLOR_STATUS_DANGER);
    }

    #[test]
    fn submit_intent_covers_every_gesture_row() {
        let spec = AgentChatInputSpec::new().with_value("hi");
        assert_eq!(
            spec.resolve_submit_intent(ComposerKey::Enter, enter(false, false)),
            SubmitIntent::Submit
        );
        assert_eq!(
            spec.resolve_submit_intent(ComposerKey::Enter, enter(true, false)),
            SubmitIntent::Newline
        );
        assert_eq!(
            spec.resolve_submit_intent(ComposerKey::Enter, enter(false, true)),
            SubmitIntent::Submit
        );
        assert_eq!(
            spec.resolve_submit_intent(ComposerKey::Other, enter(false, false)),
            SubmitIntent::None
        );
        // Escape only means stop while busy.
        assert_eq!(
            spec.resolve_submit_intent(ComposerKey::Escape, ComposerKeyModifiers::default()),
            SubmitIntent::None
        );
        assert_eq!(
            spec.clone()
                .with_status(AgentChatStatus::Busy)
                .resolve_submit_intent(ComposerKey::Escape, ComposerKeyModifiers::default()),
            SubmitIntent::Stop
        );

        // submit_on_enter=false: bare Enter is a newline, Cmd+Enter still submits.
        let manual = spec.clone().with_submit_on_enter(false);
        assert_eq!(
            manual.resolve_submit_intent(ComposerKey::Enter, enter(false, false)),
            SubmitIntent::Newline
        );
        assert_eq!(
            manual.resolve_submit_intent(ComposerKey::Enter, enter(false, true)),
            SubmitIntent::Submit
        );

        // IME composition never submits, even with a modifier.
        let composing = ComposerKeyModifiers {
            shift: false,
            command: true,
            is_composing: true,
        };
        assert_eq!(
            spec.resolve_submit_intent(ComposerKey::Enter, composing),
            SubmitIntent::Newline
        );
    }

    #[test]
    fn keyboard_submit_is_dropped_while_busy() {
        let busy = AgentChatInputSpec::new()
            .with_value("hi")
            .with_status(AgentChatStatus::Busy);
        let intent = busy.resolve_submit_intent(ComposerKey::Enter, enter(false, false));
        assert_eq!(intent, SubmitIntent::Submit);
        // …but it never reaches on_submit: stopping is deliberate.
        assert!(!busy.accepts_submit(intent));
        assert!(AgentChatInputSpec::new()
            .with_value("hi")
            .accepts_submit(intent));
    }

    #[test]
    fn attachments_switch_to_tiles_when_they_carry_a_thumbnail() {
        let file = AgentChatAttachment::new("a1", "release-notes.md");
        assert!(!file.is_thumbnail());

        let image =
            AgentChatAttachment::new("a2", "diagram.png").with_thumbnail("/tmp/diagram.png");
        assert!(image.is_thumbnail());
        assert_eq!(image.thumbnail_url.as_deref(), Some("/tmp/diagram.png"));

        // Tiles are square and grow with the size ladder.
        let spec = AgentChatInputSpec::new();
        assert!(
            spec.attachment_thumb_rem(ControlSize::Xs) < spec.attachment_thumb_rem(ControlSize::Xl)
        );
    }

    #[test]
    fn context_ring_only_renders_with_a_limit() {
        let none = AgentChatInputSpec::new();
        assert!(!none.show_context());
        assert_eq!(none.context_percentage(), None);
        assert_eq!(none.context_aria_label(), "Context used");

        let spec = AgentChatInputSpec::new().with_context(40_000.0, 200_000.0);
        assert!(spec.show_context());
        assert_eq!(spec.context_percentage(), Some(20.0));
        assert_eq!(spec.context_high(), Some(160_000.0));
        assert_eq!(spec.context_aria_label(), "Context used, 20%");

        // Overshoot clamps rather than exceeding 100%.
        let over = AgentChatInputSpec::new().with_context(999_000.0, 200_000.0);
        assert_eq!(over.context_percentage(), Some(100.0));
    }

    #[test]
    fn visible_rows_clamps_between_floor_and_ceiling() {
        let spec = AgentChatInputSpec::new().with_rows(2, 4);
        assert_eq!(spec.clone().with_value("").visible_rows(), 2);
        assert_eq!(spec.clone().with_value("a\nb\nc").visible_rows(), 3);
        assert_eq!(spec.with_value("a\nb\nc\nd\ne\nf").visible_rows(), 4);
    }
}
