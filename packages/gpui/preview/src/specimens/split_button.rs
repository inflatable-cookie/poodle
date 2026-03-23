use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{ButtonVariant, ButtonTone, SplitButtonSpec, SplitMenuItem, EyebrowSpec};
use poodle_gpui_components::{SplitButton, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let last_action = state.specimens.text.get("split-btn-action")
        .cloned()
        .unwrap_or_else(|| String::from("(none)"));

    div().flex().flex_col().gap(px(24.0))
        // --- Primary variant ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Primary variant"), theme))
                .child(
                    SplitButton::from_spec(
                        SplitButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label("Save")
                            .with_items(vec![
                                SplitMenuItem::action("save-draft", "Save as draft"),
                                SplitMenuItem::action("save-template", "Save as template"),
                                SplitMenuItem::Separator,
                                SplitMenuItem::action("discard", "Discard changes"),
                            ]),
                        theme,
                    )
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.text.insert(
                            "split-btn-action".to_string(),
                            "click: Save".to_string(),
                        );
                        cx.notify();
                    }))
                    .on_dropdown(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.text.insert(
                            "split-btn-action".to_string(),
                            "dropdown: toggle".to_string(),
                        );
                        cx.notify();
                    }))
                )
        )
        // --- Secondary variant ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Secondary variant"), theme))
                .child(
                    SplitButton::from_spec(
                        SplitButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label("Export")
                            .with_items(vec![
                                SplitMenuItem::action("csv", "Export as CSV"),
                                SplitMenuItem::action("json", "Export as JSON"),
                                SplitMenuItem::action("pdf", "Export as PDF"),
                            ]),
                        theme,
                    )
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.text.insert(
                            "split-btn-action".to_string(),
                            "click: Export".to_string(),
                        );
                        cx.notify();
                    }))
                    .on_dropdown(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.text.insert(
                            "split-btn-action".to_string(),
                            "dropdown: toggle".to_string(),
                        );
                        cx.notify();
                    }))
                )
        )
        // --- Danger tone ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Danger tone"), theme))
                .child(
                    SplitButton::from_spec(
                        SplitButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_tone(ButtonTone::Danger)
                            .with_label("Delete")
                            .with_items(vec![
                                SplitMenuItem::action("delete-selected", "Delete selected"),
                                SplitMenuItem::action("delete-all", "Delete all"),
                            ]),
                        theme,
                    )
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.text.insert(
                            "split-btn-action".to_string(),
                            "click: Delete".to_string(),
                        );
                        cx.notify();
                    }))
                )
        )
        // --- Loading state ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Loading state"), theme))
                .child(
                    SplitButton::from_spec(
                        SplitButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label("Saving...")
                            .with_loading(true),
                        theme,
                    )
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    SplitButton::from_spec(
                        SplitButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label("Save")
                            .with_disabled(true),
                        theme,
                    )
                )
        )
        // --- Last action ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Last action"), theme))
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child(format!("Last action: {}", last_action))
                )
        )
}
