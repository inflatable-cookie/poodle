use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{AppHeader, Button, Eyebrow, IconButton};
use poodle_specs::AppHeaderSpec;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, EyebrowSpec, IconButtonSpec,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_primary = theme.resolve_color("color.text.primary");

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
                    EyebrowSpec::new()
                        .with_content("Full app window header (title + menubar + utility)"),
                    theme,
                ))
                .child(
                    div()
                        .border_1()
                        .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
                        .rounded(px(8.0))
                        .overflow_hidden()
                        .child(
                            AppHeader::from_spec(
                                AppHeaderSpec::new()
                                    .with_title("Poodle Studio")
                                    .with_drag_region(true)
                                    .with_aria_label("Application header"),
                                theme,
                            )
                            .with_primary_actions(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(4.0))
                                    .child(
                                        Button::from_spec(
                                            ButtonSpec::new()
                                                .with_variant(ButtonVariant::Ghost)
                                                .with_label("File")
                                                .with_size(ControlSize::Sm),
                                            theme,
                                        )
                                        .with_id("ah-file"),
                                    )
                                    .child(
                                        Button::from_spec(
                                            ButtonSpec::new()
                                                .with_variant(ButtonVariant::Ghost)
                                                .with_label("Edit")
                                                .with_size(ControlSize::Sm),
                                            theme,
                                        )
                                        .with_id("ah-edit"),
                                    )
                                    .child(
                                        Button::from_spec(
                                            ButtonSpec::new()
                                                .with_variant(ButtonVariant::Ghost)
                                                .with_label("View")
                                                .with_size(ControlSize::Sm),
                                            theme,
                                        )
                                        .with_id("ah-view"),
                                    )
                                    .child(
                                        Button::from_spec(
                                            ButtonSpec::new()
                                                .with_variant(ButtonVariant::Ghost)
                                                .with_label("Help")
                                                .with_size(ControlSize::Sm),
                                            theme,
                                        )
                                        .with_id("ah-help"),
                                    ),
                            )
                            .with_utility_items(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(4.0))
                                    .child(
                                        IconButton::from_spec(
                                            IconButtonSpec::new()
                                                .with_icon("search")
                                                .with_size(ControlSize::Sm),
                                            theme,
                                        )
                                        .with_id("ah-search"),
                                    )
                                    .child(
                                        IconButton::from_spec(
                                            IconButtonSpec::new()
                                                .with_icon("bell")
                                                .with_size(ControlSize::Sm),
                                            theme,
                                        )
                                        .with_id("ah-bell"),
                                    )
                                    .child(
                                        IconButton::from_spec(
                                            IconButtonSpec::new()
                                                .with_icon("settings")
                                                .with_size(ControlSize::Sm),
                                            theme,
                                        )
                                        .with_id("ah-settings"),
                                    ),
                            ),
                        )
                        .child(
                            div()
                                .h(px(128.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_sm()
                                .text_color(color_to_hsla(theme.resolve_color("color.text.muted")))
                                .bg(color_to_hsla(theme.resolve_color("color.background.panel")))
                                .child("Application content area"),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With title, actions, and utility"),
                    theme,
                ))
                .child(
                    AppHeader::from_spec(AppHeaderSpec::new().with_title("My Application"), theme)
                        .with_primary_actions(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Ghost)
                                            .with_label("New")
                                            .with_size(ControlSize::Sm),
                                        theme,
                                    )
                                    .with_id("ah-new"),
                                )
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Ghost)
                                            .with_label("Open")
                                            .with_size(ControlSize::Sm),
                                        theme,
                                    )
                                    .with_id("ah-open"),
                                ),
                        )
                        .with_utility_items(
                            div().flex().items_center().gap(px(4.0)).child(
                                IconButton::from_spec(
                                    IconButtonSpec::new()
                                        .with_icon("settings")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                )
                                .with_id("ah-settings2"),
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
                    EyebrowSpec::new().with_content("Title only"),
                    theme,
                ))
                .child(AppHeader::from_spec(
                    AppHeaderSpec::new().with_title("Poodle Workstation"),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Custom identity slot"),
                    theme,
                ))
                .child(
                    AppHeader::from_spec(
                        AppHeaderSpec::new().with_aria_label("Custom identity header"),
                        theme,
                    )
                    .with_leading(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .w(px(20.0))
                                    .h(px(20.0))
                                    .rounded(px(4.0))
                                    .bg(color_to_hsla(theme.resolve_color("color.accent.base"))),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(color_to_hsla(text_primary))
                                    .child("Poodle Studio"),
                            ),
                    )
                    .with_utility_items(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                IconButton::from_spec(
                                    IconButtonSpec::new()
                                        .with_icon("bell")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                )
                                .with_id("ah-bell2"),
                            )
                            .child(
                                IconButton::from_spec(
                                    IconButtonSpec::new()
                                        .with_icon("user")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                )
                                .with_id("ah-user2"),
                            ),
                    ),
                ),
        )
        // --- Density ladder ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Density ladder"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .child(density_block(theme, "COMPACT", ControlDensity::Compact))
                        .child(density_block(theme, "DEFAULT", ControlDensity::Default))
                        .child(density_block(
                            theme,
                            "COMFORTABLE",
                            ControlDensity::Comfortable,
                        )),
                ),
        )
        // --- Size ladder ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Size ladder"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .child(size_block(theme, "XS", ControlSize::Xs))
                        .child(size_block(theme, "SM", ControlSize::Sm))
                        .child(size_block(theme, "MD", ControlSize::Md))
                        .child(size_block(theme, "LG", ControlSize::Lg))
                        .child(size_block(theme, "XL", ControlSize::Xl)),
                ),
        )
}

