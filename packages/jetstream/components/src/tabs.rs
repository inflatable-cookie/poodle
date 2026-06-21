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

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{TabVariant, TabsSpec};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

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
) -> JsEl {
    let has_icon = tab.icon.is_some();
    let has_count = tab.count.is_some();

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
        // Small pill: accent-tint bg at 14% opacity, caption-size text.
        let caption_size = rem_to_px(0.6875); // ~11px caption
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
fn build_close_button(theme: &JetstreamThemeProvider, font_size: f32) -> JsEl {
    let icon_color = resolve_color(theme, "color.text.secondary");
    let radius = (resolve_radius(theme, "radius.control") - rem_to_px(0.125)).max(0.0);
    let box_sz = rem_to_px(1.25);
    ui_element::button("")
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

// ── Underline variant ───────────────────────────────────────────────────────

fn render_underline(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // Contract §8 Tab button: padding `0 control-x` (zero vertical) +
    // min-height `calc(control-height - 0.25rem)`.
    let min_h = rem_to_px(control_height_rem(effective_size) - 0.25);

    let accent = resolve_color(theme, spec.indicator_token());
    let border = resolve_color(theme, spec.list_border_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
    let radius = resolve_radius(theme, "radius.control");

    let selected = spec.current_value().map(|s| s.to_string());

    let mut tab_bar = ui_element::div().flex_row();
    if spec.is_bordered {
        tab_bar = tab_bar.border_b_1().border_color(border);
    }

    for tab in &spec.tabs {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;

        let text_color = if is_active { text_primary } else { text_secondary };

        let mut tab_el = ui_element::button("")
            .flex_row()
            .items_center()
            .min_h(min_h)
            .pl(pad_x)
            .pr(pad_x)
            .text_size(font_size)
            // Contract §8: ALL tabs font-weight 600 — weight does not change on selection.
            .text_weight(600)
            .text_color(text_color)
            .rounded(radius)
            .focusable()
            .cursor_pointer()
            .child(build_tab_label(tab, theme, text_color, font_size));

        // Contract Tab — Text variant (selected): pill-shaped bg tint only.
        // No accent bottom border on the tab itself (the list carries the baseline).
        if is_active {
            let active_bg = tint(accent, 0.18);
            tab_el = tab_el.bg(active_bg);
        }

        if is_disabled {
            tab_el = tab_el.opacity(disabled_opacity);
        }

        tab_bar = tab_bar.child(tab_el);
    }

    tab_bar
}

// ── Card variant ────────────────────────────────────────────────────────────

fn render_card(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    // Svelte card tab: padding `0 var(--poodle-tabs-control-x)` (density-based);
    // min-height from base tab button `calc(control-height - 0.25rem)`.
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let min_h = rem_to_px(control_height_rem(effective_size) - 0.25);

    let accent = resolve_color(theme, spec.indicator_token());
    let border = resolve_color(theme, spec.list_border_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let surface_bg = resolve_color(theme, "color.background.surface");
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
    let radius = resolve_radius(theme, "radius.control");

    // Dimmed versions for unselected card border/bg.
    let card_default_bg = tint(surface_bg, 0.92);
    let card_default_border = tint(border, 0.68);
    // Selected: accent 14% mixed into surface; border accent 32% + border-subtle.
    let card_selected_bg = blend(accent, surface_bg, 0.14);
    let card_selected_border = blend(accent, border, 0.32);

    let selected = spec.current_value().map(|s| s.to_string());

    // items_end aligns shorter tabs to the bottom of the row.
    let mut tab_bar = ui_element::div()
        .flex_row()
        .items_end()
        .gap(rem_to_px(0.125));

    for tab in &spec.tabs {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;

        let text_color = if is_active { text_primary } else { text_secondary };
        let (bg, bc) = if is_active {
            (card_selected_bg, card_selected_border)
        } else {
            (card_default_bg, card_default_border)
        };

        // Contract Item — Card variant: uniform border + radius-control on all sides;
        // selected recolors border/bg only (no bottom-edge removal). Svelte matches.
        let mut tab_el = ui_element::button("")
            .flex_row()
            .items_center()
            .gap(resolve_px(theme, "space.inline.sm"))
            .min_h(min_h)
            .pl(pad_x)
            .pr(pad_x)
            .text_size(font_size)
            // Contract §8: ALL tabs font-weight 600.
            .text_weight(600)
            .text_color(text_color)
            .bg(bg)
            .border_1()
            .border_color(bc)
            .rounded(radius)
            .focusable()
            .cursor_pointer()
            .child(build_tab_label(tab, theme, text_color, font_size));

        // Contract Close button: rendered when the tab is closable.
        if tab.is_closable {
            tab_el = tab_el.child(build_close_button(theme, font_size));
        }

        if is_disabled {
            tab_el = tab_el.opacity(disabled_opacity);
        }

        tab_bar = tab_bar.child(tab_el);
    }

    tab_bar
}

// ── Pill variant ────────────────────────────────────────────────────────────

fn render_pill(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let control_height = rem_to_px(control_height_rem(effective_size));
    let tab_height = control_height - rem_to_px(0.5); // pill inner tabs slightly shorter
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let accent = resolve_color(theme, spec.indicator_token());
    let border_subtle = resolve_color(theme, spec.list_border_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
    let pill_radius = resolve_radius(theme, "radius.pill");

    // Container border: border-subtle at 68% opacity.
    let container_border = tint(border_subtle, spec.pill_border_opacity());
    // Active tab bg: accent at 18% opacity.
    let active_bg = tint(accent, spec.pill_active_bg_opacity());

    let selected = spec.current_value().map(|s| s.to_string());

    // Outer pill container.
    let mut container = ui_element::div()
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.125))
        .rounded(pill_radius)
        .border_2()
        .border_color(container_border)
        .p(rem_to_px(0.1875));

    for tab in &spec.tabs {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;

        let text_color = if is_active { text_primary } else { text_secondary };

        let mut tab_el = ui_element::button("")
            .flex_row()
            .items_center()
            .px(pad_x)
            .h(tab_height)
            .text_size(font_size)
            .text_weight(600)
            .text_color(text_color)
            .rounded(pill_radius)
            .focusable()
            .cursor_pointer()
            .child(build_tab_label(tab, theme, text_color, font_size));

        if is_active {
            tab_el = tab_el.bg(active_bg);
        }

        if is_disabled {
            tab_el = tab_el.opacity(disabled_opacity);
        }

        container = container.child(tab_el);
    }

    container
}

// ── Block variant ───────────────────────────────────────────────────────────

fn render_block(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let control_height = rem_to_px(control_height_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let accent = resolve_color(theme, spec.indicator_token());
    let border = resolve_color(theme, spec.list_border_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let surface_bg = resolve_color(theme, "color.background.surface");
    let panel_bg = resolve_color(theme, "color.background.panel");
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    // Tab list background: panel at 90% opacity.
    let list_bg = tint(panel_bg, spec.block_list_bg_opacity());
    // Separator between sibling tabs: border-subtle at 72%.
    let separator = tint(border, spec.block_separator_opacity());
    // Selected tab fill: accent mixed into surface at 12%.
    let selected_bg = blend(accent, surface_bg, spec.block_selected_accent_mix());

    let selected = spec.current_value().map(|s| s.to_string());

    let mut tab_bar = ui_element::div()
        .flex_row()
        .w_full()
        .bg(list_bg)
        .border_b_1()
        .border_color(border);

    for (idx, tab) in spec.tabs.iter().enumerate() {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;

        let text_color = if is_active { text_primary } else { text_secondary };

        // Block tabs fill equal space via flex_grow.
        let mut tab_el = ui_element::button("")
            .flex_grow()
            .flex_row()
            .items_center()
            .justify_center()
            .px(pad_x)
            .h(control_height)
            .text_size(font_size)
            // Contract §8: ALL tabs font-weight 600 — weight does not change on selection.
            .text_weight(600)
            .text_color(text_color)
            .focusable()
            .cursor_pointer()
            .child(build_tab_label(tab, theme, text_color, font_size));

        // Vertical separator: left border on all tabs except the first.
        if idx > 0 {
            tab_el = tab_el.border_l_1().border_color(separator);
        }

        if is_active {
            tab_el = tab_el.bg(selected_bg);
        }

        if is_disabled {
            tab_el = tab_el.opacity(disabled_opacity);
        }

        tab_bar = tab_bar.child(tab_el);
    }

    tab_bar
}

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
    ui_element::div()
        .flex_col()
        .child(tab_bar)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use crate::theme_ext::{resolve_color, tint};
    use poodle_specs::{TabDefinition, TabVariant, TabsSpec};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
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
}
