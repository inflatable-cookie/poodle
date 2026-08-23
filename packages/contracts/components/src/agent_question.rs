//! AgentQuestionSpec — the question an agent asks mid-turn.
//!
//! Contract: `docs/contracts/components/agent-question.md`.
//!
//! On the natives this renders a question and its selection state; it does not
//! drive selection, matching the render-only posture of every other native
//! control. See the contract's §14.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::agent_question::{
    question_progress, resolve_question_answer, shows_question_progress, submits_on_select,
    AgentQuestionAnswer, AgentQuestionItem, QuestionProgress,
};
use poodle_tokens::semantic;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AgentQuestionSpec {
    pub questions: Vec<AgentQuestionItem>,
    pub active_index: usize,
    pub selections: Vec<String>,
    /// The composer's editor text, so the answer can be resolved here.
    pub override_text: String,
    pub is_dismissible: bool,
    /// The web prop is a formatter, `(current, total) => string`. A Rust spec
    /// holds data rather than closures, so the native surface is an optional
    /// resolved override; `None` uses the default phrasing.
    pub progress_label: Option<String>,
    pub dismiss_label: String,
    pub show_shortcuts: bool,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
}

impl AgentQuestionSpec {
    pub fn new(questions: Vec<AgentQuestionItem>) -> Self {
        Self {
            questions,
            active_index: 0,
            selections: Vec::new(),
            override_text: String::new(),
            is_dismissible: false,
            progress_label: None,
            dismiss_label: "Skip this question".to_string(),
            show_shortcuts: true,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }

    pub fn with_active_index(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }
    pub fn with_selections(mut self, values: Vec<String>) -> Self {
        self.selections = values;
        self
    }
    pub fn with_override(mut self, text: impl Into<String>) -> Self {
        self.override_text = text.into();
        self
    }
    pub fn with_dismissible(mut self, dismissible: bool) -> Self {
        self.is_dismissible = dismissible;
        self
    }
    pub fn with_progress_label(mut self, label: impl Into<String>) -> Self {
        self.progress_label = Some(label.into());
        self
    }
    pub fn with_show_shortcuts(mut self, show: bool) -> Self {
        self.show_shortcuts = show;
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

    pub fn active_question(&self) -> Option<&AgentQuestionItem> {
        self.questions.get(self.active_index)
    }

    pub fn is_multi_select(&self) -> bool {
        self.active_question().is_some_and(|q| q.allow_multiple)
    }

    /// Single-select resolves on one click; multi-select never can.
    pub fn submits_on_select(&self) -> bool {
        submits_on_select(self.active_question())
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selections.iter().any(|entry| entry == value)
    }

    pub fn progress(&self) -> QuestionProgress {
        question_progress(&self.questions, self.active_index)
    }

    /// A lone question needs no picture of "1 of 1".
    pub fn shows_progress(&self) -> bool {
        shows_question_progress(&self.questions)
    }

    pub fn resolved_progress_label(&self) -> String {
        if let Some(label) = &self.progress_label {
            return label.clone();
        }
        let progress = self.progress();
        format!("{} of {}", progress.current, progress.total)
    }

    pub fn answer(&self) -> Option<AgentQuestionAnswer> {
        resolve_question_answer(
            self.active_question(),
            &self.selections,
            &self.override_text,
        )
    }

    /// Shortcuts run out after nine; the tenth option has no digit.
    pub fn shortcut_for(&self, index: usize) -> Option<usize> {
        if self.show_shortcuts && index < 9 {
            Some(index + 1)
        } else {
            None
        }
    }

    // ── Tokens ────────────────────────────────────────────────
    pub fn prompt_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }
    pub fn option_label_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }
    pub fn option_description_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
    pub fn option_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
    pub fn accent_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }
    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }
    pub fn shortcut_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }
    pub fn progress_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }
    pub fn dismiss_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    // ── Size ─────────────────────────────────────────────────
    pub fn font_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }
    pub fn prompt_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.8125,
            ControlSize::Sm => 0.875,
            ControlSize::Md => 0.9375,
            ControlSize::Lg => 1.0,
            ControlSize::Xl => 1.125,
        }
    }
    pub fn option_padding_block_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.375,
            ControlSize::Sm => 0.4375,
            ControlSize::Md => 0.5,
            ControlSize::Lg => 0.5625,
            ControlSize::Xl => 0.625,
        }
    }

    // ── Density ──────────────────────────────────────────────
    pub fn gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.75,
        }
    }
    /// The step between the question and its answers, larger than the gap that
    /// stacks progress, header and prompt — those are one unit, and the options
    /// answer them.
    pub fn prompt_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.875,
            ControlDensity::Default => 1.125,
            ControlDensity::Comfortable => 1.5,
        }
    }
    pub fn option_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.125,
            ControlDensity::Default => 0.25,
            ControlDensity::Comfortable => 0.375,
        }
    }
    pub fn option_padding_inline_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.625,
            ControlDensity::Comfortable => 0.875,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_question::{AgentQuestionOption, AgentQuestionOutcome};

    fn question(allow_multiple: bool) -> AgentQuestionItem {
        AgentQuestionItem {
            id: "placement".into(),
            prompt: "Where?".into(),
            options: vec![
                AgentQuestionOption {
                    value: "inline".into(),
                    label: "Inline".into(),
                    description: None,
                },
                AgentQuestionOption {
                    value: "composer".into(),
                    label: "Composer".into(),
                    description: None,
                },
            ],
            allow_multiple,
            ..Default::default()
        }
    }

    #[test]
    fn single_select_resolves_on_one_click_and_multi_never_does() {
        assert!(AgentQuestionSpec::new(vec![question(false)]).submits_on_select());
        assert!(!AgentQuestionSpec::new(vec![question(true)]).submits_on_select());
    }

    #[test]
    fn an_override_wins_over_a_selection() {
        let spec = AgentQuestionSpec::new(vec![question(false)])
            .with_selections(vec!["inline".into()])
            .with_override("somewhere else");
        let answer = spec.answer().expect("an answer");

        assert_eq!(answer.outcome, AgentQuestionOutcome::Override);
        assert!(answer.values.is_empty());
    }

    #[test]
    fn a_lone_question_shows_no_progress() {
        assert!(!AgentQuestionSpec::new(vec![question(false)]).shows_progress());
        assert!(AgentQuestionSpec::new(vec![question(false), question(false)]).shows_progress());
    }

    #[test]
    fn shortcuts_run_out_after_nine() {
        let spec = AgentQuestionSpec::new(vec![question(false)]);
        assert_eq!(spec.shortcut_for(0), Some(1));
        assert_eq!(spec.shortcut_for(8), Some(9));
        assert_eq!(spec.shortcut_for(9), None);
        assert_eq!(
            spec.clone().with_show_shortcuts(false).shortcut_for(0),
            None
        );
    }

    #[test]
    fn density_moves_spacing_but_not_the_type_scale() {
        let base = AgentQuestionSpec::new(vec![question(false)]);
        let dense = base.clone().with_density(ControlDensity::Compact);

        assert_ne!(
            dense.gap_rem(ControlDensity::Compact),
            base.gap_rem(ControlDensity::Default)
        );
        assert_eq!(
            dense.font_size_rem(ControlSize::Md),
            base.font_size_rem(ControlSize::Md)
        );
        assert_eq!(
            dense.prompt_size_rem(ControlSize::Md),
            base.prompt_size_rem(ControlSize::Md)
        );
    }
}
