//! JsFileUpload — drag-and-drop file upload zone backed by FileUploadSpec.
//!
//! Contract: `docs/contracts/components/file-upload.md`
//! Reference: `packages/svelte/components/src/FileUpload.svelte`
//!
//! ALL dimensions resolve from tokens / contract-exact rem. ZERO raw hsla.
//!
//! Preview-loop: actual drag/drop + native file-dialog interaction lives in
//! the preview event loop, not the component. Image preview bitmaps are
//! host-owned — `FileUploadItem::has_preview` only drives which anatomy part
//! (preview surface vs file icon) renders.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, FileUploadItem, FileUploadSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius, tint};

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

/// Dropzone padding in rem per density (contract §8 panel padding /
/// Svelte compact 1rem / comfortable 1.75rem).
fn dropzone_padding_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 1.0,
        ControlDensity::Default => 1.5,
        ControlDensity::Comfortable => 1.75,
    }
}

/// Dropzone-content gap in rem per density (Svelte content-gap override).
fn content_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.25,
        ControlDensity::Default => 0.375,
        ControlDensity::Comfortable => 0.5,
    }
}

/// File-item padding in rem per density (Svelte item-padding override).
fn item_padding_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.625,
        ControlDensity::Default => 0.75,
        ControlDensity::Comfortable => 0.875,
    }
}

/// Build a Jetstream file upload element from a FileUploadSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Root .file-upload]  <div>
///   ├── [Dropzone .file-upload__dropzone]  <div>
///   │     └── [Dropzone Content .file-upload__dropzone-content]
///   │           ├── [Icon .file-upload__icon]  <svg>
///   │           ├── [Label .file-upload__label]  <p> → [Browse]
///   │           └── [Hint .file-upload__hint]  <p>
///   └── [File List .file-upload__list]  <ul> (conditional: files.len() > 0)
///         └── [File Item .file-upload__item]  <li> (repeated)
///               ├── [Preview]  OR  [File Icon]
///               ├── [Meta] → [Name] + [Size / Error Text]
///               ├── [Remove]  <button>
///               └── [Progress] → [Progress Bar] (uploading only)
/// ```
pub fn js_file_upload(spec: &FileUploadSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let border_color: Color = resolve_color(theme, spec.border_token()).into();
    let text_secondary: Color = resolve_color(theme, "color.text.secondary").into();
    let text_tertiary: Color = resolve_color(theme, "color.text.tertiary").into();
    let accent: Color = resolve_color(theme, "color.accent.base").into();
    let radius = resolve_radius(theme, spec.radius_token());

    // ── Sizing (contract §8 size table) ──
    let min_height = rem_to_px(dropzone_min_height_rem(effective_size));
    let icon_sz = rem_to_px(icon_size_rem(effective_size));
    let label_font = rem_to_px(label_font_rem(effective_size));
    let hint_font = rem_to_px(hint_font_rem(effective_size));
    let padding = rem_to_px(dropzone_padding_rem(spec.density));
    let content_gap = rem_to_px(content_gap_rem(spec.density));
    let root_gap = rem_to_px(0.5); // space.stack.sm
    let border_width = rem_to_px(0.125); // 0.125rem dashed

    // ── Dropzone content ──
    let icon = ui_element::icon("upload")
        .w(icon_sz)
        .h(icon_sz)
        .text_color(text_secondary);

    // Label "Drop files here or browse" — the accent "browse" affordance is
    // a separate accent-colored run after the prompt (contract Browse part).
    let label_row = ui_element::div()
        .flex_row()
        .gap(rem_to_px(0.25))
        .items_center()
        .justify_center()
        .child(
            ui_element::label("Drop files here or")
                .text_size(label_font)
                .text_color(text_secondary),
        )
        .child(
            ui_element::label("browse")
                .text_size(label_font)
                .text_color(accent),
        );

    let mut dropzone_content = ui_element::div()
        .flex_col()
        .items_center()
        .gap(content_gap)
        .text_align_center()
        .child(icon)
        .child(label_row);

    // Hint: `accept · Max <size>` (contract / Svelte copy).
    let hint_text = build_hint_text(spec);
    if let Some(hint_text) = hint_text {
        dropzone_content = dropzone_content.child(
            ui_element::label(&hint_text)
                .text_size(hint_font)
                .text_color(text_tertiary),
        );
    }

    // ── Dropzone ──
    let mut dropzone = ui_element::div()
        .min_h(min_height)
        .p(padding)
        .rounded(radius)
        .bg(Color::TRANSPARENT) // contract: transparent default
        .border(border_width)
        .border_color(border_color)
        .flex_row()
        .items_center()
        .justify_center()
        .focusable()
        .child(dropzone_content);

    // Drag-active: accent border, accent @ 8% tint (contract dropzone--active).
    if spec.is_dragging {
        let drag_tint = tint(resolve_color(theme, "color.accent.base"), 0.08);
        dropzone = dropzone.border_color(accent).bg(drag_tint);
    }

    // Hover: border-focus + panel mixed 50% with transparent.
    if !spec.is_disabled {
        let panel = resolve_color(theme, "color.background.panel");
        let hover_bg: Color = color_mix(panel, glam::Vec4::new(0.0, 0.0, 0.0, 0.0), 0.5).into();
        let focus_border: Color = resolve_color(theme, spec.focus_border_token()).into();
        dropzone = dropzone
            .hover(move |s| s.bg(hover_bg).border_color(focus_border))
            .cursor_pointer();
    }

    // ── Root ──
    let mut root = ui_element::div()
        .flex_col()
        .gap(root_gap)
        .self_stretch()
        .child(dropzone);

    // ── Validation error line (below dropzone) ──
    if let Some(ref err) = spec.validation_error {
        let danger: Color = resolve_color(theme, spec.error_color_token()).into();
        root = root.child(
            ui_element::label(err)
                .text_size(rem_to_px(0.75))
                .text_color(danger),
        );
    }

    // ── File list (conditional: files.len() > 0) ──
    if spec.has_files() {
        let mut list = ui_element::div().flex_col().gap(rem_to_px(0.5));
        for item in &spec.files {
            list = list.child(file_item(spec, item, theme));
        }
        root = root.child(list);
    }

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        root = root.opacity(opacity).disabled(true);
    }

    root
}

