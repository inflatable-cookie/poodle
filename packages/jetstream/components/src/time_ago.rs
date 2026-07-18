//! TimeAgo — Jetstream relative time label backed by TimeAgoSpec.
//!
//! Contract: `docs/contracts/components/time-ago.md`
//! Reference: `packages/svelte/components/src/TimeAgo.svelte`.
//!
//! Computes the relative-time string from `spec.timestamp` (parse → diff vs
//! `now` → shared `TimeAgoSpec::format_relative`). The threshold table lives in
//! the spec so both Rust targets stay identical. Live ticking and the absolute-
//! time tooltip are preview-loop concerns (see notes), matching GPUI.

use std::time::{SystemTime, UNIX_EPOCH};

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::TimeAgoSpec;

use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_time_ago(spec: &TimeAgoSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, spec.text_color_token());

    let display = if spec.timestamp.is_empty() {
        spec.format_relative(0)
    } else {
        relative_time(spec, &spec.timestamp).unwrap_or_else(|| spec.timestamp.clone())
    };

    let label = ui_element::label(display).text_color(text_color);

    if spec.inherits_typography() {
        label
    } else {
        // Contract §8: font-size = typography.body.size. Token-resolved.
        label.text_size(resolve_px(theme, spec.font_size_token()))
    }
}

// ── Relative-time computation ─────────────────────────────────────────
//
// Parsing mirrors the GPUI primitive (`primitives/time_ago.rs`); formatting is
// delegated to `TimeAgoSpec::format_relative` (single source of truth).

/// Parse a simple ISO 8601 timestamp and return the relative-time string.
/// Supports "YYYY-MM-DDThh:mm:ss[Z]", "YYYY-MM-DD hh:mm:ss", and "YYYY-MM-DD".
/// Returns `None` if parsing fails, so the caller can fall back to the raw text.
fn relative_time(spec: &TimeAgoSpec, timestamp: &str) -> Option<String> {
    let ts = timestamp.trim();
    let ts = ts.strip_suffix('Z').unwrap_or(ts);

    let parts: Vec<&str> = if ts.contains('T') {
        ts.splitn(2, 'T').collect()
    } else if ts.contains(' ') {
        ts.splitn(2, ' ').collect()
    } else {
        vec![ts]
    };

    let date_fields: Vec<&str> = parts[0].split('-').collect();
    if date_fields.len() != 3 {
        return None;
    }
    let year: i64 = date_fields[0].parse().ok()?;
    let month: u32 = date_fields[1].parse().ok()?;
    let day: u32 = date_fields[2].parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }

    let (hour, minute, second) = if parts.len() == 2 {
        let tf: Vec<&str> = parts[1].split(':').collect();
        if tf.len() < 2 {
            return None;
        }
        let h: u32 = tf[0].parse().ok()?;
        let m: u32 = tf[1].parse().ok()?;
        let s: u32 = if tf.len() >= 3 {
            tf[2].split('.').next().unwrap_or("0").parse().ok()?
        } else {
            0
        };
        (h, m, s)
    } else {
        (0, 0, 0)
    };
    if hour > 23 || minute > 59 || second > 60 {
        return None;
    }

    let ts_epoch = datetime_to_epoch(year, month, day, hour, minute, second);
    let now_epoch = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;

    // `now − timestamp`: positive = past, negative = future. The threshold table
    // (no week tier, long-form "yesterday") lives in the shared spec method.
    Some(spec.format_relative(now_epoch - ts_epoch))
}

/// Convert a date/time to approximate Unix epoch seconds (proleptic Gregorian).
fn datetime_to_epoch(year: i64, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> i64 {
    let days_before_month: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let m_idx = (month - 1) as usize;
    let mut day_of_year = days_before_month[m_idx] as i64 + day as i64 - 1;
    if month > 2 && (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)) {
        day_of_year += 1;
    }
    let y = year - 1;
    let era_days = y * 365 + y / 4 - y / 100 + y / 400 + day_of_year;
    let unix_days = era_days - 719_162;
    unix_days * 86400 + hour as i64 * 3600 + min as i64 * 60 + sec as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    /// Build an ISO timestamp `secs` seconds before now (UTC) for stable diffs.
    fn iso_secs_ago(secs: i64) -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let target = now - secs;
        let days = target.div_euclid(86_400);
        let rem = target.rem_euclid(86_400);
        let (h, m, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
        // Inverse of datetime_to_epoch's day count (civil-from-days, proleptic Gregorian).
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let month = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = if month <= 2 { y + 1 } else { y };
        format!("{year:04}-{month:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
    }

    #[test]
    fn computes_relative_string_from_timestamp() {
        // 2 minutes ago → "2m ago" (short default). Proves it formats, not
        // labels the raw ISO string.
        let spec = TimeAgoSpec::new().with_timestamp(iso_secs_ago(2 * 60));
        let el = js_time_ago(&spec, &theme());
        let tree = probe(&el, 200.0, 40.0);
        assert!(
            tree.has_text("2m ago"),
            "expected '2m ago', got {:?}",
            tree.texts()
        );
    }

    #[test]
    fn hours_and_days_thresholds() {
        let th = theme();
        let h = js_time_ago(&TimeAgoSpec::new().with_timestamp(iso_secs_ago(3 * 3600)), &th);
        assert!(probe(&h, 200.0, 40.0).has_text("3h ago"));

        let d = js_time_ago(&TimeAgoSpec::new().with_timestamp(iso_secs_ago(2 * 86_400)), &th);
        assert!(probe(&d, 200.0, 40.0).has_text("2d ago"));
    }

    #[test]
    fn no_week_tier_ten_days() {
        // 10 days must read "10d ago", never "1w ago".
        let spec = TimeAgoSpec::new().with_timestamp(iso_secs_ago(10 * 86_400));
        let tree = probe(&js_time_ago(&spec, &theme()), 200.0, 40.0);
        assert!(tree.has_text("10d ago"), "got {:?}", tree.texts());
        assert!(!tree.has_text("1w ago"));
    }

    #[test]
    fn long_form_uses_words() {
        let spec = TimeAgoSpec::new()
            .with_short(false)
            .with_timestamp(iso_secs_ago(2 * 60));
        let tree = probe(&js_time_ago(&spec, &theme()), 240.0, 40.0);
        assert!(tree.has_text("2 minutes ago"), "got {:?}", tree.texts());
    }

    #[test]
    fn tone_resolves_text_primary() {
        // Contract §8: color = text.primary (the spec fix). The label's text
        // color must equal the resolved text.primary, not text.secondary.
        let th = theme();
        let spec = TimeAgoSpec::new().with_timestamp(iso_secs_ago(60));
        let el = js_time_ago(&spec, &th);
        // text_color_token() now returns text.primary (was secondary).
        let primary = resolve_color(&th, spec.text_color_token());
        let secondary = resolve_color(&th, "color.text.secondary");
        assert_eq!(el.style.text_color, Some(primary.into()));
        assert_ne!(el.style.text_color, Some(secondary.into()));
    }
}
