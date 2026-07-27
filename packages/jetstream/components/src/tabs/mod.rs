//! Tabs — Jetstream tabs backed by TabsSpec.
//!
//! Contract: `docs/contracts/components/tabs.md`
//! Reference: `packages/svelte/components/src/Tabs.svelte`
//!
//! Supports four variants:
//! - Underline: bottom border with accent indicator (default)
//! - Card: bordered tab boxes, active merges with content area
//! - Pill: rounded pill container with tinted active state
//! - Block: full-width tabs with vertical separators, accent-tinted selected fill

use jetstream_ui::ui_element::{self, BoxShadow, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{TabVariant, TabsSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius, tint};

// ── Shared helpers ──────────────────────────────────────────────────────────

/// Build the icon + label + count badge inner row for a tab button.
///
/// Matches the visual anatomy across all four variants — icon left of label,
/// count badge as a small pill to the right.
fn build_tab_label(
    tab: &poodle_specs::TabDefinition,
    theme: &JetstreamThemeProvider,
    text_color: glam::Vec4,
    font_size: f32,
    icon_only: bool,
) -> JsEl {
    let has_icon = tab.icon.is_some();
    let has_count = tab.count.is_some();

    // Vertical/icon-only mode: contract §8 hides `.poodle-tabs__label`. Show the
    // icon alone; if there is no icon fall back to the label so the tab is never
    // empty.
    if icon_only {
        if let Some(ref icon_name) = tab.icon {
            return ui_element::icon(icon_name.as_str())
                .w(font_size)
                .h(font_size)
                .text_color(text_color);
        }
        return ui_element::label(&tab.label)
            .text_size(font_size)
            .text_color(text_color);
    }

    // Fast path: plain label with no decoration.
    if !has_icon && !has_count {
        return ui_element::label(&tab.label)
            .text_size(font_size)
            .text_color(text_color);
    }

    // Contract §8 Tab button: gap = space.inline.sm between icon and label.
    let gap = resolve_px(theme, "space.inline.sm");
    let mut row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap);

    // Leading icon.
    if let Some(ref icon_name) = tab.icon {
        let icon_sz = font_size; // icon tracks font size
        row = row.child(
            ui_element::icon(icon_name.as_str())
                .w(icon_sz)
                .h(icon_sz)
                .text_color(text_color),
        );
    }

    row = row.child(
        ui_element::label(&tab.label)
            .text_size(font_size)
            .text_color(text_color),
    );

    // Trailing count badge.
    if let Some(count) = tab.count {
        // Small pill: accent-tint bg at 14% opacity, caption-size text from the
        // caption typography token. Badge geometry (radius 0.5625rem, min-width
        // 1.125rem, padding-x 0.3125rem) has no dedicated token — contract-exact
        // rems, noted as a token gap in the parity doc.
        let caption_size = resolve_px(theme, "typography.caption.size");
        let accent = resolve_color(theme, "color.accent.base");
        let badge_bg = tint(accent, 0.14);
        let badge = ui_element::label(format!("{count}"))
            .text_size(caption_size)
            .text_weight(600)
            .text_color(text_color)
            .bg(badge_bg)
            .rounded(rem_to_px(0.5625))
            .px(rem_to_px(0.3125))
            .min_w(rem_to_px(1.125));
        row = row.child(badge);
    }

    row
}

/// Build the optional close button for a closable tab.
///
/// Contract Close button: 1.25rem square, icon-only `x`, `text-secondary`
/// color, radius `calc(radius-control - 0.125rem)`, margin-right `0.25rem`.
/// Interaction (click / Delete) lives in the preview event loop.
fn build_close_button(theme: &JetstreamThemeProvider, font_size: f32, tab_label: &str) -> JsEl {
    let icon_color = resolve_color(theme, "color.text.secondary");
    let radius = (resolve_radius(theme, "radius.control") - rem_to_px(0.125)).max(0.0);
    let box_sz = rem_to_px(1.25);
    ui_element::button("")
        // A row of tabs yields a row of identical close glyphs, so the name
        // has to say which tab this one closes.
        .aria_label(format!("Close {tab_label}"))
        .w(box_sz)
        .h(box_sz)
        .mr(rem_to_px(0.25))
        .rounded(radius)
        .flex_row()
        .items_center()
        .justify_center()
        .focusable()
        .cursor_pointer()
        .child(
            ui_element::icon("x")
                .w(font_size)
                .h(font_size)
                .text_color(icon_color),
        )
}

