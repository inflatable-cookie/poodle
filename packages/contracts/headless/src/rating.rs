//! Rating value machinery. Mirror of core `rating.ts`.
//!
//! Pure step, clamp, snap, pointer, clear, fill, format, and keyboard math.
//! Renderer and adapters own focus, hover, and geometry.

/// Steps are capped at 1 and fall back to 1 when invalid or non-positive.
pub fn resolve_rating_step(step: f64) -> f64 {
    if !step.is_finite() || step <= 0.0 {
        1.0
    } else {
        step.min(1.0)
    }
}

pub fn round_rating_to_step(value: f64, step: f64) -> f64 {
    let rounded = (value / step).round() * step;
    round4(rounded)
}

/// Incoming display values clamp but do not quantize. `None` stays empty.
pub fn clamp_rating_display_value(value: Option<f64>, max: f64) -> Option<f64> {
    value.map(|raw| round4(raw.clamp(0.0, max)))
}

/// User-produced values clamp then snap to the effective step.
pub fn normalize_rating_value(value: Option<f64>, max: f64, step: f64) -> Option<f64> {
    let clamped = clamp_rating_display_value(value, max)?;
    Some(round_rating_to_step(clamped, step))
}

/// Display formatting: integers bare, fractions trimmed to two places.
pub fn trim_rating_fraction(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        let text = format!("{value:.2}");
        text.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// Fill ratio of the item at `index` for a given value, in `[0, 1]`.
pub fn rating_fill_ratio(index: f64, value: f64) -> f64 {
    (value - index).clamp(0.0, 1.0)
}

/// Value for a pointer position within the item at `index`:
/// `ratio_within_item` in `[0, 1]` snaps UP to the next step (minimum one step).
pub fn rating_pointer_value(
    ratio_within_item: f64,
    index: f64,
    step: f64,
    item_count: f64,
) -> f64 {
    let snapped = step.max((ratio_within_item / step).ceil() * step);
    item_count.min(index + snapped.min(1.0))
}

/// Selection with clear-on-reselect: returns the next value (`None` clears).
pub fn rating_select_value(
    next_value: f64,
    current_value: Option<f64>,
    allow_clear: bool,
) -> Option<f64> {
    if allow_clear && current_value == Some(next_value) {
        None
    } else {
        Some(next_value)
    }
}

/// Keyboard step: both directions floor at `min_selectable_value`; up caps at `item_count`.
pub fn rating_keyboard_step(
    current_value: f64,
    direction: i8,
    step: f64,
    item_count: f64,
    min_selectable_value: f64,
) -> f64 {
    if direction > 0 {
        item_count.min(min_selectable_value.max(current_value + step))
    } else {
        min_selectable_value.max(current_value - step)
    }
}

/// Effective item count: at least one star.
pub fn rating_item_count(max: u8) -> u8 {
    max.max(1)
}

/// Fractional accessibility value text.
pub fn rating_value_text(value: Option<f64>, item_count: u8) -> String {
    match value {
        None | Some(0.0) => format!("No rating selected out of {item_count}"),
        Some(raw) => format!("{} out of {item_count}", trim_rating_fraction(raw)),
    }
}

fn round4(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_resolution_caps_at_1_and_rejects_invalid() {
        assert_eq!(resolve_rating_step(0.5), 0.5);
        assert_eq!(resolve_rating_step(2.0), 1.0);
        assert_eq!(resolve_rating_step(0.0), 1.0);
        assert_eq!(resolve_rating_step(f64::NAN), 1.0);
    }

    #[test]
    fn normalize_clamps_then_snaps_null_passes_through() {
        assert_eq!(normalize_rating_value(Some(3.26), 5.0, 0.5), Some(3.5));
        assert_eq!(normalize_rating_value(Some(9.0), 5.0, 1.0), Some(5.0));
        assert_eq!(normalize_rating_value(Some(-2.0), 5.0, 1.0), Some(0.0));
        assert_eq!(normalize_rating_value(None, 5.0, 1.0), None);
        assert_eq!(clamp_rating_display_value(Some(2.123456), 5.0), Some(2.1235));
    }

    #[test]
    fn fraction_trimming_and_fill_ratio() {
        assert_eq!(trim_rating_fraction(3.0), "3");
        assert_eq!(trim_rating_fraction(3.5), "3.5");
        assert_eq!(trim_rating_fraction(3.25), "3.25");
        assert_eq!(rating_fill_ratio(2.0, 2.5), 0.5);
        assert_eq!(rating_fill_ratio(3.0, 2.5), 0.0);
        assert_eq!(rating_fill_ratio(0.0, 2.5), 1.0);
    }

    #[test]
    fn pointer_value_snaps_up_within_the_item_capped_at_item_count() {
        assert_eq!(rating_pointer_value(0.3, 2.0, 0.5, 5.0), 2.5);
        assert_eq!(rating_pointer_value(0.6, 2.0, 0.5, 5.0), 3.0);
        assert_eq!(rating_pointer_value(0.01, 2.0, 0.5, 5.0), 2.5);
        assert_eq!(rating_pointer_value(1.0, 4.0, 1.0, 5.0), 5.0);
    }

    #[test]
    fn clear_on_reselect_and_keyboard_stepping() {
        assert_eq!(rating_select_value(3.0, Some(3.0), true), None);
        assert_eq!(rating_select_value(3.0, Some(3.0), false), Some(3.0));
        assert_eq!(rating_select_value(4.0, Some(3.0), true), Some(4.0));
        assert_eq!(rating_keyboard_step(3.0, 1, 0.5, 5.0, 0.5), 3.5);
        assert_eq!(rating_keyboard_step(5.0, 1, 0.5, 5.0, 0.5), 5.0);
        assert_eq!(rating_keyboard_step(0.5, -1, 0.5, 5.0, 0.5), 0.5);
    }

    #[test]
    fn value_text_keeps_empty_distinct_from_zero_display_path() {
        assert_eq!(
            rating_value_text(None, 5),
            "No rating selected out of 5"
        );
        assert_eq!(
            rating_value_text(Some(0.0), 5),
            "No rating selected out of 5"
        );
        assert_eq!(rating_value_text(Some(3.5), 5), "3.5 out of 5");
    }
}
