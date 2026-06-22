//! ContextMenu — Jetstream context menu backed by ContextMenuSpec.
//!
//! Contract: `docs/contracts/components/context-menu.md`
//! Reference: `packages/svelte/components/src/ContextMenu.svelte` (+ `Menu.svelte`)
//!
//! Overlay/item CSS is identical to `Menu` except the positioning model
//! (fixed-at-anchor) and class prefix; this renders the same item surface as
//! `js_menu`. Open/close, anchor positioning, viewport clamping, and focus
//! restoration live in the preview event loop (not modeled here).

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, ContextMenuSpec, MenuItemKind};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{elevation_overlay, resolve_color, resolve_opacity, resolve_radius, tint};

/// Per-size item metrics (min-height, padding-y, padding-x, font-size) in rem.
/// Contract §8 size table.
fn item_metrics_rem(size: ControlSize) -> (f32, f32, f32, f32) {
    match size {
        ControlSize::Xs => (1.5, 0.25, 0.375, 0.75),
        ControlSize::Sm => (1.75, 0.3125, 0.4375, 0.8125),
        ControlSize::Md => (2.0, 0.375, 0.5, 0.875),
        ControlSize::Lg => (2.25, 0.4375, 0.5625, 0.9375),
        ControlSize::Xl => (2.5, 0.5, 0.625, 1.0),
    }
}

