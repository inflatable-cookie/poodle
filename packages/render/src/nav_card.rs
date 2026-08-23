//! NavCard — a card that navigates.
//!
//! Contract: `docs/contracts/components/nav-card.md`
//! Ported from: `packages/jetstream/components/src/nav_card.rs`.
//!
//! The arrow carries the contract-exact resting opacity (0) and reveals on
//! its OWN hover (no root→child group-hover channel) — same delta as the
//! reference tier.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    StylePatch,
};
use poodle_specs::NavCardSpec;

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn nav_card(
    spec: &NavCardSpec,
    ctx: &RenderContext<'_>,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    nav_card_inner(spec, ctx, on_click, None, true)
}

pub fn nav_card_with_icon(
    spec: &NavCardSpec,
    ctx: &RenderContext<'_>,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    icon: Option<Node>,
) -> Node {
    nav_card_inner(spec, ctx, on_click, icon, false)
}

fn nav_card_inner(
    spec: &NavCardSpec,
    ctx: &RenderContext<'_>,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    icon: Option<Node>,
    use_placeholder: bool,
) -> Node {
    let fill = ctx.theme().resolve_color(spec.fill_token());
    let border = ctx.theme().resolve_color(spec.border_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    let icon_radius = ctx.theme().resolve_radius(spec.icon_radius_token());
    let badge_radius = ctx.theme().resolve_radius(spec.badge_radius_token());
    let text_primary = ctx.theme().resolve_color(spec.title_color_token());
    let text_secondary = ctx.theme().resolve_color(spec.description_color_token());
    let accent = ctx.theme().resolve_color(spec.icon_bg_token());

    // Typography — title from label-size token; description/badge/icon-glyph
    // from contract-exact rem.
    let title_font = ctx.theme().resolve_space(spec.title_typography_token());
    let desc_font = rem_to_px(spec.description_font_size_rem());
    let badge_font = rem_to_px(spec.badge_font_size_rem());
    let icon_font = rem_to_px(spec.icon_font_size_rem());

    // Density-aware geometry (contract §8 Density Overrides table).
    let density = ctx.resolve_density(spec.density);
    let root_gap = rem_to_px(spec.root_gap_rem(density));
    let pad_x = rem_to_px(spec.padding_x_rem(density));
    let pad_y = rem_to_px(spec.padding_y_rem(density));
    let content_gap = rem_to_px(spec.content_gap_rem(density));
    let title_gap = rem_to_px(spec.title_gap_rem(density));
    let icon_size = rem_to_px(spec.icon_size_rem(density));
    let arrow_size = rem_to_px(1.0); // contract §8 Arrow: 1rem square

    // Hover: bg = color-mix(elevated 52%, surface);
    // border = color-mix(accent 28%, border-subtle).
    let elevated = ctx.theme().resolve_color(spec.hover_fill_token());
    let hover_fill = mix_srgb(elevated, fill, 0.52);
    let hover_border = mix_srgb(accent, border, 0.28);

    // ── Icon slot: accent-tinted square (contract §8 Icon) ──────────────
    let mut icon_slot = Node::container();
    {
        let s = &mut icon_slot.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(icon_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(icon_size);
        s.flex_none = true;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = icon_radius;
        c.top_right = icon_radius;
        c.bottom_right = icon_radius;
        c.bottom_left = icon_radius;
        s.descriptor.background = Some(with_alpha(accent, accent.3 * 0.12));
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    // The preview may provide a real icon node. The first-letter glyph remains
    // the host-neutral fallback used by the shared recipe when no slot exists.
    let icon_slot = if let Some(icon) = icon {
        icon_slot.child(icon)
    } else if use_placeholder {
        let mut glyph = Node::text(
            spec.title
                .chars()
                .next()
                .map_or(String::new(), |c| c.to_uppercase().to_string()),
        );
        glyph.style.descriptor.text_color = Some(accent);
        glyph.style.text_size = Some(icon_font);
        glyph.style.text_weight = Some(600);
        icon_slot.child(glyph)
    } else {
        icon_slot
    };

    // ── Title row (title + optional badge) ──────────────────────────────
    let mut title_row = Node::container();
    {
        let s = &mut title_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = title_gap;
    }
    let mut title = Node::text(&spec.title);
    title.style.descriptor.text_color = Some(text_primary);
    title.style.text_size = Some(title_font);
    title.style.text_weight = Some(600);
    let mut title_row = title_row.child(title);

    if let Some(ref badge_text) = spec.badge {
        let badge_bg = ctx.theme().resolve_color(spec.badge_bg_token());
        let badge_color = ctx.theme().resolve_color(spec.badge_color_token());
        let mut badge = Node::text(badge_text);
        {
            let s = &mut badge.style;
            s.descriptor.text_color = Some(badge_color);
            s.text_size = Some(badge_font);
            s.text_weight = Some(600);
            s.letter_spacing_em = Some(0.05); // contract §8 Badge tracking
            s.descriptor.background = Some(with_alpha(badge_bg, badge_bg.3 * 0.16));
            let c = &mut s.descriptor.corner_radii;
            c.top_left = badge_radius;
            c.top_right = badge_radius;
            c.bottom_right = badge_radius;
            c.bottom_left = badge_radius;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.375);
            pad.right = rem_to_px(0.375);
            pad.top = rem_to_px(0.0625);
            pad.bottom = rem_to_px(0.0625);
        }
        title_row = title_row.child(badge);
    }

    // ── Content column (title row + optional description) ───────────────
    let mut content = Node::container();
    {
        let s = &mut content.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = content_gap;
        s.flex_fill = true;
        s.min_width = Some(0.0);
    }
    let mut content = content.child(title_row);

    if let Some(ref desc) = spec.description {
        let mut d = Node::text(desc);
        d.style.descriptor.text_color = Some(text_secondary);
        d.style.text_size = Some(desc_font);
        content = content.child(d);
    }

    // ── Arrow (contract §2 required; reveals on its own hover) ──────────
    let mut arrow = Node::text("\u{2192}");
    {
        let s = &mut arrow.style;
        s.descriptor.text_color = Some(text_secondary);
        s.text_size = Some(arrow_size);
        s.flex_none = true;
        s.descriptor.opacity = 0.0;
        s.hover = Some(StylePatch {
            background: None,
            border_color: None,
            text_color: None,
            opacity: Some(1.0),
        });
    }

    // ── Root row ────────────────────────────────────────────────────────
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = with_alpha(border, border.3 * 0.32);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = root_gap;
    }
    let mut el = el.child(icon_slot).child(content).child(arrow);

    if spec.is_disabled {
        el.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
        el.interaction.disabled = true;
    } else {
        // Hover: elevated fill + accent-tinted border. The focus ring colour
        // is host-painted, as in the reference tier.
        el.style.descriptor.cursor = CursorHint::Pointer;
        el.interaction.focusable = true;
        el.style.hover = Some(StylePatch {
            background: Some(hover_fill),
            border_color: Some(hover_border),
            text_color: None,
            opacity: None,
        });
        if let Some(handler) = on_click {
            el.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    el
}
