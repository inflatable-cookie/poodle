use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{Orientation, ResizeHandleSpec};
use pug_gpui_components::ResizeHandle;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let panel_bg = theme.resolve_color("semantic.color.background.panel");
    let border_subtle = theme.resolve_color("semantic.color.border.subtle");

    div().flex().flex_col().gap(px(16.0))
        // --- Horizontal split (vertical handle) ---
        .child(section_label("HORIZONTAL SPLIT (VERTICAL HANDLE)", text_secondary))
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
        // --- Vertical split (horizontal handle) ---
        .child(section_label("VERTICAL SPLIT (HORIZONTAL HANDLE)", text_secondary))
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
        // --- Disabled (horizontal split) ---
        .child(section_label("DISABLED (HORIZONTAL SPLIT)", text_secondary))
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
        // --- Disabled (vertical split) ---
        .child(section_label("DISABLED (VERTICAL SPLIT)", text_secondary))
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
}

/// A mock pane for specimen layout context.
fn pane(
    label: &str,
    text_color: pug_tokens::typed::ColorValue,
    bg_color: pug_tokens::typed::ColorValue,
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

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
