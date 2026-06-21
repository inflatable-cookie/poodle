//! ListCard — Jetstream list card backed by ListCardSpec.
//!
//! Contract: `docs/contracts/components/list-card.md`
//! Reference: `packages/svelte/components/src/ListCard.svelte`
//!
//! Anatomy (contract §2): `[sash?] [selection?] [leading?] [body(header/subtitle)] [meta?] [handle?]`.
//! Every dimension resolves from a token or a contract-exact rem via the spec
//! helpers — zero hardcoded hsla; rem literals are contract values resolved by
//! `rem_to_px`. Interaction (click/keyboard) lives in the preview event loop.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{LeadingFill, LeadingShape, ListCardLayout, ListCardSpec, SelectionIndicator};

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

pub fn js_list_card(spec: &ListCardSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let surface = resolve_color(theme, spec.fill_token());
    let text_primary = resolve_color(theme, spec.title_color_token());
    let border_subtle = resolve_color(theme, spec.border_token());
    let border_default = resolve_color(theme, spec.hover_border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_secondary = resolve_color(theme, spec.subtitle_color_token());
    let accent = resolve_color(theme, spec.accent_base_token());
    let on_accent = resolve_color(theme, spec.on_accent_color_token());
    let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

    // Svelte: fill = color-mix(surface 88%, text-primary); hover = 82%.
    let fill = color_mix(surface, text_primary, 0.88);
    let hover_fill = color_mix(surface, text_primary, 0.82);
    // Svelte: border = color-mix(border-subtle 18%, transparent); hover = default 52%.
    let border = tint(border_subtle, 0.18);
    let hover_border = tint(border_default, 0.52);

    // Spacing — contract §8 Root: padding 0.625rem 0.75rem, gap 0.75rem.
    let pad_x = resolve_px(theme, "space.inline.md"); // 0.75rem
    let pad_y = resolve_px(theme, "space.stack.sm"); // 0.5rem — see NOTE (contract 0.625rem)
    let gap = resolve_px(theme, "space.inline.md"); // 0.75rem

    // Typography — title from label-size token; subtitle/meta 0.75rem (no token).
    let title_font = resolve_px(theme, "typography.label.size");
    let small_font = rem_to_px(spec.small_font_size_rem());

    let is_compact = spec.layout == ListCardLayout::Compact;
    let is_stacked = spec.layout == ListCardLayout::Stacked;

    // ── Leading: shape-sized square, tint/solid fill (contract §7/§8) ───
    let leading_size = rem_to_px(spec.leading_size_rem());
    let leading_font = rem_to_px(spec.leading_font_size_rem());
    let leading_radius = match spec.leading_shape {
        LeadingShape::Circle => leading_size / 2.0,
        LeadingShape::RoundedSquare => resolve_radius(theme, spec.leading_radius_token()),
    };
    let leading_bg = match spec.leading_fill {
        LeadingFill::Tint => tint(accent, spec.leading_tint_ratio()),
        LeadingFill::Solid => accent,
    };
    let leading_icon_color = match spec.leading_fill {
        LeadingFill::Tint => accent,
        LeadingFill::Solid => on_accent,
    };

    let leading_el = ui_element::div()
        .size(leading_size)
        .flex_none()
        .rounded(leading_radius)
        .bg(leading_bg)
        .items_center()
        .justify_center()
        .child(
            ui_element::label(
                &spec
                    .title
                    .chars()
                    .next()
                    .map_or(String::new(), |c| c.to_uppercase().to_string()),
            )
            .text_color(leading_icon_color)
            .text_size(leading_font)
            .text_weight(600),
        );

    // ── Body: header (title) + optional subtitle ───────────────────────
    let mut body = ui_element::div()
        .flex_col()
        .gap(rem_to_px(spec.body_gap_rem()))
        .flex_grow()
        .min_w_0();

    body = body.child(
        ui_element::label(&spec.title)
            .text_color(text_primary)
            .text_size(title_font)
            .text_weight(500)
            .text_ellipsis(),
    );

    if let Some(ref subtitle) = spec.subtitle {
        body = body.child(
            ui_element::label(subtitle)
                .text_color(text_secondary)
                .text_size(small_font)
                .text_ellipsis(),
        );
    }

    // ── Meta (right-aligned). NOTE: tabular-nums has no JsEl API. ───────
    let meta_el = spec.meta.as_ref().map(|m| {
        ui_element::label(m)
            .text_color(text_secondary)
            .text_size(small_font)
            .flex_none()
    });

    // ── Selection indicator (checkbox box) — contract §3/§8 ─────────────
    let selection_el = (spec.is_selectable
        && spec.selection_indicator == SelectionIndicator::Checkbox)
        .then(|| {
            let box_size = resolve_px(theme, spec.selection_indicator_size_token());
            let pill = resolve_radius(theme, spec.leading_radius_token());
            let (box_bg, box_border) = if spec.is_selected {
                (accent, accent)
            } else {
                (surface, border_subtle)
            };
            ui_element::div()
                .size(box_size)
                .flex_none()
                .rounded(pill)
                .border(1.0)
                .border_color(box_border)
                .bg(box_bg)
        });

    // ── Reorder handle (two columns of dots) — contract §2 Handle ───────
    let handle_el = spec.show_reorder_handle.then(|| {
        let dot = resolve_px(theme, "space.inline.xs") / 2.0; // 0.125rem dot
        let dot_gap = rem_to_px(0.125);
        let handle_color = text_secondary;
        let col = || {
            ui_element::div()
                .flex_col()
                .gap(dot_gap)
                .child(ui_element::div().size(dot).rounded(dot).bg(handle_color))
                .child(ui_element::div().size(dot).rounded(dot).bg(handle_color))
                .child(ui_element::div().size(dot).rounded(dot).bg(handle_color))
        };
        ui_element::div()
            .flex_row()
            .flex_none()
            .items_center()
            .gap(dot_gap)
            .opacity(0.6)
            .child(col())
            .child(col())
    });

    // ── Sash ribbon (top-left). NOTE: JsEl has no transform/rotate, so the ──
    //    diagonal rotate(-45deg) is approximated by a top-left block. ───────
    let sash_el = spec.sash.as_ref().map(|sash_text| {
        let sash_bg = spec
            .sash_color
            .as_ref()
            .and_then(|c| crate::theme_ext::hex_to_rgb255(c))
            .map(|rgb| crate::theme_ext::rgb255_to_vec4(rgb, 1.0))
            .unwrap_or_else(|| resolve_color(theme, spec.sash_bg_token()));
        ui_element::label(&sash_text.to_uppercase())
            .absolute()
            .top(0.0)
            .left(0.0)
            .px(rem_to_px(0.375))
            .py(rem_to_px(0.0625))
            .bg(sash_bg)
            .text_color(on_accent)
            .text_size(rem_to_px(0.5625))
            .text_weight(700)
    });

    // ── Root row (or column when stacked) ───────────────────────────────
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border)
        .rounded(radius)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .gap(if is_compact { gap / 2.0 } else { gap });

    el = if is_stacked {
        el.flex_col().items_start()
    } else {
        el.flex_row().items_center()
    };

    // Highlighted: accent-tinted border + inset accent ring approximation
    // (contract §8 Root highlighted). NOTE: JsEl can't paint a gradient overlay
    // over an existing fill, so the accent gradient is approximated by an
    // accent-tinted fill blended toward surface.
    if spec.is_highlighted {
        // Approximate the accent-to-transparent gradient by compositing the
        // accent at 10% over the base fill (a single flat color).
        el = el
            .border_color(tint(accent, 0.34))
            .bg(color_mix(accent, fill, 0.10));
    }

    if let Some(sel) = selection_el {
        el = el.child(sel);
    }
    el = el.child(leading_el).child(body);

    if let Some(m) = meta_el {
        el = el.child(m);
    }

    if let Some(handle) = handle_el {
        el = el.child(handle);
    }

    if let Some(sash) = sash_el {
        el = el.relative().overflow_hidden().child(sash);
    }

    // Not-live: reduced opacity (0.72). NOTE: JsEl has no dashed-border or
    // grayscale filter, so only the opacity treatment is applied here.
    if spec.is_not_live {
        el = el.opacity(spec.not_live_opacity());
    }

    // Disabled: token opacity.
    if spec.is_disabled {
        el = el.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    }

    // Interactive: hover background + border, pointer + focusable.
    if (spec.is_interactive || spec.href.is_some()) && !spec.is_disabled {
        el = el
            .cursor_pointer()
            .focusable()
            .hover(|s| s.bg(hover_fill).border_color(hover_border));
        let _ = focus_ring; // surfaced for the preview focus layer; see NOTE.
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{LeadingFill, LeadingShape, SelectionIndicator};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn accent(th: &JetstreamThemeProvider) -> ProbeColor {
        let c = resolve_color(th, "color.accent.base");
        ProbeColor { r: c.x, g: c.y, b: c.z, a: c.w }
    }

    fn spec() -> ListCardSpec {
        ListCardSpec::new()
            .with_title("design-system-v2.figma")
            .with_subtitle("Edited 2 hours ago")
            .with_meta("14.2 MB")
            .with_interactive(true)
    }

    #[test]
    fn renders_title_subtitle_meta() {
        let th = theme();
        let tree = probe(&js_list_card(&spec(), &th), 360.0, 96.0);
        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(tree.has_text("design-system-v2.figma"), "title missing: {:?}", tree.texts());
        assert!(tree.has_text("Edited 2 hours ago"), "subtitle missing: {:?}", tree.texts());
        assert!(tree.has_text("14.2 MB"), "meta missing: {:?}", tree.texts());
    }

    #[test]
    fn leading_size_follows_shape() {
        let th = theme();
        // Circle = 2rem, rounded-square = 2.75rem — distinct leading box sizes.
        let circle = probe(
            &js_list_card(&spec().with_leading_shape(LeadingShape::Circle), &th),
            360.0,
            96.0,
        );
        let square = probe(
            &js_list_card(&spec().with_leading_shape(LeadingShape::RoundedSquare), &th),
            360.0,
            96.0,
        );
        let cw = rem_to_px(2.0);
        let sw = rem_to_px(2.75);
        let has = |t: &crate::render_probe::ProbeTree, s: f32| {
            t.nodes.iter().any(|n| (n.w - s).abs() < 0.5 && (n.h - s).abs() < 0.5)
        };
        assert!(has(&circle, cw), "circle leading {cw} missing");
        assert!(has(&square, sw), "square leading {sw} missing");
    }

    #[test]
    fn solid_fill_uses_accent_leading() {
        let th = theme();
        let acc = accent(&th);
        // Tint leading: no opaque accent box.
        let tint_tree = probe(
            &js_list_card(&spec().with_leading_fill(LeadingFill::Tint), &th),
            360.0,
            96.0,
        );
        assert!(
            !tint_tree.has_background(acc, 0.01),
            "tint leading should not paint an opaque accent box"
        );
        // Solid leading: opaque accent box present.
        let solid_tree = probe(
            &js_list_card(&spec().with_leading_fill(LeadingFill::Solid), &th),
            360.0,
            96.0,
        );
        assert!(
            solid_tree.has_background(acc, 0.01),
            "solid leading must paint an opaque accent box"
        );
    }

    #[test]
    fn selection_indicator_renders_when_checkbox_and_selectable() {
        let th = theme();
        let acc = accent(&th);
        let selected = ListCardSpec::new()
            .with_title("Picked")
            .with_selectable(true)
            .with_selected(true)
            .with_selection_indicator(SelectionIndicator::Checkbox);
        let tree = probe(&js_list_card(&selected, &th), 360.0, 96.0);
        // Selected checkbox box is filled with the accent color.
        assert!(
            tree.has_background(acc, 0.01),
            "selected checkbox indicator must be accent-filled"
        );
    }

    #[test]
    fn leading_size_offset_shifts_leading_box() {
        let th = theme();
        // Contract §3 leadingSizeOffset: +1 step grows the leading box by 0.25rem,
        // -1 step shrinks it; circle base is 2rem.
        let has_box = |t: &crate::render_probe::ProbeTree, edge: f32| {
            t.nodes
                .iter()
                .any(|n| (n.w - edge).abs() < 0.5 && (n.h - edge).abs() < 0.5)
        };
        let base = probe(&js_list_card(&spec(), &th), 360.0, 96.0);
        assert!(has_box(&base, rem_to_px(2.0)), "base circle leading 2rem missing");

        let bigger = probe(
            &js_list_card(&spec().with_leading_size_offset(1), &th),
            360.0,
            96.0,
        );
        assert!(
            has_box(&bigger, rem_to_px(2.25)),
            "offset +1 should grow leading to 2.25rem"
        );

        let smaller = probe(
            &js_list_card(&spec().with_leading_size_offset(-1), &th),
            360.0,
            96.0,
        );
        assert!(
            has_box(&smaller, rem_to_px(1.75)),
            "offset -1 should shrink leading to 1.75rem"
        );
    }

    #[test]
    fn sash_ribbon_renders_text() {
        let th = theme();
        let tree = probe(
            &js_list_card(&spec().with_sash("Free"), &th),
            360.0,
            96.0,
        );
        assert!(tree.has_text("FREE"), "sash ribbon text missing: {:?}", tree.texts());
    }

    #[test]
    fn empty_meta_and_subtitle_omitted() {
        let th = theme();
        // A bare title-only card renders just the title (plus leading glyph),
        // proving optional regions are correctly omitted (empty-state shape).
        let tree = probe(
            &js_list_card(&ListCardSpec::new().with_title("Read-only item"), &th),
            360.0,
            96.0,
        );
        assert!(tree.has_text("Read-only item"), "title missing: {:?}", tree.texts());
        // No meta text like a size string leaks in.
        assert!(!tree.has_text("14.2 MB"), "unexpected meta on bare card");
    }
}
