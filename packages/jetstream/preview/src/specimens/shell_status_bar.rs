//! StatusBar specimen — bottom status row with leading/trailing slots.
//!
//! Leading region uses real `js_status_indicator` tone dots (branch + error
//! count, contract §12). Trailing metadata uses `label()` sized from the spec's
//! contract `font_size_rem()` (Svelte `font-size: inherit`) — no raw literals.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use crate::compat::js_shell_status_bar;
use crate::compat::js_status_indicator;

use poodle_specs::{
    ControlDensity, ControlSize, ShellStatusBarSpec, StatusIndicatorSpec, StatusTone,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Leading region: branch (info dot) + diagnostics (success dot), real
    // StatusIndicators (contract §12 "Default" leading region).
    let leading = |theme: &JetstreamThemeProvider| {
        vec![
            js_status_indicator(
                &StatusIndicatorSpec::new()
                    .with_status(StatusTone::Info)
                    .with_label("main"),
                theme,
            ),
            js_status_indicator(
                &StatusIndicatorSpec::new()
                    .with_status(StatusTone::Success)
                    .with_label("0 errors"),
                theme,
            ),
        ]
    };

    // Trailing metadata sized from the bar's resolved font (spec.font_size_rem),
    // matching the Svelte `font-size: inherit` on `.poodle-status-item`.
    let trailing = |spec: &ShellStatusBarSpec, items: &[&str]| -> Vec<El> {
        let font = rem_to_px(spec.font_size_rem());
        items
            .iter()
            .map(|t| label(*t).text_color(secondary).text_size(font))
            .collect()
    };

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

    let mut root = div().flex_col().gap(24.0);

    // Default (no chrome): blends into container.
    let default_spec = ShellStatusBarSpec::new().with_summary("Ready");
    root = root.child(group(
        "Default (no chrome)",
        secondary,
        js_shell_status_bar(
            &default_spec,
            theme,
            leading(theme),
            trailing(&default_spec, &["Ln 42, Col 18", "UTF-8", "TypeScript"]),
        ),
    ));

    // With chrome: 94% panel bg + border-top.
    let chrome_spec = ShellStatusBarSpec::new()
        .with_summary("Ready")
        .with_chrome(true);
    root = root.child(group(
        "With chrome",
        secondary,
        js_shell_status_bar(
            &chrome_spec,
            theme,
            leading(theme),
            trailing(&chrome_spec, &["Ln 42, Col 18", "UTF-8", "TypeScript"]),
        ),
    ));

    // Summary only: leading shows summary text, no trailing region.
    root = root.child(group(
        "Summary only",
        secondary,
        js_shell_status_bar(
            &ShellStatusBarSpec::new().with_summary("3 items selected"),
            theme,
            vec![],
            vec![],
        ),
    ));

    // Sizes: font-size + padding-block scale.
    let mut size_col = div().flex_col().gap(12.0);
    for &(_, size) in sizes {
        let spec = ShellStatusBarSpec::new()
            .with_summary("Status bar")
            .with_chrome(true)
            .with_size(size);
        size_col = size_col.child(js_shell_status_bar(
            &spec,
            theme,
            vec![],
            trailing(&spec, &["UTF-8", "TypeScript"]),
        ));
    }
    root = root.child(group("Sizes", secondary, size_col));

    // Densities: padding-inline + gap scale (height unchanged).
    let mut density_col = div().flex_col().gap(12.0);
    for &(_, density) in densities {
        let spec = ShellStatusBarSpec::new()
            .with_summary("Status bar")
            .with_chrome(true)
            .with_density(density);
        density_col = density_col.child(js_shell_status_bar(
            &spec,
            theme,
            vec![],
            trailing(&spec, &["UTF-8", "TypeScript"]),
        ));
    }
    root = root.child(group("Densities", secondary, density_col));

    root
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
