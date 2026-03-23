use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{ButtonSpec, ButtonTone, ButtonVariant, DialogKind, DialogSpec, AlertDialogSpec, AlertDialogTone};
use pug_gpui_components::{Button, Dialog, AlertDialog};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let basic_open = state.specimens.is_on("dialog-basic-open");
    let alert_open = state.specimens.is_on("dialog-alert-open");
    let persistent_open = state.specimens.is_on("dialog-persistent-open");

    div().flex().flex_col().gap(px(16.0))
        // --- Basic dialog ---
        .child(section_label("BASIC DIALOG", text_secondary))
        .child({
            let mut col = div().flex().flex_col().gap(px(8.0));

            // Trigger button
            col = col.child(
                Button::from_spec(
                    ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Open dialog"),
                    theme,
                )
                .with_id("dialog-basic-trigger")
                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                    this.state.specimens.toggles.insert("dialog-basic-open".to_string(), true);
                    cx.notify();
                }))
            );

            // Dialog (shown when open)
            if basic_open {
                let spec = DialogSpec::new()
                    .with_title("Confirm action")
                    .with_description("Are you sure you want to proceed? This action cannot be undone.");

                let actions = div().flex().gap(px(8.0))
                    .child(
                        Button::from_spec(
                            ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Cancel"),
                            theme,
                        )
                        .with_id("dialog-basic-cancel")
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.toggles.insert("dialog-basic-open".to_string(), false);
                            cx.notify();
                        }))
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Confirm"),
                            theme,
                        )
                        .with_id("dialog-basic-confirm")
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.toggles.insert("dialog-basic-open".to_string(), false);
                            cx.notify();
                        }))
                    );

                col = col.child(
                    Dialog::from_spec(spec, theme)
                        .with_actions(actions)
                );
            }

            col
        })
        // --- Alert dialog ---
        .child(section_label("ALERT DIALOG", text_secondary))
        .child({
            let mut col = div().flex().flex_col().gap(px(8.0));

            col = col.child(
                Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_tone(ButtonTone::Danger)
                        .with_label("Delete item"),
                    theme,
                )
                .with_id("dialog-alert-trigger")
                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                    this.state.specimens.toggles.insert("dialog-alert-open".to_string(), true);
                    cx.notify();
                }))
            );

            if alert_open {
                let spec = DialogSpec::new()
                    .with_title("Delete item?")
                    .with_description("This will permanently remove the item and all associated data.")
                    .with_kind(DialogKind::AlertDialog);

                let actions = div().flex().gap(px(8.0))
                    .child(
                        Button::from_spec(
                            ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Cancel"),
                            theme,
                        )
                        .with_id("dialog-alert-cancel")
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.toggles.insert("dialog-alert-open".to_string(), false);
                            cx.notify();
                        }))
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_tone(ButtonTone::Danger)
                                .with_label("Delete"),
                            theme,
                        )
                        .with_id("dialog-alert-delete")
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.toggles.insert("dialog-alert-open".to_string(), false);
                            cx.notify();
                        }))
                    );

                col = col.child(
                    Dialog::from_spec(spec, theme)
                        .with_actions(actions)
                );
            }

            col
        })
        // --- No backdrop dismiss ---
        .child(section_label("NO BACKDROP DISMISS", text_secondary))
        .child({
            let mut col = div().flex().flex_col().gap(px(8.0));

            col = col.child(
                Button::from_spec(
                    ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Open persistent dialog"),
                    theme,
                )
                .with_id("dialog-persistent-trigger")
                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                    this.state.specimens.toggles.insert("dialog-persistent-open".to_string(), true);
                    cx.notify();
                }))
            );

            if persistent_open {
                let spec = DialogSpec::new()
                    .with_title("Persistent dialog")
                    .with_description("This dialog can only be closed via the buttons or Escape key.")
                    .with_dismiss_on_backdrop(false);

                let actions = div().flex().gap(px(8.0))
                    .child(
                        Button::from_spec(
                            ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Got it"),
                            theme,
                        )
                        .with_id("dialog-persistent-gotit")
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.toggles.insert("dialog-persistent-open".to_string(), false);
                            cx.notify();
                        }))
                    );

                col = col.child(
                    Dialog::from_spec(spec, theme)
                        .with_actions(actions)
                );
            }

            col
        })

        // --- AlertDialog: Danger tone ---
        .child(section_label("ALERT DIALOG: DANGER TONE", text_secondary))
        .child(
            AlertDialog::from_spec(
                AlertDialogSpec::new("Delete this item?")
                    .with_description("This action cannot be undone. The item and all associated data will be permanently removed.")
                    .with_confirm_label("Delete")
                    .with_cancel_label("Keep it"),
                theme,
            )
            .open(true)
        )

        // --- AlertDialog: Warning tone ---
        .child(section_label("ALERT DIALOG: WARNING TONE", text_secondary))
        .child(
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
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
