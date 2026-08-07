//! MediaPreview — card with nested thumbnail, heading block, pill metadata.
//!
//! Contract: `docs/contracts/components/media-preview.md`
//! Ported from: `packages/jetstream/components/src/media_preview.rs`.
//!
//! Composes `card` (media slot) with `media_thumbnail`, header (eyebrow /
//! title / description + pill metadata), and body caption per §3/§10.

use poodle_adapter::ThemeProvider;
use poodle_node::{LayoutDirection, Node};
use poodle_specs::{CardSpec, MediaPreviewSpec, MediaThumbnailSpec};

use crate::card::card;
use crate::color::with_alpha;
use crate::presentation::rem_to_px;

pub fn media_preview(spec: &MediaPreviewSpec, theme: &dyn ThemeProvider) -> Node {
    media_preview_with_content(spec, theme, None)
}

/// Render a media preview with an optional caller-owned media slot.
pub fn media_preview_with_content(
    spec: &MediaPreviewSpec,
    theme: &dyn ThemeProvider,
    media_content: Option<Node>,
) -> Node {
    let text_primary = theme.resolve_color(spec.title_color_token());
    let text_secondary = theme.resolve_color(spec.secondary_text_token());
    let meta_fill_base = theme.resolve_color(spec.meta_fill_token());
    let meta_fill = with_alpha(meta_fill_base, meta_fill_base.3 * 0.70);
    let meta_radius = theme.resolve_radius(spec.meta_radius_token());

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
    let thumbnail =
        crate::media_thumbnail::media_thumbnail_with_content(&thumb_spec, theme, media_content);

    // ── Header: heading block + pill metadata ──────────────────
    let mut heading = Node::container();
    heading.style.descriptor.layout.direction = LayoutDirection::Column;
    heading.style.descriptor.layout.spacing.gap = section_gap;
    if let Some(ref eyebrow) = spec.eyebrow {
        let mut e = Node::text(eyebrow.to_uppercase());
        {
            let s = &mut e.style;
            s.descriptor.text_color = Some(text_secondary);
            s.text_size = Some(eyebrow_size);
            s.text_weight = Some(600);
            s.letter_spacing_em = Some(0.12); // contract Eyebrow: letter-spacing 0.12em
        }
        heading = heading.child(e);
    }
    let mut title = Node::text(&spec.title);
    title.style.descriptor.text_color = Some(text_primary);
    title.style.text_size = Some(title_size);
    title.style.text_weight = Some(600);
    heading = heading.child(title);
    if let Some(ref description) = spec.description {
        let mut d = Node::text(description);
        d.style.descriptor.text_color = Some(text_secondary);
        d.style.text_size = Some(body_size);
        heading = heading.child(d);
    }

    let mut header = Node::container();
    header.style.descriptor.layout.direction = LayoutDirection::Column;
    header.style.descriptor.layout.spacing.gap = header_gap;
    let mut header = header.child(heading);

    if spec.thumbnail_meta.is_some() || !spec.metadata.is_empty() {
        let mut meta_list = Node::container();
        {
            let s = &mut meta_list.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_wrap = true;
            s.descriptor.layout.spacing.gap = inline_gap;
        }
        let items = spec
            .thumbnail_meta
            .iter()
            .cloned()
            .chain(spec.metadata.iter().cloned());
        for item in items {
            let mut chip = Node::text(&item);
            {
                let s = &mut chip.style;
                s.descriptor.text_color = Some(text_secondary);
                s.text_size = Some(body_size);
                s.descriptor.background = Some(meta_fill);
                let c = &mut s.descriptor.corner_radii;
                c.top_left = meta_radius;
                c.top_right = meta_radius;
                c.bottom_right = meta_radius;
                c.bottom_left = meta_radius;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(meta_pad_x);
                pad.right = rem_to_px(meta_pad_x);
                pad.top = rem_to_px(meta_pad_y);
                pad.bottom = rem_to_px(meta_pad_y);
            }
            meta_list = meta_list.child(chip);
        }
        header = header.child(meta_list);
    }

    // ── Card children: [media, header, body?] ──────────────────
    let mut children = vec![thumbnail, header];
    if let Some(ref caption) = spec.caption {
        let mut body = Node::container();
        body.style.descriptor.layout.direction = LayoutDirection::Column;
        body.style.descriptor.layout.spacing.gap = section_gap;
        let mut c = Node::text(caption);
        c.style.descriptor.text_color = Some(text_secondary);
        c.style.text_size = Some(body_size);
        children.push(body.child(c));
    }

    let card_spec = CardSpec::new()
        .with_variant(spec.variant)
        .with_density(spec.resolved_density())
        .with_media(true)
        .with_aria_label(spec.title.clone());

    card(&card_spec, theme, children)
}
