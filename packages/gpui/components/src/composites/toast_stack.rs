//! ToastStack — real GPUI component backed by ToastStackSpec.

use std::rc::Rc;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{Toast, ToastPosition, ToastStackSpec};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI toast stack component backed by `ToastStackSpec`.
///
/// Renders a stack of toast notifications at a chosen screen corner.
pub struct ToastStack {
    spec: ToastStackSpec,
    theme: GpuiThemeProvider,
    on_dismiss: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_action: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ToastStack {
    type Target = ToastStackSpec;
    fn deref(&self) -> &ToastStackSpec { &self.spec }
}

impl ToastStack {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ToastStackSpec::new(), theme: theme.clone(), on_dismiss: None, on_action: None }
    }

    pub fn from_spec(spec: ToastStackSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_dismiss: None,
            on_action: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn toasts(mut self, v: Vec<Toast>) -> Self { self.spec.toasts = v; self }
    pub fn position(mut self, v: ToastPosition) -> Self { self.spec.position = v; self }


    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Rc::new(handler));
        self
    }

    pub fn on_action(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_action = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for ToastStack {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let gap = resolve_px(theme, spec.gap_token());
        let padding = resolve_px(theme, spec.padding_token());
        let fill = resolve_color(theme, spec.fill_token());
        let border_color = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let title_color = resolve_color(theme, spec.title_color_token());
        let message_color = resolve_color(theme, spec.message_color_token());
        let dismiss_color = resolve_color(theme, spec.dismiss_color_token());
        let accent = resolve_color(theme, "semantic.color.accent.base");

        // Container positioned at the chosen corner
        let mut container = div()
            .absolute()
            .flex()
            .flex_col()
            .gap(gap)
            .w(px(360.0));

        // Position based on spec
        match spec.position {
            ToastPosition::TopRight => {
                container = container.top(padding).right(padding);
            }
            ToastPosition::TopLeft => {
                container = container.top(padding).left(padding);
            }
            ToastPosition::BottomRight => {
                container = container.bottom(padding).right(padding);
            }
            ToastPosition::BottomLeft => {
                container = container.bottom(padding).left(padding);
            }
        }

        for toast in &spec.toasts {
            let tone_color = resolve_color(theme, spec.tone_color(&toast.tone));
            let dismiss_element_id =
                SharedString::from(format!("toast-dismiss-{}", toast.id));

            // Build the content column: title + optional message
            let mut content_col = div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .flex_grow()
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(title_color)
                        .child(toast.title.clone()),
                );

            if let Some(ref message) = toast.message {
                content_col = content_col.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(message_color)
                        .child(message.clone()),
                );
            }

            // Optional action button
            if let Some(ref action_label) = toast.action_label {
                let action_element_id =
                    SharedString::from(format!("toast-action-{}", toast.id));

                let mut action_btn = div()
                    .id(action_element_id)
                    .cursor_pointer()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(accent)
                    .mt(px(4.0))
                    .child(action_label.clone());

                if let Some(ref handler) = self.on_action {
                    let handler = Rc::clone(handler);
                    let toast_id = toast.id.clone();
                    action_btn = action_btn.on_click(move |_event, window, app| {
                        handler(&toast_id, window, app);
                    });
                }

                content_col = content_col.child(action_btn);
            }

            // Dismiss button
            let mut dismiss_btn = div()
                .id(dismiss_element_id)
                .cursor_pointer()
                .text_size(px(14.0))
                .text_color(dismiss_color)
                .pl(px(8.0))
                .child("\u{00d7}"); // multiplication sign as close icon

            if let Some(ref handler) = self.on_dismiss {
                let handler = Rc::clone(handler);
                let toast_id = toast.id.clone();
                dismiss_btn = dismiss_btn.on_click(move |_event, window, app| {
                    handler(&toast_id, window, app);
                });
            }

            // Toast row: accent bar + content + dismiss
            let toast_el = div()
                .flex()
                .flex_row()
                .items_start()
                .bg(fill)
                .border_1()
                .border_color(border_color)
                .rounded(radius)
                .overflow_hidden()
                .child(
                    // Left accent bar
                    div()
                        .w(px(4.0))
                        .h_full()
                        .bg(tone_color)
                        .flex_shrink_0(),
                )
                .child(
                    // Content area with padding
                    div()
                        .flex()
                        .flex_row()
                        .items_start()
                        .flex_grow()
                        .px(padding)
                        .py(padding)
                        .child(content_col)
                        .child(dismiss_btn),
                );

            container = container.child(toast_el);
        }

        container.into_any_element()
    }
}
