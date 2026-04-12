use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, Skeleton};
use poodle_specs::{EyebrowSpec, SkeletonSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let border = theme.resolve_color("color.border.default");

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Basic shapes ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Basic shapes"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(12.0))
                        .items_center()
                        .child(Skeleton::from_spec(
                            SkeletonSpec::new()
                                .with_shape("line")
                                .with_width("192.0")
                                .with_height("14.0"),
                            theme,
                        ))
                        .child(Skeleton::from_spec(
                            SkeletonSpec::new()
                                .with_shape("circle")
                                .with_width("40.0")
                                .with_height("40.0"),
                            theme,
                        ))
                        .child(Skeleton::from_spec(
                            SkeletonSpec::new()
                                .with_shape("block")
                                .with_width("128.0")
                                .with_height("48.0"),
                            theme,
                        )),
                ),
        )
        // --- Preset: avatar-line ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Preset: avatar-line"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.0))
                        .child(Skeleton::from_spec(
                            SkeletonSpec::new()
                                .with_shape("circle")
                                .with_width("36.0")
                                .with_height("36.0"),
                            theme,
                        ))
                        .child(Skeleton::from_spec(
                            SkeletonSpec::new()
                                .with_shape("line")
                                .with_width("160.0")
                                .with_height("14.0"),
                            theme,
                        )),
                ),
        )
        // --- Preset: list-item (x3) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Preset: list-item (\u{00d7}3)"),
                    theme,
                ))
                .child(list_item_preset(theme))
                .child(list_item_preset(theme))
                .child(list_item_preset(theme)),
        )
        // --- Preset: table-row (x3) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Preset: table-row (\u{00d7}3)"),
                    theme,
                ))
                .child(table_row_preset(theme, border))
                .child(table_row_preset(theme, border))
                .child(table_row_preset(theme, border)),
        )
        // --- Preset: card ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Preset: card"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(16.0))
                        .child(card_preset(theme, border))
                        .child(card_preset(theme, border)),
                ),
        )
        // --- Preset: detail-section ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Preset: detail-section"),
                    theme,
                ))
                .child(detail_section_preset(theme, 4)),
        )
        // --- Static (no animation) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Static (no animation)"),
                    theme,
                ))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new()
                        .with_shape("line")
                        .with_width("160.0")
                        .with_height("14.0")
                        .with_animated(false),
                    theme,
                )),
        )
}

/// List-item preset: avatar circle + primary line (60%) + secondary line (40%)
fn list_item_preset(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.0))
        .py(px(8.0))
        .child(
            div().flex_shrink_0().child(Skeleton::from_spec(
                SkeletonSpec::new()
                    .with_shape("circle")
                    .with_width("36.0")
                    .with_height("36.0"),
                theme,
            )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .flex_1()
                .min_w(px(0.0))
                .child(div().w(relative(0.6)).child(Skeleton::from_spec(
                    SkeletonSpec::new().with_shape("line").with_height("14.0"),
                    theme,
                )))
                .child(div().w(relative(0.4)).child(Skeleton::from_spec(
                    SkeletonSpec::new().with_shape("line").with_height("11.0"),
                    theme,
                ))),
        )
}

/// Table-row preset: 4 cells at 40%/60%/60%/20% widths
fn table_row_preset(theme: &GpuiThemeProvider, border: poodle_tokens::typed::ColorValue) -> Div {
    div()
        .flex()
        .flex_row()
        .gap(px(12.0))
        .py(px(10.0))
        .border_b_1()
        .border_color(color_to_hsla(border).opacity(0.42))
        .child(div().w(relative(0.4)).child(Skeleton::from_spec(
            SkeletonSpec::new().with_shape("line").with_height("14.0"),
            theme,
        )))
        .child(div().w(relative(0.6)).child(Skeleton::from_spec(
            SkeletonSpec::new().with_shape("line").with_height("14.0"),
            theme,
        )))
        .child(div().w(relative(0.6)).child(Skeleton::from_spec(
            SkeletonSpec::new().with_shape("line").with_height("14.0"),
            theme,
        )))
        .child(div().w(relative(0.2)).child(Skeleton::from_spec(
            SkeletonSpec::new().with_shape("line").with_height("14.0"),
            theme,
        )))
}

/// Card preset: block header + 3 body lines + pill footer
fn card_preset(theme: &GpuiThemeProvider, border: poodle_tokens::typed::ColorValue) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .p(px(16.0))
        .border_1()
        .border_color(color_to_hsla(border).opacity(0.42))
        .rounded(px(6.0))
        .flex_1()
        .child(Skeleton::from_spec(
            SkeletonSpec::new().with_shape("block").with_height("96.0"),
            theme,
        ))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(div().w(relative(0.8)).child(Skeleton::from_spec(
                    SkeletonSpec::new().with_shape("line").with_height("14.0"),
                    theme,
                )))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new().with_shape("line").with_height("14.0"),
                    theme,
                ))
                .child(div().w(relative(0.6)).child(Skeleton::from_spec(
                    SkeletonSpec::new().with_shape("line").with_height("14.0"),
                    theme,
                ))),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap(px(8.0))
                .pt(px(4.0))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new()
                        .with_shape("circle")
                        .with_width("56.0")
                        .with_height("20.0"),
                    theme,
                ))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new()
                        .with_shape("circle")
                        .with_width("56.0")
                        .with_height("20.0"),
                    theme,
                )),
        )
}

/// Detail-section preset: heading + N label-value rows
fn detail_section_preset(theme: &GpuiThemeProvider, lines: usize) -> Div {
    let mut el = div().flex().flex_col().gap(px(8.0));

    el = el.child(
        div().mb(px(4.0)).child(Skeleton::from_spec(
            SkeletonSpec::new()
                .with_shape("line")
                .with_width("128.0")
                .with_height("16.0"),
            theme,
        )),
    );

    for _ in 0..lines {
        el = el.child(
            div()
                .flex()
                .flex_row()
                .gap(px(16.0))
                .items_center()
                .child(
                    div().flex_shrink_0().w(px(96.0)).child(Skeleton::from_spec(
                        SkeletonSpec::new()
                            .with_shape("line")
                            .with_width("96.0")
                            .with_height("12.0"),
                        theme,
                    )),
                )
                .child(div().flex_1().max_w(px(224.0)).child(Skeleton::from_spec(
                    SkeletonSpec::new().with_shape("line").with_height("12.0"),
                    theme,
                ))),
        );
    }

    el
}
