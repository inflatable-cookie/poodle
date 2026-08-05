//! ListCard — one row-card in a list.
//!
//! Contract: `docs/contracts/components/list-card.md`
//! Ported from: `packages/jetstream/components/src/list_card.rs`.
//!
//! Anatomy (contract §2): `[sash?] [selection?] [leading?] [body(header(title
//! + accessories/badges) / subtitle / footer?)] [meta?] [actions?]
//! [trailing?] [handle?]`. Host-snippet slots arrive through
//! [`ListCardSlots`]; `trailing` is exclusive and replaces meta + actions.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodePosition,
    ShadowLayer, StylePatch,
};
use poodle_specs::{LeadingFill, LeadingShape, ListCardLayout, ListCardSpec, SelectionIndicator};

use crate::color::{hex_color, mix_srgb, with_alpha};
use crate::presentation::rem_to_px;

/// Host-composed slots (contract §2 / §3).
#[derive(Default)]
pub struct ListCardSlots {
    /// Avatar/icon/thumbnail content; overrides the derived first-letter glyph.
    pub leading: Option<Node>,
    /// Pills/badges in the header-accessories cluster beside the title.
    pub badges: Vec<Node>,
    /// Counter row below the subtitle.
    pub footer: Option<Node>,
    /// Explicit action triggers in the right-edge lane after meta.
    pub actions: Option<Node>,
    /// Exclusive right-edge lane; replaces meta and actions when present.
    pub trailing: Option<Node>,
    /// Supplementary header-corner content, tertiary-coloured.
    pub corner: Option<Node>,
}

fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

fn square(size: f32) -> Node {
    let mut n = Node::container();
    let s = &mut n.style;
    // Explicit Row (see switch.rs).
    s.descriptor.layout.direction = LayoutDirection::Row;
    s.descriptor.layout.width = LayoutSizing::Fixed(size);
    s.descriptor.layout.height = LayoutSizing::Fixed(size);
    s.flex_none = true;
    n
}

