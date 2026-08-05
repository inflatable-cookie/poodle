//! ToggleGroup — a row of options, single- or multi-select.
//!
//! Contract: `docs/contracts/components/toggle-group.md`
//! Ported from: `packages/jetstream/components/src/toggle_group.rs`.
//!
//! `on_change` fires with the value of the option that was activated — the
//! option, not the resulting selection: in multi-select the host owns the
//! set.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, MainAxisAlignment, Node, NodeRole,
    NodeToggled, StylePatch,
};
use poodle_specs::ToggleGroupSpec;

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    toggle_group_gap_rem,
};

pub fn toggle_group(
    spec: &ToggleGroupSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let accent = theme.resolve_color("color.accent.base");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let border_default = theme.resolve_color("color.border.default");
    let text_primary = theme.resolve_color("color.text.primary");
    let surface = theme.resolve_color("color.background.surface");
    let elevated = theme.resolve_color("color.background.elevated");
    let radius = theme.resolve_radius("radius.control");

    // Contract §8 Root: gap is density-driven.
    let gap = rem_to_px(toggle_group_gap_rem(spec.density));

    // Contract: min-height = calc(control-height − 0.25rem).
    let item_height = rem_to_px(control_height_rem(effective_size)) - rem_to_px(0.25);
    // Contract §8 Item: padding `0 toggle-group-x` (density-driven).
    let item_pad_x = rem_to_px(control_space_x_rem(spec.density));

    // Contract: item border-color = color-mix(border-subtle 82%, transparent).
    let item_border_color = with_alpha(border_subtle, border_subtle.3 * 0.82);

    // Contract: item background = color-mix(surface 93%, text-primary).
    let item_fill = mix_srgb(surface, text_primary, 0.93);

    // Contract: selected = accent tinted at 22% over the item fill.
    let selected_fill = mix_srgb(accent, item_fill, 0.22);

    // Contract: selected border = color-mix(accent-base 42%, border-default).
    let selected_border = mix_srgb(accent, border_default, 0.42);

    // Contract §8 Item: font-size = typography-label-size (flat across
    // sizes), font-weight 600, border 0.0625rem.
    let font_size = theme.resolve_space("typography.label.size");
    let border_width = rem_to_px(0.0625);

    // ── Root container ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }

    // ── Build items ──
    let is_single = matches!(
        spec.selection_mode,
        poodle_specs::ToggleGroupSelectionMode::Single
    );
    for option in &spec.options {
        let is_selected = spec.is_selected(&option.value);
        let is_item_disabled = spec.is_disabled || option.is_disabled;

        let (bg, bc) = if is_selected {
            (selected_fill, selected_border)
        } else {
            (item_fill, item_border_color)
        };

        let mut item = Node::button(&option.label);
        // Contract: selection mode decides. Single-select options are
        // `radio`s; multi-select options stay buttons that toggle.
        item.a11y.role = Some(if is_single {
            NodeRole::RadioButton
        } else {
            NodeRole::Button
        });
        item.a11y.toggled = Some(if is_selected {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        // Hit-test id so a host can route option activation.
        item.id = Some(format!("toggle:{}", option.value));
        {
            let s = &mut item.style;
            s.min_height = Some(item_height);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = item_pad_x;
            pad.right = item_pad_x;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
            s.descriptor.background = Some(bg);
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = bc;
            s.descriptor.text_color = Some(text_primary);
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }
        item.interaction.focusable = true;

        if !is_item_disabled {
            let hover_fill = mix_srgb(bg, elevated, 0.84);
            item.style.hover = Some(StylePatch {
                background: Some(hover_fill),
                border_color: None,
                text_color: None,
                opacity: None,
            });
            item.style.descriptor.cursor = CursorHint::Pointer;

            if let Some(handler) = &on_change {
                let handler = Arc::clone(handler);
                let value = option.value.clone();
                item.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }
        } else {
            item.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
            item.interaction.disabled = true;
        }

        root = root.child(item);
    }

    // ── Group-level disabled ──
    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    // Contract: the group is a `radiogroup` when selection is single.
    root.a11y.role = Some(if is_single {
        NodeRole::RadioGroup
    } else {
        NodeRole::Group
    });
    root
}
