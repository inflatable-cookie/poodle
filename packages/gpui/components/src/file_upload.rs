//! PugFileUpload — real GPUI component backed by FileUploadSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::FileUploadSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI file upload drop zone component backed by `FileUploadSpec`.
pub struct PugFileUpload {
    spec: FileUploadSpec,
    theme: GpuiThemeProvider,
}

impl PugFileUpload {
    pub fn new(spec: FileUploadSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugFileUpload {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let text_color = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let label = if spec.is_dragging {
            "Drop files here"
        } else {
            "Drag files here or click to browse"
        };

        let browse_btn = div()
            .px(px(12.0))
            .py(px(6.0))
            .rounded(px(6.0))
            .border_1()
            .border_color(accent)
            .text_sm()
            .text_color(accent)
            .cursor_pointer()
            .child("Browse");

        let mut accept_hint = div().text_xs().text_color(text_secondary);
        if let Some(ref accept) = spec.accept {
            accept_hint = accept_hint.child(format!("Accepted: {}", accept));
        }

        let mut zone = div()
            .w_full()
            .min_h(px(120.0))
            .rounded(px(8.0))
            .bg(if spec.is_dragging {
                fill.opacity(0.08)
            } else {
                fill
            })
            .border_1()
            .border_color(border)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .p(px(16.0))
            .child(
                div()
                    .text_sm()
                    .text_color(text_color)
                    .child(label),
            )
            .child(browse_btn)
            .child(accept_hint);

        if spec.is_disabled {
            zone = zone.opacity(0.48);
        }

        zone.into_any_element()
    }
}
