//! MediaUploadStatusPanel — workflow status panel for media upload.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_specs::{MediaUploadStatusPanelSpec, MediaUploadStep};

use crate::presentation::{
    control_space_x_rem, panel_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct MediaUploadStatusPanel {
    spec: MediaUploadStatusPanelSpec,
    theme: GpuiThemeProvider,
}

impl MediaUploadStatusPanel {
    pub fn from_spec(spec: MediaUploadStatusPanelSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
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
}

impl IntoElement for MediaUploadStatusPanel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let pad = rem_to_px(panel_space_x_rem(spec.density));
        let item_gap = rem_to_px(control_space_x_rem(spec.density));

        let body_size = px(font_size);
        let label_size = px(font_size * 0.85);
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent = resolve_color(theme, "color.accent.base");
        let success = resolve_color(theme, "color.status.success");
        let danger = resolve_color(theme, "color.status.danger");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let radius = resolve_radius(theme, "radius.surface");

        let (status_text, status_color) = match spec.step {
            MediaUploadStep::Checking => ("Checking file...", text_secondary),
            MediaUploadStep::Duplicate => ("Duplicate found", accent),
            MediaUploadStep::Uploading => ("Uploading...", accent),
            MediaUploadStep::Finalising => ("Finalising...", accent),
            MediaUploadStep::Complete => ("Upload complete", success),
            MediaUploadStep::Error => ("Upload failed", danger),
        };

        let mut panel = div()
            .w_full()
            .rounded(radius)
            .bg(Hsla {
                a: surface_bg.a * 0.82,
                ..surface_bg
            })
            .border_1()
            .border_color(Hsla {
                a: text_secondary.a * 0.2,
                ..text_secondary
            })
            .p(px(pad))
            .flex()
            .flex_col()
            .gap(px(item_gap));

        panel = panel.child(
            div()
                .text_size(body_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(status_color)
                .child(status_text),
        );

        // Progress bar for uploading step
        if spec.step == MediaUploadStep::Uploading {
            let progress_pct = spec.upload_progress.clamp(0.0, 100.0);
            panel = panel.child(
                div()
                    .w_full()
                    .h(px(4.0))
                    .rounded(px(2.0))
                    .bg(Hsla {
                        a: text_secondary.a * 0.2,
                        ..text_secondary
                    })
                    .child(
                        div()
                            .h(px(4.0))
                            .rounded(px(2.0))
                            .bg(accent)
                            .w(px(progress_pct / 100.0 * 200.0)), // approximate width
                    ),
            );
            panel = panel.child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(format!("{}%", progress_pct as u32)),
            );
        }

        // Duplicate label
        if spec.step == MediaUploadStep::Duplicate {
            if let Some(ref label) = spec.duplicate_label {
                panel = panel.child(
                    div()
                        .text_size(label_size)
                        .text_color(text_primary)
                        .child(label.clone()),
                );
            }
        }

        // Error message
        if spec.step == MediaUploadStep::Error {
            if let Some(ref error) = spec.upload_error {
                panel = panel.child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(error.clone()),
                );
            }
        }

        panel.into_any_element()
    }
}
