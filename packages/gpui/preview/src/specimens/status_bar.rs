use crate::app_state::AppState;
use crate::node_compat::{CompatRow, Eyebrow, StatusBar};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;

use poodle_specs::ShellStatusBarSpec;
use poodle_specs::{ControlDensity, ControlSize, EyebrowSpec, StatusIndicatorSpec, StatusTone};

/// A branch indicator (info tone dot) + diagnostics indicator (success tone dot),
/// matching the contract §12 "Default" leading region.
fn leading_items(theme: &GpuiThemeProvider) -> CompatRow {
    let branch = StatusIndicatorSpec::new()
        .with_status(StatusTone::Info)
        .with_label("main");
    let diagnostics = StatusIndicatorSpec::new()
        .with_status(StatusTone::Success)
        .with_label("0 errors");
    CompatRow::new()
        .gap(8.0)
        .child(poodle_render::status_indicator(&branch, theme))
        .child(poodle_render::status_indicator(&diagnostics, theme))
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

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let sizes: &[(&str, ControlSize)] = &[
        ("xs", ControlSize::Xs),
        ("sm", ControlSize::Sm),
        ("md", ControlSize::Md),
        ("lg", ControlSize::Lg),
        ("xl", ControlSize::Xl),
    ];
    let densities: &[(&str, ControlDensity)] = &[
        ("compact", ControlDensity::Compact),
        ("default", ControlDensity::Default),
        ("comfortable", ControlDensity::Comfortable),
    ];

    div()
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
        // --- Sizes: font-size + padding-block scale ---
        .child(group(theme, "Sizes", {
            let mut col = div().flex().flex_col().gap(px(12.0));
            for &(key, size) in sizes {
                col = col.child(
                    StatusBar::from_spec(
                        ShellStatusBarSpec::new()
                            .with_summary("Status bar")
                            .with_chrome(true),
                        theme,
                    )
                    .chrome(true)
                    .with_size(size)
                    .with_trailing_items(trailing_meta(&["UTF-8", "TypeScript"])),
                );
                let _ = key;
            }
            col
        }))
        // --- Densities: padding-inline + gap scale (height unchanged) ---
        .child(group(theme, "Densities", {
            let mut col = div().flex().flex_col().gap(px(12.0));
            for &(key, density) in densities {
                col = col.child(
                    StatusBar::from_spec(
                        ShellStatusBarSpec::new()
                            .with_summary("Status bar")
                            .with_chrome(true),
                        theme,
                    )
                    .chrome(true)
                    .with_density(density)
                    .with_trailing_items(trailing_meta(&["UTF-8", "TypeScript"])),
                );
                let _ = key;
            }
            col
        }))
}
