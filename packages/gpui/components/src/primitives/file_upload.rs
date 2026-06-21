//! FileUpload — real GPUI component backed by FileUploadSpec.
//!
//! Contract: `docs/contracts/components/file-upload.md`
//! Reference: `packages/svelte/components/src/FileUpload.svelte`
//!
//! Dropzone (dashed border, size-driven min-height, panel padding) + the
//! conditional file list (item: preview/file-icon + meta + progress + remove).
//! All visual properties resolve from tokens / contract-exact rem.
//!
//! Preview-loop: native file dialog + platform drag-and-drop are runtime-owned
//! (contract §10). Image preview bitmaps are host-owned — `has_preview` only
//! drives which leading anatomy part renders.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, FileUploadItem, FileUploadSpec, FileUploadStatus, IconSpec,
    SemanticControlSizeRole,
};

use super::icon::Icon;
use crate::presentation::{panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Dropzone min-height in rem per size (contract §8 size table).
fn dropzone_min_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 5.0,
        ControlSize::Sm => 6.0,
        ControlSize::Md => 8.0,
        ControlSize::Lg => 10.0,
        ControlSize::Xl => 12.0,
    }
}

/// Upload icon size in rem per size (contract §8 size table).
fn icon_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm | ControlSize::Md | ControlSize::Lg => 2.0,
        ControlSize::Xl => 2.5,
    }
}

/// Label font size in rem per size (contract §8 size table).
fn label_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm | ControlSize::Md => 0.875,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Hint font size in rem per size (contract §8 size table).
fn hint_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        _ => 0.8125,
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

/// Human-readable byte size matching Svelte's `formatFileSize`.
fn format_size(bytes: u64) -> String {
    poodle_specs::format_file_size(bytes)
}

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
    pub fn files(mut self, v: Vec<FileUploadItem>) -> Self {
        self.spec.files = v;
        self
    }
    pub fn file(mut self, v: FileUploadItem) -> Self {
        self.spec.files.push(v);
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
        let label_font = px(rem_to_px(label_font_rem(effective_size)));
        let hint_font = px(rem_to_px(hint_font_rem(effective_size)));
        let icon_px = rem_to_px(icon_size_rem(effective_size));
        // Panel padding from density (contract §8).
        let panel_padding_x = px(rem_to_px(panel_space_x_rem(spec.density)));
        let panel_padding_y = px(rem_to_px(panel_space_y_rem(spec.density)));
        let stack_gap = resolve_px(theme, "space.stack.sm");
        let content_gap = resolve_px(theme, "space.inline.xs");
        let dropzone_min_h = px(rem_to_px(dropzone_min_height_rem(effective_size)));
        let dropzone_radius = resolve_radius(theme, spec.radius_token());

        let panel_bg = resolve_color(theme, "color.background.panel");
        let border = resolve_color(theme, spec.border_token());
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_tertiary = resolve_color(theme, "color.text.tertiary");
        let accent = resolve_color(theme, "color.accent.base");
        let focus_border = resolve_color(theme, spec.focus_border_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let transparent = gpui::transparent_black();

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-file-upload-{}", suffix)
        } else {
            "poodle-file-upload".to_string()
        };

        // ── Dropzone content: icon + label(+browse) + hint ──
        let upload_icon = Icon::from_spec(IconSpec::new("upload"), theme)
            .with_px_size(icon_px)
            .with_color(text_secondary);

        // Label "Drop files here or browse" — the accent underlined "browse"
        // run is a separate inline span (contract Browse part).
        let (prompt, show_browse) = if spec.is_dragging {
            ("Drop to upload", false)
        } else {
            ("Drop files here or", true)
        };
        let mut label_row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(rem_to_px(0.25)))
            .text_size(label_font)
            .text_color(text_secondary)
            .child(prompt);
        if show_browse {
            label_row = label_row.child(div().text_color(accent).underline().child("browse"));
        }

        let mut dropzone_content = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(content_gap)
            .text_center()
            .child(upload_icon)
            .child(label_row);

        if let Some(hint) = build_hint_text(spec) {
            dropzone_content = dropzone_content.child(
                div()
                    .text_size(hint_font)
                    .text_color(text_tertiary)
                    .child(hint),
            );
        }

        // ── Dropzone (min-height per size, 0.125rem dashed) ──
        // Active: accent border + accent @ 8% over transparent.
        let active_bg = color_mix(accent, transparent, 0.08);
        let (zone_bg, zone_border) = if spec.is_dragging {
            (active_bg, accent)
        } else {
            (transparent, border)
        };
        // Hover: panel mixed 50% with transparent.
        let hover_bg = color_mix(panel_bg, transparent, 0.50);

        let mut zone = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w_full()
            .min_h(dropzone_min_h)
            .rounded(dropzone_radius)
            .bg(zone_bg)
            .border_2() // 0.125rem dashed
            .border_dashed()
            .border_color(zone_border)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(stack_gap)
            .px(panel_padding_x)
            .py(panel_padding_y)
            .hover(move |s| s.bg(hover_bg).border_color(focus_border))
            .focus(move |s| s.border_color(focus_border))
            .child(dropzone_content);

        if spec.is_disabled {
            zone = zone.cursor(CursorStyle::OperationNotAllowed);
        } else {
            zone = zone.cursor_pointer();
        }

        // ── Root column: dropzone + optional error line + file list ──
        let mut root = div().flex().flex_col().gap(stack_gap).child(zone);

        // Validation error line (status.danger, caption size).
        if let Some(ref err) = spec.validation_error {
            let danger = resolve_color(theme, spec.error_color_token());
            root = root.child(
                div()
                    .text_size(hint_font)
                    .text_color(danger)
                    .child(err.clone()),
            );
        }

        // File list (conditional: files.len() > 0).
        if spec.has_files() {
            let mut list = div().flex().flex_col().gap(px(rem_to_px(0.5)));
            for item in &spec.files {
                list = list.child(file_item(spec, item, theme, effective_size));
            }
            root = root.child(list);
        }

        if spec.is_disabled {
            root = root.opacity(disabled_opacity);
        }

        root.into_any_element()
    }
}

