use crate::node_compat::{Eyebrow, SidebarNav};
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, SidebarNavGroup, SidebarNavItem, SidebarNavSpec};

use crate::app_state::AppState;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

/// The single-group catalogue nav both the Examples pane and the axis
/// representatives draw from.
fn axis_groups() -> Vec<SidebarNavGroup> {
    vec![SidebarNavGroup::new(
        "catalogue",
        vec![
            SidebarNavItem::new("button", "Button"),
            SidebarNavItem::new("dock-region", "DockRegion"),
            SidebarNavItem::new("split-view", "SplitView"),
            SidebarNavItem::new("tabs", "Tabs"),
        ],
    )]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let catalogue_groups = axis_groups();

    let harness_groups = vec![
        SidebarNavGroup::new(
            "commands",
            vec![SidebarNavItem::new("shared-commands", "Shared commands")],
        )
        .with_label("Commands"),
        SidebarNavGroup::new(
            "runtime",
            vec![
                SidebarNavItem::new("device-monitor", "Device + monitor control"),
                SidebarNavItem::new("pulse-runtime-foundation", "Pulse runtime foundation"),
                SidebarNavItem::new("support-history", "Support + historical observability"),
            ],
        )
        .with_label("Runtime"),
        SidebarNavGroup::new(
            "shell",
            vec![SidebarNavItem::new("shell-kernel", "Shell kernel")],
        )
        .with_label("Shell"),
    ];
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
                    EyebrowSpec::new().with_content("Single-group catalogue"),
                    theme,
                ))
                .child(
                    div()
                        .w(px(272.0))
                        .min_h(px(320.0))
                        .border_r_1()
                        .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
                        .child(SidebarNav::from_spec(
                            SidebarNavSpec::new(catalogue_groups)
                                .with_aria_label("Catalogue navigation")
                                .with_value("dock-region"),
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
                    EyebrowSpec::new().with_content("Grouped verification nav"),
                    theme,
                ))
                .child(
                    div()
                        .w(px(272.0))
                        .min_h(px(340.0))
                        .border_r_1()
                        .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
                        .child(SidebarNav::from_spec(
                            SidebarNavSpec::new(harness_groups)
                                .with_aria_label("Verification navigation")
                                .with_value("pulse-runtime-foundation"),
                            theme,
                        )),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "sidebar-nav",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                SidebarNav::from_spec(
                    SidebarNavSpec::new(axis_groups())
                        .with_aria_label("Catalogue navigation")
                        .with_value("dock-region")
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                SidebarNav::from_spec(
                    SidebarNavSpec::new(axis_groups())
                        .with_aria_label("Catalogue navigation")
                        .with_value("dock-region")
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
