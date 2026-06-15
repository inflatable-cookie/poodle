use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{InlineListSection, Pill, Text};
use poodle_specs::{InlineListSectionSpec, PillSpec, PillTone, TextSpec};

fn row(name: &str, status: &str, theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(12.0))
        .child(Text::from_spec(TextSpec::new(name), theme))
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

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(
            InlineListSection::from_spec(
                InlineListSectionSpec::new("Versions").with_count("3"),
                theme,
            )
            .item(row("Version 3", "Ready", theme))
            .item(row("Version 2", "Archived", theme))
            .item(row("Version 1", "Archived", theme)),
        )
        .child(InlineListSection::from_spec(
            InlineListSectionSpec::new("Aliases").with_empty_message("No aliases yet."),
            theme,
        ))
        .child(
            InlineListSection::from_spec(
                InlineListSectionSpec::new("References").with_framed(false),
                theme,
            )
            .item(Text::from_spec(TextSpec::new("Version 3"), theme)),
        )
}
