//! AgentQuestionRecordSpec — the read-only record an answered question leaves.
//!
//! Contract: `docs/contracts/components/agent-question-record.md`.
//!
//! No interactive parts at all, which is what makes hosting the live question
//! in the composer safe: a re-answer affordance here would let the reader
//! change an answer the agent has already acted on.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::agent_question::{
    answered_question_summary, is_chosen_option, AgentQuestionAnswer, AgentQuestionItem,
    AgentQuestionOutcome, AnsweredQuestion,
};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct AgentQuestionRecordSpec {
    pub question: AgentQuestionItem,
    pub answer: AgentQuestionAnswer,
    pub show_options: bool,
    pub declined_label: String,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
}

impl AgentQuestionRecordSpec {
    pub fn new(question: AgentQuestionItem, answer: AgentQuestionAnswer) -> Self {
        Self {
            question,
            answer,
            show_options: true,
            declined_label: "Declined".to_string(),
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }

    pub fn with_show_options(mut self, show: bool) -> Self {
        self.show_options = show;
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

    fn record(&self) -> AnsweredQuestion {
        AnsweredQuestion {
            question: self.question.clone(),
            answer: self.answer.clone(),
        }
    }

    /// An override or a declined question took no option, so there is no list
    /// to show.
    pub fn shows_options(&self) -> bool {
        self.show_options && self.answer.outcome == AgentQuestionOutcome::Selected
    }

    pub fn is_chosen(&self, value: &str) -> bool {
        is_chosen_option(&self.record(), value)
    }

    pub fn summary(&self) -> String {
        if self.answer.outcome == AgentQuestionOutcome::Declined {
            return self.declined_label.clone();
        }
        answered_question_summary(&self.record())
    }

    /// The tick alone is not the signal; the chosen option says so in words.
    pub fn option_accessible_name(&self, value: &str, label: &str) -> String {
        if self.is_chosen(value) {
            format!("chosen: {label}")
        } else {
            label.to_string()
        }
    }

    // ── Tokens ────────────────────────────────────────────────
    pub fn surface_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }
    /// One step down from live prose: this is history, not the current subject.
    pub fn prompt_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
    pub fn chosen_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }
    pub fn unchosen_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }
    pub fn mark_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
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

    // ── Density ──────────────────────────────────────────────
    pub fn gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.5,
        }
    }
    pub fn padding_inset_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_question::AgentQuestionOption;

    fn question() -> AgentQuestionItem {
        AgentQuestionItem {
            id: "placement".to_string(),
            prompt: "Where?".to_string(),
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
            ..Default::default()
        }
    }

    fn answer(outcome: AgentQuestionOutcome, values: Vec<&str>, text: &str) -> AgentQuestionAnswer {
        AgentQuestionAnswer {
            question_id: "placement".to_string(),
            outcome,
            values: values.into_iter().map(str::to_string).collect(),
            text: text.to_string(),
        }
    }

    #[test]
    fn an_override_shows_no_option_list() {
        // Nothing was taken, so there are no alternatives to weigh it against.
        let spec = AgentQuestionRecordSpec::new(
            question(),
            answer(AgentQuestionOutcome::Override, vec![], "somewhere else"),
        );
        assert!(!spec.shows_options());
        assert_eq!(spec.summary(), "somewhere else");
    }

    #[test]
    fn a_declined_question_reads_as_declined() {
        let spec = AgentQuestionRecordSpec::new(
            question(),
            answer(AgentQuestionOutcome::Declined, vec![], ""),
        );
        assert!(!spec.shows_options());
        assert_eq!(spec.summary(), "Declined");
    }

    #[test]
    fn every_option_survives_a_selected_answer() {
        // "Why did it pick that" is usually answered by what it did not pick.
        let spec = AgentQuestionRecordSpec::new(
            question(),
            answer(AgentQuestionOutcome::Selected, vec!["composer"], ""),
        );
        assert!(spec.shows_options());
        assert_eq!(spec.question.options.len(), 2);
        assert!(spec.is_chosen("composer"));
        assert!(!spec.is_chosen("inline"));
    }

    #[test]
    fn the_chosen_option_says_so_in_words() {
        let spec = AgentQuestionRecordSpec::new(
            question(),
            answer(AgentQuestionOutcome::Selected, vec!["composer"], ""),
        );
        assert_eq!(
            spec.option_accessible_name("composer", "Composer"),
            "chosen: Composer"
        );
        assert_eq!(spec.option_accessible_name("inline", "Inline"), "Inline");
    }
}
