//! ConfirmAction — confirmation dialog backed by ConfirmActionSpec.
//!
//! Renders an AlertDialog-style overlay with backdrop, trigger button,
//! and confirm/cancel actions.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::ConfirmActionSpec;
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};
use crate::presentation::{resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem, control_space_x_rem, rem_to_px};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub struct ConfirmAction {
    spec: ConfirmActionSpec,
    theme: GpuiThemeProvider,
    /// Optional trigger element that opens the confirmation dialog.
    trigger: Option<AnyElement>,
    /// Optional body content rendered between the description and the
    /// action row (matches the Svelte default slot).
    content: Option<AnyElement>,
    on_confirm: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_cancel: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ConfirmAction {
    type Target = ConfirmActionSpec;
    fn deref(&self) -> &ConfirmActionSpec { &self.spec }
}

impl ConfirmAction {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ConfirmActionSpec::new("Confirm", "Are you sure?", "Confirm", "Cancel"),
            theme: theme.clone(),
            trigger: None,
            content: None,
            on_confirm: None,
            on_cancel: None,
        }
    }
    pub fn from_spec(spec: ConfirmActionSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), trigger: None, content: None, on_confirm: None, on_cancel: None }
    }

    /// Set a trigger element (e.g. a button) that opens the confirmation dialog.
    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    /// Body content rendered between the description and the action row.
    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    /// Set the open state via spec. Included as a fluent shortcut alongside
    /// `ConfirmActionSpec::with_open`.
    pub fn open(mut self, v: bool) -> Self { self.spec.is_open = v; self }

    /// Fired when the confirm button is clicked. Signature matches
    /// `Div::on_click` so specimen code can pass `cx.listener(...)` directly.
    pub fn on_confirm(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_confirm = Some(Box::new(handler));
        self
    }

    /// Fired when the cancel button is clicked. Signature matches
    /// `Div::on_click` so specimen code can pass `cx.listener(...)` directly.
    pub fn on_cancel(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }
}

impl IntoElement for ConfirmAction {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        // When closed, render only the trigger (if any) — no backdrop.
        if !self.spec.is_open {
            return if let Some(trigger) = self.trigger {
                div().child(trigger).into_any_element()
            } else {
                div().into_any_element()
            };
        }

        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let pad_x = rem_to_px(panel_space_x_rem(spec.density));
        let pad_y = rem_to_px(panel_space_y_rem(spec.density));
        let gap = rem_to_px(control_space_x_rem(spec.density));

        let fill = resolve_color(theme, "color.background.elevated");
        let border = resolve_color(theme, "color.border.default");
        let radius = resolve_radius(theme, "radius.surface");
        let title_color = resolve_color(theme, "color.text.primary");
        let msg_color = resolve_color(theme, "color.text.secondary");
        let confirm_fill = resolve_color(theme, spec.confirm_fill_token());
        let body_size = px(font_size);

        let mut dialog = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .px(px(pad_x)).py(px(pad_y))
            .flex().flex_col().gap(px(gap * 2.0))
            .min_w(px(360.0))
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
            ]);

        let heading_size = resolve_px(theme, "typography.heading.size");
        dialog = dialog.child(div().text_size(heading_size).text_color(title_color).font_weight(FontWeight::SEMIBOLD).child(spec.title.clone()));
        dialog = dialog.child(div().text_size(body_size).text_color(msg_color).child(spec.message.clone()));

        // Optional body content slot (matches Svelte default slot)
        if let Some(content) = self.content {
            dialog = dialog.child(content);
        }

        let control_radius = resolve_radius(theme, "radius.control");
        let hover_fill = resolve_color(theme, "color.background.elevated");
        let btn_gap_lg = resolve_px(theme, "space.inline.lg");
        let btn_gap_sm = resolve_px(theme, "space.inline.sm");

        let mut cancel_btn = div()
            .id("poodle-confirm-cancel")
            .text_size(body_size)
            .text_color(title_color)
            .cursor_pointer()
            .px(btn_gap_lg).py(btn_gap_sm)
            .rounded(control_radius)
            .hover(move |s| s.bg(hover_fill))
            .child(spec.cancel_label.clone());

        if let Some(handler) = self.on_cancel {
            cancel_btn = cancel_btn.on_click(move |event, window, cx| {
                handler(event, window, cx);
            });
        }

        let mut confirm_btn = div()
            .id("poodle-confirm-ok")
            .text_size(body_size)
            .text_color(gpui::white())
            .bg(confirm_fill)
            .rounded(control_radius)
            .px(btn_gap_lg).py(btn_gap_sm)
            .cursor_pointer()
            .font_weight(FontWeight::MEDIUM)
            .child(spec.confirm_label.clone());

        if let Some(handler) = self.on_confirm {
            confirm_btn = confirm_btn.on_click(move |event, window, cx| {
                handler(event, window, cx);
            });
        }

        let actions = div().flex().flex_row().flex_wrap().gap(px(gap)).justify_end()
            .child(cancel_btn)
            .child(confirm_btn);
        dialog = dialog.child(actions);

        // Backdrop overlay — full-viewport scrim with centered dialog
        let backdrop = div()
            .id("poodle-confirm-backdrop")
            .absolute()
            .inset_0()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .flex()
            .items_center()
            .justify_center()
            .occlude()
            .child(dialog);

        // If a trigger is provided, render it alongside the backdrop.
        // The trigger is what the user clicks to open the confirmation.
        if let Some(trigger) = self.trigger {
            div()
                .child(trigger)
                .child(backdrop)
                .into_any_element()
        } else {
            backdrop.into_any_element()
        }
    }
}
