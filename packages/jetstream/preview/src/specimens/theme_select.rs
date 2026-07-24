//! ThemeSelect specimen — theme picker with swatch tiles.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::theme_ext::*;
use poodle_jetstream_components::theme_select::js_theme_select;
use poodle_specs::{ControlDensity, ControlSize, ThemeOption, ThemeSelectSpec, ThemeSwatch};

fn themes() -> Vec<ThemeOption> {
    vec![
        ThemeOption::new("dark", "Dark", ThemeSwatch::new("#0e1012", "#15181b", "#f0b24d", "#eef2f6", "#333")),
        ThemeOption::new("light", "Light", ThemeSwatch::new("#e7eef5", "#dbe5ef", "#2d86f3", "#131a22", "#75869b")),
        ThemeOption::new("midnight", "Midnight", ThemeSwatch::new("#0b1020", "#121933", "#6d8cff", "#e6ecff", "#333")),
        ThemeOption::new("nord", "Nord", ThemeSwatch::new("#2e3440", "#3b4252", "#88c0d0", "#eceff4", "#4c566a")),
        ThemeOption::new("rose", "Rose", ThemeSwatch::new("#1a1114", "#241a1e", "#f65c8a", "#f6eef1", "#333")),
        ThemeOption::new("forest", "Forest", ThemeSwatch::new("#0e1512", "#15201b", "#4dc98a", "#e8f3ec", "#333")),
    ]
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Theme picker (open)",
            secondary,
            js_theme_select(
                &ThemeSelectSpec::new().with_themes(themes()).with_value("midnight").with_open(true),
                theme,
            ),
        ))
        .child(group(
            "Disabled",
            secondary,
            js_theme_select(
                &ThemeSelectSpec::new().with_themes(themes()).with_value("nord").with_disabled(true),
                theme,
            ),
        ))
        .child(group(
            "Sizes (xs–xl)",
            secondary,
            div().flex_col().gap(8.0).children(
                [ControlSize::Xs, ControlSize::Sm, ControlSize::Md, ControlSize::Lg, ControlSize::Xl]
                    .into_iter()
                    .map(|size| {
                        js_theme_select(
                            &ThemeSelectSpec::new().with_themes(themes()).with_value("dark").with_size(size),
                            theme,
                        )
                    }),
            ),
        ))
        .child(group(
            "Densities",
            secondary,
            div().flex_col().gap(8.0).children(
                [ControlDensity::Compact, ControlDensity::Default, ControlDensity::Comfortable]
                    .into_iter()
                    .map(|density| {
                        js_theme_select(
                            &ThemeSelectSpec::new().with_themes(themes()).with_value("dark").with_density(density),
                            theme,
                        )
                    }),
            ),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