/// Label above a ladder header, mirroring the Svelte specimen's variant-block label.
fn ladder_label(theme: &GpuiThemeProvider, label: &str) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::BOLD)
        .text_color(color_to_hsla(theme.resolve_color("color.text.muted")))
        .child(label.to_string())
}

/// A "My Application" header with New/Open ghost actions and a settings utility
/// icon — the demo shape used by both ladders (matches the Svelte specimen).
fn demo_header(spec: AppHeaderSpec, theme: &GpuiThemeProvider, id_suffix: &str) -> AppHeader {
    let action_size = spec.effective_size();
    AppHeader::from_spec(spec, theme)
        .with_primary_actions(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Ghost)
                            .with_label("New")
                            .with_size(action_size),
                        theme,
                    )
                    .with_id(format!("ah-new-{id_suffix}")),
                )
                .child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Ghost)
                            .with_label("Open")
                            .with_size(action_size),
                        theme,
                    )
                    .with_id(format!("ah-open-{id_suffix}")),
                ),
        )
        .with_utility_items(
            div().flex().items_center().gap(px(4.0)).child(
                IconButton::from_spec(
                    IconButtonSpec::new()
                        .with_icon("settings")
                        .with_size(action_size),
                    theme,
                )
                .with_id(format!("ah-settings-{id_suffix}")),
            ),
        )
}

fn density_block(theme: &GpuiThemeProvider, label: &str, density: ControlDensity) -> Div {
    div().flex().flex_col().gap(px(8.0)).child(ladder_label(theme, label)).child(demo_header(
        AppHeaderSpec::new()
            .with_title("My Application")
            .with_density(density),
        theme,
        &format!("density-{label}").to_lowercase(),
    ))
}

fn size_block(theme: &GpuiThemeProvider, label: &str, size: ControlSize) -> Div {
    div().flex().flex_col().gap(px(8.0)).child(ladder_label(theme, label)).child(demo_header(
        AppHeaderSpec::new()
            .with_title("My Application")
            .with_size(size),
        theme,
        &format!("size-{label}").to_lowercase(),
    ))
}
