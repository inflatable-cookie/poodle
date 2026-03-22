use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{AlertDialogSpec, AlertDialogTone};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub struct AlertDialog {
    spec: AlertDialogSpec,
    backdrop_fill: Hsla,
    dialog_fill: Hsla,
    dialog_radius: Pixels,
    button_radius: Pixels,
    title_color: Hsla,
    description_color: Hsla,
    confirm_fill: Hsla,
    confirm_text_color: Hsla,
    cancel_text_color: Hsla,
    content_gap: Pixels,
    actions_gap: Pixels,
    padding_x: Pixels,
    padding_y: Pixels,
    border_color: Hsla,
    on_confirm: Option<Box<dyn Fn(&mut Window, &mut App)>>,
    on_cancel: Option<Box<dyn Fn(&mut Window, &mut App)>>,
}

impl std::ops::Deref for AlertDialog {
    type Target = AlertDialogSpec;
    fn deref(&self) -> &AlertDialogSpec { &self.spec }
}

impl AlertDialog {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        let spec = AlertDialogSpec::default();
        Self {
            backdrop_fill: resolve_color(theme, spec.backdrop_fill_token()),
            dialog_fill: resolve_color(theme, spec.dialog_fill_token()),
            dialog_radius: resolve_radius(theme, spec.dialog_radius_token()),
            button_radius: resolve_radius(theme, spec.button_radius_token()),
            title_color: resolve_color(theme, spec.title_color_token()),
            description_color: resolve_color(theme, spec.description_color_token()),
            confirm_fill: resolve_color(theme, spec.confirm_fill_token()),
            confirm_text_color: resolve_color(theme, spec.confirm_text_color_token()),
            cancel_text_color: resolve_color(theme, spec.cancel_text_color_token()),
            content_gap: resolve_px(theme, spec.content_gap_token()),
            actions_gap: resolve_px(theme, spec.actions_gap_token()),
            padding_x: resolve_px(theme, spec.padding_x_token()),
            padding_y: resolve_px(theme, spec.padding_y_token()),
            border_color: resolve_color(theme, spec.border_token()),
            on_confirm: None,
            on_cancel: None,
            spec,
        }
    }

    pub fn from_spec(spec: AlertDialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            backdrop_fill: resolve_color(theme, spec.backdrop_fill_token()),
            dialog_fill: resolve_color(theme, spec.dialog_fill_token()),
            dialog_radius: resolve_radius(theme, spec.dialog_radius_token()),
            button_radius: resolve_radius(theme, spec.button_radius_token()),
            title_color: resolve_color(theme, spec.title_color_token()),
            description_color: resolve_color(theme, spec.description_color_token()),
            confirm_fill: resolve_color(theme, spec.confirm_fill_token()),
            confirm_text_color: resolve_color(theme, spec.confirm_text_color_token()),
            cancel_text_color: resolve_color(theme, spec.cancel_text_color_token()),
            content_gap: resolve_px(theme, spec.content_gap_token()),
            actions_gap: resolve_px(theme, spec.actions_gap_token()),
            padding_x: resolve_px(theme, spec.padding_x_token()),
            padding_y: resolve_px(theme, spec.padding_y_token()),
            border_color: resolve_color(theme, spec.border_token()),
            on_confirm: None,
            on_cancel: None,
            spec,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = v.into(); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn tone(mut self, v: AlertDialogTone) -> Self { self.spec.tone = v; self }
    pub fn confirm_label(mut self, v: impl Into<String>) -> Self { self.spec.confirm_label = v.into(); self }
    pub fn cancel_label(mut self, v: impl Into<String>) -> Self { self.spec.cancel_label = v.into(); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn on_confirm(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_confirm = Some(Box::new(handler));
        self
    }

    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
}

impl IntoElement for AlertDialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let is_open = self.spec.open.unwrap_or(false);

        if !is_open {
            return div().into_any_element();
        }

        // Header
        let mut header = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_color(self.title_color)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.spec.title.clone()),
            );
        if let Some(ref desc) = self.spec.description {
            header = header.child(
                div()
                    .text_color(self.description_color)
                    .child(desc.clone()),
            );
        }

        // Cancel button
        let mut cancel_btn = div()
            .id("alert-dialog-cancel")
            .cursor_pointer()
            .px(self.padding_x)
            .py(px(6.0))
            .rounded(self.button_radius)
            .text_color(self.cancel_text_color)
            .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.1)))
            .child(self.spec.cancel_label.clone());
        if let Some(handler) = self.on_cancel {
            let handler = std::rc::Rc::new(handler);
            let esc_handler = handler.clone();
            cancel_btn = cancel_btn.on_click(move |_, window, app| {
                handler(window, app);
            });

            // Confirm button
            let mut confirm_btn = div()
                .id("alert-dialog-confirm")
                .cursor_pointer()
                .px(self.padding_x)
                .py(px(6.0))
                .rounded(self.button_radius)
                .bg(self.confirm_fill)
                .text_color(self.confirm_text_color)
                .hover(|s| s.opacity(0.9))
                .child(self.spec.confirm_label.clone());
            if let Some(handler) = self.on_confirm {
                confirm_btn = confirm_btn.on_click(move |_, window, app| {
                    handler(window, app);
                });
            }

            let actions = div()
                .flex()
                .flex_row()
                .justify_end()
                .gap(self.actions_gap)
                .child(cancel_btn)
                .child(confirm_btn);

            // Dialog card
            let mut dialog_card = div()
                .id("alert-dialog")
                .focusable()
                .flex()
                .flex_col()
                .bg(self.dialog_fill)
                .rounded(self.dialog_radius)
                .border_1()
                .border_color(self.border_color)
                .shadow(vec![
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.12),
                        offset: point(px(0.0), px(8.0)),
                        blur_radius: px(24.0),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.08),
                        offset: point(px(0.0), px(2.0)),
                        blur_radius: px(8.0),
                        spread_radius: px(0.0),
                    },
                ])
                .px(self.padding_x)
                .py(self.padding_y)
                .gap(self.content_gap)
                .max_w(px(480.0))
                .min_w(px(320.0))
                .occlude()
                .child(header)
                .child(actions);

            // Escape key to cancel
            dialog_card = dialog_card.on_key_down(move |event: &KeyDownEvent, window, cx| {
                if event.keystroke.key == "escape" {
                    esc_handler(window, cx);
                }
            });

            // Backdrop
            div()
                .id("alert-dialog-backdrop")
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .bg(self.backdrop_fill)
                .occlude()
                .child(dialog_card)
                .into_any_element()
        } else {
            // No cancel handler — build without Escape wiring
            let mut confirm_btn = div()
                .id("alert-dialog-confirm")
                .cursor_pointer()
                .px(self.padding_x)
                .py(px(6.0))
                .rounded(self.button_radius)
                .bg(self.confirm_fill)
                .text_color(self.confirm_text_color)
                .hover(|s| s.opacity(0.9))
                .child(self.spec.confirm_label.clone());
            if let Some(handler) = self.on_confirm {
                confirm_btn = confirm_btn.on_click(move |_, window, app| {
                    handler(window, app);
                });
            }

            let actions = div()
                .flex()
                .flex_row()
                .justify_end()
                .gap(self.actions_gap)
                .child(cancel_btn)
                .child(confirm_btn);

            let dialog_card = div()
                .id("alert-dialog")
                .focusable()
                .flex()
                .flex_col()
                .bg(self.dialog_fill)
                .rounded(self.dialog_radius)
                .border_1()
                .border_color(self.border_color)
                .shadow(vec![
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.12),
                        offset: point(px(0.0), px(8.0)),
                        blur_radius: px(24.0),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.08),
                        offset: point(px(0.0), px(2.0)),
                        blur_radius: px(8.0),
                        spread_radius: px(0.0),
                    },
                ])
                .px(self.padding_x)
                .py(self.padding_y)
                .gap(self.content_gap)
                .max_w(px(480.0))
                .min_w(px(320.0))
                .occlude()
                .child(header)
                .child(actions);

            div()
                .id("alert-dialog-backdrop")
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .bg(self.backdrop_fill)
                .occlude()
                .child(dialog_card)
                .into_any_element()
        }
    }
}
