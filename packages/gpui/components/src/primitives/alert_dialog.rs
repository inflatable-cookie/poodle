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

        let aria_label = self.spec.aria_label.clone().unwrap_or_else(|| self.spec.title.clone());

        // Backdrop
        div()
            .id("alert-dialog-backdrop")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(self.backdrop_fill)
            .child(
                // Dialog card
                div()
                    .id("alert-dialog")
                    .flex()
                    .flex_col()
                    .bg(self.dialog_fill)
                    .rounded(self.dialog_radius)
                    .border_1()
                    .border_color(self.border_color)
                    .px(self.padding_x)
                    .py(self.padding_y)
                    .gap(self.content_gap)
                    .max_w(px(480.0))
                    .min_w(px(320.0))
                    .occlude()
                    .map(|el| {
                        el.child(
                            div()
                                .id("alert-dialog-header")
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(
                                    div()
                                        .text_color(self.title_color)
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child(self.spec.title.clone()),
                                )
                                .when_some(self.spec.description.as_ref(), |el, desc| {
                                    el.child(
                                        div()
                                            .text_color(self.description_color)
                                            .child(desc.clone()),
                                    )
                                }),
                        )
                    })
                    .child(
                        // Actions row
                        div()
                            .id("alert-dialog-actions")
                            .flex()
                            .flex_row()
                            .justify_end()
                            .gap(self.actions_gap)
                            .child(
                                // Cancel button (ghost style)
                                div()
                                    .id("alert-dialog-cancel")
                                    .cursor_pointer()
                                    .px(self.padding_x)
                                    .py(px(6.0))
                                    .rounded(self.dialog_radius)
                                    .text_color(self.cancel_text_color)
                                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.1)))
                                    .child(self.spec.cancel_label.clone())
                                    .when_some(self.on_cancel, |el, handler| {
                                        el.on_mouse_down(MouseButton::Left, move |_, window, app| {
                                            handler(window, app);
                                        })
                                    }),
                            )
                            .child(
                                // Confirm button (filled style)
                                div()
                                    .id("alert-dialog-confirm")
                                    .cursor_pointer()
                                    .px(self.padding_x)
                                    .py(px(6.0))
                                    .rounded(self.dialog_radius)
                                    .bg(self.confirm_fill)
                                    .text_color(self.confirm_text_color)
                                    .hover(|s| s.opacity(0.9))
                                    .child(self.spec.confirm_label.clone())
                                    .when_some(self.on_confirm, |el, handler| {
                                        el.on_mouse_down(MouseButton::Left, move |_, window, app| {
                                            handler(window, app);
                                        })
                                    }),
                            ),
                    ),
            )
            .into_any_element()
    }
}
