//! JsFileUpload — drag-and-drop file upload zone backed by FileUploadSpec.
//!
//! Contract: `docs/contracts/components/file-upload.md`
//! Reference: `packages/svelte/primitives/src/FileUpload.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::{ControlSize, FileUploadSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Dropzone min-height in rem per size (contract section 8).
fn dropzone_min_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 5.0,
        ControlSize::Sm => 6.0,
        ControlSize::Md => 8.0,
        ControlSize::Lg => 10.0,
        ControlSize::Xl => 12.0,
    }
}

/// Icon size in rem per size (contract section 8).
fn icon_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm | ControlSize::Md | ControlSize::Lg => 2.0,
        ControlSize::Xl => 2.5,
    }
}

/// Label font size in rem per size (contract section 8).
fn label_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm | ControlSize::Md => 0.875,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Hint font size in rem per size (contract section 8).
fn hint_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        _ => 0.8125,
    }
}

/// Build a Jetstream file upload element from a FileUploadSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Root .file-upload]  <div>
///   ├── [Dropzone .file-upload__dropzone]  <div>
///   │     ├── [Hidden Input .file-upload__input]  <input type="file">
///   │     └── [Dropzone Content .file-upload__dropzone-content]
///   │           ├── [Icon .file-upload__icon]  <svg>
///   │           ├── [Label .file-upload__label]  <p>
///   │           │     └── [Browse .file-upload__browse]  <span>
///   │           └── [Hint .file-upload__hint]  <p>
///   └── [File List .file-upload__list]  <ul> (conditional)
/// ```
pub fn js_file_upload(spec: &FileUploadSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let _fill: Color = resolve_color(theme, spec.fill_token()).into(); // used by file list items at runtime
    let border_color: Color = resolve_color(theme, spec.border_token()).into();
    let _text_primary: Color = resolve_color(theme, spec.text_color_token()).into(); // used by file name labels at runtime
    let text_secondary: Color = resolve_color(theme, "color.text.secondary").into();
    let text_tertiary: Color = resolve_color(theme, "color.text.tertiary").into();
    let accent: Color = resolve_color(theme, "color.accent.base").into();
    let radius = resolve_radius(theme, spec.radius_token());

    // ── Sizing ──
    let min_height = rem_to_px(dropzone_min_height_rem(effective_size));
    let icon_sz = rem_to_px(icon_size_rem(effective_size));
    let label_font = rem_to_px(label_font_rem(effective_size));
    let hint_font = rem_to_px(hint_font_rem(effective_size));
    let padding = rem_to_px(1.5); // Contract: 1.5rem padding
    let content_gap = rem_to_px(0.375); // Contract: 0.375rem gap in dropzone content
    let root_gap = rem_to_px(0.5); // Contract: gap = var(--poodle-space-stack-sm, 0.5rem)
    let border_width = rem_to_px(0.125); // Contract: 0.125rem dashed border

    // ── Dropzone content ──
    let icon = ui_element::icon("upload")
        .w(icon_sz)
        .h(icon_sz)
        .text_color(text_secondary);

    // Label with "browse" accent text
    let label = ui_element::label("Drop files here or browse")
        .text_size(label_font)
        .text_color(text_secondary);

    // Hint: file constraints description
    let hint_text = build_hint_text(spec);
    let hint = ui_element::label(&hint_text)
        .text_size(hint_font)
        .text_color(text_tertiary);

    let dropzone_content = ui_element::div()
        .flex_col()
        .items_center()
        .gap(content_gap)
        .text_align_center()
        .child(icon)
        .child(label)
        .child(hint);

    // ── Dropzone ──
    let mut dropzone = ui_element::div()
        .min_h(min_height)
        .p(padding)
        .rounded(radius)
        .bg(Color::TRANSPARENT) // Contract: transparent default
        .border(border_width)
        .border_color(border_color)
        .flex_row()
        .items_center()
        .justify_center()
        .focusable()
        .child(dropzone_content);

    // Drag-active state: accent border, tinted background
    if spec.is_dragging {
        let drag_tint = Color::new(accent.r, accent.g, accent.b, accent.a * 0.08);
        dropzone = dropzone
            .border_color(accent)
            .bg(drag_tint);
    }

    // Hover state
    if !spec.is_disabled {
        let panel_bg: Color = resolve_color(theme, "color.background.panel").into();
        let hover_bg = Color::new(panel_bg.r, panel_bg.g, panel_bg.b, panel_bg.a * 0.50);
        let focus_border: Color = resolve_color(theme, spec.focus_border_token()).into();
        dropzone = dropzone
            .hover(|s| s.bg(hover_bg).border_color(focus_border))
            .cursor_pointer();
    }

    // ── Root ──
    let mut root = ui_element::div()
        .flex_col()
        .gap(root_gap)
        .self_stretch()
        .child(dropzone);

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        root = root.opacity(opacity).disabled(true);
    }

    root
}

/// Build hint text describing file constraints.
fn build_hint_text(spec: &FileUploadSpec) -> String {
    let mut parts = Vec::new();
    if let Some(ref accept) = spec.accept {
        parts.push(format!("Accepted: {}", accept));
    }
    if let Some(max) = spec.max_size {
        let mb = max as f64 / (1024.0 * 1024.0);
        parts.push(format!("Max {:.0} MB", mb));
    }
    if spec.is_multiple {
        parts.push(String::from("Multiple files allowed"));
    }
    if parts.is_empty() {
        String::from("Upload files")
    } else {
        parts.join(" · ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn root_has_dropzone_child() {
        let spec = FileUploadSpec::new();
        let el = js_file_upload(&spec, &theme());
        assert!(!el.children.is_empty(), "Root should contain dropzone");
    }

    #[test]
    fn disabled_has_reduced_opacity() {
        let spec = FileUploadSpec::new().with_disabled(true);
        let el = js_file_upload(&spec, &theme());
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn dragging_state_changes_dropzone_background() {
        let normal = js_file_upload(&FileUploadSpec::new(), &theme());
        let dragging = js_file_upload(&FileUploadSpec::new().with_dragging(true), &theme());
        let normal_bg = normal.children[0].style.background;
        let dragging_bg = dragging.children[0].style.background;
        assert_ne!(normal_bg, dragging_bg, "Drag-active should change dropzone background");
    }
}
