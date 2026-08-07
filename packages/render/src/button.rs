//! Button — the primary interactive control.
//!
//! Contract: `docs/contracts/components/button.md`
//! Ported from: `packages/jetstream/components/src/button.rs`; metrics and
//! state recipes re-transcribed from
//! `packages/gpui/components/src/primitives/button.rs` (the axis-faithful
//! tier, matching Svelte): axis-layered tokens plus per-size/per-density
//! offsets instead of the fixed per-size/per-density tables, the full
//! variant × tone state recipes, and the pressed/toggle treatment. At base
//! tokens (the Jetstream provider, no axes) md/default reproduces the old
//! fixed values; under a preview axis the button now follows the axis.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeAnimation, StylePatch, TextAlign,
};
use poodle_specs::{
    ButtonSpec, ButtonTone, ButtonVariant, ControlDensity, SpinnerSize, SpinnerSpec,
};

use crate::color::{mix_srgb, with_alpha, BLACK, TRANSPARENT, WHITE};
use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem, size_min_width_rem,
    size_padding_x_offset_rem,
};

/// The old tier's `color_mix_black`: scales RGB toward black at `ratio` while
/// preserving alpha (`mix_srgb(c, BLACK, r)` would lerp alpha toward opaque).
fn mix_black(c: ColorValue, ratio: f32) -> ColorValue {
    ColorValue(c.0 * ratio, c.1 * ratio, c.2 * ratio, c.3)
}

