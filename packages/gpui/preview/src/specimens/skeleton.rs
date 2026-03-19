use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SkeletonSpec;
use pug_gpui_components::PugSkeleton;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let border = theme.resolve_color("semantic.color.border.default");

    div().flex().flex_col().gap(px(24.0))
        // --- Basic shapes ---
        .child(
            div().flex().flex_col().gap(px(12.0))
                .child(section_label("Basic shapes", text_secondary))
                .child(
                    div().flex().flex_row().flex_wrap().gap(px(12.0)).items_center()
                        // Line shape: 12rem wide
                        .child(PugSkeleton::new(
                            SkeletonSpec::new()
                                .with_shape("line")
                                .with_width("192.0")
                                .with_height("14.0"),
                            theme,
                        ))
                        // Circle shape: 2.5rem diameter
                        .child(PugSkeleton::new(
                            SkeletonSpec::new()
                                .with_shape("circle")
                                .with_width("40.0")
                                .with_height("40.0"),
                            theme,
                        ))
                        // Block shape: 8rem x 3rem
                        .child(PugSkeleton::new(
                            SkeletonSpec::new()
                                .with_shape("block")
                                .with_width("128.0")
                                .with_height("48.0"),
                            theme,
                        ))
                )
        )
        // --- Preset: avatar-line ---
        .child(
            div().flex().flex_col().gap(px(12.0))
                .child(section_label("Preset: avatar-line", text_secondary))
                .child(
                    div().flex().flex_row().items_center().gap(px(12.0))
                        // Avatar circle 2.25rem
                        .child(PugSkeleton::new(
                            SkeletonSpec::new()
                                .with_shape("circle")
                                .with_width("36.0")
                                .with_height("36.0"),
                            theme,
                        ))
                        // Line 10rem
                        .child(PugSkeleton::new(
                            SkeletonSpec::new()
                                .with_shape("line")
                                .with_width("160.0")
                                .with_height("14.0"),
                            theme,
                        ))
                )
        )
        // --- Preset: list-item (x3) ---
        .child(
            div().flex().flex_col().gap(px(0.0))
                .child(section_label("Preset: list-item (\u{00d7}3)", text_secondary))
                .child(list_item_preset(theme))
                .child(list_item_preset(theme))
                .child(list_item_preset(theme))
        )
        // --- Preset: table-row (x3) ---
        .child(
            div().flex().flex_col().gap(px(0.0))
                .child(section_label("Preset: table-row (\u{00d7}3)", text_secondary))
                .child(table_row_preset(theme, border))
                .child(table_row_preset(theme, border))
                .child(table_row_preset(theme, border))
        )
        // --- Preset: card ---
        .child(
            div().flex().flex_col().gap(px(12.0))
                .child(section_label("Preset: card", text_secondary))
                .child(
                    div().flex().flex_row().gap(px(16.0))
                        .child(card_preset(theme, border))
                        .child(card_preset(theme, border))
                )
        )
        // --- Preset: detail-section ---
        .child(
            div().flex().flex_col().gap(px(12.0))
                .child(section_label("Preset: detail-section", text_secondary))
                .child(detail_section_preset(theme, 4))
        )
        // --- Static (no animation) ---
        .child(
            div().flex().flex_col().gap(px(12.0))
                .child(section_label("Static (no animation)", text_secondary))
                .child(PugSkeleton::new(
                    SkeletonSpec::new()
                        .with_shape("line")
                        .with_width("160.0")
                        .with_height("14.0")
                        .with_animated(false),
                    theme,
                ))
        )
}

/// List-item preset: avatar circle + primary line (60%) + secondary line (40%)
fn list_item_preset(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_row().items_center().gap(px(12.0)).py(px(8.0))
        // Avatar 2.25rem circle
        .child(
            div().flex_shrink_0().child(PugSkeleton::new(
                SkeletonSpec::new()
                    .with_shape("circle")
                    .with_width("36.0")
                    .with_height("36.0"),
                theme,
            ))
        )
        // Text lines
        .child(
            div().flex().flex_col().gap(px(6.0)).flex_1().min_w(px(0.0))
                .child(
                    div().w(relative(0.6)).child(PugSkeleton::new(
                        SkeletonSpec::new()
                            .with_shape("line")
                            .with_height("14.0"),
                        theme,
                    ))
                )
                .child(
                    div().w(relative(0.4)).child(PugSkeleton::new(
                        SkeletonSpec::new()
                            .with_shape("line")
                            .with_height("11.0"),
                        theme,
                    ))
                )
        )
}

