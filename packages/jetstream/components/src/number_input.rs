//! NumberInput — Jetstream number input backed by NumberInputSpec.
//!
//! Supports: prefix/suffix affixes, validation state icon, bounds-based
//! disabled stepper buttons, and proper token resolution throughout.

use jetstream_runtime::ui_element::{self, JsEl};
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
    let btn_gap = rem_to_px(0.25);

    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let fill = resolve_color(theme, spec.fill_token());
    let text_color = resolve_color(theme, spec.text_color_token());
    let stepper_icon_color = resolve_color(theme, spec.stepper_icon_color_token());
    let affix_color = resolve_color(theme, "color.text.secondary");
    let affix_sep_color = resolve_color(theme, "color.border.subtle");
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    let value_text = spec.formatted_value();

    // Bounds checks for stepper button disabled state
    let at_min = !spec.min.is_infinite() && spec.value <= spec.min;
    let at_max = !spec.max.is_infinite() && spec.value >= spec.max;

    // ── Decrement button ───────────────────────────────────────────────────
    let mut dec_btn = ui_element::button("")
        .pl(pad_x).pr(btn_gap)
        .focusable()
        .cursor_pointer()
        .child(
            ui_element::icon("minus")
                .w(icon_size).h(icon_size)
                .text_color(stepper_icon_color),
        );
    if at_min {
        dec_btn = dec_btn.opacity(disabled_opacity).disabled(true);
    }

    // ── Increment button ───────────────────────────────────────────────────
    let mut inc_btn = ui_element::button("")
        .pl(btn_gap).pr(pad_x)
        .focusable()
        .cursor_pointer()
        .child(
            ui_element::icon("plus")
                .w(icon_size).h(icon_size)
                .text_color(stepper_icon_color),
        );
    if at_max {
        inc_btn = inc_btn.opacity(disabled_opacity).disabled(true);
    }

    // ── Root container ─────────────────────────────────────────────────────
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .h(height)
        .flex_row().items_center();

    el = el.child(dec_btn);

    // Prefix affix (inside left edge, after decrement button)
    if let Some(prefix) = &spec.prefix {
        let divider = ui_element::div()
            .w(1.0)
            .bg(affix_sep_color);
        el = el
            .child(
                ui_element::label(prefix.as_str())
                    .text_color(affix_color)
                    .text_size(font_size),
            )
            .child(divider);
    }

    // Value display
    el = el.child(
        ui_element::label(&value_text)
            .text_color(text_color)
            .text_size(font_size)
            .grow()
            .text_align_center(),
    );

    // Validation state icon (trailing position, before suffix/increment)
    match spec.validation_state {
        ValidationState::Invalid => {
            let color = resolve_color(theme, "color.status.danger");
            el = el.child(
                ui_element::icon("alert-circle")
                    .w(inline_sz).h(inline_sz)
                    .text_color(color),
            );
        }
        ValidationState::Valid
        | ValidationState::Pending
        | ValidationState::None => {}
    }

    // Suffix affix (inside right edge, before increment button)
    if let Some(suffix) = &spec.suffix {
        let divider = ui_element::div()
            .w(1.0)
            .bg(affix_sep_color);
        el = el
            .child(divider)
            .child(
                ui_element::label(suffix.as_str())
                    .text_color(affix_color)
                    .text_size(font_size),
            );
    }

    el = el.child(inc_btn);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity);
    }

    el
}
