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
    control_height_rem, rem_to_px, resolve_semantic_size, size_font_rem, size_icon_inset_rem,
    size_min_width_rem,
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
    // Absent size follows the presentation default (md at the base tier) —
    // the same resolution the web pair performs for `size = null`.
    let effective_size = resolve_semantic_size(spec.size.unwrap_or_default(), spec.size_role);

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

    // ── Metrics follow the web CSS (the reference): button.css hardcodes
    // the per-size height ladder and keeps padding at the control-x token,
    // with density-only offsets. The old token-plus-offset recipe disagreed
    // with the CSS (lg rendered 42px against the CSS's 44px) — the
    // normalized observation comparison caught it.
    let height = rem_to_px(control_height_rem(effective_size));
    let min_width = rem_to_px(size_min_width_rem(effective_size));
    // Svelte: compact -0.125rem, comfortable +0.125rem density offset on padding.
    let density = spec.density.unwrap_or_default();
    let density_pad_offset = rem_to_px(match density {
        ControlDensity::Compact => -0.125,
        ControlDensity::Default => 0.0,
        ControlDensity::Comfortable => 0.125,
    });
    let pad_x = theme.resolve_space(spec.horizontal_padding_token()) + density_pad_offset;
    let label_size = rem_to_px(size_font_rem(effective_size));
    // The content gap ladders on density, not size.
    let gap = match density {
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

    // Icon padding adjustment (contract §8): per-size reduction on the icon
    // side, matching button.css's `[data-has-leading]`/`[data-has-trailing]`
    // per-size rules.
    let icon_inset = rem_to_px(size_icon_inset_rem(effective_size));
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
            // Border: the primary fill darkened toward black (contract §8:
            // color-mix(accent-base 84%, black)).
            (base_fill, mix_black(base_fill, 0.84), base_text)
        }
        (ButtonVariant::Ghost, Some(status_color)) => {
            // Ghost × status: text takes the status colour.
            (base_fill, base_border, status_color)
        }
        (ButtonVariant::Secondary, None) => {
            // Secondary default: elevation stacking (contract §8) — the
            // surface mixed 88% toward text-primary, not the raw surface
            // token. The g15.047 comparator measured the raw-surface fill
            // ~26/255 off the web reference.
            (
                mix_srgb(surface, text_primary, 0.88),
                base_border,
                base_text,
            )
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
            // Contract §8 pressed: color-mix(accent-base 85%, black) border.
            (accent, mix_black(accent, 0.85), text_inverse)
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
            // Old-tier formula, retained: hover/active mix from the FINAL
            // fill toward elevated. The g15.047 batch captures no hover or
            // active frame, so the contract §8 text-primary stacking for
            // those states stays an unmeasured suspicion, not a repair.
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
        // Focus-visible treatment (contract §8): the accent focus-ring color
        // takes the border while the node holds focus — the native counterpart
        // of the web `:focus-visible` outline. Its presence is also the
        // observation channel for the focus-visible state.
        el.style.focus = Some(StylePatch {
            background: None,
            border_color: Some(theme.resolve_color(spec.focus_ring_color_token())),
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
        // Contract §8 icon wrapper: icons and the spinner ride in a fixed
        // icon-md box that reserves layout space, glyph centred inside — the
        // native counterpart of the web `.poodle-button__icon` /
        // `.poodle-button__spinner` wrapper. The g15.047 comparator measured
        // GPUI reserving only the 12px glyph box, which shifted the label
        // 2px against the web layout.
        let icon_box = theme.resolve_space("size.icon.md");
        let wrap_glyph = |glyph: Node| -> Node {
            let mut wrapper = Node::container();
            wrapper.style.descriptor.layout.width = LayoutSizing::Fixed(icon_box);
            wrapper.style.descriptor.layout.height = LayoutSizing::Fixed(icon_box);
            wrapper.style.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            wrapper.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            wrapper.child(glyph)
        };
        if spec.is_loading {
            let mut spinner = Node::icon("spinner", spinner_size);
            spinner.style.descriptor.text_color = Some(text_color);
            spinner.style.animation = Some(NodeAnimation::spin("poodle-spinner-ring", 0.8));
            el = el.child(wrap_glyph(spinner));
        }
        if let Some(ref icon_name) = spec.leading_icon {
            let mut icon = Node::icon(icon_name.as_str(), icon_size);
            icon.style.descriptor.text_color = Some(text_color);
            el = el.child(wrap_glyph(icon));
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
            el = el.child(wrap_glyph(icon));
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

    // ── Accessibility projection (contract §8) ──
    // Toggle state mirrors the web `aria-pressed`; disclosure state mirrors
    // `aria-expanded`. None = attribute omitted, exactly like the web
    // implementations.
    if spec.is_toggle_mode() {
        el.a11y.toggled = Some(if spec.current_pressed() {
            poodle_node::NodeToggled::True
        } else {
            poodle_node::NodeToggled::False
        });
    }
    if let Some(expanded) = spec.aria_expanded {
        el.a11y.expanded = Some(expanded);
    }
    el.a11y.controls = spec.controls.clone();
    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    // The renderer declares the role; observers read `a11y.role` alone and
    // never branch on node kinds.
    el.a11y.role = Some(poodle_node::NodeRole::Button);

    // ── Semantic token roles (the native data-* counterpart) ──
    // Observers read these; the web pair projects the same values through
    // its data attributes. Resolved size/density mirror the web's resolved
    // attribute values.
    el.roles.insert(
        "variant".to_owned(),
        format!("{:?}", spec.variant).to_ascii_lowercase(),
    );
    el.roles.insert(
        "tone".to_owned(),
        format!("{:?}", spec.tone).to_ascii_lowercase(),
    );
    el.roles.insert(
        "size".to_owned(),
        format!("{effective_size:?}").to_ascii_lowercase(),
    );
    el.roles.insert(
        "density".to_owned(),
        format!("{:?}", density).to_ascii_lowercase(),
    );
    el.roles.insert(
        "fit".to_owned(),
        format!("{:?}", spec.fit).to_ascii_lowercase(),
    );
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
    fn metrics_follow_the_web_css_ladder() {
        // The web CSS (the reference) hardcodes the per-size height ladder:
        // xs 1.5rem, sm 1.75rem, md 2.25rem, lg 2.75rem, xl 3.25rem.
        let cases = [
            (ControlSize::Xs, 24.0),
            (ControlSize::Sm, 28.0),
            (ControlSize::Md, 36.0),
            (ControlSize::Lg, 44.0),
            (ControlSize::Xl, 52.0),
        ];
        for (size, expected) in cases {
            let spec = ButtonSpec::new().with_size(size);
            let node = button(&spec, &theme(), None);
            assert_eq!(fixed_height(&node), expected, "height for {size:?}");
        }

        // pad_x = space.control.x token + density offset only — the CSS has
        // no per-size padding offsets.
        let theme = theme();
        let base = theme.resolve_space("space.control.x");
        let cases = [
            ((ControlSize::Md, ControlDensity::Default), base),
            ((ControlSize::Sm, ControlDensity::Compact), base - rem_to_px(0.125)),
            ((ControlSize::Lg, ControlDensity::Comfortable), base + rem_to_px(0.125)),
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
        // Contract §8: color-mix(in srgb, accent-base 84%, black).
        assert_eq!(node.style.descriptor.border.color, mix_black(accent, 0.84));
    }

    #[test]
    fn secondary_default_idle_fill_is_elevation_stacked() {
        // Contract §8: color-mix(in srgb, surface 88%, text-primary) — the
        // g15.047 comparator measured the raw-surface fill drifting ~26/255
        // from the web reference; this pins the repaired formula.
        let theme = theme();
        let surface = theme.resolve_color("color.background.surface");
        let text_primary = theme.resolve_color("color.text.primary");
        let border_default = theme.resolve_color("color.border.default");

        let node = button(&ButtonSpec::new(), &theme, None);
        assert_eq!(
            node.style.descriptor.background,
            Some(mix_srgb(surface, text_primary, 0.88))
        );
        assert_eq!(node.style.descriptor.border.color, border_default);
        assert_eq!(node.style.descriptor.text_color, Some(text_primary));
    }

    #[test]
    fn pressed_non_primary_toggle_takes_the_accent_treatment() {
        let theme = theme();
        let accent = theme.resolve_color("color.accent.base");
        let inverse = theme.resolve_color("color.text.inverse");

        let spec = ButtonSpec::new().with_pressed(true);
        let node = button(&spec, &theme, None);
        assert_eq!(node.style.descriptor.background, Some(accent));
        // Contract §8 pressed: color-mix(accent-base 85%, black) border.
        assert_eq!(node.style.descriptor.border.color, mix_black(accent, 0.85));
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

        // Default secondary: hover/active keep the old-tier mix from the
        // final fill toward elevated — unmeasured by the g15.047 batch, so
        // the contract §8 text-primary stacking for these states is recorded
        // as a suspicion, not repaired here. The idle-fill repair means the
        // mix now starts from the stacked fill.
        let node = button(&ButtonSpec::new(), &theme, None);
        let hover = node.style.hover.expect("hover patch");
        let active = node.style.active.expect("active patch");
        let idle_fill = mix_srgb(surface, text_primary, 0.88);
        assert_eq!(hover.background, Some(mix_srgb(idle_fill, elevated, 0.84)));
        assert_eq!(active.background, Some(mix_srgb(idle_fill, elevated, 0.72)));
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
    fn icons_and_spinner_ride_in_the_icon_md_wrapper_box() {
        // Contract §8 icon wrapper: `.poodle-button__icon` / `__spinner`
        // reserve a fixed icon-md box with the glyph centred inside. The
        // g15.047 comparator measured GPUI reserving only the 12px glyph box,
        // shifting the label 2px against the web layout.
        let theme = theme();
        let wrapper_edge = theme.resolve_space("size.icon.md");

        let spec = ButtonSpec::new()
            .with_label("Run")
            .with_leading_icon("play");
        let node = button(&spec, &theme, None);
        let wrapper = node
            .children
            .iter()
            .find(|child| {
                child
                    .find(&|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "play"))
                    .is_some()
            })
            .expect("leading icon wrapper exists");
        assert!(matches!(wrapper.kind, poodle_node::NodeKind::Container));
        assert_eq!(
            wrapper.style.descriptor.layout.width,
            LayoutSizing::Fixed(wrapper_edge)
        );
        assert_eq!(
            wrapper.style.descriptor.layout.height,
            LayoutSizing::Fixed(wrapper_edge)
        );
        // The glyph itself keeps the sm icon token inside the wrapper.
        assert_eq!(icon_size_of(&node, "play"), theme.resolve_space("size.icon.sm"));

        let loading = button(&ButtonSpec::new().with_loading(true), &theme, None);
        let spinner_wrapper = loading
            .children
            .iter()
            .find(|child| {
                child
                    .find(&|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "spinner"))
                    .is_some()
            })
            .expect("spinner wrapper exists");
        assert_eq!(
            spinner_wrapper.style.descriptor.layout.width,
            LayoutSizing::Fixed(wrapper_edge)
        );
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

    /// g14.001 retained regression. Native Button projected no toggle state,
    /// no disclosure state, and no focus-visible treatment, so a pressed
    /// toggle, an open disclosure trigger, and a keyboard-focused control were
    /// all indistinguishable from an idle one — on the backend and to
    /// assistive technology alike. Absence stays absence: a non-toggle button
    /// omits `toggled` exactly as the web omits `aria-pressed`.
    #[test]
    fn toggle_disclosure_and_focus_state_reach_the_accessibility_channel() {
        let plain = button(&ButtonSpec::new().with_label("Save"), &theme(), None);
        assert_eq!(plain.a11y.toggled, None);
        assert_eq!(plain.a11y.expanded, None);
        assert_eq!(plain.a11y.role, Some(poodle_node::NodeRole::Button));
        // The focus-visible treatment is also the observation channel for the
        // state, so a focusable control without one is unobservable.
        assert!(plain.interaction.focusable);
        assert!(plain.style.focus.is_some());

        let pressed = button(
            &ButtonSpec::new().with_label("Mute").with_pressed(true),
            &theme(),
            None,
        );
        assert_eq!(pressed.a11y.toggled, Some(poodle_node::NodeToggled::True));

        let unpressed = button(
            &ButtonSpec::new().with_label("Mute").with_pressed(false),
            &theme(),
            None,
        );
        assert_eq!(unpressed.a11y.toggled, Some(poodle_node::NodeToggled::False));

        let disclosure = button(
            &ButtonSpec::new().with_label("Details").with_aria_expanded(true),
            &theme(),
            None,
        );
        assert_eq!(disclosure.a11y.expanded, Some(true));
    }

    /// Disclosure targets mirror the web `aria-controls` (contract §3):
    /// a spec carrying `with_controls(...)` lands on `node.a11y.controls`;
    /// a bare spec omits it, exactly like the web omits the attribute.
    #[test]
    fn controls_target_reaches_the_accessibility_channel() {
        let plain = button(&ButtonSpec::new().with_label("Save"), &theme(), None);
        assert_eq!(plain.a11y.controls, None);

        let node = button(
            &ButtonSpec::new().with_label("Details").with_controls("details"),
            &theme(),
            None,
        );
        assert_eq!(node.a11y.controls.as_deref(), Some("details"));
    }

    /// g14.001 retained regression: the semantic token roles the web projects
    /// through `data-*` had no native counterpart, so nothing downstream could
    /// tell a primary from a ghost without reading resolved pixels.
    #[test]
    fn semantic_token_roles_are_stamped_with_resolved_values() {
        let spec = ButtonSpec::new()
            .with_label("Save")
            .with_variant(poodle_specs::ButtonVariant::Primary)
            .with_size(ControlSize::Lg)
            .with_density(ControlDensity::Compact);
        let node = button(&spec, &theme(), None);

        assert_eq!(node.roles.get("variant").map(String::as_str), Some("primary"));
        assert_eq!(node.roles.get("tone").map(String::as_str), Some("default"));
        // Resolved, not the declared base: the web pair reports the same.
        assert_eq!(node.roles.get("size").map(String::as_str), Some("lg"));
        assert_eq!(node.roles.get("density").map(String::as_str), Some("compact"));
        assert_eq!(node.roles.get("fit").map(String::as_str), Some("default"));
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
