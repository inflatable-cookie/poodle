//! InlineEditableField — click-to-edit field backed by InlineEditableFieldSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec};
use pug_composites::InlineEditableFieldSpec;
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct InlineEditableField {
    spec: InlineEditableFieldSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for InlineEditableField {
    type Target = InlineEditableFieldSpec;
    fn deref(&self) -> &InlineEditableFieldSpec { &self.spec }
}

impl InlineEditableField {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: InlineEditableFieldSpec::new(""), theme: theme.clone() }
    }
    pub fn from_spec(spec: InlineEditableFieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for InlineEditableField {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let text_color = resolve_color(theme, spec.text_color_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.control");
        let display = if spec.value.is_empty() { spec.placeholder.as_deref().unwrap_or("") } else { &spec.value };
        let placeholder_color = resolve_color(theme, "semantic.color.text.secondary");
        let display_color = if spec.value.is_empty() { placeholder_color } else { text_color };
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let hover_border = resolve_color(theme, "semantic.color.border.default");

        if spec.is_editing {
            // Editing mode: bordered input-like field
            let mut el = div()
                .id("pug-inline-edit")
                .border_1()
                .border_color(border)
                .rounded(radius)
                .bg(surface_bg)
                .px(px(8.0))
                .py(px(4.0))
                .text_size(px(14.0))
                .text_color(display_color)
                .focus(move |s| s.border_color(focus_ring))
                .child(display.to_string());

            if spec.is_disabled {
                let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
                el = el.opacity(opacity).cursor(CursorStyle::OperationNotAllowed);
            }
            el.into_any_element()
        } else {
            // Display mode: text with hover hint border + edit icon
            let edit_icon = Icon::from_spec(
                IconSpec::new("pencil").with_size(IconSize::Sm),
                theme,
            ).with_color(placeholder_color);

            let mut el = div()
                .id("pug-inline-edit-display")
                .flex()
                .items_center()
                .gap(px(4.0))
                .px(px(8.0))
                .py(px(4.0))
                .rounded(radius)
                .border_1()
                .border_color(gpui::transparent_black())
                .text_size(px(14.0))
                .text_color(display_color)
                .child(display.to_string())
                .child(
                    div()
                        .opacity(0.0)
                        .child(edit_icon),
                )
                .when(!spec.is_disabled, |el| {
                    el.cursor_pointer()
                        .hover(move |s| s.border_color(hover_border))
                });

            if spec.is_disabled {
                let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
                el = el.opacity(opacity).cursor(CursorStyle::OperationNotAllowed);
            }
            el.into_any_element()
        }
    }
}
