use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, SidebarNav};
use poodle_specs::{EyebrowSpec, SidebarNavGroup, SidebarNavItem, SidebarNavSpec};

use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let catalogue_groups = vec![
        SidebarNavGroup::new(
            "controls",
            vec![
                SidebarNavItem::new("button", "Button"),
                SidebarNavItem::new("icon-button", "IconButton"),
                SidebarNavItem::new("split-button", "SplitButton"),
            ],
        )
        .with_label("Controls"),
        SidebarNavGroup::new(
            "workstation",
            vec![
                SidebarNavItem::new("app-header", "AppHeader"),
                SidebarNavItem::new("dock-region", "DockRegion"),
                SidebarNavItem::new("split-view", "SplitView"),
                SidebarNavItem::new("status-bar", "StatusBar"),
            ],
        )
        .with_label("Workstation"),
    ];

    let harness_groups = vec![
        SidebarNavGroup::new(
            "commands",
            vec![
                SidebarNavItem::new("shared-commands", "Shared commands"),
                SidebarNavItem::new("search-index", "Search index"),
            ],
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
            vec![
                SidebarNavItem::new("shell-kernel", "Shell kernel"),
                SidebarNavItem::new("workspace-layout", "Workspace layout"),
            ],
        )
        .with_label("Shell"),
    ];

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
                    EyebrowSpec::new().with_content("Component catalogue"),
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
                    EyebrowSpec::new().with_content("Grouped workstation nav"),
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
}
