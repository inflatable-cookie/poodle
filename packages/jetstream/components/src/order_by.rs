//! OrderBy — Jetstream sort control backed by OrderBySpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonTone, ButtonVariant, ControlSize, IconButtonSpec, OrderBySpec, SortDirection,
};

use crate::button::js_button;
use crate::icon_button::js_icon_button;
use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size, size_padding_x_offset_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

pub fn js_order_by(spec: &OrderBySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let label_font = rem_to_px(if matches!(effective_size, ControlSize::Xs) {
        0.625
    } else {
        0.75
    });
    let summary_font = rem_to_px(0.875);
    let trigger_h = rem_to_px(match effective_size {
        ControlSize::Xs => 1.625,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.0,
        ControlSize::Lg => 2.25,
        ControlSize::Xl => 2.5,
    });
    let trigger_gap = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.375,
        poodle_specs::ControlDensity::Default => 0.5,
        poodle_specs::ControlDensity::Comfortable => 0.625,
    });
    let trigger_pad_x = rem_to_px(match effective_size {
        ControlSize::Xs => 0.5,
        _ => 0.75 + size_padding_x_offset_rem(effective_size),
    });
    let panel_gap = rem_to_px(0.75);
    let list_gap = rem_to_px(0.5);
    let item_gap = rem_to_px(0.75);
    let item_pad_x = rem_to_px(0.75);
    let item_pad_y = rem_to_px(0.625);

    let text_primary = resolve_color(theme, spec.field_text_token());
    let text_secondary = resolve_color(theme, spec.label_color_token());
    let border = resolve_color(theme, spec.field_border_token());
    let surface = resolve_color(theme, spec.field_fill_token());
    let elevated = resolve_color(theme, spec.field_hover_fill_token());
    let radius = resolve_radius(theme, spec.radius_token());

    let mut root = ui_element::div().flex_col().gap(rem_to_px(0.375));

    let trigger = ui_element::div()
        .flex_row()
        .items_center()
        .gap(trigger_gap)
        .min_w(rem_to_px(12.0))
        .max_w(rem_to_px(28.0))
        .min_h(trigger_h)
        .pl(trigger_pad_x)
        .pr(trigger_pad_x)
        .rounded(radius)
        .border(1.0)
        .border_color(border)
        .bg(surface)
        .child(
            ui_element::label("SORT BY")
                .text_color(text_secondary)
                .text_size(label_font)
                .text_weight(500),
        )
        .child(
            ui_element::label(&spec.summary_text())
                .text_color(text_primary)
                .text_size(summary_font),
        )
        .child(
            ui_element::icon("chevron-down")
                .w(rem_to_px(0.875))
                .h(rem_to_px(0.875))
                .text_color(text_secondary),
        );

    let mut trigger_row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.375))
        .child(trigger);
    if spec.has_value() {
        trigger_row = trigger_row.child(
            ui_element::button("×")
                .w(rem_to_px(1.75))
                .h(rem_to_px(1.75))
                .rounded(radius)
                .text_color(text_secondary)
                .focusable(),
        );
    }
    root = root.child(trigger_row);

    if spec.is_open {
        let current_value = spec.current_value();
        let mut panel = ui_element::div().flex_col().gap(panel_gap);

        if current_value.is_empty() {
            panel = panel.child(
                ui_element::label("No sort fields selected")
                    .text_color(text_secondary)
                    .text_size(summary_font),
            );
        } else {
            let mut list = ui_element::div().flex_col().gap(list_gap);
            for (index, item) in current_value.iter().enumerate() {
                let dir_label = match item.direction {
                    SortDirection::Asc => "Ascending",
                    SortDirection::Desc => "Descending",
                };

                let actions = ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(rem_to_px(0.25))
                    .child(js_icon_button(
                        &IconButtonSpec::new()
                            .with_icon(match item.direction {
                                SortDirection::Asc => "arrow-up",
                                SortDirection::Desc => "arrow-down",
                            })
                            .with_aria_label("Toggle direction")
                            .with_size(ControlSize::Sm),
                        theme,
                    ))
                    .child(js_icon_button(
                        &IconButtonSpec::new()
                            .with_icon("chevron-up")
                            .with_aria_label("Move up")
                            .with_size(ControlSize::Sm)
                            .with_disabled(index == 0),
                        theme,
                    ))
                    .child(js_icon_button(
                        &IconButtonSpec::new()
                            .with_icon("chevron-down")
                            .with_aria_label("Move down")
                            .with_size(ControlSize::Sm)
                            .with_disabled(index + 1 == current_value.len()),
                        theme,
                    ))
                    .child(js_icon_button(
                        &IconButtonSpec::new()
                            .with_icon("x")
                            .with_aria_label("Remove")
                            .with_size(ControlSize::Sm)
                            .with_tone(ButtonTone::Danger),
                        theme,
                    ));

                list = list.child(
                    ui_element::div()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .gap(item_gap)
                        .pl(item_pad_x)
                        .pr(item_pad_x)
                        .pt(item_pad_y)
                        .pb(item_pad_y)
                        .rounded(resolve_radius(theme, "radius.surface"))
                        .border(1.0)
                        .border_color(border)
                        .bg(tint(elevated, 0.88))
                        .child(
                            ui_element::div()
                                .flex_col()
                                .gap(rem_to_px(0.125))
                                .child(
                                    ui_element::div()
                                        .flex_row()
                                        .items_center()
                                        .gap(rem_to_px(0.375))
                                        .child(
                                            ui_element::label("⠿")
                                                .text_color(text_secondary)
                                                .text_size(summary_font),
                                        )
                                        .child(
                                            ui_element::label(&spec.active_label(&item.key))
                                                .text_color(text_primary)
                                                .text_size(summary_font)
                                                .text_weight(600),
                                        ),
                                )
                                .child(
                                    ui_element::label(dir_label)
                                        .text_color(text_secondary)
                                        .text_size(summary_font),
                                ),
                        )
                        .child(actions),
                );
            }
            panel = panel.child(list);
        }

        if !spec.available_fields().is_empty()
            && spec
                .max_fields
                .map(|max| spec.active_count() < max)
                .unwrap_or(true)
        {
            panel = panel.child(
                ui_element::div().child(
                    ui_element::div()
                        .min_w(rem_to_px(10.0))
                        .min_h(rem_to_px(control_height_rem(ControlSize::Sm)))
                        .pl(rem_to_px(0.75))
                        .pr(rem_to_px(0.75))
                        .rounded(radius)
                        .border(1.0)
                        .border_color(border)
                        .bg(surface)
                        .child(
                            ui_element::label("+ Add field")
                                .text_color(text_secondary)
                                .text_size(summary_font),
                        ),
                ),
            );
        }

        if spec.active_count() >= 2 {
            panel = panel.child(js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Ghost)
                    .with_size(ControlSize::Sm)
                    .with_label("Clear all"),
                theme,
            ));
        }

        root = root.child(
            ui_element::div()
                .rounded(resolve_radius(theme, "radius.surface"))
                .border(1.0)
                .border_color(border)
                .bg(elevated)
                .pl(rem_to_px(0.75))
                .pr(rem_to_px(0.75))
                .pt(rem_to_px(0.75))
                .pb(rem_to_px(0.75))
                .child(panel),
        );
    }

    if spec.is_disabled {
        root = root.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    }

    root
}
