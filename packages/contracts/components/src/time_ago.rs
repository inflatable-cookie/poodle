use poodle_tokens::semantic;

use crate::InlineTypographyMode;

/// Absolute-time format used by the tooltip (`formatAbsolute` in `TimeAgo.svelte`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TimeAgoTooltipFormat {
    /// `year/month/day` only.
    Date,
    /// `year/month/day` + `hour:minute` (Svelte default).
    #[default]
    Datetime,
    /// `datetime` + seconds + short timezone name.
    Full,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeAgoSpec {
    pub timestamp: String,
    /// When true (default), the relative text is periodically recomputed.
    /// Live ticking is owned by the runtime/preview loop.
    pub live: bool,
    /// Live update interval in milliseconds. Contract default `30000`.
    pub interval: u32,
    /// When true (default), renders compact forms like "2m ago".
    /// When false, renders long forms like "2 minutes ago".
    pub short: bool,
    pub aria_label: Option<String>,
    pub typography: InlineTypographyMode,
    /// Absolute-time format used for the tooltip. Contract default `datetime`.
    pub tooltip_format: TimeAgoTooltipFormat,
    /// Optional IANA timezone for tooltip formatting (`null` = local).
    pub timezone: Option<String>,
}

impl Default for TimeAgoSpec {
    fn default() -> Self {
        Self {
            timestamp: String::new(),
            live: true,
            interval: 30_000,
            short: true,
            aria_label: None,
            typography: InlineTypographyMode::default(),
            tooltip_format: TimeAgoTooltipFormat::default(),
            timezone: None,
        }
    }
}

impl TimeAgoSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = timestamp.into();
        self
    }

    pub fn with_live(mut self, live: bool) -> Self {
        self.live = live;
        self
    }

    pub fn with_interval(mut self, interval: u32) -> Self {
        self.interval = interval;
        self
    }

    pub fn with_short(mut self, short: bool) -> Self {
        self.short = short;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_typography(mut self, typography: InlineTypographyMode) -> Self {
        self.typography = typography;
        self
    }

    pub fn with_tooltip_format(mut self, format: TimeAgoTooltipFormat) -> Self {
        self.tooltip_format = format;
        self
    }

    pub fn with_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }

    /// Contract §8: root color is `text-primary` (was incorrectly `secondary`).
    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    pub fn inherits_typography(&self) -> bool {
        self.typography == InlineTypographyMode::Inherit
    }

    /// Format the relative-time string for a difference measured in seconds.
    ///
    /// `diff_seconds` is `now − timestamp` (positive = past, negative = future).
    /// Shared by both Rust targets so the threshold table is defined once and
    /// matches `formatRelative` in `TimeAgo.svelte` exactly (no week tier).
    pub fn format_relative(&self, diff_seconds: i64) -> String {
        format_relative(diff_seconds, self.short)
    }
}

// ── Relative-time formatting (single source of truth) ─────────────────
//
// Mirrors `formatRelative` in `packages/svelte/components/src/TimeAgo.svelte`.
// Thresholds in seconds: 5, 60, 3600, 86400, 2_592_000 (30d), 31_536_000 (365d).
// No "week" tier — Svelte/contract go straight from days (<30d) to months.

const MINUTE: u64 = 60;
const HOUR: u64 = 3_600;
const DAY: u64 = 86_400;
const MONTH: u64 = 30 * DAY; // 2_592_000
const YEAR: u64 = 365 * DAY; // 31_536_000

/// Format a relative-time label. `diff_seconds` is `now − timestamp`.
pub fn format_relative(diff_seconds: i64, short: bool) -> String {
    let is_future = diff_seconds < 0;
    let abs = diff_seconds.unsigned_abs();

    if abs < 5 {
        return if short { "now" } else { "just now" }.to_string();
    }

    let (value, unit_short, unit_singular, unit_plural) = if abs < MINUTE {
        (abs, "s", "second", "seconds")
    } else if abs < HOUR {
        (abs / MINUTE, "m", "minute", "minutes")
    } else if abs < DAY {
        (abs / HOUR, "h", "hour", "hours")
    } else if abs < MONTH {
        let days = abs / DAY;
        // Long form, past, exactly one day → "yesterday".
        if !short && !is_future && days == 1 {
            return "yesterday".to_string();
        }
        (days, "d", "day", "days")
    } else if abs < YEAR {
        (abs / MONTH, "mo", "month", "months")
    } else {
        (abs / YEAR, "y", "year", "years")
    };

    if short {
        if is_future {
            format!("in {value}{unit_short}")
        } else {
            format!("{value}{unit_short} ago")
        }
    } else {
        let unit = if value == 1 {
            unit_singular
        } else {
            unit_plural
        };
        if is_future {
            format!("in {value} {unit}")
        } else {
            format!("{value} {unit} ago")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_contract() {
        let s = TimeAgoSpec::new();
        assert!(s.live, "live defaults to true");
        assert_eq!(s.interval, 30_000);
        assert!(s.short);
        assert_eq!(s.tooltip_format, TimeAgoTooltipFormat::Datetime);
        assert_eq!(s.text_color_token(), semantic::COLOR_TEXT_PRIMARY);
    }

    #[test]
    fn short_thresholds_match_svelte() {
        // just-now
        assert_eq!(format_relative(0, true), "now");
        assert_eq!(format_relative(4, true), "now");
        // seconds
        assert_eq!(format_relative(30, true), "30s ago");
        // minutes
        assert_eq!(format_relative(2 * 60, true), "2m ago");
        // hours
        assert_eq!(format_relative(3 * 3600, true), "3h ago");
        // days
        assert_eq!(format_relative(2 * 86_400, true), "2d ago");
        // months (no week tier: 10 days is still days)
        assert_eq!(format_relative(10 * 86_400, true), "10d ago");
        assert_eq!(format_relative(40 * 86_400, true), "1mo ago");
        // years
        assert_eq!(format_relative(400 * 86_400, true), "1y ago");
    }

    #[test]
    fn future_short_uses_in_prefix() {
        assert_eq!(format_relative(-(5 * 60), true), "in 5m");
        assert_eq!(format_relative(-(2 * 3600), true), "in 2h");
    }

    #[test]
    fn long_form_pluralizes_and_yesterday() {
        assert_eq!(format_relative(0, false), "just now");
        assert_eq!(format_relative(60, false), "1 minute ago");
        assert_eq!(format_relative(120, false), "2 minutes ago");
        // exactly one day, long, past → "yesterday"
        assert_eq!(format_relative(86_400, false), "yesterday");
        // two days stays "2 days ago"
        assert_eq!(format_relative(2 * 86_400, false), "2 days ago");
        // future one day is NOT "yesterday"
        assert_eq!(format_relative(-86_400, false), "in 1 day");
    }

    #[test]
    fn no_week_tier() {
        // 10 days must be "10d ago" / "10 days ago", never "1w ago".
        assert_eq!(format_relative(10 * 86_400, true), "10d ago");
        assert_eq!(format_relative(10 * 86_400, false), "10 days ago");
    }
}
