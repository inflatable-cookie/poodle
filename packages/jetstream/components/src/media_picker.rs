//! MediaPicker — Jetstream media picker backed by MediaPickerSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::MediaPickerSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_radius, resolve_px};

pub fn js_media_picker(spec: &MediaPickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));

    // Size-driven thumb dimensions from contract
    let thumb_size = rem_to_px(match effective_size {
        poodle_primitives::ControlSize::Xs => 3.5,
        poodle_primitives::ControlSize::Sm => 4.25,
        poodle_primitives::ControlSize::Md => 4.5,
        poodle_primitives::ControlSize::Lg => 5.0,
        poodle_primitives::ControlSize::Xl => 5.5,
    });

    // Density-driven spacing from contract
    let (grid_gap, item_pad) = match spec.density {
        poodle_primitives::ControlDensity::Compact => (0.25, 0.25),
        poodle_primitives::ControlDensity::Default => (0.375, 0.375),
        poodle_primitives::ControlDensity::Comfortable => (0.5, 0.5),
    };

    let fill = resolve_color(theme, spec.fill_token());
    let radius = resolve_radius(theme, "radius.surface");
    let ctrl_radius = resolve_radius(theme, "radius.control");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.default");
    let gap = resolve_px(theme, "space.stack.sm");

    // Root: dialog body content
    let mut el = ui_element::div()
        .bg(fill)
        .rounded(radius)
        .flex_col().gap(gap)
        .min_h(rem_to_px(20.0));

    // Title
    el = el.child(
        ui_element::label(&spec.title)
            .text_color(text_primary).text_size(rem_to_px(1.0)).text_weight(600)
    );

    // Tabs bar: Browse | Upload
    let tab_bar = ui_element::div()
        .flex_row().gap(rem_to_px(0.5))
        .child(
            ui_element::button("Browse")
                .text_color(text_primary).text_size(font_size)
                .focusable()
        )
        .child(
            ui_element::button("Upload")
                .text_color(text_secondary).text_size(font_size)
                .focusable()
        );
    el = el.child(tab_bar);

    // Search input placeholder
    el = el.child(
        ui_element::div()
            .border(1.0).border_color(border)
            .rounded(ctrl_radius)
            .pl(rem_to_px(0.5)).pr(rem_to_px(0.5))
            .pt(rem_to_px(0.25)).pb(rem_to_px(0.25))
            .child(
                ui_element::label("Search...")
                    .text_color(text_secondary).text_size(font_size)
            )
    );

    // Grid placeholder (showing selected_count indication)
    let mut grid = ui_element::div()
        .flex_row().flex_wrap().gap(rem_to_px(grid_gap));

    // Show placeholder items
    for i in 0..4 {
        let item = ui_element::div()
            .flex_col().items_center().gap(rem_to_px(0.25))
            .pl(rem_to_px(item_pad)).pr(rem_to_px(item_pad))
            .pt(rem_to_px(item_pad)).pb(rem_to_px(item_pad))
            .border(1.0).border_color(if i == 0 { border } else { glam::Vec4::new(0.0, 0.0, 0.0, 0.0) })
            .rounded(ctrl_radius)
            .child(
                ui_element::div()
                    .w(thumb_size).h(thumb_size)
                    .rounded(rem_to_px(0.25))
                    .bg(resolve_color(theme, "color.background.panel"))
            )
            .child(
                ui_element::label(&format!("Item {}", i + 1))
                    .text_color(text_secondary)
                    .text_size(rem_to_px(0.8125))
            );
        grid = grid.child(item);
    }
    el = el.child(grid);

    // Selected count indicator
    if spec.selected_count > 0 {
        el = el.child(
            ui_element::label(&format!("{} selected", spec.selected_count))
                .text_color(text_secondary).text_size(font_size)
        );
    }

    el
}
