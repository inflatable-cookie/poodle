use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{AlertDialog, Button, Eyebrow};
use poodle_specs::{
    AlertDialogSpec, AlertDialogTone, ButtonSpec, ButtonTone, ButtonVariant, EyebrowSpec,
};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let danger_open = state.specimens.is_on("alert-danger-open");
    let warning_open = state.specimens.is_on("alert-warning-open");
    let last_action = state
        .specimens
        .text
        .get("alert-last-action")
        .cloned()
        .unwrap_or_default();

    div().flex().flex_col().gap(px(24.0))
        // --- Danger tone (default) — interactive trigger ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Danger tone (default)"), theme))
                .child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_tone(ButtonTone::Danger)
                            .with_label("Delete item"),
                        theme,
                    )
                    .with_id("alert-danger-trigger")
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.toggles.insert("alert-danger-open".to_string(), true);
                        cx.notify();
                    }))
                )
                .when(danger_open, |el| {
                    el.child(
                        AlertDialog::from_spec(
                            AlertDialogSpec::new("Delete this item?")
                                .with_description("This action cannot be undone. The item and all associated data will be permanently removed.")
                                .with_confirm_label("Delete")
                                .with_cancel_label("Keep it"),
                            theme,
                        )
                        .open(true)
                    )
                })
        )
        // --- Warning tone — interactive trigger ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Warning tone"), theme))
                .child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label("Reset settings"),
                        theme,
                    )
                    .with_id("alert-warning-trigger")
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.toggles.insert("alert-warning-open".to_string(), true);
                        cx.notify();
                    }))
                )
                .when(warning_open, |el| {
                    el.child(
                        AlertDialog::from_spec(
                            AlertDialogSpec::new("Reset all settings?")
                                .with_description("Your customized settings will be restored to their default values.")
                                .with_tone(AlertDialogTone::Warning)
                                .with_confirm_label("Reset")
                                .with_cancel_label("Cancel"),
                            theme,
                        )
                        .open(true)
                    )
                })
        )
        // --- Working / pending state (always shown: label swap + disabled buttons) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Working / pending state"), theme))
                .child(
                    AlertDialog::from_spec(
                        AlertDialogSpec::new("Archive this project?")
                            .with_description("The project will be hidden from active lists but can still be restored later.")
                            .with_confirm_label("Archive")
                            .with_cancel_label("Cancel"),
                        theme,
                    )
                    .open(true)
                    .working(true)
                    .working_label("Archiving\u{2026}")
                )
        )
        // --- With item-detail row (always shown) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With item-detail row"), theme))
                .child(
                    AlertDialog::from_spec(
                        AlertDialogSpec::new("Remove this user?")
                            .with_description("The following user will lose access immediately.")
                            .with_confirm_label("Remove")
                            .with_cancel_label("Cancel"),
                        theme,
                    )
                    .open(true)
                    .item_detail("User", "Ada Lovelace · ada@example.com")
                )
        )
        // --- Last action ---
        .when(!last_action.is_empty(), |el| {
            el.child(
                div().flex().flex_col().gap(px(8.0))
                    .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Last action"), theme))
                    .child(
                        div().text_sm().text_color(color_to_hsla(text_secondary))
                            .child(last_action)
                    )
            )
        })
}
