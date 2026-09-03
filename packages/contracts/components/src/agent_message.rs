//! AgentMessageSpec — one turn of prose in an agent conversation.
//!
//! Contract: `docs/contracts/components/agent-message.md`.
//!
//! Markdown is parsed into the shared block model by `poodle-markdown`, so the
//! natives render the same document the web target does.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::agent_transcript::TranscriptRole;
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct AgentMessageSpec {
    pub markdown: String,
    pub role: TranscriptRole,
    pub is_streaming: bool,
    pub link_target: Option<String>,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
}

impl Default for AgentMessageSpec {
    fn default() -> Self {
        Self::new("")
    }
}

impl AgentMessageSpec {
    pub fn new(markdown: impl Into<String>) -> Self {
        Self {
            markdown: markdown.into(),
            role: TranscriptRole::Assistant,
            is_streaming: false,
            link_target: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }

    pub fn with_role(mut self, role: TranscriptRole) -> Self {
        self.role = role;
        self
    }
    pub fn with_streaming(mut self, streaming: bool) -> Self {
        self.is_streaming = streaming;
        self
    }
    pub fn with_link_target(mut self, target: impl Into<String>) -> Self {
        self.link_target = Some(target.into());
        self
    }
    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }
    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    /// An empty message contributes no box: a turn with nothing in it should
    /// not reserve space in the transcript.
    pub fn renders(&self) -> bool {
        !self.markdown.trim().is_empty() || self.is_streaming
    }

    pub fn is_user(&self) -> bool {
        matches!(self.role, TranscriptRole::User)
    }

    // ── Tokens ────────────────────────────────────────────────
    pub fn text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }
    pub fn quote_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
    pub fn quote_rule_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
    pub fn code_span_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
    /// A user turn sits on the surface step; an assistant turn has no chrome.
    pub fn user_surface_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }
    pub fn caret_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    // ── Size ─────────────────────────────────────────────────
    pub fn font_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.75,
            ControlSize::Sm => 0.875,
            ControlSize::Md => 0.9375,
            ControlSize::Lg => 1.0,
            ControlSize::Xl => 1.125,
        }
    }
    /// The prose measure, so long answers stay readable.
    pub fn measure_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 38.0,
            ControlSize::Sm => 42.0,
            ControlSize::Md => 46.0,
            ControlSize::Lg => 50.0,
            ControlSize::Xl => 54.0,
        }
    }

    // ── Density ──────────────────────────────────────────────
    pub fn block_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.875,
            ControlDensity::Comfortable => 1.125,
        }
    }
    pub fn list_indent_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 1.0,
            ControlDensity::Default => 1.25,
            ControlDensity::Comfortable => 1.5,
        }
    }
    pub fn padding_inset_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_message_does_not_render() {
        assert!(!AgentMessageSpec::new("").renders());
        assert!(!AgentMessageSpec::new("   \n ").renders());
        assert!(AgentMessageSpec::new("text").renders());
    }

    #[test]
    fn a_streaming_message_renders_even_before_any_tokens_arrive() {
        assert!(AgentMessageSpec::new("").with_streaming(true).renders());
    }

    #[test]
    fn density_moves_spacing_but_never_the_type_scale() {
        let base = AgentMessageSpec::new("x");
        let dense = base.clone().with_density(ControlDensity::Compact);

        assert_ne!(
            dense.block_gap_rem(ControlDensity::Compact),
            base.block_gap_rem(ControlDensity::Default)
        );
        assert_eq!(
            dense.font_size_rem(ControlSize::Md),
            base.font_size_rem(ControlSize::Md)
        );
        assert_eq!(
            dense.measure_rem(ControlSize::Md),
            base.measure_rem(ControlSize::Md)
        );
    }
}
