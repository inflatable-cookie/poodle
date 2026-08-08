//! Duration entry machinery (DurationInput).
//!
//! Contract: `docs/contracts/components/duration-input.md`, "Behavior Machine",
//! which names `@inflatable-cookie/poodle-core`' `duration.ts` as the authority. This is a
//! faithful port of `packages/core/src/duration.ts` — same carry rules, same
//! clamps, same swallowed carries at the hour bound — so the web and Rust
//! targets cannot drift on what a keystroke means.

/// Hours, minutes and seconds as entered.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DurationValue {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

/// Which segment a keystroke is aimed at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DurationSegment {
    Hours,
    Minutes,
    Seconds,
}

fn clamp(value: i64, min: i64, max: i64) -> i64 {
    value.max(min).min(max)
}

pub fn duration_total_seconds(value: DurationValue) -> u32 {
    value.hours * 3600 + value.minutes * 60 + value.seconds
}

/// Step a segment by ±`delta` with carry/borrow, hours clamped to
/// `[0, max_hours]`.
///
/// A carry that would push hours past the bound is **swallowed**, not clamped
/// into a partial change: 59 minutes stepping up at max hours stays where it
/// is rather than rolling the minutes to 0 and losing an hour's worth of entry.
/// That is the web behaviour this ports.
pub fn adjust_duration_segment(
    value: DurationValue,
    segment: DurationSegment,
    delta: i64,
    max_hours: u32,
) -> DurationValue {
    let max_hours = i64::from(max_hours);
    let (mut hours, minutes, seconds) = (
        i64::from(value.hours),
        i64::from(value.minutes),
        i64::from(value.seconds),
    );

    match segment {
        DurationSegment::Hours => DurationValue {
            hours: clamp(hours + delta, 0, max_hours) as u32,
            minutes: minutes as u32,
            seconds: seconds as u32,
        },
        DurationSegment::Minutes => {
            let mut next_minutes = minutes + delta;
            if next_minutes >= 60 {
                next_minutes = 0;
                hours = clamp(hours + 1, 0, max_hours);
            }
            if next_minutes < 0 {
                next_minutes = 59;
                hours = clamp(hours - 1, 0, max_hours);
            }
            DurationValue {
                hours: hours as u32,
                minutes: next_minutes as u32,
                seconds: seconds as u32,
            }
        }
        DurationSegment::Seconds => {
            let mut next_seconds = seconds + delta;
            let mut next_minutes = minutes;
            let mut next_hours = hours;

            if next_seconds >= 60 {
                next_seconds = 0;
                next_minutes += 1;
            }
            if next_seconds < 0 {
                next_seconds = 59;
                next_minutes -= 1;
            }
            if next_minutes >= 60 {
                next_minutes = 0;
                next_hours = clamp(next_hours + 1, 0, max_hours);
            }
            if next_minutes < 0 {
                next_minutes = 59;
                next_hours = clamp(next_hours - 1, 0, max_hours);
            }

            DurationValue {
                hours: next_hours as u32,
                minutes: clamp(next_minutes, 0, 59) as u32,
                seconds: clamp(next_seconds, 0, 59) as u32,
            }
        }
    }
}

/// Direct segment entry: clamp into the segment's valid range.
pub fn set_duration_segment(
    value: DurationValue,
    segment: DurationSegment,
    raw: i64,
    max_hours: u32,
) -> DurationValue {
    match segment {
        DurationSegment::Hours => DurationValue {
            hours: clamp(raw, 0, i64::from(max_hours)) as u32,
            ..value
        },
        DurationSegment::Minutes => DurationValue {
            minutes: clamp(raw, 0, 59) as u32,
            ..value
        },
        DurationSegment::Seconds => DurationValue {
            seconds: clamp(raw, 0, 59) as u32,
            ..value
        },
    }
}

pub fn pad_duration_segment(value: u32) -> String {
    format!("{value:02}")
}

