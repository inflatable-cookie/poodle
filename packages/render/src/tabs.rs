//! Tabs — a tab bar in four variants: underline, card, pill, block.
//!
//! Contract: `docs/contracts/components/tabs.md`
//! Ported from: `packages/jetstream/components/src/tabs/`. Content for the
//! active tab renders below this element, by the caller. Only the card
//! variant renders close buttons, matching the old tier.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeRole, ShadowLayer,
};
use poodle_specs::{TabVariant, TabsSpec};

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{
    control_height_rem, control_space_x_rem, panel_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};

pub type TabHandler = Arc<dyn Fn(&str) + Send + Sync>;

fn rounded_all(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

/// Icon + label + count badge, the anatomy shared by all variants.
fn build_tab_label(
    tab: &poodle_specs::TabDefinition,
    theme: &dyn ThemeProvider,
    text_color: ColorValue,
    font_size: f32,
    icon_only: bool,
) -> Node {
    let has_icon = tab.icon.is_some();
    let has_count = tab.count.is_some();

    // Vertical/icon-only: icon alone, label fallback so the tab is never empty.
    if icon_only {
        if let Some(ref icon_name) = tab.icon {
            let mut i = Node::icon(icon_name.as_str(), theme.resolve_space("size.icon.sm"));
            i.style.descriptor.text_color = Some(text_color);
            return i;
        }
        let mut l = Node::text(&tab.label);
        l.style.text_size = Some(font_size);
        l.style.descriptor.text_color = Some(text_color);
        return l;
    }

    if !has_icon && !has_count {
        let mut l = Node::text(&tab.label);
        l.style.text_size = Some(font_size);
        l.style.descriptor.text_color = Some(text_color);
        return l;
    }

    let gap = theme.resolve_space("space.inline.sm");
    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
    }

    if let Some(ref icon_name) = tab.icon {
        let mut i = Node::icon(icon_name.as_str(), theme.resolve_space("size.icon.sm"));
        i.style.descriptor.text_color = Some(text_color);
        row = row.child(i);
    }

    let mut l = Node::text(&tab.label);
    l.style.text_size = Some(font_size);
    l.style.descriptor.text_color = Some(text_color);
    row = row.child(l);

    if let Some(count) = tab.count {
        let caption_size = theme.resolve_space("typography.caption.size");
        let surface = theme.resolve_color("color.background.surface");
        let badge_bg = mix_srgb(text_color, surface, 0.14);
        let mut badge = Node::text(format!("{count}"));
        {
            let s = &mut badge.style;
            s.text_size = Some(caption_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.background = Some(badge_bg);
            s.descriptor.layout.spacing.padding.left = rem_to_px(0.3125);
            s.descriptor.layout.spacing.padding.right = rem_to_px(0.3125);
            s.min_width = Some(rem_to_px(1.125));
        }
        rounded_all(&mut badge, rem_to_px(0.5625));
        row = row.child(badge);
    }

    row
}

/// The card variant's close button; interaction wires through `on_close`.
fn build_close_button(theme: &dyn ThemeProvider, tab_label: &str) -> Node {
    let icon_color = theme.resolve_color("color.icon.muted");
    let icon_size = theme.resolve_space("size.icon.sm");
    let mut btn = Node::button("");
    btn.a11y.label = Some(format!("Close {tab_label}"));
    {
        let s = &mut btn.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.cursor = CursorHint::Pointer;
    }
    btn.interaction.focusable = true;
    let mut x = Node::icon("x", icon_size);
    x.style.descriptor.text_color = Some(icon_color);
    btn.child(x)
}

/// Transient reorder-drag visuals: source dims, target rings.
fn apply_drag_state(node: &mut Node, tab_value: &str, spec: &TabsSpec, theme: &dyn ThemeProvider) {
    node.id = Some(format!("tabs:{tab_value}"));
    if spec.is_drag_value(tab_value) {
        node.style.descriptor.opacity = 0.4;
    }
    if spec.is_drop_target(tab_value) {
        let accent = theme.resolve_color("color.accent.base");
        rounded_all(node, theme.resolve_radius("radius.control"));
        node.style.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: rem_to_px(0.125),
            color: accent,
            inset: true,
        }];
    }
}

pub fn tabs(
    spec: &TabsSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<TabHandler>,
    on_close: Option<TabHandler>,
) -> Node {
    let tab_bar = match spec.variant {
        TabVariant::Underline => render_underline(spec, theme, on_change.as_ref()),
        TabVariant::Card => render_card(spec, theme, on_change.as_ref(), on_close.as_ref()),
        TabVariant::Pill => render_pill(spec, theme, on_change.as_ref()),
        TabVariant::Block => render_block(spec, theme, on_change.as_ref()),
    };

    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    let mut root = root.child(tab_bar);
    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::TabList);
    root
}

