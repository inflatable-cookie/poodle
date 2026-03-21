//! FormDialog — dialog wrapping a form with submit/cancel actions.
//! No contract spec exists — implemented from Svelte reference.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct FormDialog {
    theme: GpuiThemeProvider,
    title: Option<String>,
    submit_label: String,
    cancel_label: String,
    content: Option<AnyElement>,
}

impl FormDialog {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            title: None,
            submit_label: "Submit".to_string(),
            cancel_label: "Cancel".to_string(),
            content: None,
        }
    }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.title = Some(v.into()); self }
    pub fn submit_label(mut self, v: impl Into<String>) -> Self { self.submit_label = v.into(); self }
    pub fn cancel_label(mut self, v: impl Into<String>) -> Self { self.cancel_label = v.into(); self }
    pub fn with_content(mut self, c: impl IntoElement) -> Self { self.content = Some(c.into_any_element()); self }
}

impl IntoElement for FormDialog {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, "semantic.color.background.elevated");
        let border = resolve_color(&self.theme, "semantic.color.border.default");
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");
        let title_color = resolve_color(&self.theme, "semantic.color.text.primary");

        let mut el = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .px(px(24.0)).py(px(20.0))
            .flex().flex_col().gap(px(16.0))
            .min_w(px(420.0));

        if let Some(title) = &self.title {
            el = el.child(div().text_color(title_color).font_weight(FontWeight::SEMIBOLD).child(title.clone()));
        }
        if let Some(content) = self.content {
            el = el.child(content);
        }
        let actions = div().flex().flex_row().gap(px(8.0)).justify_end()
            .child(div().text_sm().text_color(title_color).cursor_pointer().child(self.cancel_label))
            .child(div().text_sm().text_color(gpui::white())
                .bg(resolve_color(&self.theme, "semantic.color.accent.base"))
                .rounded(resolve_radius(&self.theme, "semantic.radius.control"))
                .px(px(12.0)).py(px(6.0)).cursor_pointer()
                .child(self.submit_label));
        el = el.child(actions);
        el.into_any_element()
    }
}
