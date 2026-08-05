//! ContextMenu — the right-click panel. Same item surface as [`crate::menu`]
//! with per-size metrics and muted checks; positioning is the host's.
//!
//! Contract: `docs/contracts/components/context-menu.md`
//! Ported from: `packages/jetstream/components/src/context_menu.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutSizing, Node, NodeRole,
    NodeToggled, StylePatch,
};
use poodle_specs::{ContextMenuSpec, ControlSize, MenuItemKind};

use crate::color::with_alpha;
use crate::presentation::{rem_to_px, resolve_semantic_size};

/// Per-size item metrics (min-height, padding-y, padding-x, font-size) in rem.
fn item_metrics_rem(size: ControlSize) -> (f32, f32, f32, f32) {
    match size {
        ControlSize::Xs => (1.5, 0.25, 0.375, 0.75),
        ControlSize::Sm => (1.75, 0.3125, 0.4375, 0.8125),
        ControlSize::Md => (2.0, 0.375, 0.5, 0.875),
        ControlSize::Lg => (2.25, 0.4375, 0.5625, 0.9375),
        ControlSize::Xl => (2.5, 0.5, 0.625, 1.0),
    }
}

pub fn context_menu(
    spec: &ContextMenuSpec,
    theme: &dyn ThemeProvider,
    on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let (item_min_h_rem, item_py_rem, item_px_rem, font_rem) = item_metrics_rem(effective_size);

    let item_min_h = rem_to_px(item_min_h_rem);
    let item_py = rem_to_px(item_py_rem);
    let item_px = rem_to_px(item_px_rem);
    let font_size = rem_to_px(font_rem);
    let meta_font = rem_to_px(0.6875);
    let item_gap = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.375,
        poodle_specs::ControlDensity::Default => 0.5,
        poodle_specs::ControlDensity::Comfortable => 0.625,
    });
    let overlay_pad = rem_to_px(0.25);
    let min_width = rem_to_px(14.0);
    let separator_my = rem_to_px(0.25);

    let fill = theme.resolve_color(spec.menu.surface_fill_token());
    let border = theme.resolve_color(spec.menu.overlay_border_token());
    let radius = theme.resolve_radius(spec.menu.overlay_radius_token());
    let item_radius = (theme.resolve_radius("radius.control") - rem_to_px(0.125)).max(0.0);
    let text_color = theme.resolve_color(spec.menu.item_text_token());
    let muted_color = theme.resolve_color("color.text.secondary");
    let separator_color = theme.resolve_color(spec.menu.separator_color_token());
    let danger_color = theme.resolve_color("color.status.danger");
    let accent_color = theme.resolve_color(spec.menu.item_highlight_token());
    let disabled_opacity = theme.resolve_opacity(spec.menu.disabled_opacity_token());

    let hover_tint = with_alpha(accent_color, accent_color.3 * 0.16);
    let separator_bg = with_alpha(separator_color, separator_color.3 * 0.72);

    let mut el = Node::container();
    el.a11y.role = Some(NodeRole::Menu);
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
        s.descriptor.layout.spacing.padding.top = overlay_pad;
        s.descriptor.layout.spacing.padding.bottom = overlay_pad;
        s.descriptor.layout.spacing.padding.left = overlay_pad;
        s.descriptor.layout.spacing.padding.right = overlay_pad;
        s.min_width = Some(min_width);
        s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
        s.overlay = true;
    }

    let build_item = |entry: &poodle_specs::MenuEntry, leading: Option<Node>| -> Node {
        let label_color = if entry.is_destructive {
            danger_color
        } else {
            text_color
        };

        let mut item = Node::container();
        {
            let s = &mut item.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = item_gap;
            s.min_height = Some(item_min_h);
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
        label.style.descriptor.text_color = Some(label_color);
        label.style.text_size = Some(font_size);
        label.style.descriptor.layout.width = LayoutSizing::Grow;
        item = item.child(label);

        if matches!(entry.kind, MenuItemKind::Action) && entry.is_checked {
            let mut check = Node::icon("check", font_size);
            check.style.descriptor.text_color = Some(muted_color);
            item = item.child(check);
        } else if let Some(ref shortcut) = entry.shortcut_label {
            let mut meta = Node::text(shortcut);
            meta.style.descriptor.text_color = Some(muted_color);
            meta.style.text_size = Some(meta_font);
            meta.style.font_family = Some(FontFamily::Mono);
            item = item.child(meta);
        }

        if entry.is_disabled {
            item.style.descriptor.opacity = disabled_opacity;
        } else {
            item.style.descriptor.cursor = CursorHint::Pointer;
            item.style.hover = Some(StylePatch {
                background: Some(hover_tint),
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

    for entry in &spec.menu.items {
        match entry.kind {
            MenuItemKind::Separator => {
                let mut sep = Node::container();
                {
                    let s = &mut sep.style;
                    // Explicit Row (see switch.rs).
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.height = LayoutSizing::Fixed(1.0);
                    s.descriptor.background = Some(separator_bg);
                    s.descriptor.layout.spacing.margin.top = separator_my;
                    s.descriptor.layout.spacing.margin.bottom = separator_my;
                }
                sep.a11y.role = Some(NodeRole::Splitter);
                el = el.child(sep);
            }
            MenuItemKind::Checkbox | MenuItemKind::Radio => {
                let leading = if entry.is_checked {
                    let mut c = Node::icon("check", font_size);
                    c.style.descriptor.text_color = Some(muted_color);
                    c
                } else {
                    let mut s = Node::container();
                    // Explicit Row (see switch.rs).
                    s.style.descriptor.layout.direction = LayoutDirection::Row;
                    s.style.descriptor.layout.width = LayoutSizing::Fixed(font_size);
                    s.style.descriptor.layout.height = LayoutSizing::Fixed(font_size);
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

    el
}
