//! CardRadioGroup — Jetstream card radio group backed by CardRadioGroupSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::CardRadioGroupSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_card_radio_group(spec: &CardRadioGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Size-driven indicator dimensions from contract
    let indicator_size = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.875,
        poodle_specs::ControlSize::Sm => 1.0,
        poodle_specs::ControlSize::Md => 1.125,
        poodle_specs::ControlSize::Lg => 1.25,
        poodle_specs::ControlSize::Xl => 1.375,
    });
    let dot_size = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.25,
        poodle_specs::ControlSize::Sm => 0.375,
        poodle_specs::ControlSize::Md => 0.375,
        poodle_specs::ControlSize::Lg => 0.4375,
        poodle_specs::ControlSize::Xl => 0.5,
    });
    let title_font = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.75,
        poodle_specs::ControlSize::Sm => 0.8125,
        poodle_specs::ControlSize::Md => 0.9375,
        poodle_specs::ControlSize::Lg => 1.0625,
        poodle_specs::ControlSize::Xl => 1.125,
    });
    let desc_font = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.6875,
        poodle_specs::ControlSize::Sm => 0.75,
        poodle_specs::ControlSize::Md => 0.8125,
        poodle_specs::ControlSize::Lg => 0.875,
        poodle_specs::ControlSize::Xl => 0.9375,
    });

    // Density-driven grid gap from contract
    let grid_gap = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.5,
        poodle_specs::ControlDensity::Default => 0.75,
        poodle_specs::ControlDensity::Comfortable => 1.0,
    });

    let selected_fill = resolve_color(theme, spec.selected_fill_token());
    let unselected_fill = resolve_color(theme, spec.unselected_fill_token());
    let border_color = resolve_color(theme, spec.border_token());
    let accent = resolve_color(theme, "color.accent.base");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_inverse = resolve_color(theme, "color.text.inverse");
    let radius = resolve_radius(theme, "radius.surface");

    let current_value = spec.current_value();

    // Root: grid container with role="radiogroup"
    let mut el = ui_element::div()
        .flex_row().flex_wrap().gap(grid_gap);

    for option in &spec.options {
        let is_checked = current_value == Some(option.value.as_str());
        let is_item_disabled = spec.is_disabled || option.is_disabled;

        let mut card = ui_element::div()
            .flex_col().grow()
            .bg(if is_checked { selected_fill } else { unselected_fill })
            .border(1.0).border_color(border_color)
            .rounded(radius)
            .pl(rem_to_px(0.75)).pr(rem_to_px(0.75))
            .pt(rem_to_px(0.75)).pb(rem_to_px(0.75));

        if is_item_disabled {
            card = card.opacity(0.48);
        }

        // Header row: indicator + title
        let mut header = ui_element::div()
            .flex_row().items_center().gap(rem_to_px(0.5));

        // Radio indicator
        let mut indicator = ui_element::div()
            .w(indicator_size).h(indicator_size)
            .rounded(999.0)
            .items_center().justify_center();

        if is_checked {
            indicator = indicator.bg(accent).border(rem_to_px(0.125)).border_color(accent);
            // Inner dot
            indicator = indicator.child(
                ui_element::div()
                    .w(dot_size).h(dot_size)
                    .rounded(999.0)
                    .bg(text_inverse)
            );
        } else {
            indicator = indicator.border(rem_to_px(0.125)).border_color(border_color);
        }

        header = header.child(indicator);
        header = header.child(
            ui_element::label(&option.label)
                .text_color(text_primary).text_size(title_font).text_weight(600)
        );

        card = card.child(header);

        // Description (optional)
        if let Some(ref description) = option.description {
            card = card.child(
                ui_element::label(description)
                    .text_color(text_secondary).text_size(desc_font)
            );
        }

        el = el.child(card);
    }

    el
}
