//! Popover — Jetstream popover surface backed by PopoverSpec.
//!
//! Contract: `docs/contracts/components/popover.md`
//! Reference: `packages/svelte/components/src/Popover.svelte`
//!
//! Renders the anchored floating surface (the open panel at the current state).
//! Open/close, placement anchoring, outside-dismiss, and Escape live in the
//! preview event loop, not the component (accepted runtime delta — Jetstream
//! has no DOM/overlay-positioning engine in the component layer). `overlay()`
//! lifts the surface above normal content.

use jetstream_runtime::ui_element::{self, BoxShadow, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::PopoverSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{elevation_overlay, resolve_color, resolve_px, resolve_radius, tint};

pub fn js_popover(spec: &PopoverSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    // Contract §8 surface: background = background-elevated, border =
    // border-subtle at 74%, radius = radius-surface.
    let fill = resolve_color(theme, spec.surface_fill_token());
    let border = tint(
        resolve_color(theme, spec.surface_border_token()),
        spec.surface_border_alpha(),
    );
    let border_width = resolve_px(theme, "border.width.default");
    let radius = resolve_radius(theme, "radius.surface");

    // Contract §8 padding = space.panel.y / space.panel.x (token-resolved,
    // matching GPUI), not raw rem literals.
    let pad_x = resolve_px(theme, "space.panel.x");
    let pad_y = resolve_px(theme, "space.panel.y");

    // Contract §7: min-width 14rem, max-width min(24rem, 90vw). The 90vw clamp
    // is viewport-relative and not expressible here, so the 24rem arm is used.
    // Both overridable via surfaceMinWidth/surfaceMaxWidth (resolved in spec).
    let min_w = rem_to_px(spec.effective_surface_min_width_rem());
    let max_w = rem_to_px(spec.effective_surface_max_width_rem());

    // Contract §8 box-shadow is a 3-layer stack: an inset top highlight over two
    // drop shadows. `elevation_overlay` supplies the token-accurate primary drop;
    // the contract's `inset 0 0.0625rem 0 rgba(255,255,255,0.08)` top highlight is
    // layered on top via `shadow_layers` (rendered alongside the primary shadow).
    let mut el = elevation_overlay(
        ui_element::div()
            .bg(fill)
            .border(border_width)
            .border_color(border)
            .rounded(radius)
            .pl(pad_x)
            .pr(pad_x)
            .pt(pad_y)
            .pb(pad_y)
            .min_w(min_w)
            .max_w(max_w)
            .shadow_layers(vec![BoxShadow {
                offset_x: 0.0,
                offset_y: rem_to_px(0.0625),
                blur: 0.0,
                spread: 0.0,
                color: glam::Vec4::new(1.0, 1.0, 1.0, 0.08),
                inset: true,
            }]),
    )
    .overlay(); // Render above normal content.

    if let Some(content_el) = content {
        el = el.child(content_el);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    /// Surface paints the elevated background fill and carries a border.
    #[test]
    fn surface_uses_elevated_fill() {
        let th = theme();
        let spec = PopoverSpec::new();
        let el = js_popover(&spec, &th, None);
        let tree = probe(&el, 400.0, 300.0);

        let fill = resolve_color(&th, spec.surface_fill_token());
        let fill_color = ProbeColor {
            r: fill.x,
            g: fill.y,
            b: fill.z,
            a: fill.w,
        };
        assert!(
            tree.has_background(fill_color, 0.02),
            "popover surface must paint the elevated background fill"
        );
        // Border width is applied (token-resolved).
        assert!(
            el.style.border_width > 0.0,
            "popover surface must carry a border"
        );
    }

    /// Surface enforces the contract min-width (14rem default).
    #[test]
    fn surface_applies_min_width() {
        let el = js_popover(&PopoverSpec::new(), &theme(), None);
        // 14rem * 16 = 224px.
        assert_eq!(el.layout.min_size.width, taffy::Dimension::length(224.0));
    }

    /// surfaceMinWidth override changes the resolved min-width.
    #[test]
    fn surface_min_width_override() {
        let spec = PopoverSpec::new().with_surface_min_width_rem(20.0);
        let el = js_popover(&spec, &theme(), None);
        assert_eq!(el.layout.min_size.width, taffy::Dimension::length(320.0));
    }

    /// Body content renders inside the surface.
    #[test]
    fn surface_renders_body_content() {
        let content = ui_element::label("Quick settings");
        let el = js_popover(&PopoverSpec::new(), &theme(), Some(content));
        let tree = probe(&el, 400.0, 300.0);
        assert!(
            tree.has_text("Quick settings"),
            "popover body content missing: {:?}",
            tree.texts()
        );
    }

    /// Header + body regions (multiple text children) both render.
    #[test]
    fn surface_renders_header_and_body_regions() {
        let content = ui_element::div()
            .flex_col()
            .child(ui_element::label("Settings"))
            .child(ui_element::label("Adjust your preferences."));
        let el = js_popover(&PopoverSpec::new(), &theme(), Some(content));
        let tree = probe(&el, 400.0, 300.0);
        assert!(tree.has_text("Settings"), "header region missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Adjust your preferences."),
            "body region missing: {:?}",
            tree.texts()
        );
    }
}
