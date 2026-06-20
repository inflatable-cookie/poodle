//! NavigationMenu — Jetstream nav menu backed by NavigationMenuSpec.
//!
//! Contract: `docs/contracts/components/navigation-menu.md`
//! Reference: `packages/svelte/components/src/NavigationMenu.svelte`
//!
//! Triggers are pill-style buttons: idle = surface 88% bg + border-subtle 72%
//! border; active (open) = accent 16% bg + (accent 42% blended with
//! border-default) border. All metrics resolve from tokens.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::NavigationMenuSpec;

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{
    color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius, tint,
};

pub fn js_navigation_menu(spec: &NavigationMenuSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    // List gap = --poodle-space-inline-sm (contract §7/§8).
    let list_gap = resolve_px(theme, "space.inline.sm");

    // Trigger pill geometry.
    let radius = resolve_radius(theme, spec.trigger_radius_token());
    // Border width = 0.0625rem (contract §8 trigger border), matching peers.
    let border_w = rem_to_px(0.0625);

    let text_primary = resolve_color(theme, "color.text.primary");
    let accent = resolve_color(theme, "color.accent.base");
    let surface = resolve_color(theme, "color.background.surface");
    let border_subtle = resolve_color(theme, "color.border.subtle");
    let border_default = resolve_color(theme, "color.border.default");

    // Idle trigger (contract §8 base):
    //   background = color-mix(surface 88%, transparent)
    //   border     = color-mix(border-subtle 72%, transparent)
    let idle_bg = tint(surface, 0.88);
    let idle_border = tint(border_subtle, 0.72);

    // Active (open) trigger (contract §8 "Open (active)"):
    //   background   = color-mix(accent 16%, transparent)
    //   border-color = color-mix(accent 42%, border-default)
    let active_bg = tint(accent, 0.16);
    let active_border = color_mix(accent, border_default, 0.42);

    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    let current = spec.current_value();

    let mut el = ui_element::div().flex_row().items_center().gap(list_gap);

    for entry in &spec.items {
        let is_active = current == Some(entry.value.as_str());

        // Text stays primary in both states (contract §8 trigger `color`);
        // weight 600 per the trigger typography.
        let (bg, border_color) = if is_active {
            (active_bg, active_border)
        } else {
            (idle_bg, idle_border)
        };

        let mut btn = ui_element::button(&entry.label)
            .text_color(text_primary)
            .text_size(font_size)
            .text_weight(600)
            .pl(pad_x)
            .pr(pad_x)
            .rounded(radius)
            .border(border_w)
            .border_color(border_color)
            .bg(bg)
            .focusable()
            .cursor_pointer();

        if entry.is_disabled {
            btn = btn.opacity(disabled_opacity).disabled(true);
        }

        el = el.child(btn);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::NavigationMenuEntry;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn vec4_to_probe(c: glam::Vec4) -> ProbeColor {
        ProbeColor {
            r: c.x,
            g: c.y,
            b: c.z,
            a: c.w,
        }
    }

    fn spec() -> NavigationMenuSpec {
        NavigationMenuSpec::new(vec![
            NavigationMenuEntry::new("home", "Home"),
            NavigationMenuEntry::new("components", "Components"),
        ])
        .with_value("components")
    }

    #[test]
    fn active_item_uses_accent_fill_and_border() {
        let th = theme();
        let el = js_navigation_menu(&spec(), &th);
        let tree = probe(&el, 400.0, 80.0);

        let accent = resolve_color(&th, "color.accent.base");
        let surface = resolve_color(&th, "color.background.surface");
        let border_default = resolve_color(&th, "color.border.default");

        let active_bg = vec4_to_probe(tint(accent, 0.16));
        let idle_bg = vec4_to_probe(tint(surface, 0.88));

        // The active trigger renders with the accent-16% fill...
        assert!(
            tree.has_background(active_bg, 0.01),
            "active trigger fill missing (expected accent 16%): {}",
            tree.to_json()
        );
        // ...and an inactive trigger renders with the idle surface-88% fill,
        // proving active and inactive are styled differently.
        assert!(
            tree.has_background(idle_bg, 0.01),
            "idle trigger fill missing (expected surface 88%): {}",
            tree.to_json()
        );
        // The active fill must differ from the idle fill.
        assert!(
            !active_bg.approx(idle_bg, 0.01),
            "active and idle fills should differ"
        );

        // Border color is not surfaced through ProbeNode, so assert the active
        // trigger's blended border directly on the built JsEl tree: the second
        // item ("components") is active and must carry the accent-42%/border-
        // default blend, distinct from the inactive item's border-subtle 72%.
        let active_border = color_mix(accent, border_default, 0.42);
        let idle_border = tint(resolve_color(&th, "color.border.subtle"), 0.72);

        let active_trigger = &el.children[1];
        let inactive_trigger = &el.children[0];
        let active_bc = active_trigger.style.border_color.expect("active border");
        let inactive_bc = inactive_trigger.style.border_color.expect("idle border");

        assert!(
            (active_bc.r - active_border.x).abs() < 0.01
                && (active_bc.g - active_border.y).abs() < 0.01
                && (active_bc.b - active_border.z).abs() < 0.01,
            "active border should be accent 42% blended with border-default"
        );
        assert!(
            (inactive_bc.r - idle_border.x).abs() < 0.01
                && (inactive_bc.g - idle_border.y).abs() < 0.01
                && (inactive_bc.b - idle_border.z).abs() < 0.01,
            "idle border should be border-subtle 72%"
        );
    }

    #[test]
    fn trigger_uses_pill_radius_and_border_width() {
        let th = theme();
        let el = js_navigation_menu(&spec(), &th);
        // The first child trigger should carry the control radius + 1px border.
        let expected_radius = resolve_radius(&th, "radius.control");
        let trigger = &el.children[0];
        assert!(
            (trigger.style.corner_radii[0] - expected_radius).abs() < 0.01,
            "trigger radius should resolve from radius.control"
        );
        assert!(
            (trigger.style.border_width - rem_to_px(0.0625)).abs() < 0.01,
            "trigger border width should be 0.0625rem"
        );
    }
}
