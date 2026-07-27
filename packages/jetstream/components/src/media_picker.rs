//! MediaPicker — Jetstream media picker backed by MediaPickerSpec.
//!
//! Contract: `docs/contracts/components/media-picker.md`
//! Reference: `packages/svelte/components/src/MediaPicker.svelte`
//!
//! Single-select, select-and-close model (no multi-select / confirm footer).
//! The browse grid renders the real `spec.items`; the upload tab composes the
//! real `js_file_upload` dropzone. Thumbnail bitmaps are host-owned
//! (preview-loop) — `MediaPickerItem::has_thumbnail` only drives the
//! placeholder-vs-image anatomy split.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, FileUploadSpec, MediaPickerItem, MediaPickerSpec};

use crate::file_upload::js_file_upload;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Thumbnail square size in rem per size (contract §8 size table).
fn thumb_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 3.5,
        ControlSize::Sm => 4.25,
        ControlSize::Md => 4.5,
        ControlSize::Lg => 5.0,
        ControlSize::Xl => 5.5,
    }
}

/// Browse-grid gap + item padding in rem per density (contract §8).
fn grid_gap_and_pad_rem(density: ControlDensity) -> (f32, f32) {
    match density {
        ControlDensity::Compact => (0.25, 0.25),
        ControlDensity::Default => (0.375, 0.375),
        ControlDensity::Comfortable => (0.5, 0.5),
    }
}

pub fn js_media_picker(spec: &MediaPickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let label_size = resolve_px(theme, "typography.label.size");

    let fill = resolve_color(theme, spec.fill_token());
    let radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let border = resolve_color(theme, "color.border.default");
    let ctrl_radius = resolve_radius(theme, spec.item_radius_token());
    let gap = resolve_px(theme, "space.stack.sm");

    // ── Root: dialog body content ──
    let mut root = ui_element::div()
        .bg(fill)
        .rounded(radius)
        .flex_col()
        .gap(gap)
        .min_h(rem_to_px(20.0));

    // ── Title ──
    root = root.child(
        ui_element::label(&spec.title)
            .text_color(text_primary)
            .text_size(rem_to_px(1.0))
            .text_weight(600),
    );

    // ── Tabs: Browse | Upload (active reflects spec.active_tab) ──
    let browsing = spec.is_browsing();
    let tab = |text: &str, active: bool| -> JsEl {
        let mut el = ui_element::button(text)
            .text_size(font_size)
            .focusable()
            .px(rem_to_px(0.5))
            .pb(rem_to_px(0.25));
        if active {
            el = el.text_color(accent).border_b_2().border_color(accent);
        } else {
            el = el.text_color(text_secondary);
        }
        el
    };
    root = root.child(
        ui_element::div()
            .flex_row()
            .gap(rem_to_px(0.5))
            .child(tab("Browse", browsing))
            .child(tab("Upload", !browsing)),
    );

    if browsing {
        // ── Search field (contract §2 search; a real TextInput) ──
        root = root.child(
            ui_element::div()
                .mt(rem_to_px(0.25))
                .child(
                    ui_element::text_input("", "Search media...")
                        // A placeholder is not an accessible name.
                        .aria_label("Search media")
                        .self_stretch()
                        .border(1.0)
                        .border_color(border)
                        .rounded(ctrl_radius)
                        .px(rem_to_px(0.5))
                        .py(rem_to_px(0.25))
                        .text_size(font_size)
                        .text_color(text_secondary),
                ),
        );

        // ── Grid OR empty state ──
        if spec.has_items() {
            let (grid_gap, item_pad) = grid_gap_and_pad_rem(spec.density);
            let thumb_size = rem_to_px(thumb_size_rem(effective_size));
            // Contract: the media grid is a `listbox` of selectable `option`s.
            let mut grid = ui_element::div()
                .aria_role(jetstream_ui::accesskit::Role::ListBox)
                .flex_row()
                .flex_wrap()
                .gap(rem_to_px(grid_gap))
                .max_h(rem_to_px(20.0))
                .overflow_scroll();
            for item in &spec.items {
                grid = grid.child(grid_item(item, spec, theme, thumb_size, item_pad, label_size));
            }
            root = root.child(grid);
        } else {
            let empty_text = spec
                .empty_message
                .clone()
                .unwrap_or_else(|| "No media items found.".to_string());
            root = root.child(
                ui_element::div()
                    .min_h(rem_to_px(10.0))
                    .self_stretch()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .child(
                        ui_element::label(&empty_text)
                            .text_color(text_secondary)
                            .text_size(rem_to_px(0.875)),
                    ),
            );
        }
    } else {
        // ── Upload tab: compose the real FileUpload dropzone ──
        let mut upload_spec = FileUploadSpec::new()
            .with_multiple(true)
            .with_size(spec.size)
            .with_size_role(spec.size_role)
            .with_density(spec.density);
        if let Some(ref accept) = spec.accept {
            upload_spec = upload_spec.with_accept(accept.clone());
        }
        if let Some(max) = spec.max_file_size {
            upload_spec = upload_spec.with_max_size(max);
        }
        root = root.child(
            ui_element::div()
                .mt(rem_to_px(0.25))
                .self_stretch()
                .child(js_file_upload(&upload_spec, theme)),
        );
    }

    root
}

