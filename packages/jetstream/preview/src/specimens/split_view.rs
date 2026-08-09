//! SplitView specimen — two-pane split layout in horizontal and vertical orientations.

use crate::compat::js_split_view;
use crate::compat::{rem_to_px, size_font_rem};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlSize, SplitOrientation, SplitToggleVisibility, SplitViewSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");
    let bg = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.subtle");

    let font_size = rem_to_px(size_font_rem(ControlSize::Md));
    // Standard container heights in rem: 9.375rem ≈ 150px, 12.5rem ≈ 200px.
    let h_short = rem_to_px(9.375);
    let h_tall = rem_to_px(12.5);
    let pad = rem_to_px(0.5);

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Horizontal split",
            secondary,
            div().h(h_short).child(js_split_view(
                &SplitViewSpec::new(SplitOrientation::Horizontal),
                theme,
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Primary")
                                .text_color(text_primary)
                                .text_size(font_size),
                        ),
                ),
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Secondary")
                                .text_color(secondary)
                                .text_size(font_size),
                        ),
                ),
            )),
        ))
        .child(group(
            "Vertical split",
            secondary,
            div().h(h_tall).child(js_split_view(
                &SplitViewSpec::new(SplitOrientation::Vertical),
                theme,
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Top pane")
                                .text_color(text_primary)
                                .text_size(font_size),
                        ),
                ),
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Bottom pane")
                                .text_color(secondary)
                                .text_size(font_size),
                        ),
                ),
            )),
        ))
        .child(group(
            "Horizontal with collapse toggles",
            secondary,
            div().h(h_short).child(js_split_view(
                &SplitViewSpec::new(SplitOrientation::Horizontal)
                    .with_show_collapse_primary(true)
                    .with_show_collapse_secondary(true),
                theme,
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Primary")
                                .text_color(text_primary)
                                .text_size(font_size),
                        ),
                ),
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Secondary")
                                .text_color(secondary)
                                .text_size(font_size),
                        ),
                ),
            )),
        ))
        .child(group(
            "Vertical with collapse toggles",
            secondary,
            div().h(h_tall).child(js_split_view(
                &SplitViewSpec::new(SplitOrientation::Vertical)
                    .with_show_collapse_primary(true)
                    .with_show_collapse_secondary(true),
                theme,
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Top pane")
                                .text_color(text_primary)
                                .text_size(font_size),
                        ),
                ),
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Bottom pane")
                                .text_color(secondary)
                                .text_size(font_size),
                        ),
                ),
            )),
        ))
        .child(group(
            "Hover-revealed toggles (pointer onto the seam)",
            secondary,
            div().h(h_short).child(js_split_view(
                &SplitViewSpec::new(SplitOrientation::Horizontal)
                    .with_show_collapse_primary(true)
                    .with_show_collapse_secondary(true)
                    .with_toggle_visibility(SplitToggleVisibility::Hover),
                theme,
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Primary")
                                .text_color(text_primary)
                                .text_size(font_size),
                        ),
                ),
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Secondary")
                                .text_color(secondary)
                                .text_size(font_size),
                        ),
                ),
            )),
        ))
        .child(group(
            "Primary pane only (secondary empty)",
            secondary,
            div().h(h_short).child(js_split_view(
                &SplitViewSpec::new(SplitOrientation::Horizontal),
                theme,
                Some(
                    div()
                        .grow()
                        .bg(bg)
                        .border_1()
                        .border_color(border)
                        .p(pad)
                        .child(
                            label("Primary only")
                                .text_color(text_primary)
                                .text_size(font_size),
                        ),
                ),
                None,
            )),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
