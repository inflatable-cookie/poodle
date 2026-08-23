//! LicenceStatus — a window onto the held licence: usability, trust basis,
//! and both coverage windows.
//!
//! Contract: `docs/contracts/components/licence-status.md`
//!
//! Renders supplied state and pure core view data
//! (`poodle_headless::licence::licence_status_view`); it owns no entitlement
//! or licence transition. Every display decision — title, tone, row terms,
//! which timestamp goes where — resolves once in the headless mirror, so the
//! native surface cannot disagree with the web pair about what a licence
//! state means.
//!
//! `usable` and `attention` are authority reads. They reach the tree as
//! semantic token roles (the native data-* counterpart) and nothing else: no
//! branch hides a row, disables a control, or turns a licence read into a
//! feature permission.

use std::time::{SystemTime, UNIX_EPOCH};

use poodle_headless::licence::{
    civil_from_days, format_time_date_parts, licence_status_view, LicenceStatusInput,
    LicenceStatusView,
};
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node, NodeRole};
use poodle_specs::{LicenceStatusSpec, StatusIndicatorSpec, StatusTone, TimeAgoSpec};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;
use crate::status_indicator::status_indicator;

fn now_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Resolve an authority timestamp into **local** civil parts, matching the
/// web's `toLocaleTimeString`/`toLocaleDateString` treatment of the quiet
/// `inGrace` line. Unix targets use the platform's `localtime`; other
/// platforms fall back to the same instant in UTC (the GPUI target is Unix).
fn local_time_parts(epoch_seconds: i64) -> (i64, i64, i64, i64, i64) {
    #[cfg(unix)]
    {
        let mut tm: libc::tm = unsafe { std::mem::zeroed() };
        let t = epoch_seconds as libc::time_t;
        if !unsafe { libc::localtime_r(&t, &mut tm) }.is_null() {
            return (
                tm.tm_year as i64 + 1900,
                tm.tm_mon as i64 + 1,
                tm.tm_mday as i64,
                tm.tm_hour as i64,
                tm.tm_min as i64,
            );
        }
    }
    let days = epoch_seconds.div_euclid(86_400);
    let seconds_of_day = epoch_seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    (
        year,
        month,
        day,
        seconds_of_day / 3_600,
        (seconds_of_day % 3_600) / 60,
    )
}

/// The absolute local time/date for the quiet `inGrace` line. Every runtime
/// uses the contract's fixed 24-hour day/month/year presentation.
fn absolute_time_text(timestamp_ms: i64) -> String {
    let (year, month, day, hour, minute) = local_time_parts(timestamp_ms / 1_000);
    format_time_date_parts(year, month, day, hour, minute)
}

/// Relative text for a timestamp row: `ends in 5m` / `ended 2d ago`, through
/// the shared `TimeAgoSpec::format_relative` thresholds.
fn relative_row_text(timestamp_ms: i64, future_prefix: &str, past_prefix: &str) -> String {
    let spec = TimeAgoSpec::new()
        .with_future_prefix(future_prefix)
        .with_past_prefix(past_prefix);
    spec.format_relative(now_seconds() - timestamp_ms / 1_000)
}

fn indicator_tone(view: &LicenceStatusView) -> StatusTone {
    use poodle_headless::licence::LicenceStatusIndicator as I;
    match view.indicator {
        I::Neutral => StatusTone::Neutral,
        I::Success => StatusTone::Success,
        I::Warning => StatusTone::Warning,
        I::Danger => StatusTone::Danger,
    }
}

