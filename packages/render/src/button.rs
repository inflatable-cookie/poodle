//! Button — the primary interactive control.
//!
//! Contract: `docs/contracts/components/button.md`
//! Ported from: `packages/jetstream/components/src/button.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, ShadowLayer, StylePatch, TextAlign,
};
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant};

use crate::color::{mix_srgb, TRANSPARENT};
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem, size_min_width_rem, size_padding_x_offset_rem,
};

/// Build a button node. `on_click` fires unless disabled or loading — the
/// contract dims a loading button, drops the cursor and removes it from the
/// tab order, so it must not fire either.
pub fn button(
    spec: &ButtonSpec,
    theme: &dyn ThemeProvider,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let tone = spec.effective_tone();

    let status_token = match tone {
        ButtonTone::Danger => Some("color.status.danger"),
        ButtonTone::Success => Some("color.status.success"),
        ButtonTone::Warning => Some("color.status.warning"),
        ButtonTone::Default => None,
    };

    // ── Variant × tone colours (contract) ──
    let fill: ColorValue = match (spec.variant, status_token) {
        (ButtonVariant::Ghost, _) => TRANSPARENT,
        (ButtonVariant::Secondary, Some(status)) => {
            // Danger/Success secondary: color-mix(status 16%, background-surface)
            let status_color = theme.resolve_color(status);
            let surface = theme.resolve_color("color.background.surface");
            mix_srgb(status_color, surface, 0.16)
        }
        _ => theme.resolve_color(spec.resolved_fill_token()),
    };

    let text_color = theme.resolve_color(spec.resolved_text_token());

    let border_color: ColorValue = match (spec.variant, status_token) {
        (ButtonVariant::Ghost, _) => TRANSPARENT,
        (ButtonVariant::Secondary, Some(status)) => {
            let status_color = theme.resolve_color(status);
            let border_default = theme.resolve_color("color.border.default");
            mix_srgb(status_color, border_default, 0.46)
        }
        _ => theme.resolve_color(spec.resolved_border_token()),
    };

    // Hover/active (contract: mix fill with elevated)
    let elevated = theme.resolve_color("color.background.elevated");
    let hover_fill = mix_srgb(fill, elevated, 0.84);
    let active_fill = mix_srgb(fill, elevated, 0.72);
    let text_primary = theme.resolve_color("color.text.primary");
    let hover_border = mix_srgb(border_color, text_primary, 0.78);

    // ── Sizing ──
    let height = rem_to_px(control_height_rem(effective_size));
    let min_width = rem_to_px(size_min_width_rem(effective_size));
    let base_pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_x = base_pad_x + rem_to_px(size_padding_x_offset_rem(effective_size));

    // Icon-side padding reduction (contract §8).
    let icon_inset = theme.resolve_space(ButtonSpec::icon_side_inset_token());
    let has_leading = spec.leading_icon.is_some() || spec.is_loading;
    let has_trailing = spec.trailing_icon.is_some() || spec.chevron;
    let pad_left = if has_leading { pad_x - icon_inset } else { pad_x };
    let pad_right = if has_trailing { pad_x - icon_inset } else { pad_x };

    let radius = theme.resolve_radius(spec.radius_token());
    let gap = theme.resolve_space(ButtonSpec::content_gap_token());
    let label_size = rem_to_px(size_font_rem(effective_size));
    let icon_size = rem_to_px(size_font_rem(effective_size));
    let is_disabled = spec.is_disabled || spec.is_loading;

    let has_icons = has_leading || has_trailing;
    let label_text = spec.label.clone().unwrap_or_default();

    // With icons: button root with empty label, children carry the content.
    // Without: the root carries the label directly.
    let button_label = if has_icons {
        String::new()
    } else {
        label_text.clone()
    };

    let mut el = Node::button(button_label);
    {
        let s = &mut el.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.min_width = Some(min_width);
        s.descriptor.layout.spacing.padding.left = pad_left;
        s.descriptor.layout.spacing.padding.right = pad_right;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.background = Some(fill);
        s.descriptor.text_color = Some(text_color);
        s.text_size = Some(label_size);
        s.text_weight = Some(500); // contract: typography-label-weight
        s.letter_spacing_em = Some(0.01); // contract §8
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.text_align = Some(TextAlign::Center);
        // Border — 1px for non-ghost, 1px transparent for ghost (keeps layout).
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border_color;
    }
    el.interaction.focusable = true;

    // Contract §8 shadows: inset top highlight on solid buttons, plus an outset
    // drop on primary. Ghost none.
    if !matches!(spec.variant, ButtonVariant::Ghost) {
        let is_primary = matches!(spec.variant, ButtonVariant::Primary);
        let highlight_alpha = if is_primary { 0.14 } else { 0.08 };
        let mut layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: rem_to_px(0.0625),
            blur: 0.0,
            spread: 0.0,
            color: ColorValue(1.0, 1.0, 1.0, highlight_alpha),
            inset: true,
        }];
        if is_primary {
            layers.push(ShadowLayer {
                offset_x: 0.0,
                offset_y: rem_to_px(0.375),
                blur: rem_to_px(1.125),
                spread: 0.0,
                color: ColorValue(0.0, 0.0, 0.0, 0.18),
                inset: false,
            });
        }
        el.style.shadow_layers = layers;
    }

    if !is_disabled {
        el.style.hover = Some(StylePatch {
            background: Some(hover_fill),
            border_color: Some(hover_border),
            text_color: None,
            opacity: None,
        });
        el.style.active = Some(StylePatch {
            background: Some(active_fill),
            border_color: None,
            text_color: None,
            opacity: None,
        });
        el.style.descriptor.cursor = CursorHint::Pointer;
        if let Some(handler) = on_click {
            el.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    if is_disabled {
        el.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        el.interaction.disabled = true;
    }

    // ── Children (only when icons/spinner present) ──
    if has_icons {
        if spec.is_loading {
            let mut spinner = Node::icon("loader", icon_size);
            spinner.style.descriptor.text_color = Some(text_color);
            el = el.child(spinner);
        }
        if let Some(ref icon_name) = spec.leading_icon {
            let mut icon = Node::icon(icon_name.as_str(), icon_size);
            icon.style.descriptor.text_color = Some(text_color);
            el = el.child(icon);
        }
        if !label_text.is_empty() {
            let mut label = Node::text(&label_text);
            label.style.text_size = Some(label_size);
            label.style.descriptor.text_color = Some(text_color);
            label.style.letter_spacing_em = Some(0.01);
            el = el.child(label);
        }
        if let Some(ref icon_name) = spec.trailing_icon {
            let mut icon = Node::icon(icon_name.as_str(), icon_size);
            icon.style.descriptor.text_color = Some(text_color);
            el = el.child(icon);
        }
        if spec.chevron {
            let mut chevron = Node::icon("chevron-down", icon_size);
            chevron.style.descriptor.text_color = Some(text_color);
            chevron.style.descriptor.opacity = 0.5;
            el = el.child(chevron);
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}
