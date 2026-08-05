//! TimeAgo — relative time label.
//!
//! Contract: `docs/contracts/components/time-ago.md`
//! Ported from: `packages/jetstream/components/src/time_ago.rs`.
//!
//! Computes the relative-time string from `spec.timestamp` (parse → diff vs
//! `now` → shared `TimeAgoSpec::format_relative`). The threshold table lives
//! in the spec. Live ticking and the absolute-time tooltip are host concerns.

use std::time::{SystemTime, UNIX_EPOCH};

use poodle_adapter::ThemeProvider;
use poodle_node::Node;
use poodle_specs::TimeAgoSpec;

pub fn time_ago(spec: &TimeAgoSpec, theme: &dyn ThemeProvider) -> Node {
    let text_color = theme.resolve_color(spec.text_color_token());

    let display = if spec.timestamp.is_empty() {
        spec.format_relative(0)
    } else {
        relative_time(spec, &spec.timestamp).unwrap_or_else(|| spec.timestamp.clone())
    };

    let mut label = Node::text(display);
    label.style.descriptor.text_color = Some(text_color);

    if !spec.inherits_typography() {
        // Contract §8: font-size = typography.body.size. Token-resolved.
        label.style.text_size = Some(theme.resolve_space(spec.font_size_token()));
    }
    if let Some(aria) = spec.aria_label.as_deref() {
        if !aria.is_empty() {
            label.a11y.label = Some(aria.to_string());
        }
    }
    label
}

// ── Relative-time computation ─────────────────────────────────────────
//
// Parsing mirrors the reference tier; formatting is delegated to
// `TimeAgoSpec::format_relative` (single source of truth).

/// Parse a simple ISO 8601 timestamp and return the relative-time string.
/// Supports "YYYY-MM-DDThh:mm:ss[Z]", "YYYY-MM-DD hh:mm:ss", and
/// "YYYY-MM-DD". Returns `None` if parsing fails, so the caller can fall
/// back to the raw text.
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

    // `now − timestamp`: positive = past, negative = future. The threshold
    // table (no week tier, long-form "yesterday") lives in the shared spec.
    Some(spec.format_relative(now_epoch - ts_epoch))
}

/// Convert a date/time to approximate Unix epoch seconds (proleptic
/// Gregorian).
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
