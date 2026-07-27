//! NumberInput — Jetstream number input backed by NumberInputSpec.
//!
//! Supports: prefix/suffix boxed affixes, validation border + icon, optional
//! +/- steppers (gated on `show_steppers`), bounds-based disabled stepper
//! buttons, and proper token resolution throughout.
//!
//! Contract: `docs/contracts/components/number-input.md`
//! Reference: `packages/svelte/components/src/NumberInput.svelte`
//!
//! Interaction note: typing, stepping, clamping and key handling live in the
//! preview event loop, not the component (immediate-mode runtime). JsEl has no
//! `:focus-within` hook, so the contract focus ring is a documented JsEl gap.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{NumberInputSpec, ValidationState};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_number_input(spec: &NumberInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let inline_sz = rem_to_px(size_font_rem(effective_size));
    // Inner stepper gap from the smallest inline-space token (was a bare 0.25rem).
    let btn_gap = theme.resolve_space_px(spec.stepper_gap_token());
    let border_width = theme.resolve_space_px(spec.border_width_token());

    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let fill = resolve_color(theme, spec.fill_token());
    let text_color = resolve_color(theme, spec.text_color_token());
    let stepper_icon_color = resolve_color(theme, spec.stepper_icon_color_token());
    // Boxed-affix chrome (Svelte: border-default box + surface bg + muted text).
    let affix_text = resolve_color(theme, spec.affix_text_token());
    let affix_bg = resolve_color(theme, spec.affix_fill_token());
    let affix_border = resolve_color(theme, spec.affix_border_token());
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    // Svelte recolors the field border per validation state.
    let effective_border = match spec.validation_state {
        ValidationState::Invalid => resolve_color(theme, "color.status.danger"),
        ValidationState::Valid => resolve_color(theme, "color.status.success"),
        ValidationState::Pending => resolve_color(theme, "color.accent.base"),
        ValidationState::None => border,
    };

    let value_text = spec.formatted_value();

    // Bounds checks for stepper button disabled state.
    let at_min = !spec.min.is_infinite() && spec.value <= spec.min;
    let at_max = !spec.max.is_infinite() && spec.value >= spec.max;

    // ── Root container ─────────────────────────────────────────────────────
    let mut el = ui_element::div()
        .bg(fill)
        .border(border_width)
        .border_color(effective_border)
        .rounded(radius)
        .h(height)
        .flex_row()
        .items_center();

    // ── Decrement button (only when steppers enabled) ──────────────────────
    if spec.show_steppers {
        let mut dec_btn = ui_element::button("")
            .aria_label("Decrement")
            .id("poodle-number-input-dec")
            .pl(pad_x)
            .pr(btn_gap)
            .focusable()
            .cursor_pointer()
            .child(
                ui_element::icon("minus")
                    .w(icon_size)
                    .h(icon_size)
                    .text_color(stepper_icon_color),
            );
        if at_min || spec.is_disabled || spec.is_read_only {
            dec_btn = dec_btn.opacity(disabled_opacity).disabled(true);
        }
        el = el.child(dec_btn);
    }

    // Prefix affix (boxed: border + surface bg, inside left edge).
    if let Some(prefix) = &spec.prefix {
        el = el.child(affix_box(prefix, affix_text, affix_bg, affix_border, border_width, font_size, pad_x, height));
    }

    // Value display.
    el = el.child(
        ui_element::label(&value_text)
            .text_color(text_color)
            .text_size(font_size)
            .grow()
            .text_align_center(),
    );

    // Validation state icon (trailing, before suffix/increment).
    if let ValidationState::Invalid = spec.validation_state {
        let color = resolve_color(theme, "color.status.danger");
        el = el.child(
            ui_element::icon("alert-circle")
                .w(inline_sz)
                .h(inline_sz)
                .text_color(color),
        );
    }

    // Suffix affix (boxed, inside right edge).
    if let Some(suffix) = &spec.suffix {
        el = el.child(affix_box(suffix, affix_text, affix_bg, affix_border, border_width, font_size, pad_x, height));
    }

    // ── Increment button (only when steppers enabled) ──────────────────────
    if spec.show_steppers {
        let mut inc_btn = ui_element::button("")
            .aria_label("Increment")
            .id("poodle-number-input-inc")
            .pl(btn_gap)
            .pr(pad_x)
            .focusable()
            .cursor_pointer()
            .child(
                ui_element::icon("plus")
                    .w(icon_size)
                    .h(icon_size)
                    .text_color(stepper_icon_color),
            );
        if at_max || spec.is_disabled || spec.is_read_only {
            inc_btn = inc_btn.opacity(disabled_opacity).disabled(true);
        }
        el = el.child(inc_btn);
    }

    if spec.is_disabled {
        el = el.opacity(disabled_opacity).disabled(true);
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
        .aria_role(jetstream_ui::accesskit::Role::SpinButton)
}

