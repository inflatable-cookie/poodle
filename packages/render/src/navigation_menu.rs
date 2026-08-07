//! NavigationMenu — a horizontal menu bar with an optional viewport panel.
//!
//! Contract: `docs/contracts/components/navigation-menu.md`
//! Ported from: `packages/jetstream/components/src/navigation_menu.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeRole, StylePatch};
use poodle_specs::{ControlDensity, NavigationMenuSpec};

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size};

/// Trigger horizontal padding in rem per density (contract §8 Density table):
/// compact 0.5, default/comfortable 0.75 — NOT the generic ladder.
fn nav_trigger_pad_x_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default | ControlDensity::Comfortable => 0.75,
    }
}

fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

/// `on_change` fires with the value of the entry that was chosen.
pub fn navigation_menu(
    spec: &NavigationMenuSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.6875,
        poodle_specs::ControlSize::Sm | poodle_specs::ControlSize::Md => 0.75,
        poodle_specs::ControlSize::Lg => 0.8125,
        poodle_specs::ControlSize::Xl => 0.875,
    });
    let pad_x = rem_to_px(nav_trigger_pad_x_rem(spec.density));

    // List gap = space-inline-sm (contract §7/§8).
    let list_gap = theme.resolve_space("space.inline.sm");

    // Trigger pill geometry.
    let radius = theme.resolve_radius(spec.trigger_radius_token());
    // Border width = 0.0625rem (contract §8 trigger border).
    let border_w = rem_to_px(0.0625);
    // Trigger min-height = size-control-height (contract §8).
    let control_height = theme.resolve_space("size.control.height");

    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");
    let surface = theme.resolve_color("color.background.surface");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let border_default = theme.resolve_color("color.border.default");

    // Hover/focus trigger: background = color-mix(accent 12%, transparent).
    let hover_bg = with_alpha(accent, accent.3 * 0.12);

    // Idle trigger: surface@88% fill, border-subtle@72% border.
    let idle_bg = with_alpha(surface, surface.3 * 0.88);
    let idle_border = with_alpha(border_subtle, border_subtle.3 * 0.72);

    // Active (open) trigger: accent@16% fill, accent42%↔border-default border.
    let active_bg = with_alpha(accent, accent.3 * 0.16);
    let active_border = mix_srgb(accent, border_default, 0.42);

    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());

    let current = spec.current_value();

    // List: inline-flex row, wrap, gap inline-sm, align center.
    let mut list = Node::container();
    {
        let s = &mut list.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = list_gap;
    }

    for entry in &spec.items {
        let is_active = current == Some(entry.value.as_str());

        let (bg, border_color) = if is_active {
            (active_bg, active_border)
        } else {
            (idle_bg, idle_border)
        };

        // Contract §3 `icon`: an entry with a leading icon composes icon +
        // label as explicit children separated by the trigger gap; the icon
        // is tinted to the trigger foreground and sized to the trigger font.
        let mut btn = {
            let mut b = Node::button("");
            b.a11y.role = Some(NodeRole::Button);
            b.style.descriptor.layout.direction = LayoutDirection::Row;
            b.style.descriptor.layout.spacing.gap = list_gap;
            if let Some(ref icon_name) = entry.icon {
                let mut glyph = Node::icon(icon_name.as_str(), font_size);
                glyph.style.descriptor.text_color = Some(text_primary);
                b = b.child(glyph);
            }
            let mut label = Node::text(&entry.label);
            label.style.descriptor.text_color = Some(text_primary);
            label.style.text_size = Some(font_size);
            label.style.text_weight = Some(600);
            b.child(label)
        };
        {
            let s = &mut btn.style;
            s.min_height = Some(control_height);
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pad_x;
            pad.right = pad_x;
            s.descriptor.border.width = border_w;
            s.descriptor.border.color = border_color;
            s.descriptor.background = Some(bg);
            s.descriptor.cursor = CursorHint::Pointer;
        }
        all_corners(&mut btn, radius);
        btn.interaction.focusable = true;

        if entry.is_disabled {
            btn.style.descriptor.opacity = disabled_opacity;
            btn.interaction.disabled = true;
        } else {
            if let Some(handler) = &on_change {
                let handler = Arc::clone(handler);
                let value = entry.value.clone();
                btn.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }

            // Hover: accent-12% fill. Active triggers keep their open fill on
            // hover in the reference — the patch overrides bg on both, which
            // matches the reference tier's `.hover(bg)` behaviour.
            btn.style.hover = Some(StylePatch {
                background: Some(hover_bg),
                border_color: None,
                text_color: None,
                opacity: None,
            });
        }

        list = list.child(btn);
    }

    // Root: column of list + optional viewport, gap stack-md, min-width 0.
    let root_gap = theme.resolve_space(spec.viewport_gap_token());
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.min_width = Some(0.0);
        s.descriptor.layout.spacing.gap = root_gap;
    }
    let mut root = root.child(list);

    // Viewport — rendered only when an item is active (contract §2/§4).
    // Contract §8: panel padding, border-subtle@74% hairline, radius-surface,
    // panel@96% fill and elevation-overlay shadow.
    if let Some(active_item) = spec.current_item() {
        let panel_x = rem_to_px(panel_space_x_rem(spec.density));
        let panel_y = rem_to_px(panel_space_y_rem(spec.density));
        let viewport_radius = theme.resolve_radius(spec.viewport_radius_token());
        let panel = theme.resolve_color("color.background.panel");
        let panel_bg = with_alpha(panel, panel.3 * 0.96);
        let viewport_border = with_alpha(border_subtle, border_subtle.3 * 0.74);

        let mut viewport = Node::container();
        {
            let s = &mut viewport.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.min_width = Some(0.0);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = panel_y;
            pad.bottom = panel_y;
            pad.left = panel_x;
            pad.right = panel_x;
            s.descriptor.border.width = border_w;
            s.descriptor.border.color = viewport_border;
            s.descriptor.background = Some(panel_bg);
            s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
        }
        all_corners(&mut viewport, viewport_radius);

        // Content slot: the entry's `description` stands in for host viewport
        // content; when absent, only the panel chrome renders.
        let mut viewport = viewport;
        if let Some(description) = active_item.description.as_deref() {
            let mut d = Node::text(description);
            d.style.descriptor.text_color = Some(text_secondary);
            d.style.text_size = Some(theme.resolve_space("typography.body.size"));
            viewport = viewport.child(d);
        }

        root = root.child(viewport);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}
