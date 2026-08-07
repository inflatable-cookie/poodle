use crate::app_state::AppState;
use crate::node_compat::{Breadcrumbs, Eyebrow};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{BreadcrumbItem, BreadcrumbsSpec, EyebrowSpec};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Basic ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Basic"),
                    theme,
                ))
                .child(Breadcrumbs::from_spec(
                    BreadcrumbsSpec::new(vec![
                        BreadcrumbItem::new("home", "Home"),
                        BreadcrumbItem::new("projects", "Projects"),
                        BreadcrumbItem::new("poodle", "Poodle").with_is_current(true),
                    ]),
                    theme,
                )),
        )
        // --- Deep path ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Deep path"),
                    theme,
                ))
                .child(Breadcrumbs::from_spec(
                    BreadcrumbsSpec::new(vec![
                        BreadcrumbItem::new("home", "Home"),
                        BreadcrumbItem::new("workspace", "Workspace"),
                        BreadcrumbItem::new("projects", "Projects"),
                        BreadcrumbItem::new("poodle", "Poodle Design System"),
                        BreadcrumbItem::new("primitives", "Primitives"),
                        BreadcrumbItem::new("button", "Button").with_is_current(true),
                    ]),
                    theme,
                )),
        )
        // --- Collapsed (max 3 visible) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Collapsed (max 3 visible)"),
                    theme,
                ))
                .child(Breadcrumbs::from_spec(
                    BreadcrumbsSpec::new(vec![
                        BreadcrumbItem::new("home", "Home"),
                        BreadcrumbItem::new("workspace", "Workspace"),
                        BreadcrumbItem::new("projects", "Projects"),
                        BreadcrumbItem::new("poodle", "Poodle Design System"),
                        BreadcrumbItem::new("primitives", "Primitives"),
                        BreadcrumbItem::new("button", "Button").with_is_current(true),
                    ])
                    .with_max_visible_items(3),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "breadcrumbs",
        examples,
        |size, theme: &GpuiThemeProvider| {
            Breadcrumbs::from_spec(
                BreadcrumbsSpec::new(vec![
                    BreadcrumbItem::new("home", "Home"),
                    BreadcrumbItem::new("docs", "Docs"),
                    BreadcrumbItem::new("page", "Page").with_is_current(true),
                ]),
                theme,
            )
            .size(size)
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            Breadcrumbs::from_spec(
                BreadcrumbsSpec::new(vec![
                    BreadcrumbItem::new("home", "Home"),
                    BreadcrumbItem::new("docs", "Docs"),
                    BreadcrumbItem::new("page", "Page").with_is_current(true),
                ]),
                theme,
            )
            .with_density(density)
            .into_any_element()
        },
    )
}