fn wire_select(node: &mut Node, is_disabled: bool, value: &str, on_change: Option<&TabHandler>) {
    if let (false, Some(handler)) = (is_disabled, on_change) {
        let handler = Arc::clone(handler);
        let value = value.to_string();
        node.interaction.on_activate = Some(Arc::new(move || handler(&value)));
    }
}

fn render_underline(
    spec: &TabsSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<&TabHandler>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let control_y = theme.resolve_space("space.control.y");

    let accent = theme.resolve_color(spec.indicator_token());
    let border = theme.resolve_color(spec.list_border_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    let radius = theme.resolve_radius("radius.control");

    let selected = spec.current_value().map(|s| s.to_string());
    let vertical = spec.is_vertical();
    let full_width = spec.uses_full_width();

    let mut tab_bar = Node::container();
    {
        let s = &mut tab_bar.style;
        if vertical {
            s.descriptor.layout.direction = LayoutDirection::Column;
            if spec.is_bordered {
                s.border_right_width = Some(1.0);
                s.descriptor.border.color = border;
                s.descriptor.layout.spacing.padding.right = rem_to_px(0.5);
            }
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            if spec.is_bordered {
                s.border_bottom_width = Some(1.0);
                s.descriptor.border.color = border;
            }
            if full_width {
                s.fill_width = true;
            }
        }
    }

    for tab in &spec.tabs {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;
        let text_color = if is_active {
            text_primary
        } else {
            text_secondary
        };

        let mut tab_el = Node::container();
        tab_el.a11y.role = Some(NodeRole::Tab);
        tab_el.a11y.label = Some(tab.label.clone());
        tab_el.a11y.selected = Some(is_active);
        {
            let s = &mut tab_el.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.spacing.padding.top = control_y;
            s.descriptor.layout.spacing.padding.bottom = control_y;
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.cursor = CursorHint::Pointer;
            if full_width {
                s.flex_fill = true;
                s.fill_width = true;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            }
            if is_active {
                s.descriptor.background = Some(with_alpha(accent, accent.3 * 0.18));
            }
            if is_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        rounded_all(&mut tab_el, radius);
        tab_el.interaction.focusable = true;
        let mut tab_el = tab_el.child(build_tab_label(tab, theme, text_color, font_size, vertical));

        apply_drag_state(&mut tab_el, tab.value.as_str(), spec, theme);
        wire_select(&mut tab_el, is_disabled, &tab.value, on_change);
        tab_bar = tab_bar.child(tab_el);
    }
    tab_bar
}

fn render_card(
    spec: &TabsSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<&TabHandler>,
    on_close: Option<&TabHandler>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let control_y = theme.resolve_space("space.control.y");

    let accent = theme.resolve_color(spec.indicator_token());
    let border = theme.resolve_color(spec.list_border_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let surface_bg = theme.resolve_color("color.background.surface");
    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    let radius = theme.resolve_radius("radius.control");

    let card_default_bg = with_alpha(surface_bg, surface_bg.3 * 0.92);
    let card_default_border = with_alpha(border, border.3 * 0.68);
    // The old GPUI tier uses its sRGB `color_mix` helper for selected cards.
    let card_selected_bg = mix_srgb(accent, surface_bg, 0.14);
    let card_selected_border = mix_srgb(accent, border, 0.32);

    let selected = spec.current_value().map(|s| s.to_string());
    let vertical = spec.is_vertical();
    let full_width = spec.uses_full_width();

    let mut tab_bar = Node::container();
    {
        let s = &mut tab_bar.style;
        if vertical {
            s.descriptor.layout.direction = LayoutDirection::Column;
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::End;
            if full_width {
                s.fill_width = true;
            }
        }
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
    }

    for tab in &spec.tabs {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;
        let text_color = if is_active {
            text_primary
        } else {
            text_secondary
        };
        let (bg, bc) = if is_active {
            (card_selected_bg, card_selected_border)
        } else {
            (card_default_bg, card_default_border)
        };

        let mut tab_el = Node::container();
        {
            let s = &mut tab_el.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.spacing.padding.top = control_y;
            s.descriptor.layout.spacing.padding.bottom = control_y;
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.background = Some(bg);
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = bc;
            s.descriptor.cursor = CursorHint::Pointer;
            if full_width {
                s.flex_fill = true;
                s.fill_width = true;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            }
            if is_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        rounded_all(&mut tab_el, radius);
        tab_el.interaction.focusable = true;
        let mut tab_el = tab_el.child(build_tab_label(tab, theme, text_color, font_size, vertical));

        if tab.is_closable {
            let mut close = build_close_button(theme, &tab.label);
            close.interaction.on_activate = Some(match (is_disabled, on_close) {
                (false, Some(handler)) => {
                    let handler = Arc::clone(handler);
                    let value = tab.value.clone();
                    Arc::new(move || handler(&value))
                }
                // Inert but still the nearest clickable: an unwired X would
                // bubble to the tab and select what it was closing.
                _ => Arc::new(|| {}),
            });
            tab_el = tab_el.child(close);
        }

        apply_drag_state(&mut tab_el, tab.value.as_str(), spec, theme);
        wire_select(&mut tab_el, is_disabled, &tab.value, on_change);
        tab_bar = tab_bar.child(tab_el);
    }
    tab_bar
}

fn render_pill(spec: &TabsSpec, theme: &dyn ThemeProvider, on_change: Option<&TabHandler>) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let control_height = rem_to_px(control_height_rem(effective_size));
    let tab_height = control_height - rem_to_px(0.5);
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let accent = theme.resolve_color(spec.indicator_token());
    let border_subtle = theme.resolve_color(spec.list_border_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    let pill_radius = theme.resolve_radius("radius.pill");

    let container_border = with_alpha(border_subtle, border_subtle.3 * spec.pill_border_opacity());
    let active_bg = with_alpha(accent, accent.3 * spec.pill_active_bg_opacity());

    let selected = spec.current_value().map(|s| s.to_string());

    let mut container = Node::container();
    {
        let s = &mut container.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.descriptor.border.width = 2.0;
        s.descriptor.border.color = container_border;
        let p = rem_to_px(0.1875);
        s.descriptor.layout.spacing.padding.left = p;
        s.descriptor.layout.spacing.padding.right = p;
        s.descriptor.layout.spacing.padding.top = p;
        s.descriptor.layout.spacing.padding.bottom = p;
    }
    rounded_all(&mut container, pill_radius);

    for tab in &spec.tabs {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;
        let text_color = if is_active {
            text_primary
        } else {
            text_secondary
        };

        let mut tab_el = Node::container();
        {
            let s = &mut tab_el.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.height = LayoutSizing::Fixed(tab_height);
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.cursor = CursorHint::Pointer;
            if is_active {
                s.descriptor.background = Some(active_bg);
            }
            if is_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        rounded_all(&mut tab_el, pill_radius);
        tab_el.interaction.focusable = true;
        // Pill is always horizontal.
        let mut tab_el = tab_el.child(build_tab_label(tab, theme, text_color, font_size, false));

        apply_drag_state(&mut tab_el, tab.value.as_str(), spec, theme);
        wire_select(&mut tab_el, is_disabled, &tab.value, on_change);
        container = container.child(tab_el);
    }
    container
}

fn render_block(
    spec: &TabsSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<&TabHandler>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let control_height = rem_to_px(control_height_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let accent = theme.resolve_color(spec.indicator_token());
    let border = theme.resolve_color(spec.list_border_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let surface_bg = theme.resolve_color("color.background.surface");
    let panel_bg = theme.resolve_color("color.background.panel");
    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());

    let list_bg = with_alpha(panel_bg, panel_bg.3 * spec.block_list_bg_opacity());
    let separator = with_alpha(border, border.3 * spec.block_separator_opacity());
    let selected_bg = mix_srgb(accent, surface_bg, spec.block_selected_accent_mix());

    let selected = spec.current_value().map(|s| s.to_string());
    let vertical = spec.is_vertical();

    let mut tab_bar = Node::container();
    {
        let s = &mut tab_bar.style;
        s.descriptor.background = Some(list_bg);
        s.descriptor.border.color = border;
        if vertical {
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.border_right_width = Some(1.0);
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.fill_width = true;
            s.border_bottom_width = Some(1.0);
        }
    }

    for (idx, tab) in spec.tabs.iter().enumerate() {
        let is_active = selected.as_deref() == Some(tab.value.as_str());
        let is_disabled = tab.is_disabled;
        let text_color = if is_active {
            text_primary
        } else {
            text_secondary
        };

        let mut tab_el = Node::container();
        {
            let s = &mut tab_el.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.height = LayoutSizing::Fixed(control_height);
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.cursor = CursorHint::Pointer;
            if vertical {
                s.fill_width = true;
            } else if spec.uses_full_width() {
                s.flex_fill = true;
                s.fill_width = true;
            }
            // Sibling separator: left border (horizontal) / top border (vertical).
            if idx > 0 {
                s.descriptor.border.color = separator;
                if vertical {
                    s.border_top_width = Some(1.0);
                } else {
                    s.border_left_width = Some(1.0);
                }
            }
            if is_active {
                s.descriptor.background = Some(selected_bg);
            }
            if is_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        tab_el.interaction.focusable = true;
        let mut tab_el = tab_el.child(build_tab_label(tab, theme, text_color, font_size, vertical));

        apply_drag_state(&mut tab_el, tab.value.as_str(), spec, theme);
        wire_select(&mut tab_el, is_disabled, &tab.value, on_change);
        tab_bar = tab_bar.child(tab_el);
    }
    tab_bar
}
