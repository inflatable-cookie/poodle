//! TimeAgo — real GPUI component backed by TimeAgoSpec.

use std::time::{SystemTime, UNIX_EPOCH};

use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::TimeAgoSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI relative time display component backed by `TimeAgoSpec`.
pub struct TimeAgo {
    spec: TimeAgoSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for TimeAgo {
    type Target = TimeAgoSpec;
    fn deref(&self) -> &TimeAgoSpec { &self.spec }
}

impl TimeAgo {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TimeAgoSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: TimeAgoSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn timestamp(mut self, v: impl Into<String>) -> Self { self.spec.timestamp = v.into(); self }
    pub fn live(mut self, v: bool) -> Self { self.spec.live = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

}

impl IntoElement for TimeAgo {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let text_color = resolve_color(theme, spec.text_color_token());

        let display = if spec.timestamp.is_empty() {
            "just now".to_string()
        } else {
            relative_time(&spec.timestamp).unwrap_or_else(|| spec.timestamp.clone())
        };

        div()
            .text_size(px(12.0))
            .text_color(text_color)
            .child(display)
            .into_any_element()
    }
}

// ── Relative time computation ─────────────────────────────────────

/// Parse a simple ISO 8601 timestamp and return a relative time string.
/// Supports formats: "YYYY-MM-DDThh:mm:ss", "YYYY-MM-DDThh:mm:ssZ",
/// "YYYY-MM-DD hh:mm:ss", and "YYYY-MM-DD".
/// Returns `None` if parsing fails, allowing the caller to fall back.
fn relative_time(timestamp: &str) -> Option<String> {
    let ts = timestamp.trim();
    // Strip trailing 'Z' if present (treat as UTC either way).
    let ts = ts.strip_suffix('Z').unwrap_or(ts);

    let parts: Vec<&str> = if ts.contains('T') {
        ts.splitn(2, 'T').collect()
    } else if ts.contains(' ') {
        ts.splitn(2, ' ').collect()
    } else {
        vec![ts]
    };

    let date_part = parts[0];
    let date_fields: Vec<&str> = date_part.split('-').collect();
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
        let time_fields: Vec<&str> = parts[1].split(':').collect();
        if time_fields.len() < 2 {
            return None;
        }
        let h: u32 = time_fields[0].parse().ok()?;
        let m: u32 = time_fields[1].parse().ok()?;
        let s: u32 = if time_fields.len() >= 3 {
            // Handle fractional seconds by taking only the integer part.
            let s_str = time_fields[2].split('.').next().unwrap_or("0");
            s_str.parse().ok()?
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

    // Convert parsed date/time to approximate Unix epoch seconds.
    let ts_epoch = datetime_to_epoch(year, month, day, hour, minute, second);

    let now_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs() as i64;

    let diff = now_epoch - ts_epoch;

    if diff < 0 {
        // Future timestamps: show "just now" rather than negative values.
        return Some("just now".to_string());
    }

    Some(format_duration(diff as u64))
}

fn format_duration(seconds: u64) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = 3600;
    const DAY: u64 = 86400;
    const WEEK: u64 = 7 * DAY;
    const MONTH: u64 = 30 * DAY;
    const YEAR: u64 = 365 * DAY;

    if seconds < MINUTE {
        "just now".to_string()
    } else if seconds < HOUR {
        format!("{}m ago", seconds / MINUTE)
    } else if seconds < DAY {
        format!("{}h ago", seconds / HOUR)
    } else if seconds < WEEK {
        format!("{}d ago", seconds / DAY)
    } else if seconds < MONTH {
        format!("{}w ago", seconds / WEEK)
    } else if seconds < YEAR {
        format!("{}mo ago", seconds / MONTH)
    } else {
        format!("{}y ago", seconds / YEAR)
    }
}

/// Convert a date/time to approximate Unix epoch seconds.
/// Uses a simple days-since-epoch calculation (no leap-second precision needed).
fn datetime_to_epoch(year: i64, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> i64 {
    // Days in each month for a non-leap year.
    let days_before_month: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

    let m_idx = (month - 1) as usize;
    let mut day_of_year = days_before_month[m_idx] as i64 + day as i64 - 1;

    // Leap-year adjustment: if past February, add 1 if leap year.
    if month > 2 && (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)) {
        day_of_year += 1;
    }

    // Total days from year 1 through (year-1), then add day_of_year.
    let y = year - 1;
    let era_days = y * 365 + y / 4 - y / 100 + y / 400 + day_of_year;

    // Unix epoch: 1970-01-01 = day 719_162 from year 0 in proleptic Gregorian.
    let unix_days = era_days - 719_162;
    unix_days * 86400 + hour as i64 * 3600 + min as i64 * 60 + sec as i64
}