/// Apply the transient reorder-drag visuals to a built tab element.
///
/// Contract §4 States:
/// - drag-source (the tab being dragged): `opacity: 0.4`.
/// - drop-target (the tab being dragged over): `box-shadow: inset 0 0 0
///   0.125rem accent-base` with `border-radius: radius-control`.
///
/// Both can apply at once when the same tab is somehow both (normally they are
/// different tabs), so the two states are layered independently.
fn apply_drag_state(
    mut tab_el: JsEl,
    tab_value: &str,
    spec: &TabsSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    // Hit-test id so the preview host can identify which tab is under the
    // cursor during a drag (matches the tree's `tree:<value>` convention).
    tab_el = tab_el.id(format!("tabs:{tab_value}"));
    if spec.is_drag_value(tab_value) {
        tab_el = tab_el.opacity(0.4);
    }
    if spec.is_drop_target(tab_value) {
        let accent = resolve_color(theme, "color.accent.base");
        tab_el = tab_el
            .rounded(resolve_radius(theme, "radius.control"))
            .shadow_layers(vec![BoxShadow {
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: rem_to_px(0.125),
                color: accent,
                inset: true,
            }]);
    }
    tab_el
}


mod variants;
use variants::{render_block, render_card, render_pill, render_underline};

// ── Color blending helper ───────────────────────────────────────────────────

/// Blend `over` into `base` by `fraction` of `over`.
/// Equivalent to `color-mix(over fraction, base)`.
fn blend(over: glam::Vec4, base: glam::Vec4, fraction: f32) -> glam::Vec4 {
    glam::Vec4::new(
        over.x * fraction + base.x * (1.0 - fraction),
        over.y * fraction + base.y * (1.0 - fraction),
        over.z * fraction + base.z * (1.0 - fraction),
        over.w * fraction + base.w * (1.0 - fraction),
    )
}

// ── Public entry point ──────────────────────────────────────────────────────

