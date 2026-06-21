use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, IconButton, InlineListSection, Pill, Text};
use poodle_specs::{
    ButtonVariant, EyebrowSpec, IconButtonSpec, InlineListSectionSpec, PillSpec, PillTone, TextSpec,
    TextWeight,
};

fn row(name: &str, status: &str, theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(12.0))
        .w_full()
        .min_w_0()
        .child(Text::from_spec(
            TextSpec::new(name).with_weight(TextWeight::Medium),
            theme,
        ))
        .child(Pill::from_spec(
            PillSpec::new()
                .with_label(status)
                .with_tone(if status == "Ready" {
                    PillTone::Success
                } else {
                    PillTone::Neutral
                }),
            theme,
        ))
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
