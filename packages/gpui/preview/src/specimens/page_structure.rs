use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{PageHeaderSpec, PageLoadingSpec, PaginationSummarySpec};
use pug_gpui_components::{PageHeader, PageLoading, PaginationSummary, Button};
use pug_primitives::{ButtonSpec, ButtonVariant, ControlSize};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- PageHeader: Basic ---
        .child(section_label("PAGE HEADER: BASIC", text_secondary))
        .child(
            PageHeader::from_spec(
                PageHeaderSpec::new("Components")
                    .with_subtitle("Browse and manage your component library."),
                theme,
            )
        )

        // --- PageHeader: With eyebrow and actions ---
        .child(section_label("PAGE HEADER: WITH EYEBROW & ACTIONS", text_secondary))
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

        // --- PageHeader: Title only ---
        .child(section_label("PAGE HEADER: TITLE ONLY", text_secondary))
        .child(
            PageHeader::from_spec(
                PageHeaderSpec::new("Settings"),
                theme,
            )
        )

        // --- PageHeader: With breadcrumbs ---
        .child(section_label("PAGE HEADER: WITH BREADCRUMBS", text_secondary))
        .child(
            PageHeader::from_spec(
                PageHeaderSpec::new("Button")
                    .with_subtitle("Primary interactive control for triggering actions."),
                theme,
            )
            .with_breadcrumbs(
                div().flex().items_center().gap(px(4.0)).text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Home")
                    .child(div().child("/").mx(px(2.0)))
                    .child("Components")
                    .child(div().child("/").mx(px(2.0)))
                    .child("Primitives")
            )
        )

        // --- PageLoading: Indeterminate ---
        .child(section_label("PAGE LOADING: INDETERMINATE", text_secondary))
        .child(
            PageLoading::from_spec(
                PageLoadingSpec::new()
                    .with_message("Loading components..."),
                theme,
            )
        )

        // --- PageLoading: Determinate ---
        .child(section_label("PAGE LOADING: DETERMINATE (60%)", text_secondary))
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

        // --- PaginationSummary ---
        .child(section_label("PAGINATION SUMMARY", text_secondary))
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
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