/// Build a single file-list row (File Item anatomy).
fn file_item(
    spec: &FileUploadSpec,
    item: &FileUploadItem,
    theme: &GpuiThemeProvider,
    _size: ControlSize,
) -> AnyElement {
    let item_radius = resolve_radius(theme, spec.item_radius_token());
    let panel = resolve_color(theme, spec.item_fill_token());
    let surface = resolve_color(theme, spec.item_surface_token());
    let danger = resolve_color(theme, spec.item_error_token());
    let icon_color = resolve_color(theme, spec.item_icon_token());
    let name_color = resolve_color(theme, spec.item_name_token());
    let size_color = resolve_color(theme, spec.item_size_token());
    let accent = resolve_color(theme, spec.progress_fill_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let transparent = gpui::transparent_black();

    // surface @ 82% over transparent (file-icon / progress-track / remove-hover).
    let surface_mix = color_mix(surface, transparent, 0.82);
    // Error item background = danger @ 10% over panel.
    let item_bg = if item.is_error() {
        color_mix(danger, panel, 0.10)
    } else {
        panel
    };

    // ── Leading: preview surface OR file icon (2rem square) ──
    let leading: AnyElement = if item.has_preview && spec.show_preview {
        div()
            .w(px(rem_to_px(2.0)))
            .h(px(rem_to_px(2.0)))
            .rounded(px(rem_to_px(0.375)))
            .bg(surface_mix)
            .into_any_element()
    } else {
        div()
            .w(px(rem_to_px(2.0)))
            .h(px(rem_to_px(2.0)))
            .rounded(px(rem_to_px(0.375)))
            .bg(surface_mix)
            .flex()
            .items_center()
            .justify_center()
            .child(
                Icon::from_spec(IconSpec::new("file"), theme)
                    .with_px_size(rem_to_px(1.25))
                    .with_color(icon_color),
            )
            .into_any_element()
    };

    // ── Meta: name + size/status (or error) ──
    let size_text = if item.is_error() {
        match &item.error {
            Some(err) => format!("{} · {}", format_size(item.size), err),
            None => format_size(item.size),
        }
    } else {
        let suffix = match item.status {
            FileUploadStatus::Uploading => format!(" · {}%", item.progress),
            FileUploadStatus::Complete => " · Complete".to_string(),
            _ => String::new(),
        };
        format!("{}{}", format_size(item.size), suffix)
    };
    let size_color = if item.is_error() { danger } else { size_color };

    let mut meta = div()
        .flex()
        .flex_col()
        .gap(px(rem_to_px(0.125)))
        .min_w_0()
        .flex_grow()
        .child(
            div()
                .text_size(px(rem_to_px(0.875)))
                .text_color(name_color)
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(item.name.clone()),
        )
        .child(
            div()
                .text_size(px(rem_to_px(0.8125)))
                .text_color(size_color)
                .child(size_text),
        );

    // ── Progress track + bar (uploading only) ──
    if item.is_uploading() {
        let frac = (item.progress.min(100) as f32) / 100.0;
        meta = meta.child(
            div()
                .w_full()
                .h(px(rem_to_px(0.25)))
                .rounded(px(999.0))
                .bg(surface_mix)
                .overflow_hidden()
                .child(
                    div()
                        .h_full()
                        .w(relative(frac))
                        .bg(accent),
                ),
        );
    }

    // ── Remove button (1.75rem pill) ──
    let remove = div()
        .id(SharedString::from(format!("file-upload-remove-{}", item.id)))
        .flex()
        .items_center()
        .justify_center()
        .w(px(rem_to_px(1.75)))
        .h(px(rem_to_px(1.75)))
        .rounded(px(999.0))
        .bg(transparent)
        .text_color(icon_color)
        .cursor_pointer()
        .hover(move |s| s.bg(surface_mix).text_color(text_primary))
        .child(
            Icon::from_spec(IconSpec::new("x"), theme)
                .with_px_size(rem_to_px(0.875))
                .with_color(icon_color),
        );

    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(rem_to_px(0.75)))
        .p(px(rem_to_px(item_padding_rem(spec.density))))
        .rounded(item_radius)
        .bg(item_bg)
        .child(leading)
        .child(meta)
        .child(remove)
        .into_any_element()
}

/// Build dropzone hint: `accept · Max <size>` (contract / Svelte copy).
fn build_hint_text(spec: &FileUploadSpec) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(ref accept) = spec.accept {
        parts.push(accept.clone());
    }
    if let Some(max) = spec.max_size {
        parts.push(format!("Max {}", format_size(max)));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" · "))
    }
}
