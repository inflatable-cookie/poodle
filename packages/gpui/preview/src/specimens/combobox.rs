use gpui::*;
use gpui::prelude::FluentBuilder;
use pug_adapter::ThemeProvider;
use pug_primitives::{ComboboxSpec, ComboboxOption};
use pug_gpui_components::Combobox;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let framework_options = vec![
        ComboboxOption::new("react", "React"),
        ComboboxOption::new("vue", "Vue"),
        ComboboxOption::new("angular", "Angular"),
        ComboboxOption::new("svelte", "Svelte"),
        ComboboxOption::new("solid", "Solid"),
        ComboboxOption::new("preact", "Preact"),
    ];

    let db_options = vec![
        ComboboxOption::new("postgres", "PostgreSQL")
            .with_description("Advanced open-source relational database"),
        ComboboxOption::new("mongo", "MongoDB")
            .with_description("Document-oriented NoSQL database"),
        ComboboxOption::new("redis", "Redis")
            .with_description("In-memory key-value data store"),
        ComboboxOption::new("sqlite", "SQLite")
            .with_description("Lightweight embedded SQL engine"),
    ];

    let selected_framework = state.specimens.text.get("combobox-framework").cloned();
    let selected_db = state.specimens.text.get("combobox-db").cloned();

    div().flex().flex_col().gap(px(16.0)).max_w(px(320.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child({
                    let mut spec = ComboboxSpec::new()
                        .with_options(framework_options.clone())
                        .with_placeholder("Select a framework…");
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
        // --- With descriptions ---
        .child(section_label("WITH DESCRIPTIONS", text_secondary))
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child({
                    let mut spec = ComboboxSpec::new()
                        .with_options(db_options)
                        .with_placeholder("Select a database…");
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
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            Combobox::from_spec(
                ComboboxSpec::new()
                    .with_options(framework_options)
                    .with_value("svelte")
                    .with_disabled(true),
                theme,
            ).with_id("disabled")
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