/// Table-row preset: 4 cells at 40%/60%/60%/20% widths
fn table_row_preset(
    theme: &GpuiThemeProvider,
    border: pug_tokens::typed::ColorValue,
) -> Div {
    div()
        .flex().flex_row().gap(px(12.0))
        .py(px(10.0))
        .border_b_1()
        .border_color(color_to_hsla(border).opacity(0.42))
        .child(
            div().w(relative(0.4)).child(PugSkeleton::new(
                SkeletonSpec::new().with_shape("line").with_height("14.0"),
                theme,
            ))
        )
        .child(
            div().w(relative(0.6)).child(PugSkeleton::new(
                SkeletonSpec::new().with_shape("line").with_height("14.0"),
                theme,
            ))
        )
        .child(
            div().w(relative(0.6)).child(PugSkeleton::new(
                SkeletonSpec::new().with_shape("line").with_height("14.0"),
                theme,
            ))
        )
        .child(
            div().w(relative(0.2)).child(PugSkeleton::new(
                SkeletonSpec::new().with_shape("line").with_height("14.0"),
                theme,
            ))
        )
}

/// Card preset: block header + 3 body lines + pill footer
fn card_preset(
    theme: &GpuiThemeProvider,
    border: pug_tokens::typed::ColorValue,
) -> Div {
    div()
        .flex().flex_col().gap(px(12.0))
        .p(px(16.0))
        .border_1()
        .border_color(color_to_hsla(border).opacity(0.42))
        .rounded(px(6.0))
        .flex_1()
        // Block header 6rem
        .child(PugSkeleton::new(
            SkeletonSpec::new()
                .with_shape("block")
                .with_height("96.0"),
            theme,
        ))
        // Body lines: 80%, 100%, 60%
        .child(
            div().flex().flex_col().gap(px(6.0))
                .child(
                    div().w(relative(0.8)).child(PugSkeleton::new(
                        SkeletonSpec::new().with_shape("line").with_height("14.0"),
                        theme,
                    ))
                )
                .child(PugSkeleton::new(
                    SkeletonSpec::new().with_shape("line").with_height("14.0"),
                    theme,
                ))
                .child(
                    div().w(relative(0.6)).child(PugSkeleton::new(
                        SkeletonSpec::new().with_shape("line").with_height("14.0"),
                        theme,
                    ))
                )
        )
        // Footer: pill elements
        .child(
            div().flex().flex_row().gap(px(8.0)).pt(px(4.0))
                .child(PugSkeleton::new(
                    SkeletonSpec::new()
                        .with_shape("circle")
                        .with_width("56.0")
                        .with_height("20.0"),
                    theme,
                ))
                .child(PugSkeleton::new(
                    SkeletonSpec::new()
                        .with_shape("circle")
                        .with_width("56.0")
                        .with_height("20.0"),
                    theme,
                ))
        )
}

/// Detail-section preset: heading + N label-value rows
fn detail_section_preset(theme: &GpuiThemeProvider, lines: usize) -> Div {
    let mut el = div().flex().flex_col().gap(px(10.0));

    // Heading: 8rem wide, 1rem tall
    el = el.child(
        div().mb(px(4.0)).child(PugSkeleton::new(
            SkeletonSpec::new()
                .with_shape("line")
                .with_width("128.0")
                .with_height("16.0"),
            theme,
        ))
    );

    // Detail rows: label (6rem) + value (flexible, max 14rem)
    for _ in 0..lines {
        el = el.child(
            div().flex().flex_row().gap(px(16.0)).items_center()
                .child(
                    div().flex_shrink_0().w(px(96.0)).child(PugSkeleton::new(
                        SkeletonSpec::new()
                            .with_shape("line")
                            .with_width("96.0")
                            .with_height("12.0"),
                        theme,
                    ))
                )
                .child(
                    div().flex_1().max_w(px(224.0)).child(PugSkeleton::new(
                        SkeletonSpec::new()
                            .with_shape("line")
                            .with_height("12.0"),
                        theme,
                    ))
                )
        );
    }

    el
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
