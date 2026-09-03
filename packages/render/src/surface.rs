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
    use poodle_tokens::semantic;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn surface_resolves_exact_contract_tones_borders_elevation_radius_and_padding() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);

        // ── Exact Background Tones ─────────────────────────────────────
        // Panel tone (default): background-surface 96% alpha
        let panel_spec = SurfaceSpec::new().with_tone(SurfaceTone::Panel);
        let panel_node = surface(&panel_spec, &ctx, vec![]);
        let base_surface = ctx.theme().resolve_color(semantic::COLOR_BACKGROUND_SURFACE);
        let expected_panel_bg = with_alpha(base_surface, base_surface.3 * 0.96);
        assert_eq!(
            panel_node.style.descriptor.background,
            Some(expected_panel_bg),
            "Panel tone must resolve exact surface background with 96% alpha"
        );
        assert_eq!(panel_node.style.descriptor.shadow, None);

        // Canvas tone: background-canvas 98% alpha
        let canvas_spec = SurfaceSpec::new().with_tone(SurfaceTone::Canvas);
        let canvas_node = surface(&canvas_spec, &ctx, vec![]);
        let base_canvas = ctx.theme().resolve_color(semantic::COLOR_BACKGROUND_CANVAS);
        let expected_canvas_bg = with_alpha(base_canvas, base_canvas.3 * 0.98);
        assert_eq!(
            canvas_node.style.descriptor.background,
            Some(expected_canvas_bg),
            "Canvas tone must resolve exact canvas background with 98% alpha"
        );

        // Elevated tone: background-elevated 96% mixed over background-panel
        let elevated_spec = SurfaceSpec::new().with_tone(SurfaceTone::Elevated);
        let elevated_node = surface(&elevated_spec, &ctx, vec![]);
        let base_elevated = ctx.theme().resolve_color(semantic::COLOR_BACKGROUND_ELEVATED);
        let base_panel = ctx.theme().resolve_color(semantic::COLOR_BACKGROUND_PANEL);
        let expected_elevated_bg = mix_srgb(base_elevated, base_panel, 0.96);
        assert_eq!(
            elevated_node.style.descriptor.background,
            Some(expected_elevated_bg),
            "Elevated tone must resolve exact elevated background mixed over panel"
        );
        assert_eq!(
            elevated_node.style.descriptor.shadow,
            Some(poodle_tokens::typed::semantic::ELEVATION_SURFACE),
            "Elevated tone must carry the elevation shadow"
        );

        // Elevated via flag on panel tone
        let panel_elevated = surface(
            &SurfaceSpec::new()
                .with_tone(SurfaceTone::Panel)
                .with_elevation(true),
            &ctx,
            vec![],
        );
        assert_eq!(
            panel_elevated.style.descriptor.shadow,
            Some(poodle_tokens::typed::semantic::ELEVATION_SURFACE)
        );

        // ── Exact Border Variants ──────────────────────────────────────
        let subtle = surface(
            &SurfaceSpec::new().with_border(SurfaceBorder::Subtle),
            &ctx,
            vec![],
        );
        let base_border_subtle = ctx.theme().resolve_color(semantic::COLOR_BORDER_SUBTLE);
        let expected_border_subtle_color =
            with_alpha(base_border_subtle, base_border_subtle.3 * 0.74);
        let expected_border_width = ctx
            .theme()
            .resolve_border_width(semantic::BORDER_WIDTH_DEFAULT);
        assert_eq!(
            subtle.style.descriptor.border.color,
            expected_border_subtle_color,
            "Subtle border must resolve exact border-subtle color with 74% alpha"
        );
        assert_eq!(
            subtle.style.descriptor.border.width, expected_border_width,
            "Subtle border must resolve exact default border width"
        );

        let default_border = surface(
            &SurfaceSpec::new().with_border(SurfaceBorder::Default),
            &ctx,
            vec![],
        );
        let base_border_default = ctx.theme().resolve_color(semantic::COLOR_BORDER_DEFAULT);
        let expected_border_default_color =
            with_alpha(base_border_default, base_border_default.3 * 1.0);
        assert_eq!(
            default_border.style.descriptor.border.color,
            expected_border_default_color,
            "Default border must resolve exact border-default color"
        );
        assert_eq!(
            default_border.style.descriptor.border.width, expected_border_width,
            "Default border must resolve exact default border width"
        );

        let no_border = surface(
            &SurfaceSpec::new().with_border(SurfaceBorder::None),
            &ctx,
            vec![],
        );
        assert_eq!(no_border.style.descriptor.border.width, 0.0);

        // ── Exact Corner Radii ─────────────────────────────────────────
        let expected_radius = ctx.theme().resolve_radius(semantic::RADIUS_SURFACE);
        assert_eq!(
            panel_node.style.descriptor.corner_radii.top_left,
            expected_radius
        );
        assert_eq!(
            panel_node.style.descriptor.corner_radii.top_right,
            expected_radius
        );
        assert_eq!(
            panel_node.style.descriptor.corner_radii.bottom_right,
            expected_radius
        );
        assert_eq!(
            panel_node.style.descriptor.corner_radii.bottom_left,
            expected_radius
        );

        // ── Exact Padding Scales (Panel Insets) ────────────────────────
        let pad_none = surface(
            &SurfaceSpec::new().with_padding(PaddingScale::None),
            &ctx,
            vec![],
        );
        assert_eq!(pad_none.style.descriptor.layout.spacing.padding.left, 0.0);
        assert_eq!(pad_none.style.descriptor.layout.spacing.padding.right, 0.0);
        assert_eq!(pad_none.style.descriptor.layout.spacing.padding.top, 0.0);
        assert_eq!(pad_none.style.descriptor.layout.spacing.padding.bottom, 0.0);

        let pad_sm = surface(
            &SurfaceSpec::new().with_padding(PaddingScale::Sm),
            &ctx,
            vec![],
        );
        let expected_sm_x = ctx.theme().resolve_space(semantic::SPACE_INLINE_SM);
        let expected_sm_y = ctx.theme().resolve_space(semantic::SPACE_STACK_SM);
        assert_eq!(pad_sm.style.descriptor.layout.spacing.padding.left, expected_sm_x);
        assert_eq!(pad_sm.style.descriptor.layout.spacing.padding.right, expected_sm_x);
        assert_eq!(pad_sm.style.descriptor.layout.spacing.padding.top, expected_sm_y);
        assert_eq!(pad_sm.style.descriptor.layout.spacing.padding.bottom, expected_sm_y);

        let pad_md = surface(
            &SurfaceSpec::new().with_padding(PaddingScale::Md),
            &ctx,
            vec![],
        );
        let expected_md_x = ctx.theme().resolve_space(semantic::SPACE_PANEL_X);
        let expected_md_y = ctx.theme().resolve_space(semantic::SPACE_PANEL_Y);
        assert_eq!(pad_md.style.descriptor.layout.spacing.padding.left, expected_md_x);
        assert_eq!(pad_md.style.descriptor.layout.spacing.padding.right, expected_md_x);
        assert_eq!(pad_md.style.descriptor.layout.spacing.padding.top, expected_md_y);
        assert_eq!(pad_md.style.descriptor.layout.spacing.padding.bottom, expected_md_y);

        let pad_lg = surface(
            &SurfaceSpec::new().with_padding(PaddingScale::Lg),
            &ctx,
            vec![],
        );
        let expected_lg_x = ctx.theme().resolve_space(semantic::SPACE_INLINE_LG);
        let expected_lg_y = ctx.theme().resolve_space(semantic::SPACE_STACK_LG);
        assert_eq!(pad_lg.style.descriptor.layout.spacing.padding.left, expected_lg_x);
        assert_eq!(pad_lg.style.descriptor.layout.spacing.padding.right, expected_lg_x);
        assert_eq!(pad_lg.style.descriptor.layout.spacing.padding.top, expected_lg_y);
        assert_eq!(pad_lg.style.descriptor.layout.spacing.padding.bottom, expected_lg_y);
    }

    struct MockBorderWidthResolverTheme;
    impl poodle_adapter::ThemeProvider for MockBorderWidthResolverTheme {
        fn resolve_color(&self, _: &str) -> poodle_node::ColorValue {
            poodle_node::ColorValue(0.0, 0.0, 0.0, 1.0)
        }
        fn resolve_space(&self, token: &str) -> f32 {
            // Space resolver returns 0.0 for border width tokens, matching GpuiThemeProvider
            if token == semantic::BORDER_WIDTH_DEFAULT {
                0.0
            } else {
                16.0
            }
        }
        fn resolve_radius(&self, _: &str) -> f32 {
            4.0
        }
        fn resolve_border_width(&self, token: &str) -> f32 {
            if token == semantic::BORDER_WIDTH_DEFAULT {
                1.0
            } else {
                0.0
            }
        }
        fn resolve_opacity(&self, _: &str) -> f32 {
            1.0
        }
    }

    #[test]
    fn surface_border_width_queries_resolve_border_width_not_resolve_space() {
        let theme = MockBorderWidthResolverTheme;
        let ctx = RenderContext::new(&theme);
        let subtle = surface(
            &SurfaceSpec::new().with_border(SurfaceBorder::Subtle),
            &ctx,
            vec![],
        );
        assert_eq!(
            subtle.style.descriptor.border.width, 1.0,
            "Surface border width must resolve through resolve_border_width (1.0), not resolve_space (0.0)"
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
