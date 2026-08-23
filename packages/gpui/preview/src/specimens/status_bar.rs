use crate::app_state::AppState;
use crate::node_compat::{CompatRow, Eyebrow, StatusBar};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;

use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use poodle_render::context::RenderContext;
use poodle_specs::ShellStatusBarSpec;
use poodle_specs::{EyebrowSpec, StatusIndicatorSpec, StatusTone};

/// A branch indicator (info tone dot) + diagnostics indicator (success tone dot),
/// matching the contract §12 "Default" leading region.
fn leading_items(theme: &GpuiThemeProvider) -> CompatRow {
    let branch = StatusIndicatorSpec::new()
        .with_status(StatusTone::Info)
        .with_label("main");
    let diagnostics = StatusIndicatorSpec::new()
        .with_status(StatusTone::Success)
        .with_label("0 errors");
    let ctx = RenderContext::new(theme);
    CompatRow::new()
        .gap(8.0)
        .child(poodle_render::status_indicator(&branch, &ctx))
        .child(poodle_render::status_indicator(&diagnostics, &ctx))
}

/// Trailing cursor/encoding/language metadata. Plain text children that inherit
/// the bar's resolved font-size + secondary text color (Svelte `font-size:
/// inherit`); no per-item size/color overrides.
fn trailing_meta(items: &[&str]) -> CompatRow {
    let mut row = CompatRow::new().gap(8.0);
    for item in items {
        row = row.child(*item);
    }
    row
}

fn group(theme: &GpuiThemeProvider, label: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default (no chrome): full bar, blends into container ---
        .child(group(
            theme,
            "Default (no chrome)",
            StatusBar::from_spec(ShellStatusBarSpec::new().with_summary("Ready"), theme)
                .with_leading_items(leading_items(theme))
                .with_trailing_items(trailing_meta(&["Ln 42, Col 18", "UTF-8", "TypeScript"])),
        ))
        // --- With chrome: component-driven border-top + 94% panel bg ---
        .child(group(
            theme,
            "With chrome",
            StatusBar::from_spec(
                ShellStatusBarSpec::new()
                    .with_summary("Ready")
                    .with_chrome(true),
                theme,
            )
            .chrome(true)
            .with_leading_items(leading_items(theme))
            .with_trailing_items(trailing_meta(&[
                "Ln 42, Col 18",
                "UTF-8",
                "TypeScript",
            ])),
        ))
        // --- Summary only: leading region shows summary text, no trailing ---
        .child(group(
            theme,
            "Summary only",
            StatusBar::from_spec(
                ShellStatusBarSpec::new().with_summary("3 items selected"),
                theme,
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "status-bar",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                StatusBar::from_spec(
                    ShellStatusBarSpec::new()
                        .with_summary("Status bar")
                        .with_chrome(true),
                    theme,
                )
                .chrome(true)
                .with_size(size)
                .with_trailing_items(trailing_meta(&["UTF-8", "TypeScript"]))
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                StatusBar::from_spec(
                    ShellStatusBarSpec::new()
                        .with_summary("Status bar")
                        .with_chrome(true),
                    theme,
                )
                .chrome(true)
                .with_density(density)
                .with_trailing_items(trailing_meta(&["UTF-8", "TypeScript"]))
                .into_any_element()
            }),
    )
}
