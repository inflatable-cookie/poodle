//! TabStrip — closable/reorderable tab row (horizontal or vertical).
//!
//! Contract: `docs/contracts/components/tab-strip.md`
//! Ported from: `packages/jetstream/components/src/tab_strip.rs`.
//!
//! Keyboard nav + selection commit are host-owned; tabs are focusable.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodeRole};
use poodle_specs::{Orientation, SemanticControlSizeRole, TabStripSpec};

use crate::color::with_alpha;
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
    size_padding_x_offset_rem,
};

/// Host callbacks: select (tab value) and close (tab value).
#[derive(Default)]
pub struct TabStripHandlers {
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

/// Build the per-item close button (contract §2 CloseButton).
///
/// 1.25rem square, icon-only `x`, `text-secondary` color, radius
/// `radius-control − 0.125rem`.
fn build_close_button(
    spec: &TabStripSpec,
    theme: &dyn ThemeProvider,
    font_size: f32,
    on_press: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let icon_color = theme.resolve_color("color.text.secondary");
    let box_sz = rem_to_px(spec.close_button_size_rem());
    let radius = (theme.resolve_radius("radius.control")
        - rem_to_px(spec.close_button_radius_inset_rem()))
    .max(0.0);

    let mut btn = Node::button("");
    {
        let s = &mut btn.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(box_sz);
        s.descriptor.layout.height = LayoutSizing::Fixed(box_sz);
        s.descriptor.layout.spacing.margin.left =
            theme.resolve_space(spec.close_button_gap_token());
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.cursor = CursorHint::Pointer;
    }
    btn.interaction.focusable = true;
    if let Some(handler) = on_press {
        btn.interaction.on_activate = Some(Arc::new(move || handler()));
    }
    let mut x = Node::icon("x", font_size);
    x.style.descriptor.text_color = Some(icon_color);
    btn.child(x)
}

pub fn tab_strip(
    spec: &TabStripSpec,
    theme: &dyn ThemeProvider,
    handlers: TabStripHandlers,
) -> Node {
    // ── Size / density resolution (contract §6 + size/density axes) ──────
    let effective_size = resolve_semantic_size(spec.size, SemanticControlSizeRole::Control);
    let font_size = rem_to_px(size_font_rem(effective_size));
    // Tab inline padding = density control-x + per-size offset (mirrors Button).
    let pad_x =
        rem_to_px(control_space_x_rem(spec.density) + size_padding_x_offset_rem(effective_size));
    // Tab min-height tracks control-height − 0.25rem (same as Tabs underline).
    let min_h = rem_to_px(control_height_rem(effective_size) - 0.25);
    let item_gap = theme.resolve_space(spec.item_gap_token());
    let close_gap = theme.resolve_space(spec.close_button_gap_token());

    // ── Colors ───────────────────────────────────────────────────────────
    let accent = theme.resolve_color("color.accent.base");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border = theme.resolve_color("color.border.subtle");
    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    // Vertical active-tab fill: accent tinted by the named multiplier.
    let vertical_active_bg = with_alpha(accent, accent.3 * spec.vertical_active_fill_opacity());

    let is_vertical = spec.orientation == Orientation::Vertical;
    // Selection uses the spec fallback (value → default_value → first enabled).
    let selected = spec.current_value().map(|s| s.to_string());

    // ── Strip container ──────────────────────────────────────────────────
    let mut strip = Node::container();
    {
        let s = &mut strip.style;
        s.descriptor.layout.spacing.gap = item_gap;
        if is_vertical {
            s.descriptor.layout.direction = LayoutDirection::Column;
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.border_bottom_width = Some(1.0);
            s.descriptor.border.color = border;
        }
    }

    for item in &spec.items {
        let is_active = selected.as_deref() == Some(item.value.as_str());
        let is_disabled = item.is_disabled;
        let text_color = if is_active {
            accent
        } else if is_disabled {
            text_secondary
        } else {
            text_primary
        };

        // Inner row: label (+ optional close button), visible in both axes.
        let mut tab = Node::button("");
        {
            let s = &mut tab.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = close_gap;
            s.min_height = Some(min_h);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pad_x;
            pad.right = pad_x;
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.cursor = CursorHint::Pointer;
        }
        tab.interaction.focusable = true;
        if !is_disabled {
            if let Some(handler) = &handlers.on_select {
                let handler = Arc::clone(handler);
                let value = item.value.clone();
                tab.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }
        }

        let mut label = Node::text(&item.label);
        label.style.text_size = Some(font_size);
        label.style.descriptor.text_color = Some(text_color);
        let mut tab = tab.child(label);

        // Active indicator: bottom accent border (horizontal) / accent fill
        // (vertical).
        if is_active {
            if is_vertical {
                tab.style.descriptor.background = Some(vertical_active_bg);
            } else {
                tab.style.border_bottom_width = Some(1.0);
                tab.style.descriptor.border.color = accent;
            }
        }

        if item.is_closable {
            let on_close = handlers.on_close.as_ref().map(|handler| {
                let handler = Arc::clone(handler);
                let value = item.value.clone();
                Arc::new(move || handler(&value)) as Arc<dyn Fn() + Send + Sync>
            });
            tab = tab.child(build_close_button(spec, theme, font_size, on_close));
        }

        if is_disabled {
            tab.style.descriptor.opacity = disabled_opacity;
        }

        strip = strip.child(tab);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            strip.a11y.label = Some(label.to_string());
        }
    }
    strip.a11y.role = Some(NodeRole::TabList);
    strip
}
