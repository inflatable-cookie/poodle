use gpui::*;
use poodle_primitives::{BreadcrumbItem, BreadcrumbsSpec, EyebrowSpec};
use poodle_gpui_components::{Breadcrumbs, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Basic ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic"), theme))
                .child(
                    Breadcrumbs::from_spec(
                        BreadcrumbsSpec::new(vec![
                            BreadcrumbItem::new("home", "Home"),
                            BreadcrumbItem::new("projects", "Projects"),
                            BreadcrumbItem::new("poodle", "Poodle").with_is_current(true),
                        ]),
                        theme,
                    )
                )
        )
        // --- Deep path ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Deep path"), theme))
                .child(
                    Breadcrumbs::from_spec(
                        BreadcrumbsSpec::new(vec![
                            BreadcrumbItem::new("home", "Home"),
                            BreadcrumbItem::new("workspace", "Workspace"),
                            BreadcrumbItem::new("projects", "Projects"),
                            BreadcrumbItem::new("poodle", "Poodle Design System"),
                            BreadcrumbItem::new("primitives", "Primitives"),
                            BreadcrumbItem::new("button", "Button").with_is_current(true),
                        ]),
                        theme,
                    )
                )
        )
        // --- Collapsed (max 3 visible) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Collapsed (max 3 visible)"), theme))
                .child(
                    Breadcrumbs::from_spec(
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
                    )
                )
        )
}
