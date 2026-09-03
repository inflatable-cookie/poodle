//! Surface — themed container.
//!
//! Contract: `docs/contracts/components/surface.md` (+ `surface-elevation.md`)
//! Ported from: `packages/jetstream/components/src/surface.rs`.
//!
//! All fills, the border color/width, the radius, and the elevation treatment
//! resolve from tokens or `SurfaceSpec` methods. The CSS color-mix
//! percentages (96% / 98% / 74%) are centralized on the spec.

use poodle_node::Node;
use poodle_specs::SurfaceSpec;

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;

pub fn surface(spec: &SurfaceSpec, ctx: &RenderContext<'_>, children: Vec<Node>) -> Node {
    let padding = spec.resolved_padding();

    // ── Fill (contract §8) ──────────────────────────────────────
    //   panel/base: color-mix(background-surface 96%, transparent) → alpha-only
    //   canvas:     color-mix(background-canvas 98%, transparent)  → alpha-only
    //   elevated:   color-mix(background-elevated 96%, background-panel)
    let base_fill = ctx.theme().resolve_color(spec.resolved_background_token());
    let mix_ratio = spec.fill_mix_ratio();
    let bg = match spec.fill_mix_over_token() {
        Some(over_token) => mix_srgb(base_fill, ctx.theme().resolve_color(over_token), mix_ratio),
        // Non-elevated tones mix toward transparent → scale alpha only.
        None => with_alpha(base_fill, base_fill.3 * mix_ratio),
    };

    let mut el = Node::container();
    if let Some(role) = spec.role {
        el.a11y.role = Some(match role {
            poodle_specs::SurfaceRole::Group => poodle_node::NodeRole::Group,
            poodle_specs::SurfaceRole::Region => poodle_node::NodeRole::Region,
        });
    }
    if let Some(label) = &spec.label {
        el.a11y.label = Some(label.clone());
    }

    {
        let s = &mut el.style;
        // Explicit Row (see switch.rs): the old surface kept the default.
        s.descriptor.layout.direction = poodle_node::LayoutDirection::Row;
        s.descriptor.background = Some(bg);
        let r = ctx.theme().resolve_radius(spec.radius_token());
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
        let base_border = ctx.theme().resolve_color(border_token);
        let s = &mut el.style;
        s.descriptor.border.color =
            with_alpha(base_border, base_border.3 * spec.border_mix_ratio());
        s.descriptor.border.width = spec
            .resolved_border_width()
            .map(|t| ctx.theme().resolve_border_width(t))
            .unwrap_or(0.0);
    }

    // ── Elevation shadow (contract §8) ──────────────────────────
    // Elevated surfaces resolve `elevation.surface` — token-accurate.
    if spec.is_elevated_resolved() {
        el.style.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_SURFACE);
    }

    if let Some(h) = padding.horizontal {
        let px_val = ctx.theme().resolve_space(h);
        let pad = &mut el.style.descriptor.layout.spacing.padding;
        pad.left = px_val;
        pad.right = px_val;
    }
    if let Some(v) = padding.vertical {
        let px_val = ctx.theme().resolve_space(v);
        let pad = &mut el.style.descriptor.layout.spacing.padding;
        pad.top = px_val;
        pad.bottom = px_val;
    }

    for child in children {
        el = el.child(child);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{PaddingScale, SurfaceBorder, SurfaceRole, SurfaceTone};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn surface_resolves_tones_borders_elevation_and_padding() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);

        // Panel tone (default)
        let panel_spec = SurfaceSpec::new().with_tone(SurfaceTone::Panel);
        let panel_node = surface(&panel_spec, &ctx, vec![]);
        assert!(panel_node.style.descriptor.background.is_some());
        assert_eq!(panel_node.style.descriptor.shadow, None);

        // Canvas tone
        let canvas_spec = SurfaceSpec::new().with_tone(SurfaceTone::Canvas);
        let canvas_node = surface(&canvas_spec, &ctx, vec![]);
        assert_ne!(
            canvas_node.style.descriptor.background,
            panel_node.style.descriptor.background
        );

        // Elevated tone
        let elevated_spec = SurfaceSpec::new().with_tone(SurfaceTone::Elevated);
        let elevated_node = surface(&elevated_spec, &ctx, vec![]);
        assert_eq!(
            elevated_node.style.descriptor.shadow,
            Some(poodle_tokens::typed::semantic::ELEVATION_SURFACE)
        );

        // Border variants
        let subtle = surface(
            &SurfaceSpec::new().with_border(SurfaceBorder::Subtle),
            &ctx,
            vec![],
        );
        let default_border = surface(
            &SurfaceSpec::new().with_border(SurfaceBorder::Default),
            &ctx,
            vec![],
        );
        let no_border = surface(
            &SurfaceSpec::new().with_border(SurfaceBorder::None),
            &ctx,
            vec![],
        );
        assert_ne!(
            subtle.style.descriptor.border.color,
            default_border.style.descriptor.border.color
        );
        assert_eq!(no_border.style.descriptor.border.width, 0.0);

        // Radius
        let expected_radius = ctx
            .theme()
            .resolve_radius(poodle_tokens::semantic::RADIUS_SURFACE);
        assert_eq!(panel_node.style.descriptor.corner_radii.top_left, expected_radius);

        // Padding scales
        let pad_none = surface(
            &SurfaceSpec::new().with_padding(PaddingScale::None),
            &ctx,
            vec![],
        );
        let pad_sm = surface(
            &SurfaceSpec::new().with_padding(PaddingScale::Sm),
            &ctx,
            vec![],
        );
        let pad_md = surface(
            &SurfaceSpec::new().with_padding(PaddingScale::Md),
            &ctx,
            vec![],
        );
        let pad_lg = surface(
            &SurfaceSpec::new().with_padding(PaddingScale::Lg),
            &ctx,
            vec![],
        );
        assert_eq!(pad_none.style.descriptor.layout.spacing.padding.left, 0.0);
        assert!(
            pad_sm.style.descriptor.layout.spacing.padding.left
                < pad_md.style.descriptor.layout.spacing.padding.left
        );
        assert!(
            pad_md.style.descriptor.layout.spacing.padding.top
                < pad_lg.style.descriptor.layout.spacing.padding.top
        );
    }

    #[test]
    fn surface_maps_semantics_and_contains_children() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);

        // Decorative (no role)
        let deco = surface(&SurfaceSpec::new(), &ctx, vec![]);
        assert_eq!(deco.a11y.role, None);
        assert_eq!(deco.a11y.label, None);

        // Region role + label
        let region = surface(
            &SurfaceSpec::new()
                .with_role(SurfaceRole::Region)
                .with_label("Settings section"),
            &ctx,
            vec![],
        );
        assert_eq!(region.a11y.role, Some(poodle_node::NodeRole::Region));
        assert_eq!(region.a11y.label.as_deref(), Some("Settings section"));

        // Group role
        let group = surface(
            &SurfaceSpec::new().with_role(SurfaceRole::Group),
            &ctx,
            vec![],
        );
        assert_eq!(group.a11y.role, Some(poodle_node::NodeRole::Group));

        // Child containment
        let child = Node::text("Child text");
        let parent = surface(&SurfaceSpec::new(), &ctx, vec![child]);
        assert_eq!(parent.children.len(), 1);
        match &parent.children[0].kind {
            poodle_node::NodeKind::Text { content } => assert_eq!(content, "Child text"),
            _ => panic!("Expected child text node"),
        }
    }
}