/// Build a button node. `on_click` fires unless disabled or loading — the
/// contract dims a loading button, drops the cursor and removes it from the
/// tab order, so it must not fire either.
pub fn button(
    spec: &ButtonSpec,
    theme: &dyn ThemeProvider,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Base tokens ──
    let base_fill = theme.resolve_color(spec.resolved_fill_token());
    let base_text = theme.resolve_color(spec.resolved_text_token());
    let base_border = theme.resolve_color(spec.resolved_border_token());
    let elevated = theme.resolve_color("color.background.elevated");
    let surface = theme.resolve_color("color.background.surface");
    let border_default = theme.resolve_color("color.border.default");
    let text_primary = theme.resolve_color("color.text.primary");
    let radius = theme.resolve_radius(spec.radius_token());

    // Status tones mirror each other within each variant (contract §8 Tone) —
    // resolve the family colour once and match on its presence. The match is
    // on (variant, tone), NOT effective_tone: the legacy Danger variant falls
    // through to the default arms exactly like the old GPUI tier.
    let status = match spec.tone {
        ButtonTone::Danger => Some(theme.resolve_color("color.status.danger")),
        ButtonTone::Success => Some(theme.resolve_color("color.status.success")),
        ButtonTone::Warning => Some(theme.resolve_color("color.status.warning")),
        ButtonTone::Default => None,
    };

    // ── Axis-faithful metrics (g12.019 recipe correction): the axis-layered
    // token plus the per-size offset — the old GPUI tier's form, matching
    // Svelte's CSS vars — not the fixed tables (`control_height_rem` /
    // `control_space_x_rem`), which ignore the theme's density/control-size
    // layering.
    let height = theme.resolve_space(spec.control_height_token())
        + rem_to_px(size_height_offset_rem(effective_size));
    let min_width = rem_to_px(size_min_width_rem(effective_size));
    // Svelte: compact -0.125rem, comfortable +0.125rem density offset on padding.
    let density_pad_offset = rem_to_px(match spec.density {
        ControlDensity::Compact => -0.125,
        ControlDensity::Default => 0.0,
        ControlDensity::Comfortable => 0.125,
    });
    let pad_x = theme.resolve_space(spec.horizontal_padding_token())
        + rem_to_px(size_padding_x_offset_rem(effective_size))
        + density_pad_offset;
    let label_size = rem_to_px(size_font_rem(effective_size));
    // The content gap ladders on density, not size.
    let gap = match spec.density {
        ControlDensity::Compact => theme.resolve_space("space.inline.xs"),
        ControlDensity::Default => theme.resolve_space(ButtonSpec::content_gap_token()),
        ControlDensity::Comfortable => theme.resolve_space("space.inline.md"),
    };
    // Icons are always the sm icon token (the old GPUI tier's `IconSize::Sm`),
    // not a per-control-size ladder stop.
    let icon_size = theme.resolve_space(spec.icon_size_token());
    // The loading spinner is the old tier's `SpinnerSize::Sm` — a fixed
    // ladder stop from the spec (12px), not a theme token.
    let spinner_size = SpinnerSpec::new().with_size(SpinnerSize::Sm).size_px();

    // Icon padding adjustment (contract §8): reduce padding on icon side by 0.125rem.
    let icon_inset = theme.resolve_space(ButtonSpec::icon_side_inset_token());
    let has_leading = spec.leading_icon.is_some() || spec.is_loading;
    let has_trailing = spec.trailing_icon.is_some() || spec.chevron;
    let pad_left = if has_leading {
        pad_x - icon_inset
    } else {
        pad_x
    };
    let pad_right = if has_trailing {
        pad_x - icon_inset
    } else {
        pad_x
    };

    let is_disabled = spec.is_disabled || spec.is_loading;
    let is_ghost = matches!(spec.variant, ButtonVariant::Ghost);

    // ── Variant × tone colours (contract §8) ──
    let (fill, border_color, text_color) = match (spec.variant, status) {
        (ButtonVariant::Secondary, Some(status_color)) => {
            // Status-tinted secondary: color-mix(status 16%, surface); the
            // idle border stays plain border-default.
            (
                mix_srgb(status_color, surface, 0.16),
                border_default,
                base_text,
            )
        }
        (ButtonVariant::Primary, _) => {
            // Border: the fill darkened toward black (treatment-interactive-primary-border).
            (base_fill, mix_black(base_fill, 0.86), base_text)
        }
        (ButtonVariant::Ghost, Some(status_color)) => {
            // Ghost × status: text takes the status colour.
            (base_fill, base_border, status_color)
        }
        _ => (base_fill, base_border, base_text),
    };

    // Ghost: transparent fill and border (contract §8 CSS Custom Properties).
    let (fill, border_color) = if is_ghost {
        (TRANSPARENT, TRANSPARENT)
    } else {
        (fill, border_color)
    };

    // Pressed/toggle state (contract §8 Pressed/toggle state): non-primary
    // variants get accent fill, darkened accent border, inverse text.
    let is_pressed = spec.is_toggle_mode() && spec.current_pressed();
    let (fill, border_color, text_color) =
        if is_pressed && !matches!(spec.variant, ButtonVariant::Primary) {
            let accent = theme.resolve_color("color.accent.base");
            let text_inverse = theme.resolve_color("color.text.inverse");
            (accent, mix_black(accent, 0.86), text_inverse)
        } else {
            (fill, border_color, text_color)
        };

    // ── Hover/active colours (contract §8 Hover/Active) ──
    // Computed after the ghost/pressed overrides: the default arm mixes from
    // the FINAL fill and border, exactly like the old GPUI tier.
    let (hover_fill, active_fill, hover_border) = match (spec.variant, status) {
        (ButtonVariant::Ghost, Some(status_color)) => (
            with_alpha(status_color, 0.12),
            with_alpha(status_color, 0.18),
            with_alpha(status_color, 0.28),
        ),
        (ButtonVariant::Secondary, Some(status_color)) => (
            mix_srgb(status_color, surface, 0.24),
            mix_srgb(status_color, surface, 0.32),
            mix_srgb(status_color, border_default, 0.62),
        ),
        (ButtonVariant::Primary, Some(status_color)) => {
            let hover = mix_srgb(status_color, WHITE, 0.88);
            (
                hover,
                mix_srgb(status_color, BLACK, 0.88),
                mix_black(hover, 0.86),
            )
        }
        _ => {
            let hover = mix_srgb(fill, elevated, 0.84);
            let active = mix_srgb(fill, elevated, 0.72);
            // Ghost's idle border is transparent; its hover border mixes from
            // border-default instead (the old tier's special case).
            let border_base = if is_ghost {
                border_default
            } else {
                border_color
            };
            (hover, active, mix_srgb(border_base, text_primary, 0.78))
        }
    };

    let has_icons = has_leading || has_trailing;
    let label_text = spec.label.clone().unwrap_or_default();

    // Icon-only (contract §8 Icon-only): square — width = height, no
    // min-width, no horizontal padding.
    let is_icon_only = label_text.is_empty() && !spec.chevron;

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
        if is_icon_only {
            s.descriptor.layout.width = LayoutSizing::Fixed(height);
        } else {
            s.min_width = Some(min_width);
            s.descriptor.layout.spacing.padding.left = pad_left;
            s.descriptor.layout.spacing.padding.right = pad_right;
        }
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.background = Some(fill);
        s.descriptor.text_color = Some(text_color);
        s.text_size = Some(label_size);
        s.text_weight = Some(500); // contract: typography-label-weight
        s.line_height = Some(1.0); // old tier: line_height(relative(1.0))
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

    // The old GPUI tier paints no idle shadow. Its hover-only shadow cannot
    // be expressed by StylePatch and does not affect the closed-state gate.

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
        el.style.descriptor.cursor = CursorHint::NotAllowed;
        el.interaction.disabled = true;
    }

    // ── Children (only when icons/spinner present) ──
    if has_icons {
        if spec.is_loading {
            let mut spinner = Node::icon("spinner", spinner_size);
            spinner.style.descriptor.text_color = Some(text_color);
            spinner.style.animation = Some(NodeAnimation::spin("poodle-spinner-ring", 0.8));
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
            label.style.no_wrap = true; // old tier: whitespace_nowrap on the label
            label.style.min_width = Some(0.0);
            el = el.child(label);
        }
        if let Some(ref icon_name) = spec.trailing_icon {
            let mut icon = Node::icon(icon_name.as_str(), icon_size);
            icon.style.descriptor.text_color = Some(text_color);
            el = el.child(icon);
        }
        if spec.chevron {
            // Old tier: the chevron sits in a flex/center wrapper at 0.5
            // opacity with margin-left = -space.inline.sm * 0.25, tightening
            // it against the gap. The wrapper holds only the icon, so the
            // margin and opacity land on the icon node directly.
            let mut chevron = Node::icon("chevron-down", icon_size);
            chevron.style.descriptor.text_color = Some(text_color);
            chevron.style.descriptor.opacity = 0.5;
            chevron.style.descriptor.layout.spacing.margin.left =
                -theme.resolve_space("space.inline.sm") * 0.25;
            el = el.child(chevron);
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{ControlDensity, ControlSize};

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn fixed_height(node: &Node) -> f32 {
        match node.style.descriptor.layout.height {
            LayoutSizing::Fixed(h) => h,
            ref other => panic!("expected fixed height, got {other:?}"),
        }
    }

    fn icon_size_of(node: &Node, name: &str) -> f32 {
        let icon = node
            .find(
                &|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name: n_name, .. } if n_name == name),
            )
            .unwrap_or_else(|| panic!("icon {name} exists"));
        match icon.kind {
            poodle_node::NodeKind::Icon { size, .. } => size,
            _ => unreachable!(),
        }
    }

    #[test]
    fn metrics_follow_the_axis_faithful_recipe() {
        // height = size.control.height token (36px at base) + per-size offset
        let cases = [
            (ControlSize::Xs, 28.0),
            (ControlSize::Sm, 30.0),
            (ControlSize::Md, 36.0),
            (ControlSize::Lg, 42.0),
            (ControlSize::Xl, 44.0),
        ];
        for (size, expected) in cases {
            let spec = ButtonSpec::new().with_size(size);
            let node = button(&spec, &theme(), None);
            assert_eq!(fixed_height(&node), expected, "height for {size:?}");
        }

        // pad_x = space.control.x token + per-size offset + density offset
        let theme = theme();
        let base = theme.resolve_space("space.control.x");
        let cases = [
            // md/default: base + 0 + 0
            ((ControlSize::Md, ControlDensity::Default), base),
            // sm/compact: base - 0.125rem (size) - 0.125rem (density)
            (
                (ControlSize::Sm, ControlDensity::Compact),
                base - rem_to_px(0.125) - rem_to_px(0.125),
            ),
            // lg/comfortable: base + 0.125rem (size) + 0.125rem (density)
            (
                (ControlSize::Lg, ControlDensity::Comfortable),
                base + rem_to_px(0.125) + rem_to_px(0.125),
            ),
        ];
        for ((size, density), expected) in cases {
            // A label keeps the button out of the icon-only (square) recipe.
            let spec = ButtonSpec::new()
                .with_label("Save")
                .with_size(size)
                .with_density(density);
            let node = button(&spec, &theme, None);
            let padding = node.style.descriptor.layout.spacing.padding;
            assert_eq!(padding.left, expected, "pad_x for {size:?}/{density:?}");
            assert_eq!(padding.right, expected, "pad_x for {size:?}/{density:?}");
        }
    }

    #[test]
    fn gap_ladders_on_density() {
        let theme = theme();
        let cases = [
            (
                ControlDensity::Compact,
                theme.resolve_space("space.inline.xs"),
            ),
            (
                ControlDensity::Default,
                theme.resolve_space(ButtonSpec::content_gap_token()),
            ),
            (
                ControlDensity::Comfortable,
                theme.resolve_space("space.inline.md"),
            ),
        ];
        for (density, expected) in cases {
            let spec = ButtonSpec::new().with_density(density);
            let node = button(&spec, &theme, None);
            assert_eq!(
                node.style.descriptor.layout.spacing.gap, expected,
                "gap for {density:?}"
            );
        }
    }

    #[test]
    fn secondary_status_tone_idle_has_tinted_fill_and_plain_border() {
        let theme = theme();
        let danger = theme.resolve_color("color.status.danger");
        let surface = theme.resolve_color("color.background.surface");
        let border_default = theme.resolve_color("color.border.default");
        let text_primary = theme.resolve_color("color.text.primary");

        // Default variant is Secondary.
        let spec = ButtonSpec::new().with_tone(ButtonTone::Danger);
        let node = button(&spec, &theme, None);
        assert_eq!(
            node.style.descriptor.background,
            Some(mix_srgb(danger, surface, 0.16))
        );
        // The idle border is plain border-default — no status mix.
        assert_eq!(node.style.descriptor.border.color, border_default);
        assert_eq!(node.style.descriptor.text_color, Some(text_primary));
    }

    #[test]
    fn primary_border_is_the_darkened_fill() {
        let theme = theme();
        let accent = theme.resolve_color("color.accent.base");
        let spec = ButtonSpec::new().with_variant(ButtonVariant::Primary);
        let node = button(&spec, &theme, None);
        assert_eq!(node.style.descriptor.background, Some(accent));
        assert_eq!(node.style.descriptor.border.color, mix_black(accent, 0.86));
    }

    #[test]
    fn pressed_non_primary_toggle_takes_the_accent_treatment() {
        let theme = theme();
        let accent = theme.resolve_color("color.accent.base");
        let inverse = theme.resolve_color("color.text.inverse");

        let spec = ButtonSpec::new().with_pressed(true);
        let node = button(&spec, &theme, None);
        assert_eq!(node.style.descriptor.background, Some(accent));
        assert_eq!(node.style.descriptor.border.color, mix_black(accent, 0.86));
        assert_eq!(node.style.descriptor.text_color, Some(inverse));

        // Primary keeps its own recipe when pressed.
        let danger = theme.resolve_color("color.status.danger");
        let spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Primary)
            .with_tone(ButtonTone::Danger)
            .with_pressed(true);
        let node = button(&spec, &theme, None);
        assert_eq!(node.style.descriptor.background, Some(danger));
    }

    #[test]
    fn hover_active_recipes_follow_variant_and_tone() {
        let theme = theme();
        let danger = theme.resolve_color("color.status.danger");
        let surface = theme.resolve_color("color.background.surface");
        let elevated = theme.resolve_color("color.background.elevated");
        let border_default = theme.resolve_color("color.border.default");
        let text_primary = theme.resolve_color("color.text.primary");

        // Ghost × status: the status colour at scaled alphas.
        let spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Ghost)
            .with_tone(ButtonTone::Danger);
        let node = button(&spec, &theme, None);
        let hover = node.style.hover.expect("hover patch");
        let active = node.style.active.expect("active patch");
        assert_eq!(hover.background, Some(with_alpha(danger, 0.12)));
        assert_eq!(active.background, Some(with_alpha(danger, 0.18)));
        assert_eq!(hover.border_color, Some(with_alpha(danger, 0.28)));

        // Secondary × status: mixes toward surface; border toward border-default.
        let spec = ButtonSpec::new().with_tone(ButtonTone::Danger);
        let node = button(&spec, &theme, None);
        let hover = node.style.hover.expect("hover patch");
        let active = node.style.active.expect("active patch");
        assert_eq!(hover.background, Some(mix_srgb(danger, surface, 0.24)));
        assert_eq!(active.background, Some(mix_srgb(danger, surface, 0.32)));
        assert_eq!(
            hover.border_color,
            Some(mix_srgb(danger, border_default, 0.62))
        );

        // Primary × status: hover toward white, active toward black, hover
        // border the darkened hover fill.
        let spec = ButtonSpec::new()
            .with_variant(ButtonVariant::Primary)
            .with_tone(ButtonTone::Danger);
        let node = button(&spec, &theme, None);
        let hover = node.style.hover.expect("hover patch");
        let active = node.style.active.expect("active patch");
        let hover_fill = mix_srgb(danger, WHITE, 0.88);
        assert_eq!(hover.background, Some(hover_fill));
        assert_eq!(active.background, Some(mix_srgb(danger, BLACK, 0.88)));
        assert_eq!(hover.border_color, Some(mix_black(hover_fill, 0.86)));

        // Default secondary: fill toward elevated, border toward text-primary.
        let node = button(&ButtonSpec::new(), &theme, None);
        let hover = node.style.hover.expect("hover patch");
        let active = node.style.active.expect("active patch");
        assert_eq!(hover.background, Some(mix_srgb(surface, elevated, 0.84)));
        assert_eq!(active.background, Some(mix_srgb(surface, elevated, 0.72)));
        assert_eq!(
            hover.border_color,
            Some(mix_srgb(border_default, text_primary, 0.78))
        );

        // Ghost default: idle border transparent, hover border mixes from
        // border-default (not from the transparent idle border).
        let spec = ButtonSpec::new().with_variant(ButtonVariant::Ghost);
        let node = button(&spec, &theme, None);
        assert_eq!(node.style.descriptor.border.color, TRANSPARENT);
        let hover = node.style.hover.expect("hover patch");
        assert_eq!(
            hover.border_color,
            Some(mix_srgb(border_default, text_primary, 0.78))
        );
    }

    #[test]
    fn icon_only_button_is_square_with_no_min_width_or_padding() {
        let theme = theme();
        let spec = ButtonSpec::new().with_leading_icon("plus");
        let node = button(&spec, &theme, None);
        let height = fixed_height(&node);
        assert_eq!(
            node.style.descriptor.layout.width,
            LayoutSizing::Fixed(height)
        );
        assert_eq!(node.style.min_width, None);
        let padding = node.style.descriptor.layout.spacing.padding;
        assert_eq!(padding.left, 0.0);
        assert_eq!(padding.right, 0.0);
    }

    #[test]
    fn icons_use_the_sm_icon_token_not_the_font_ladder() {
        let theme = theme();
        let icon_token = theme.resolve_space("size.icon.sm");
        let spec = ButtonSpec::new()
            .with_label("Save")
            .with_leading_icon("check");
        let node = button(&spec, &theme, None);
        assert_eq!(icon_size_of(&node, "check"), icon_token);
        // The font ladder stop for md (13px) would be the old behaviour.
        assert_ne!(icon_token, rem_to_px(size_font_rem(ControlSize::Md)));
    }

    #[test]
    fn chevron_pulls_in_with_a_negative_left_margin() {
        let theme = theme();
        let spec = ButtonSpec::new().with_label("More").with_chevron(true);
        let node = button(&spec, &theme, None);
        let chevron = node
            .find(
                &|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "chevron-down"),
            )
            .expect("chevron icon");
        assert_eq!(chevron.style.descriptor.opacity, 0.5);
        assert_eq!(
            chevron.style.descriptor.layout.spacing.margin.left,
            -theme.resolve_space("space.inline.sm") * 0.25
        );
    }

    #[test]
    fn loading_disables_activation_and_shows_the_sm_spinner() {
        let theme = theme();
        let spec = ButtonSpec::new().with_label("Save").with_loading(true);
        let node = button(
            &spec,
            &theme,
            Some(Arc::new(|| panic!("loading must not fire"))),
        );
        assert!(node.interaction.on_activate.is_none());
        assert!(node.interaction.disabled);
        assert_eq!(node.style.descriptor.cursor, CursorHint::NotAllowed);
        assert!(node.style.shadow_layers.is_empty());
        let spinner = SpinnerSpec::new().with_size(SpinnerSize::Sm).size_px();
        assert_eq!(icon_size_of(&node, "spinner"), spinner);
        assert!(node
            .find(&|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "spinner"))
            .expect("spinner icon")
            .style
            .animation
            .is_some());
    }

    #[test]
    fn clicking_reports_through_the_handler() {
        use std::sync::Mutex;
        let clicks: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let sink = Arc::clone(&clicks);
        let spec = ButtonSpec::new().with_label("Save");
        let node = button(
            &spec,
            &theme(),
            Some(Arc::new(move || *sink.lock().unwrap() += 1)),
        );
        let activate = node.interaction.on_activate.expect("activatable");
        activate();
        assert_eq!(*clicks.lock().unwrap(), 1);
    }
}
