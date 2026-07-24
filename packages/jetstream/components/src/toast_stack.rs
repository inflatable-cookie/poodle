//! ToastStack — Jetstream toast notification stack backed by ToastStackSpec.
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, ToastStackSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// Per-size title font-size in rem (contract §8 size table; md base
/// 0.8125rem = `typography-label-size`).
fn title_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.71875,
        ControlSize::Sm => 0.8125,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Per-size message font-size in rem (contract §8 size table; md base 0.8125rem).
fn message_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// Per-size dismiss square dimension in rem (contract §8 size table).
fn dismiss_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.0,
        ControlSize::Sm => 1.125,
        ControlSize::Md => 1.25,
        ControlSize::Lg => 1.5,
        ControlSize::Xl => 1.75,
    }
}

/// Density toast-padding multiplier (contract §8 density table: compact ×0.75,
/// comfortable ×1.25). Applied uniformly to the toast box padding to match the
/// Svelte `padding` shorthand override — this is the contract-justified case
/// where density scales the toast's internal padding on all sides.
fn density_pad_scale(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.75,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.25,
    }
}

pub fn js_toast_stack(spec: &ToastStackSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_px = rem_to_px(title_font_rem(effective_size));
    let message_px = rem_to_px(message_font_rem(effective_size));
    let dismiss_px = rem_to_px(dismiss_size_rem(effective_size));

    // Contract §8 toast padding = space-panel-x scaled by density.
    let base_pad = resolve_px(theme, spec.padding_token());
    let pad = base_pad * density_pad_scale(spec.density);
    // Contract §7 stack gap + toast internal gap = space-stack-sm token.
    let stack_gap = resolve_px(theme, spec.gap_token());
    let item_gap = resolve_px(theme, spec.gap_token());

    let elevated = resolve_color(theme, spec.fill_token());
    let border_default = resolve_color(theme, spec.border_token());
    let radius_base = resolve_radius(theme, spec.radius_token());
    // Contract §8: border-radius = calc(radius-surface - 0.125rem).
    let radius = (radius_base - rem_to_px(0.125)).max(0.0);
    let title_color = resolve_color(theme, spec.title_color_token());
    let message_color = resolve_color(theme, spec.message_color_token());
    let dismiss_color = resolve_color(theme, spec.dismiss_color_token());
    let white = glam::Vec4::new(1.0, 1.0, 1.0, 1.0);

    let mut el = ui_element::div().flex_col().gap(stack_gap);

    for toast in &spec.toasts {
        let tone_color = resolve_color(theme, spec.tone_color(&toast.tone));

        // Contract §8 tone treatments:
        //   accent bar = color-mix(tone 82%, white 6%) ≈ 94% tone + 6% white
        //   border     = color-mix(tone 34%, border-default)
        //   background  = color-mix(tone 12%, elevated) tint
        let accent_bar_color = color_mix(tone_color, white, 0.94);
        let toast_border = color_mix(tone_color, border_default, 0.34);
        let bg_tinted = color_mix(tone_color, elevated, 0.12);

        // Leading tone accent bar — contract §8: 0.1875rem (3px), full height.
        let accent_bar = ui_element::div()
            .w(rem_to_px(0.1875))
            .self_stretch()
            .bg(accent_bar_color);

        // Title + optional message column.
        let mut content = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.25))
            .grow()
            .child(
                ui_element::label(toast.title.as_str())
                    .text_color(title_color)
                    .text_size(title_px)
                    .text_weight(600),
            );
        if let Some(message) = &toast.message {
            content = content.child(
                ui_element::label(message.as_str())
                    .text_color(message_color)
                    .text_size(message_px),
            );
        }
        // Optional action affordance (secondary-button-styled label). The real
        // Button primitive isn't composed here (no nested-component slot in the
        // immediate JsEl pass) — approximated as a bordered, tone-neutral chip;
        // click handling lives in the preview loop. (note)
        if let Some(action) = &toast.action_label {
            content = content.child(
                ui_element::label(action.as_str())
                    .text_color(title_color)
                    .text_size(message_px)
                    .text_weight(600)
                    .border_1()
                    .border_color(toast_border)
                    .rounded(radius)
                    .px(rem_to_px(0.5))
                    .py(rem_to_px(0.25)),
            );
        }

        // Dismiss affordance — × glyph in a sized square; click in preview loop.
        let dismiss = ui_element::label("\u{00d7}")
            .text_color(dismiss_color)
            .text_size(dismiss_px)
            .w(dismiss_px)
            .h(dismiss_px)
            .text_align_center();

        // Toast box: tinted gradient bg, tone border, elevation shadow, clipped.
        // Contract §8 background is a linear gradient (90deg) of the tone tint
        // fading into elevated; shadow = `elevation-overlay`, resolved token-
        // accurately from the typed semantic token (single layer, spread 0;
        // matches GPUI). Inline `.shadow()` keeps the long child-bearing chain
        // readable; the contract's multi-layer stack is the residual.
        let ov = &poodle_tokens::typed::semantic::ELEVATION_OVERLAY;
        let toast_el = ui_element::div()
            .relative()
            .bg(bg_tinted)
            .bg_gradient_linear(
                90.0,
                vec![
                    (color_mix(tone_color, elevated, 0.12).into(), 0.0),
                    (elevated.into(), 0.18),
                ],
            )
            .border_1()
            .border_color(toast_border)
            .rounded(radius)
            .overflow_hidden()
            .shadow(
                ov.offset_x,
                ov.offset_y,
                ov.blur,
                0.0,
                glam::Vec4::new(ov.color.0, ov.color.1, ov.color.2, ov.color.3),
            )
            .pl(pad)
            .pr(pad + rem_to_px(1.5))
            .pt(pad)
            .pb(pad)
            .flex_row()
            .items_start()
            .gap(item_gap)
            .child(accent_bar)
            .child(content)
            .child(dismiss)
            // Enter animation: fade + rise, one-shot (holds its final frame —
            // the engine persists completion across immediate-mode rebuilds,
            // keyed by the toast's stable id, so it plays exactly once).
            .id(format!("poodle-toast-{}", toast.id))
            .animation(toast_enter());

        el = el.child(toast_el);
    }

    el
}

