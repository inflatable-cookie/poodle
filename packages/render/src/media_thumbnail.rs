//! MediaThumbnail — framed preview surface with badge/play/state overlays.
//!
//! Contract: `docs/contracts/components/media-thumbnail.md`
//! Ported from: `packages/jetstream/components/src/media_thumbnail.rs`.
//!
//! No image decode in the node tier either: the `ready` frame always shows
//! the token-resolved fallback icon. Height derives from the aspect ratio
//! against a reference frame width (no CSS `aspect-ratio` downstream).

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
    NodePosition,
};
use poodle_specs::{MediaFrameWidth, MediaState, MediaThumbnailSpec};
use poodle_specs::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::color::with_alpha;
use crate::presentation::rem_to_px;
use crate::spinner::spinner;

const FRAME_FILL_REF_REM: f32 = 20.0;
const FRAME_XL_REM: f32 = 24.0;

pub fn media_thumbnail(spec: &MediaThumbnailSpec, theme: &dyn ThemeProvider) -> Node {
    let panel = theme.resolve_color(spec.frame_panel_token());
    let border = theme.resolve_color(spec.frame_border_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let placeholder_color = theme.resolve_color(spec.placeholder_icon_token());
    let surface = theme.resolve_color("color.background.surface");
    let elevated = theme.resolve_color("color.background.elevated");

    // Frame radius: calc(radius.surface − 0.125rem).
    let frame_radius = (theme.resolve_radius(spec.frame_radius_token())
        - rem_to_px(spec.frame_radius_inset_rem()))
    .max(0.0);
    let badge_radius = theme.resolve_radius(spec.badge_radius_token());
    let play_radius = theme.resolve_radius(spec.play_radius_token());

    let body_size = rem_to_px(0.875);
    let label_size = rem_to_px(0.8125);
    let caption_title_size = rem_to_px(0.875);
    let caption_meta_size = rem_to_px(0.8125);
    let badge_text_size = rem_to_px(0.6875);

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };

    // Frame width / height (contract §9 Aspect Ratios).
    let (ref_width_px, explicit_width) = match spec.frame_width {
        MediaFrameWidth::Fill => (rem_to_px(FRAME_FILL_REF_REM), None),
        MediaFrameWidth::Xl => (rem_to_px(FRAME_XL_REM), None),
        MediaFrameWidth::Px(w) => (w, Some(w)),
    };
    let derived_h = spec.frame_height_for_width(ref_width_px);
    let frame_h = spec
        .frame_min_height
        .map(|m| m.max(derived_h))
        .unwrap_or(derived_h);
    let frame_h = spec
        .frame_max_height
        .map(|m| frame_h.min(m))
        .unwrap_or(frame_h);

    // ── Frame ──────────────────────────────────────────────────
    let mut frame = Node::container();
    {
        let s = &mut frame.style;
        s.descriptor.background = Some(panel);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.layout.height = LayoutSizing::Fixed(frame_h);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        // Old tier: `.w(ref).grow()` — fixed reference width plus flex-grow
        // (grow does not replace the width channel).
        s.descriptor.layout.width = LayoutSizing::Fixed(explicit_width.unwrap_or(ref_width_px));
        if explicit_width.is_none() {
            s.flex_grow = Some(1.0);
            s.self_stretch = true;
        }
    }
    all_radius(&mut frame, frame_radius);
    frame.position = NodePosition::Relative;

    if spec.shows_fallback_copy() {
        // ── State display (loading / error / empty) ────────────
        let mut state = Node::container();
        {
            let s = &mut state.style;
            s.descriptor.background = Some(with_alpha(surface, surface.3 * 0.78));
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.fill_height = true;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.5);
            if spec.is_compact() {
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            } else {
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
                s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            }
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.875);
            pad.right = rem_to_px(0.875);
            pad.top = rem_to_px(0.875);
            pad.bottom = rem_to_px(0.875);
        }

        if spec.state == MediaState::Loading {
            state = state.child(spinner(
                &SpinnerSpec::new()
                    .with_variant(SpinnerVariant::Grid)
                    .with_size(if spec.is_compact() {
                        SpinnerSize::Sm
                    } else {
                        SpinnerSize::Md
                    })
                    .with_tone(SpinnerTone::Accent),
                theme,
            ));
        }

        let mut title = Node::text(spec.resolved_state_title());
        title.style.descriptor.text_color = Some(text_primary);
        title.style.text_size = Some(body_size);
        title.style.text_weight = Some(600);
        state = state.child(title);

        if spec.state_message_visible() {
            let mut message = Node::text(spec.resolved_state_message().unwrap());
            message.style.descriptor.text_color = Some(text_secondary);
            message.style.text_size = Some(label_size);
            state = state.child(message);
        }

        frame = frame.child(state);
    } else {
        // ── Placeholder fallback icon (contract §9) ────────────
        let mut fallback = Node::icon(spec.fallback_icon(), rem_to_px(1.75));
        fallback.style.descriptor.text_color = Some(placeholder_color);
        frame = frame.child(fallback);

        // ── Play indicator (audio/video, contract §3) ─────────
        if spec.shows_play_indicator() {
            let play_color = theme.resolve_color(spec.play_color_token());
            let mut chip = Node::container();
            {
                let s = &mut chip.style;
                s.descriptor.background = Some(with_alpha(elevated, elevated.3 * 0.78));
                s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(2.0));
                s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(2.0));
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            }
            all_radius(&mut chip, play_radius);
            chip.position = NodePosition::Absolute {
                top: None,
                left: Some(rem_to_px(0.625)),
                right: None,
                bottom: Some(rem_to_px(0.625)),
            };
            let mut glyph = Node::icon(spec.play_indicator_icon(), rem_to_px(0.9375));
            glyph.style.descriptor.text_color = Some(play_color);
            frame = frame.child(chip.child(glyph));
        }
    }

    // ── Badge overlay (contract §9) ────────────────────────────
    if let Some(ref badge_label) = spec.badge_label {
        let badge_color = theme.resolve_color(spec.badge_text_token());
        let inset = if spec.is_compact() { 0.5 } else { 0.625 };
        let mut badge = Node::container();
        {
            let s = &mut badge.style;
            s.descriptor.background = Some(with_alpha(surface, surface.3 * 0.74));
            s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(1.5));
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.625);
            pad.right = rem_to_px(0.625);
        }
        all_radius(&mut badge, badge_radius);
        badge.position = NodePosition::Absolute {
            top: Some(rem_to_px(inset)),
            left: None,
            right: Some(rem_to_px(inset)),
            bottom: None,
        };
        let mut label = Node::text(badge_label.to_uppercase());
        {
            let s = &mut label.style;
            s.descriptor.text_color = Some(badge_color);
            s.text_size = Some(badge_text_size);
            s.text_weight = Some(600);
            s.letter_spacing_em = Some(0.04); // contract badge: letter-spacing 0.04em
        }
        frame = frame.child(badge.child(label));
    }

    // ── Root: frame + caption ──────────────────────────────────
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        if !spec.is_compact() {
            s.descriptor.layout.spacing.gap = rem_to_px(0.5);
        }
    }
    root = root.child(frame);

    if spec.caption_visible() {
        let mut caption = Node::container();
        {
            let s = &mut caption.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        }
        if let Some(ref title) = spec.title {
            let mut t = Node::text(title);
            t.style.descriptor.text_color = Some(text_primary);
            t.style.text_size = Some(caption_title_size);
            t.style.text_weight = Some(500);
            caption = caption.child(t);
        }
        if let Some(ref meta) = spec.meta {
            let mut m = Node::text(meta);
            m.style.descriptor.text_color = Some(text_secondary);
            m.style.text_size = Some(caption_meta_size);
            caption = caption.child(m);
        }
        root = root.child(caption);
    }

    root
}
