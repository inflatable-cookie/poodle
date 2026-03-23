use gpui::*;
use pug_composites::{PageHeaderSpec, PageLoadingSpec, PaginationSummarySpec};
use pug_gpui_components::{PageHeader, PageLoading, PaginationSummary, Button, Breadcrumbs, Eyebrow};
use pug_primitives::{ButtonSpec, ButtonVariant, ControlSize, BreadcrumbItem, BreadcrumbsSpec, EyebrowSpec};
use pug_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {

    div().flex().flex_col().gap(px(24.0))
        // --- PageHeader: Basic ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic"), theme))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Components")
                            .with_subtitle("Browse and manage your component library."),
                        theme,
                    )
                )
        )

        // --- PageHeader: With eyebrow and actions ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With eyebrow and actions"), theme))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Button")
                            .with_eyebrow("Primitive")
                            .with_subtitle("Primary interactive control for triggering actions."),
                        theme,
                    )
                    .with_actions(
                        div().flex().gap(px(6.0))
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Secondary)
                                        .with_label("View source")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ph-source")
                            )
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Primary)
                                        .with_label("Edit")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ph-edit")
                            )
                    )
                )
        )

        // --- PageHeader: Title only ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Title only"), theme))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Settings"),
                        theme,
                    )
                )
        )

        // --- PageHeader: With breadcrumbs ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With breadcrumbs"), theme))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Button")
                            .with_subtitle("Primary interactive control for triggering actions."),
                        theme,
                    )
                    .with_breadcrumbs(
                        Breadcrumbs::from_spec(
                            BreadcrumbsSpec::new(vec![
                                BreadcrumbItem::new("home", "Home"),
                                BreadcrumbItem::new("components", "Components"),
                                BreadcrumbItem::new("primitives", "Primitives"),
                            ]),
                            theme,
                        )
                    )
                )
        )

        // --- PageLoading: Indeterminate ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Page loading: indeterminate"), theme))
                .child(
                    PageLoading::from_spec(
                        PageLoadingSpec::new()
                            .with_message("Loading components..."),
                        theme,
                    )
                )
        )

        // --- PageLoading: Determinate ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Page loading: determinate (60%)"), theme))
                .child(
                    PageLoading::from_spec(
                        PageLoadingSpec::new()
                            .with_value(60.0)
                            .with_max(100.0)
                            .with_message("Importing data...")
                            .with_can_cancel(true),
                        theme,
                    )
                )
        )

        // --- Breadcrumbs: Basic ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Breadcrumbs: basic"), theme))
                .child(
                    Breadcrumbs::from_spec(
                        BreadcrumbsSpec::new(vec![
                            BreadcrumbItem::new("home", "Home"),
                            BreadcrumbItem::new("projects", "Projects"),
                            BreadcrumbItem::new("pug", "Pug").with_is_current(true),
                        ]),
                        theme,
                    )
                )
        )

        // --- Breadcrumbs: Deep path ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Breadcrumbs: deep path"), theme))
                .child(
                    Breadcrumbs::from_spec(
                        BreadcrumbsSpec::new(vec![
                            BreadcrumbItem::new("home", "Home"),
                            BreadcrumbItem::new("workspace", "Workspace"),
                            BreadcrumbItem::new("projects", "Projects"),
                            BreadcrumbItem::new("pug", "Pug Design System"),
                            BreadcrumbItem::new("primitives", "Primitives"),
                            BreadcrumbItem::new("button", "Button").with_is_current(true),
                        ]),
                        theme,
                    )
                )
        )

        // --- Breadcrumbs: Collapsed (max 3 visible) ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Breadcrumbs: collapsed (max 3 visible)"), theme))
                .child(
                    Breadcrumbs::from_spec(
                        BreadcrumbsSpec::new(vec![
                            BreadcrumbItem::new("home", "Home"),
                            BreadcrumbItem::new("workspace", "Workspace"),
                            BreadcrumbItem::new("projects", "Projects"),
                            BreadcrumbItem::new("pug", "Pug Design System"),
                            BreadcrumbItem::new("primitives", "Primitives"),
                            BreadcrumbItem::new("button", "Button").with_is_current(true),
                        ])
                        .with_max_visible_items(3),
                        theme,
                    )
                )
        )

        // --- PaginationSummary ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Pagination summary"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            PaginationSummary::from_spec(
                                PaginationSummarySpec::new(1, 20, 156),
                                theme,
                            )
                        )
                        .child(
                            PaginationSummary::from_spec(
                                PaginationSummarySpec::new(5, 20, 1000),
                                theme,
                            )
                        )
                )
        )
}
