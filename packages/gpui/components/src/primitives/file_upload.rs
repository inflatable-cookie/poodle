//! FileUpload — real GPUI component backed by FileUploadSpec.
//!
//! Contract: dropzone with dashed border, min-height 8rem,
//! radius-surface, panel padding. No hover on root.
//! Dropzone border uses GPUI `.border_dashed()` per contract.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, FileUploadSpec, IconSize, IconSpec, SemanticControlSizeRole,
};

use super::icon::Icon;
use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI file upload drop zone component backed by `FileUploadSpec`.
pub struct FileUpload {
    spec: FileUploadSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
}

impl std::ops::Deref for FileUpload {
    type Target = FileUploadSpec;
    fn deref(&self) -> &FileUploadSpec {
        &self.spec
    }
}

impl FileUpload {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: FileUploadSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
        }
    }

    pub fn from_spec(spec: FileUploadSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn accept(mut self, v: impl Into<String>) -> Self {
        self.spec.accept = Some(v.into());
        self
    }
    pub fn max_size(mut self, v: u64) -> Self {
        self.spec.max_size = Some(v);
        self
    }
    pub fn max_files(mut self, v: u32) -> Self {
        self.spec.max_files = Some(v);
        self
    }
    pub fn multiple(mut self, v: bool) -> Self {
        self.spec.is_multiple = v;
        self
    }
    pub fn compress(mut self, v: bool) -> Self {
        self.spec.compress = v;
        self
    }
    pub fn validation_error(mut self, v: impl Into<String>) -> Self {
        self.spec.validation_error = Some(v.into());
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn dragging(mut self, v: bool) -> Self {
        self.spec.is_dragging = v;
        self
    }

    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

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

        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let body_font = px(rem_to_px(size_font_rem(effective_size)));
        // Contract: panel padding from density
        let panel_padding_x = px(rem_to_px(panel_space_x_rem(spec.density)));
        let panel_padding_y = px(rem_to_px(panel_space_y_rem(spec.density)));
        let stack_gap = resolve_px(theme, "space.stack.sm");
        let helper_gap = resolve_px(theme, "space.inline.xs");
        let caption_size = resolve_px(theme, "typography.caption.size");
        let control_pad_y = resolve_px(theme, "space.control.y");
        let dropzone_min_h = resolve_px(theme, "size.fileUpload.dropZoneMinHeight");
        let dropzone_radius = resolve_radius(theme, spec.radius_token());
        let control_radius = resolve_radius(theme, "radius.control");

        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let text_color = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent = resolve_color(theme, "color.accent.base");
        let focus_border = resolve_color(theme, spec.focus_border_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let body_size = body_font;

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
            .px(resolve_px(theme, "space.inline.md"))
            .py(control_pad_y)
            .rounded(control_radius)
            .border_1()
            .border_color(accent)
            .text_size(body_size)
            .text_color(accent)
            .cursor_pointer()
            .child("Browse");

        // Helper text block — stacks the accept hint, max-files cap,
        // and compress notice in a small column. Kept empty when none
        // of the hints apply.
        let mut helper_block = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(helper_gap)
            .text_size(caption_size)
            .text_color(text_secondary);

        if let Some(ref accept) = spec.accept {
            helper_block = helper_block.child(div().child(format!("Accepted: {}", accept)));
        }

        if let Some(max_files) = spec.max_files {
            if spec.is_multiple {
                helper_block = helper_block.child(div().child(format!("Up to {max_files} files")));
            }
        }

        if spec.compress {
            helper_block =
                helper_block.child(div().child("Images will be compressed before upload"));
        }

        // Contract: dropzone fill when dragging uses accent at 8% opacity
        let bg = if spec.is_dragging {
            fill.opacity(0.08)
        } else {
            fill
        };

        // Contract: upload icon — Md size for dropzone prominence
        let upload_icon = Icon::from_spec(IconSpec::new("upload").with_size(IconSize::Md), theme)
            .with_color(text_secondary);

        // Contract: min-height 8rem, dashed border
        let mut zone = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w_full()
            .min_h(dropzone_min_h)
            .rounded(dropzone_radius)
            .bg(bg)
            .border_2() // 0.125rem = 2px (contract: 0.125rem dashed)
            .border_dashed()
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
                    .text_size(body_size)
                    .text_color(text_color)
                    .child(label),
            )
            .child(browse_btn)
            .child(helper_block);

        if spec.is_disabled {
            zone = zone
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        // Validation error message rendered below the dropzone when
        // the spec carries one. Uses status.danger foreground.
        if let Some(ref err) = spec.validation_error {
            let danger = resolve_color(theme, spec.error_color_token());
            return div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(zone)
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(danger)
                        .child(err.clone()),
                )
                .into_any_element();
        }

        zone.into_any_element()
    }
}
