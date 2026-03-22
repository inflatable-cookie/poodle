//! ConfirmAction — confirmation dialog backed by ConfirmActionSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::ConfirmActionSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct ConfirmAction {
    spec: ConfirmActionSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for ConfirmAction {
    type Target = ConfirmActionSpec;
    fn deref(&self) -> &ConfirmActionSpec { &self.spec }
}

impl ConfirmAction {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ConfirmActionSpec::new("Confirm", "Are you sure?", "Confirm", "Cancel"), theme: theme.clone() }
    }
    pub fn from_spec(spec: ConfirmActionSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
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

        let mut el = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .px(px(24.0)).py(px(20.0))
            .flex().flex_col().gap(px(16.0))
            .min_w(px(360.0));

        el = el.child(div().text_color(title_color).font_weight(FontWeight::SEMIBOLD).child(spec.title.clone()));
        el = el.child(div().text_size(px(14.0)).text_color(msg_color).child(spec.message.clone()));

        let actions = div().flex().flex_row().gap(px(8.0)).justify_end()
            .child(div().text_size(px(14.0)).text_color(title_color).cursor_pointer().child(spec.cancel_label.clone()))
            .child(div().text_size(px(14.0)).text_color(gpui::white()).bg(confirm_fill)
                .rounded(resolve_radius(theme, "semantic.radius.control"))
                .px(px(12.0)).py(px(6.0)).cursor_pointer()
                .child(spec.confirm_label.clone()));
        el = el.child(actions);
        el.into_any_element()
    }
}
