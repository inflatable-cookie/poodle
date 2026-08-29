//! NumberInput entry machinery. Mirror of core `number-input.ts`.
//!
//! Contract: `docs/contracts/components/number-input.md`, "Behavior Machine".
//! Pure decimal draft classification, configuration checks, committed
//! value/raw-draft transitions, and step/commit effects. Adapters own
//! focus, caret, DOM/native events, async validation, and callback
//! execution.

#[derive(Clone, Debug, PartialEq)]
pub struct NumberInputContext {
    pub committed: Option<f64>,
    pub default_value: Option<f64>,
    pub draft: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    /// Authored step; `None` means omitted (effective step is `1`).
    pub step: Option<f64>,
    /// Authored precision; `None` means omitted.
    pub precision: Option<f64>,
    pub disabled: bool,
    pub read_only: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberInputEvent {
    RawEdit { text: String },
    Clear,
    Enter,
    Blur,
    Escape,
    Step { direction: i32 },
    Home,
    End,
    Replace { value: Option<f64> },
    SetDisabled { disabled: bool },
    SetReadOnly { read_only: bool },
    SetConstraints {
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
        precision: Option<f64>,
        default_value: Option<f64>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberInputEffect {
    EmitDraftValueChange { draft: Option<String> },
    EmitValueChange { value: Option<f64> },
    EmitCommit { value: Option<f64> },
}

pub type NumberInputResult = (NumberInputContext, Vec<NumberInputEffect>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumberDraftKind {
    Empty,
    Incomplete,
    Malformed,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumberDecimal {
    pub negative: bool,
    /// Absolute integer coefficient; value = ± digits / 10^scale.
    pub digits: i128,
    pub scale: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumberDraftClassification {
    pub kind: NumberDraftKind,
    pub decimal: Option<NumberDecimal>,
    pub fractional_digits: Option<u32>,
}

fn idle(context: NumberInputContext) -> NumberInputResult {
    (context, Vec::new())
}

fn is_finite_number(value: f64) -> bool {
    value.is_finite()
}

fn is_non_negative_integer(value: f64) -> bool {
    value.is_finite() && value >= 0.0 && value == value.trunc()
}

/// Effective step used for alignment and stepping when config is valid.
pub fn number_input_effective_step(step: Option<f64>) -> f64 {
    step.unwrap_or(1.0)
}

pub fn number_input_config_valid(context: &NumberInputContext) -> bool {
    let check_optional = |value: Option<f64>| -> bool {
        value.is_none_or(is_finite_number)
    };

    if !check_optional(context.committed) {
        return false;
    }
    if !check_optional(context.default_value) {
        return false;
    }
    if !check_optional(context.min) {
        return false;
    }
    if !check_optional(context.max) {
        return false;
    }

    if let Some(step) = context.step {
        if !is_finite_number(step) || step <= 0.0 {
            return false;
        }
    }

    if let Some(precision) = context.precision {
        if !is_non_negative_integer(precision) {
            return false;
        }
    }

    if let (Some(min), Some(max)) = (context.min, context.max) {
        if min > max {
            return false;
        }
    }

    true
}

fn strip_sign(text: &str) -> (bool, &str) {
    if let Some(body) = text.strip_prefix('-') {
        (true, body)
    } else {
        (false, text)
    }
}

/// Portable draft syntax: optional leading `-`, digits, at most one `.`.
fn draft_syntax_ok(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    let mut i = 0usize;
    if bytes[0] == b'-' {
        i = 1;
        if i >= bytes.len() {
            return false;
        }
    }

    if bytes[i] == b'.' {
        i += 1;
        if i >= bytes.len() || !bytes[i].is_ascii_digit() {
            return false;
        }
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        return i == bytes.len();
    }

    if !bytes[i].is_ascii_digit() {
        return false;
    }
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i < bytes.len() {
        if bytes[i] != b'.' {
            return false;
        }
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }

    i == bytes.len()
}

fn is_incomplete_literal(text: &str) -> bool {
    matches!(text, "-" | "." | "-.")
}

fn strip_leading_zeros(digits: &str) -> String {
    let trimmed = digits.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn parse_number_decimal(text: &str) -> Option<NumberDecimal> {
    if !draft_syntax_ok(text) || is_incomplete_literal(text) || text.ends_with('.') {
        return None;
    }

    let (negative, body) = strip_sign(text);
    let mut parts = body.splitn(2, '.');
    let int_raw = parts.next().unwrap_or("");
    let frac_raw = parts.next().unwrap_or("");
    let int_part = if int_raw.is_empty() { "0" } else { int_raw };
    let digits_text = strip_leading_zeros(&format!("{int_part}{frac_raw}"));
    let digits: i128 = digits_text.parse().ok()?;

    Some(NumberDecimal {
        negative: negative && digits_text != "0",
        digits,
        scale: frac_raw.len() as u32,
    })
}

pub fn classify_number_draft(text: &str) -> NumberDraftClassification {
    if text.is_empty() {
        return NumberDraftClassification {
            kind: NumberDraftKind::Empty,
            decimal: None,
            fractional_digits: None,
        };
    }

    if is_incomplete_literal(text) || (draft_syntax_ok(text) && text.ends_with('.')) {
        return NumberDraftClassification {
            kind: NumberDraftKind::Incomplete,
            decimal: None,
            fractional_digits: None,
        };
    }

    if !draft_syntax_ok(text) {
        return NumberDraftClassification {
            kind: NumberDraftKind::Malformed,
            decimal: None,
            fractional_digits: None,
        };
    }

    let Some(decimal) = parse_number_decimal(text) else {
        return NumberDraftClassification {
            kind: NumberDraftKind::Malformed,
            decimal: None,
            fractional_digits: None,
        };
    };

    let (_, body) = strip_sign(text);
    let frac = body
        .split_once('.')
        .map(|(_, frac)| frac.len() as u32)
        .unwrap_or(0);

    NumberDraftClassification {
        kind: NumberDraftKind::Complete,
        decimal: Some(decimal),
        fractional_digits: Some(frac),
    }
}

fn pow10(exp: u32) -> Option<i128> {
    let mut result: i128 = 1;
    for _ in 0..exp {
        result = result.checked_mul(10)?;
    }
    Some(result)
}

fn rescale_digits(decimal: NumberDecimal, target_scale: u32) -> Option<i128> {
    let delta = target_scale as i32 - decimal.scale as i32;
    let magnitude = if delta >= 0 {
        decimal.digits.checked_mul(pow10(delta as u32)?)?
    } else {
        decimal.digits / pow10((-delta) as u32)?
    };
    Some(if decimal.negative {
        -magnitude
    } else {
        magnitude
    })
}

fn trim_trailing_zeros_and_dot(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut end = bytes.len();
    while end > 0 && bytes[end - 1] == b'0' {
        end -= 1;
    }
    if end > 0 && bytes[end - 1] == b'.' {
        end -= 1;
    }
    text[..end].to_string()
}

fn decimal_from_number(value: f64) -> Option<NumberDecimal> {
    if !is_finite_number(value) {
        return None;
    }

    parse_number_decimal(&format_shortest_decimal(value))
}

fn common_scale(scales: &[u32]) -> u32 {
    scales.iter().copied().max().unwrap_or(0)
}

pub fn format_number_decimal(decimal: NumberDecimal, precision: Option<f64>) -> String {
    if let Some(precision) = precision {
        let precision = precision as u32;
        let Some(scaled) = rescale_digits(decimal, precision) else {
            return String::new();
        };
        let negative = scaled < 0;
        let abs = if negative { -scaled } else { scaled };
        let raw = format!("{abs}");
        let pad = precision as usize + 1;
        let raw = if raw.len() < pad {
            format!("{:0>width$}", raw, width = pad)
        } else {
            raw
        };
        let (int_part, frac_part) = if precision == 0 {
            (raw.as_str(), "")
        } else {
            let split = raw.len() - precision as usize;
            let int_part = if raw[..split].is_empty() {
                "0"
            } else {
                &raw[..split]
            };
            (int_part, &raw[split..])
        };
        let body = if precision == 0 {
            int_part.to_string()
        } else {
            format!("{int_part}.{frac_part}")
        };
        if negative && abs != 0 {
            format!("-{body}")
        } else {
            body
        }
    } else {
        if decimal.digits == 0 {
            return "0".to_string();
        }

        let raw = format!("{}", decimal.digits);
        let pad = decimal.scale as usize + 1;
        let raw = if raw.len() < pad {
            format!("{:0>width$}", raw, width = pad)
        } else {
            raw
        };

        let (mut int_part, mut frac_part) = if decimal.scale == 0 {
            (raw.clone(), String::new())
        } else {
            let split = raw.len() - decimal.scale as usize;
            let int_part = if raw[..split].is_empty() {
                "0".to_string()
            } else {
                raw[..split].to_string()
            };
            (int_part, raw[split..].to_string())
        };

        while frac_part.ends_with('0') {
            frac_part.pop();
        }
        int_part = strip_leading_zeros(&int_part);
        let body = if frac_part.is_empty() {
            int_part
        } else {
            format!("{int_part}.{frac_part}")
        };
        if decimal.negative {
            format!("-{body}")
        } else {
            body
        }
    }
}

/// Shortest canonical decimal for a finite number.
pub fn format_shortest_decimal(value: f64) -> String {
    if !is_finite_number(value) {
        return String::new();
    }

    if value == 0.0 {
        return "0".to_string();
    }

    let abs = value.abs();
    let mut text = abs.to_string();

    if text.contains('e') || text.contains('E') || (abs != 0.0 && (abs < 1e-6 || abs >= 1e21)) {
        text = trim_trailing_zeros_and_dot(&format!("{abs:.16}"));
    }

    if value < 0.0 {
        format!("-{text}")
    } else {
        text
    }
}

pub fn format_number_committed(value: Option<f64>, precision: Option<f64>) -> String {
    let Some(value) = value else {
        return String::new();
    };

    let Some(decimal) = decimal_from_number(value) else {
        return String::new();
    };

    format_number_decimal(decimal, precision)
}

pub fn number_in_bounds(value: f64, min: Option<f64>, max: Option<f64>) -> bool {
    if let Some(min) = min {
        if value < min {
            return false;
        }
    }
    if let Some(max) = max {
        if value > max {
            return false;
        }
    }
    true
}

pub fn number_step_aligned(value: f64, min: Option<f64>, step: Option<f64>) -> bool {
    let effective_step = number_input_effective_step(step);

    if !is_finite_number(effective_step) || effective_step <= 0.0 {
        return false;
    }

    let Some(value_decimal) = decimal_from_number(value) else {
        return false;
    };
    let Some(origin_decimal) = decimal_from_number(min.unwrap_or(0.0)) else {
        return false;
    };
    let Some(step_decimal) = decimal_from_number(effective_step) else {
        return false;
    };

    let scale = common_scale(&[
        value_decimal.scale,
        origin_decimal.scale,
        step_decimal.scale,
    ]);
    let Some(value_digits) = rescale_digits(value_decimal, scale) else {
        return false;
    };
    let Some(origin_digits) = rescale_digits(origin_decimal, scale) else {
        return false;
    };
    let Some(step_digits) = rescale_digits(step_decimal, scale) else {
        return false;
    };

    if step_digits == 0 {
        return false;
    }

    let delta = value_digits - origin_digits;
    delta % step_digits == 0
}

pub fn number_precision_ok(fractional_digits: u32, precision: Option<f64>) -> bool {
    match precision {
        None => true,
        Some(precision) => fractional_digits as f64 <= precision,
    }
}

pub fn number_draft_constraint_valid(
    text: &str,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    precision: Option<f64>,
) -> bool {
    let classified = classify_number_draft(text);

    let (Some(decimal), Some(fractional_digits)) =
        (classified.decimal, classified.fractional_digits)
    else {
        return false;
    };

    if classified.kind != NumberDraftKind::Complete {
        return false;
    }

    if !number_precision_ok(fractional_digits, precision) {
        return false;
    }

    let value = number_decimal_to_number(decimal);

    is_finite_number(value)
        && number_in_bounds(value, min, max)
        && number_step_aligned(value, min, step)
}

pub fn number_decimal_to_number(decimal: NumberDecimal) -> f64 {
    format_number_decimal(decimal, None)
        .parse::<f64>()
        .unwrap_or(f64::NAN)
}

pub fn number_value_constraint_valid(
    value: Option<f64>,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    precision: Option<f64>,
) -> bool {
    let Some(value) = value else {
        return true;
    };

    if !is_finite_number(value) {
        return false;
    }

    let text = format_number_committed(Some(value), precision);
    let classified = classify_number_draft(&text);

    let Some(fractional_digits) = classified.fractional_digits else {
        return false;
    };

    if classified.kind != NumberDraftKind::Complete {
        return false;
    }

    number_precision_ok(fractional_digits, precision)
        && number_in_bounds(value, min, max)
        && number_step_aligned(value, min, step)
}

fn last_on_grid(min: Option<f64>, max: f64, step: Option<f64>) -> Option<f64> {
    let origin = min.unwrap_or(0.0);
    let effective_step = number_input_effective_step(step);
    let origin_decimal = decimal_from_number(origin)?;
    let max_decimal = decimal_from_number(max)?;
    let step_decimal = decimal_from_number(effective_step)?;

    let scale = common_scale(&[
        origin_decimal.scale,
        max_decimal.scale,
        step_decimal.scale,
    ]);
    let origin_digits = rescale_digits(origin_decimal, scale)?;
    let max_digits = rescale_digits(max_decimal, scale)?;
    let step_digits = rescale_digits(step_decimal, scale)?;

    if step_digits <= 0 {
        return None;
    }

    let delta = max_digits - origin_digits;
    let remainder = ((delta % step_digits) + step_digits) % step_digits;
    let last_digits = max_digits - remainder;
    let negative = last_digits < 0;
    let abs = if negative { -last_digits } else { last_digits };
    let decimal = NumberDecimal {
        negative,
        digits: abs,
        scale,
    };
    let value = number_decimal_to_number(decimal);

    if number_value_constraint_valid(Some(value), min, Some(max), step, None) {
        Some(value)
    } else {
        None
    }
}

pub fn step_number_value(
    current: Option<f64>,
    direction: i32,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    precision: Option<f64>,
) -> Option<f64> {
    let effective_step = number_input_effective_step(step);

    let config = NumberInputContext {
        committed: None,
        default_value: None,
        draft: None,
        min,
        max,
        step,
        precision,
        disabled: false,
        read_only: false,
    };
    if !number_input_config_valid(&config) {
        return None;
    }

    if current.is_none() {
        if direction > 0 {
            let origin = min.unwrap_or(0.0);
            return if number_value_constraint_valid(Some(origin), min, max, step, precision) {
                Some(origin)
            } else {
                None
            };
        }

        if let Some(max) = max {
            return last_on_grid(min, max, step);
        }

        let origin = min.unwrap_or(0.0);
        let candidate = origin - effective_step;
        return if number_value_constraint_valid(Some(candidate), min, max, step, precision) {
            Some(candidate)
        } else {
            None
        };
    }

    let current = current?;
    let current_decimal = decimal_from_number(current)?;
    let step_decimal = decimal_from_number(effective_step)?;

    let precision_scale = precision.map(|p| p as u32).unwrap_or(0);
    let scale = common_scale(&[current_decimal.scale, step_decimal.scale, precision_scale]);
    let current_digits = rescale_digits(current_decimal, scale)?;
    let step_digits = rescale_digits(step_decimal, scale)?;
    let next_digits = current_digits + i128::from(direction) * step_digits;
    let negative = next_digits < 0;
    let abs = if negative { -next_digits } else { next_digits };
    let next = number_decimal_to_number(NumberDecimal {
        negative,
        digits: abs,
        scale,
    });

    if !number_value_constraint_valid(Some(next), min, max, step, precision) {
        return None;
    }

    Some(next)
}

fn active_draft_text(context: &NumberInputContext) -> Option<&str> {
    context.draft.as_deref()
}

fn display_text(context: &NumberInputContext) -> String {
    if let Some(draft) = &context.draft {
        return draft.clone();
    }

    format_number_committed(context.committed, context.precision)
}

fn valid_draft_value(context: &NumberInputContext, text: &str) -> Option<f64> {
    if !number_draft_constraint_valid(
        text,
        context.min,
        context.max,
        context.step,
        context.precision,
    ) {
        return None;
    }

    parse_number_decimal(text).map(number_decimal_to_number)
}

fn push_unique(effects: &mut Vec<NumberInputEffect>, effect: NumberInputEffect) {
    if effects.last() == Some(&effect) {
        return;
    }
    effects.push(effect);
}

fn resolve_committed(
    context: NumberInputContext,
    value: Option<f64>,
    commit: bool,
    draft: Option<String>,
) -> NumberInputResult {
    let mut effects = Vec::new();
    let draft_changed = draft != context.draft;
    let value_changed = match (value, context.committed) {
        (Some(a), Some(b)) => a != b,
        (None, None) => false,
        _ => true,
    };

    let mut next = NumberInputContext {
        committed: value,
        draft: draft.clone(),
        ..context.clone()
    };

    if draft_changed {
        push_unique(
            &mut effects,
            NumberInputEffect::EmitDraftValueChange {
                draft: draft.clone(),
            },
        );
    }

    if value_changed {
        push_unique(
            &mut effects,
            NumberInputEffect::EmitValueChange { value },
        );
    }

    if commit {
        push_unique(&mut effects, NumberInputEffect::EmitCommit { value });
    }

    // After a successful resolve, normalize draft display through committed formatting.
    if draft.is_none() && value.is_some() {
        next.draft = None;
    }

    (next, effects)
}

fn set_draft(
    mut context: NumberInputContext,
    draft: String,
    emit_value: Option<Option<f64>>,
) -> NumberInputResult {
    let mut effects = Vec::new();
    let draft_changed = context.draft.as_deref() != Some(draft.as_str());

    if draft_changed {
        push_unique(
            &mut effects,
            NumberInputEffect::EmitDraftValueChange {
                draft: Some(draft.clone()),
            },
        );
    }

    context.draft = Some(draft);

    if let Some(emit_value) = emit_value {
        let value_changed = match (emit_value, context.committed) {
            (Some(a), Some(b)) => a != b,
            (None, None) => false,
            _ => true,
        };
        if value_changed {
            context.committed = emit_value;
            push_unique(
                &mut effects,
                NumberInputEffect::EmitValueChange { value: emit_value },
            );
        }
    }

    (context, effects)
}

fn discard_draft(mut context: NumberInputContext) -> NumberInputResult {
    if context.draft.is_none() {
        return idle(context);
    }

    context.draft = None;
    (
        context,
        vec![NumberInputEffect::EmitDraftValueChange { draft: None }],
    )
}

fn commit_current(context: NumberInputContext) -> NumberInputResult {
    let draft = active_draft_text(&context).map(str::to_owned);

    let Some(draft) = draft else {
        return (
            context.clone(),
            vec![NumberInputEffect::EmitCommit {
                value: context.committed,
            }],
        );
    };

    if draft.is_empty() {
        return resolve_committed(context, None, true, None);
    }

    let Some(value) = valid_draft_value(&context, &draft) else {
        return idle(context);
    };

    resolve_committed(context, Some(value), true, None)
}

pub fn number_input_invalid(context: &NumberInputContext) -> bool {
    if !number_input_config_valid(context) {
        return true;
    }

    let Some(draft) = active_draft_text(context) else {
        return false;
    };

    if draft.is_empty() {
        return false;
    }

    valid_draft_value(context, draft).is_none()
}

impl Default for NumberInputContext {
    fn default() -> Self {
        Self {
            committed: None,
            default_value: None,
            draft: None,
            min: None,
            max: None,
            step: None,
            precision: None,
            disabled: false,
            read_only: false,
        }
    }
}

pub fn number_input_context() -> NumberInputContext {
    NumberInputContext::default()
}

pub fn number_input_transition(
    context: NumberInputContext,
    event: NumberInputEvent,
) -> NumberInputResult {
    match &event {
        NumberInputEvent::SetDisabled { disabled } => {
            let mut next = context;
            next.disabled = *disabled;
            return (next, Vec::new());
        }
        NumberInputEvent::SetReadOnly { read_only } => {
            let mut next = context;
            next.read_only = *read_only;
            return (next, Vec::new());
        }
        NumberInputEvent::SetConstraints {
            min,
            max,
            step,
            precision,
            default_value,
        } => {
            let mut next = context;
            next.min = *min;
            next.max = *max;
            next.step = *step;
            next.precision = *precision;
            next.default_value = *default_value;
            return (next, Vec::new());
        }
        NumberInputEvent::Replace { value } => {
            let had_draft = context.draft.is_some();
            let mut next = context;
            next.committed = *value;
            next.draft = None;
            let effects = if had_draft {
                vec![NumberInputEffect::EmitDraftValueChange { draft: None }]
            } else {
                Vec::new()
            };
            return (next, effects);
        }
        _ => {}
    }

    if context.disabled || context.read_only {
        return idle(context);
    }

    if !number_input_config_valid(&context) {
        return idle(context);
    }

    match event {
        NumberInputEvent::RawEdit { text } => {
            if text.is_empty() {
                return set_draft(context, String::new(), Some(None));
            }

            let classified = classify_number_draft(&text);

            if classified.kind == NumberDraftKind::Complete && classified.decimal.is_some() {
                let value = valid_draft_value(&context, &text);
                return set_draft(context, text, value.map(Some));
            }

            set_draft(context, text, None)
        }
        NumberInputEvent::Clear => set_draft(context, String::new(), Some(None)),
        NumberInputEvent::Enter => commit_current(context),
        NumberInputEvent::Blur => {
            let draft = active_draft_text(&context).map(str::to_owned);

            let Some(draft) = draft else {
                return idle(context);
            };

            if draft.is_empty() || valid_draft_value(&context, &draft).is_some() {
                return commit_current(context);
            }

            discard_draft(context)
        }
        NumberInputEvent::Escape => discard_draft(context),
        NumberInputEvent::Step { direction } => {
            let direction = if direction < 0 { -1 } else { 1 };
            let draft = active_draft_text(&context);
            let mut from = context.committed;

            if let Some(draft) = draft {
                if draft.is_empty() {
                    from = None;
                } else {
                    let Some(draft_value) = valid_draft_value(&context, draft) else {
                        return idle(context);
                    };
                    from = Some(draft_value);
                }
            }

            let Some(next) = step_number_value(
                from,
                direction,
                context.min,
                context.max,
                context.step,
                context.precision,
            ) else {
                return idle(context);
            };

            resolve_committed(context, Some(next), true, None)
        }
        NumberInputEvent::Home => {
            let Some(min) = context.min else {
                return idle(context);
            };

            if !number_value_constraint_valid(
                Some(min),
                context.min,
                context.max,
                context.step,
                context.precision,
            ) {
                return idle(context);
            }

            resolve_committed(context, Some(min), true, None)
        }
        NumberInputEvent::End => {
            let Some(max) = context.max else {
                return idle(context);
            };

            if !number_value_constraint_valid(
                Some(max),
                context.min,
                context.max,
                context.step,
                context.precision,
            ) {
                return idle(context);
            }

            resolve_committed(context, Some(max), true, None)
        }
        NumberInputEvent::SetDisabled { .. }
        | NumberInputEvent::SetReadOnly { .. }
        | NumberInputEvent::SetConstraints { .. }
        | NumberInputEvent::Replace { .. } => idle(context),
    }
}

pub fn number_input_display_text(context: &NumberInputContext) -> String {
    display_text(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_empty_incomplete_complete_and_malformed() {
        assert_eq!(classify_number_draft("").kind, NumberDraftKind::Empty);
        assert_eq!(classify_number_draft("-").kind, NumberDraftKind::Incomplete);
        assert_eq!(classify_number_draft(".").kind, NumberDraftKind::Incomplete);
        assert_eq!(classify_number_draft("-.").kind, NumberDraftKind::Incomplete);
        assert_eq!(classify_number_draft("1.").kind, NumberDraftKind::Incomplete);
        assert_eq!(classify_number_draft("01.20").kind, NumberDraftKind::Complete);
        assert_eq!(classify_number_draft(".5").kind, NumberDraftKind::Complete);
        assert_eq!(classify_number_draft("-12.5").kind, NumberDraftKind::Complete);
        assert_eq!(classify_number_draft("1e3").kind, NumberDraftKind::Malformed);
        assert_eq!(classify_number_draft(" 1").kind, NumberDraftKind::Malformed);
        assert_eq!(classify_number_draft("0x10").kind, NumberDraftKind::Malformed);
    }

    #[test]
    fn omitted_step_is_one_and_invalid_config_is_inert() {
        assert_eq!(number_input_effective_step(None), 1.0);
        assert!(number_step_aligned(3.0, None, None));

        let mut context = number_input_context();
        context.step = Some(-1.0);
        let (next, effects) =
            number_input_transition(context.clone(), NumberInputEvent::RawEdit { text: "1".into() });
        assert!(effects.is_empty());
        assert_eq!(next, context);
    }

    #[test]
    fn clear_empty_emits_draft_and_null_value() {
        let mut context = number_input_context();
        context.committed = Some(4.0);
        let (next, effects) = number_input_transition(context, NumberInputEvent::Clear);
        assert_eq!(next.draft.as_deref(), Some(""));
        assert_eq!(next.committed, None);
        assert_eq!(
            effects,
            vec![
                NumberInputEffect::EmitDraftValueChange {
                    draft: Some("".into())
                },
                NumberInputEffect::EmitValueChange { value: None },
            ]
        );
    }

    #[test]
    fn blur_discards_unresolved_draft() {
        let mut context = number_input_context();
        context.committed = Some(2.0);
        context.draft = Some("1.".into());
        let (next, effects) = number_input_transition(context, NumberInputEvent::Blur);
        assert!(next.draft.is_none());
        assert_eq!(next.committed, Some(2.0));
        assert_eq!(
            effects,
            vec![NumberInputEffect::EmitDraftValueChange { draft: None }]
        );
    }

    #[test]
    fn step_from_empty_baseline_uses_min_or_zero() {
        let context = number_input_context();
        let (next, effects) =
            number_input_transition(context, NumberInputEvent::Step { direction: 1 });
        assert_eq!(next.committed, Some(0.0));
        assert!(effects.iter().any(|e| matches!(
            e,
            NumberInputEffect::EmitCommit { value: Some(v) } if *v == 0.0
        )));
    }
}
