use gpui::*;
use flint_primitives::{ButtonSpec, ButtonTone, ButtonVariant, DialogKind, DialogSpec, AlertDialogSpec, AlertDialogTone, EyebrowSpec};
use flint_gpui_components::{Button, Dialog, AlertDialog, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let basic_open = state.specimens.is_on("dialog-basic-open");
    let alert_open = state.specimens.is_on("dialog-alert-open");
    let persistent_open = state.specimens.is_on("dialog-persistent-open");

    div().flex().flex_col().gap(px(24.0))
        // --- Basic dialog ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic dialog"), theme))
                .child({
                    let mut col = div().flex().flex_col().gap(px(8.0));

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
        )
        // --- Alert dialog ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Alert dialog"), theme))
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
        )
        // --- No backdrop dismiss ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("No backdrop dismiss"), theme))
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
        )
        // --- AlertDialog: Danger tone ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Alert dialog: danger tone"), theme))
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
        )
        // --- AlertDialog: Warning tone ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Alert dialog: warning tone"), theme))
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
        )
}
