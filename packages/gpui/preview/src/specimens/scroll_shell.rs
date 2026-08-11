use crate::node_compat::{Eyebrow, ScrollShell};
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::{Direction, EyebrowSpec, ScrollShellSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border = theme.resolve_color("color.border.default");

    let surface_row = |label: &str| {
        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.height = LayoutSizing::Fixed(24.0);
            s.descriptor.layout.spacing.padding.left = 8.0;
            s.descriptor.layout.spacing.padding.right = 8.0;
            s.fill_width = true;
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = border;
            s.descriptor.corner_radii.top_left = 3.0;
            s.descriptor.corner_radii.top_right = 3.0;
            s.descriptor.corner_radii.bottom_right = 3.0;
            s.descriptor.corner_radii.bottom_left = 3.0;
        }
        let mut text = Node::text(label);
        text.style.text_size = Some(12.0);
        text.style.descriptor.text_color = Some(text_secondary);
        row.child(text)
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Vertical scroll ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Vertical scroll"),
                    theme,
                ))
                .child(
                    div().h(px(160.0)).child(
                        ScrollShell::from_spec(
                            ScrollShellSpec::new()
                                .with_direction(Direction::Vertical)
                                .with_label("Scrollable content"),
                            theme,
                        )
                        .with_child(surface_row("Item 1"))
                        .with_child(surface_row("Item 2"))
                        .with_child(surface_row("Item 3"))
                        .with_child(surface_row("Item 4"))
                        .with_child(surface_row("Item 5"))
                        .with_child(surface_row("Item 6"))
                        .with_child(surface_row("Item 7"))
                        .with_child(surface_row("Item 8"))
                        .with_child(surface_row("Item 9"))
                        .with_child(surface_row("Item 10"))
                        .with_child(surface_row("Item 11"))
                        .with_child(surface_row("Item 12")),
                    ),
                ),
        )
        // --- Horizontal scroll ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Horizontal scroll"),
                    theme,
                ))
                .child(
                    div().h(px(40.0)).child(
                        ScrollShell::from_spec(
                            ScrollShellSpec::new()
                                .with_direction(Direction::Horizontal)
                                .with_label("Horizontal items"),
                            theme,
                        )
                        .with_child({
                            let mut row = Node::container();
                            row.style.descriptor.layout.direction = LayoutDirection::Row;
                            row.style.descriptor.layout.spacing.gap = 4.0;
                            row.child(column_item("Column 1", border, text_secondary))
                                .child(column_item("Column 2", border, text_secondary))
                                .child(column_item("Column 3", border, text_secondary))
                                .child(column_item("Column 4", border, text_secondary))
                                .child(column_item("Column 5", border, text_secondary))
                                .child(column_item("Column 6", border, text_secondary))
                                .child(column_item("Column 7", border, text_secondary))
                                .child(column_item("Column 8", border, text_secondary))
                                .child(column_item("Column 9", border, text_secondary))
                                .child(column_item("Column 10", border, text_secondary))
                        }),
                    ),
                ),
        )
}

fn column_item(
    label: &str,
    border: poodle_tokens::typed::ColorValue,
    text: poodle_tokens::typed::ColorValue,
) -> Node {
    let mut item = Node::container();
    {
        let s = &mut item.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.height = LayoutSizing::Fixed(28.0);
        s.descriptor.layout.spacing.padding.left = 12.0;
        s.descriptor.layout.spacing.padding.right = 12.0;
        s.flex_none = true;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.corner_radii.top_left = 3.0;
        s.descriptor.corner_radii.top_right = 3.0;
        s.descriptor.corner_radii.bottom_right = 3.0;
        s.descriptor.corner_radii.bottom_left = 3.0;
    }
    let mut content = Node::text(label);
    content.style.text_size = Some(12.0);
    content.style.descriptor.text_color = Some(text);
    content.style.no_wrap = true;
    item.child(content)
}
