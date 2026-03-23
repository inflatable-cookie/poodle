//! FileUpload — real GPUI component backed by FileUploadSpec.
//!
//! Contract: dropzone with dashed border, min-height 8rem,
//! radius-surface, panel padding. No hover on root.
//! Note: GPUI has no dashed border — we approximate with solid.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{FileUploadSpec, IconSize, IconSpec};

use super::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI file upload drop zone component backed by `FileUploadSpec`.
pub struct FileUpload {
    spec: FileUploadSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
}

impl std::ops::Deref for FileUpload {
    type Target = FileUploadSpec;
    fn deref(&self) -> &FileUploadSpec { &self.spec }
}

impl FileUpload {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: FileUploadSpec::new(), theme: theme.clone(), id_suffix: None }
    }

    pub fn from_spec(spec: FileUploadSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn accept(mut self, v: impl Into<String>) -> Self { self.spec.accept = Some(v.into()); self }
    pub fn max_size(mut self, v: u64) -> Self { self.spec.max_size = Some(v); self }
    pub fn multiple(mut self, v: bool) -> Self { self.spec.is_multiple = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn dragging(mut self, v: bool) -> Self { self.spec.is_dragging = v; self }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }
}

impl IntoElement for FileUpload {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Contract: panel padding
        let panel_padding_x = resolve_px(theme, "semantic.space.panel.x");
        let panel_padding_y = resolve_px(theme, "semantic.space.panel.y");
        let stack_gap = resolve_px(theme, "semantic.space.stack.sm");
        let dropzone_radius = resolve_radius(theme, spec.radius_token());
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let text_color = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let focus_border = resolve_color(theme, spec.focus_border_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        let label = if spec.is_dragging {
            "Drop files here"
        } else {
            "Drag files here or click to browse"
        };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-file-upload-{}", suffix)
        } else {
            "poodle-file-upload".to_string()
        };

        // Contract: browse button with accent border
        let browse_btn = div()
            .px(resolve_px(theme, "semantic.space.inline.md"))
            .py(px(6.0))
            .rounded(control_radius)
            .border_1()
            .border_color(accent)
            .text_size(px(14.0))
            .text_color(accent)
            .cursor_pointer()
            .child("Browse");

        let mut accept_hint = div().text_size(px(12.0)).text_color(text_secondary);
        if let Some(ref accept) = spec.accept {
            accept_hint = accept_hint.child(format!("Accepted: {}", accept));
        }

        // Contract: dropzone fill when dragging uses accent at 8% opacity
        let bg = if spec.is_dragging {
            fill.opacity(0.08)
        } else {
            fill
        };

        // Contract: upload icon — Md size for dropzone prominence
        let upload_icon = Icon::from_spec(
            IconSpec::new("upload").with_size(IconSize::Md),
            theme,
        )
        .with_color(text_secondary);

        // Contract: min-height 8rem, dashed border (solid here — GPUI has no dashed)
        let mut zone = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w_full()
            .min_h(px(128.0)) // 8rem
            .rounded(dropzone_radius)
            .bg(bg)
            .border_2() // 0.125rem = 2px (contract: 0.125rem dashed)
            .border_color(border)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(stack_gap)
            .px(panel_padding_x)
            .py(panel_padding_y)
            // Contract: focus = border-color change
            .focus(move |s| s.border_color(focus_border))
            .child(upload_icon)
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(text_color)
                    .child(label),
            )
            .child(browse_btn)
            .child(accept_hint);

        if spec.is_disabled {
            zone = zone
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        zone.into_any_element()
    }
}