/// Build a single file-list row (File Item anatomy).
fn file_item(spec: &FileUploadSpec, item: &FileUploadItem, theme: &JetstreamThemeProvider) -> JsEl {
    let item_radius = resolve_radius(theme, spec.item_radius_token());
    let panel = resolve_color(theme, spec.item_fill_token());
    let surface = resolve_color(theme, spec.item_surface_token());
    let danger = resolve_color(theme, spec.item_error_token());
    let icon_color: Color = resolve_color(theme, spec.item_icon_token()).into();
    let name_color: Color = resolve_color(theme, spec.item_name_token()).into();
    let size_color: Color = resolve_color(theme, spec.item_size_token()).into();
    let accent: Color = resolve_color(theme, spec.progress_fill_token()).into();

    // surface @ 82% for file-icon / progress-track backgrounds.
    let surface_mix: Color = color_mix(surface, glam::Vec4::new(0.0, 0.0, 0.0, 0.0), 0.82).into();
    // Error item background = danger @ 10% over panel.
    let item_bg: Color = if item.is_error() {
        color_mix(danger, panel, 0.10).into()
    } else {
        panel.into()
    };

    // ── Preview / file icon (2rem square) ──
    let leading = if item.has_preview && spec.show_preview {
        // Preview bitmap is host-owned; render the framed surface placeholder.
        ui_element::div()
            .w(rem_to_px(2.0))
            .h(rem_to_px(2.0))
            .rounded(rem_to_px(0.375))
            .bg(surface_mix)
            .object_fit_cover()
    } else {
        ui_element::div()
            .w(rem_to_px(2.0))
            .h(rem_to_px(2.0))
            .rounded(rem_to_px(0.375))
            .bg(surface_mix)
            .flex_col()
            .items_center()
            .justify_center()
            .child(
                ui_element::icon("file")
                    .w(rem_to_px(1.25))
                    .h(rem_to_px(1.25))
                    .text_color(icon_color),
            )
    };

    // ── Meta: name + size/error ──
    let size_line = if item.is_error() {
        let danger_text: Color = danger.into();
        match &item.error {
            Some(err) => format!("{} · {}", item.formatted_size(), err),
            None => item.formatted_size(),
        }
        .pipe(|text| {
            ui_element::label(&text)
                .text_size(rem_to_px(0.8125))
                .text_color(danger_text)
        })
    } else {
        let suffix = match item.status {
            poodle_specs::FileUploadStatus::Uploading => format!(" · {}%", item.progress),
            poodle_specs::FileUploadStatus::Complete => " · Complete".to_string(),
            _ => String::new(),
        };
        ui_element::label(&format!("{}{}", item.formatted_size(), suffix))
            .text_size(rem_to_px(0.8125))
            .text_color(size_color)
    };

    let mut meta = ui_element::div()
        .flex_col()
        .gap(rem_to_px(0.125))
        .min_w_0()
        .grow()
        .child(
            ui_element::label(&item.name)
                .text_size(rem_to_px(0.875))
                .text_color(name_color)
                .whitespace_nowrap()
                .text_ellipsis(),
        )
        .child(size_line);

    // ── Progress track + bar (uploading only) ──
    if item.is_uploading() {
        meta = meta.child(progress_track(item.progress, surface_mix, accent));
    }

    // ── Remove button (1.75rem pill) ──
    let remove = ui_element::button("")
        .w(rem_to_px(1.75))
        .h(rem_to_px(1.75))
        .rounded(rem_to_px(999.0))
        .bg(Color::TRANSPARENT)
        .flex_col()
        .items_center()
        .justify_center()
        .focusable()
        .cursor_pointer()
        .hover(move |s| s.bg(surface_mix))
        .child(
            ui_element::icon("x")
                .w(rem_to_px(0.875))
                .h(rem_to_px(0.875))
                .text_color(icon_color),
        );

    ui_element::div()
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.75))
        .p(rem_to_px(item_padding_rem(spec.density)))
        .rounded(item_radius)
        .bg(item_bg)
        .self_stretch()
        .child(leading)
        .child(meta)
        .child(remove)
}

