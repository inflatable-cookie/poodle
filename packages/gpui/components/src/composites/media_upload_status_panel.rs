//! MediaUploadStatusPanel — workflow status panel for media upload.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{MediaUploadStatusPanelSpec, MediaUploadStep};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub struct MediaUploadStatusPanel {
    spec: MediaUploadStatusPanelSpec,
    theme: GpuiThemeProvider,
}

impl MediaUploadStatusPanel {
    pub fn from_spec(spec: MediaUploadStatusPanelSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for MediaUploadStatusPanel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let body_size = resolve_px(theme, "semantic.typography.body.size");
        let label_size = resolve_px(theme, "semantic.typography.label.size");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let success = resolve_color(theme, "semantic.color.status.success");
        let danger = resolve_color(theme, "semantic.color.status.danger");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let radius = resolve_radius(theme, "semantic.radius.surface");

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
            .bg(Hsla { a: surface_bg.a * 0.82, ..surface_bg })
            .border_1().border_color(Hsla { a: text_secondary.a * 0.2, ..text_secondary })
            .p(px(16.0))
            .flex().flex_col().gap(px(8.0));

        panel = panel.child(
            div().text_size(body_size).font_weight(FontWeight::SEMIBOLD)
                .text_color(status_color)
                .child(status_text)
        );

        // Progress bar for uploading step
        if spec.step == MediaUploadStep::Uploading {
            let progress_pct = spec.upload_progress.clamp(0.0, 100.0);
            panel = panel.child(
                div().w_full().h(px(4.0)).rounded(px(2.0))
                    .bg(Hsla { a: text_secondary.a * 0.2, ..text_secondary })
                    .child(
                        div()
                            .h(px(4.0)).rounded(px(2.0))
                            .bg(accent)
                            .w(Pixels(progress_pct / 100.0 * 200.0)) // approximate width
                    )
            );
            panel = panel.child(
                div().text_size(label_size).text_color(text_secondary)
                    .child(format!("{}%", progress_pct as u32))
            );
        }

        // Duplicate label
        if spec.step == MediaUploadStep::Duplicate {
            if let Some(ref label) = spec.duplicate_label {
                panel = panel.child(
                    div().text_size(label_size).text_color(text_primary)
                        .child(label.clone())
                );
            }
        }

        // Error message
        if spec.step == MediaUploadStep::Error {
            if let Some(ref error) = spec.upload_error {
                panel = panel.child(
                    div().text_size(label_size).text_color(text_secondary)
                        .child(error.clone())
                );
            }
        }

        panel.into_any_element()
    }
}
