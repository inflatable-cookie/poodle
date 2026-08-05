//! IconButton — a button whose whole label is its glyph.
//!
//! Contract: `docs/contracts/components/icon-button.md`
//! Ported from: `packages/jetstream/components/src/icon_button.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, ShadowLayer, StylePatch,
};
use poodle_specs::{ButtonTone, ButtonVariant, ControlSize, IconButtonSpec};

use crate::color::{mix_srgb, BLACK, TRANSPARENT};
use crate::presentation::{
    control_height_rem, icon_button_size_delta_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};

/// Build an icon-button node. `on_click` fires unless disabled or loading.
pub fn icon_button(
    spec: &IconButtonSpec,
    theme: &dyn ThemeProvider,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let tone = spec.tone;

    // Square: md control-height ± contract per-size delta.
    let md_height = rem_to_px(control_height_rem(ControlSize::Md));
    let size_px = md_height + rem_to_px(icon_button_size_delta_rem(effective_size));

    // Glyph tracks the supporting-visual size (contract §13).
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));

    let radius = theme.resolve_radius("radius.control");
    let is_pressed = spec.is_pressed.unwrap_or(false);
    let is_unavailable = spec.is_disabled || spec.is_loading;

    let surface = theme.resolve_color("color.background.surface");
    let elevated = theme.resolve_color("color.background.elevated");
    let accent = theme.resolve_color("color.accent.base");
    let danger = theme.resolve_color("color.status.danger");
    let border_default = theme.resolve_color("color.border.default");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_inverse = theme.resolve_color("color.text.inverse");

    let status: Option<ColorValue> = match (spec.variant, tone) {
        (ButtonVariant::Danger, _) => Some(danger),
        (_, ButtonTone::Danger) => Some(danger),
        (_, ButtonTone::Success) => Some(theme.resolve_color("color.status.success")),
        (_, ButtonTone::Warning) => Some(theme.resolve_color("color.status.warning")),
        (_, ButtonTone::Default) => None,
    };

    // ── Variant × tone (contract §8) ──
    let (mut fill, mut border, mut text_color) = match (spec.variant, status) {
        (ButtonVariant::Ghost, None) => (TRANSPARENT, TRANSPARENT, text_primary),
        (ButtonVariant::Ghost, Some(s)) => (TRANSPARENT, TRANSPARENT, s),
        (ButtonVariant::Primary, None) => (accent, mix_srgb(accent, BLACK, 0.84), text_inverse),
        (ButtonVariant::Primary, Some(s)) => (s, mix_srgb(s, BLACK, 0.84), text_inverse),
        (ButtonVariant::Danger, Some(s)) => (s, mix_srgb(s, BLACK, 0.84), text_inverse),
        (ButtonVariant::Danger, None) => (danger, mix_srgb(danger, BLACK, 0.84), text_inverse),
        (ButtonVariant::Secondary, None) => (surface, border_default, text_primary),
        (ButtonVariant::Secondary, Some(s)) => (
            mix_srgb(s, surface, 0.16),
            mix_srgb(s, border_default, 0.46),
            text_primary,
        ),
    };

    // Pressed (non-primary): solid accent treatment.
    if is_pressed && !matches!(spec.variant, ButtonVariant::Primary) {
        fill = accent;
        border = mix_srgb(accent, BLACK, 0.85);
        text_color = text_inverse;
    }

    let hover_fill = mix_srgb(fill, elevated, 0.76);
    let hover_border = mix_srgb(border, text_primary, 0.74);
    let active_fill = mix_srgb(fill, elevated, 0.64);

    let icon_name = spec.icon.as_deref().unwrap_or("help-circle");

    let mut el = Node::button("");
    {
        let s = &mut el.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(size_px);
        s.descriptor.layout.width = LayoutSizing::Fixed(size_px);
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.background = Some(fill);
        s.descriptor.text_color = Some(text_color);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    el.interaction.focusable = true;

    // Shadow: none for ghost or pressed; inset top highlight otherwise.
    if !matches!(spec.variant, ButtonVariant::Ghost) && !is_pressed {
        el.style.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: rem_to_px(0.0625),
            blur: 0.0,
            spread: 0.0,
            color: ColorValue(1.0, 1.0, 1.0, 0.08),
            inset: true,
        }];
    }

    // Glyph or spinner — mutually exclusive.
    let glyph_name = if spec.is_loading { "loader" } else { icon_name };
    let mut glyph = Node::icon(glyph_name, icon_size);
    glyph.style.descriptor.text_color = Some(text_color);
    el = el.child(glyph);

    if is_unavailable {
        el.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        el.interaction.disabled = true;
    } else {
        el.style.descriptor.cursor = CursorHint::Pointer;
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
        if let Some(handler) = on_click {
            el.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}
