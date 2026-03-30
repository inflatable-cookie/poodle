//! MediaUploadStatusPanel — Jetstream media upload status panel backed by MediaUploadStatusPanelSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::{MediaUploadStatusPanelSpec, MediaUploadStep};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_px, tint};

pub fn js_media_upload_status_panel(spec: &MediaUploadStatusPanelSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));

    let gap = resolve_px(theme, "semantic.space.stack.sm");
    let pad_y = resolve_px(theme, "semantic.space.panel.y");
    let pad_x = resolve_px(theme, "semantic.space.panel.x");
    let action_gap = resolve_px(theme, "semantic.space.inline.sm");

    // Size-driven progress dimensions from contract
    let progress_height = rem_to_px(match effective_size {
        poodle_primitives::ControlSize::Xs => 0.25,
        poodle_primitives::ControlSize::Sm => 0.3125,
        poodle_primitives::ControlSize::Md => 0.375,
        poodle_primitives::ControlSize::Lg => 0.4375,
        poodle_primitives::ControlSize::Xl => 0.5,
    });
    let progress_max_width = rem_to_px(match effective_size {
        poodle_primitives::ControlSize::Xs => 11.0,
        poodle_primitives::ControlSize::Sm => 12.5,
        poodle_primitives::ControlSize::Md => 14.0,
        poodle_primitives::ControlSize::Lg => 15.5,
        poodle_primitives::ControlSize::Xl => 17.0,
    });

    // Density-driven padding from contract
    let (final_pad_y, final_pad_x) = match spec.density {
        poodle_primitives::ControlDensity::Compact => (rem_to_px(0.5), rem_to_px(0.5)),
        poodle_primitives::ControlDensity::Default => (pad_y, pad_x),
        poodle_primitives::ControlDensity::Comfortable => (rem_to_px(1.0), rem_to_px(1.0)),
    };

    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let accent = resolve_color(theme, "semantic.color.accent.base");
    let success = resolve_color(theme, "semantic.color.status.success");
    let danger = resolve_color(theme, "semantic.color.status.danger");
    let warning = resolve_color(theme, "semantic.color.status.warning");
    let surface = resolve_color(theme, "semantic.color.background.surface");

    // Root
    let mut el = ui_element::div()
        .flex_col().items_center().gap(gap)
        .pl(final_pad_x).pr(final_pad_x).pt(final_pad_y).pb(final_pad_y);

    match spec.step {
        MediaUploadStep::Checking => {
            // Spinner placeholder + copy
            el = el.child(
                ui_element::label("\u{25CC}")
                    .text_color(accent).text_size(rem_to_px(1.5))
            );
            el = el.child(
                ui_element::label("Checking for duplicates...")
                    .text_color(text_secondary).text_size(font_size)
            );
        }
        MediaUploadStep::Duplicate => {
            el = el.child(
                ui_element::label("This file already exists.")
                    .text_color(warning).text_size(font_size)
            );
            if let Some(ref dup_label) = spec.duplicate_label {
                el = el.child(
                    ui_element::label(dup_label)
                        .text_color(text_primary).text_size(font_size).text_weight(700)
                );
            }
            let actions = ui_element::div()
                .flex_row().justify_center().gap(action_gap).flex_wrap()
                .child(
                    ui_element::button("Upload as new")
                        .text_color(text_primary).text_size(font_size)
                        .focusable()
                )
                .child(
                    ui_element::button("Use existing")
                        .text_color(text_primary).text_size(font_size)
                        .focusable()
                );
            el = el.child(actions);
        }
        MediaUploadStep::Uploading => {
            // Progress bar
            let track_bg = tint(surface, 0.82);
            let progress_bar = ui_element::div()
                .min_h(progress_height)
                .max_w(progress_max_width).self_stretch()
                .rounded(999.0)
                .bg(track_bg)
                .child(
                    ui_element::div()
                        .min_h(progress_height)
                        .rounded(999.0)
                        .bg(accent)
                        .w(spec.upload_progress * progress_max_width / 100.0)
                );
            el = el.child(progress_bar);
            el = el.child(
                ui_element::label(&format!("Uploading... {:.0}%", spec.upload_progress))
                    .text_color(text_secondary).text_size(font_size)
            );
        }
        MediaUploadStep::Finalising => {
            el = el.child(
                ui_element::label("\u{25CC}")
                    .text_color(accent).text_size(rem_to_px(1.5))
            );
            el = el.child(
                ui_element::label("Finalising...")
                    .text_color(text_secondary).text_size(font_size)
            );
        }
        MediaUploadStep::Complete => {
            el = el.child(
                ui_element::label("Upload complete.")
                    .text_color(success).text_size(font_size)
            );
            let actions = ui_element::div()
                .flex_row().justify_center().gap(action_gap).flex_wrap()
                .child(
                    ui_element::button("Upload another")
                        .text_color(text_primary).text_size(font_size)
                        .focusable()
                )
                .child(
                    ui_element::button("Use this media")
                        .text_color(text_primary).text_size(font_size)
                        .focusable()
                );
            el = el.child(actions);
        }
        MediaUploadStep::Error => {
            let error_msg = spec.upload_error.as_deref().unwrap_or("Upload failed");
            el = el.child(
                ui_element::label(error_msg)
                    .text_color(danger).text_size(font_size)
            );
            let actions = ui_element::div()
                .flex_row().justify_center().gap(action_gap)
                .child(
                    ui_element::button("Try again")
                        .text_color(text_primary).text_size(font_size)
                        .focusable()
                );
            el = el.child(actions);
        }
    }

    el
}
