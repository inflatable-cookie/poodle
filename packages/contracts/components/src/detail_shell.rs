use poodle_tokens::semantic;

use crate::composite_types::ScrollOwner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DetailState {
    Ready,
    Empty,
    Loading,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailShellSpec {
    pub title: Option<String>,
    pub scroll_owner: ScrollOwner,
    pub state: DetailState,
    pub aria_label: Option<String>,
    /// Heading text for the state region; falls back to "Detail state".
    pub state_title: Option<String>,
    /// Body text for the state region (optional `<p>` under the title).
    pub state_message: Option<String>,
}

impl Default for DetailShellSpec {
    fn default() -> Self {
        Self {
            title: None,
            scroll_owner: ScrollOwner::Content,
            state: DetailState::Ready,
            aria_label: None,
            state_title: None,
            state_message: None,
        }
    }
}

impl DetailShellSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_scroll_owner(mut self, scroll_owner: ScrollOwner) -> Self {
        self.scroll_owner = scroll_owner;
        self
    }

    pub fn with_state(mut self, state: DetailState) -> Self {
        self.state = state;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_state_title(mut self, state_title: impl Into<String>) -> Self {
        self.state_title = Some(state_title.into());
        self
    }

    pub fn with_state_message(mut self, state_message: impl Into<String>) -> Self {
        self.state_message = Some(state_message.into());
        self
    }

    pub fn has_ready_content(&self) -> bool {
        self.state == DetailState::Ready
    }

    /// The `data-scroll-mode` value the contract emits: `Shell` → "shell",
    /// `Content` → "body".
    pub fn scroll_mode_value(&self) -> &'static str {
        match self.scroll_owner {
            ScrollOwner::Shell => "shell",
            ScrollOwner::Content => "body",
        }
    }

    /// The effective state-region heading. Contract §4: falls back to
    /// "Detail state" when no `state_title` is set.
    pub fn effective_state_title(&self) -> &str {
        self.state_title.as_deref().unwrap_or("Detail state")
    }

    // ── Token methods ────────────────────────────────────────

    pub fn body_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// Root + body/state stacking gap (contract §9: `space-stack-lg`).
    pub fn stack_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_LG
    }

    /// State-region surface fill base (contract §10: `background-panel`,
    /// mixed toward `background-elevated`).
    pub fn state_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// State-region surface fill mix target (contract §10).
    pub fn state_fill_mix_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    /// State-region corner radius (contract §9: `radius-surface`).
    pub fn state_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    /// State-region vertical/horizontal padding base tokens (contract §9:
    /// "doubled panel spacing"). Doubled vertically, 1.5× horizontally to match
    /// the Svelte `calc(panel-y * 2) calc(panel-x * 1.5)`.
    pub fn state_pad_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
    }
    pub fn state_pad_x_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    /// State-region border (contract: `0.0625rem solid transparent`).
    pub fn state_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// State-message body text color (contract §10: `text-secondary`).
    pub fn state_message_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// State-title heading color.
    pub fn state_title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }
}