pub fn licence_status(spec: &LicenceStatusSpec, ctx: &RenderContext<'_>) -> Node {
    let view = licence_status_view(LicenceStatusInput {
        usability: spec.usability.clone(),
        trust_basis: spec.trust_basis.clone(),
        use_until: spec.use_until,
        update_until: spec.update_until,
        usable: spec.usable,
        attention: spec.attention,
    });

    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let text_tertiary = ctx.theme().resolve_color("color.text.tertiary");

    // ── Head: StatusIndicator + state title ──
    let indicator = status_indicator(
        &StatusIndicatorSpec::new().with_status(indicator_tone(&view)),
        ctx,
    );
    let mut title = Node::text(&view.title);
    title.style.text_size = Some(rem_to_px(1.0));
    title.style.text_weight = Some(600);
    title.style.descriptor.text_color = Some(text_primary);
    let mut head = Node::container();
    {
        let s = &mut head.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
    }
    let head = head.child(indicator).child(title);

    // ── State body ──
    let mut body_text = view.body.text.clone();
    if let Some(timestamp_ms) = view.body.timestamp_ms {
        body_text.push(' ');
        body_text.push_str(&relative_row_text(timestamp_ms, "ends", "ended"));
    }
    let mut body = Node::text(&body_text);
    body.style.text_size = Some(ctx.theme().resolve_space("typography.body.size"));
    body.style.descriptor.text_color = Some(text_primary);

    // ── Definition list: coverage rows + trust basis ──
    // Use coverage, update coverage and trust basis are three labelled
    // values, never merged — a single "expires" line is how someone with
    // lapsed updates is told they have lost the software they own.
    let mut dl = Node::container();
    {
        let s = &mut dl.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.xs");
    }
    for row in &view.coverage {
        let value = match row.timestamp_ms {
            Some(timestamp_ms) => relative_row_text(
                timestamp_ms,
                row.future_prefix.as_deref().unwrap_or("ends"),
                row.past_prefix.as_deref().unwrap_or("ended"),
            ),
            None => row.text.clone().unwrap_or_default(),
        };
        dl = dl.child(dl_row(ctx, &row.term, &value, text_primary, text_secondary));
    }
    let trust_value = match view.trust.timestamp_ms {
        Some(timestamp_ms) => {
            format!("{} {}", view.trust.text, relative_row_text(timestamp_ms, "ends", "ended"))
        }
        None => view.trust.text.clone(),
    };
    let dl = dl.child(dl_row(
        ctx,
        &view.trust.term,
        &trust_value,
        text_primary,
        text_secondary,
    ));

    // ── Quiet detail (inGrace only): absolute local time and date ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.sm");
    }
    let mut root = root.child(head).child(body).child(dl);
    if let Some(detail) = &view.detail {
        let detail_text = match detail.timestamp_ms {
            Some(timestamp_ms) => format!(
                "{} {}",
                detail.text,
                absolute_time_text(timestamp_ms)
            ),
            None => detail.text.clone(),
        };
        let mut quiet = Node::text(&detail_text);
        quiet.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
        quiet.style.descriptor.text_color = Some(text_tertiary);
        root = root.child(quiet);
    }

    // ── Section identity + authority reads as data state ──
    root.a11y.role = Some(NodeRole::Region);
    root.a11y.label = Some(spec.title.clone());
    root.roles.insert("state".to_owned(), view.state.to_string());
    root.roles.insert(
        "tone".to_owned(),
        format!("{:?}", view.tone).to_ascii_lowercase(),
    );
    root.roles.insert(
        "attention".to_owned(),
        format!("{:?}", view.attention).to_ascii_lowercase(),
    );
    root.roles.insert("usable".to_owned(), view.usable.to_string());
    root
}

