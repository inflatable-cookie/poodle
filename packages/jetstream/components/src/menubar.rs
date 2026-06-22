//! Menubar — Jetstream horizontal menu bar backed by MenubarSpec.
//!
//! Contract: `docs/contracts/components/menubar.md`
//! Reference: `packages/svelte/components/src/Menubar.svelte`
//!
//! Anatomy (contract §2):
//!   Root → List (visual chrome) → Group* → { Trigger, Overlay (when open) }
//!                                              Overlay → { Item | Separator }*
//!
//! The List provides the bordered chrome around the trigger strip (border-subtle
//! 72%, radius-surface, panel-96% bg, 0.1875rem padding, 0.125rem gap). Each
//! Trigger is a borderless label button (radius-control, label typography, weight
//! 600); the open trigger gets an accent-14% fill (contract §8 "Open / Hover /
//! Focus"). When a menu is open its Overlay renders below the trigger row with
//! the submenu items: action rows (label + optional shortcut meta), checkbox/radio
//! rows, and separators.
//!
//! Overlay placement: the Svelte overlay is `position: absolute` under its
//! specific trigger group; JsEl has no absolute positioning here, so the overlay
//! renders in document flow below the trigger row (an accepted GPUI/Jetstream
//! layout delta, mirroring the GPUI menubar). The contract `box-shadow`
//! (`elevation.overlay`) is approximated with `shadow_md()` — JsEl exposes only a
//! preset medium shadow, not a token-driven box-shadow channel; noted as a runtime
//! limit. Interaction (trigger click, item nav, outside-click close) lives in the
//! preview event loop.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{MenuItemKind, MenubarSpec};

use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{
    color_mix, elevation_overlay, resolve_color, resolve_opacity, resolve_px, resolve_radius, tint,
};

/// Trigger / item label weight. Font weight is a typography constant (no px
/// dimension); mirrors the GPUI `FontWeight::SEMIBOLD` and the contract's
/// `typography.label.weight` (600), following the `form_shell.rs` convention.
const LABEL_WEIGHT: u16 = 600;

