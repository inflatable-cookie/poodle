//! Menubar — top-level triggers with a flow-placed dropdown for the open menu.
//!
//! Contract: `docs/contracts/components/menubar.md`
//! Ported from: `packages/jetstream/components/src/menubar.rs`. The open
//! overlay renders in flow below the trigger row, matching both old native
//! tiers' accepted delta from the web's absolute placement.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutSizing, Node, NodeRole,
    StylePatch,
};
use poodle_specs::{MenuItemKind, MenubarSpec};

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};

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
    let item_font = theme.resolve_space("typography.body.size");
    let meta_font = rem_to_px(0.6875);
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let item_pad_y = theme.resolve_space("space.control.y");

    let control_height = theme.resolve_space("size.control.height");
    let control_radius = theme.resolve_radius("radius.control");
    let list_radius = theme.resolve_radius(spec.list_radius_token());
    let border_w = rem_to_px(0.0625);
    let list_gap = rem_to_px(0.125);
    let list_pad = rem_to_px(0.1875);
    let overlay_min_w = rem_to_px(12.0);
    let overlay_pad = rem_to_px(0.25);
    let item_radius = (control_radius - rem_to_px(0.125)).max(0.0);
    let separator_h = rem_to_px(0.0625);
    let separator_my = rem_to_px(0.25);
    let item_gap = rem_to_px(0.5);

    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");
    let panel = theme.resolve_color("color.background.panel");
    let elevated = theme.resolve_color("color.background.elevated");
    let border_subtle = theme.resolve_color(spec.list_border_token());
    let border_default = theme.resolve_color("color.border.default");

    let list_border = with_alpha(border_subtle, border_subtle.3 * 0.72);
    let list_bg = with_alpha(panel, panel.3 * 0.96);
    let open_bg = with_alpha(accent, accent.3 * 0.14);
    let overlay_bg = mix_srgb(elevated, panel, 0.98);
    let overlay_border = with_alpha(border_default, border_default.3 * 0.72);
    let item_hover = with_alpha(accent, accent.3 * 0.16);
    let separator_color = with_alpha(border_subtle, border_subtle.3 * 0.72);

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
                });
            }
        }
        rounded_all(&mut btn, control_radius);
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
    if let Some(menu) = spec.current_menu() {
        if !menu.items.is_empty() {
            let mut overlay = Node::container();
            overlay.a11y.role = Some(NodeRole::Menu);
            {
                let s = &mut overlay.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.min_width = Some(overlay_min_w);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.top = overlay_pad;
                pad.bottom = overlay_pad;
                pad.left = overlay_pad;
                pad.right = overlay_pad;
                s.descriptor.border.width = border_w;
                s.descriptor.border.color = overlay_border;
                s.descriptor.background = Some(overlay_bg);
                s.descriptor.layout.spacing.margin.top = overlay_pad;
                s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
                s.overlay = true;
            }
            rounded_all(&mut overlay, list_radius);

            for item in &menu.items {
                match item.kind {
                    MenuItemKind::Separator => {
                        let mut sep = Node::container();
                        {
                            let s = &mut sep.style;
                            // Explicit Row (see switch.rs).
                            s.descriptor.layout.direction = LayoutDirection::Row;
                            s.descriptor.layout.height = LayoutSizing::Fixed(separator_h);
                            s.descriptor.background = Some(separator_color);
                            s.descriptor.layout.spacing.margin.top = separator_my;
                            s.descriptor.layout.spacing.margin.bottom = separator_my;
                        }
                        sep.a11y.role = Some(NodeRole::Splitter);
                        overlay = overlay.child(sep);
                    }
                    _ => {
                        let mut row = Node::container();
                        row.a11y.role = Some(match item.kind {
                            MenuItemKind::Checkbox => NodeRole::MenuItemCheckBox,
                            MenuItemKind::Radio => NodeRole::MenuItemRadio,
                            _ => NodeRole::MenuItem,
                        });
                        {
                            let s = &mut row.style;
                            s.descriptor.layout.direction = LayoutDirection::Row;
                            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                            s.descriptor.layout.spacing.gap = item_gap;
                            s.min_height = Some(control_height);
                            let pad = &mut s.descriptor.layout.spacing.padding;
                            pad.top = item_pad_y;
                            pad.bottom = item_pad_y;
                            pad.left = pad_x;
                            pad.right = pad_x;
                            if item.is_disabled {
                                s.descriptor.opacity = disabled_opacity;
                            } else {
                                s.descriptor.cursor = CursorHint::Pointer;
                                s.hover = Some(StylePatch {
                                    background: Some(item_hover),
                                    border_color: None,
                                    text_color: None,
                                });
                            }
                        }
                        rounded_all(&mut row, item_radius);
                        row.interaction.focusable = true;

                        let mut label = Node::text(&item.label);
                        label.style.descriptor.text_color = Some(text_primary);
                        label.style.text_size = Some(item_font);
                        label.style.descriptor.layout.width = LayoutSizing::Grow;
                        let mut row = row.child(label);

                        if item.is_checked {
                            let mut check = Node::text("✓");
                            check.style.descriptor.text_color = Some(text_secondary);
                            check.style.text_size = Some(meta_font);
                            row = row.child(check);
                        } else if let Some(ref shortcut) = item.shortcut_label {
                            let mut meta = Node::text(shortcut);
                            meta.style.descriptor.text_color = Some(text_secondary);
                            meta.style.text_size = Some(meta_font);
                            meta.style.font_family = Some(FontFamily::Mono);
                            row = row.child(meta);
                        }

                        if !item.is_disabled {
                            if let Some(handler) = &on_select {
                                let handler = Arc::clone(handler);
                                let value = item.value.clone();
                                row.interaction.on_activate =
                                    Some(Arc::new(move || handler(&value)));
                            }
                        }
                        overlay = overlay.child(row);
                    }
                }
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