/// A boxed prefix/suffix affix — Svelte `.poodle-number-input__prefix`:
/// bordered box with surface bg + muted text, full control height.
#[allow(clippy::too_many_arguments)]
fn affix_box(
    text: &str,
    text_color: impl Into<jetstream_ui::Color>,
    bg: impl Into<jetstream_ui::Color>,
    border_color: impl Into<jetstream_ui::Color>,
    border_width: f32,
    font_size: f32,
    pad_x: f32,
    height: f32,
) -> JsEl {
    ui_element::div()
        .h(height)
        .bg(bg)
        .border(border_width)
        .border_color(border_color)
        .pl(pad_x)
        .pr(pad_x)
        .flex_row()
        .items_center()
        .justify_center()
        .child(
            ui_element::label(text)
                .text_color(text_color)
                .text_size(font_size),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::ControlSize;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn field_renders_value_text() {
        let spec = NumberInputSpec::new(42.0);
        let tree = probe(&js_number_input(&spec, &theme()), 240.0, 48.0);
        assert!(tree.has_text("42"), "value missing: {:?}", tree.texts());
    }

    #[test]
    fn steppers_gated_on_show_steppers() {
        let th = theme();
        // Default: no steppers.
        let without = probe(&js_number_input(&NumberInputSpec::new(1.0), &th), 240.0, 48.0);
        assert!(
            without.find_token("poodle-number-input-inc").is_none(),
            "steppers should be hidden by default"
        );
        // Enabled: both stepper buttons present.
        let with = probe(
            &js_number_input(&NumberInputSpec::new(1.0).with_steppers(true), &th),
            240.0,
            48.0,
        );
        assert!(with.find_token("poodle-number-input-inc").is_some());
        assert!(with.find_token("poodle-number-input-dec").is_some());
    }

    #[test]
    fn suffix_unit_renders_as_affix() {
        let spec = NumberInputSpec::new(80.0).with_suffix("kg");
        let tree = probe(&js_number_input(&spec, &theme()), 240.0, 48.0);
        assert!(tree.has_text("kg"), "suffix unit missing: {:?}", tree.texts());
        assert!(tree.has_text("80"));
    }

    #[test]
    fn prefix_renders_as_affix() {
        let spec = NumberInputSpec::new(5.0).with_prefix("$");
        let tree = probe(&js_number_input(&spec, &theme()), 240.0, 48.0);
        assert!(tree.has_text("$"), "prefix missing: {:?}", tree.texts());
    }

    #[test]
    fn invalid_recolors_border() {
        let th = theme();
        let valid = js_number_input(&NumberInputSpec::new(1.0), &th);
        let invalid = js_number_input(
            &NumberInputSpec::new(1.0).with_validation_state(ValidationState::Invalid),
            &th,
        );
        assert_ne!(
            valid.style.border_color, invalid.style.border_color,
            "invalid state should recolor the field border"
        );
    }

    #[test]
    fn precision_formats_value() {
        let spec = NumberInputSpec::new(3.2).with_precision(2);
        let tree = probe(&js_number_input(&spec, &theme()), 240.0, 48.0);
        assert!(tree.has_text("3.20"), "precision format missing: {:?}", tree.texts());
    }

    #[test]
    fn size_changes_height() {
        let th = theme();
        let sm = probe(
            &js_number_input(&NumberInputSpec::new(1.0).with_size(ControlSize::Sm), &th),
            240.0,
            80.0,
        );
        let xl = probe(
            &js_number_input(&NumberInputSpec::new(1.0).with_size(ControlSize::Xl), &th),
            240.0,
            80.0,
        );
        assert!(
            xl.nodes[0].h > sm.nodes[0].h,
            "xl ({}) should be taller than sm ({})",
            xl.nodes[0].h,
            sm.nodes[0].h
        );
    }

    #[test]
    fn disabled_reduces_opacity() {
        let spec = NumberInputSpec::new(1.0).with_disabled(true);
        let el = js_number_input(&spec, &theme());
        assert!(el.style.opacity < 1.0);
    }
}
