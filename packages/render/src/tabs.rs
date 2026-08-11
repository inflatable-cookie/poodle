//! Tabs — a tab bar in the card, pill, and block variants.
//!
//! Contract: `docs/contracts/components/tabs.md`
//! Ported from: `packages/jetstream/components/src/tabs/`. Content for the
//! active tab renders below this element, by the caller. The card variant
//! renders icon, count, and close-button accessories; close buttons wire
//! through `on_close` (inert when unwired, so an unwired X does not bubble to
//! the tab and select what it was closing).
//!
//! There is no `TabVariant::Strip`: the strip renders through the separate
//! `TabStripSpec`/`TabStrip` component on the native targets. Known gap,
//! deliberately deferred — recorded in the g13-013 batch log.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeRole, ShadowLayer,
};
use poodle_specs::{ActiveEdge, ActiveFill, TabVariant, TabsSpec};

use crate::color::{mix_srgb, with_alpha, TRANSPARENT};
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};

pub type TabHandler = Arc<dyn Fn(&str) + Send + Sync>;

fn rounded_all(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

/// The selection edge (contract §8 `activeEdge`): `Outline` draws a 1px accent
/// border around the active tab — `mix_srgb(accent, border-subtle, 0.32)`,
/// the former card selected-border value; `Underline` draws a 2px accent
/// border along the inline-end side — bottom horizontal, right vertical —
/// the former strip variant's indicator. Both keep a transparent reserve
/// border on every tab so selection never shifts layout. The edge axis is an
/// enum, so exactly one of these can apply. Applied after separator borders
/// (which use per-side color overrides), so block separators survive.
fn apply_active_edge(
    node: &mut Node,
    is_active: bool,
    vertical: bool,
    spec: &TabsSpec,
    theme: &dyn ThemeProvider,
) {
    match spec.active_edge {
        ActiveEdge::None => {}
        ActiveEdge::Outline => {
            let accent = theme.resolve_color(spec.indicator_token());
            let border = theme.resolve_color(spec.list_border_token());
            let selected = mix_srgb(accent, border, 0.32);
            node.style.descriptor.border.width = 1.0;
            node.style.descriptor.border.color = if is_active { selected } else { TRANSPARENT };
        }
        ActiveEdge::Underline => {
            let accent = theme.resolve_color(spec.indicator_token());
            let edge = if is_active { accent } else { TRANSPARENT };
            if vertical {
                node.style.border_right_width = Some(rem_to_px(0.125));
                node.style.descriptor.border.color = edge;
            } else {
                node.style.border_bottom_width = Some(rem_to_px(0.125));
                node.style.border_color_bottom = Some(edge);
            }
        }
    }
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

fn render_card(
    spec: &TabsSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<&TabHandler>,
    on_close: Option<&TabHandler>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let control_y = theme.resolve_space("space.control.y");

    let accent = theme.resolve_color(spec.indicator_token());
    let border = theme.resolve_color(spec.list_border_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_inverse = theme.resolve_color("color.text.inverse");
    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    let radius = theme.resolve_radius("radius.control");

    // activeEdge: outline/underline borders on the selected tab, with a
    // transparent reserve border on every tab so the bar never shifts when
    // the selected border becomes visible (see `apply_active_edge`).
    let selected = spec.current_value().map(|s| s.to_string());
    let vertical = spec.is_vertical();
    let full_width = spec.uses_full_width();
    let solid = spec.active_fill == ActiveFill::Solid;

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
        let text_color = if is_active && solid {
            text_inverse
        } else if is_active {
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
            if is_active && spec.active_fill != ActiveFill::None {
                s.descriptor.background = if solid {
                    Some(accent)
                } else {
                    Some(with_alpha(accent, accent.3 * 0.18))
                };
            }
            if is_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        apply_active_edge(&mut tab_el, is_active, vertical, spec, theme);
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
            if is_active && spec.active_fill != ActiveFill::None {
                s.descriptor.background = Some(active_bg);
            }
            if is_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        apply_active_edge(&mut tab_el, is_active, false, spec, theme);
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
            // Sibling separator: left border (horizontal) / top border
            // (vertical). Per-side color overrides, so the selection edge
            // (which owns `descriptor.border.color`) does not clobber them.
            if idx > 0 {
                if vertical {
                    s.border_top_width = Some(1.0);
                    s.border_color_top = Some(separator);
                } else {
                    s.border_left_width = Some(1.0);
                    s.border_color_left = Some(separator);
                }
            }
            if is_active && spec.active_fill != ActiveFill::None {
                s.descriptor.background = Some(selected_bg);
            }
            if is_disabled {
                s.descriptor.opacity = disabled_opacity;
            }
        }
        apply_active_edge(&mut tab_el, is_active, vertical, spec, theme);
        tab_el.interaction.focusable = true;
        let mut tab_el = tab_el.child(build_tab_label(tab, theme, text_color, font_size, vertical));

        apply_drag_state(&mut tab_el, tab.value.as_str(), spec, theme);
        wire_select(&mut tab_el, is_disabled, &tab.value, on_change);
        tab_bar = tab_bar.child(tab_el);
    }
    tab_bar
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{ActiveEdge, ActiveFill, TabDefinition};
    use std::sync::Mutex;

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// The tab element for `value` — `apply_drag_state` tags it `tabs:{value}`.
    fn tab_of<'a>(root: &'a Node, value: &str) -> &'a Node {
        root.find(&|n| n.id.as_deref() == Some(&format!("tabs:{value}")))
            .unwrap_or_else(|| panic!("tab {value} exists"))
    }

    #[test]
    fn card_renderer_renders_icon_count_and_close_wired_to_on_close() {
        let theme = theme();
        let closed = Arc::new(Mutex::new(Vec::new()));
        let on_close: TabHandler = {
            let closed = Arc::clone(&closed);
            Arc::new(move |value: &str| closed.lock().unwrap().push(value.to_string()))
        };
        let spec = TabsSpec::new(vec![
            TabDefinition::new("index.ts", "index.ts").with_icon("file"),
            TabDefinition::new("App.svelte", "App.svelte")
                .with_count(3)
                .with_closable(true),
        ])
        .with_variant(TabVariant::Card)
        .with_value("index.ts");

        let root = tabs(&spec, &theme, None, Some(on_close));

        assert!(
            root.find(&|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "file"))
                .is_some(),
            "the card renderer draws the item icon"
        );
        assert!(
            root.find(&|n| matches!(&n.kind, poodle_node::NodeKind::Text { content } if content == "3"))
                .is_some(),
            "the card renderer draws the count badge"
        );

        let close = root
            .find(&|n| n.a11y.label.as_deref() == Some("Close App.svelte"))
            .unwrap_or_else(|| panic!("the closable tab renders a close button"));
        close.interaction.on_activate.as_ref().unwrap()();
        assert_eq!(*closed.lock().unwrap(), vec!["App.svelte"]);
    }

    #[test]
    fn card_renderer_solid_fill_uses_accent_with_inverse_foreground() {
        let theme = theme();
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A"), TabDefinition::new("b", "B")])
            .with_variant(TabVariant::Card)
            .with_active_fill(ActiveFill::Solid)
            .with_value("a");

        let root = tabs(&spec, &theme, None, None);
        let accent = theme.resolve_color(spec.indicator_token());
        let inverse = theme.resolve_color("color.text.inverse");

        let active = tab_of(&root, "a");
        assert_eq!(active.style.descriptor.background, Some(accent));
        assert_eq!(active.style.descriptor.text_color, Some(inverse));

        let inactive = tab_of(&root, "b");
        assert_eq!(inactive.style.descriptor.background, None);
        assert_eq!(
            inactive.style.descriptor.text_color,
            Some(theme.resolve_color("color.text.secondary"))
        );
    }

    #[test]
    fn card_renderer_active_edge_outline_borders_only_the_selected_tab() {
        let theme = theme();
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A"), TabDefinition::new("b", "B")])
            .with_variant(TabVariant::Card)
            .with_active_edge(ActiveEdge::Outline)
            .with_value("a");

        let root = tabs(&spec, &theme, None, None);
        let accent = theme.resolve_color(spec.indicator_token());
        let border = theme.resolve_color(spec.list_border_token());
        let expected = mix_srgb(accent, border, 0.32);

        let active = tab_of(&root, "a");
        assert_eq!(active.style.descriptor.border.width, 1.0);
        assert_eq!(active.style.descriptor.border.color, expected);

        // Unselected tabs keep a transparent border so the outline never
        // shifts the bar when selection moves.
        let inactive = tab_of(&root, "b");
        assert_eq!(inactive.style.descriptor.border.width, 1.0);
        assert_eq!(
            inactive.style.descriptor.border.color,
            ColorValue(0.0, 0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn card_renderer_does_not_draw_outline_by_default() {
        let theme = theme();
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A"), TabDefinition::new("b", "B")])
            .with_variant(TabVariant::Card)
            .with_value("a");

        let root = tabs(&spec, &theme, None, None);
        assert_eq!(tab_of(&root, "a").style.descriptor.border.width, 0.0);
        assert_eq!(tab_of(&root, "b").style.descriptor.border.width, 0.0);
    }

    #[test]
    fn block_renderer_underline_edges_only_the_selected_tab() {
        let theme = theme();
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A"), TabDefinition::new("b", "B")])
            .with_variant(TabVariant::Block)
            .with_active_edge(ActiveEdge::Underline)
            .with_value("a");

        let root = tabs(&spec, &theme, None, None);
        let accent = theme.resolve_color(spec.indicator_token());

        let active = tab_of(&root, "a");
        assert_eq!(active.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(active.style.border_color_bottom, Some(accent));

        // Unselected tabs keep a transparent reserve edge so the underline
        // never shifts the bar when selection moves.
        let inactive = tab_of(&root, "b");
        assert_eq!(inactive.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(inactive.style.border_color_bottom, Some(TRANSPARENT));
    }

    #[test]
    fn block_renderer_vertical_underline_uses_the_inline_end_edge() {
        let theme = theme();
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A"), TabDefinition::new("b", "B")])
            .with_variant(TabVariant::Block)
            .with_orientation(poodle_specs::Orientation::Vertical)
            .with_active_edge(ActiveEdge::Underline)
            .with_value("a");

        let root = tabs(&spec, &theme, None, None);
        let accent = theme.resolve_color(spec.indicator_token());

        let active = tab_of(&root, "a");
        assert_eq!(active.style.border_right_width, Some(rem_to_px(0.125)));
        assert_eq!(active.style.descriptor.border.color, accent);
    }

    #[test]
    fn block_renderer_keeps_separators_under_outline() {
        let theme = theme();
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A"), TabDefinition::new("b", "B")])
            .with_variant(TabVariant::Block)
            .with_active_edge(ActiveEdge::Outline)
            .with_value("a");

        let root = tabs(&spec, &theme, None, None);
        let separator = with_alpha(
            theme.resolve_color("color.border.subtle"),
            theme.resolve_color("color.border.subtle").3 * 0.72,
        );

        // The second item's left separator survives the outline: per-side
        // color override wins over the uniform outline border.
        let second = tab_of(&root, "b");
        assert_eq!(second.style.border_left_width, Some(1.0));
        assert_eq!(second.style.border_color_left, Some(separator));
        // The outline still applies to the remaining sides.
        assert_eq!(second.style.descriptor.border.width, 1.0);
        assert_eq!(second.style.descriptor.border.color, TRANSPARENT);
    }

    #[test]
    fn none_fill_suppresses_selected_background_on_every_variant() {
        let theme = theme();
        let text_primary = theme.resolve_color("color.text.primary");
        for variant in [TabVariant::Card, TabVariant::Pill, TabVariant::Block] {
            let spec =
                TabsSpec::new(vec![TabDefinition::new("a", "A"), TabDefinition::new("b", "B")])
                    .with_variant(variant)
                    .with_active_fill(ActiveFill::None)
                    .with_value("a");

            let root = tabs(&spec, &theme, None, None);
            let active = tab_of(&root, "a");
            assert_eq!(
                active.style.descriptor.background, None,
                "{variant:?} must not fill the selected tab under None"
            );
            // The selected text colour is unaffected: text-primary, never the
            // inverse swap solid uses.
            assert_eq!(
                active.style.descriptor.text_color,
                Some(text_primary),
                "{variant:?} selected text colour must be unaffected"
            );
            let inactive = tab_of(&root, "b");
            assert_eq!(inactive.style.descriptor.background, None);
        }
    }

    #[test]
    fn block_none_fill_keeps_underline_edge() {
        let theme = theme();
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A"), TabDefinition::new("b", "B")])
            .with_variant(TabVariant::Block)
            .with_active_fill(ActiveFill::None)
            .with_active_edge(ActiveEdge::Underline)
            .with_value("a");

        let root = tabs(&spec, &theme, None, None);
        let accent = theme.resolve_color(spec.indicator_token());

        // The strip equivalent: underline and no fill.
        let active = tab_of(&root, "a");
        assert_eq!(active.style.descriptor.background, None);
        assert_eq!(active.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(active.style.border_color_bottom, Some(accent));

        // Unselected tabs keep the transparent reserve edge.
        let inactive = tab_of(&root, "b");
        assert_eq!(inactive.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(inactive.style.border_color_bottom, Some(TRANSPARENT));
    }
}
