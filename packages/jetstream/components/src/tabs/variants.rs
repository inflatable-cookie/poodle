//! Tabs — per-variant JsEl builders. Split out of `tabs/mod.rs`
//! (god-file decomposition); unchanged.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::TabsSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

use super::*;

// ── Underline variant ───────────────────────────────────────────────────────

pub(super) fn render_underline(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
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
    let vertical = spec.is_vertical();
    let full_width = spec.uses_full_width();

    // Contract §8: horizontal list carries a bottom border baseline; vertical
    // list (Text vertical table) shifts the rule to the inline-end (right) edge.
    let mut tab_bar = if vertical {
        let mut bar = ui_element::div().flex_col();
        if spec.is_bordered {
            bar = bar.border_r_1().border_color(border).pr(rem_to_px(0.5));
        }
        bar
    } else {
        let mut bar = ui_element::div().flex_row();
        if spec.is_bordered {
            bar = bar.border_b_1().border_color(border);
        }
        // Contract §8 Full-width: list becomes flex + width:100%.
        if full_width {
            bar = bar.w_full();
        }
        bar
    };

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
            .cursor_pointer();

        // Contract §8 Full-width: each tab flexes to equal width with centered
        // label. Contract §8 vertical: icon-only (label hidden).
        if full_width {
            tab_el = tab_el.flex_grow().w_full().justify_center();
        }
        tab_el = tab_el.child(build_tab_label(tab, theme, text_color, font_size, vertical));

        // Contract Tab — Text variant (selected): pill-shaped bg tint only.
        // No accent bottom border on the tab itself (the list carries the baseline).
        if is_active {
            let active_bg = tint(accent, 0.18);
            tab_el = tab_el.bg(active_bg);
        }

        if is_disabled {
            tab_el = tab_el.opacity(disabled_opacity);
        }

        tab_el = apply_drag_state(tab_el, tab.value.as_str(), spec, theme);

        tab_bar = tab_bar.child(tab_el);
    }

    tab_bar
}

// ── Card variant ────────────────────────────────────────────────────────────

pub(super) fn render_card(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
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
    let vertical = spec.is_vertical();
    let full_width = spec.uses_full_width();

    // Card list: row of bordered cards (horizontal) or stacked column (vertical).
    // items_end aligns shorter tabs to the bottom of the row.
    let mut tab_bar = if vertical {
        ui_element::div().flex_col().gap(rem_to_px(0.125))
    } else {
        let mut bar = ui_element::div()
            .flex_row()
            .items_end()
            .gap(rem_to_px(0.125));
        if full_width {
            bar = bar.w_full();
        }
        bar
    };

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
            .cursor_pointer();

        // Contract §8 Full-width: equal-width cards with centered content.
        if full_width {
            tab_el = tab_el.flex_grow().w_full().justify_center();
        }
        tab_el = tab_el.child(build_tab_label(tab, theme, text_color, font_size, vertical));

        // Contract Close button: rendered when the tab is closable.
        if tab.is_closable {
            tab_el = tab_el.child(build_close_button(theme, font_size));
        }

        if is_disabled {
            tab_el = tab_el.opacity(disabled_opacity);
        }

        tab_el = apply_drag_state(tab_el, tab.value.as_str(), spec, theme);

        tab_bar = tab_bar.child(tab_el);
    }

    tab_bar
}

// ── Pill variant ────────────────────────────────────────────────────────────

pub(super) fn render_pill(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
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
            // Pill is always horizontal (vertical pill not in contract §8).
            .child(build_tab_label(tab, theme, text_color, font_size, false));

        if is_active {
            tab_el = tab_el.bg(active_bg);
        }

        if is_disabled {
            tab_el = tab_el.opacity(disabled_opacity);
        }

        tab_el = apply_drag_state(tab_el, tab.value.as_str(), spec, theme);

        container = container.child(tab_el);
    }

    container
}

// ── Block variant ───────────────────────────────────────────────────────────

pub(super) fn render_block(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
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
    let vertical = spec.is_vertical();

    // Contract §8: block list is a full-width flex row (horizontal) or a
    // stacked column (vertical), tinted panel bg + a closing border on the
    // block axis edge.
    let mut tab_bar = if vertical {
        ui_element::div()
            .flex_col()
            .bg(list_bg)
            .border_r_1()
            .border_color(border)
    } else {
        ui_element::div()
            .flex_row()
            .w_full()
            .bg(list_bg)
            .border_b_1()
            .border_color(border)
    };

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
            .child(build_tab_label(tab, theme, text_color, font_size, vertical));

        // Contract §8: separator between sibling items. Horizontal = left
        // border; vertical = top border (block vertical item table).
        if idx > 0 {
            tab_el = if vertical {
                tab_el.border_t_1().border_color(separator)
            } else {
                tab_el.border_l_1().border_color(separator)
            };
        }

        if is_active {
            tab_el = tab_el.bg(selected_bg);
        }

        if is_disabled {
            tab_el = tab_el.opacity(disabled_opacity);
        }

        tab_el = apply_drag_state(tab_el, tab.value.as_str(), spec, theme);

        tab_bar = tab_bar.child(tab_el);
    }

    tab_bar
}

