use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::ThemeSelect;
use poodle_specs::{ThemeOption, ThemeSelectSpec, ThemeSwatch};

fn demo_themes() -> Vec<ThemeOption> {
    vec![
        ThemeOption::new("dark", "Dark", ThemeSwatch::new("#0e1012", "#15181b", "#f0b24d", "#eef2f6", "#333")),
        ThemeOption::new("light", "Light", ThemeSwatch::new("#e7eef5", "#dbe5ef", "#2d86f3", "#131a22", "#75869b")),
        ThemeOption::new("midnight", "Midnight", ThemeSwatch::new("#0b1020", "#121933", "#6d8cff", "#e6ecff", "#333")),
        ThemeOption::new("nord", "Nord", ThemeSwatch::new("#2e3440", "#3b4252", "#88c0d0", "#eceff4", "#4c566a")),
        ThemeOption::new("rose", "Rose", ThemeSwatch::new("#1a1114", "#241a1e", "#f65c8a", "#f6eef1", "#333")),
        ThemeOption::new("forest", "Forest", ThemeSwatch::new("#0e1512", "#15201b", "#4dc98a", "#e8f3ec", "#333")),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(ThemeSelect::from_spec(
            ThemeSelectSpec::new()
                .with_themes(demo_themes())
                .with_value("midnight")
                .with_open(true),
            theme,
        ))
        .child(ThemeSelect::from_spec(
            ThemeSelectSpec::new()
                .with_themes(demo_themes())
                .with_value("nord")
                .with_disabled(true),
            theme,
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "theme-select",
        examples,
        |size, theme: &GpuiThemeProvider| {
            ThemeSelect::from_spec(
                ThemeSelectSpec::new().with_themes(demo_themes()).with_value("dark"),
                theme,
            )
            .size(size)
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            ThemeSelect::from_spec(
                ThemeSelectSpec::new().with_themes(demo_themes()).with_value("dark"),
                theme,
            )
            .with_density(density)
            .into_any_element()
        },
    )
}
