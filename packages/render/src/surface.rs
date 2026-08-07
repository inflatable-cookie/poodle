//! Surface — themed container.
//!
//! Contract: `docs/contracts/components/surface.md` (+ `surface-elevation.md`)
//! Ported from: `packages/jetstream/components/src/surface.rs`.
//!
//! All fills, the border color/width, the radius, and the elevation treatment
//! resolve from tokens or `SurfaceSpec` methods. The CSS color-mix
//! percentages (96% / 98% / 74%) are centralized on the spec.

use poodle_adapter::ThemeProvider;
use poodle_node::Node;
use poodle_specs::SurfaceSpec;

use crate::color::{mix_srgb, with_alpha};

pub fn surface(spec: &SurfaceSpec, theme: &dyn ThemeProvider, children: Vec<Node>) -> Node {
    let padding = spec.resolved_padding();

    // ── Fill (contract §8) ──────────────────────────────────────
    //   panel/base: color-mix(background-surface 96%, transparent) → alpha-only
    //   canvas:     color-mix(background-canvas 98%, transparent)  → alpha-only
    //   elevated:   color-mix(background-elevated 96%, background-panel)
    let base_fill = theme.resolve_color(spec.resolved_background_token());
    let mix_ratio = spec.fill_mix_ratio();
    let bg = match spec.fill_mix_over_token() {
        Some(over_token) => mix_srgb(base_fill, theme.resolve_color(over_token), mix_ratio),
        // Non-elevated tones mix toward transparent → scale alpha only.
        None => with_alpha(base_fill, base_fill.3 * mix_ratio),
    };

    let mut el = Node::container();
    {
        let s = &mut el.style;
        // Explicit Row (see switch.rs): the old surface kept the default.
        s.descriptor.layout.direction = poodle_node::LayoutDirection::Row;
        s.descriptor.background = Some(bg);
        let r = theme.resolve_radius(spec.radius_token());
        s.descriptor.corner_radii.top_left = r;
        s.descriptor.corner_radii.top_right = r;
        s.descriptor.corner_radii.bottom_right = r;
        s.descriptor.corner_radii.bottom_left = r;
    }

    // ── Border (contract §8) ────────────────────────────────────
    //   subtle:  color-mix(border-subtle 74%, transparent)
    //   default: border-default full
    //   none:    no border
    // Width resolves from `border.width.default` (0.0625rem), not a raw 1.0.
    if let Some(border_token) = spec.resolved_border_color() {
        let base_border = theme.resolve_color(border_token);
        let s = &mut el.style;
        s.descriptor.border.color =
            with_alpha(base_border, base_border.3 * spec.border_mix_ratio());
        s.descriptor.border.width = spec
            .resolved_border_width()
            .map(|t| theme.resolve_space(t))
            .unwrap_or(0.0);
    }

    // ── Elevation shadow (contract §8) ──────────────────────────
    // Elevated surfaces resolve `elevation.surface` — token-accurate.
    if spec.is_elevated_resolved() {
        el.style.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_SURFACE);
    }

    if let Some(h) = padding.horizontal {
        let px_val = theme.resolve_space(h);
        let pad = &mut el.style.descriptor.layout.spacing.padding;
        pad.left = px_val;
        pad.right = px_val;
    }
    if let Some(v) = padding.vertical {
        let px_val = theme.resolve_space(v);
        let pad = &mut el.style.descriptor.layout.spacing.padding;
        pad.top = px_val;
        pad.bottom = px_val;
    }

    for child in children {
        el = el.child(child);
    }

    el
}