pub fn js_menubar(spec: &MenubarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    // Item body font (contract item base `typography.body.size`); meta is smaller.
    let item_font = resolve_px(theme, "typography.body.size");
    let meta_font = rem_to_px(0.6875); // contract §8 Meta font-size 0.6875rem
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let item_pad_y = resolve_px(theme, "space.control.y");

    // Trigger / list geometry.
    let control_height = resolve_px(theme, "size.control.height");
    let control_radius = resolve_radius(theme, "radius.control");
    let list_radius = resolve_radius(theme, spec.list_radius_token());
    let border_w = rem_to_px(0.0625); // contract §8 list/overlay border width
    // List gap 0.125rem / padding 0.1875rem (contract §8 List — literal rems, not
    // tokens; the spec's trigger_gap_token resolves to inline-sm which is unused).
    let list_gap = rem_to_px(0.125);
    let list_pad = rem_to_px(0.1875);
    // Overlay min-width 12rem, padding 0.25rem (contract §8 Overlay).
    let overlay_min_w = rem_to_px(12.0);
    let overlay_pad = rem_to_px(0.25);
    // Item radius = calc(radius.control - 0.125rem) (contract §8 Item).
    let item_radius = (control_radius - rem_to_px(0.125)).max(0.0);
    // Separator height 0.0625rem, vertical margin 0.25rem (contract §8 Separator).
    let separator_h = rem_to_px(0.0625);
    let separator_my = rem_to_px(0.25);
    let item_gap = rem_to_px(0.5); // label↔meta gap

    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let panel = resolve_color(theme, "color.background.panel");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border_subtle = resolve_color(theme, spec.list_border_token());
    let border_default = resolve_color(theme, "color.border.default");

    // List chrome (contract §8 List): border-subtle 72%, panel 96%.
    let list_border = tint(border_subtle, 0.72);
    let list_bg = tint(panel, 0.96);
    // Trigger open/hover/focus (contract §8): accent 14%.
    let open_bg = tint(accent, 0.14);
    // Overlay (contract §8): bg = color-mix(elevated 98%, panel); border = border-default 72%.
    let overlay_bg = color_mix(elevated, panel, 0.98);
    let overlay_border = tint(border_default, 0.72);
    // Item hover/focus (contract §8 Item): accent 16%.
    let item_hover = tint(accent, 0.16);
    // Separator (contract §8): border-subtle 72%.
    let separator_color = tint(border_subtle, 0.72);

    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
    let open_value = spec.current_value();

    // List `.menubar__list`: bordered, padded trigger strip (contract §8 List).
    let mut list = ui_element::div()
        .flex_row()
        .items_center()
        .gap(list_gap)
        .pt(list_pad)
        .pb(list_pad)
        .pl(list_pad)
        .pr(list_pad)
        .rounded(list_radius)
        .border(border_w)
        .border_color(list_border)
        .bg(list_bg);

    for entry in &spec.items {
        let is_open = open_value == Some(entry.value.as_str());

        // Trigger `.menubar__trigger`: borderless, radius-control, label weight
        // 600, min-height control-height (contract §8 Trigger). Text stays
        // text-primary in all states (contract §8 trigger `color`).
        let mut btn = ui_element::button(&entry.label)
            .text_color(text_primary)
            .text_size(font_size)
            .text_weight(LABEL_WEIGHT)
            .min_h(control_height)
            .items_center()
            .pl(pad_x)
            .pr(pad_x)
            .rounded(control_radius)
            .focusable()
            .cursor_pointer();

        if is_open {
            // Open trigger: accent-14% fill (contract §8 Open/Hover/Focus).
            btn = btn.bg(open_bg);
        }

        if entry.is_disabled {
            btn = btn.opacity(disabled_opacity).disabled(true);
        } else {
            // Hover/focus shares the open treatment (accent 14%).
            btn = btn.hover(move |s| s.bg(open_bg));
        }

        list = list.child(btn);
    }

    // Root `.menubar`: inline-flex wrapper. Modeled as a column so the open
    // overlay can render below the trigger row (flow placement — accepted delta).
    let mut root = ui_element::div().flex_col().min_w_0().child(list);

    // Overlay `.menubar__overlay` — rendered only for the open menu (contract §2
    // conditional / §4 "menu open"). Renders the submenu items.
    if let Some(menu) = spec.current_menu() {
        if !menu.items.is_empty() {
            // Token-accurate `elevation-overlay` from the typed semantic token
            // via the runtime shadow builder (single layer, spread 0; matches
            // GPUI's mapping).
            let mut overlay = elevation_overlay(
                ui_element::div()
                    .flex_col()
                    .min_w(overlay_min_w)
                    .pt(overlay_pad)
                    .pb(overlay_pad)
                    .pl(overlay_pad)
                    .pr(overlay_pad)
                    .rounded(list_radius)
                    .border(border_w)
                    .border_color(overlay_border)
                    .bg(overlay_bg),
            )
            .mt(overlay_pad) // ~0.25rem gap below the trigger row (contract overlay top offset)
            .overlay();

            for item in &menu.items {
                match item.kind {
                    // Separator `.menubar__separator`: 0.0625rem divider with
                    // 0.25rem vertical margin (contract §8 Separator).
                    MenuItemKind::Separator => {
                        overlay = overlay.child(
                            ui_element::div()
                                .h(separator_h)
                                .bg(separator_color)
                                .mt(separator_my)
                                .mb(separator_my),
                        );
                    }
                    // Item `.menubar__item`: grid [label | meta], radius
                    // calc(radius.control - 0.125rem), body font, control y/x
                    // padding (contract §8 Item).
                    _ => {
                        let mut row = ui_element::div()
                            .flex_row()
                            .items_center()
                            .gap(item_gap)
                            .min_h(control_height)
                            .pt(item_pad_y)
                            .pb(item_pad_y)
                            .pl(pad_x)
                            .pr(pad_x)
                            .rounded(item_radius)
                            .focusable()
                            .child(
                                ui_element::label(&item.label)
                                    .text_color(text_primary)
                                    .text_size(item_font)
                                    .grow(),
                            );

                        // Meta `.menubar__meta`: shortcut label (or check glyph
                        // for checked items) in text-secondary, 0.6875rem
                        // (contract §8 Meta). Code-family is approximated — JsEl
                        // has no font-family channel.
                        if item.is_checked {
                            row = row.child(
                                ui_element::label("✓")
                                    .text_color(text_secondary)
                                    .text_size(meta_font),
                            );
                        } else if let Some(ref shortcut) = item.shortcut_label {
                            row = row.child(
                                ui_element::label(shortcut)
                                    .text_color(text_secondary)
                                    .text_size(meta_font),
                            );
                        }

                        if item.is_disabled {
                            row = row.opacity(disabled_opacity);
                        } else {
                            row = row.cursor_pointer().hover(move |s| s.bg(item_hover));
                        }

                        overlay = overlay.child(row);
                    }
                }
            }

            root = root.child(overlay);
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{MenuEntry, MenubarEntry};

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

    fn file_menu() -> MenubarEntry {
        MenubarEntry::new(
            "file",
            "File",
            vec![
                MenuEntry::new("new", "New").with_shortcut_label("Cmd+N"),
                MenuEntry::new("open", "Open...").with_shortcut_label("Cmd+O"),
                MenuEntry::new("sep", "").with_kind(MenuItemKind::Separator),
                MenuEntry::new("quit", "Quit")
                    .with_shortcut_label("Cmd+Q")
                    .with_disabled(true),
            ],
        )
    }

    fn spec() -> MenubarSpec {
        MenubarSpec::new(vec![
            file_menu(),
            MenubarEntry::new("edit", "Edit", vec![MenuEntry::new("undo", "Undo")]),
        ])
        .with_value("file")
    }

    #[test]
    fn list_chrome_renders_with_panel_bg_and_triggers() {
        let th = theme();
        let el = js_menubar(&spec(), &th);
        let tree = probe(&el, 480.0, 240.0);

        // List chrome carries the panel-96% background.
        let list_bg = vec4_to_probe(tint(resolve_color(&th, "color.background.panel"), 0.96));
        assert!(
            tree.has_background(list_bg, 0.01),
            "menubar list chrome bg missing (expected panel 96%): {}",
            tree.to_json()
        );

        // Both top-level triggers render.
        assert!(tree.has_text("File"), "File trigger missing: {:?}", tree.texts());
        assert!(tree.has_text("Edit"), "Edit trigger missing: {:?}", tree.texts());

        // The list node (children[0] of root) carries radius-surface + the
        // border-subtle 72% border.
        let list = &el.children[0];
        let expected_radius = resolve_radius(&th, "radius.surface");
        assert!(
            (list.style.corner_radii[0] - expected_radius).abs() < 0.01,
            "list radius should resolve from radius.surface"
        );
        let list_border = tint(resolve_color(&th, "color.border.subtle"), 0.72);
        let bc = list.style.border_color.expect("list border");
        assert!(
            (bc.r - list_border.x).abs() < 0.01
                && (bc.g - list_border.y).abs() < 0.01
                && (bc.b - list_border.z).abs() < 0.01,
            "list border should be border-subtle 72%"
        );
    }

    #[test]
    fn open_trigger_uses_accent_fill() {
        let th = theme();
        let el = js_menubar(&spec(), &th);
        // The open trigger ("file") carries the accent-14% fill.
        let open_bg = tint(resolve_color(&th, "color.accent.base"), 0.14);
        let list = &el.children[0];
        let open_trigger = &list.children[0];
        let bg = open_trigger.style.background.expect("open trigger fill");
        assert!(
            (bg.r - open_bg.x).abs() < 0.01
                && (bg.g - open_bg.y).abs() < 0.01
                && (bg.b - open_bg.z).abs() < 0.01
                && (bg.a - open_bg.w).abs() < 0.01,
            "open trigger fill should be accent 14%"
        );
        // Trigger weight is the label-weight constant (600).
        assert_eq!(open_trigger.style.text_weight, Some(LABEL_WEIGHT));
    }

    #[test]
    fn open_menu_renders_items_separators_and_shortcuts() {
        let th = theme();
        let el = js_menubar(&spec(), &th);
        let tree = probe(&el, 480.0, 320.0);

        // The overlay renders the File menu's items + shortcut meta.
        assert!(tree.has_text("New"), "New item missing: {:?}", tree.texts());
        assert!(tree.has_text("Open..."), "Open item missing: {:?}", tree.texts());
        assert!(tree.has_text("Cmd+N"), "shortcut meta missing: {:?}", tree.texts());
        assert!(tree.has_text("Quit"), "Quit item missing: {:?}", tree.texts());

        // Overlay node (root.children[1]) carries elevated-mix bg + border-default 72%.
        assert_eq!(el.children.len(), 2, "root should hold list + overlay");
        let overlay = &el.children[1];
        let overlay_bg = color_mix(
            resolve_color(&th, "color.background.elevated"),
            resolve_color(&th, "color.background.panel"),
            0.98,
        );
        let bg = overlay.style.background.expect("overlay bg");
        assert!(
            (bg.r - overlay_bg.x).abs() < 0.01
                && (bg.g - overlay_bg.y).abs() < 0.01
                && (bg.b - overlay_bg.z).abs() < 0.01,
            "overlay bg should be elevated 98% over panel"
        );
        let overlay_border = tint(resolve_color(&th, "color.border.default"), 0.72);
        let bc = overlay.style.border_color.expect("overlay border");
        assert!(
            (bc.r - overlay_border.x).abs() < 0.01
                && (bc.g - overlay_border.y).abs() < 0.01
                && (bc.b - overlay_border.z).abs() < 0.01,
            "overlay border should be border-default 72%"
        );

        // A separator divider exists in the overlay (a thin Panel child).
        let separator_color = tint(resolve_color(&th, "color.border.subtle"), 0.72);
        let has_separator = overlay
            .children
            .iter()
            .any(|c| c.style.background.is_some_and(|b| {
                (b.r - separator_color.x).abs() < 0.01
                    && (b.g - separator_color.y).abs() < 0.01
                    && (b.b - separator_color.z).abs() < 0.01
            }));
        assert!(has_separator, "overlay should contain a separator divider");
    }

    #[test]
    fn no_overlay_when_menu_empty() {
        let th = theme();
        // Open menu has no items → no overlay renders.
        let nav = MenubarSpec::new(vec![MenubarEntry::new("empty", "Empty", vec![])])
            .with_value("empty");
        let el = js_menubar(&nav, &th);
        assert_eq!(
            el.children.len(),
            1,
            "no overlay should render when the open menu has no items"
        );
    }
}
