//! Menubar — top-level triggers with a flow-placed dropdown for the open menu.
//!
//! Contract: `docs/contracts/components/menubar.md`
//! Ported from: `packages/jetstream/components/src/menubar.rs`. The open
//! overlay renders in flow below the trigger row, matching both old native
//! tiers' accepted delta from the web's absolute placement.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeRole, StylePatch};
use poodle_specs::{MenuSpec, MenubarSpec};

use crate::color::with_alpha;
use crate::menu::menu as render_menu;
use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem,
};

const LABEL_WEIGHT: u16 = 600;

fn rounded_all(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

pub fn menubar(
    spec: &MenubarSpec,
    theme: &dyn ThemeProvider,
    on_trigger: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let control_height = theme.resolve_space("size.control.height")
        + rem_to_px(size_height_offset_rem(effective_size));
    let control_radius = theme.resolve_radius("radius.control");
    let list_radius = theme.resolve_radius(spec.list_radius_token());
    let border_w = rem_to_px(0.0625);
    let list_gap = rem_to_px(0.125);
    let list_pad = rem_to_px(0.1875);

    let text_primary = theme.resolve_color("color.text.primary");
    let accent = theme.resolve_color("color.accent.base");
    let panel = theme.resolve_color("color.background.panel");
    let border_subtle = theme.resolve_color(spec.list_border_token());

    let list_border = with_alpha(border_subtle, border_subtle.3 * 0.72);
    let list_bg = with_alpha(panel, panel.3 * 0.96);
    let open_bg = with_alpha(accent, accent.3 * 0.14);

    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    let open_value = spec.current_value();

    // ── Trigger strip ──
    let mut list = Node::container();
    {
        let s = &mut list.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = list_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = list_pad;
        pad.bottom = list_pad;
        pad.left = list_pad;
        pad.right = list_pad;
        s.descriptor.border.width = border_w;
        s.descriptor.border.color = list_border;
        s.descriptor.background = Some(list_bg);
    }
    rounded_all(&mut list, list_radius);

    for entry in &spec.items {
        let is_open = open_value == Some(entry.value.as_str());

        let mut btn = Node::button(&entry.label);
        {
            let s = &mut btn.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.text_color = Some(text_primary);
            s.text_size = Some(font_size);
            s.text_weight = Some(LABEL_WEIGHT);
            s.min_height = Some(control_height);
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.cursor = CursorHint::Pointer;
            if is_open {
                s.descriptor.background = Some(open_bg);
            }
            if entry.is_disabled {
                s.descriptor.opacity = disabled_opacity;
            } else {
                s.hover = Some(StylePatch {
                    background: Some(open_bg),
                    border_color: None,
                    text_color: None,
                    opacity: None,
                });
            }
        }
        rounded_all(&mut btn, control_radius);
        btn.a11y.role = Some(NodeRole::Button);
        btn.interaction.focusable = true;
        if entry.is_disabled {
            btn.interaction.disabled = true;
        } else if let Some(handler) = &on_trigger {
            let handler = Arc::clone(handler);
            let value = entry.value.clone();
            btn.interaction.on_activate = Some(Arc::new(move || handler(&value)));
        }

        list = list.child(btn);
    }

    // ── Root: column so the open overlay renders below the strip ──
    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.min_width = Some(0.0);
    let mut root = root.child(list);

    // ── Open overlay ──
    if let Some(open_menu) = spec.current_menu() {
        if !open_menu.items.is_empty() {
            // Menubar's own `dismissOnOutsideInteract` wins over the composed
            // menu's (the alert_dialog pattern: the renderer resolves the
            // composed spec's dismissal from its own spec state).
            let menu_spec = MenuSpec::new(open_menu.items.clone())
                .with_dismiss_on_outside_interact(spec.dismiss_on_outside_interact);
            root = root.child(render_menu(&menu_spec, theme, on_select));
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::MenuBar);
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn open_spec() -> MenubarSpec {
        MenubarSpec::new(vec![poodle_specs::MenubarEntry::new(
            "file",
            "File",
            vec![poodle_specs::MenuEntry::new("open", "Open")],
        )])
        .with_value("file")
    }

    #[test]
    fn refusal_forwarded_into_open_overlay_surface() {
        // Default `true`: the open menu surface stays marker-free.
        let node = menubar(&open_spec(), &theme(), None, None);
        assert!(node
            .find(&|n| n.a11y.role == Some(NodeRole::Menu))
            .and_then(|n| n.interaction.on_activate.as_ref())
            .is_none());

        // Menubar's own refusal wins over the composed MenuSpec default and
        // reaches the rendered open overlay.
        let refusing = open_spec().with_dismiss_on_outside_interact(false);
        let node = menubar(&refusing, &theme(), None, None);
        let menu_node = node
            .find(&|n| n.a11y.role == Some(NodeRole::Menu))
            .expect("open menu overlay");
        assert!(menu_node.interaction.on_activate.is_some());
    }
}