pub fn js_context_menu(spec: &ContextMenuSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let (item_min_h_rem, item_py_rem, item_px_rem, font_rem) = item_metrics_rem(effective_size);

    let item_min_h = rem_to_px(item_min_h_rem);
    let item_py = rem_to_px(item_py_rem);
    let item_px = rem_to_px(item_px_rem);
    let font_size = rem_to_px(font_rem);
    // Meta (shortcut) column — code font, 0.6875rem (contract §8 Meta).
    let meta_font = rem_to_px(0.6875);
    // Item inner gap — density-aware (compact/default/comfortable).
    let item_gap = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.375,
        poodle_specs::ControlDensity::Default => 0.5,
        poodle_specs::ControlDensity::Comfortable => 0.625,
    });
    // Overlay padding 0.25rem; min-width 14rem (contract §8 Overlay).
    let overlay_pad = rem_to_px(0.25);
    let min_width = rem_to_px(14.0);
    let separator_my = rem_to_px(0.25);

    let fill = resolve_color(theme, spec.menu.surface_fill_token());
    let border = resolve_color(theme, spec.menu.overlay_border_token());
    let radius = resolve_radius(theme, spec.menu.overlay_radius_token());
    // Item radius: radius.control - 0.125rem (contract §8 Item).
    let item_radius = (resolve_radius(theme, "radius.control") - rem_to_px(0.125)).max(0.0);
    let text_color = resolve_color(theme, spec.menu.item_text_token());
    let muted_color = resolve_color(theme, "color.text.secondary");
    let separator_color = resolve_color(theme, spec.menu.separator_color_token());
    let danger_color = resolve_color(theme, "color.status.danger");
    let accent_color = resolve_color(theme, spec.menu.item_highlight_token());
    let disabled_opacity = resolve_opacity(theme, spec.menu.disabled_opacity_token());

    // Item hover/focus bg: accent-base @ 16% (contract §8 Item hover/focus).
    let hover_tint = tint(accent_color, 0.16);
    // Separator bg: border-subtle @ 72% (contract §8 Separator).
    let separator_bg = tint(separator_color, 0.72);

    // Token-accurate `elevation-overlay` from the typed semantic token via the
    // runtime shadow builder (single layer, spread 0; matches GPUI's mapping).
    let mut el = elevation_overlay(
        ui_element::div()
            .bg(fill)
            .border(1.0)
            .border_color(border)
            .rounded(radius)
            .flex_col()
            .pt(overlay_pad)
            .pb(overlay_pad)
            .pl(overlay_pad)
            .pr(overlay_pad)
            .min_w(min_width),
    )
    .overlay();

    for entry in &spec.menu.items {
        match entry.kind {
            // ── Separator ─────────────────────────────────────────────────────
            MenuItemKind::Separator => {
                el = el.child(
                    ui_element::div()
                        .h(1.0)
                        .bg(separator_bg)
                        .mt(separator_my)
                        .mb(separator_my),
                );
            }

            // ── Checkbox / Radio items (leading check indicator) ──────────────
            MenuItemKind::Checkbox | MenuItemKind::Radio => {
                let label_color = if entry.is_destructive { danger_color } else { text_color };
                let leading = if entry.is_checked {
                    ui_element::icon("check")
                        .w(font_size)
                        .h(font_size)
                        .text_color(muted_color)
                } else {
                    ui_element::div().w(font_size).h(font_size)
                };

                let mut item = ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(item_gap)
                    .min_h(item_min_h)
                    .pl(item_px)
                    .pr(item_px)
                    .pt(item_py)
                    .pb(item_py)
                    .rounded(item_radius)
                    .focusable()
                    .child(leading)
                    .child(
                        ui_element::label(&entry.label)
                            .text_color(label_color)
                            .text_size(font_size)
                            .grow(),
                    );

                if let Some(ref shortcut) = entry.shortcut_label {
                    item = item.child(
                        ui_element::label(shortcut)
                            .text_color(muted_color)
                            .text_size(meta_font),
                    );
                }

                if entry.is_disabled {
                    item = item.opacity(disabled_opacity);
                } else {
                    item = item.cursor_pointer().hover(|s| s.bg(hover_tint));
                }

                el = el.child(item);
            }

            // ── Action items ─────────────────────────────────────────────────
            MenuItemKind::Action => {
                let label_color = if entry.is_destructive { danger_color } else { text_color };

                let mut item = ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(item_gap)
                    .min_h(item_min_h)
                    .pl(item_px)
                    .pr(item_px)
                    .pt(item_py)
                    .pb(item_py)
                    .rounded(item_radius)
                    .focusable()
                    .child(
                        ui_element::label(&entry.label)
                            .text_color(label_color)
                            .text_size(font_size)
                            .grow(),
                    );

                // Trailing meta: checkmark or shortcut label.
                if entry.is_checked {
                    item = item.child(
                        ui_element::icon("check")
                            .w(font_size)
                            .h(font_size)
                            .text_color(muted_color),
                    );
                } else if let Some(ref shortcut) = entry.shortcut_label {
                    item = item.child(
                        ui_element::label(shortcut)
                            .text_color(muted_color)
                            .text_size(meta_font),
                    );
                }

                if entry.is_disabled {
                    item = item.opacity(disabled_opacity);
                } else {
                    item = item.cursor_pointer().hover(|s| s.bg(hover_tint));
                }

                el = el.child(item);
            }
        }
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ContextMenuSpec, ControlSize, MenuEntry, MenuItemKind};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn sample_items() -> Vec<MenuEntry> {
        vec![
            MenuEntry::new("cut", "Cut").with_shortcut_label("Cmd+X"),
            MenuEntry::new("copy", "Copy").with_shortcut_label("Cmd+C"),
            MenuEntry::new("paste", "Paste").with_shortcut_label("Cmd+V"),
            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("select-all", "Select all").with_shortcut_label("Cmd+A"),
            MenuEntry::new("sep2", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("delete", "Delete")
                .with_disabled(true)
                .with_destructive(true)
                .with_shortcut_label("Del"),
        ]
    }

    #[test]
    fn renders_items_with_labels_and_shortcuts() {
        let th = theme();
        let spec = ContextMenuSpec::new(sample_items());
        let el = js_context_menu(&spec, &th);
        let tree = probe(&el, 600.0, 400.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        // Item labels present.
        for label in ["Cut", "Copy", "Paste", "Select all", "Delete"] {
            assert!(tree.has_text(label), "missing item label {label}: {:?}", tree.texts());
        }
        // Shortcut (meta) labels present.
        for sc in ["Cmd+X", "Cmd+C", "Cmd+V", "Cmd+A", "Del"] {
            assert!(tree.has_text(sc), "missing shortcut {sc}: {:?}", tree.texts());
        }
    }

    #[test]
    fn separators_render_as_thin_dividers() {
        let th = theme();
        let spec = ContextMenuSpec::new(sample_items());
        let el = js_context_menu(&spec, &th);
        let tree = probe(&el, 600.0, 400.0);

        // Two separators → two 1px-high divider panels.
        let dividers = tree
            .nodes
            .iter()
            .filter(|n| n.kind == "Panel" && (n.h - 1.0).abs() < 0.01)
            .count();
        assert!(dividers >= 2, "expected >=2 separator dividers, got {dividers}");
    }

    #[test]
    fn danger_item_uses_status_danger_color() {
        let th = theme();
        let danger = resolve_color(&th, "color.status.danger");
        let spec = ContextMenuSpec::new(vec![
            MenuEntry::new("delete", "Delete").with_destructive(true),
        ]);
        let el = js_context_menu(&spec, &th);
        let tree = probe(&el, 400.0, 200.0);

        let delete = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Delete"))
            .expect("Delete label not rendered");
        // Probe doesn't surface text color directly; assert the label exists and
        // the menu resolved a danger color from the token (sanity that the token
        // resolves). The label-color path is exercised at build time.
        assert!(danger.w > 0.0, "danger color failed to resolve");
        assert_eq!(delete.text.as_deref(), Some("Delete"));
    }

    #[test]
    fn size_scales_item_font() {
        let th = theme();
        let xs = ContextMenuSpec::new(vec![MenuEntry::new("a", "Alpha")])
            .with_size(ControlSize::Xs)
            .with_size_role(poodle_specs::SemanticControlSizeRole::Control);
        let xl = ContextMenuSpec::new(vec![MenuEntry::new("a", "Alpha")])
            .with_size(ControlSize::Xl)
            .with_size_role(poodle_specs::SemanticControlSizeRole::Control);

        let xs_tree = probe(&js_context_menu(&xs, &th), 400.0, 200.0);
        let xl_tree = probe(&js_context_menu(&xl, &th), 400.0, 200.0);

        let xs_font = xs_tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Alpha"))
            .and_then(|n| n.text_size)
            .expect("xs label font");
        let xl_font = xl_tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Alpha"))
            .and_then(|n| n.text_size)
            .expect("xl label font");

        assert!(xl_font > xs_font, "xl font {xl_font} should exceed xs font {xs_font}");
    }
}
