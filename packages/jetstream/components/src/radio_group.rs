//! RadioGroup — Jetstream radio group backed by RadioGroupSpec.
//!
//! Contract: `docs/contracts/components/radio-group.md`
//! Reference: `packages/svelte/components/src/RadioGroup.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, Orientation, RadioGroupSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

/// Indicator (outer circle) size in rem by size variant.
///
/// Contract size table (section 8):
/// - xs: icon-default − 0.125rem  = 0.875rem
/// - sm: icon-default             = 1.0rem
/// - md: 1.125rem (explicit)
/// - lg: icon-default + 0.375rem  = 1.375rem
/// - xl: icon-default + 0.625rem  = 1.625rem
fn indicator_size_rem(size: ControlSize) -> f32 {
    // icon-default = size.icon.md = 1.0rem (16px)
    const ICON_DEFAULT: f32 = 1.0;
    match size {
        ControlSize::Xs => ICON_DEFAULT - 0.125,
        ControlSize::Sm => ICON_DEFAULT,
        ControlSize::Md => 1.125,
        ControlSize::Lg => ICON_DEFAULT + 0.375,
        ControlSize::Xl => ICON_DEFAULT + 0.625,
    }
}

/// Dot (inner filled circle) size in rem by size variant.
///
/// Contract size table (section 8):
/// - xs: icon-default × 0.40  = 0.40rem
/// - sm: icon-default × 0.45  = 0.45rem
/// - md: 0.5rem (explicit)
/// - lg: icon-default × 0.55  = 0.55rem
/// - xl: icon-default × 0.60  = 0.60rem
fn dot_size_rem(size: ControlSize) -> f32 {
    const ICON_DEFAULT: f32 = 1.0;
    match size {
        ControlSize::Xs => ICON_DEFAULT * 0.40,
        ControlSize::Sm => ICON_DEFAULT * 0.45,
        ControlSize::Md => 0.5,
        ControlSize::Lg => ICON_DEFAULT * 0.55,
        ControlSize::Xl => ICON_DEFAULT * 0.60,
    }
}

pub fn js_radio_group(spec: &RadioGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let indicator_size = rem_to_px(indicator_size_rem(effective_size));
    let dot_size = rem_to_px(dot_size_rem(effective_size));
    // Contract: border 0.0625rem (1px) solid
    let border_width = rem_to_px(0.0625);

    // Contract: group gap driven by spec.option_gap_token() (space-stack-sm vertical, space-inline-md horizontal)
    let group_gap = resolve_px(theme, spec.option_gap_token());
    // Contract: option item gap = space-inline-sm (between indicator and label)
    let item_gap = resolve_px(theme, "space.inline.sm");

    let accent = resolve_color(theme, "color.accent.base");
    let border = resolve_color(theme, "color.border.default");
    let text_color = resolve_color(theme, "color.text.primary");
    let selected_value = spec.value.as_deref().or(spec.default_value.as_deref());

    let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");

    let mut el = match spec.orientation {
        Orientation::Horizontal => ui_element::div().flex_row().gap(group_gap),
        Orientation::Vertical => ui_element::div().flex_col().gap(group_gap),
    };

    for option in &spec.options {
        let is_selected = selected_value == Some(option.value.as_str());
        let indicator_color = if is_selected { accent } else { border };
        let indicator_bg = resolve_color(theme, "color.background.surface");

        // Radio indicator: circle with inner dot when selected
        let mut indicator = ui_element::div()
            .w(indicator_size).h(indicator_size)
            .rounded(indicator_size * 0.5)
            .bg(indicator_bg)
            .border(border_width).border_color(indicator_color)
            .items_center().justify_center();

        if is_selected {
            indicator = indicator.child(
                ui_element::div()
                    .w(dot_size).h(dot_size)
                    .rounded(dot_size * 0.5)
                    .bg(accent)
            );
        }

        let mut row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(item_gap)
            .cursor_pointer();
        row = row.child(indicator);
        row = row.child(
            ui_element::label(&option.label)
                .text_color(text_color)
                .text_size(font_size)
        );

        // Contract: per-option disabled applies opacity to that option row only
        if option.is_disabled {
            row = row.opacity(disabled_opacity);
        }

        el = el.child(row);
    }

    // Contract: group-level disabled → opacity on the whole group
    if spec.is_disabled {
        el = el.opacity(disabled_opacity);
    }

    el
}
