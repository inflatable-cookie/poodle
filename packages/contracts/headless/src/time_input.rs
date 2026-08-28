//! TimeInput entry machinery. Mirror of core `time-input.ts`.
//!
//! Contract: `docs/contracts/components/time-input.md`, "Behavior Machine".
//! Pure parse/format, bounds, step alignment, and draft-versus-commit
//! transitions. Adapters own focus, drawing, native events, and callback
//! execution.

const SECONDS_PER_DAY: i64 = 24 * 60 * 60;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeParts {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeSegment {
    Hour,
    Minute,
    Second,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeInputDraft {
    pub hour: String,
    pub minute: String,
    pub second: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeInputContext {
    pub committed: Option<String>,
    pub default_value: Option<String>,
    pub draft: Option<TimeInputDraft>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: f64,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeInputEvent {
    Digit {
        segment: TimeSegment,
        digit: u32,
    },
    ClearSegment {
        segment: TimeSegment,
    },
    ClearAll,
    Step {
        direction: i32,
    },
    Blur,
    Escape,
    Replace {
        value: Option<String>,
    },
    CommitText {
        text: String,
    },
    SetDisabled {
        disabled: bool,
    },
    SetConstraints {
        min: Option<String>,
        max: Option<String>,
        step: f64,
        default_value: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeInputEffect {
    EmitValueChange { value: Option<String> },
}

pub type TimeInputResult = (TimeInputContext, Vec<TimeInputEffect>);

fn pad2(value: u32) -> String {
    format!("{value:02}")
}

fn empty_draft() -> TimeInputDraft {
    TimeInputDraft {
        hour: String::new(),
        minute: String::new(),
        second: String::new(),
    }
}

pub fn is_positive_whole_step(step: f64) -> bool {
    step.is_finite() && step > 0.0 && step == step.trunc()
}

pub fn time_has_seconds(value: Option<&str>) -> bool {
    value.is_some_and(|raw| raw.bytes().filter(|byte| *byte == b':').count() >= 2)
}

fn parse_two_digits(raw: &str) -> Option<u32> {
    let bytes = raw.as_bytes();
    if bytes.len() != 2 || !bytes[0].is_ascii_digit() || !bytes[1].is_ascii_digit() {
        return None;
    }

    Some(u32::from(bytes[0] - b'0') * 10 + u32::from(bytes[1] - b'0'))
}

pub fn parse_time(value: Option<&str>) -> Option<TimeParts> {
    let value = value?;
    let mut parts = value.split(':');
    let hour = parse_two_digits(parts.next()?)?;
    let minute = parse_two_digits(parts.next()?)?;
    let second = match parts.next() {
        None => 0,
        Some(raw) => parse_two_digits(raw)?,
    };
    if parts.next().is_some() || hour > 23 || minute > 59 || second > 59 {
        return None;
    }

    Some(TimeParts {
        hour,
        minute,
        second,
    })
}

pub fn format_time(parts: TimeParts, with_seconds: bool) -> String {
    let clock = format!("{}:{}", pad2(parts.hour), pad2(parts.minute));
    if with_seconds {
        format!("{}:{}", clock, pad2(parts.second))
    } else {
        clock
    }
}

pub fn time_to_seconds(parts: TimeParts) -> i64 {
    i64::from(parts.hour) * 3600 + i64::from(parts.minute) * 60 + i64::from(parts.second)
}

pub fn wrap_seconds(total: i64) -> i64 {
    ((total % SECONDS_PER_DAY) + SECONDS_PER_DAY) % SECONDS_PER_DAY
}

pub fn seconds_to_time(total: i64) -> TimeParts {
    let wrapped = wrap_seconds(total);
    TimeParts {
        hour: (wrapped / 3600) as u32,
        minute: ((wrapped % 3600) / 60) as u32,
        second: (wrapped % 60) as u32,
    }
}

pub fn time_seconds_visible(
    committed: Option<&str>,
    default_value: Option<&str>,
    min: Option<&str>,
    max: Option<&str>,
    step: f64,
) -> bool {
    step < 60.0
        || time_has_seconds(committed)
        || time_has_seconds(default_value)
        || time_has_seconds(min)
        || time_has_seconds(max)
}

fn bound_seconds(value: Option<&str>) -> Option<i64> {
    parse_time(value).map(time_to_seconds)
}

pub fn time_in_bounds(parts: TimeParts, min: Option<&str>, max: Option<&str>) -> bool {
    let seconds = time_to_seconds(parts);
    let min_seconds = bound_seconds(min);
    let max_seconds = bound_seconds(max);

    match (min_seconds, max_seconds) {
        (None, None) => true,
        (Some(min_seconds), Some(max_seconds)) if min_seconds > max_seconds => {
            seconds >= min_seconds || seconds <= max_seconds
        }
        (min_seconds, max_seconds) => {
            min_seconds.is_none_or(|min_seconds| seconds >= min_seconds)
                && max_seconds.is_none_or(|max_seconds| seconds <= max_seconds)
        }
    }
}

pub fn time_step_aligned(parts: TimeParts, min: Option<&str>, step: f64) -> bool {
    if !is_positive_whole_step(step) {
        return false;
    }

    let origin = bound_seconds(min).unwrap_or(0);
    let delta = time_to_seconds(parts) - origin;
    let step = step as i64;
    ((delta % step) + step) % step == 0
}

pub fn time_constraint_valid(
    value: Option<&str>,
    min: Option<&str>,
    max: Option<&str>,
    step: f64,
) -> bool {
    let Some(value) = value else {
        return true;
    };
    let Some(parts) = parse_time(Some(value)) else {
        return false;
    };

    time_in_bounds(parts, min, max) && time_step_aligned(parts, min, step)
}

fn format_from_seconds(seconds: i64, with_seconds: bool) -> String {
    format_time(seconds_to_time(seconds), with_seconds)
}

pub fn step_time_seconds(
    current: Option<i64>,
    direction: i32,
    min: Option<&str>,
    max: Option<&str>,
    step: f64,
) -> Option<i64> {
    if !is_positive_whole_step(step) {
        return current;
    }

    let step = step as i64;
    let direction = i64::from(direction);
    let min_seconds = bound_seconds(min);
    let max_seconds = bound_seconds(max);
    let overnight =
        matches!((min_seconds, max_seconds), (Some(min_s), Some(max_s)) if min_s > max_s);
    let origin = min_seconds.unwrap_or(0);
    let from = match current {
        Some(seconds) => seconds,
        None if direction > 0 => origin - step,
        None => max_seconds
            .or(min_seconds)
            .map(|bound| bound + step)
            .unwrap_or(0),
    };
    let candidate = from + direction * step;

    if min_seconds.is_none() && max_seconds.is_none() {
        return Some(wrap_seconds(candidate));
    }

    if overnight {
        let wrapped = wrap_seconds(candidate);
        if let (Some(min_seconds), Some(max_seconds)) = (min_seconds, max_seconds) {
            if wrapped >= min_seconds || wrapped <= max_seconds {
                return Some(wrapped);
            }

            return Some(if direction > 0 {
                max_seconds
            } else {
                min_seconds
            });
        }
    }

    let low = min_seconds.unwrap_or(0);
    let high = max_seconds.unwrap_or(SECONDS_PER_DAY - 1);
    Some(candidate.clamp(low, high))
}

fn show_seconds(context: &TimeInputContext) -> bool {
    time_seconds_visible(
        context.committed.as_deref(),
        context.default_value.as_deref(),
        context.min.as_deref(),
        context.max.as_deref(),
        context.step,
    )
}

fn draft_from_committed(committed: Option<&str>) -> TimeInputDraft {
    match parse_time(committed) {
        None => empty_draft(),
        Some(parts) => TimeInputDraft {
            hour: pad2(parts.hour),
            minute: pad2(parts.minute),
            second: pad2(parts.second),
        },
    }
}

fn text_to_draft(text: &str) -> TimeInputDraft {
    let mut parts = text.split(':');
    TimeInputDraft {
        hour: parts.next().unwrap_or("").to_string(),
        minute: parts.next().unwrap_or("").to_string(),
        second: parts.next().unwrap_or("").to_string(),
    }
}

fn visible_empty(draft: &TimeInputDraft, seconds_visible: bool) -> bool {
    draft.hour.is_empty()
        && draft.minute.is_empty()
        && (!seconds_visible || draft.second.is_empty())
}

fn draft_candidate(draft: &TimeInputDraft, seconds_visible: bool) -> Option<String> {
    if draft.hour.len() != 2 || draft.minute.len() != 2 {
        return None;
    }

    if seconds_visible {
        if draft.second.len() != 2 {
            return None;
        }

        return Some(format!("{}:{}:{}", draft.hour, draft.minute, draft.second));
    }

    Some(format!("{}:{}", draft.hour, draft.minute))
}

fn commit_value(mut context: TimeInputContext, value: Option<String>) -> TimeInputResult {
    if context.committed == value && context.draft.is_none() {
        return (context, Vec::new());
    }

    context.committed = value.clone();
    context.draft = None;
    (context, vec![TimeInputEffect::EmitValueChange { value }])
}

fn try_commit_draft(context: TimeInputContext, draft: TimeInputDraft) -> TimeInputResult {
    let seconds_visible = show_seconds(&context);

    if visible_empty(&draft, seconds_visible) {
        return commit_value(context, None);
    }

    if let Some(candidate) = draft_candidate(&draft, seconds_visible) {
        if let Some(parts) = parse_time(Some(&candidate)) {
            if time_in_bounds(parts, context.min.as_deref(), context.max.as_deref())
                && time_step_aligned(parts, context.min.as_deref(), context.step)
            {
                return commit_value(context, Some(format_time(parts, seconds_visible)));
            }
        }
    }

    let mut next = context;
    next.draft = Some(draft);
    (next, Vec::new())
}

fn with_draft(context: &TimeInputContext) -> TimeInputDraft {
    context
        .draft
        .clone()
        .unwrap_or_else(|| draft_from_committed(context.committed.as_deref()))
}

fn apply_digit(mut draft: TimeInputDraft, segment: TimeSegment, digit: u32) -> TimeInputDraft {
    let current = match segment {
        TimeSegment::Hour => &draft.hour,
        TimeSegment::Minute => &draft.minute,
        TimeSegment::Second => &draft.second,
    };
    let next = if current.is_empty() || current.len() >= 2 {
        digit.to_string()
    } else {
        format!("{current}{digit}")
    };

    match segment {
        TimeSegment::Hour => draft.hour = next,
        TimeSegment::Minute => draft.minute = next,
        TimeSegment::Second => draft.second = next,
    }

    draft
}

fn idle(context: TimeInputContext) -> TimeInputResult {
    (context, Vec::new())
}

pub fn time_input_invalid(context: &TimeInputContext) -> bool {
    context.draft.is_some()
}

impl Default for TimeInputContext {
    fn default() -> Self {
        Self {
            committed: None,
            default_value: None,
            draft: None,
            min: None,
            max: None,
            step: 60.0,
            disabled: false,
        }
    }
}

pub fn time_input_transition(context: TimeInputContext, event: TimeInputEvent) -> TimeInputResult {
    match &event {
        TimeInputEvent::SetDisabled { disabled } => {
            let mut next = context;
            next.disabled = *disabled;
            return (next, Vec::new());
        }
        TimeInputEvent::SetConstraints {
            min,
            max,
            step,
            default_value,
        } => {
            let mut next = context;
            next.min = min.clone();
            next.max = max.clone();
            next.step = *step;
            next.default_value = default_value.clone();
            return (next, Vec::new());
        }
        TimeInputEvent::Replace { value } => {
            let mut next = context;
            next.committed = value.clone();
            next.draft = None;
            return (next, Vec::new());
        }
        _ => {}
    }

    if context.disabled {
        return idle(context);
    }

    match event {
        TimeInputEvent::Digit { segment, digit } => {
            if digit > 9 {
                return idle(context);
            }

            let draft = apply_digit(with_draft(&context), segment, digit);
            try_commit_draft(context, draft)
        }
        TimeInputEvent::ClearSegment { segment } => {
            let mut draft = with_draft(&context);
            match segment {
                TimeSegment::Hour => draft.hour.clear(),
                TimeSegment::Minute => draft.minute.clear(),
                TimeSegment::Second => draft.second.clear(),
            }
            try_commit_draft(context, draft)
        }
        TimeInputEvent::ClearAll => commit_value(context, None),
        TimeInputEvent::CommitText { text } => {
            if text.is_empty() {
                return commit_value(context, None);
            }

            let seconds_visible = show_seconds(&context);
            if let Some(parts) = parse_time(Some(&text)) {
                let formatted = format_time(parts, seconds_visible);
                if time_constraint_valid(
                    Some(&formatted),
                    context.min.as_deref(),
                    context.max.as_deref(),
                    context.step,
                ) {
                    return commit_value(context, Some(formatted));
                }
            }

            let mut next = context;
            next.draft = Some(text_to_draft(&text));
            (next, Vec::new())
        }
        TimeInputEvent::Step { direction } => {
            let direction = if direction < 0 { -1 } else { 1 };
            if !is_positive_whole_step(context.step) {
                let mut next = context;
                next.draft = None;
                return (next, Vec::new());
            }

            let current = parse_time(context.committed.as_deref()).map(time_to_seconds);
            let Some(next_seconds) = step_time_seconds(
                current,
                direction,
                context.min.as_deref(),
                context.max.as_deref(),
                context.step,
            ) else {
                let mut next = context;
                next.draft = None;
                return (next, Vec::new());
            };

            let formatted = format_from_seconds(next_seconds, show_seconds(&context));
            commit_value(context, Some(formatted))
        }
        TimeInputEvent::Blur | TimeInputEvent::Escape => {
            if context.draft.is_none() {
                return idle(context);
            }

            let mut next = context;
            next.draft = None;
            (next, Vec::new())
        }
        TimeInputEvent::SetDisabled { .. }
        | TimeInputEvent::SetConstraints { .. }
        | TimeInputEvent::Replace { .. } => idle(context),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rejects_unpadded_and_impossible_values() {
        assert_eq!(
            parse_time(Some("14:30")),
            Some(TimeParts {
                hour: 14,
                minute: 30,
                second: 0
            })
        );
        assert!(parse_time(Some("9:30")).is_none());
        assert!(parse_time(Some("24:00")).is_none());
        assert!(parse_time(Some("14:30:00.5")).is_none());
    }

    #[test]
    fn overnight_step_crosses_midnight_and_stops_at_the_gap() {
        assert_eq!(
            step_time_seconds(
                Some(23 * 3600 + 30 * 60),
                1,
                Some("22:00"),
                Some("06:00"),
                1800.0
            ),
            Some(0)
        );
        assert_eq!(
            step_time_seconds(Some(6 * 3600), 1, Some("22:00"), Some("06:00"), 1800.0),
            Some(6 * 3600)
        );
        assert_eq!(
            step_time_seconds(Some(22 * 3600), -1, Some("22:00"), Some("06:00"), 1800.0),
            Some(22 * 3600)
        );
    }

    #[test]
    fn complete_digits_commit_and_partial_digits_stay_local() {
        let context = TimeInputContext {
            committed: Some("14:30".into()),
            ..TimeInputContext::default()
        };
        let (next, effects) = time_input_transition(
            context.clone(),
            TimeInputEvent::Digit {
                segment: TimeSegment::Hour,
                digit: 1,
            },
        );
        assert!(effects.is_empty());
        assert!(time_input_invalid(&next));

        let (committed, effects) = time_input_transition(
            next,
            TimeInputEvent::Digit {
                segment: TimeSegment::Hour,
                digit: 5,
            },
        );
        assert_eq!(
            effects,
            vec![TimeInputEffect::EmitValueChange {
                value: Some("15:30".into())
            }]
        );
        assert_eq!(committed.committed.as_deref(), Some("15:30"));
        assert!(committed.draft.is_none());
    }
}
