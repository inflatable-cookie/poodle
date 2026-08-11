use crate::node_compat::{Eyebrow, IconButton, InlineListSection, IntoCompatNode, Pill, Text};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment};
use poodle_specs::{
    ButtonVariant, EyebrowSpec, IconButtonSpec, InlineListSectionSpec, PillSpec, PillTone,
    TextSpec, TextWeight,
};

fn row(name: &str, status: &str, theme: &GpuiThemeProvider) -> poodle_node::Node {
    let mut row = poodle_node::Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    row.style.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
    row.style.descriptor.layout.spacing.gap = 12.0;
    row.style.descriptor.layout.width = LayoutSizing::Grow;
    row.style.min_width = Some(0.0);
    row.children = vec![
        Text::node_from_spec(TextSpec::new(name).with_weight(TextWeight::Medium), theme),
        Pill::from_spec(
            PillSpec::new()
                .with_label(status)
                .with_tone(if status == "Ready" {
                    PillTone::Success
                } else {
                    PillTone::Neutral
                }),
            theme,
        )
        .into_compat_node(),
    ];
    row
}

fn group(label: &str, theme: &GpuiThemeProvider, body: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(body)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(20.0))
        // -- Framed with count pill + header action --
        .child(group(
            "Framed — count pill and header action",
            theme,
            InlineListSection::from_spec(
                InlineListSectionSpec::new("Versions").with_count("3"),
                theme,
            )
            .with_action(IconButton::from_spec(
                IconButtonSpec::new()
                    .with_icon("plus")
                    .with_aria_label("Add version")
                    .with_variant(ButtonVariant::Secondary),
                theme,
            ))
            .item(row("Version 3", "Ready", theme))
            .item(row("Version 2", "Archived", theme))
            .item(row("Version 1", "Archived", theme)),
        ))
        // -- Header actions without a count --
        .child(group(
            "Header actions (no count)",
            theme,
            InlineListSection::from_spec(InlineListSectionSpec::new("Usages"), theme)
                .with_action(IconButton::from_spec(
                    IconButtonSpec::new()
                        .with_icon("external-link")
                        .with_aria_label("Open all usages")
                        .with_variant(ButtonVariant::Ghost),
                    theme,
                ))
                .item(row("checkout-flow", "Ready", theme))
                .item(row("billing-portal", "Archived", theme)),
        ))
        // -- Empty state --
        .child(group(
            "Empty state",
            theme,
            InlineListSection::from_spec(
                InlineListSectionSpec::new("Aliases").with_empty_message("No aliases yet."),
                theme,
            ),
        ))
        // -- Unframed (no card) --
        .child(group(
            "Unframed (no card)",
            theme,
            InlineListSection::from_spec(
                InlineListSectionSpec::new("References")
                    .with_count("2")
                    .with_framed(false),
                theme,
            )
            .item(row("Version 3", "Ready", theme))
            .item(row("Version 2", "Archived", theme)),
        ))
}