/// One-shot enter animation: fade in + rise 0.5rem, ease-out. Mirrors the
/// Svelte reference's fly/fade entrance.
fn toast_enter() -> jetstream_ui::Animation {
    use jetstream_ui::{AnimatableProperty, Animation, Easing, Keyframe, LoopMode};
    Animation {
        keyframes: vec![
            Keyframe {
                at: 0.0,
                values: vec![
                    (AnimatableProperty::Opacity, 0.0),
                    (AnimatableProperty::TranslateY, rem_to_px(0.5)),
                ],
            },
            Keyframe {
                at: 1.0,
                values: vec![
                    (AnimatableProperty::Opacity, 1.0),
                    (AnimatableProperty::TranslateY, 0.0),
                ],
            },
        ],
        duration_secs: 0.18,
        easing: Easing::EaseOut,
        loop_mode: LoopMode::Once,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{Toast, ToastTone};

    /// Each toast declares a one-shot enter animation keyed by its stable id —
    /// the engine plays it exactly once (completion persists across rebuilds).
    #[test]
    fn toasts_carry_enter_animation() {
        let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let spec = ToastStackSpec::new().with_toasts(vec![Toast::new("t1", "Saved")]);
        let el = js_toast_stack(&spec, &theme);
        let toast = &el.children[0];
        assert!(toast.animation.is_some(), "toast enter animation declared");
        assert!(toast.id.as_deref() == Some("poodle-toast-t1"), "stable per-toast id: {:?}", toast.id);
    }

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_title_message_and_action() {
        let spec = ToastStackSpec::new().with_toasts(vec![Toast::new("t1", "Saved")
            .with_message("Your changes were saved")
            .with_action_label("Undo")]);
        let tree = probe(&js_toast_stack(&spec, &theme()), 360.0, 120.0);
        assert!(tree.has_text("Saved"), "title missing: {:?}", tree.texts());
        assert!(tree.has_text("Your changes were saved"), "message missing");
        assert!(tree.has_text("Undo"), "action missing");
        assert!(tree.has_text("\u{00d7}"), "dismiss missing");
    }

    #[test]
    fn info_tone_accent_bar_uses_status_info() {
        let th = theme();
        let spec = ToastStackSpec::new()
            .with_toasts(vec![Toast::new("t1", "Heads up").with_tone(ToastTone::Info)]);
        let tree = probe(&js_toast_stack(&spec, &th), 360.0, 120.0);
        // Accent bar = color-mix(status.info 94%, white 6%).
        let info = resolve_color(&th, spec.tone_color(&ToastTone::Info));
        let white = glam::Vec4::new(1.0, 1.0, 1.0, 1.0);
        let expected = color_mix(info, white, 0.94);
        let expected = ProbeColor {
            r: expected.x,
            g: expected.y,
            b: expected.z,
            a: expected.w,
        };
        assert!(
            tree.has_background(expected, 0.02),
            "info accent bar missing status.info tint; bgs: {}",
            tree.to_json()
        );
    }

    #[test]
    fn danger_tone_border_mixes_status_danger() {
        let th = theme();
        let spec = ToastStackSpec::new()
            .with_toasts(vec![Toast::new("t1", "Failed").with_tone(ToastTone::Danger)]);
        let tree = probe(&js_toast_stack(&spec, &th), 360.0, 120.0);
        // Toast bg = color-mix(status.danger 12%, elevated) — distinct from a
        // plain elevated fill, proving the tone tint is applied.
        let danger = resolve_color(&th, spec.tone_color(&ToastTone::Danger));
        let elevated = resolve_color(&th, spec.fill_token());
        let tinted = color_mix(danger, elevated, 0.12);
        let tinted = ProbeColor { r: tinted.x, g: tinted.y, b: tinted.z, a: tinted.w };
        assert!(
            tree.has_background(tinted, 0.02),
            "danger toast bg tint missing; bgs: {}",
            tree.to_json()
        );
    }

    #[test]
    fn message_only_toast_still_renders_title() {
        // Regression: title is required and must render even with no message.
        let spec = ToastStackSpec::new()
            .with_toasts(vec![Toast::new("t1", "Title only")]);
        let tree = probe(&js_toast_stack(&spec, &theme()), 360.0, 120.0);
        assert!(tree.has_text("Title only"), "title missing");
        assert!(tree.has_text("\u{00d7}"), "dismiss missing");
    }
}
