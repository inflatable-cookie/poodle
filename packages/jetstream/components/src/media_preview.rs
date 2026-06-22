//! MediaPreview — Jetstream media preview backed by MediaPreviewSpec.
//!
//! Composes `js_card` (media slot) with a nested `js_media_thumbnail`, a header
//! (eyebrow / title / description + pill metadata), and a body (caption), per
//! the contract §3/§10 anatomy. Size and density resolve from the spec's
//! contract-exact rem tables.
use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CardSpec, MediaPreviewSpec, MediaThumbnailSpec};

use crate::card::js_card;
use crate::media_thumbnail::js_media_thumbnail;
use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_media_preview(spec: &MediaPreviewSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.secondary_text_token());
    let meta_fill_base = resolve_color(theme, spec.meta_fill_token());
    let meta_fill = Color::new(
        meta_fill_base.x,
        meta_fill_base.y,
        meta_fill_base.z,
        meta_fill_base.w * 0.70,
    );
    let meta_radius = resolve_radius(theme, spec.meta_radius_token());

    let eyebrow_size = rem_to_px(spec.eyebrow_size_rem());
    let title_size = rem_to_px(spec.title_size_rem());
    let body_size = rem_to_px(spec.body_size_rem());
    let (meta_pad_y, meta_pad_x) = spec.meta_padding_rem();
    let header_gap = rem_to_px(spec.header_gap_rem());
    let section_gap = rem_to_px(spec.section_gap_rem());
    let inline_gap = rem_to_px(0.375);

    // ── Media slot: nested MediaThumbnail (contract §3/§10) ────
    let mut thumb_spec = MediaThumbnailSpec::new(spec.kind)
        .with_state(spec.state)
        .with_aspect_ratio(spec.aspect_ratio)
        .with_show_caption(false);
    if let Some(ref badge) = spec.badge {
        thumb_spec = thumb_spec.with_badge_label(badge.clone());
    }
    if let Some(ref state_title) = spec.state_title {
        thumb_spec = thumb_spec.with_state_title(state_title.clone());
    }
    if let Some(ref state_message) = spec.state_message {
        thumb_spec = thumb_spec.with_state_message(state_message.clone());
    }
    let thumbnail = js_media_thumbnail(&thumb_spec, theme);

    // ── Header: heading block + pill metadata ──────────────────
    let mut heading = ui_element::div().flex_col().gap(section_gap);
    if let Some(ref eyebrow) = spec.eyebrow {
        heading = heading.child(
            ui_element::label(&eyebrow.to_uppercase())
                .text_color(text_secondary)
                .text_size(eyebrow_size)
                .text_weight(600)
                .letter_spacing_em(0.12), // contract Eyebrow: letter-spacing 0.12em
        );
    }
    heading = heading.child(
        ui_element::label(&spec.title)
            .text_color(text_primary)
            .text_size(title_size)
            .text_weight(600),
    );
    if let Some(ref description) = spec.description {
        heading = heading.child(
            ui_element::label(description)
                .text_color(text_secondary)
                .text_size(body_size),
        );
    }

    let mut header = ui_element::div().flex_col().gap(header_gap).child(heading);

    if spec.thumbnail_meta.is_some() || !spec.metadata.is_empty() {
        let mut meta_list = ui_element::div().flex_row().flex_wrap().gap(inline_gap);
        let items = spec
            .thumbnail_meta
            .iter()
            .cloned()
            .chain(spec.metadata.iter().cloned());
        for item in items {
            meta_list = meta_list.child(
                ui_element::label(&item)
                    .text_color(text_secondary)
                    .text_size(body_size)
                    .bg(meta_fill)
                    .rounded(meta_radius)
                    .pl(rem_to_px(meta_pad_x))
                    .pr(rem_to_px(meta_pad_x))
                    .pt(rem_to_px(meta_pad_y))
                    .pb(rem_to_px(meta_pad_y)),
            );
        }
        header = header.child(meta_list);
    }

    // ── Card children: [media, header, body?] ──────────────────
    let mut children = vec![thumbnail, header];
    if let Some(ref caption) = spec.caption {
        children.push(
            ui_element::div().flex_col().gap(section_gap).child(
                ui_element::label(caption)
                    .text_color(text_secondary)
                    .text_size(body_size),
            ),
        );
    }

    let card_spec = CardSpec::new()
        .with_variant(spec.variant)
        .with_density(spec.resolved_density())
        .with_media(true)
        .with_aria_label(spec.title.clone());

    js_card(&card_spec, theme, children)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{AspectRatio, MediaKind, MediaState};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_title_eyebrow_and_meta_chips() {
        let th = theme();
        let spec = MediaPreviewSpec::new(MediaKind::Image, "Hero banner")
            .with_eyebrow("Image")
            .with_description("Main landing page banner image.")
            .with_metadata(vec![
                "1920 x 1080".to_string(),
                "245 KB".to_string(),
                "PNG".to_string(),
            ]);
        let tree = probe(&js_media_preview(&spec, &th), 600.0, 600.0);

        assert!(!tree.is_empty());
        assert!(tree.has_text("Hero banner"), "title missing: {:?}", tree.texts());
        // Eyebrow uppercased.
        assert!(tree.has_text("IMAGE"), "eyebrow missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Main landing page banner image."),
            "description missing: {:?}",
            tree.texts()
        );
        // All three meta chips render.
        assert!(tree.has_text("1920 x 1080"));
        assert!(tree.has_text("245 KB"));
        assert!(tree.has_text("PNG"));
        // Nested thumbnail placeholder icon present (image fallback).
        assert!(tree.texts().contains(&"image"), "thumbnail icon missing: {:?}", tree.texts());
    }

    #[test]
    fn thumbnail_meta_prepends_to_meta_list() {
        let th = theme();
        let spec = MediaPreviewSpec::new(MediaKind::Image, "Cover")
            .with_thumbnail_meta("Waveform")
            .with_metadata(vec!["WAV".to_string()]);
        let tree = probe(&js_media_preview(&spec, &th), 600.0, 600.0);
        assert!(tree.has_text("Waveform"), "thumbnail_meta missing: {:?}", tree.texts());
        assert!(tree.has_text("WAV"));
    }

    #[test]
    fn error_state_renders_state_posture_in_frame() {
        let th = theme();
        let spec = MediaPreviewSpec::new(MediaKind::Document, "Corrupted file")
            .with_state(MediaState::Error)
            .with_state_title("Preview unavailable")
            .with_state_message("This file cannot be previewed.")
            .with_aspect_ratio(AspectRatio::Landscape);
        let tree = probe(&js_media_preview(&spec, &th), 600.0, 600.0);
        assert!(tree.has_text("Corrupted file"), "title missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Preview unavailable"),
            "state title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("This file cannot be previewed."),
            "state message missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn caption_renders_in_body() {
        let th = theme();
        let spec = MediaPreviewSpec::new(MediaKind::Image, "Banner")
            .with_caption("A short descriptive caption.");
        let tree = probe(&js_media_preview(&spec, &th), 600.0, 600.0);
        assert!(
            tree.has_text("A short descriptive caption."),
            "caption missing: {:?}",
            tree.texts()
        );
    }
}
