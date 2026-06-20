//! NavigationMenu — Jetstream nav menu backed by NavigationMenuSpec.
//!
//! Contract: `docs/contracts/components/navigation-menu.md`
//! Reference: `packages/svelte/components/src/NavigationMenu.svelte`
//!
//! Triggers are pill-style buttons: idle = surface 88% bg + border-subtle 72%
//! border; active (open) = accent 16% bg + (accent 42% blended with
//! border-default) border. All metrics resolve from tokens.
//!
//! When an item is active, a viewport panel (contract §2/§8) renders below the
//! trigger row: panel-96% bg, border-subtle 74% border, radius-surface, panel
//! x/y padding. Its content is the active item's `description` (the Rust-only
//! viewport content shortcut GPUI uses); when the active item carries no
//! description, only the panel chrome renders. The contract's `box-shadow`
//! (`elevation.overlay`) is NOT applied — JsEl has no box-shadow channel (only
//! background gradients); this is an accepted runtime limit, noted here.

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

    // List `.navigation-menu__list`: inline-flex row, wrap, gap inline-sm,
    // align center (contract §8 List).
    let mut list = ui_element::div()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(list_gap);

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

        list = list.child(btn);
    }

    // Root `.navigation-menu`: grid container for list + viewport, gap
    // stack-md, min-width 0 (contract §8 Root). Modeled as a column.
    let root_gap = resolve_px(theme, spec.viewport_gap_token());
    let mut root = ui_element::div()
        .flex_col()
        .min_w_0()
        .gap(root_gap)
        .child(list);

    // Viewport `.navigation-menu__viewport` — rendered only when an item is
    // active (contract §2 conditional / §4 "item active"). Contract §8 tokens:
    //   padding      = space-panel-y / space-panel-x
    //   border       = 0.0625rem solid color-mix(border-subtle 74%, transparent)
    //   border-radius= radius-surface (spec.viewport_radius_token())
    //   background   = color-mix(background-panel 96%, transparent)
    //   box-shadow   = elevation-overlay — NOT applied (no JsEl box-shadow).
    if let Some(active_item) = spec.current_item() {
        let panel_x = resolve_px(theme, "space.panel.x");
        let panel_y = resolve_px(theme, "space.panel.y");
        let viewport_radius = resolve_radius(theme, spec.viewport_radius_token());
        let panel_bg = tint(resolve_color(theme, "color.background.panel"), 0.96);
        let viewport_border = tint(border_subtle, 0.74);

        let mut viewport = ui_element::div()
            .flex_col()
            .min_w_0()
            .pt(panel_y)
            .pb(panel_y)
            .pl(panel_x)
            .pr(panel_x)
            .rounded(viewport_radius)
            .border(border_w)
            .border_color(viewport_border)
            .bg(panel_bg);

        // Content slot: the host-provided viewport content for the active item.
        // The Rust spec carries this as the entry's `description` (GPUI's
        // viewport content shortcut). When absent, only the panel chrome
        // renders — no fabricated content.
        if let Some(description) = active_item.description.as_deref() {
            viewport = viewport.child(
                ui_element::label(description)
                    .text_color(text_primary)
                    .text_size(font_size),
            );
        }

        root = root.child(viewport);
    }

    root
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

        // el is now the root column: children[0] is the list; triggers are the
        // list's children.
        let list = &el.children[0];
        let active_trigger = &list.children[1];
        let inactive_trigger = &list.children[0];
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
        let trigger = &el.children[0].children[0];
        assert!(
            (trigger.style.corner_radii[0] - expected_radius).abs() < 0.01,
            "trigger radius should resolve from radius.control"
        );
        assert!(
            (trigger.style.border_width - rem_to_px(0.0625)).abs() < 0.01,
            "trigger border width should be 0.0625rem"
        );
    }

    #[test]
    fn viewport_panel_renders_with_panel_bg_and_border() {
        let th = theme();
        // Active item carries a description → viewport content slot populated.
        let nav = NavigationMenuSpec::new(vec![
            NavigationMenuEntry::new("home", "Home"),
            NavigationMenuEntry::new("components", "Components")
                .with_description("Active section"),
        ])
        .with_value("components");

        let el = js_navigation_menu(&nav, &th);
        let tree = probe(&el, 400.0, 160.0);

        // The viewport panel renders with the panel-96% background...
        let panel_bg = vec4_to_probe(tint(resolve_color(&th, "color.background.panel"), 0.96));
        assert!(
            tree.has_background(panel_bg, 0.01),
            "viewport panel bg missing (expected background-panel 96%): {}",
            tree.to_json()
        );

        // ...and its description content made it through layout.
        assert!(
            tree.has_text("Active section"),
            "viewport content (active item description) missing: {:?}",
            tree.texts()
        );

        // Assert the viewport node directly on the JsEl tree: root → [list,
        // viewport]; the viewport carries radius-surface + border-subtle 74%.
        assert_eq!(el.children.len(), 2, "root should hold list + viewport");
        let viewport = &el.children[1];
        let expected_radius = resolve_radius(&th, "radius.surface");
        assert!(
            (viewport.style.corner_radii[0] - expected_radius).abs() < 0.01,
            "viewport radius should resolve from radius.surface"
        );
        let viewport_border = tint(resolve_color(&th, "color.border.subtle"), 0.74);
        let bc = viewport.style.border_color.expect("viewport border");
        assert!(
            (bc.r - viewport_border.x).abs() < 0.01
                && (bc.g - viewport_border.y).abs() < 0.01
                && (bc.b - viewport_border.z).abs() < 0.01,
            "viewport border should be border-subtle 74%"
        );
    }

    #[test]
    fn no_viewport_when_no_active_item() {
        let th = theme();
        // All items disabled → current_value()/current_item() resolve to None.
        let nav = NavigationMenuSpec::new(vec![
            NavigationMenuEntry::new("home", "Home").with_disabled(true),
        ]);
        let el = js_navigation_menu(&nav, &th);
        // Root holds only the list, no viewport.
        assert_eq!(
            el.children.len(),
            1,
            "no viewport should render when no item is active"
        );
    }
}
