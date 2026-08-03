//! Agent question machinery. Mirror of core `agent-question.ts`: answer
//! resolution, selection toggling, batch progress, and the record an answered
//! question leaves in the transcript.
//!
//! Contract: `docs/contracts/components/agent-question.md`.
//!
//! Parity with the TS core is enforced by `vectors/agent-question.json`, run by
//! both runtimes.

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AgentQuestionOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AgentQuestionItem {
    pub id: String,
    /// Short label shown as an eyebrow above the prompt.
    pub header: Option<String>,
    pub prompt: String,
    pub options: Vec<AgentQuestionOption>,
    /// Opt-in, per question.
    ///
    /// Single-select is the default because it can resolve on one click: the
    /// first click is also the last. With several answers a click cannot be
    /// told from a first-of-several, so multi-select always needs an explicit
    /// submit.
    pub allow_multiple: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentQuestionOutcome {
    Selected,
    Override,
    Declined,
}

impl AgentQuestionOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            AgentQuestionOutcome::Selected => "selected",
            AgentQuestionOutcome::Override => "override",
            AgentQuestionOutcome::Declined => "declined",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentQuestionAnswer {
    pub question_id: String,
    pub outcome: AgentQuestionOutcome,
    /// Chosen option values. Empty for `Override` and `Declined`.
    pub values: Vec<String>,
    /// The free-text answer. Empty unless `outcome` is `Override`.
    pub text: String,
}

/// True when a click on an option should also resolve the question.
pub fn submits_on_select(question: Option<&AgentQuestionItem>) -> bool {
    question.is_some_and(|q| !q.allow_multiple)
}

/// Apply a click on an option.
///
/// Single-select replaces; multi-select toggles.
pub fn toggle_question_selection(
    question: Option<&AgentQuestionItem>,
    selections: &[String],
    value: &str,
) -> Vec<String> {
    let Some(question) = question else {
        return Vec::new();
    };

    if !question.allow_multiple {
        return vec![value.to_string()];
    }

    if selections.iter().any(|entry| entry == value) {
        selections
            .iter()
            .filter(|entry| *entry != value)
            .cloned()
            .collect()
    } else {
        let mut next = selections.to_vec();
        next.push(value.to_string());
        next
    }
}

/// Resolve the answer for a question, given what is selected and what is typed.
///
/// Override wins. Typing clears the selection rather than the editor locking
/// once an option is picked: locking traps the reader, who ticks a box, finds
/// none of the options fit, and then has to untick before they can type.
pub fn resolve_question_answer(
    question: Option<&AgentQuestionItem>,
    selections: &[String],
    override_text: &str,
) -> Option<AgentQuestionAnswer> {
    let question = question?;

    let text = override_text.trim();
    if !text.is_empty() {
        return Some(AgentQuestionAnswer {
            question_id: question.id.clone(),
            outcome: AgentQuestionOutcome::Override,
            values: Vec::new(),
            text: text.to_string(),
        });
    }

    if !selections.is_empty() {
        // Answer order follows the question's options, not the order they were
        // clicked: the agent reads a set, and click order is not information.
        let ordered: Vec<String> = question
            .options
            .iter()
            .map(|option| option.value.clone())
            .filter(|value| selections.contains(value))
            .collect();

        return Some(AgentQuestionAnswer {
            question_id: question.id.clone(),
            outcome: AgentQuestionOutcome::Selected,
            values: ordered,
            text: String::new(),
        });
    }

    None
}

/// Dismissal resolves the question as declined.
///
/// A turn cannot finish with an open question, so dismissal has to send
/// something. It is a resolution, not an escape from one.
pub fn decline_question(question: &AgentQuestionItem) -> AgentQuestionAnswer {
    AgentQuestionAnswer {
        question_id: question.id.clone(),
        outcome: AgentQuestionOutcome::Declined,
        values: Vec::new(),
        text: String::new(),
    }
}

/// True when an answer can be submitted from this state.
pub fn can_submit_question(
    question: Option<&AgentQuestionItem>,
    selections: &[String],
    override_text: &str,
) -> bool {
    resolve_question_answer(question, selections, override_text).is_some()
}

// ── Batch progress ──

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuestionProgressState {
    Answered,
    Current,
    Pending,
}

impl QuestionProgressState {
    pub fn as_str(self) -> &'static str {
        match self {
            QuestionProgressState::Answered => "answered",
            QuestionProgressState::Current => "current",
            QuestionProgressState::Pending => "pending",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QuestionProgress {
    /// One entry per question, in order.
    pub states: Vec<QuestionProgressState>,
    /// 1-based position, for the "2 of 4" label.
    pub current: usize,
    pub total: usize,
}

impl Default for QuestionProgressState {
    fn default() -> Self {
        QuestionProgressState::Pending
    }
}

/// Position in a batch.
///
/// Reported, never navigable: going back would mean changing an answer the
/// agent already has.
pub fn question_progress(questions: &[AgentQuestionItem], active_index: usize) -> QuestionProgress {
    let total = questions.len();
    let clamped = if total == 0 {
        0
    } else {
        active_index.min(total - 1)
    };

    QuestionProgress {
        states: (0..total)
            .map(|index| match index.cmp(&clamped) {
                std::cmp::Ordering::Less => QuestionProgressState::Answered,
                std::cmp::Ordering::Equal => QuestionProgressState::Current,
                std::cmp::Ordering::Greater => QuestionProgressState::Pending,
            })
            .collect(),
        current: if total == 0 { 0 } else { clamped + 1 },
        total,
    }
}

/// Progress is chrome for a batch; a lone question needs no picture of "1 of 1".
pub fn shows_question_progress(questions: &[AgentQuestionItem]) -> bool {
    questions.len() > 1
}

/// The next index after resolving one, clamped at the end of the batch.
pub fn next_question_index(questions: &[AgentQuestionItem], active_index: usize) -> usize {
    (active_index + 1).min(questions.len())
}

/// True once every question has been resolved and the turn may continue.
pub fn question_batch_complete(questions: &[AgentQuestionItem], active_index: usize) -> bool {
    active_index >= questions.len()
}

// ── The record an answered question leaves ──

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnsweredQuestion {
    pub question: AgentQuestionItem,
    pub answer: AgentQuestionAnswer,
}

/// What the transcript shows for an answered question.
///
/// Read-only by construction: the pending question lives in the composer, and
/// this is the record it leaves behind, so there is never a second input on
/// screen.
pub fn answered_question_summary(record: &AnsweredQuestion) -> String {
    match record.answer.outcome {
        AgentQuestionOutcome::Declined => "Declined".to_string(),
        AgentQuestionOutcome::Override => record.answer.text.clone(),
        AgentQuestionOutcome::Selected => record
            .answer
            .values
            .iter()
            .map(|value| {
                record
                    .question
                    .options
                    .iter()
                    .find(|option| &option.value == value)
                    .map(|option| option.label.clone())
                    .unwrap_or_else(|| value.clone())
            })
            .collect::<Vec<_>>()
            .join(", "),
    }
}

/// True when an option was the one chosen, for rendering the record.
pub fn is_chosen_option(record: &AnsweredQuestion, value: &str) -> bool {
    record.answer.outcome == AgentQuestionOutcome::Selected
        && record.answer.values.iter().any(|entry| entry == value)
}
