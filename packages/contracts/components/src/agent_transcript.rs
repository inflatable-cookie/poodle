//! AgentTranscriptSpec — the output surface of an agent conversation.
//!
//! Contract: `docs/contracts/components/agent-transcript.md`.
//!
//! `AgentChatInput` owns the composer and scopes out the transcript; this is
//! the other side of that boundary. Grouping and windowing come from
//! `poodle-headless`, shared with the web target through JSON vectors.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::agent_transcript::{
    group_transcript_items, transcript_window, TranscriptBlock, TranscriptItem, TranscriptWindow,
};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct AgentTranscriptSpec {
    pub items: Vec<TranscriptItem>,
    pub is_virtualized: bool,
    pub estimated_block_height: f32,
    pub overscan: usize,
    pub is_auto_scroll: bool,
    pub pin_threshold: f32,
    pub jump_label: String,
    pub aria_label: String,
    pub empty_label: String,
    pub expanded_tool_runs: Vec<String>,
    pub expanded_tool_calls: Vec<String>,
    pub expanded_changed_files: Vec<String>,
    pub expanded_subagent_groups: Vec<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for AgentTranscriptSpec {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl AgentTranscriptSpec {
    pub fn new(items: Vec<TranscriptItem>) -> Self {
        Self {
            items,
            // The natives render unwindowed: neither runtime measures blocks
            // during spec resolution, and Jetstream's scroll container
            // materializes every child regardless. See the contract's deltas.
            is_virtualized: false,
            estimated_block_height: 120.0,
            overscan: 3,
            is_auto_scroll: true,
            pin_threshold: 32.0,
            jump_label: "Jump to latest".to_string(),
            aria_label: "Conversation".to_string(),
            empty_label: "No messages yet".to_string(),
            expanded_tool_runs: Vec::new(),
            expanded_tool_calls: Vec::new(),
            expanded_changed_files: Vec::new(),
            expanded_subagent_groups: Vec::new(),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_virtualized(mut self, virtualized: bool) -> Self {
        self.is_virtualized = virtualized;
        self
    }
    pub fn with_estimated_block_height(mut self, height: f32) -> Self {
        self.estimated_block_height = height;
        self
    }
    pub fn with_overscan(mut self, overscan: usize) -> Self {
        self.overscan = overscan;
        self
    }
    pub fn with_auto_scroll(mut self, auto_scroll: bool) -> Self {
        self.is_auto_scroll = auto_scroll;
        self
    }
    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = label.into();
        self
    }
    pub fn with_empty_label(mut self, label: impl Into<String>) -> Self {
        self.empty_label = label.into();
        self
    }
    pub fn with_expanded_tool_runs(mut self, ids: Vec<String>) -> Self {
        self.expanded_tool_runs = ids;
        self
    }
    pub fn with_expanded_tool_calls(mut self, ids: Vec<String>) -> Self {
        self.expanded_tool_calls = ids;
        self
    }
    pub fn with_expanded_changed_files(mut self, ids: Vec<String>) -> Self {
        self.expanded_changed_files = ids;
        self
    }
    pub fn with_expanded_subagent_groups(mut self, ids: Vec<String>) -> Self {
        self.expanded_subagent_groups = ids;
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

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Derived, never stored: a stored copy would drift from `items` on append.
    pub fn blocks(&self) -> Vec<TranscriptBlock> {
        group_transcript_items(&self.items)
    }

    /// The activity footer is the last activity item, drawn outside the block
    /// flow so it stays pinned under the transcript rather than scrolling as a
    /// block of its own.
    pub fn activity_label(&self) -> Option<&str> {
        self.items.iter().rev().find_map(|item| match item {
            TranscriptItem::Activity(activity) => Some(activity.label.as_str()),
            _ => None,
        })
    }

    pub fn rendered_blocks(&self) -> Vec<TranscriptBlock> {
        self.blocks()
            .into_iter()
            .filter(|block| !matches!(block, TranscriptBlock::Activity(_)))
            .collect()
    }

    /// Only meaningful when the host drives a scroll position; the natives
    /// render unwindowed by default.
    pub fn window(
        &self,
        heights: &[f64],
        scroll_top: f64,
        viewport_height: f64,
    ) -> TranscriptWindow {
        transcript_window(
            heights,
            self.estimated_block_height as f64,
            scroll_top,
            viewport_height,
            self.overscan,
        )
    }

    // ── Tokens ────────────────────────────────────────────────
    pub fn activity_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
    pub fn jump_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
    pub fn jump_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
    pub fn jump_radius_token(&self) -> &'static str {
        semantic::RADIUS_PILL
    }
    pub fn jump_shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }
    pub fn jump_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
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

    // ── Density ──────────────────────────────────────────────
    pub fn padding_inset_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.75,
            ControlDensity::Default => 1.0,
            ControlDensity::Comfortable => 1.5,
        }
    }
    pub fn block_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.875,
            ControlDensity::Default => 1.25,
            ControlDensity::Comfortable => 1.75,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_transcript::{
        ToolCallStatus, TranscriptActivity, TranscriptMessage, TranscriptToolCall,
    };

    fn call(id: &str) -> TranscriptItem {
        TranscriptItem::ToolCall(TranscriptToolCall {
            id: id.to_string(),
            label: "Ran command".to_string(),
            status: ToolCallStatus::Success,
            ..Default::default()
        })
    }

    fn message(id: &str) -> TranscriptItem {
        TranscriptItem::Message(TranscriptMessage {
            id: id.to_string(),
            markdown: "text".to_string(),
            ..Default::default()
        })
    }

    #[test]
    fn adjacent_tool_calls_group_and_a_message_splits_them() {
        let spec = AgentTranscriptSpec::new(vec![call("a"), call("b"), message("m"), call("c")]);
        let kinds: Vec<&str> = spec.blocks().iter().map(TranscriptBlock::kind).collect();

        assert_eq!(kinds, vec!["tool-run", "message", "tool-run"]);
    }

    #[test]
    fn the_activity_footer_is_not_a_block() {
        let spec = AgentTranscriptSpec::new(vec![
            message("m"),
            TranscriptItem::Activity(TranscriptActivity {
                id: "act".to_string(),
                label: "Working for 1h 1m".to_string(),
                spinning: None,
            }),
        ]);

        assert_eq!(spec.activity_label(), Some("Working for 1h 1m"));
        assert_eq!(spec.rendered_blocks().len(), 1);
    }

    #[test]
    fn the_natives_default_to_unwindowed() {
        // Neither runtime measures blocks during spec resolution.
        assert!(!AgentTranscriptSpec::default().is_virtualized);
    }

    #[test]
    fn density_moves_spacing_but_not_the_type_scale() {
        let base = AgentTranscriptSpec::default();
        let dense = base.clone().with_density(ControlDensity::Compact);

        assert_ne!(dense.block_gap_rem(), base.block_gap_rem());
        assert_eq!(dense.font_size_rem(), base.font_size_rem());
    }
}
