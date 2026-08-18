use crate::app_state::AppState;
use crate::node_compat::{Button, DetailItem, Eyebrow};
use crate::specimens::specimen_axes::density_key;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{LayoutDirection, Node};
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, DetailItemLayout,
    DetailItemPresentation, DetailItemSpec, EyebrowSpec,
};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let success = theme.resolve_color("color.status.success");

    let mut status_value = Node::container();
    status_value.style.descriptor.layout.direction = LayoutDirection::Row;
    status_value.style.descriptor.layout.spacing.padding.left = 8.0;
    status_value.style.descriptor.layout.spacing.padding.right = 8.0;
    status_value.style.descriptor.layout.spacing.padding.top = 2.0;
    status_value.style.descriptor.layout.spacing.padding.bottom = 2.0;
    status_value.style.descriptor.background =
        Some(poodle_render::color::with_alpha(success, success.3 * 0.15));
    let radii = &mut status_value.style.descriptor.corner_radii;
    radii.top_left = 999.0;
    radii.top_right = 999.0;
    radii.bottom_right = 999.0;
    radii.bottom_left = 999.0;
    let mut status_text = Node::text("Active");
    status_text.style.text_size = Some(12.0);
    status_text.style.text_weight = Some(500);
    status_text.style.descriptor.text_color = Some(success);
    let status_value = status_value.child(status_text);
    let examples = div().flex().flex_col().gap(px(24.0))
        // --- Basic label-value pairs ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic label-value pairs"), theme))
                .child(
                    div().flex().flex_col()
                        .child(DetailItem::from_spec(DetailItemSpec::new("Name").with_value("Poodle Design System"), theme))
                        .child(DetailItem::from_spec(DetailItemSpec::new("Version").with_value("2.1.0"), theme))
                        .child(DetailItem::from_spec(DetailItemSpec::new("License").with_value("MIT"), theme))
                )
        )
        // --- With description ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With description"), theme))
                .child(
                    DetailItem::from_spec(
                        DetailItemSpec::new("API endpoint")
                            .with_value("https://api.example.com/v2")
                            .with_description("Base URL for all API requests.")
                            .with_truncate_value(true),
                        theme,
                    )
                )
        )
        // --- With action slot ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With action slot"), theme))
                .child(
                    DetailItem::from_spec(
                        DetailItemSpec::new("Email").with_value("clay@example.com"),
                        theme,
                    )
                    .with_action(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_size(ControlSize::Sm)
                                .with_label("Change"),
                            theme,
                        ).with_id("dr-change")
                    )
                )
        )
        // --- With value slot ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With value slot"), theme))
                .child(
                    DetailItem::from_spec(
                        DetailItemSpec::new("Status"),
                        theme,
                    )
                    .with_value_content(status_value)
                )
        )
        // --- Surface presentation ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Surface presentation"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(DetailItem::from_spec(
                            DetailItemSpec::new("Name")
                                .with_value("Alice Chen")
                                .with_presentation(DetailItemPresentation::Surface),
                            theme,
                        ))
                        .child(DetailItem::from_spec(
                            DetailItemSpec::new("Role")
                                .with_value("Engineer")
                                .with_presentation(DetailItemPresentation::Surface),
                            theme,
                        ))
                        .child(DetailItem::from_spec(
                            DetailItemSpec::new("Email")
                                .with_value("alice@example.com")
                                .with_presentation(DetailItemPresentation::Surface),
                            theme,
                        )
                        .with_action(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_size(ControlSize::Sm)
                                    .with_label("Edit"),
                                theme,
                            ).with_id("dr-surface-edit")
                        ))
                )
        )
        // --- Stacked layout ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Stacked layout"), theme))
                .child(
                    DetailItem::from_spec(
                        DetailItemSpec::new("Arrangement")
                            .with_value("2CF8B3D0-F592-4D87-8F9F-74D6B42E0E7D:main:external:0:0:3440:1440:1000|37D8832A...")
                            .with_truncate_value(true)
                            .with_layout(DetailItemLayout::Stacked),
                        theme,
                    )
                )
        )
        // --- Simple vs surface presentation ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Simple vs surface presentation"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(DetailItem::from_spec(
                            DetailItemSpec::new("Simple")
                                .with_value("Plain row, no chrome")
                                .with_presentation(DetailItemPresentation::Simple),
                            theme,
                        ))
                        .child(DetailItem::from_spec(
                            DetailItemSpec::new("Surface")
                                .with_value("Elevated card row")
                                .with_presentation(DetailItemPresentation::Surface),
                            theme,
                        ))
                )
        )
        // --- Empty value (em-dash placeholder) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Empty value (em-dash)"), theme))
                .child(
                    DetailItem::from_spec(
                        DetailItemSpec::new("Owner"),
                        theme,
                    )
                )
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "detail-item",
        examples,
        SpecimenAxes::examples_only().with_densities(|density, theme: &GpuiThemeProvider| {
            density_demo(density, theme).into_any_element()
        }),
    )
}

fn density_demo(density: ControlDensity, theme: &GpuiThemeProvider) -> Div {
    let label = density_key(density);
    let muted = theme.resolve_color("color.text.muted");
    div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(
            div()
                .text_xs()
                .text_color(color_to_hsla(muted))
                .child(label.to_string()),
        )
        .child(
            DetailItem::from_spec(
                DetailItemSpec::new("Storage")
                    .with_value("84.2 GB")
                    .with_description("Current usage for the active workspace.")
                    .with_presentation(DetailItemPresentation::Surface)
                    .with_density(density),
                theme,
            )
            .with_action(
                Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_size(ControlSize::Sm)
                        .with_label("Manage"),
                    theme,
                )
                .with_id(format!("dr-density-{label}")),
            ),
        )
}