/// The whole card is the hit target; only an interactive card fires
/// (`is_interactive` or an `href`, and not disabled).
pub fn list_card(
    spec: &ListCardSpec,
    theme: &dyn ThemeProvider,
    slots: ListCardSlots,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let surface = theme.resolve_color(spec.fill_token());
    let text_primary = theme.resolve_color(spec.title_color_token());
    let border_subtle = theme.resolve_color(spec.border_token());
    let border_default = theme.resolve_color(spec.hover_border_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let text_secondary = theme.resolve_color(spec.subtitle_color_token());
    let accent = theme.resolve_color(spec.accent_base_token());
    let on_accent = theme.resolve_color(spec.on_accent_color_token());

    // fill = color-mix(surface 88%, text-primary); hover = 82%.
    let fill = mix_srgb(surface, text_primary, 0.88);
    let hover_fill = mix_srgb(surface, text_primary, 0.82);
    // border = border-subtle@18%; hover = default@52%.
    let border = with_alpha(border_subtle, border_subtle.3 * 0.18);
    let hover_border = with_alpha(border_default, border_default.3 * 0.52);

    // Spacing — contract §8 Root.
    let pad_x = theme.resolve_space("space.inline.md"); // 0.75rem
    let pad_y = theme.resolve_space("space.stack.sm"); // 0.5rem (contract 0.625 — reference delta)
    let gap = theme.resolve_space("space.inline.md"); // 0.75rem

    // Typography — title from label-size token; subtitle/meta small.
    let title_font = theme.resolve_space("typography.label.size");
    let small_font = rem_to_px(spec.small_font_size_rem());

    let is_compact = spec.layout == ListCardLayout::Compact;
    let is_stacked = spec.layout == ListCardLayout::Stacked;

    // ── Leading: shape-sized square, tint/solid fill (contract §7/§8) ───
    let leading_size = rem_to_px(spec.leading_size_rem());
    let leading_font = rem_to_px(spec.leading_font_size_rem());
    let leading_radius = match spec.leading_shape {
        LeadingShape::Circle => leading_size / 2.0,
        LeadingShape::RoundedSquare => theme.resolve_radius(spec.leading_radius_token()),
    };
    let leading_bg = match spec.leading_fill {
        LeadingFill::Tint => with_alpha(accent, accent.3 * spec.leading_tint_ratio()),
        LeadingFill::Solid => accent,
    };
    let leading_icon_color = match spec.leading_fill {
        LeadingFill::Tint => accent,
        LeadingFill::Solid => on_accent,
    };

    // The styled leading box always paints the shape/tint/solid fill; the
    // host slot overrides the derived first-letter glyph.
    let leading_inner = slots.leading.unwrap_or_else(|| {
        let mut glyph = Node::text(
            spec.title
                .chars()
                .next()
                .map_or(String::new(), |c| c.to_uppercase().to_string()),
        );
        glyph.style.descriptor.text_color = Some(leading_icon_color);
        glyph.style.text_size = Some(leading_font);
        glyph.style.text_weight = Some(600);
        glyph
    });

    let mut leading_el = square(leading_size);
    {
        let s = &mut leading_el.style;
        s.descriptor.background = Some(leading_bg);
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = poodle_node::MainAxisAlignment::Center;
        s.descriptor.layout.overflow_x = poodle_node::LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = poodle_node::LayoutOverflow::Hidden;
        s.descriptor.text_color = Some(leading_icon_color);
    }
    all_corners(&mut leading_el, leading_radius);
    let leading_el = leading_el.child(leading_inner);

    // ── Body: header (title + accessories) + subtitle + footer ──────────
    let mut body = Node::container();
    {
        let s = &mut body.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(spec.body_gap_rem());
        s.flex_fill = true;
        s.min_width = Some(0.0);
    }

    // Title truncates (flex 1, min-width 0); accessories are shrink-proof.
    let mut title_el = Node::text(&spec.title);
    {
        let s = &mut title_el.style;
        s.descriptor.text_color = Some(text_primary);
        s.text_size = Some(title_font);
        s.text_weight = Some(500);
        s.text_ellipsis = true;
        s.flex_fill = true;
        s.min_width = Some(0.0);
    }

    let mut body = if slots.badges.is_empty() && slots.corner.is_none() {
        body.child(title_el)
    } else {
        // Header-accessories cluster (contract §8): shrink-proof, wraps,
        // gap space.inline.sm; badges/corner groups use space.inline.xs.
        let mut accessories = Node::container();
        {
            let s = &mut accessories.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_none = true;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.flex_wrap = true;
            s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
        }

        if !slots.badges.is_empty() {
            let mut group = Node::container();
            {
                let s = &mut group.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.xs");
            }
            let mut group = group;
            for badge in slots.badges {
                group = group.child(badge);
            }
            accessories = accessories.child(group);
        }

        if let Some(corner) = slots.corner {
            // Corner group — tertiary text color (contract §8).
            let text_tertiary = theme.resolve_color("color.text.tertiary");
            let mut group = Node::container();
            {
                let s = &mut group.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.xs");
                s.descriptor.text_color = Some(text_tertiary);
            }
            accessories = accessories.child(group.child(corner));
        }

        let mut header = Node::container();
        {
            let s = &mut header.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = rem_to_px(spec.header_gap_rem());
        }
        body.child(header.child(title_el).child(accessories))
    };

    if let Some(ref subtitle) = spec.subtitle {
        let mut sub = Node::text(subtitle);
        sub.style.descriptor.text_color = Some(text_secondary);
        sub.style.text_size = Some(small_font);
        sub.style.text_ellipsis = true;
        body = body.child(sub);
    }

    // Footer — gap 0.5rem, margin-top 0.125rem (contract §8 Footer).
    if let Some(footer) = slots.footer {
        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = rem_to_px(spec.footer_gap_rem());
            s.descriptor.layout.spacing.margin.top = rem_to_px(0.125);
        }
        body = body.child(row.child(footer));
    }

    // ── Right-edge lanes (contract §7): trailing is exclusive ───────────
    let has_trailing = slots.trailing.is_some();

    let meta_el = (!has_trailing)
        .then(|| spec.meta.as_ref())
        .flatten()
        .map(|m| {
            let mut meta = Node::text(m);
            meta.style.descriptor.text_color = Some(text_secondary);
            meta.style.text_size = Some(small_font);
            meta.style.flex_none = true;
            meta
        });

    let lane = |gap: Option<f32>, child: Node, theme: &dyn ThemeProvider| -> Node {
        let mut l = Node::container();
        {
            let s = &mut l.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_none = true;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            if let Some(g) = gap {
                s.descriptor.layout.spacing.gap = g;
            }
        }
        let _ = theme;
        l.child(child)
    };
    let actions_el = slots
        .actions
        .filter(|_| !has_trailing)
        .map(|a| lane(Some(theme.resolve_space("space.inline.xs")), a, theme));
    let trailing_el = slots.trailing.map(|t| lane(None, t, theme));

    // ── Selection indicator (checkbox box) — contract §3/§8 ─────────────
    let selection_el = (spec.is_selectable
        && spec.selection_indicator == SelectionIndicator::Checkbox)
        .then(|| {
            let box_size = theme.resolve_space(spec.selection_indicator_size_token());
            let pill = theme.resolve_radius(spec.leading_radius_token());
            let (box_bg, box_border) = if spec.is_selected {
                (accent, accent)
            } else {
                (surface, border_subtle)
            };
            let mut b = square(box_size);
            {
                let s = &mut b.style;
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = box_border;
                s.descriptor.background = Some(box_bg);
            }
            all_corners(&mut b, pill);
            b
        });

    // ── Reorder handle (two columns of dots) — contract §2 Handle ───────
    let handle_el = spec.show_reorder_handle.then(|| {
        let dot = theme.resolve_space("space.inline.xs") / 2.0; // 0.125rem dot
        let dot_gap = rem_to_px(0.125);
        let handle_color = text_secondary;
        let dot_el = || {
            let mut d = square(dot);
            d.style.descriptor.background = Some(handle_color);
            all_corners(&mut d, dot);
            d
        };
        let col = || {
            let mut c = Node::container();
            {
                let s = &mut c.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = dot_gap;
            }
            c.child(dot_el()).child(dot_el()).child(dot_el())
        };
        let mut h = Node::container();
        {
            let s = &mut h.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_none = true;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = dot_gap;
            s.descriptor.opacity = 0.6;
        }
        h.child(col()).child(col())
    });

    // ── Sash ribbon (top-left block approximation of the diagonal) ──────
    // A hex sash colour lands in sRGB and linearises at the adapter edge
    // (the established custom-hex fix over the old raw-bytes path).
    let sash_el = spec.sash.as_ref().map(|sash_text| {
        let sash_bg = spec
            .sash_color
            .as_ref()
            .and_then(|c| hex_color(c))
            .unwrap_or_else(|| theme.resolve_color(spec.sash_bg_token()));
        let mut sash = Node::text(sash_text.to_uppercase());
        sash.position = NodePosition::Absolute {
            top: Some(0.0),
            left: Some(0.0),
            right: None,
            bottom: None,
        };
        {
            let s = &mut sash.style;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.375);
            pad.right = rem_to_px(0.375);
            pad.top = rem_to_px(0.0625);
            pad.bottom = rem_to_px(0.0625);
            s.descriptor.background = Some(sash_bg);
            s.descriptor.text_color = Some(on_accent);
            s.text_size = Some(rem_to_px(0.5625));
            s.text_weight = Some(700);
        }
        sash
    });

    // ── Root row (or column when stacked) ───────────────────────────────
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.layout.spacing.gap = if is_compact { gap / 2.0 } else { gap };
        if is_stacked {
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        }
    }
    all_corners(&mut el, radius);

    // Highlighted: accent-tinted border + inset accent ring (contract §8),
    // accent composited at 10% over the base fill.
    if spec.is_highlighted {
        let s = &mut el.style;
        s.descriptor.border.color = with_alpha(accent, accent.3 * 0.34);
        s.descriptor.background = Some(mix_srgb(accent, fill, 0.10));
        s.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: rem_to_px(0.0625),
            color: with_alpha(accent, accent.3 * 0.12),
            inset: true,
        }];
    }

    // Active — a bar down the leading edge as an inset shadow so the card's
    // radius clips it (a child rectangle juts out squarely). Composes with
    // whatever `highlighted` already layered.
    if spec.is_active {
        el.style.shadow_layers.push(ShadowLayer {
            offset_x: rem_to_px(spec.active_bar_width_rem()),
            offset_y: 0.0,
            blur: 0.0,
            spread: 0.0,
            color: accent,
            inset: true,
        });
    }

    let mut el = el;
    if let Some(sel) = selection_el {
        el = el.child(sel);
    }
    el = el.child(leading_el).child(body);

    // Right edge: meta + actions, OR the exclusive trailing lane.
    if let Some(m) = meta_el {
        el = el.child(m);
    }
    if let Some(a) = actions_el {
        el = el.child(a);
    }
    if let Some(t) = trailing_el {
        el = el.child(t);
    }
    if let Some(handle) = handle_el {
        el = el.child(handle);
    }

    if let Some(sash) = sash_el {
        el.position = NodePosition::Relative;
        el.style.descriptor.layout.overflow_x = poodle_node::LayoutOverflow::Hidden;
        el.style.descriptor.layout.overflow_y = poodle_node::LayoutOverflow::Hidden;
        el = el.child(sash);
    }

    // Not-live: dashed 0.1875rem border at default@72%, grayscale(1), and
    // reduced opacity (contract §8).
    if spec.is_not_live {
        let s = &mut el.style;
        s.descriptor.border.width = rem_to_px(0.1875);
        s.border_dashed = true;
        s.descriptor.border.color = with_alpha(border_default, border_default.3 * 0.72);
        s.grayscale = 1.0;
        s.descriptor.opacity = spec.not_live_opacity();
    }

    // Disabled: token opacity.
    if spec.is_disabled {
        el.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    }

    // Interactive: hover background + border, pointer + focusable. Not-live
    // cards restore their dashed border to full border-default on hover.
    let interactive = (spec.is_interactive || spec.href.is_some()) && !spec.is_disabled;
    if interactive {
        let hover_border = if spec.is_not_live {
            border_default
        } else {
            hover_border
        };
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

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}

// Silence unused-import lint when hex sash isn't exercised.
#[allow(unused)]
fn _t(_: ColorValue) {}
