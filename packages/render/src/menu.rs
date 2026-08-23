//! Menu — the panel of a dropdown menu: actions, checks, radios, separators.
//!
//! Contract: `docs/contracts/components/menu.md`
//! Ported from: `packages/jetstream/components/src/menu.rs`. Renders the
//! panel only — the trigger belongs to the consumer, and open/close policy is
//! the host's.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole, NodeToggled, StylePatch,
};
use poodle_specs::{ControlDensity, ControlSize, MenuItemKind, MenuSpec};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::{control_height_rem, rem_to_px};

pub fn menu(
    spec: &MenuSpec,
    ctx: &RenderContext<'_>,
    on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(match effective_size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    });
    let item_min_height = rem_to_px(control_height_rem(effective_size));
    let meta_font_size = ctx.theme().resolve_space("typography.caption.size");

    let item_px = rem_to_px(match density {
        ControlDensity::Compact => 0.375,
        ControlDensity::Default | ControlDensity::Comfortable => 0.75,
    });
    let item_py = ctx.theme().resolve_space("space.control.y");
    let menu_py = rem_to_px(0.25);
    let item_gap = ctx.theme().resolve_space("space.inline.sm");
    let separator_my = rem_to_px(0.25);
    let item_radius = (ctx.theme()
        .resolve_radius(spec.overlay_radius_token())
        .min(ctx.theme().resolve_radius("radius.control"))
        - rem_to_px(0.125))
    .max(0.0);

    let elevated = ctx.theme().resolve_color(spec.surface_fill_token());
    let panel = ctx.theme().resolve_color("color.background.panel");
    let fill = mix_srgb(elevated, panel, 0.98);
    let border_base = ctx.theme().resolve_color(spec.overlay_border_token());
    let border = with_alpha(border_base, border_base.3 * 0.72);
    let radius = ctx.theme().resolve_radius(spec.overlay_radius_token());
    let text_color = ctx.theme().resolve_color(spec.item_text_token());
    let muted_color = ctx.theme().resolve_color("color.text.secondary");
    let separator_base = ctx.theme().resolve_color(spec.separator_color_token());
    let danger_color = ctx.theme().resolve_color("color.status.danger");
    let accent_color = ctx.theme().resolve_color(spec.item_highlight_token());
    let disabled_opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());

    let hover_tint = with_alpha(accent_color, accent_color.3 * 0.16);
    let danger_hover_tint = with_alpha(danger_color, danger_color.3 * 0.14);
    let separator_color = with_alpha(separator_base, separator_base.3 * 0.72);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.padding.top = menu_py;
        s.descriptor.layout.spacing.padding.bottom = menu_py;
        s.descriptor.layout.spacing.padding.left = menu_py;
        s.descriptor.layout.spacing.padding.right = menu_py;
        s.min_width = Some(ctx.theme().resolve_space("size.menu.minWidth"));
        // Token-accurate elevation.overlay, same mapping as select's panel.
        s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
        s.overlay = true;
    }

    // One item body shared by all three interactive kinds.
    let build_item = |entry: &poodle_specs::MenuEntry, leading: Option<Node>| -> Node {
        let label_color = if entry.is_destructive {
            danger_color
        } else {
            text_color
        };

        let mut item = Node::container();
        item.id = Some(format!("menu-item:{}", entry.value));
        {
            let s = &mut item.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
            s.descriptor.layout.spacing.gap = item_gap;
            s.min_height = Some(item_min_height);
            s.descriptor.layout.spacing.padding.left = item_px;
            s.descriptor.layout.spacing.padding.right = item_px;
            s.descriptor.layout.spacing.padding.top = item_py;
            s.descriptor.layout.spacing.padding.bottom = item_py;
            s.descriptor.corner_radii.top_left = item_radius;
            s.descriptor.corner_radii.top_right = item_radius;
            s.descriptor.corner_radii.bottom_right = item_radius;
            s.descriptor.corner_radii.bottom_left = item_radius;
        }
        item.interaction.focusable = true;

        if let Some(lead) = leading {
            item = item.child(lead);
        }

        let mut label = Node::text(&entry.label);
        label.style.descriptor.text_color = Some(if entry.is_disabled {
            muted_color
        } else {
            label_color
        });
        label.style.text_size = Some(font_size);
        label.style.descriptor.layout.width = LayoutSizing::Grow;
        item = item.child(label);

        // Trailing: a check (action kind only) or the mono shortcut hint.
        if matches!(entry.kind, MenuItemKind::Action) && entry.is_checked {
            let mut check = Node::icon("check", font_size);
            check.style.descriptor.text_color = Some(accent_color);
            item = item.child(check);
        } else if let Some(ref shortcut) = entry.shortcut_label {
            let mut meta = Node::text(shortcut);
            meta.style.descriptor.text_color = Some(muted_color);
            meta.style.text_size = Some(meta_font_size);
            item = item.child(meta);
        }

        if entry.is_disabled {
            item.style.descriptor.opacity = disabled_opacity;
        } else {
            let hover = if entry.is_destructive {
                danger_hover_tint
            } else {
                hover_tint
            };
            item.style.descriptor.cursor = CursorHint::Pointer;
            item.style.hover = Some(StylePatch {
                background: Some(hover),
                border_color: None,
                text_color: None,
                opacity: None,
            });
            if let Some(handler) = &on_action {
                let handler = Arc::clone(handler);
                let value = entry.value.clone();
                item.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }
        }
        item
    };

    for entry in &spec.items {
        match entry.kind {
            MenuItemKind::Separator => {
                let mut sep = Node::container();
                {
                    let s = &mut sep.style;
                    // Explicit Row (see switch.rs).
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.0625));
                    s.descriptor.background = Some(separator_color);
                    s.descriptor.layout.spacing.margin.top = separator_my;
                    s.descriptor.layout.spacing.margin.bottom = separator_my;
                }
                sep.a11y.role = Some(NodeRole::Splitter);
                el = el.child(sep);
            }
            MenuItemKind::Checkbox | MenuItemKind::Radio => {
                // Leading check or a blank spacer keeping labels aligned.
                let check_size = font_size;
                let leading = if entry.is_checked {
                    let mut c = Node::icon("check", check_size);
                    c.style.descriptor.text_color = Some(accent_color);
                    c
                } else {
                    let mut s = Node::container();
                    // Explicit Row (see switch.rs).
                    s.style.descriptor.layout.direction = LayoutDirection::Row;
                    s.style.descriptor.layout.width = LayoutSizing::Fixed(check_size);
                    s.style.descriptor.layout.height = LayoutSizing::Fixed(check_size);
                    s
                };
                let mut item = build_item(entry, Some(leading));
                item.a11y.role = Some(match entry.kind {
                    MenuItemKind::Radio => NodeRole::MenuItemRadio,
                    _ => NodeRole::MenuItemCheckBox,
                });
                item.a11y.toggled = Some(if entry.is_checked {
                    NodeToggled::True
                } else {
                    NodeToggled::False
                });
                el = el.child(item);
            }
            MenuItemKind::Action => {
                let mut item = build_item(entry, None);
                item.a11y.role = Some(NodeRole::MenuItem);
                el = el.child(item);
            }
        }
    }

    // Contract `dismissOnOutsideInteract` (default `true`): a *refusal* flag —
    // native overlays dismiss on outside interact by default, and `false` tells
    // the host not to. The node vocabulary has no outside-interact channel for
    // non-modal overlays, so the refusal rides the surface's interaction as an
    // inert activation: a host implementing outside-dismissal must not dismiss
    // a menu surface carrying this marker (the node-tree form of the web
    // layer's `dismissOnOutsideInteract: false`).
    if !spec.dismiss_on_outside_interact {
        el.interaction.on_activate = Some(Arc::new(|| {}));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.role = Some(NodeRole::Menu);
    el
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn outside_interact_refusal_marks_the_menu_surface() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        // Web default `true`: the surface carries no refusal marker.
        let node = menu(&MenuSpec::default(), &ctx, None);
        assert!(node.interaction.on_activate.is_none());

        // Refusal (`dismissOnOutsideInteract: false`): the surface carries
        // the inert activation marker a host keys outside-dismissal on.
        let refusing = MenuSpec::default().with_dismiss_on_outside_interact(false);
        let node = menu(&refusing, &ctx, None);
        assert!(node.interaction.on_activate.is_some());
    }
}
