//! ThemeSelect specimen — theme picker with swatch tiles.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use crate::compat::js_theme_select;
use poodle_specs::{ControlDensity, ControlSize, ThemeOption, ThemeSelectSpec, ThemeSwatch};

fn themes() -> Vec<ThemeOption> {
    vec![
        ThemeOption::new(
            "eclipse",
            "Eclipse",
            ThemeSwatch::new("#0e1012", "#15181b", "#f0b24d", "#eef2f6", "#333"),
        ),
        ThemeOption::new(
            "iceberg",
            "Iceberg",
            ThemeSwatch::new("#e7eef5", "#dbe5ef", "#2d86f3", "#131a22", "#75869b"),
        ),
        ThemeOption::new(
            "midnight",
            "Midnight",
            ThemeSwatch::new("#0b1020", "#121933", "#6d8cff", "#e6ecff", "#333"),
        ),
        ThemeOption::new(
            "nord",
            "Nord",
            ThemeSwatch::new("#2e3440", "#3b4252", "#88c0d0", "#eceff4", "#4c566a"),
        ),
        ThemeOption::new(
            "rose",
            "Rose",
            ThemeSwatch::new("#1a1114", "#241a1e", "#f65c8a", "#f6eef1", "#333"),
        ),
        ThemeOption::new(
            "forest",
            "Forest",
            ThemeSwatch::new("#0e1512", "#15201b", "#4dc98a", "#e8f3ec", "#333"),
        ),
    ]
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Disabled",
            secondary,
            js_theme_select(
                &ThemeSelectSpec::new()
                    .with_themes(themes())
                    .with_value("nord")
                    .with_disabled(true),
                theme,
            ),
        ))
        .child(group(
            "Sizes (xs–xl)",
            secondary,
            div().flex_col().gap(8.0).children(
                [
                    ControlSize::Xs,
                    ControlSize::Sm,
                    ControlSize::Md,
                    ControlSize::Lg,
                    ControlSize::Xl,
                ]
                .into_iter()
                .map(|size| {
                    js_theme_select(
                        &ThemeSelectSpec::new()
                            .with_themes(themes())
                            .with_value("eclipse")
                            .with_size(size),
                        theme,
                    )
                }),
            ),
        ))
        .child(group(
            "Densities",
            secondary,
            div().flex_col().gap(8.0).children(
                [
                    ControlDensity::Compact,
                    ControlDensity::Default,
                    ControlDensity::Comfortable,
                ]
                .into_iter()
                .map(|density| {
                    js_theme_select(
                        &ThemeSelectSpec::new()
                            .with_themes(themes())
                            .with_value("eclipse")
                            .with_density(density),
                        theme,
                    )
                }),
            ),
        ))
        // Last on purpose: the popover is anchored and absolutely positioned,
        // so it overlays whatever follows it. Jetstream specimens are pure
        // functions of the theme with no host state, so this one cannot be
        // driven open by a click the way the GPUI specimen now is.
        .child(group(
            "Theme picker (open)",
            secondary,
            js_theme_select(
                &ThemeSelectSpec::new()
                    .with_themes(themes())
                    .with_value("midnight")
                    .with_open(true),
                theme,
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