/// One selectable browse-grid item: thumbnail (image surface or placeholder)
/// + truncated label.
fn grid_item(
    item: &MediaPickerItem,
    spec: &MediaPickerSpec,
    theme: &JetstreamThemeProvider,
    thumb_size: f32,
    item_pad: f32,
    label_size: f32,
) -> JsEl {
    let panel = resolve_color(theme, spec.item_hover_fill_token());
    let item_radius = resolve_radius(theme, spec.item_radius_token());
    let label_color: Color = resolve_color(theme, spec.label_token()).into();
    let placeholder_color: Color = resolve_color(theme, spec.placeholder_icon_token()).into();
    let hover_fill: Color = panel.into();
    let hover_border: Color = resolve_color(theme, spec.item_border_token()).into();

    // Thumbnail: real bitmap is host-owned. With a thumbnail, paint the panel
    // surface frame (image overlay = preview-loop); without, render the
    // centered placeholder image glyph.
    let thumb = if item.has_thumbnail {
        ui_element::div()
            // Each tile is a selectable `option` of the media listbox.
            .aria_role(jetstream_ui::accesskit::Role::ListBoxOption)
            .w(thumb_size)
            .h(thumb_size)
            .rounded(rem_to_px(0.25))
            .bg(hover_fill)
            .object_fit_cover()
    } else {
        ui_element::div()
            .w(thumb_size)
            .h(thumb_size)
            .rounded(rem_to_px(0.25))
            .bg(hover_fill)
            .flex_col()
            .items_center()
            .justify_center()
            .child(
                ui_element::icon("image")
                    .w(rem_to_px(1.5))
                    .h(rem_to_px(1.5))
                    .text_color(placeholder_color),
            )
    };

    ui_element::button(&item.label)
        .flex_col()
        .items_center()
        .gap(rem_to_px(0.25))
        .p(rem_to_px(item_pad))
        .border(1.0)
        .border_color(Color::TRANSPARENT)
        .rounded(item_radius)
        .bg(Color::TRANSPARENT)
        .focusable()
        .cursor_pointer()
        .hover(move |s| s.bg(hover_fill).border_color(hover_border))
        .child(thumb)
        .child(
            ui_element::label(&item.label)
                .text_size(label_size)
                .text_color(label_color)
                .max_w(thumb_size)
                .whitespace_nowrap()
                .text_ellipsis(),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{MediaKind, MediaPickerTab};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn sample_items() -> Vec<MediaPickerItem> {
        vec![
            MediaPickerItem::new("img-1", "Banner image", MediaKind::Image).with_thumbnail(true),
            MediaPickerItem::new("img-2", "Profile photo", MediaKind::Image).with_thumbnail(true),
            MediaPickerItem::new("doc-1", "Readme.pdf", MediaKind::Document),
            MediaPickerItem::new("vid-1", "Demo video", MediaKind::Video),
        ]
    }

    #[test]
    fn renders_title_and_tabs() {
        let spec = MediaPickerSpec::new("Select an asset").with_open(true);
        let tree = probe(&js_media_picker(&spec, &theme()), 480.0, 400.0);
        assert!(tree.has_text("Select an asset"), "title missing: {:?}", tree.texts());
        assert!(tree.has_text("Browse"), "browse tab missing: {:?}", tree.texts());
        assert!(tree.has_text("Upload"), "upload tab missing: {:?}", tree.texts());
    }

    #[test]
    fn browse_grid_renders_real_item_labels() {
        let spec = MediaPickerSpec::new("Pick")
            .with_open(true)
            .with_items(sample_items());
        let tree = probe(&js_media_picker(&spec, &theme()), 480.0, 480.0);
        // Real item labels (no fabricated "Item N").
        assert!(tree.has_text("Banner image"), "item 1 missing: {:?}", tree.texts());
        assert!(tree.has_text("Readme.pdf"), "item 3 missing: {:?}", tree.texts());
        assert!(
            !tree.texts().iter().any(|t| t.starts_with("Item ")),
            "fabricated placeholder items must not render: {:?}",
            tree.texts()
        );
        // Placeholder (non-thumbnail) items render the image glyph.
        assert!(
            tree.texts().contains(&"image"),
            "placeholder icon missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn empty_state_shows_message() {
        let spec = MediaPickerSpec::new("Pick")
            .with_open(true)
            .with_empty_message("Nothing here yet.");
        let tree = probe(&js_media_picker(&spec, &theme()), 480.0, 400.0);
        assert!(
            tree.has_text("Nothing here yet."),
            "empty message missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn empty_state_uses_default_message() {
        let spec = MediaPickerSpec::new("Pick").with_open(true);
        let tree = probe(&js_media_picker(&spec, &theme()), 480.0, 400.0);
        assert!(
            tree.has_text("No media items found."),
            "default empty message missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn upload_tab_renders_file_upload_dropzone() {
        let spec = MediaPickerSpec::new("Pick")
            .with_open(true)
            .with_active_tab(MediaPickerTab::Upload)
            .with_accept("image/*");
        let tree = probe(&js_media_picker(&spec, &theme()), 480.0, 400.0);
        // FileUpload dropzone prompt present; browse grid absent.
        assert!(
            tree.has_text("Drop files here or"),
            "upload dropzone missing on upload tab: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn no_fabricated_selection_count() {
        let spec = MediaPickerSpec::new("Pick")
            .with_open(true)
            .with_items(sample_items());
        let tree = probe(&js_media_picker(&spec, &theme()), 480.0, 480.0);
        assert!(
            !tree.texts().iter().any(|t| t.contains("selected")),
            "select-and-close model must not show a selection count: {:?}",
            tree.texts()
        );
    }
}
