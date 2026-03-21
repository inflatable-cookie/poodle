use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{StatusIndicatorSpec, StatusTone};
use pug_gpui_components::StatusIndicator;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // --- All statuses ---
    let mut neutral = StatusIndicatorSpec::new().with_status(StatusTone::Neutral);
    neutral.label = Some("Neutral".to_string());

    let mut info = StatusIndicatorSpec::new().with_status(StatusTone::Info);
    info.label = Some("Info".to_string());

    let mut success = StatusIndicatorSpec::new().with_status(StatusTone::Success);
    success.label = Some("Success".to_string());

    let mut warning = StatusIndicatorSpec::new().with_status(StatusTone::Warning);
    warning.label = Some("Warning".to_string());

    let mut danger = StatusIndicatorSpec::new().with_status(StatusTone::Danger);
    danger.label = Some("Danger".to_string());

    let mut pending = StatusIndicatorSpec::new().with_status(StatusTone::Pending);
    pending.label = Some("Pending".to_string());

    // --- Without labels (dot only) ---
    let mut online = StatusIndicatorSpec::new().with_status(StatusTone::Success);
    online.aria_label = Some("Online".to_string());

    let mut away = StatusIndicatorSpec::new().with_status(StatusTone::Warning);
    away.aria_label = Some("Away".to_string());

    let mut offline = StatusIndicatorSpec::new().with_status(StatusTone::Danger);
    offline.aria_label = Some("Offline".to_string());

    let mut unknown = StatusIndicatorSpec::new().with_status(StatusTone::Neutral);
    unknown.aria_label = Some("Unknown".to_string());

    // --- Slot content ---
    let mut build = StatusIndicatorSpec::new().with_status(StatusTone::Success);
    build.label = Some("Build passing".to_string());

    div().flex().flex_col().gap(px(24.0))
        // --- All statuses ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("ALL STATUSES", text_secondary))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(StatusIndicator::from_spec(neutral, theme))
                        .child(StatusIndicator::from_spec(info, theme))
                        .child(StatusIndicator::from_spec(success, theme))
                        .child(StatusIndicator::from_spec(warning, theme))
                        .child(StatusIndicator::from_spec(danger, theme))
                        .child(StatusIndicator::from_spec(pending, theme))
                )
        )
        // --- Without labels (dot only) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("WITHOUT LABELS (DOT ONLY)", text_secondary))
                .child(
                    div().flex().gap(px(16.0)).items_center()
                        .child(StatusIndicator::from_spec(online, theme))
                        .child(StatusIndicator::from_spec(away, theme))
                        .child(StatusIndicator::from_spec(offline, theme))
                        .child(StatusIndicator::from_spec(unknown, theme))
                )
        )
        // --- Slot content ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("SLOT CONTENT", text_secondary))
                .child(StatusIndicator::from_spec(build, theme))
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
