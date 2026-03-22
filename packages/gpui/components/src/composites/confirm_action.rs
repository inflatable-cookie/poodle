//! ConfirmAction — confirmation dialog backed by ConfirmActionSpec.
//!
//! Renders an AlertDialog-style overlay with backdrop, trigger button,
//! and confirm/cancel actions.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::ConfirmActionSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct ConfirmAction {
    spec: ConfirmActionSpec,
    theme: GpuiThemeProvider,
    /// Optional trigger element that opens the confirmation dialog.
    trigger: Option<AnyElement>,
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
        }
    }
    pub fn from_spec(spec: ConfirmActionSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), trigger: None }
    }

    /// Set a trigger element (e.g. a button) that opens the confirmation dialog.
    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }
}

impl IntoElement for ConfirmAction {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let fill = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, "semantic.color.border.default");
        let radius = resolve_radius(theme, "semantic.radius.surface");
        let title_color = resolve_color(theme, "semantic.color.text.primary");
        let msg_color = resolve_color(theme, "semantic.color.text.secondary");
        let confirm_fill = resolve_color(theme, spec.confirm_fill_token());

        let mut dialog = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .px(px(24.0)).py(px(20.0))
            .flex().flex_col().gap(px(16.0))
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

        dialog = dialog.child(div().text_size(px(18.0)).text_color(title_color).font_weight(FontWeight::SEMIBOLD).child(spec.title.clone()));
        dialog = dialog.child(div().text_size(px(14.0)).text_color(msg_color).child(spec.message.clone()));

        let actions = div().flex().flex_row().flex_wrap().gap(px(8.0)).justify_end()
            .child(div().text_size(px(14.0)).text_color(title_color).cursor_pointer().child(spec.cancel_label.clone()))
            .child(div().text_size(px(14.0)).text_color(gpui::white()).bg(confirm_fill)
                .rounded(resolve_radius(theme, "semantic.radius.control"))
                .px(px(12.0)).py(px(6.0)).cursor_pointer()
                .child(spec.confirm_label.clone()));
        dialog = dialog.child(actions);

        // Backdrop overlay — full-viewport scrim with centered dialog
        let backdrop = div()
            .id("pug-confirm-backdrop")
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
