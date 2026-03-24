use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{ComboboxSpec, ComboboxOption, EyebrowSpec};
use poodle_gpui_components::{Combobox, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let framework_options = vec![
        ComboboxOption::new("svelte", "Svelte"),
        ComboboxOption::new("react", "React"),
        ComboboxOption::new("vue", "Vue"),
        ComboboxOption::new("angular", "Angular"),
        ComboboxOption::new("solid", "SolidJS"),
        ComboboxOption::new("lit", "Lit"),
    ];

    let db_options = vec![
        ComboboxOption::new("postgres", "PostgreSQL")
            .with_description("Relational database"),
        ComboboxOption::new("mongo", "MongoDB")
            .with_description("Document database"),
        ComboboxOption::new("redis", "Redis")
            .with_description("In-memory key-value store"),
        ComboboxOption::new("sqlite", "SQLite")
            .with_description("Embedded relational database"),
    ];

    let selected_framework = state.specimens.text.get("combobox-framework").cloned();
    let selected_db = state.specimens.text.get("combobox-db").cloned();

    div().flex().flex_col().gap(px(24.0)).max_w(px(384.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child({
                            let mut spec = ComboboxSpec::new()
                                .with_options(framework_options.clone())
                                .with_placeholder("Choose a framework...");
                            if let Some(ref val) = selected_framework {
                                spec = spec.with_value(val);
                            }
                            Combobox::from_spec(spec, theme)
                                .with_id("framework")
                                .on_change(cx.listener(|this, val: &str, _w, cx| {
                                    this.state.specimens.text.insert(
                                        "combobox-framework".to_string(),
                                        val.to_string(),
                                    );
                                    cx.notify();
                                }))
                        })
                        .when(selected_framework.is_some(), |d| {
                            d.child(
                                div().text_sm().text_color(color_to_hsla(text_secondary))
                                    .child(format!("Selected: {}", selected_framework.as_deref().unwrap_or("")))
                            )
                        })
                )
        )
        // --- With descriptions ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With descriptions"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child({
                            let mut spec = ComboboxSpec::new()
                                .with_options(db_options)
                                .with_placeholder("Choose a database...");
                            if let Some(ref val) = selected_db {
                                spec = spec.with_value(val);
                            }
                            Combobox::from_spec(spec, theme)
                                .with_id("database")
                                .on_change(cx.listener(|this, val: &str, _w, cx| {
                                    this.state.specimens.text.insert(
                                        "combobox-db".to_string(),
                                        val.to_string(),
                                    );
                                    cx.notify();
                                }))
                        })
                        .when(selected_db.is_some(), |d| {
                            d.child(
                                div().text_sm().text_color(color_to_hsla(text_secondary))
                                    .child(format!("Selected: {}", selected_db.as_deref().unwrap_or("")))
                            )
                        })
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    Combobox::from_spec(
                        ComboboxSpec::new()
                            .with_options(framework_options)
                            .with_placeholder("Disabled")
                            .with_disabled(true),
                        theme,
                    ).with_id("disabled")
                )
        )
}
