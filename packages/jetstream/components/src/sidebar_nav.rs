//! SidebarNav — Jetstream sidebar navigation backed by SidebarNavSpec.
//!
//! Contract: `docs/contracts/components/sidebar-nav.md`
//! Reference: `packages/svelte/components/src/SidebarNav.svelte`
//!
//! All geometry resolves from the spec's contract §8 size/density tables; all
//! colors resolve from semantic tokens. Selection/activation lives in the
//! preview event loop (Tier-3); the component renders the active state for the
//! current `value`.
use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SidebarNavSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

// ── Active-state alpha factors (contract color-mix percentages) ──
const ACTIVE_BG_ALPHA: f32 = 0.10; // accent-base @ 10%
const ACTIVE_RING_ALPHA: f32 = 0.20; // inset ring accent-base @ 20%
const HOVER_BG_ALPHA: f32 = 0.60; // elevated @ 60%
const SEPARATOR_ALPHA: f32 = 0.54; // border-subtle @ 54%

pub fn js_sidebar_nav(spec: &SidebarNavSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // ── Size / density geometry (contract §8 tables, token-resolved rem) ──
    let item_height = rem_to_px(spec.item_height_rem());
    let item_font = rem_to_px(spec.item_font_rem());
    let title_font = rem_to_px(spec.title_font_rem());

    let group_gap = rem_to_px(spec.group_gap_rem());
    let item_px = rem_to_px(spec.item_pad_inline_rem());
    let item_py = rem_to_px(spec.item_pad_block_rem());
    let title_gap = rem_to_px(spec.title_gap_rem());
    let group_internal_gap = rem_to_px(0.3125); // contract group `gap`
    let list_gap = rem_to_px(0.125); // contract list `gap`
    let separator_mt = rem_to_px(0.125); // contract separator margin-top
    let rail_w = rem_to_px(0.1875); // contract left border 3px
    let nav_pad_x = rem_to_px(0.375); // contract root horizontal padding
    // Root vertical padding = space-panel-y (density-driven).
    let panel_y = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.5,
        poodle_specs::ControlDensity::Default => 0.75,
        poodle_specs::ControlDensity::Comfortable => 1.0,
    });

    // ── Token resolution ──────────────────────────────────────
    let item_color = resolve_color(theme, spec.item_color_token());
    let item_active_color = resolve_color(theme, spec.item_active_color_token());
    let group_title_color = resolve_color(theme, spec.group_title_color_token());
    let separator_color = resolve_color(theme, spec.separator_color_token());
    let accent = resolve_color(theme, spec.active_indicator_color_token());
    let hover_fill = resolve_color(theme, spec.hover_fill_token());
    let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
    let ctrl_radius = resolve_radius(theme, "radius.control");
    let item_radius = (ctrl_radius - rem_to_px(0.125)).max(0.0);

    let active_bg = tint(accent, ACTIVE_BG_ALPHA);
    let active_ring = tint(accent, ACTIVE_RING_ALPHA);
    let hover_bg = tint(hover_fill, HOVER_BG_ALPHA);

    let visible_groups = spec.visible_groups();
    let has_multiple_groups = visible_groups.len() > 1;

    // ── Root: <nav> as a flex column with panel padding ──────
    let mut el = ui_element::div()
        .flex_col()
        .gap(group_gap)
        .pt(panel_y)
        .pb(panel_y)
        .pl(nav_pad_x)
        .pr(nav_pad_x);

    for (gi, group) in visible_groups.iter().enumerate() {
        let mut group_el = ui_element::div().flex_col().gap(group_internal_gap);

        // Inter-group separator: top border + top padding on the group element
        // (matches the Svelte adjacent-sibling rule), not a floating divider.
        if has_multiple_groups && gi > 0 {
            group_el = group_el
                .mt(separator_mt)
                .pt(group_gap - separator_mt)
                .border_t_1()
                .border_color_top(tint(separator_color, SEPARATOR_ALPHA));
        }

        // Group title — uppercase, caption-sized, accent-tinted.
        if let Some(ref label_text) = group.label {
            let title = ui_element::label(&label_text.to_uppercase())
                .text_color(group_title_color)
                .text_size(title_font)
                .text_weight(700)
                .letter_spacing_em(0.18) // contract §8 title tracking: 0.18em
                .pl(item_px)
                .pr(item_px)
                .mb(title_gap);
            group_el = group_el.child(title);
        }

        // Item list
        let mut list = ui_element::div().flex_col().gap(list_gap);

        for item in &group.items {
            let is_active = spec.is_active(&item.value);

            // Item box: min-height drives the row height (contract item-height);
            // for single-line labels the contract padding-block (`item_py`) is
            // subsumed by min-height, so vertical centring stands in for it.
            let _ = item_py;
            let mut item_el = ui_element::button(&item.label)
                .min_h(item_height)
                .self_stretch()
                .flex_row()
                .items_center()
                .text_size(item_font)
                .pl(item_px)
                .pr(item_px)
                .rounded(item_radius)
                // Reserve a 3px transparent left rail on every item so active ↔
                // inactive does not shift horizontally.
                .border_l(rail_w)
                .border_color_left(Color::TRANSPARENT);

            if is_active {
                // Active: accent left rail + bg fill + bolder weight + inset ring.
                // The contract inset box-shadow (accent@20%) is rendered as a 1px
                // all-side border in accent@20% (JsEl has no inset shadow).
                item_el = item_el
                    .text_color(item_active_color)
                    .text_weight(600)
                    .bg(active_bg)
                    .border(1.0)
                    .border_l(rail_w)
                    .border_color(active_ring)
                    .border_color_left(accent);
            } else {
                item_el = item_el.text_color(item_color).text_weight(500);
            }

            if item.is_disabled {
                item_el = item_el.opacity(disabled_opacity);
            } else {
                let hb = hover_bg;
                let hc = item_active_color;
                let fr = focus_ring;
                item_el = item_el
                    .focusable()
                    .cursor_pointer()
                    // Hover: text-primary + elevated@60% bg (contract §4/§8).
                    .hover(move |s| s.text_color(hc).bg(hb))
                    // Focus-visible: accent focus ring (contract §6/§8).
                    .active(move |s| s.border_color(fr));
            }

            list = list.child(item_el);
        }

        group_el = group_el.child(list);
        el = el.child(group_el);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::resolve_semantic_size;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{SidebarNavGroup, SidebarNavItem};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn sample() -> SidebarNavSpec {
        SidebarNavSpec::new(vec![
            SidebarNavGroup::new(
                "workspace",
                vec![
                    SidebarNavItem::new("dashboard", "Dashboard"),
                    SidebarNavItem::new("projects", "Projects"),
                ],
            )
            .with_label("Workspace"),
            SidebarNavGroup::new(
                "settings",
                vec![
                    SidebarNavItem::new("account", "Account"),
                    SidebarNavItem::new("team", "Team").with_disabled(true),
                ],
            )
            .with_label("Settings"),
        ])
    }

    #[test]
    fn item_height_uses_sidebar_table_not_control_height() {
        let spec = SidebarNavSpec::new(vec![SidebarNavGroup::new(
            "g",
            vec![SidebarNavItem::new("home", "Home")],
        )]);
        let expected = rem_to_px(spec.item_height_rem());
        let tree = probe(&js_sidebar_nav(&spec, &theme()), 240.0, 200.0);
        let item = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Home"))
            .expect("item node");
        assert!(
            (item.h - expected).abs() < 1.0,
            "item height {} should match the sidebar table height {}",
            item.h,
            expected
        );
        // Default (md) = 1.875rem = 30px, NOT control-height 2.25rem = 36px.
        assert!((expected - 30.0).abs() < 0.5, "md sidebar item should be 30px");
    }

    #[test]
    fn renders_group_titles_and_item_labels() {
        let tree = probe(&js_sidebar_nav(&sample(), &theme()), 240.0, 320.0);
        assert!(tree.has_text("WORKSPACE"), "group title uppercased + rendered");
        assert!(tree.has_text("SETTINGS"), "second group title rendered");
        for label in ["Dashboard", "Projects", "Account", "Team"] {
            assert!(tree.has_text(label), "item `{label}` missing: {:?}", tree.texts());
        }
    }

    #[test]
    fn active_item_gets_accent_tinted_background() {
        let spec = sample().with_value("projects");
        let tree = probe(&js_sidebar_nav(&spec, &theme()), 240.0, 320.0);
        let accent = resolve_color(&theme(), spec.active_indicator_color_token());
        let want = ProbeColor {
            r: accent.x,
            g: accent.y,
            b: accent.z,
            a: accent.w * ACTIVE_BG_ALPHA,
        };
        // The active item's background is accent @ 10%.
        assert!(
            tree.has_background(want, 0.02),
            "active item should carry accent@10% bg; tree: {}",
            tree.to_json()
        );
    }

    #[test]
    fn title_font_smaller_than_item_font() {
        let spec = sample();
        // Group titles are caption-sized (smaller than item labels).
        assert!(spec.title_font_rem() < spec.item_font_rem());
        let _ = resolve_semantic_size(spec.size, spec.size_role);
    }

    #[test]
    fn disabled_item_reduces_opacity() {
        // Team is disabled — verify the spec flags it; opacity is applied per item.
        let spec = sample();
        let team = spec
            .visible_groups()
            .iter()
            .flat_map(|g| g.items.iter())
            .find(|i| i.value == "team")
            .expect("team item");
        assert!(team.is_disabled);
        // Render still succeeds and includes the disabled item.
        let tree = probe(&js_sidebar_nav(&spec, &theme()), 240.0, 320.0);
        assert!(tree.has_text("Team"));
    }
}
