use crate::app_state::AppState;
use crate::node_compat::{AppHeader, Button, Eyebrow, IconButton, IntoCompatNode};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::AppHeaderSpec;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, EyebrowSpec, IconButtonSpec,
};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
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
                                &[
                                    ("ah-file", "File"),
                                    ("ah-edit", "Edit"),
                                    ("ah-view", "View"),
                                    ("ah-help", "Help"),
                                ],
                                ControlSize::Sm,
                                4.0,
                            ))
                            .with_utility_items(utility_row(
                                theme,
                                &[
                                    ("ah-search", "search"),
                                    ("ah-bell", "bell"),
                                    ("ah-settings", "settings"),
                                ],
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
        // --- Centred header: destination centre, actions + utility trailing ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Centred header (destination tabs in the centre)"),
                    theme,
                ))
                .child(centered_header(theme, "c")),
        )
        // --- Centred header at narrow width (≤45rem viewport) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Centred header at narrow width (≤45rem viewport)"),
                    theme,
                ))
                // 40rem frame: the native renderer has no viewport breakpoint,
                // so the centred row holds at narrow width (web reflows via
                // the CSS media query; see the contract §8).
                .child(div().w(px(640.0)).child(centered_header(theme, "n"))),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "app-header",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                size_block(theme, size).into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                density_block(theme, density).into_any_element()
            }),
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

/// A destination-style centre region: three muted labels standing in for a
/// tabs group (mirrors soundcheck's centred destinations).
fn destination_row(theme: &GpuiThemeProvider) -> Node {
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    row.style.descriptor.layout.spacing.gap = 16.0;
    for label in ["Editor", "Preview", "Terminal"] {
        let mut t = Node::text(label);
        t.style.text_size = Some(12.0);
        t.style.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
        row = row.child(t);
    }
    row
}

/// The centred demo header shared by the centred and narrow groups: a
/// "My Application" title, destination centre, New/Open actions, settings
/// utility.
fn centered_header(theme: &GpuiThemeProvider, id_suffix: &str) -> AppHeader {
    let new_id = format!("ah-new-{id_suffix}");
    let open_id = format!("ah-open-{id_suffix}");
    let settings_id = format!("ah-settings-{id_suffix}");
    AppHeader::from_spec(AppHeaderSpec::new().with_title("My Application"), theme)
        .with_center(destination_row(theme))
        .with_primary_actions(action_row(
            theme,
            &[(new_id.as_str(), "New"), (open_id.as_str(), "Open")],
            ControlSize::Sm,
            6.0,
        ))
        .with_utility_items(utility_row(
            theme,
            &[(settings_id.as_str(), "settings")],
            ControlSize::Sm,
            4.0,
        ))
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
            &[(new_id.as_str(), "New"), (open_id.as_str(), "Open")],
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

fn density_block(theme: &GpuiThemeProvider, density: ControlDensity) -> Div {
    let label = density_key(density).to_uppercase();
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(ladder_label(theme, &label))
        .child(demo_header(
            AppHeaderSpec::new()
                .with_title("My Application")
                .with_density(density),
            theme,
            &format!("density-{label}").to_lowercase(),
        ))
}

fn size_block(theme: &GpuiThemeProvider, size: ControlSize) -> Div {
    let label = size_key(size).to_uppercase();
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(ladder_label(theme, &label))
        .child(demo_header(
            AppHeaderSpec::new()
                .with_title("My Application")
                .with_size(size),
            theme,
            &format!("size-{label}").to_lowercase(),
        ))
}
