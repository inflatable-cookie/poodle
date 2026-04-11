//! EditableList — Jetstream editable list backed by EditableListSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::EditableListSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_editable_list(spec: &EditableListSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let label_font = rem_to_px(0.8125);

    // Size-driven remove button size from contract
    let remove_size = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 1.0,
        poodle_specs::ControlSize::Sm => 1.125,
        poodle_specs::ControlSize::Md => 1.25,
        poodle_specs::ControlSize::Lg => 1.375,
        poodle_specs::ControlSize::Xl => 1.5,
    });
    let item_x = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.5,
        poodle_specs::ControlSize::Sm => 0.625,
        poodle_specs::ControlSize::Md => 0.625,
        poodle_specs::ControlSize::Lg => 0.75,
        poodle_specs::ControlSize::Xl => 0.875,
    });
    let add_x = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.625,
        poodle_specs::ControlSize::Sm => 0.75,
        poodle_specs::ControlSize::Md => 0.75,
        poodle_specs::ControlSize::Lg => 0.875,
        poodle_specs::ControlSize::Xl => 1.0,
    });

    // Density-driven spacing from contract
    let (list_gap, static_gap, item_y, add_gap) = match spec.density {
        poodle_specs::ControlDensity::Compact => (0.375, 0.0625, 0.375, 0.25),
        poodle_specs::ControlDensity::Default => (0.5, 0.125, 0.5, 0.375),
        poodle_specs::ControlDensity::Comfortable => (0.625, 0.1875, 0.625, 0.5),
    };

    let input_fill = resolve_color(theme, spec.input_fill_token());
    let input_border = resolve_color(theme, spec.input_border_token());
    let remove_color = resolve_color(theme, spec.remove_color_token());
    let counter_color = resolve_color(theme, spec.counter_color_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let ctrl_radius = resolve_radius(theme, "radius.control");
    let ctrl_height = rem_to_px(control_height_rem(effective_size));
    let ctrl_px = rem_to_px(control_space_x_rem(spec.density));

    // Root
    let mut el = ui_element::div()
        .flex_col().gap(rem_to_px(list_gap));

    if spec.is_disabled {
        el = el.opacity(0.48);
    }

    // Item list
    if spec.item_count > 0 {
        let mut item_list = ui_element::div()
            .flex_col().gap(rem_to_px(static_gap));

        for i in 0..spec.item_count {
            let item_row = ui_element::div()
                .flex_row().items_center()
                .pl(rem_to_px(item_x)).pr(rem_to_px(item_x))
                .pt(rem_to_px(item_y)).pb(rem_to_px(item_y))
                .rounded(ctrl_radius)
                .bg(input_fill)
                .child(
                    ui_element::label(&format!("Item {}", i + 1))
                        .text_color(text_primary).text_size(font_size)
                        .grow()
                )
                .child(
                    ui_element::button("\u{00D7}")
                        .w(remove_size).h(remove_size)
                        .text_color(remove_color)
                        .text_size(rem_to_px(0.75))
                        .rounded(rem_to_px(0.25))
                );
            item_list = item_list.child(item_row);
        }
        el = el.child(item_list);
    }

    // Add row
    if spec.can_add() {
        let add_row = ui_element::div()
            .flex_row().gap(rem_to_px(add_gap))
            .child(
                ui_element::div()
                    .grow().h(ctrl_height)
                    .border(1.0).border_color(input_border)
                    .rounded(ctrl_radius)
                    .bg(input_fill)
                    .pl(ctrl_px).pr(ctrl_px)
                    .items_center()
                    .child(
                        ui_element::label(&spec.placeholder)
                            .text_color(resolve_color(theme, "color.text.secondary"))
                            .text_size(font_size)
                    )
            )
            .child(
                ui_element::button(&spec.add_label)
                    .h(ctrl_height)
                    .pl(add_x).pr(add_x)
                    .border(1.0).border_color(input_border)
                    .rounded(ctrl_radius)
                    .bg(input_fill)
                    .text_color(text_primary)
                    .text_size(label_font)
                    .text_weight(500)
                    .focusable()
            );
        el = el.child(add_row);
    }

    // Counter
    if spec.shows_counter() {
        let max = spec.max_items.unwrap_or(0);
        el = el.child(
            ui_element::label(&format!("{}/{}", spec.item_count, max))
                .text_color(counter_color).text_size(label_font)
        );
    }

    el
}
