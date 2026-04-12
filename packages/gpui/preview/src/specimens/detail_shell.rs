use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, DetailItem, DetailSection, DetailShell, Eyebrow};
use poodle_specs::{ButtonSpec, ButtonVariant, ControlSize, DetailItemSpec, EyebrowSpec};
use poodle_specs::{DetailSectionSpec, DetailShellSpec, DetailState};

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
                            .with_content(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(6.0))
                                    .child(region_block("Section 1", accent, border))
                                    .child(region_block("Section 2", accent, border))
                                    .child(region_block("Section 3", accent, border)),
                            ),
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
                        .with_content(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(8.0))
                                .child(
                                    DetailSection::from_spec(
                                        DetailSectionSpec::new().with_title("General"),
                                        theme,
                                    )
                                    .with_body(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(DetailItem::from_spec(
                                                DetailItemSpec::new("Owner").with_value("Clay"),
                                                theme,
                                            ))
                                            .child(DetailItem::from_spec(
                                                DetailItemSpec::new("Created")
                                                    .with_value("March 2025"),
                                                theme,
                                            ))
                                            .child(DetailItem::from_spec(
                                                DetailItemSpec::new("Repository")
                                                    .with_value("github.com/poodle-ui/poodle"),
                                                theme,
                                            )),
                                    ),
                                )
                                .child(
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
                                    .with_body(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(DetailItem::from_spec(
                                                DetailItemSpec::new("Theme").with_value("Dark"),
                                                theme,
                                            ))
                                            .child(DetailItem::from_spec(
                                                DetailItemSpec::new("Density")
                                                    .with_value("Compact"),
                                                theme,
                                            ))
                                            .child(DetailItem::from_spec(
                                                DetailItemSpec::new("Default size")
                                                    .with_value("Medium"),
                                                theme,
                                            )),
                                    ),
                                )
                                .child(
                                    DetailSection::from_spec(
                                        DetailSectionSpec::new().with_title("Integrations"),
                                        theme,
                                    )
                                    .with_body(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(DetailItem::from_spec(
                                                DetailItemSpec::new("Figma")
                                                    .with_value("Connected"),
                                                theme,
                                            ))
                                            .child(DetailItem::from_spec(
                                                DetailItemSpec::new("Storybook")
                                                    .with_value("Not configured"),
                                                theme,
                                            )),
                                    ),
                                ),
                        ),
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
                    div().h(px(100.0)).child(DetailShell::from_spec(
                        DetailShellSpec::new()
                            .with_title("Error")
                            .with_state(DetailState::Error),
                        theme,
                    )),
                ),
        )
}

fn region_block(
    label: &str,
    accent: poodle_tokens::typed::ColorValue,
    border: poodle_tokens::typed::ColorValue,
) -> Div {
    div()
        .min_h(px(48.0))
        .w_full()
        .rounded(px(8.0))
        .border_1()
        .border_color(color_to_hsla(border))
        .bg(color_to_hsla(accent).opacity(0.12))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .child(label.to_string()),
        )
}