/// Progress track (0.25rem tall, pill) with a proportional accent fill.
///
/// JsEl widths are px, not %, so the fill is modelled as a flex pair: the
/// filled run grows `progress` parts and the remainder grows `100 - progress`
/// parts. Keeps the bar proportional and token-driven.
fn progress_track(progress: u8, track_bg: Color, fill: Color) -> JsEl {
    let p = progress.min(100) as f32;
    let mut bar = ui_element::div()
        .h_full()
        .bg(fill)
        .flex_basis(p)
        .flex_grow();
    if p <= 0.0 {
        bar = bar.flex_none().w(0.0);
    }
    let mut track = ui_element::div()
        .self_stretch()
        .h(rem_to_px(0.25))
        .rounded(rem_to_px(999.0))
        .bg(track_bg)
        .overflow_hidden()
        .flex_row()
        .child(bar);
    if p < 100.0 {
        let remainder = ui_element::div().h_full().flex_basis(100.0 - p).flex_grow();
        track = track.child(remainder);
    }
    track
}

/// Build dropzone hint: `accept · Max <size>` (contract / Svelte copy).
fn build_hint_text(spec: &FileUploadSpec) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(ref accept) = spec.accept {
        parts.push(accept.clone());
    }
    if let Some(max) = spec.max_size {
        parts.push(format!("Max {}", poodle_specs::format_file_size(max)));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" · "))
    }
}

/// Tiny `.pipe()` helper to keep the size-line construction readable.
trait Pipe: Sized {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}
impl Pipe for String {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{FileUploadItem, FileUploadStatus};

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
    fn dropzone_prompt_and_browse_render() {
        let tree = probe(&js_file_upload(&FileUploadSpec::new(), &theme()), 400.0, 200.0);
        assert!(
            tree.has_text("Drop files here or"),
            "dropzone prompt missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("browse"),
            "accent browse affordance missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn hint_shows_accept_and_max_size() {
        let spec = FileUploadSpec::new()
            .with_accept("image/*")
            .with_max_size(5 * 1024 * 1024);
        let tree = probe(&js_file_upload(&spec, &theme()), 400.0, 200.0);
        assert!(
            tree.has_text("image/* · Max 5.0 MB"),
            "hint copy missing: {:?}",
            tree.texts()
        );
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
        assert_ne!(
            normal_bg, dragging_bg,
            "Drag-active should change dropzone background"
        );
    }

    #[test]
    fn file_list_renders_name_size_and_progress() {
        let spec = FileUploadSpec::new()
            .with_file(
                FileUploadItem::new("a", "photo.png", 2048)
                    .with_preview(true)
                    .with_progress(40),
            )
            .with_file(FileUploadItem::new("b", "report.pdf", 5 * 1024 * 1024));
        let tree = probe(&js_file_upload(&spec, &theme()), 480.0, 320.0);

        // File names render.
        assert!(tree.has_text("photo.png"), "name 1 missing: {:?}", tree.texts());
        assert!(tree.has_text("report.pdf"), "name 2 missing: {:?}", tree.texts());
        // Uploading row shows size + progress percent.
        assert!(
            tree.has_text("2.0 KB · 40%"),
            "uploading size/progress missing: {:?}",
            tree.texts()
        );
        // Non-image row renders the file-icon glyph; remove button "x" present.
        assert!(tree.texts().contains(&"file"), "file icon missing: {:?}", tree.texts());
        assert!(tree.texts().contains(&"x"), "remove glyph missing: {:?}", tree.texts());
    }

    #[test]
    fn error_file_shows_error_text() {
        let spec = FileUploadSpec::new().with_file(
            FileUploadItem::new("c", "huge.zip", 99 * 1024 * 1024)
                .with_error("Exceeds size limit"),
        );
        let tree = probe(&js_file_upload(&spec, &theme()), 480.0, 200.0);
        assert!(
            tree.texts().iter().any(|t| t.contains("Exceeds size limit")),
            "error text missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn validation_error_line_renders() {
        let spec = FileUploadSpec::new().with_validation_error("Only images allowed");
        let tree = probe(&js_file_upload(&spec, &theme()), 400.0, 200.0);
        assert!(
            tree.has_text("Only images allowed"),
            "validation error missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn complete_file_shows_complete_label() {
        let spec = FileUploadSpec::new().with_file(
            FileUploadItem::new("d", "done.txt", 1024).with_status(FileUploadStatus::Complete),
        );
        let tree = probe(&js_file_upload(&spec, &theme()), 480.0, 200.0);
        assert!(
            tree.texts().iter().any(|t| t.contains("Complete")),
            "complete label missing: {:?}",
            tree.texts()
        );
    }
}
