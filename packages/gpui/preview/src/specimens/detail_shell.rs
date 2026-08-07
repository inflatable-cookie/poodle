use crate::node_compat::{Button, DetailItem, DetailSection, DetailShell, Eyebrow, IntoCompatNode};
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::{ButtonSpec, ButtonVariant, ControlSize, DetailItemSpec, EyebrowSpec};
use poodle_specs::{DetailSectionSpec, DetailShellSpec, DetailState};

fn node_column(gap: f32, children: Vec<Node>) -> Node {
    let mut node = Node::container();
    node.style.descriptor.layout.direction = LayoutDirection::Column;
    node.style.descriptor.layout.spacing.gap = gap;
    children.into_iter().fold(node, Node::child)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let accent = theme.resolve_color("color.accent.base");
    let border = theme.resolve_color("color.border.subtle");

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Layout structure"),
                    theme,
                ))
                .child(
                    div().h(px(180.0)).child(
                        DetailShell::from_spec(DetailShellSpec::new(), theme)
                            .with_header(region_block("Header", accent, border))
                            .with_content(node_column(
                                6.0,
                                vec![
                                    region_block("Section 1", accent, border),
                                    region_block("Section 2", accent, border),
                                    region_block("Section 3", accent, border),
                                ],
                            )),
                    ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Multi-section layout with header"),
                    theme,
                ))
                .child(
                    div().h(px(240.0)).child(
                        DetailShell::from_spec(
                            DetailShellSpec::new().with_title("Poodle Design System"),
                            theme,
                        )
                        .with_content(node_column(
                            8.0,
                            vec![
                                DetailSection::from_spec(
                                    DetailSectionSpec::new().with_title("General"),
                                    theme,
                                )
                                .with_body(node_column(
                                    0.0,
                                    vec![
                                        DetailItem::from_spec(
                                            DetailItemSpec::new("Owner").with_value("Clay"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                        DetailItem::from_spec(
                                            DetailItemSpec::new("Created").with_value("March 2025"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                        DetailItem::from_spec(
                                            DetailItemSpec::new("Repository")
                                                .with_value("github.com/poodle-ui/poodle"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                    ],
                                ))
                                .into_compat_node(),
                                DetailSection::from_spec(
                                    DetailSectionSpec::new().with_title("Configuration"),
                                    theme,
                                )
                                .with_actions(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Ghost)
                                            .with_size(ControlSize::Sm)
                                            .with_label("Reset"),
                                        theme,
                                    )
                                    .with_id("ds-reset"),
                                )
                                .with_body(node_column(
                                    0.0,
                                    vec![
                                        DetailItem::from_spec(
                                            DetailItemSpec::new("Theme").with_value("Dark"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                        DetailItem::from_spec(
                                            DetailItemSpec::new("Density").with_value("Compact"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                        DetailItem::from_spec(
                                            DetailItemSpec::new("Default size")
                                                .with_value("Medium"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                    ],
                                ))
                                .into_compat_node(),
                                DetailSection::from_spec(
                                    DetailSectionSpec::new().with_title("Integrations"),
                                    theme,
                                )
                                .with_body(node_column(
                                    0.0,
                                    vec![
                                        DetailItem::from_spec(
                                            DetailItemSpec::new("Figma").with_value("Connected"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                        DetailItem::from_spec(
                                            DetailItemSpec::new("Storybook")
                                                .with_value("Not configured"),
                                            theme,
                                        )
                                        .into_compat_node(),
                                    ],
                                ))
                                .into_compat_node(),
                            ],
                        )),
                    ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Loading state"),
                    theme,
                ))
                .child(
                    div().h(px(100.0)).child(DetailShell::from_spec(
                        DetailShellSpec::new()
                            .with_title("Loading")
                            .with_state(DetailState::Loading),
                        theme,
                    )),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Error state"),
                    theme,
                ))
                .child(
                    div().h(px(120.0)).child(DetailShell::from_spec(
                        DetailShellSpec::new()
                            .with_title("Error")
                            .with_state(DetailState::Error)
                            .with_state_title("Failed to load")
                            .with_state_message("Something went wrong. Please try again."),
                        theme,
                    )),
                ),
        )
}

fn region_block(
    label: &str,
    accent: poodle_tokens::typed::ColorValue,
    border: poodle_tokens::typed::ColorValue,
) -> Node {
    let mut node = Node::container();
    {
        let style = &mut node.style;
        style.descriptor.layout.direction = LayoutDirection::Row;
        style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        style.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        style.min_height = Some(48.0);
        style.fill_width = true;
        style.descriptor.border.width = 1.0;
        style.descriptor.border.color = border;
        style.descriptor.background =
            Some(poodle_render::color::with_alpha(accent, accent.3 * 0.12));
        let radii = &mut style.descriptor.corner_radii;
        radii.top_left = 8.0;
        radii.top_right = 8.0;
        radii.bottom_right = 8.0;
        radii.bottom_left = 8.0;
    }
    let mut text = Node::text(label);
    text.style.text_size = Some(14.0);
    text.style.text_weight = Some(600);
    node.child(text)
}
