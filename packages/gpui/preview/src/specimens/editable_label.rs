use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{EditableLabelSpec, EyebrowSpec};
use poodle_gpui_components::{EditableLabel, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let label_value = state.specimens.text.get("editable-label-value").cloned()
        .unwrap_or_else(|| "Untitled project".to_string());
    let label_editing = state.specimens.is_on("editable-label-editing");

    let rename_value = state.specimens.text.get("editable-label-rename").cloned()
        .unwrap_or_else(|| "Click to rename".to_string());
    let rename_editing = state.specimens.is_on("editable-label-rename-editing");

    let last_event = state.specimens.text.get("editable-label-event").cloned();

    div().flex().flex_col().gap(px(24.0)).max_w(px(384.0))
        // --- Double-click to edit (default) ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Double-click to edit (default)"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value(&label_value)
                            .with_editing(label_editing),
                        theme,
                    )
                    .with_id("default")
                    .on_change(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("editable-label-value".to_string(), val.to_string());
                        cx.notify();
                    }))
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("editable-label-value".to_string(), val.to_string());
                        this.state.specimens.toggles.insert("editable-label-editing".to_string(), false);
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
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Click to edit with icon"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value(&rename_value)
                            .with_editing(rename_editing),
                        theme,
                    )
                    .with_id("with-icon")
                    .on_change(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("editable-label-rename".to_string(), val.to_string());
                        cx.notify();
                    }))
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert("editable-label-rename".to_string(), val.to_string());
                        this.state.specimens.toggles.insert("editable-label-rename-editing".to_string(), false);
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
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Empty state"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_placeholder("Add a description\u{2026}"),
                        theme,
                    )
                    .with_id("empty")
                    .on_commit(cx.listener(|this, val: &str, _w, cx| {
                        this.state.specimens.text.insert(
                            "editable-label-event".to_string(),
                            format!("Committed empty-state: \"{}\"", val),
                        );
                        cx.notify();
                    }))
                )
        )

        // --- Flush variant ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Flush variant"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value("Currently editing this label")
                            .with_editing(true),
                        theme,
                    )
                    .with_id("prefilled")
                )
        )

        // --- With max length ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With max length"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value("Locked label")
                            .with_disabled(true),
                        theme,
                    )
                    .with_id("disabled")
                )
        )

        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    EditableLabel::from_spec(
                        EditableLabelSpec::new()
                            .with_value("Locked label")
                            .with_disabled(true),
                        theme,
                    )
                    .with_id("disabled2")
                )
        )

        // --- Last event ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Last event"), theme))
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child(match last_event {
                            Some(ref evt) => evt.clone(),
                            None => "No events yet \u{2014} edit a label above.".to_string(),
                        })
                )
        )
}
