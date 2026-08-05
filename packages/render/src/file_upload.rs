//! FileUpload — a drag-and-drop upload zone and its file list.
//!
//! Contract: `docs/contracts/components/file-upload.md`
//! Ported from: `packages/jetstream/components/src/file_upload.rs`.
//!
//! Anatomy (from contract):
//! ```text
//! [Root]
//!   ├── [Dropzone]  dashed border, icon + "Drop files here or browse" + hint
//!   └── [File List]  (conditional: files.len() > 0)
//!         └── [File Item]  preview-or-icon + name + size/error + remove
//!               └── [Progress] (uploading only)
//! ```
//!
//! Actual drag/drop + native file-dialog interaction is host-owned. Image
//! preview bitmaps are host-owned — `FileUploadItem::has_preview` only drives
//! which anatomy part (preview surface vs file icon) renders. `on_remove`
//! carries the removed file's name — the identity the list itself displays.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, StylePatch, TextAlign,
};
use poodle_specs::{ControlDensity, ControlSize, FileUploadItem, FileUploadSpec};

use crate::color::{mix_srgb, with_alpha, TRANSPARENT};
use crate::presentation::{rem_to_px, resolve_semantic_size};

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

fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

pub fn file_upload(
    spec: &FileUploadSpec,
    theme: &dyn ThemeProvider,
    on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let border_color = theme.resolve_color(spec.border_token());
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_tertiary = theme.resolve_color("color.text.tertiary");
    let accent = theme.resolve_color("color.accent.base");
    let radius = theme.resolve_radius(spec.radius_token());

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
    let mut icon = Node::icon("upload", icon_sz);
    icon.style.descriptor.text_color = Some(text_secondary);

    // Label "Drop files here or browse" — the accent "browse" affordance is
    // a separate accent-colored run after the prompt (contract Browse part).
    let mut label_row = Node::container();
    {
        let s = &mut label_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.spacing.gap = rem_to_px(0.25);
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    let mut prompt = Node::text("Drop files here or");
    prompt.style.text_size = Some(label_font);
    prompt.style.descriptor.text_color = Some(text_secondary);
    let mut browse = Node::text("browse");
    browse.style.text_size = Some(label_font);
    browse.style.descriptor.text_color = Some(accent);
    let label_row = label_row.child(prompt).child(browse);

    let mut dropzone_content = Node::container();
    {
        let s = &mut dropzone_content.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = content_gap;
        s.text_align = Some(TextAlign::Center);
    }
    let mut dropzone_content = dropzone_content.child(icon).child(label_row);

    // Hint: `accept · Max <size>` (contract / Svelte copy).
    if let Some(hint_text) = build_hint_text(spec) {
        let mut hint = Node::text(&hint_text);
        hint.style.text_size = Some(hint_font);
        hint.style.descriptor.text_color = Some(text_tertiary);
        dropzone_content = dropzone_content.child(hint);
    }

    // ── Dropzone ──
    let mut dropzone = Node::container();
    {
        let s = &mut dropzone.style;
        s.min_height = Some(min_height);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = padding;
        pad.right = padding;
        pad.top = padding;
        pad.bottom = padding;
        s.descriptor.background = Some(TRANSPARENT); // contract: transparent default
        s.descriptor.border.width = border_width;
        s.border_dashed = true; // contract: 0.125rem dashed
        s.descriptor.border.color = border_color;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    all_corners(&mut dropzone, radius);
    dropzone.interaction.focusable = true;

    // Drag-active: accent border, accent @ 8% tint (contract dropzone--active).
    if spec.is_dragging {
        dropzone.style.descriptor.border.color = accent;
        dropzone.style.descriptor.background = Some(with_alpha(accent, accent.3 * 0.08));
    }

    // Hover: border-focus + panel mixed 50% with transparent.
    if !spec.is_disabled {
        let panel = theme.resolve_color("color.background.panel");
        let hover_bg = mix_srgb(panel, TRANSPARENT, 0.5);
        let focus_border = theme.resolve_color(spec.focus_border_token());
        dropzone.style.hover = Some(StylePatch {
            background: Some(hover_bg),
            border_color: Some(focus_border),
            text_color: None,
        });
        dropzone.style.descriptor.cursor = CursorHint::Pointer;
    }
    let dropzone = dropzone.child(dropzone_content);

    // ── Root ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
        s.self_stretch = true;
    }
    let mut root = root.child(dropzone);

    // ── Validation error line (below dropzone) ──
    if let Some(ref err) = spec.validation_error {
        let danger = theme.resolve_color(spec.error_color_token());
        let mut line = Node::text(err);
        line.style.text_size = Some(rem_to_px(0.75));
        line.style.descriptor.text_color = Some(danger);
        root = root.child(line);
    }

    // ── File list (conditional: files.len() > 0) ──
    if spec.has_files() {
        let mut list = Node::container();
        {
            let s = &mut list.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.5);
        }
        for item in &spec.files {
            list = list.child(file_item(spec, item, theme, on_remove.as_ref()));
        }
        root = root.child(list);
    }

    // ── Disabled state ──
    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        root.interaction.disabled = true;
    }

    root
}