/// Build a Jetstream tabs element from a TabsSpec.
///
/// Anatomy:
/// ```text
/// [Root]  — flex-col wrapper
///   └── [TabBar]  — flex-row of tab buttons (variant-specific)
/// ```
/// Content for the active tab is rendered by the caller below this element.
pub fn js_tabs(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let tab_bar = match spec.variant {
        TabVariant::Underline => render_underline(spec, theme),
        TabVariant::Card => render_card(spec, theme),
        TabVariant::Pill => render_pill(spec, theme),
        TabVariant::Block => render_block(spec, theme),
    };

    // Wrap tab bar in a flex-col container. Content is rendered by the caller
    // below the tab bar — TabDefinition carries no content field in this API.
    let root = ui_element::div()
        .flex_col()
        .child(tab_bar);
    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
        .aria_role(jetstream_ui::accesskit::Role::TabList)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use crate::theme_ext::{resolve_color, tint};
    use poodle_specs::{TabDefinition, TabVariant, TabsSpec};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn three_tabs() -> Vec<TabDefinition> {
        vec![
            TabDefinition::new("a", "Overview"),
            TabDefinition::new("b", "Features"),
            TabDefinition::new("c", "Pricing"),
        ]
    }

    /// Active text/underline tab carries the accent-18% background tint
    /// (the variant treatment + active indicator).
    #[test]
    fn underline_active_tab_has_accent_tint() {
        let th = theme();
        let spec = TabsSpec::new(three_tabs())
            .with_variant(TabVariant::Underline)
            .with_value("a");
        let tree = probe(&js_tabs(&spec, &th), 600.0, 120.0);

        assert!(tree.has_text("Overview"));
        assert!(tree.has_text("Features"));

        let accent = resolve_color(&th, spec.indicator_token());
        let want = tint(accent, 0.18);
        let want = ProbeColor { r: want.x, g: want.y, b: want.z, a: want.w };
        assert!(
            tree.has_background(want, 0.02),
            "active underline tab missing accent-18% tint; bgs: {}",
            tree.to_json()
        );
    }

    /// Pill active tab carries accent-18% fill — confirms variant treatment differs
    /// from the underline path while sharing the active-indicator tint.
    #[test]
    fn pill_active_tab_has_accent_fill() {
        let th = theme();
        let spec = TabsSpec::new(three_tabs())
            .with_variant(TabVariant::Pill)
            .with_value("b");
        let tree = probe(&js_tabs(&spec, &th), 600.0, 120.0);

        let accent = resolve_color(&th, spec.indicator_token());
        let want = tint(accent, spec.pill_active_bg_opacity());
        let want = ProbeColor { r: want.x, g: want.y, b: want.z, a: want.w };
        assert!(
            tree.has_background(want, 0.02),
            "active pill tab missing accent fill; tree: {}",
            tree.to_json()
        );
    }

    /// Block active tab carries the accent-into-surface selected fill.
    #[test]
    fn block_active_tab_has_selected_fill() {
        let th = theme();
        let spec = TabsSpec::new(three_tabs())
            .with_variant(TabVariant::Block)
            .with_value("c");
        let tree = probe(&js_tabs(&spec, &th), 600.0, 120.0);

        let accent = resolve_color(&th, spec.indicator_token());
        let surface = resolve_color(&th, "color.background.surface");
        let want = blend(accent, surface, spec.block_selected_accent_mix());
        let want = ProbeColor { r: want.x, g: want.y, b: want.z, a: want.w };
        assert!(
            tree.has_background(want, 0.02),
            "active block tab missing selected fill; tree: {}",
            tree.to_json()
        );
    }

    /// Closable card tab renders the `x` close-button icon.
    #[test]
    fn card_closable_tab_renders_close_icon() {
        let th = theme();
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "index.ts"),
            TabDefinition::new("b", "App.svelte").with_closable(true),
        ])
        .with_variant(TabVariant::Card)
        .with_value("a");
        let tree = probe(&js_tabs(&spec, &th), 600.0, 120.0);

        assert!(tree.has_text("App.svelte"));
        assert!(
            tree.has_text("x"),
            "closable card tab missing close icon; icons: {:?}",
            tree.texts()
        );
    }

    /// Count badge renders the numeric label next to the tab text.
    #[test]
    fn tab_count_badge_renders() {
        let th = theme();
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "Inbox").with_count(12),
            TabDefinition::new("b", "Sent"),
        ])
        .with_variant(TabVariant::Card)
        .with_value("a");
        let tree = probe(&js_tabs(&spec, &th), 600.0, 120.0);

        assert!(tree.has_text("Inbox"));
        assert!(
            tree.has_text("12"),
            "count badge text missing; texts: {:?}",
            tree.texts()
        );
    }

    /// Disabled tab still renders its label (and is dimmed via opacity, which
    /// the probe does not surface — structure is the assertable signal).
    #[test]
    fn disabled_tab_still_renders() {
        let th = theme();
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "Overview"),
            TabDefinition::new("b", "FAQ").with_disabled(true),
        ])
        .with_variant(TabVariant::Underline)
        .with_value("a");
        let tree = probe(&js_tabs(&spec, &th), 600.0, 120.0);
        assert!(tree.has_text("FAQ"));
    }

    /// Contract §8 Full-width: tabs flex to equal widths spanning the row.
    /// Each underline tab button should be roughly container-width / n wide,
    /// markedly wider than the content-sized default.
    #[test]
    fn full_width_tabs_share_row_equally() {
        let th = theme();
        let width = 600.0;
        let spec = TabsSpec::new(three_tabs())
            .with_variant(TabVariant::Underline)
            .with_value("a")
            .with_full_width(true);
        let tree = probe(&js_tabs(&spec, &th), width, 120.0);

        // Collect the three tab button widths.
        // Tabs are focusable Panels (bare divs) directly under the bar.
        let tab_widths: Vec<f32> = tree
            .nodes
            .iter()
            .filter(|n| n.depth == 2 && n.kind == "Panel")
            .map(|n| n.w)
            .collect();
        assert_eq!(tab_widths.len(), 3, "expected 3 tab buttons");
        // Equal width: each ~ width/3; allow generous tolerance for borders.
        let expected = width / 3.0;
        for w in &tab_widths {
            assert!(
                (*w - expected).abs() < expected * 0.25,
                "full-width tab {w} not near equal share {expected}; tree: {}",
                tree.to_json()
            );
        }
    }

    /// Contract §8 vertical: label is hidden, the icon stands alone. A vertical
    /// underline tablist renders icons but NOT the label text.
    #[test]
    fn vertical_underline_is_icon_only() {
        let th = theme();
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "Explorer").with_icon("folder"),
            TabDefinition::new("b", "Search").with_icon("search"),
        ])
        .with_variant(TabVariant::Underline)
        .with_orientation(poodle_specs::Orientation::Vertical)
        .with_value("a");
        let tree = probe(&js_tabs(&spec, &th), 80.0, 240.0);

        // Icon names surface as Icon nodes.
        let icon_names: Vec<&str> = tree
            .nodes
            .iter()
            .filter(|n| n.kind == "Icon")
            .filter_map(|n| n.text.as_deref())
            .collect();
        assert!(icon_names.contains(&"folder"), "icons: {icon_names:?}");
        // Labels are hidden in icon-only vertical mode.
        assert!(
            !tree.has_text("Explorer"),
            "vertical tab should hide label; texts: {:?}",
            tree.texts()
        );
    }

    /// Contract §4 drag states: setting drag-source + drop-target still renders
    /// all tab labels (opacity/inset-ring are applied but not surfaced by the
    /// probe; structure is the assertable signal that the states are wired).
    #[test]
    fn drag_states_still_render_all_tabs() {
        let th = theme();
        let spec = TabsSpec::new(three_tabs())
            .with_variant(TabVariant::Card)
            .with_value("a")
            .with_reorderable(true)
            .with_drag_value(Some("b".into()))
            .with_drop_target_value(Some("c".into()));
        let tree = probe(&js_tabs(&spec, &th), 600.0, 120.0);

        assert!(tree.has_text("Overview"));
        assert!(tree.has_text("Features"));
        assert!(tree.has_text("Pricing"));
    }

    /// Contract §8 vertical: a vertical block tablist stacks tabs in a column,
    /// so the bar is taller than it is wide for a narrow viewport.
    #[test]
    fn vertical_block_stacks_into_column() {
        let th = theme();
        let spec = TabsSpec::new(three_tabs())
            .with_variant(TabVariant::Block)
            .with_orientation(poodle_specs::Orientation::Vertical)
            .with_value("a");
        let tree = probe(&js_tabs(&spec, &th), 120.0, 300.0);

        // Buttons should be stacked: distinct y offsets, increasing down the column.
        // Tabs are focusable Panels (bare divs) directly under the bar.
        let ys: Vec<f32> = tree
            .nodes
            .iter()
            .filter(|n| n.depth == 2 && n.kind == "Panel")
            .map(|n| n.y)
            .collect();
        assert_eq!(ys.len(), 3, "expected 3 stacked tab buttons");
        assert!(
            ys[1] > ys[0] && ys[2] > ys[1],
            "vertical block tabs should stack with increasing y: {ys:?}"
        );
    }
}

