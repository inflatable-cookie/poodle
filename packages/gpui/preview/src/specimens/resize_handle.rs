use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{EyebrowSpec, Orientation, ResizeHandleSpec};
use poodle_gpui_components::{Eyebrow, ResizeHandle};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let panel_bg = theme.resolve_color("semantic.color.background.panel");
    let border_subtle = theme.resolve_color("semantic.color.border.subtle");

    div().flex().flex_col().gap(px(24.0))
        // --- Horizontal split (vertical handle — drag left/right) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Horizontal split (vertical handle \u{2014} drag left/right)"), theme))
                .child(
                    div()
                        .flex()
                        .items_start()
                        .h(px(96.0))
                        .border_1()
                        .border_color(color_to_hsla(border_subtle))
                        .rounded(px(6.0))
                        .overflow_hidden()
                        .child(pane("Left", text_secondary, panel_bg))
                        .child(ResizeHandle::from_spec(
                            ResizeHandleSpec::new()
                                .with_orientation(Orientation::Horizontal)
                                .with_aria_label("Resize horizontal"),
                            theme,
                        ))
                        .child(pane("Right", text_secondary, panel_bg)),
                )
        )
        // --- Vertical split (horizontal handle — drag up/down) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Vertical split (horizontal handle \u{2014} drag up/down)"), theme))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .h(px(160.0))
                        .border_1()
                        .border_color(color_to_hsla(border_subtle))
                        .rounded(px(6.0))
                        .overflow_hidden()
                        .child(pane("Top", text_secondary, panel_bg))
                        .child(ResizeHandle::from_spec(
                            ResizeHandleSpec::new()
                                .with_orientation(Orientation::Vertical)
                                .with_aria_label("Resize vertical"),
                            theme,
                        ))
                        .child(pane("Bottom", text_secondary, panel_bg)),
                )
        )
        // --- Disabled (horizontal split) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled (horizontal split)"), theme))
                .child(
                    div()
                        .flex()
                        .items_start()
                        .h(px(96.0))
                        .border_1()
                        .border_color(color_to_hsla(border_subtle))
                        .rounded(px(6.0))
                        .overflow_hidden()
                        .child(pane("Left", text_secondary, panel_bg))
                        .child(ResizeHandle::from_spec(
                            ResizeHandleSpec::new()
                                .with_orientation(Orientation::Horizontal)
                                .with_disabled(true)
                                .with_aria_label("Disabled resize"),
                            theme,
                        ))
                        .child(pane("Right", text_secondary, panel_bg)),
                )
        )
        // --- Disabled (vertical split) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled (vertical split)"), theme))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .h(px(160.0))
                        .border_1()
                        .border_color(color_to_hsla(border_subtle))
                        .rounded(px(6.0))
                        .overflow_hidden()
                        .child(pane("Top", text_secondary, panel_bg))
                        .child(ResizeHandle::from_spec(
                            ResizeHandleSpec::new()
                                .with_orientation(Orientation::Vertical)
                                .with_disabled(true)
                                .with_aria_label("Disabled resize vertical"),
                            theme,
                        ))
                        .child(pane("Bottom", text_secondary, panel_bg)),
                )
        )
}

/// A mock pane for specimen layout context.
fn pane(
    label: &str,
    text_color: poodle_tokens::typed::ColorValue,
    bg_color: poodle_tokens::typed::ColorValue,
) -> Div {
    div()
        .flex_1()
        .flex()
        .items_center()
        .justify_center()
        .text_sm()
        .text_color(color_to_hsla(text_color))
        .bg(color_to_hsla(bg_color).opacity(0.5))
        .child(label.to_string())
}
