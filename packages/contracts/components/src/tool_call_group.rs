//! ToolCallGroupSpec — a contiguous run of tool calls as one unit.
//!
//! Contract: `docs/contracts/components/tool-call-group.md`.
//!
//! A turn can contain dozens of tool calls and a transcript listing all of them
//! is unreadable. A run compresses to one row plus a count.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::agent_transcript::{ToolCallStatus, TranscriptToolCall, TranscriptToolRun};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct ToolCallGroupSpec {
    pub id: String,
    pub calls: Vec<TranscriptToolCall>,
    pub is_expanded: bool,
    pub expanded_calls: Vec<String>,
    /// The web prop is a formatter, `(count) => string`. A Rust spec holds data
    /// rather than closures, so the native surface is an optional resolved
    /// override; `None` uses the default phrasing. See the contract's deltas.
    pub more_label: Option<String>,
    pub fewer_label: String,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for ToolCallGroupSpec {
    fn default() -> Self {
        Self::new("", Vec::new())
    }
}

impl ToolCallGroupSpec {
    pub fn new(id: impl Into<String>, calls: Vec<TranscriptToolCall>) -> Self {
        Self {
            id: id.into(),
            calls,
            is_expanded: false,
            expanded_calls: Vec::new(),
            more_label: None,
            fewer_label: "Show fewer tool calls".to_string(),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }
    pub fn with_expanded_calls(mut self, ids: Vec<String>) -> Self {
        self.expanded_calls = ids;
        self
    }
    pub fn with_more_label(mut self, label: impl Into<String>) -> Self {
        self.more_label = Some(label.into());
        self
    }
    pub fn with_fewer_label(mut self, label: impl Into<String>) -> Self {
        self.fewer_label = label.into();
        self
    }
    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    fn run(&self) -> TranscriptToolRun {
        TranscriptToolRun {
            id: self.id.clone(),
            calls: self.calls.clone(),
        }
    }

    pub fn hidden_count(&self) -> usize {
        self.calls.len().saturating_sub(1)
    }

    /// Omitted entirely rather than disabled when there is nothing to reveal,
    /// so a single-call run leaves no stray tab stop.
    pub fn shows_toggle(&self) -> bool {
        self.hidden_count() > 0
    }

    pub fn status(&self) -> ToolCallStatus {
        self.run().status()
    }

    /// Collapsed shows the run's newest call; expanded lists every call in
    /// order and therefore ends on that same call, so expanding never moves the
    /// row you were reading.
    pub fn rendered_calls(&self) -> &[TranscriptToolCall] {
        if self.is_expanded || self.calls.is_empty() {
            &self.calls
        } else {
            &self.calls[self.calls.len() - 1..]
        }
    }

    pub fn resolved_more_label(&self) -> String {
        self.more_label
            .clone()
            .unwrap_or_else(|| format!("+{} previous tool calls", self.hidden_count()))
    }

    /// A collapsed failing run must not be announced identically to a passing
    /// one, so a non-success status is carried in the name as well as colour.
    pub fn toggle_accessible_name(&self) -> String {
        if self.is_expanded {
            return self.fewer_label.clone();
        }
        let suffix = match self.status() {
            ToolCallStatus::Error => ", contains a failure",
            ToolCallStatus::Running => ", in progress",
            ToolCallStatus::Success => "",
        };
        format!("{}{suffix}", self.resolved_more_label())
    }

    // ── Tokens ────────────────────────────────────────────────
    pub fn toggle_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
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

    // ── Size ─────────────────────────────────────────────────
    pub fn font_size_rem(&self) -> f32 {
        match self.size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }
    pub fn icon_size_rem(&self) -> f32 {
        match self.size {
            ControlSize::Xs => 0.75,
            ControlSize::Sm => 0.8125,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 1.0,
            ControlSize::Xl => 1.125,
        }
    }

    // ── Density ──────────────────────────────────────────────
    pub fn row_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.0,
            ControlDensity::Default => 0.125,
            ControlDensity::Comfortable => 0.25,
        }
    }
    pub fn gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.6875,
        }
    }
    pub fn padding_inline_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.75,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(id: &str, status: ToolCallStatus) -> TranscriptToolCall {
        TranscriptToolCall {
            id: id.to_string(),
            label: "Ran command".to_string(),
            status,
            ..Default::default()
        }
    }

    #[test]
    fn collapsed_shows_the_newest_call_not_the_oldest() {
        let spec = ToolCallGroupSpec::new(
            "run",
            vec![
                call("a", ToolCallStatus::Success),
                call("b", ToolCallStatus::Success),
                call("c", ToolCallStatus::Success),
            ],
        );
        assert_eq!(spec.rendered_calls().len(), 1);
        assert_eq!(spec.rendered_calls()[0].id, "c");
        assert_eq!(spec.hidden_count(), 2);
    }

    #[test]
    fn expanded_ends_on_the_call_that_was_visible_while_collapsed() {
        let calls = vec![
            call("a", ToolCallStatus::Success),
            call("b", ToolCallStatus::Success),
        ];
        let collapsed = ToolCallGroupSpec::new("run", calls.clone());
        let expanded = collapsed.clone().with_expanded(true);

        assert_eq!(
            expanded.rendered_calls().last().map(|c| c.id.as_str()),
            collapsed.rendered_calls().last().map(|c| c.id.as_str())
        );
    }

    #[test]
    fn a_single_call_run_renders_no_toggle() {
        assert!(!ToolCallGroupSpec::new("run", vec![call("a", ToolCallStatus::Success)]).shows_toggle());
    }

    #[test]
    fn a_buried_failure_still_reaches_the_collapsed_name() {
        // The whole reason run status exists: the failure is not the newest
        // call, so without this it is invisible until someone expands.
        let spec = ToolCallGroupSpec::new(
            "run",
            vec![
                call("a", ToolCallStatus::Success),
                call("b", ToolCallStatus::Error),
                call("c", ToolCallStatus::Success),
            ],
        );
        assert_eq!(spec.status(), ToolCallStatus::Error);
        assert!(spec.toggle_accessible_name().contains("contains a failure"));
    }

    #[test]
    fn an_empty_run_does_not_panic() {
        assert_eq!(ToolCallGroupSpec::new("run", Vec::new()).rendered_calls().len(), 0);
    }
}
