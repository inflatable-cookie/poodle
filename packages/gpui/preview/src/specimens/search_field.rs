use gpui::*;
use gpui::prelude::FluentBuilder;
use flint_adapter::ThemeProvider;
use flint_primitives::{SearchFieldSpec, EyebrowSpec};
use flint_gpui_components::{SearchField, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let query = state.specimens.text.get("search-query").cloned()
        .unwrap_or_default();
    let last_submit = state.specimens.text.get("search-submit").cloned()
        .unwrap_or_default();

    div().flex().flex_col().gap(px(24.0)).max_w(px(384.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            SearchField::from_spec(
                                SearchFieldSpec::new()
                                    .with_placeholder("Search components...")
                                    .with_value(&query)
                                    .with_aria_label("Search components"),
                                theme,
                            )
                            .with_id("sf-default")
                            .on_change(cx.listener(|this, val: &str, _w, cx| {
                                this.state.specimens.text.insert("search-query".to_string(), val.to_string());
                                cx.notify();
                            }))
                            .on_submit(cx.listener(|this, val: &str, _w, cx| {
                                this.state.specimens.text.insert("search-submit".to_string(), val.to_string());
                                cx.notify();
                            }))
                        )
                        .when(!query.is_empty(), |d| {
                            d.child(
                                div().text_sm()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!("Filtering: {}", query))
                            )
                        })
                        .when(!last_submit.is_empty(), |d| {
                            d.child(
                                div().text_sm()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!("Submitted: {}", last_submit))
                            )
                        })
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    SearchField::from_spec(
                        SearchFieldSpec::new()
                            .with_value("locked query")
                            .with_disabled(true)
                            .with_aria_label("Disabled search"),
                        theme,
                    )
                    .with_id("sf-disabled")
                )
        )
        // --- Read-only ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Read-only"), theme))
                .child(
                    SearchField::from_spec(
                        SearchFieldSpec::new()
                            .with_value("active filter")
                            .with_read_only(true)
                            .with_aria_label("Read-only search"),
                        theme,
                    )
                    .with_id("sf-readonly")
                )
        )
}
