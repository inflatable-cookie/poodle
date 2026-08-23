//! ToolCallSpec — one row of agent work.
//!
//! Contract: `docs/contracts/components/tool-call.md`.
//!
//! The row is skimmable at a glance and openable when it matters. Most tool
//! calls are never read; the ones that are, are read because something broke.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct ToolCallSpec {
    pub id: String,
    pub label: String,
    pub detail: Option<String>,
    pub status: ToolCallStatus,
    pub icon: Option<String>,
    pub output: Option<String>,
    pub output_language: Option<String>,
    pub is_expanded: bool,
    /// `None` inherits from the presentation context; an explicit value wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value wins.
    pub density: Option<ControlDensity>,
}

impl Default for ToolCallSpec {
    fn default() -> Self {
        Self::new("", "")
    }
}

impl ToolCallSpec {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            detail: None,
            status: ToolCallStatus::Success,
            icon: None,
            output: None,
            output_language: None,
            is_expanded: false,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
    pub fn with_status(mut self, status: ToolCallStatus) -> Self {
        self.status = status;
        self
    }
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
    pub fn with_output(mut self, output: impl Into<String>) -> Self {
        self.output = Some(output.into());
        self
    }
    pub fn with_output_language(mut self, language: impl Into<String>) -> Self {
        self.output_language = Some(language.into());
        self
    }
    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
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

    /// A row with no output is not interactive at all — not a disabled button,
    /// no chevron, no tab stop.
    pub fn has_output(&self) -> bool {
        self.output.as_ref().is_some_and(|o| !o.is_empty())
    }

    pub fn is_interactive(&self) -> bool {
        self.has_output()
    }

    /// The icon for a kind of work, shared across every target so a "Ran
    /// command" row is the same glyph everywhere. `icon` overrides it, so a
    /// host with its own vocabulary is never stuck with the fallback.
    pub fn resolved_icon(&self) -> &str {
        if let Some(icon) = &self.icon {
            return icon;
        }
        let key = self.label.trim().to_ascii_lowercase();
        if key.starts_with("ran command") || key.starts_with("command") {
            "terminal"
        } else if key.starts_with("file change") || key.starts_with("edited") {
            "file-pen"
        } else if key.starts_with("search") {
            "search"
        } else if key.starts_with("read") {
            "file-text"
        } else {
            "dot"
        }
    }

    pub fn status_icon(&self) -> &'static str {
        match self.status {
            ToolCallStatus::Running => "loader",
            ToolCallStatus::Success => "check",
            ToolCallStatus::Error => "x",
        }
    }

    /// Status reaches assistive technology through the name; colour and glyph
    /// do not. `success` is omitted as the unremarkable case, and the detail is
    /// carried in full — the truncation is visual, and the whole command is
    /// exactly what a truncated row is hiding.
    pub fn accessible_name(&self) -> String {
        let detail = self
            .detail
            .as_ref()
            .map(|d| format!(": {d}"))
            .unwrap_or_default();
        let status = match self.status {
            ToolCallStatus::Success => String::new(),
            other => format!(", {}", other.as_str()),
        };
        format!("{}{detail}{status}", self.label)
    }

    // ── Tokens ────────────────────────────────────────────────
    /// Secondary, not primary: a tool row is chrome next to the prose around
    /// it, and at equal brightness the two compete.
    pub fn label_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
    pub fn detail_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }
    /// The payload is the dimmest thing in the transcript — there to be
    /// recognised, not read. The text ramp bottoms out at tertiary, so the last
    /// step down is opacity rather than a fourth colour.
    pub fn detail_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_MUTED
    }
    pub fn icon_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
    pub fn success_token(&self) -> &'static str {
        semantic::COLOR_STATUS_SUCCESS
    }
    pub fn danger_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }
    pub fn hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }
    pub fn focus_ring_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
    pub fn focus_ring_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_FOCUS
    }

    // ── Size: intrinsic dimensions ───────────────────────────
    pub fn row_height_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 1.375,
            ControlSize::Sm => 1.5,
            ControlSize::Md => 1.75,
            ControlSize::Lg => 2.0,
            ControlSize::Xl => 2.25,
        }
    }
    pub fn font_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }
    pub fn icon_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.75,
            ControlSize::Sm => 0.8125,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 1.0,
            ControlSize::Xl => 1.125,
        }
    }
    pub fn padding_block_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.125,
            ControlSize::Sm => 0.1875,
            ControlSize::Md => 0.25,
            ControlSize::Lg => 0.3125,
            ControlSize::Xl => 0.375,
        }
    }

    // ── Density: spacing between siblings, never height ──────
    pub fn gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.6875,
        }
    }
    pub fn padding_inline_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.75,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_without_output_is_not_interactive() {
        assert!(!ToolCallSpec::new("a", "Ran command").is_interactive());
        assert!(ToolCallSpec::new("a", "Ran command")
            .with_output("ok")
            .is_interactive());
    }

    #[test]
    fn empty_output_does_not_count_as_output() {
        assert!(!ToolCallSpec::new("a", "Ran command")
            .with_output("")
            .is_interactive());
    }

    #[test]
    fn the_icon_follows_the_label_unless_overridden() {
        assert_eq!(
            ToolCallSpec::new("a", "Ran command").resolved_icon(),
            "terminal"
        );
        assert_eq!(
            ToolCallSpec::new("a", "File change").resolved_icon(),
            "file-pen"
        );
        assert_eq!(
            ToolCallSpec::new("a", "Something else").resolved_icon(),
            "dot"
        );
        assert_eq!(
            ToolCallSpec::new("a", "Something else")
                .with_icon("sparkles")
                .resolved_icon(),
            "sparkles"
        );
    }

    #[test]
    fn the_name_carries_status_but_omits_success() {
        let ok = ToolCallSpec::new("a", "Ran command").with_detail("bun test");
        assert_eq!(ok.accessible_name(), "Ran command: bun test");

        let bad = ok.clone().with_status(ToolCallStatus::Error);
        assert_eq!(bad.accessible_name(), "Ran command: bun test, error");
    }

    #[test]
    fn density_never_changes_row_height() {
        let base = ToolCallSpec::new("a", "x");
        for density in [ControlDensity::Compact, ControlDensity::Comfortable] {
            assert_eq!(
                base.clone().with_density(density).row_height_rem(ControlSize::Md),
                base.row_height_rem(ControlSize::Md)
            );
        }
    }
}