/// Build a single file-list row (File Item anatomy).
fn file_item(
    spec: &FileUploadSpec,
    item: &FileUploadItem,
    theme: &dyn ThemeProvider,
    on_remove: Option<&Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let item_radius = theme.resolve_radius(spec.item_radius_token());
    let panel = theme.resolve_color(spec.item_fill_token());
    let surface = theme.resolve_color(spec.item_surface_token());
    let danger = theme.resolve_color(spec.item_error_token());
    let icon_color = theme.resolve_color(spec.item_icon_token());
    let name_color = theme.resolve_color(spec.item_name_token());
    let size_color = theme.resolve_color(spec.item_size_token());
    let accent = theme.resolve_color(spec.progress_fill_token());

    // surface @ 82% for file-icon / progress-track backgrounds.
    let surface_mix = mix_srgb(surface, TRANSPARENT, 0.82);
    // Error item background = danger @ 10% over panel.
    let item_bg = if item.is_error() {
        mix_srgb(danger, panel, 0.10)
    } else {
        panel
    };

    // ── Preview / file icon (2rem square) ──
    let leading = {
        let mut square = Node::container();
        {
            let s = &mut square.style;
            // Explicit Row default not needed here: the icon variant centers
            // in a Column like the old tier; the preview variant is empty.
            s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(2.0));
            s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(2.0));
            s.descriptor.background = Some(surface_mix);
        }
        all_corners(&mut square, rem_to_px(0.375));
        if item.has_preview && spec.show_preview {
            // Preview bitmap is host-owned; the framed surface placeholder.
            // Old tier leaves the div at default Row direction here.
            square.style.descriptor.layout.direction = LayoutDirection::Row;
            square
        } else {
            let s = &mut square.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            let mut glyph = Node::icon("file", rem_to_px(1.25));
            glyph.style.descriptor.text_color = Some(icon_color);
            square.child(glyph)
        }
    };

    // ── Meta: name + size/error ──
    let size_line = if item.is_error() {
        let text = match &item.error {
            Some(err) => format!("{} · {}", item.formatted_size(), err),
            None => item.formatted_size(),
        };
        let mut line = Node::text(&text);
        line.style.text_size = Some(rem_to_px(0.8125));
        line.style.descriptor.text_color = Some(danger);
        line
    } else {
        let suffix = match item.status {
            poodle_specs::FileUploadStatus::Uploading => format!(" · {}%", item.progress),
            poodle_specs::FileUploadStatus::Complete => " · Complete".to_string(),
            _ => String::new(),
        };
        let mut line = Node::text(&format!("{}{}", item.formatted_size(), suffix));
        line.style.text_size = Some(rem_to_px(0.8125));
        line.style.descriptor.text_color = Some(size_color);
        line
    };

    let mut meta = Node::container();
    {
        let s = &mut meta.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.min_width = Some(0.0);
        s.descriptor.layout.width = LayoutSizing::Grow;
    }
    let mut name = Node::text(&item.name);
    name.style.text_size = Some(rem_to_px(0.875));
    name.style.descriptor.text_color = Some(name_color);
    name.style.no_wrap = true;
    name.style.text_ellipsis = true;
    let mut meta = meta.child(name).child(size_line);

    // ── Progress track + bar (uploading only) ──
    if item.is_uploading() {
        meta = meta.child(progress_track(item.progress, surface_mix, accent));
    }

    // ── Remove button (1.75rem pill) ──
    let mut remove = Node::button("");
    // Svelte names each remove button after the file it removes, so a list
    // of them is not a row of identical "remove" buttons.
    remove.a11y.label = Some(format!("Remove {}", item.name));
    {
        let s = &mut remove.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(1.75));
        s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(1.75));
        s.descriptor.background = Some(TRANSPARENT);
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.cursor = CursorHint::Pointer;
        s.hover = Some(StylePatch {
            background: Some(surface_mix),
            border_color: None,
            text_color: None,
        });
    }
    all_corners(&mut remove, rem_to_px(999.0));
    remove.interaction.focusable = true;
    let mut x = Node::icon("x", rem_to_px(0.875));
    x.style.descriptor.text_color = Some(icon_color);
    let mut remove = remove.child(x);

    if let Some(handler) = on_remove {
        let handler = Arc::clone(handler);
        let name = item.name.clone();
        remove.interaction.on_activate = Some(Arc::new(move || handler(&name)));
    }

    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.75);
        let p = rem_to_px(item_padding_rem(spec.density));
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = p;
        pad.right = p;
        pad.top = p;
        pad.bottom = p;
        s.descriptor.background = Some(item_bg);
        s.self_stretch = true;
    }
    all_corners(&mut row, item_radius);
    row.child(leading).child(meta).child(remove)
}

/// Progress track (0.25rem tall, pill) with a proportional accent fill.
///
/// Widths are px, not %, so the fill is modelled as a flex pair: the filled
/// run grows `progress` parts and the remainder grows `100 - progress`
/// parts. Keeps the bar proportional and token-driven.
fn progress_track(progress: u8, track_bg: ColorValue, fill: ColorValue) -> Node {
    let p = progress.min(100) as f32;
    let mut bar = Node::container();
    {
        let s = &mut bar.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.fill_height = true;
        s.descriptor.background = Some(fill);
        s.flex_basis = Some(p);
        s.flex_fill = true;
    }
    if p <= 0.0 {
        let s = &mut bar.style;
        // Old tier: flex_basis(0) + flex_grow() (which zeroes min-size) then
        // flex_none + w(0) — net grow 0, shrink 0, basis 0, min 0, width 0.
        s.flex_fill = false;
        s.flex_none = true;
        s.min_width = Some(0.0);
        s.min_height = Some(0.0);
        s.descriptor.layout.width = LayoutSizing::Fixed(0.0);
    }
    let mut track = Node::container();
    {
        let s = &mut track.style;
        s.self_stretch = true;
        s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.25));
        s.descriptor.background = Some(track_bg);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.descriptor.layout.direction = LayoutDirection::Row;
    }
    all_corners(&mut track, rem_to_px(999.0));
    let mut track = track.child(bar);
    if p < 100.0 {
        let mut remainder = Node::container();
        {
            let s = &mut remainder.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.fill_height = true;
            s.flex_basis = Some(100.0 - p);
            s.flex_fill = true;
        }
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
