use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{EditableLabelSpec, EyebrowSpec};
use poodle_gpui_components::{EditableLabel, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let title_value = state.specimens.text.get("editable-label-title").cloned()
        .unwrap_or_else(|| "My project title".to_string());

    let empty_value = state.specimens.text.get("editable-label-empty").cloned()
        .unwrap_or_default();

    let flush_value = state.specimens.text.get("editable-label-flush").cloned()
        .unwrap_or_else(|| "Inline heading".to_string());

    let last_event = state.specimens.text.get("editable-label-event").cloned();

    div().flex().flex_col().gap(px(24.0)).max_w(px(384.0))
        // --- Double-click to edit (default) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Double-click to edit (default)"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value(&title_value),
                        theme,
                    )
                    .with_id("default")
                    .on_change(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("editable-label-title".to_string(), val.to_string());
                        cx.notify();
                    }))
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("editable-label-title".to_string(), val.to_string());
                        this.state.specimens.text.insert(
                            "editable-label-event".to_string(),
                            format!("Committed: \"{}\"", val),
                        );
                        cx.notify();
                    }))
                )
        )

        // --- Click to edit with icon ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Click to edit with icon"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value(&title_value),
                        theme,
                    )
                    .with_id("with-icon")
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("editable-label-title".to_string(), val.to_string());
                        this.state.specimens.text.insert(
                            "editable-label-event".to_string(),
                            format!("Committed: \"{}\"", val),
                        );
                        cx.notify();
                    }))
                )
        )

        // --- Empty state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Empty state"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value(&empty_value)
                            .with_placeholder("Add a description\u{2026}"),
                        theme,
                    )
                    .with_id("empty")
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("editable-label-empty".to_string(), val.to_string());
                        this.state.specimens.text.insert(
                            "editable-label-event".to_string(),
                            format!("Committed: \"{}\"", val),
                        );
                        cx.notify();
                    }))
                )
        )

        // --- Flush variant ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Flush variant"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value(&flush_value),
                        theme,
                    )
                    .with_id("flush")
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("editable-label-flush".to_string(), val.to_string());
                        cx.notify();
                    }))
                )
        )

        // --- With max length ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With max length"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value("Short text"),
                        theme,
                    )
                    .with_id("max-length")
                )
        )

        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value("Read-only value")
                            .with_disabled(true),
                        theme,
                    )
                    .with_id("disabled")
                )
        )

        // --- Last event ---
        .when(last_event.is_some(), |el| {
            el.child(
                div().flex().flex_col().gap(px(8.0))
                    .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Last event"), theme))
                    .child(
                        div().text_sm().text_color(color_to_hsla(text_secondary))
                            .child(last_event.unwrap_or_default())
                    )
            )
        })
}