/// One `dt`/`dd` pair rendered as a labelled row (web §6: semantic
/// definition list).
fn dl_row(
    ctx: &RenderContext<'_>,
    term: &str,
    value: &str,
    term_color: poodle_node::ColorValue,
    value_color: poodle_node::ColorValue,
) -> Node {
    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.md");
    }
    let mut term_node = Node::text(term);
    term_node.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
    term_node.style.descriptor.text_color = Some(term_color);
    let mut value_node = Node::text(value);
    value_node.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
    value_node.style.descriptor.text_color = Some(value_color);
    row.child(term_node).child(value_node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::licence::{
        LicenceAttention, LicenceUsability,
    };
    use poodle_specs::LicenceStatusSpec;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> LicenceStatusSpec {
        LicenceStatusSpec::new()
    }

    fn texts(node: &Node) -> Vec<String> {
        node.texts()
            .into_iter()
            .map(str::to_string)
            .filter(|t| !t.is_empty())
            .collect()
    }

    /// One case per usability state, with the web's copy.
    #[test]
    fn every_state_renders_with_the_web_copy() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let active = licence_status(
            &spec().with_usability(LicenceUsability::Active),
            &ctx,
        );
        let rendered = texts(&active);
        assert!(rendered.iter().any(|t| t == "Licence active"));
        assert!(rendered.iter().any(|t| t == "Use is currently covered."));

        let grace = licence_status(
            &spec().with_usability(LicenceUsability::InGrace { until: 1_799_999_100 }),
            &ctx,
        );
        let rendered = texts(&grace);
        assert!(rendered.iter().any(|t| t == "Licence active"));
        assert!(rendered.iter().any(|t| t.starts_with("Use continues until")));
        assert!(rendered.iter().any(|t| t.contains("2027")), "{rendered:?}");

        let expired = licence_status(
            &spec().with_usability(LicenceUsability::UseWindowExpired { at: 1_799_999_100 }),
            &ctx,
        );
        let rendered = texts(&expired);
        assert!(rendered.iter().any(|t| t == "Use coverage ended"));
        assert!(rendered.iter().any(|t| t.starts_with("This licence stopped covering use")));

        let lapsed = licence_status(
            &spec().with_usability(LicenceUsability::LeaseLapsed { at: 1_799_999_100 }),
            &ctx,
        );
        let rendered = texts(&lapsed);
        assert!(rendered.iter().any(|t| t == "Licence confirmation required"));

        let clock = licence_status(
            &spec().with_usability(LicenceUsability::ClockRefused),
            &ctx,
        );
        let rendered = texts(&clock);
        assert!(rendered.iter().any(|t| t == "Check this machine's clock"));
        assert!(rendered.iter().any(|t| t.contains("clock moved backwards")));
        assert!(!rendered.iter().any(|t| t.to_lowercase().contains("expired")));
    }

    /// Use and update coverage are two visible rows in all null/value
    /// combinations, plus the trust row.
    #[test]
    fn coverage_rows_cover_null_and_value_combinations() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_status(
            &spec()
                .with_use_until(None)
                .with_update_until(Some(1_800_000_000)),
            &ctx,
        );
        let rendered = texts(&node);
        assert!(rendered.iter().any(|t| t == "Use coverage"));
        assert!(rendered.iter().any(|t| t == "No end date"));
        assert!(rendered.iter().any(|t| t == "Updates"));
        assert!(rendered.iter().any(|t| t.starts_with("end in ")));
        assert!(rendered.iter().any(|t| t == "Trust basis"));
        assert!(rendered.iter().any(|t| t == "verified on this machine"));
    }

    /// The root reports state/attention/usable as data state and nothing
    /// gates on them.
    #[test]
    fn authority_reads_reach_the_tree_as_data_state_only() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_status(
            &spec()
                .with_usability(LicenceUsability::InGrace { until: 1_800_000_000 })
                .with_attention(LicenceAttention::Informational)
                .with_usable(true),
            &ctx,
        );
        assert_eq!(node.roles.get("state").map(String::as_str), Some("inGrace"));
        assert_eq!(
            node.roles.get("attention").map(String::as_str),
            Some("informational")
        );
        assert_eq!(node.roles.get("usable").map(String::as_str), Some("true"));
        assert_eq!(
            node.a11y.label.as_deref(),
            Some("Licence"),
            "the section carries the accessible name"
        );
    }
}
