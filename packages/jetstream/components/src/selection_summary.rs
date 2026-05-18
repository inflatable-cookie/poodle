//! SelectionSummary — Jetstream selection summary backed by SelectionSummarySpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SelectionSummarySpec;

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::resolve_color;

pub fn js_selection_summary(spec: &SelectionSummarySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let chip_font = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.6875,
        poodle_specs::ControlSize::Sm => 0.71875,
        poodle_specs::ControlSize::Md => 0.75,
        poodle_specs::ControlSize::Lg => 0.8125,
        poodle_specs::ControlSize::Xl => 0.875,
    });
    let gap = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.375,
        poodle_specs::ControlDensity::Default => control_space_x_rem(spec.density),
        poodle_specs::ControlDensity::Comfortable => 0.75,
    });
    let chip_radius = rem_to_px(0.5);

    let text_color = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_tertiary = resolve_color(theme, "color.text.tertiary");
    let surface = resolve_color(theme, "color.background.surface");
    let elevated = resolve_color(theme, "color.background.elevated");
    let chip_bg = elevated.lerp(surface, 0.40);
    let overflow_bg = elevated.lerp(surface, 0.32);
    let chip_border = resolve_color(theme, "color.border.subtle");
    let accent = resolve_color(theme, "color.accent.base");
    let bottom_pad = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.5,
        poodle_specs::ControlDensity::Default => 0.625,
        poodle_specs::ControlDensity::Comfortable => 0.75,
    });
    let chip_px = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.625,
        poodle_specs::ControlDensity::Default => 0.75,
        poodle_specs::ControlDensity::Comfortable => 0.875,
    });
    let overflow_px = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.5,
        poodle_specs::ControlDensity::Default => 0.625,
        poodle_specs::ControlDensity::Comfortable => 0.75,
    });
    let chip_min_h = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 1.0,
        poodle_specs::ControlSize::Sm => 1.125,
        poodle_specs::ControlSize::Md => 1.5,
        poodle_specs::ControlSize::Lg => 1.75,
        poodle_specs::ControlSize::Xl => 2.0,
    });

    let mut el = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap)
        .flex_wrap()
        .self_stretch();
    el = el.pb(bottom_pad).min_h(chip_min_h);

    if spec.items.is_empty() {
        return el.child(
            ui_element::label("No selection")
                .text_color(text_tertiary)
                .text_size(chip_font),
        );
    }

    for item in spec.items.iter().take(spec.visible_item_count()) {
        let mut chip = ui_element::button("")
            .flex_row()
            .items_center()
            .gap(gap)
            .text_color(text_color)
            .pl(chip_px)
            .pr(chip_px)
            .min_h(chip_min_h)
            .rounded(chip_radius)
            .bg(chip_bg)
            .border(1.0)
            .border_color(chip_border)
            .child(
                ui_element::label(&item.label)
                    .text_color(text_color)
                    .text_size(chip_font),
            );

        if let Some(meta) = item.meta.as_ref() {
            chip = chip.child(
                ui_element::label(meta)
                    .text_color(text_secondary)
                    .text_size(chip_font),
            );
        }

        chip = chip.child(
            ui_element::label("×")
                .text_color(text_secondary)
                .text_size(chip_font),
        );

        el = el.child(chip);
    }

    if spec.overflow_count() > 0 {
        el = el.child(
            ui_element::label(&format!("+{} more", spec.overflow_count()))
                .text_color(text_secondary)
                .text_size(chip_font)
                .pl(overflow_px)
                .pr(overflow_px)
                .min_h(chip_min_h)
                .rounded(chip_radius)
                .bg(overflow_bg)
                .border(1.0)
                .border_color(chip_border),
        );
    }

    if let Some(clear) = &spec.clear_action {
        el = el.child(
            ui_element::div().flex_grow().flex_row().justify_end().child(
                ui_element::button(&clear.label)
                    .text_color(accent)
                    .text_size(font_size)
                    .focusable(),
            ),
        );
    }

    el
}
