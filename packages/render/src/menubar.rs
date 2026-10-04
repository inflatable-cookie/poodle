//! Menubar — top-level triggers with a flow-placed dropdown for the open menu.
//!
//! Contract: `docs/contracts/components/menubar.md`
//! Ported from: `packages/jetstream/components/src/menubar.rs`. The open
//! overlay renders in flow below the trigger row, matching both old native
//! tiers' accepted delta from the web's absolute placement.

use std::sync::Arc;

use poodle_headless::menu::{menu_list_navigate, MenuListMove};
use poodle_node::{
    CrossAxisAlignment, CursorHint, DismissReason, HasPopup, LayoutDirection, Node, NodeKey,
    NodeRole, StylePatch,
};
use poodle_specs::{MenuSpec, MenubarSpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::menu::menu as render_menu;
use crate::presentation::{control_space_x_rem, rem_to_px, size_font_rem, size_height_offset_rem};

const LABEL_WEIGHT: u16 = 600;

/// The layer id the open composition registers on the backend dismiss
/// stack. Containment is the strip plus the open overlay (Svelte tests
/// outside dismissal against both), so trigger presses never dismiss.
/// Single-flight like the shared `menu-item:*` ids.
pub const MENUBAR_LAYER_ID: &str = "menubar-layer";

/// Host-owned interaction intent. The backend turns trigger keys and
/// dismissal into real listeners; the host owns the open value.
#[derive(Clone, Default)]
pub struct MenubarHandlers {
    /// Trigger activation (click / Enter / Space). The host toggles.
    pub on_trigger: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Overlay item commit. The host closes.
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Document-level dismissal (escape / outside). Present registers the
    /// open composition on the dismiss stack; absent keeps the previous
    /// behavior (the host owns dismissal entirely).
    pub on_dismiss: Option<Arc<dyn Fn(DismissReason) + Send + Sync>>,
}

/// Focus target for one arrow step across triggers, or `None` when the
/// step goes nowhere (single enabled trigger). Wrapping skip-disabled
/// matches Svelte's `findNextEnabledIndex`/`firstEnabledIndex` pair.
fn roving_target(disabled: &[bool], ids: &[String], idx: usize, mv: MenuListMove) -> Option<String> {
    let next = menu_list_navigate(disabled, idx, mv);
    if next == idx {
        return None;
    }
    Some(ids[next].clone())
}

fn rounded_all(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

pub fn menubar(spec: &MenubarSpec, ctx: &RenderContext<'_>, handlers: MenubarHandlers) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(density));

    let control_height = ctx.theme().resolve_space("size.control.height")
        + rem_to_px(size_height_offset_rem(effective_size));
    let control_radius = ctx.theme().resolve_radius("radius.control");
    let list_radius = ctx.theme().resolve_radius(spec.list_radius_token());
    let border_w = rem_to_px(0.0625);
    let list_gap = rem_to_px(0.125);
    let list_pad = rem_to_px(0.1875);

    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let accent = ctx.theme().resolve_color("color.accent.base");
    let panel = ctx.theme().resolve_color("color.background.panel");
    let border_subtle = ctx.theme().resolve_color(spec.list_border_token());

    let list_border = with_alpha(border_subtle, border_subtle.3 * 0.72);
    let list_bg = with_alpha(panel, panel.3 * 0.96);
    let open_bg = with_alpha(accent, accent.3 * 0.14);

    let disabled_opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
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

    // Trigger focus roving (Svelte moves focus only on arrows; opening is
    // a host transition below). Disabled triggers are skipped with
    // wrapping, through the shared menu-list machinery.
    let trigger_disabled: Vec<bool> =
        spec.items.iter().map(|entry| entry.is_disabled).collect();
    let trigger_ids: Vec<String> = spec
        .items
        .iter()
        .map(|entry| format!("menubar-trigger:{}", entry.value))
        .collect();
    // The open composition (strip plus overlay) registers one containment
    // unit, but only while a host actually owns dismissal.
    let layered = handlers.on_dismiss.is_some() && open_value.is_some();

    for (idx, entry) in spec.items.iter().enumerate() {
        let is_open = open_value == Some(entry.value.as_str());
        // Svelte trigger identity: role=menuitem with popup linkage. The
        // overlay id below must match `aria-controls` exactly.
        let overlay_id = format!("menubar-menu:{}", entry.value);
        let trigger_id = trigger_ids[idx].clone();

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
                // Svelte paints the same accent tint for hover, keyboard
                // focus, and the open trigger; the focus patch also mints
                // the backend focus handle keyboard proofs drive.
                s.hover = Some(StylePatch {
                    background: Some(open_bg),
                    border_color: None,
                    text_color: None,
                    opacity: None,
                });
            }
        }
        rounded_all(&mut btn, control_radius);
        btn.id = Some(trigger_id);
        btn.a11y.role = Some(NodeRole::MenuItem);
        btn.a11y.has_popup = Some(HasPopup::Menu);
        btn.a11y.expanded = Some(is_open);
        if is_open {
            btn.a11y.controls = Some(overlay_id.clone());
        }
        btn.interaction.focusable = true;
        if layered {
            btn.interaction.dismiss_layer = Some(MENUBAR_LAYER_ID.to_string());
        }
        if entry.is_disabled {
            btn.interaction.disabled = true;
        } else {
            btn.style.focus = Some(StylePatch {
                background: Some(open_bg),
                border_color: None,
                text_color: None,
                opacity: None,
            });
            // Trigger keys (Svelte trigger keydown): arrows rove focus with
            // wrapping; ArrowDown opens through the trigger channel (the
            // host toggles, which opens from closed, then applies its own
            // focus-first-item effect like Svelte's open effect). Escape is
            // deliberately absent: the window dismisses the registered
            // layer, so handling it here would fire twice.
            {
                let key_trigger = handlers.on_trigger.clone();
                let value = entry.value.clone();
                let siblings = trigger_disabled.clone();
                let ids = trigger_ids.clone();
                let trigger_open = is_open;
                btn.interaction.on_key = Some(Arc::new(move |key, _modifiers| {
                    match key {
                        NodeKey::ArrowRight => {
                            roving_target(&siblings, &ids, idx, MenuListMove::Next)
                        }
                        NodeKey::ArrowLeft => {
                            roving_target(&siblings, &ids, idx, MenuListMove::Prev)
                        }
                        NodeKey::Home => {
                            roving_target(&siblings, &ids, idx, MenuListMove::First)
                        }
                        NodeKey::End => {
                            roving_target(&siblings, &ids, idx, MenuListMove::Last)
                        }
                        NodeKey::ArrowDown => {
                            // Svelte opens the focused trigger's menu: a
                            // no-op when this menu is already open, an
                            // open through the trigger channel otherwise
                            // (the host toggles, which opens from closed
                            // and switches from another menu).
                            if trigger_open {
                                return None;
                            }
                            if let Some(trigger) = &key_trigger {
                                trigger(&value);
                            }
                            None
                        }
                        _ => None,
                    }
                }));
            }
            if let Some(handler) = &handlers.on_trigger {
                let handler = Arc::clone(handler);
                let value = entry.value.clone();
                btn.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }
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
                .with_aria_label(open_menu.label.clone())
                .with_dismiss_on_outside_interact(spec.dismiss_on_outside_interact);
            let mut overlay = render_menu(&menu_spec, ctx, handlers.on_select.clone());
            // The trigger's `aria-controls` target: `menubar-menu:{value}`.
            overlay.id = Some(format!("menubar-menu:{}", open_menu.value));
            if let Some(on_dismiss) = handlers.on_dismiss.clone() {
                overlay.interaction.dismiss_layer = Some(MENUBAR_LAYER_ID.to_string());
                overlay.interaction.on_dismiss = Some(on_dismiss);
            }
            root = root.child(overlay);
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
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        // Default `true`: the open menu surface stays marker-free.
        let node = menubar(&open_spec(), &ctx, MenubarHandlers::default());
        assert!(node
            .find(&|n| n.a11y.role == Some(NodeRole::Menu))
            .and_then(|n| n.interaction.on_activate.as_ref())
            .is_none());

        // Menubar's own refusal wins over the composed MenuSpec default and
        // reaches the rendered open overlay.
        let refusing = open_spec().with_dismiss_on_outside_interact(false);
        let node = menubar(&refusing, &ctx, MenubarHandlers::default());
        let menu_node = node
            .find(&|n| n.a11y.role == Some(NodeRole::Menu))
            .expect("open menu overlay");
        assert!(menu_node.interaction.on_activate.is_some());
    }

    #[test]
    fn trigger_keys_and_dismiss_layer_follow_the_handlers() {
        use poodle_node::{NodeKey, NodeModifiers};
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        // No handlers: triggers rove and activate, but no layer registers.
        let bare = menubar(&open_spec(), &ctx, MenubarHandlers::default());
        let trigger = bare
            .find(&|n| n.id.as_deref() == Some("menubar-trigger:file"))
            .expect("file trigger");
        assert!(trigger.interaction.on_key.is_some());
        assert!(trigger.interaction.dismiss_layer.is_none());
        // ArrowDown with no trigger channel is inert.
        let down = trigger.interaction.on_key.as_ref().expect("key handler");
        assert_eq!(down(NodeKey::ArrowDown, NodeModifiers::default()), None);

        // Dismiss handler: the open strip and overlay share one layer.
        let layered = menubar(
            &open_spec(),
            &ctx,
            MenubarHandlers {
                on_dismiss: Some(Arc::new(|_| {})),
                ..MenubarHandlers::default()
            },
        );
        let trigger = layered
            .find(&|n| n.id.as_deref() == Some("menubar-trigger:file"))
            .expect("file trigger");
        assert_eq!(
            trigger.interaction.dismiss_layer.as_deref(),
            Some(MENUBAR_LAYER_ID)
        );
        let overlay = layered
            .find(&|n| n.a11y.role == Some(NodeRole::Menu))
            .expect("open overlay");
        assert_eq!(
            overlay.interaction.dismiss_layer.as_deref(),
            Some(MENUBAR_LAYER_ID)
        );
        assert!(overlay.interaction.on_dismiss.is_some());

        // Closed: no layer anywhere even with a handler.
        let closed = menubar(
            &MenubarSpec::new(vec![poodle_specs::MenubarEntry::new(
                "file",
                "File",
                vec![poodle_specs::MenuEntry::new("open", "Open")],
            )]),
            &ctx,
            MenubarHandlers {
                on_dismiss: Some(Arc::new(|_| {})),
                ..MenubarHandlers::default()
            },
        );
        assert!(closed
            .find(&|n| n.interaction.dismiss_layer.is_some())
            .is_none());
    }
}
