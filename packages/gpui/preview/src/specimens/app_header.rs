use crate::node_compat::{AppHeader, Button, Eyebrow, IconButton, IntoCompatNode};
use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::AppHeaderSpec;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, EyebrowSpec, IconButtonSpec,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
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
                            .with_primary_actions(action_row(
                                theme,
                                &[("ah-file", "File"), ("ah-edit", "Edit"),
                                  ("ah-view", "View"), ("ah-help", "Help")],
                                ControlSize::Sm,
                                4.0,
                            ))
                            .with_utility_items(utility_row(
                                theme,
                                &[("ah-search", "search"), ("ah-bell", "bell"),
                                  ("ah-settings", "settings")],
                                ControlSize::Sm,
                                4.0,
                            )),
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
                        .with_primary_actions(action_row(
                            theme,
                            &[("ah-new", "New"), ("ah-open", "Open")],
                            ControlSize::Sm,
                            6.0,
                        ))
                        .with_utility_items(utility_row(
                            theme,
                            &[("ah-settings2", "settings")],
                            ControlSize::Sm,
                            4.0,
                        )),
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
                    .with_leading(identity_slot(theme))
                    .with_utility_items(utility_row(
                        theme,
                        &[("ah-bell2", "bell"), ("ah-user2", "user")],
                        ControlSize::Sm,
                        4.0,
                    )),
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

fn action_row(
    theme: &GpuiThemeProvider,
    labels: &[(&str, &str)],
    size: ControlSize,
    gap: f32,
) -> Node {
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    row.style.descriptor.layout.spacing.gap = gap;
    for (id, label) in labels {
        row = row.child(
            Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Ghost)
                    .with_label(*label)
                    .with_size(size),
                theme,
            )
            .with_id(*id)
            .into_compat_node(),
        );
    }
    row
}

fn utility_row(
    theme: &GpuiThemeProvider,
    items: &[(&str, &str)],
    size: ControlSize,
    gap: f32,
) -> Node {
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    row.style.descriptor.layout.spacing.gap = gap;
    for (id, icon) in items {
        row = row.child(
            IconButton::from_spec(
                IconButtonSpec::new().with_icon(*icon).with_size(size),
                theme,
            )
            .with_id(*id)
            .into_compat_node(),
        );
    }
    row
}

fn identity_slot(theme: &GpuiThemeProvider) -> Node {
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    row.style.descriptor.layout.spacing.gap = 8.0;

    let mut mark = Node::container();
    mark.style.descriptor.layout.direction = LayoutDirection::Row;
    mark.style.descriptor.layout.width = LayoutSizing::Fixed(20.0);
    mark.style.descriptor.layout.height = LayoutSizing::Fixed(20.0);
    mark.style.descriptor.background = Some(theme.resolve_color("color.accent.base"));
    for corner in [
        &mut mark.style.descriptor.corner_radii.top_left,
        &mut mark.style.descriptor.corner_radii.top_right,
        &mut mark.style.descriptor.corner_radii.bottom_right,
        &mut mark.style.descriptor.corner_radii.bottom_left,
    ] {
        *corner = 4.0;
    }

    let mut title = Node::text("Poodle Studio");
    title.style.text_size = Some(14.0);
    title.style.text_weight = Some(600);
    title.style.descriptor.text_color = Some(theme.resolve_color("color.text.primary"));
    row.child(mark).child(title)
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
    let new_id = format!("ah-new-{id_suffix}");
    let open_id = format!("ah-open-{id_suffix}");
    let settings_id = format!("ah-settings-{id_suffix}");
    AppHeader::from_spec(spec, theme)
        .with_primary_actions(action_row(
            theme,
            &[
                (new_id.as_str(), "New"),
                (open_id.as_str(), "Open"),
            ],
            action_size,
            6.0,
        ))
        .with_utility_items(utility_row(
            theme,
            &[(settings_id.as_str(), "settings")],
            action_size,
            4.0,
        ))
}

fn density_block(theme: &GpuiThemeProvider, label: &str, density: ControlDensity) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(ladder_label(theme, label))
        .child(demo_header(
            AppHeaderSpec::new()
                .with_title("My Application")
                .with_density(density),
            theme,
            &format!("density-{label}").to_lowercase(),
        ))
}

fn size_block(theme: &GpuiThemeProvider, label: &str, size: ControlSize) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(ladder_label(theme, label))
        .child(demo_header(
            AppHeaderSpec::new()
                .with_title("My Application")
                .with_size(size),
            theme,
            &format!("size-{label}").to_lowercase(),
        ))
}
