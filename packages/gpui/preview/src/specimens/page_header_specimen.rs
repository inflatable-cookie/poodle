use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_components::PageHeaderSpec;
use poodle_components::{
    ButtonSpec, ButtonVariant, ControlSize, BreadcrumbItem, BreadcrumbsSpec, EyebrowSpec,
    PillAppearance, PillSpec, PillTone, StatusTone, TimeAgoSpec,
};
use poodle_gpui_components::{Button, Breadcrumbs, Eyebrow, PageHeader, Pill, TimeAgo};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

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
        // --- With back link and actions ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With back link and actions"), theme))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Media Library")
                            .with_subtitle("Browse, review, and manage uploaded files.")
                            .with_back("/dashboard", "Dashboard"),
                        theme,
                    )
                    .with_actions(
                        div().flex().gap(px(6.0))
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Secondary)
                                        .with_label("Upload")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ph-media-upload")
                            )
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Secondary)
                                        .with_label("Settings")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ph-media-settings")
                            )
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
                                        .with_variant(ButtonVariant::Secondary)
                                        .with_label("Edit")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ph-edit")
                            )
                    )
                )
        )
        // --- With count ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With count"), theme))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Users")
                            .with_count(128)
                            .with_back("/dashboard", "Dashboard"),
                        theme,
                    )
                )
        )
        // --- Section and banner ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Section and banner"), theme))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Nightly Sync")
                            .with_section("Scheduled Task")
                            .with_back("/system/tasks", "Tasks")
                            .with_back_is_contextual(true)
                            .with_banner("This task is currently paused.", StatusTone::Warning),
                        theme,
                    )
                    .with_actions(
                        div().flex().gap(px(6.0))
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Secondary)
                                        .with_label("Run now")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ph-sync-run")
                            )
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Secondary)
                                        .with_label("Edit")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ph-sync-edit")
                            )
                    )
                )
        )
        // --- With MetaBar ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With MetaBar"), theme))
                .child(
                    PageHeader::from_spec(
                        PageHeaderSpec::new("Nightly Sync")
                            .with_section("Scheduled Task")
                            .with_back("/system/tasks", "Tasks"),
                        theme,
                    )
                    .with_meta(
                        div().flex().items_center().gap(px(12.0))
                            .child(
                                Pill::from_spec(
                                    PillSpec::new()
                                        .with_label("Active")
                                        .with_tone(PillTone::Success)
                                        .with_appearance(PillAppearance::Badge),
                                    theme,
                                )
                            )
                            .child(
                                div().text_size(px(13.0)).text_color(color_to_hsla(text_secondary))
                                    .child("Every 6 hours")
                            )
                            .child(
                                div().flex().items_center().gap(px(4.0))
                                    .text_size(px(13.0))
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(div().child("Last run"))
                                    .child(
                                        TimeAgo::from_spec(
                                            TimeAgoSpec::new().with_timestamp("2026-03-30T08:15:00Z"),
                                            theme,
                                        )
                                    )
                            )
                    )
                    .with_actions(
                        div().flex().gap(px(6.0))
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Secondary)
                                        .with_label("Run now")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ph-meta-run")
                            )
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Secondary)
                                        .with_label("Edit schedule")
                                        .with_size(ControlSize::Sm),
                                    theme,
                                ).with_id("ph-meta-edit")
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
