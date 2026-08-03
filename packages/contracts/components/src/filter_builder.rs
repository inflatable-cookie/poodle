//! FilterBuilderSpec — generic filter-expression model, Rust mirror of the
//! `@poodle/svelte` FilterBuilder type model + `filter-builder-model.ts` logic.
//!
//! Contract: `docs/contracts/components/filter-builder.md`.
//!
//! Poodle understands fields, operators, operands and a single root AND/OR
//! combinator. It never understands application vocabulary and never evaluates
//! the expression — the host owns evaluation and serialization.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FilterCombinator {
    And,
    Or,
}

impl Default for FilterCombinator {
    fn default() -> Self {
        Self::And
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FilterFieldKind {
    Boolean,
    Enum,
    MultiEnum,
    Text,
    Number,
    Range,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FilterOperandKind {
    None,
    Text,
    Number,
    Boolean,
    Options,
    Range,
}

/// Discriminated operand payload. Never collapse to an untyped value —
/// cross-renderer semantics stay explicit.
#[derive(Clone, Debug, PartialEq)]
pub enum FilterOperand {
    None,
    Text(String),
    Number(f64),
    Boolean(bool),
    Options(Vec<String>),
    Range { min: Option<f64>, max: Option<f64> },
}

impl FilterOperand {
    pub fn kind(&self) -> FilterOperandKind {
        match self {
            FilterOperand::None => FilterOperandKind::None,
            FilterOperand::Text(_) => FilterOperandKind::Text,
            FilterOperand::Number(_) => FilterOperandKind::Number,
            FilterOperand::Boolean(_) => FilterOperandKind::Boolean,
            FilterOperand::Options(_) => FilterOperandKind::Options,
            FilterOperand::Range { .. } => FilterOperandKind::Range,
        }
    }

    /// A blank operand of the given kind, used to seed a draft. Number uses NaN
    /// as the unset sentinel so `0` remains a valid entered value.
    pub fn empty(kind: FilterOperandKind) -> Self {
        match kind {
            FilterOperandKind::None => FilterOperand::None,
            FilterOperandKind::Text => FilterOperand::Text(String::new()),
            FilterOperandKind::Number => FilterOperand::Number(f64::NAN),
            FilterOperandKind::Boolean => FilterOperand::Boolean(true),
            FilterOperandKind::Options => FilterOperand::Options(Vec::new()),
            FilterOperandKind::Range => FilterOperand::Range {
                min: None,
                max: None,
            },
        }
    }

    /// Whether the operand carries enough data to commit.
    pub fn is_valid(&self) -> bool {
        match self {
            FilterOperand::None => true,
            FilterOperand::Text(value) => !value.trim().is_empty(),
            FilterOperand::Number(value) => value.is_finite(),
            FilterOperand::Boolean(_) => true,
            FilterOperand::Options(values) => !values.is_empty(),
            FilterOperand::Range { min, max } => min.is_some() || max.is_some(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterOption {
    pub value: String,
    pub label: String,
    pub is_disabled: bool,
    pub group: Option<String>,
}

impl FilterOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
            group: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterOperatorDefinition {
    pub key: String,
    pub label: String,
    pub operand_kind: FilterOperandKind,
}

impl FilterOperatorDefinition {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<String>,
        operand_kind: FilterOperandKind,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            operand_kind,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterFieldDefinition {
    pub key: String,
    pub label: String,
    pub kind: FilterFieldKind,
    pub operators: Vec<FilterOperatorDefinition>,
    pub options: Vec<FilterOption>,
    pub default_operator: Option<String>,
    pub allow_multiple: bool,
    pub is_disabled: bool,
}

impl FilterFieldDefinition {
    pub fn new(key: impl Into<String>, label: impl Into<String>, kind: FilterFieldKind) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            kind,
            operators: Vec::new(),
            options: Vec::new(),
            default_operator: None,
            allow_multiple: false,
            is_disabled: false,
        }
    }

    pub fn with_options(mut self, options: Vec<FilterOption>) -> Self {
        self.options = options;
        self
    }

    pub fn with_operators(mut self, operators: Vec<FilterOperatorDefinition>) -> Self {
        self.operators = operators;
        self
    }

    pub fn with_default_operator(mut self, key: impl Into<String>) -> Self {
        self.default_operator = Some(key.into());
        self
    }

    pub fn with_allow_multiple(mut self, allow_multiple: bool) -> Self {
        self.allow_multiple = allow_multiple;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    /// Standard operator set for this field's kind, or the custom override.
    pub fn resolved_operators(&self) -> Vec<FilterOperatorDefinition> {
        if self.operators.is_empty() {
            default_operators_for_kind(self.kind)
        } else {
            self.operators.clone()
        }
    }

    /// The operator key a fresh draft should start on.
    pub fn resolved_default_operator(&self) -> String {
        let operators = self.resolved_operators();
        if let Some(key) = &self.default_operator {
            if operators.iter().any(|op| &op.key == key) {
                return key.clone();
            }
        }
        operators
            .first()
            .map(|op| op.key.clone())
            .unwrap_or_default()
    }

    pub fn find_operator(&self, operator_key: &str) -> Option<FilterOperatorDefinition> {
        self.resolved_operators()
            .into_iter()
            .find(|op| op.key == operator_key)
    }

    fn option_label(&self, value: &str) -> String {
        self.options
            .iter()
            .find(|option| option.value == value)
            .map(|option| option.label.clone())
            .unwrap_or_else(|| value.to_string())
    }
}

/// Standard operator set for each field kind. A field may restrict or relabel
/// these via `operators`.
pub fn default_operators_for_kind(kind: FilterFieldKind) -> Vec<FilterOperatorDefinition> {
    use FilterOperandKind as K;
    match kind {
        FilterFieldKind::Boolean => vec![FilterOperatorDefinition::new("is", "is", K::Boolean)],
        FilterFieldKind::Enum => vec![
            FilterOperatorDefinition::new("is", "is", K::Options),
            FilterOperatorDefinition::new("is_not", "is not", K::Options),
        ],
        FilterFieldKind::MultiEnum => vec![
            FilterOperatorDefinition::new("any_of", "is any of", K::Options),
            FilterOperatorDefinition::new("all_of", "is all of", K::Options),
            FilterOperatorDefinition::new("none_of", "is none of", K::Options),
        ],
        FilterFieldKind::Text => vec![
            FilterOperatorDefinition::new("contains", "contains", K::Text),
            FilterOperatorDefinition::new("not_contains", "does not contain", K::Text),
            FilterOperatorDefinition::new("equals", "equals", K::Text),
            FilterOperatorDefinition::new("starts_with", "starts with", K::Text),
            FilterOperatorDefinition::new("ends_with", "ends with", K::Text),
        ],
        FilterFieldKind::Number => vec![
            FilterOperatorDefinition::new("eq", "equals", K::Number),
            FilterOperatorDefinition::new("neq", "not equal", K::Number),
            FilterOperatorDefinition::new("gt", "greater than", K::Number),
            FilterOperatorDefinition::new("gte", "at least", K::Number),
            FilterOperatorDefinition::new("lt", "less than", K::Number),
            FilterOperatorDefinition::new("lte", "at most", K::Number),
        ],
        FilterFieldKind::Range => vec![
            FilterOperatorDefinition::new("between", "between", K::Range),
            FilterOperatorDefinition::new("outside", "outside", K::Range),
        ],
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterClause {
    pub id: String,
    pub key: String,
    pub operator: String,
    pub operand: FilterOperand,
}

impl FilterClause {
    pub fn new(
        id: impl Into<String>,
        key: impl Into<String>,
        operator: impl Into<String>,
        operand: FilterOperand,
    ) -> Self {
        Self {
            id: id.into(),
            key: key.into(),
            operator: operator.into(),
            operand,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterExpression {
    pub combinator: FilterCombinator,
    pub clauses: Vec<FilterClause>,
}

impl Default for FilterExpression {
    fn default() -> Self {
        Self {
            combinator: FilterCombinator::And,
            clauses: Vec::new(),
        }
    }
}

/// In-progress draft in the open popover: adding a new clause (`editing_id`
/// none) or editing an existing one (`editing_id` set). Mirrors the web's local
/// draft state so the render is a faithful function of the full component state.
#[derive(Clone, Debug, PartialEq)]
pub struct FilterDraft {
    pub key: String,
    pub operator: String,
    pub operand: FilterOperand,
    pub editing_id: Option<String>,
}

impl FilterDraft {
    /// Seed a draft for a field at its default operator with a blank operand.
    pub fn adding(field: &FilterFieldDefinition) -> Self {
        let operator = field.resolved_default_operator();
        let operand_kind = field
            .find_operator(&operator)
            .map(|op| op.operand_kind)
            .unwrap_or(FilterOperandKind::None);
        Self {
            key: field.key.clone(),
            operator,
            operand: FilterOperand::empty(operand_kind),
            editing_id: None,
        }
    }

    /// Load an existing clause into a draft for editing.
    pub fn editing(clause: &FilterClause) -> Self {
        Self {
            key: clause.key.clone(),
            operator: clause.operator.clone(),
            operand: clause.operand.clone(),
            editing_id: Some(clause.id.clone()),
        }
    }
}

#[derive(Clone)]
pub struct FilterBuilderSpec {
    pub fields: Vec<FilterFieldDefinition>,
    pub value: FilterExpression,
    pub aria_label: String,
    pub is_disabled: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    pub max_clauses: Option<usize>,
    pub is_compact: bool,
    pub show_clear_button: bool,
    pub show_pills: bool,
    pub show_combinator: bool,
    pub is_open: bool,
    /// Active draft in the open popover, if any.
    pub draft: Option<FilterDraft>,
    /// Which nested Select inside the open panel shows its option list.
    ///
    /// Native-only: on the web each Select owns its popup, but the native
    /// hosts hold every piece of state, including this nested one. At most
    /// one picker is open at a time, which is what the `Option` models.
    pub open_picker: Option<FilterBuilderPicker>,
}

/// The nested Selects inside the open filter panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilterBuilderPicker {
    /// The "+ Add filter" field Select.
    AddField,
    /// The draft clause's operator Select.
    Operator,
    /// The draft clause's operand Select (enum fields only).
    Operand,
}

impl FilterBuilderSpec {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            value: FilterExpression::default(),
            aria_label: "Filter".to_string(),
            is_disabled: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            max_clauses: None,
            is_compact: false,
            show_clear_button: true,
            show_pills: true,
            show_combinator: false,
            is_open: false,
            draft: None,
            open_picker: None,
        }
    }

    pub fn with_fields(mut self, fields: Vec<FilterFieldDefinition>) -> Self {
        self.fields = fields;
        self
    }

    pub fn with_value(mut self, value: FilterExpression) -> Self {
        self.value = value;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = label.into();
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_max_clauses(mut self, max_clauses: usize) -> Self {
        self.max_clauses = Some(max_clauses);
        self
    }

    pub fn with_compact(mut self, compact: bool) -> Self {
        self.is_compact = compact;
        self
    }

    pub fn with_show_clear_button(mut self, show: bool) -> Self {
        self.show_clear_button = show;
        self
    }

    pub fn with_show_pills(mut self, show: bool) -> Self {
        self.show_pills = show;
        self
    }

    pub fn with_show_combinator(mut self, show: bool) -> Self {
        self.show_combinator = show;
        self
    }

    pub fn with_draft(mut self, draft: FilterDraft) -> Self {
        self.draft = Some(draft);
        self
    }

    pub fn with_open_picker(mut self, picker: FilterBuilderPicker) -> Self {
        self.open_picker = Some(picker);
        self
    }

    /// Whether a draft (add or edit) is active in the open popover.
    pub fn is_drafting(&self) -> bool {
        self.draft.is_some()
    }

    /// Whether the active draft is editing an existing clause (vs adding).
    pub fn is_editing(&self) -> bool {
        self.draft.as_ref().is_some_and(|d| d.editing_id.is_some())
    }

    /// The field definition backing the active draft, if any.
    pub fn draft_field(&self) -> Option<&FilterFieldDefinition> {
        self.draft.as_ref().and_then(|d| self.field(&d.key))
    }

    /// Whether the active draft is complete enough to commit.
    pub fn is_draft_valid(&self) -> bool {
        match &self.draft {
            Some(draft) => self.is_clause_complete(&draft.key, &draft.operator, &draft.operand),
            None => false,
        }
    }

    /// Whether the add-field row shows (not drafting, room for more, fields left).
    pub fn show_add_row(&self) -> bool {
        !self.is_drafting() && self.can_add_more() && !self.available_fields().is_empty()
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn field(&self, key: &str) -> Option<&FilterFieldDefinition> {
        self.fields.iter().find(|field| field.key == key)
    }

    pub fn active_count(&self) -> usize {
        self.value.clauses.len()
    }

    pub fn has_value(&self) -> bool {
        !self.value.clauses.is_empty()
    }

    pub fn can_add_more(&self) -> bool {
        match self.max_clauses {
            Some(max) => self.active_count() < max,
            None => true,
        }
    }

    /// Fields still available to add. A field is unavailable when it already has
    /// a clause and does not allow multiples.
    pub fn available_fields(&self) -> Vec<&FilterFieldDefinition> {
        self.fields
            .iter()
            .filter(|field| !field.is_disabled)
            .filter(|field| {
                if field.allow_multiple {
                    return true;
                }
                !self
                    .value
                    .clauses
                    .iter()
                    .any(|clause| clause.key == field.key)
            })
            .collect()
    }

    pub fn combinator_visible(&self) -> bool {
        // Hidden while editing a chip — the combinator combines the whole stack,
        // not one clause (matches the Svelte/React `editingId === null` gate).
        self.show_combinator && self.value.clauses.len() >= 2 && !self.is_editing()
    }

    /// Whether a draft clause is complete: valid operator whose operand kind
    /// matches, with a valid operand.
    pub fn is_clause_complete(&self, key: &str, operator: &str, operand: &FilterOperand) -> bool {
        let Some(field) = self.field(key) else {
            return false;
        };
        let Some(op) = field.find_operator(operator) else {
            return false;
        };
        if op.operand_kind != operand.kind() {
            return false;
        }
        operand.is_valid()
    }

    /// Human-readable pill label for a committed clause.
    pub fn clause_label(&self, clause: &FilterClause) -> String {
        let Some(field) = self.field(&clause.key) else {
            return format!("{} {}", clause.key, clause.operator)
                .trim()
                .to_string();
        };
        let operator_label = field
            .find_operator(&clause.operator)
            .map(|op| op.label)
            .unwrap_or_else(|| clause.operator.clone());
        let value = self.operand_text(field, &clause.operand);
        if value.is_empty() {
            format!("{} {}", field.label, operator_label)
        } else {
            format!("{} {} {}", field.label, operator_label, value)
        }
    }

    fn operand_text(&self, field: &FilterFieldDefinition, operand: &FilterOperand) -> String {
        match operand {
            FilterOperand::None => String::new(),
            FilterOperand::Text(value) => format!("\"{value}\""),
            FilterOperand::Number(value) => {
                if value.is_finite() {
                    format_number(*value)
                } else {
                    String::new()
                }
            }
            FilterOperand::Boolean(value) => if *value { "true" } else { "false" }.to_string(),
            FilterOperand::Options(values) => values
                .iter()
                .map(|value| field.option_label(value))
                .collect::<Vec<_>>()
                .join(", "),
            FilterOperand::Range { min, max } => match (min, max) {
                (Some(min), Some(max)) => {
                    format!("{} – {}", format_number(*min), format_number(*max))
                }
                (Some(min), None) => format!("≥ {}", format_number(*min)),
                (None, Some(max)) => format!("≤ {}", format_number(*max)),
                (None, None) => String::new(),
            },
        }
    }

    /// Trigger summary text: "Filter" / "1 filter" / "N filters".
    pub fn summary_text(&self) -> String {
        match self.active_count() {
            0 => "Filter".to_string(),
            1 => "1 filter".to_string(),
            n => format!("{n} filters"),
        }
    }

    /// Opener label. When the combinator is live (opted in + 2+ clauses) this
    /// reflects the match mode — "All" / "Any" — so the mode is visible in the
    /// always-shown trigger; otherwise the static "Filter".
    pub fn opener_label(&self) -> &'static str {
        if self.combinator_visible() {
            match self.value.combinator {
                FilterCombinator::And => "All",
                FilterCombinator::Or => "Any",
            }
        } else {
            "Filter"
        }
    }

    // ── Token accessors (shared by GPUI + Jetstream) ──────────────────────

    pub fn label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Muted color for the empty-state summary placeholder. Maps to Svelte
    /// `--poodle-color-text-muted`.
    pub fn muted_color_token(&self) -> &'static str {
        "color.text.placeholder"
    }

    pub fn field_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn field_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn field_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn field_hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn count_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn count_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_INVERSE
    }

    pub fn surface_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn surface_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn item_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn reset_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
}

impl Default for FilterBuilderSpec {
    fn default() -> Self {
        Self::new()
    }
}

/// Render a number without a trailing `.0` for integral values.
fn format_number(value: f64) -> String {
    if value.fract() == 0.0 && value.is_finite() {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_spec() -> FilterBuilderSpec {
        FilterBuilderSpec::new().with_fields(vec![
            FilterFieldDefinition::new("format", "Format", FilterFieldKind::MultiEnum)
                .with_options(vec![
                    FilterOption::new("clap", "CLAP"),
                    FilterOption::new("vst3", "VST3"),
                ]),
            FilterFieldDefinition::new("hidden", "Hidden", FilterFieldKind::Boolean),
            FilterFieldDefinition::new("tag-count", "Tag count", FilterFieldKind::Number),
        ])
    }

    #[test]
    fn default_operators_cover_all_kinds() {
        assert_eq!(
            default_operators_for_kind(FilterFieldKind::Boolean).len(),
            1
        );
        assert_eq!(default_operators_for_kind(FilterFieldKind::Enum).len(), 2);
        assert_eq!(
            default_operators_for_kind(FilterFieldKind::MultiEnum).len(),
            3
        );
        assert_eq!(default_operators_for_kind(FilterFieldKind::Text).len(), 5);
        assert_eq!(default_operators_for_kind(FilterFieldKind::Number).len(), 6);
        assert_eq!(default_operators_for_kind(FilterFieldKind::Range).len(), 2);
    }

    #[test]
    fn operand_validity_rejects_incomplete() {
        assert!(!FilterOperand::Text(String::new()).is_valid());
        assert!(FilterOperand::Text("x".into()).is_valid());
        assert!(!FilterOperand::Number(f64::NAN).is_valid());
        assert!(FilterOperand::Number(0.0).is_valid());
        assert!(!FilterOperand::Options(vec![]).is_valid());
        assert!(FilterOperand::Boolean(false).is_valid());
        assert!(!FilterOperand::Range {
            min: None,
            max: None
        }
        .is_valid());
        assert!(FilterOperand::Range {
            min: Some(3.0),
            max: None
        }
        .is_valid());
    }

    #[test]
    fn clause_completeness_checks_operand_kind_match() {
        let spec = sample_spec();
        // any_of on a multi-enum needs Options.
        assert!(spec.is_clause_complete(
            "format",
            "any_of",
            &FilterOperand::Options(vec!["clap".into()])
        ));
        // Wrong operand kind → incomplete.
        assert!(!spec.is_clause_complete("format", "any_of", &FilterOperand::Text("clap".into())));
        // Unknown operator → incomplete.
        assert!(!spec.is_clause_complete(
            "format",
            "nope",
            &FilterOperand::Options(vec!["clap".into()])
        ));
    }

    #[test]
    fn clause_label_reads_naturally() {
        let spec = sample_spec();
        let clause = FilterClause::new(
            "format-1",
            "format",
            "any_of",
            FilterOperand::Options(vec!["clap".into(), "vst3".into()]),
        );
        assert_eq!(spec.clause_label(&clause), "Format is any of CLAP, VST3");

        let boolean = FilterClause::new("hidden-1", "hidden", "is", FilterOperand::Boolean(false));
        assert_eq!(spec.clause_label(&boolean), "Hidden is false");

        let number = FilterClause::new(
            "tag-count-1",
            "tag-count",
            "gte",
            FilterOperand::Number(3.0),
        );
        assert_eq!(spec.clause_label(&number), "Tag count at least 3");
    }

    #[test]
    fn summary_and_combinator_visibility() {
        // show_combinator gates the toggle; with it off it never shows.
        let mut spec = sample_spec();
        assert_eq!(spec.summary_text(), "Filter");
        assert!(!spec.combinator_visible());

        spec.value.clauses.push(FilterClause::new(
            "hidden-1",
            "hidden",
            "is",
            FilterOperand::Boolean(false),
        ));
        assert_eq!(spec.summary_text(), "1 filter");

        spec.value.clauses.push(FilterClause::new(
            "tag-count-1",
            "tag-count",
            "gte",
            FilterOperand::Number(3.0),
        ));
        assert_eq!(spec.summary_text(), "2 filters");
        // 2 clauses but show_combinator defaults off → hidden.
        assert!(!spec.combinator_visible());

        // Opt in → visible at 2+, still hidden at <2.
        spec.show_combinator = true;
        assert!(spec.combinator_visible());
        spec.value.clauses.pop();
        assert!(!spec.combinator_visible());
    }

    #[test]
    fn draft_state_and_edit_scoped_combinator() {
        let mut spec = sample_spec().with_show_combinator(true);
        spec.value.clauses.push(FilterClause::new(
            "f1",
            "format",
            "any_of",
            FilterOperand::Options(vec!["clap".into()]),
        ));
        spec.value.clauses.push(FilterClause::new(
            "h1",
            "hidden",
            "is",
            FilterOperand::Boolean(false),
        ));

        // Overview: 2 clauses + opted in → combinator visible.
        assert!(!spec.is_drafting());
        assert!(spec.combinator_visible());
        assert!(spec.show_add_row());

        // Adding a new clause → drafting, not editing; combinator stays.
        let format = spec.field("format").unwrap().clone();
        spec.draft = Some(FilterDraft::adding(&format));
        assert!(spec.is_drafting() && !spec.is_editing());
        assert!(spec.combinator_visible());
        assert!(!spec.show_add_row());
        // Fresh multi-enum draft has no selected options → not valid yet.
        assert!(!spec.is_draft_valid());

        // Editing an existing clause → combinator hidden.
        let clause = spec.value.clauses[1].clone();
        spec.draft = Some(FilterDraft::editing(&clause));
        assert!(spec.is_editing());
        assert!(!spec.combinator_visible());
        assert_eq!(spec.draft_field().map(|f| f.key.as_str()), Some("hidden"));
        // Boolean operand is always valid.
        assert!(spec.is_draft_valid());
    }

    #[test]
    fn opener_label_reflects_live_combinator() {
        let mut spec = sample_spec();
        // No combinator in effect → "Filter".
        assert_eq!(spec.opener_label(), "Filter");
        spec.show_combinator = true;
        spec.value.clauses.push(FilterClause::new(
            "a",
            "hidden",
            "is",
            FilterOperand::Boolean(true),
        ));
        spec.value.clauses.push(FilterClause::new(
            "b",
            "tag-count",
            "gte",
            FilterOperand::Number(1.0),
        ));
        // Opted in + 2 clauses → mode label.
        assert_eq!(spec.opener_label(), "All");
        spec.value.combinator = FilterCombinator::Or;
        assert_eq!(spec.opener_label(), "Any");
        // Toggle off → back to "Filter" even with 2 clauses.
        spec.show_combinator = false;
        assert_eq!(spec.opener_label(), "Filter");
    }

    #[test]
    fn available_fields_respects_allow_multiple() {
        let mut spec = sample_spec();
        // hidden has a clause and is single → drops out; format/tag-count remain.
        spec.value.clauses.push(FilterClause::new(
            "hidden-1",
            "hidden",
            "is",
            FilterOperand::Boolean(false),
        ));
        let keys: Vec<_> = spec
            .available_fields()
            .iter()
            .map(|f| f.key.clone())
            .collect();
        assert!(keys.contains(&"format".to_string()));
        assert!(keys.contains(&"tag-count".to_string()));
        assert!(!keys.contains(&"hidden".to_string()));
    }
}