/// Typing a digit into a segment.
///
/// Segments accept two digits, so a digit **shifts** into the segment rather
/// than replacing it: 0 then 4 then 5 in minutes reads 00 → 04 → 45, the way a
/// clock field behaves. Once two digits are in, the oldest falls off.
pub fn type_duration_digit(
    value: DurationValue,
    segment: DurationSegment,
    digit: u32,
    max_hours: u32,
) -> DurationValue {
    let current = match segment {
        DurationSegment::Hours => value.hours,
        DurationSegment::Minutes => value.minutes,
        DurationSegment::Seconds => value.seconds,
    };
    let shifted = i64::from(current % 10) * 10 + i64::from(digit);
    set_duration_segment(value, segment, shifted, max_hours)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(h: u32, m: u32, s: u32) -> DurationValue {
        DurationValue {
            hours: h,
            minutes: m,
            seconds: s,
        }
    }

    #[test]
    fn seconds_carry_into_minutes_and_minutes_into_hours() {
        assert_eq!(
            adjust_duration_segment(v(0, 0, 59), DurationSegment::Seconds, 1, 99),
            v(0, 1, 0)
        );
        assert_eq!(
            adjust_duration_segment(v(0, 59, 59), DurationSegment::Seconds, 1, 99),
            v(1, 0, 0)
        );
        assert_eq!(
            adjust_duration_segment(v(0, 59, 0), DurationSegment::Minutes, 1, 99),
            v(1, 0, 0)
        );
    }

    #[test]
    fn seconds_borrow_back_down_through_minutes_and_hours() {
        assert_eq!(
            adjust_duration_segment(v(0, 1, 0), DurationSegment::Seconds, -1, 99),
            v(0, 0, 59)
        );
        assert_eq!(
            adjust_duration_segment(v(1, 0, 0), DurationSegment::Seconds, -1, 99),
            v(0, 59, 59)
        );
    }

    /// The bound swallows the carry rather than clamping into a partial change:
    /// this is the web behaviour, and the reason the port exists.
    #[test]
    fn carries_at_the_hour_bound_are_swallowed() {
        assert_eq!(
            adjust_duration_segment(v(9, 59, 0), DurationSegment::Minutes, 1, 9),
            v(9, 0, 0)
        );
        assert_eq!(
            adjust_duration_segment(v(0, 0, 0), DurationSegment::Seconds, -1, 9),
            v(0, 59, 59)
        );
    }

    #[test]
    fn hours_clamp_at_both_ends() {
        assert_eq!(
            adjust_duration_segment(v(0, 0, 0), DurationSegment::Hours, -1, 9),
            v(0, 0, 0)
        );
        assert_eq!(
            adjust_duration_segment(v(9, 0, 0), DurationSegment::Hours, 1, 9),
            v(9, 0, 0)
        );
    }

    #[test]
    fn direct_entry_clamps_per_segment() {
        assert_eq!(
            set_duration_segment(v(0, 0, 0), DurationSegment::Minutes, 99, 9),
            v(0, 59, 0)
        );
        assert_eq!(
            set_duration_segment(v(0, 0, 0), DurationSegment::Hours, 50, 9),
            v(9, 0, 0)
        );
    }

    /// Two-digit segments shift, so typing reads like a clock field.
    #[test]
    fn typing_digits_shifts_them_into_the_segment() {
        let mut value = v(0, 0, 0);
        value = type_duration_digit(value, DurationSegment::Minutes, 4, 99);
        assert_eq!(value.minutes, 4);
        value = type_duration_digit(value, DurationSegment::Minutes, 5, 99);
        assert_eq!(value.minutes, 45);
        // A third digit drops the oldest: 45 -> 5 -> 56.
        value = type_duration_digit(value, DurationSegment::Minutes, 6, 99);
        assert_eq!(value.minutes, 56);
    }

    #[test]
    fn typing_past_a_segment_bound_clamps() {
        let value = type_duration_digit(v(0, 5, 0), DurationSegment::Minutes, 9, 99);
        assert_eq!(value.minutes, 59);
    }

    #[test]
    fn total_seconds_and_padding_match_the_web_helpers() {
        assert_eq!(duration_total_seconds(v(1, 2, 3)), 3723);
        assert_eq!(pad_duration_segment(7), "07");
        assert_eq!(pad_duration_segment(42), "42");
    }
}
