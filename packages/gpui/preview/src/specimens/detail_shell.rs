use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{
    Button, DetailItem, DetailSection, DetailShell, Eyebrow, IntoCompatNode, PageHeader, Pill,
    Separator,
};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_render::context::RenderContext;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlSize, DetailItemSpec, DetailSectionSpec, DetailShellSpec,
    DetailState, EyebrowSpec, PageHeaderSpec, PillAppearance, PillSpec, PillTone,
    SeparatorSpec,
};
use std::sync::Arc;

fn node_column(gap: f32, children: Vec<Node>) -> Node {
    let mut node = Node::container();
    node.style.descriptor.layout.direction = LayoutDirection::Column;
    node.style.descriptor.layout.spacing.gap = gap;
    children.into_iter().fold(node, Node::child)
}

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

fn config_text(state: &AppState, key: &str, default: &str) -> String {
    state
        .specimens
        .text
        .get(key)
        .cloned()
        .unwrap_or_else(|| default.to_string())
}

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let accent = theme.resolve_color("color.accent.base");
    let border = theme.resolve_color("color.border.subtle");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let events = state.node_events.clone();

    let config_theme = config_text(state, "detail-shell-theme", "Dark");
    let config_density = config_text(state, "detail-shell-density", "Compact");
    let config_size = config_text(state, "detail-shell-default-size", "Medium");
    let shell_action = state
        .specimens
        .text
        .get("detail-shell-action")
        .cloned()
        .unwrap_or_default();

    let edit_events = events.clone();
    let edit_theme = config_theme.clone();
    let reset_events = events.clone();

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Layout structure",
            theme,
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
        ))
        .child(
            group(
                "Multi-section layout with header",
                theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        div().h(px(280.0)).child(
                            DetailShell::from_spec(DetailShellSpec::new(), theme)
                                .with_header(
                                    PageHeader::from_spec(
                                        PageHeaderSpec::new("Poodle Design System")
                                            .with_eyebrow("Project")
                                            .with_subtitle(
                                                "A comprehensive component library.",
                                            ),
                                        theme,
                                    )
                                    .with_actions({
                                        let theme = theme.clone();
                                        move |ctx: &RenderContext<'_>| {
                                            let mut row = Node::container();
                                            row.style.descriptor.layout.direction =
                                                LayoutDirection::Row;
                                            row.style.descriptor.layout.spacing.gap = 6.0;
                                            row = row.child(
                                                Pill::from_spec(
                                                    PillSpec::new()
                                                        .with_label("Active")
                                                        .with_tone(PillTone::Success)
                                                        .with_appearance(PillAppearance::Badge),
                                                    &theme,
                                                )
                                                .into_node_with(ctx),
                                            );
                                            row.child(
                                                Button::from_spec(
                                                    ButtonSpec::new()
                                                        .with_variant(ButtonVariant::Secondary)
                                                        .with_label("Edit"),
                                                    &theme,
                                                )
                                                .with_id("detail-shell-edit")
                                                .on_click(Arc::new(move || {
                                                    let next = if edit_theme == "Light" {
                                                        "Dark"
                                                    } else {
                                                        "Light"
                                                    };
                                                    let mut queue = edit_events.lock().unwrap();
                                                    queue.push(NodeSpecimenEvent::SetText {
                                                        key: "detail-shell-theme".to_string(),
                                                        value: next.to_string(),
                                                    });
                                                    queue.push(NodeSpecimenEvent::SetText {
                                                        key: "detail-shell-action".to_string(),
                                                        value: "Edit project".to_string(),
                                                    });
                                                }))
                                                .into_node_with(ctx),
                                            )
                                        }
                                    })
                                    .into_compat_node(),
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
                                                    DetailItemSpec::new("Owner")
                                                        .with_value("Clay"),
                                                    theme,
                                                )
                                                .into_compat_node(),
                                                DetailItem::from_spec(
                                                    DetailItemSpec::new("Created")
                                                        .with_value("March 2025"),
                                                    theme,
                                                )
                                                .into_compat_node(),
                                                DetailItem::from_spec(
                                                    DetailItemSpec::new("Repository")
                                                        .with_value(
                                                            "github.com/poodle-ui/poodle",
                                                        ),
                                                    theme,
                                                )
                                                .into_compat_node(),
                                            ],
                                        ))
                                        .into_compat_node(),
                                        Separator::from_spec(SeparatorSpec::new(), theme)
                                            .into_compat_node(),
                                        DetailSection::from_spec(
                                            DetailSectionSpec::new()
                                                .with_title("Configuration"),
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
                                            .with_id("detail-shell-reset")
                                            .on_click(Arc::new(move || {
                                                let mut queue =
                                                    reset_events.lock().unwrap();
                                                queue.push(NodeSpecimenEvent::SetText {
                                                    key: "detail-shell-theme".to_string(),
                                                    value: "Dark".to_string(),
                                                });
                                                queue.push(NodeSpecimenEvent::SetText {
                                                    key: "detail-shell-density".to_string(),
                                                    value: "Compact".to_string(),
                                                });
                                                queue.push(NodeSpecimenEvent::SetText {
                                                    key: "detail-shell-default-size"
                                                        .to_string(),
                                                    value: "Medium".to_string(),
                                                });
                                                queue.push(NodeSpecimenEvent::SetText {
                                                    key: "detail-shell-action".to_string(),
                                                    value: "Reset configuration".to_string(),
                                                });
                                            })),
                                        )
                                        .with_body(node_column(
                                            0.0,
                                            vec![
                                                DetailItem::from_spec(
                                                    DetailItemSpec::new("Theme")
                                                        .with_value(&config_theme),
                                                    theme,
                                                )
                                                .into_compat_node(),
                                                DetailItem::from_spec(
                                                    DetailItemSpec::new("Density")
                                                        .with_value(&config_density),
                                                    theme,
                                                )
                                                .into_compat_node(),
                                                DetailItem::from_spec(
                                                    DetailItemSpec::new("Default size")
                                                        .with_value(&config_size),
                                                    theme,
                                                )
                                                .into_compat_node(),
                                            ],
                                        ))
                                        .into_compat_node(),
                                        Separator::from_spec(SeparatorSpec::new(), theme)
                                            .into_compat_node(),
                                        DetailSection::from_spec(
                                            DetailSectionSpec::new()
                                                .with_title("Integrations"),
                                            theme,
                                        )
                                        .with_body(node_column(
                                            0.0,
                                            vec![
                                                DetailItem::from_spec(
                                                    DetailItemSpec::new("Figma")
                                                        .with_value("Connected"),
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
                    )
                    .when(!shell_action.is_empty(), |d| {
                        d.child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("Last action: {shell_action}")),
                        )
                    }),
            ),
        )
        .child(group(
            "Loading state",
            theme,
            div().h(px(100.0)).child(DetailShell::from_spec(
                DetailShellSpec::new()
                    .with_title("Loading")
                    .with_state(DetailState::Loading),
                theme,
            )),
        ))
        .child(group(
            "Error state",
            theme,
            div().h(px(120.0)).child(DetailShell::from_spec(
                DetailShellSpec::new()
                    .with_title("Error")
                    .with_state(DetailState::Error)
                    .with_state_title("Failed to load")
                    .with_state_message("Something went wrong. Please try again."),
                theme,
            )),
        ))
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
