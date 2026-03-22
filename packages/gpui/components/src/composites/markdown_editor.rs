//! MarkdownEditor — markdown editing with preview backed by MarkdownEditorSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::MarkdownEditorSpec;
use pug_primitives::{IconSize, IconSpec};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct MarkdownEditor {
    spec: MarkdownEditorSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for MarkdownEditor {
    type Target = MarkdownEditorSpec;
    fn deref(&self) -> &MarkdownEditorSpec { &self.spec }
}

impl MarkdownEditor {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: MarkdownEditorSpec::new(), theme: theme.clone() }
    }
    pub fn from_spec(spec: MarkdownEditorSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for MarkdownEditor {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let border = resolve_color(&self.theme, self.spec.border_token());
        let toolbar_fill = resolve_color(&self.theme, self.spec.toolbar_fill_token());
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");
        let text_color = resolve_color(&self.theme, "semantic.color.text.primary");
        let muted = resolve_color(&self.theme, "semantic.color.text.secondary");
        let hover_bg = resolve_color(&self.theme, "semantic.color.bg.hover");
        let active_bg = resolve_color(&self.theme, "semantic.color.bg.active");

        let display = if self.spec.value.is_empty() { self.spec.placeholder.as_deref().unwrap_or("Type here...") } else { &self.spec.value };
        let color = if self.spec.value.is_empty() { muted } else { text_color };

        // Helper: toolbar icon button
        let toolbar_btn = |icon_name: &str, theme: &GpuiThemeProvider| -> Div {
            div()
                .flex().items_center().justify_center()
                .w(px(28.0)).h(px(24.0)).rounded(px(4.0))
                .cursor(CursorStyle::PointingHand)
                .hover(|s| s.bg(hover_bg))
                .child(
                    Icon::from_spec(
                        IconSpec::new(icon_name).with_size(IconSize::Sm),
                        theme,
                    ).with_color(muted)
                )
        };

        // Helper: mode button
        let mode_btn = |label: &str, is_active: bool| -> Div {
            let mut btn = div()
                .text_size(px(12.0)).px(px(8.0)).py(px(2.0)).rounded(px(4.0))
                .cursor(CursorStyle::PointingHand);
            if is_active {
                btn = btn.bg(active_bg).text_color(text_color);
            } else {
                btn = btn.text_color(muted).hover(|s| s.bg(hover_bg));
            }
            btn.child(label.to_string())
        };

        let mut el = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .flex().flex_col().min_h(px(200.0));

        // Toolbar
        el = el.child(
            div().bg(toolbar_fill).px(px(8.0)).py(px(4.0))
                .flex().flex_row().items_center().gap(px(2.0))
                .border_b_1().border_color(border)
                // Formatting icons
                .child(toolbar_btn("bold", &self.theme))
                .child(toolbar_btn("italic", &self.theme))
                .child(toolbar_btn("heading-1", &self.theme))
                // Spacer
                .child(div().flex_grow())
                // Mode indicator buttons
                .child(
                    div().flex().flex_row().gap(px(2.0))
                        .child(mode_btn("Edit", true))
                        .child(mode_btn("Preview", false))
                )
        );

        // Editor area
        el = el.child(
            div().px(px(12.0)).py(px(8.0)).flex_grow()
                .text_size(px(14.0)).text_color(color)
                .child(display.to_string())
        );

        if self.spec.is_disabled {
            el = el.opacity(resolve_opacity(&self.theme, "semantic.state.opacity.disabled"));
        }
        el.into_any_element()
    }
}
