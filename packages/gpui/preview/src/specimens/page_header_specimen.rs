use gpui::*;
use poodle_composites::PageHeaderSpec;
use poodle_primitives::{ButtonSpec, ButtonVariant, ControlSize, BreadcrumbItem, BreadcrumbsSpec, EyebrowSpec};
use poodle_gpui_components::{PageHeader, Button, Breadcrumbs, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Basic ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic"), theme))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Components")
                            .with_subtitle("Browse and manage your component library."),
                        theme,
                    )
                )
        )
        // --- With eyebrow and actions ---
        .child(
            div().flex().flex_col().gap(px(8.0))
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
        // --- Title only ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Title only"), theme))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Settings"),
                        theme,
                    )
                )
        )
        // --- With breadcrumbs ---
        .child(
            div().flex().flex_col().gap(px(8.0))
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
}
