//! NavigationMenu — a horizontal menu bar with an optional viewport panel.
//!
//! Contract: `docs/contracts/components/navigation-menu.md`
//! Ported from: `packages/jetstream/components/src/navigation_menu.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeRole, StylePatch};
use poodle_specs::{ActiveEdge, ActiveFill, ControlDensity, NavigationMenuSpec};

use crate::color::{mix_srgb, with_alpha, TRANSPARENT};
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
    let text_inverse = theme.resolve_color("color.text.inverse");
    let accent = theme.resolve_color("color.accent.base");
    let surface = theme.resolve_color("color.background.surface");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let border_default = theme.resolve_color("color.border.default");

    // Hover/focus trigger: background = color-mix(accent 12%, transparent).
    let hover_bg = with_alpha(accent, accent.3 * 0.12);

    // Idle trigger: surface@88% fill, borderless since g13.016 (the edge
    // is opt-in via `activeEdge`).
    let idle_bg = with_alpha(surface, surface.3 * 0.88);

    // Active (open) trigger: accent@16% fill.
    let active_bg = with_alpha(accent, accent.3 * 0.16);

    // activeEdge::Outline: the former default trigger border, now opt-in.
    // Open trigger border = accent-42% ↔ border-default (the old open border
    // value); other triggers carry a transparent reserve border so selection
    // does not shift layout.
    let outline_selected_border = mix_srgb(accent, border_default, 0.42);

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

        // Solid fill: fully accent-filled open trigger with an inverse
        // foreground (the same token the primary Button uses on accent-base).
        let solid = is_active && spec.active_fill == ActiveFill::Solid;
        let text_color = if solid { text_inverse } else { text_primary };

        // activeEdge — the border axis is an enum, so exactly one of
        // outline/underline applies. Outline restores the former default
        // trigger border; underline draws the accent bottom edge. Both keep a
        // transparent reserve on every trigger so selection does not shift
        // layout.
        let outline_on = spec.active_edge == ActiveEdge::Outline;
        let (bg, border_color, border_width) = if is_active {
            let fill_bg = if solid {
                accent
            } else if spec.active_fill == ActiveFill::None {
                // The off value of the fill axis: no selection fill, so the
                // open trigger keeps the idle trigger fill (which is not a
                // selection treatment). Selection is marked by the edge and
                // the selected text colour alone.
                idle_bg
            } else {
                active_bg
            };
            (
                fill_bg,
                if outline_on { outline_selected_border } else { TRANSPARENT },
                if outline_on { border_w } else { 0.0 },
            )
        } else {
            // Reserve a transparent border under the outline so the open
            // trigger's visible border does not shift the list.
            (idle_bg, TRANSPARENT, if outline_on { border_w } else { 0.0 })
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
                glyph.style.descriptor.text_color = Some(text_color);
                b = b.child(glyph);
            }
            let mut label = Node::text(&entry.label);
            label.style.descriptor.text_color = Some(text_color);
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
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = border_color;
            if spec.active_edge == ActiveEdge::Underline {
                s.border_bottom_width = Some(rem_to_px(0.125));
                s.border_color_bottom = Some(if is_active { accent } else { TRANSPARENT });
            }
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

            // Hover: accent-12% fill. A solid open trigger keeps its accent
            // fill on hover — without this the fill reverts to the tint while
            // the foreground stays text-inverse, leaving inverse text on a
            // light tint (mirrors the web CSS hover-survival rule).
            btn.style.hover = Some(StylePatch {
                background: Some(if solid { accent } else { hover_bg }),
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

        // Contract `dismissOnOutsideInteract` (default `true`): a *refusal*
        // flag — native overlays dismiss on outside interact by default. The
        // refusal rides the surface's interaction as an inert activation: a
        // host implementing outside-dismissal must not dismiss a viewport
        // carrying this marker (see menu.rs for the full contract note).
        if !spec.dismiss_on_outside_interact {
            viewport.interaction.on_activate = Some(Arc::new(|| {}));
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

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeKind;
    use poodle_specs::NavigationMenuEntry;

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn items() -> Vec<NavigationMenuEntry> {
        vec![
            NavigationMenuEntry::new("a", "A"),
            NavigationMenuEntry::new("b", "B"),
        ]
    }

    /// The trigger button whose subtree contains `label`.
    fn trigger_of<'a>(root: &'a Node, label: &str) -> &'a Node {
        root.find(&|n| {
            n.a11y.role == Some(NodeRole::Button) && n.has_text(label)
        })
        .unwrap_or_else(|| panic!("trigger {label} exists"))
    }

    /// The label text node for `label`.
    fn label_text<'a>(root: &'a Node, label: &str) -> &'a Node {
        root.find(&|n| matches!(&n.kind, NodeKind::Text { content } if content == label))
            .unwrap_or_else(|| panic!("label text {label} exists"))
    }

    #[test]
    fn default_trigger_is_borderless_with_tint_fill() {
        let theme = theme();
        let spec = NavigationMenuSpec::new(items()).with_value("a");
        let accent = theme.resolve_color("color.accent.base");

        let root = navigation_menu(&spec, &theme, None);
        let open = trigger_of(&root, "A");
        assert_eq!(open.style.descriptor.border.width, 0.0);
        assert_eq!(
            open.style.descriptor.background,
            Some(with_alpha(accent, accent.3 * 0.16))
        );

        let closed = trigger_of(&root, "B");
        assert_eq!(closed.style.descriptor.border.width, 0.0);
        assert_eq!(
            closed.style.descriptor.background,
            Some(with_alpha(
                theme.resolve_color("color.background.surface"),
                theme.resolve_color("color.background.surface").3 * 0.88
            ))
        );
    }

    #[test]
    fn active_edge_outline_reserves_transparent_border_and_marks_open_trigger() {
        let theme = theme();
        let spec = NavigationMenuSpec::new(items())
            .with_value("a")
            .with_active_edge(ActiveEdge::Outline);
        let accent = theme.resolve_color("color.accent.base");
        let border_default = theme.resolve_color("color.border.default");

        let root = navigation_menu(&spec, &theme, None);
        let open = trigger_of(&root, "A");
        assert_eq!(open.style.descriptor.border.width, rem_to_px(0.0625));
        assert_eq!(
            open.style.descriptor.border.color,
            mix_srgb(accent, border_default, 0.42)
        );

        let closed = trigger_of(&root, "B");
        assert_eq!(closed.style.descriptor.border.width, rem_to_px(0.0625));
        assert_eq!(closed.style.descriptor.border.color, TRANSPARENT);
    }

    #[test]
    fn active_underline_edges_only_the_open_trigger() {
        let theme = theme();
        let spec = NavigationMenuSpec::new(items())
            .with_value("a")
            .with_active_edge(ActiveEdge::Underline);
        let accent = theme.resolve_color("color.accent.base");

        let root = navigation_menu(&spec, &theme, None);
        let open = trigger_of(&root, "A");
        assert_eq!(open.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(open.style.border_color_bottom, Some(accent));

        // Closed triggers keep a transparent reserve edge so the underline
        // never shifts the list.
        let closed = trigger_of(&root, "B");
        assert_eq!(closed.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(closed.style.border_color_bottom, Some(TRANSPARENT));
    }

    #[test]
    fn solid_fill_uses_accent_with_inverse_foreground() {
        let theme = theme();
        let spec = NavigationMenuSpec::new(items())
            .with_value("a")
            .with_active_fill(ActiveFill::Solid);
        let accent = theme.resolve_color("color.accent.base");
        let inverse = theme.resolve_color("color.text.inverse");

        let root = navigation_menu(&spec, &theme, None);
        let open = trigger_of(&root, "A");
        assert_eq!(open.style.descriptor.background, Some(accent));
        assert_eq!(label_text(&root, "A").style.descriptor.text_color, Some(inverse));

        let closed = trigger_of(&root, "B");
        assert_eq!(
            closed.style.descriptor.background,
            Some(with_alpha(
                theme.resolve_color("color.background.surface"),
                theme.resolve_color("color.background.surface").3 * 0.88
            ))
        );
        assert_eq!(
            label_text(&root, "B").style.descriptor.text_color,
            Some(theme.resolve_color("color.text.primary"))
        );
    }

    #[test]
    fn solid_open_trigger_keeps_accent_on_hover() {
        let theme = theme();
        let spec = NavigationMenuSpec::new(items())
            .with_value("a")
            .with_active_fill(ActiveFill::Solid);
        let accent = theme.resolve_color("color.accent.base");

        let root = navigation_menu(&spec, &theme, None);
        let open_hover = trigger_of(&root, "A").style.hover.as_ref().expect("hover patch");
        assert_eq!(open_hover.background, Some(accent));

        let closed_hover = trigger_of(&root, "B").style.hover.as_ref().expect("hover patch");
        assert_eq!(
            closed_hover.background,
            Some(with_alpha(accent, accent.3 * 0.12))
        );
    }

    #[test]
    fn none_fill_keeps_idle_background_and_underline_still_marks_open_trigger() {
        let theme = theme();
        let spec = NavigationMenuSpec::new(items())
            .with_value("a")
            .with_active_fill(ActiveFill::None)
            .with_active_edge(ActiveEdge::Underline);
        let accent = theme.resolve_color("color.accent.base");
        let surface = theme.resolve_color("color.background.surface");
        let idle = with_alpha(surface, surface.3 * 0.88);

        let root = navigation_menu(&spec, &theme, None);
        let open = trigger_of(&root, "A");
        // No selection fill: the open trigger keeps the idle trigger fill.
        assert_eq!(open.style.descriptor.background, Some(idle));
        // The underline still renders.
        assert_eq!(open.style.border_bottom_width, Some(rem_to_px(0.125)));
        assert_eq!(open.style.border_color_bottom, Some(accent));
        // The selected text colour is unaffected (text-primary, not inverse).
        assert_eq!(
            label_text(&root, "A").style.descriptor.text_color,
            Some(theme.resolve_color("color.text.primary"))
        );

        let closed = trigger_of(&root, "B");
        assert_eq!(closed.style.descriptor.background, Some(idle));
        assert_eq!(closed.style.border_color_bottom, Some(TRANSPARENT));
    }

    #[test]
    fn outside_interact_refusal_marks_the_open_viewport() {
        // Web default `true` + active item: no refusal marker anywhere.
        let spec = NavigationMenuSpec::new(items()).with_value("a");
        let node = navigation_menu(&spec, &theme(), None);
        assert!(node.find(&|n| n.interaction.on_activate.is_some()).is_none());

        // Refusal: the open viewport carries the inert activation marker a
        // host keys outside-dismissal on.
        let refusing = spec.with_dismiss_on_outside_interact(false);
        let node = navigation_menu(&refusing, &theme(), None);
        assert!(node.find(&|n| n.interaction.on_activate.is_some()).is_some());
    }
}
