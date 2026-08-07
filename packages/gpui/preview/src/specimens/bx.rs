use crate::node_compat::{Box, Eyebrow};
use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_specs::{BoxSpec, Dimension, EyebrowSpec, Overflow, PaddingScale};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border = theme.resolve_color("color.border.default");

    let body = |content: &str| {
        let mut node = Node::text(content);
        node.style.text_size = Some(14.0);
        node.style.descriptor.text_color = Some(text_secondary);
        node.style.fill_width = true;
        node
    };

    let demo_outline = |child: AnyElement| {
        div()
            .border_1()
            .border_color(color_to_hsla(border))
            .rounded(px(4.0))
            .child(child)
    };

    div().flex().flex_col().gap(px(24.0))
        // --- Default (no padding) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default (no padding)"), theme))
                .child(demo_outline(
                    Box::from_spec(BoxSpec::new(), theme)
                        .with_child(body("Content inside a Box with no padding."))
                        .into_any_element(),
                ))
        )
        // --- With padding ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With padding"), theme))
                .child(demo_outline(
                    Box::from_spec(BoxSpec::new().with_padding(PaddingScale::Lg), theme)
                        .with_child(body("Content inside a Box with large padding."))
                        .into_any_element(),
                ))
        )
        // --- Fixed dimensions (spec-driven: Box resolves 12rem/6rem) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Fixed dimensions"), theme))
                .child(demo_outline(
                    Box::from_spec(
                        BoxSpec::new()
                            .with_padding(PaddingScale::Md)
                            .with_width(Dimension::new("12rem"))
                            .with_height(Dimension::new("6rem")),
                        theme,
                    )
                    .with_child(body("Fixed 12\u{00d7}6rem box."))
                    .into_any_element(),
                ))
        )
        // --- Overflow hidden (spec-driven: Box resolves 10rem/3rem + clip) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Overflow hidden"), theme))
                .child(demo_outline(
                    Box::from_spec(
                        BoxSpec::new()
                            .with_padding(PaddingScale::Sm)
                            .with_width(Dimension::new("10rem"))
                            .with_height(Dimension::new("3rem"))
                            .with_overflow(Overflow::Hidden),
                        theme,
                    )
                    .with_child(body("This text is too long and will be clipped by the overflow hidden setting on the box container."))
                    .into_any_element(),
                ))
        )
}
