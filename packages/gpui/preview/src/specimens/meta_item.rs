use crate::node_compat::{Code, CompatRow, Eyebrow, MetaItem, Pill};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CodeSpec, EyebrowSpec, InlineTypographyMode, MetaItemSpec, PillSpec, PillTone};

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(480.0))
        .child(group(
            "Labeled",
            theme,
            MetaItem::from_spec(MetaItemSpec::new().with_label("Owner"), theme).with_value("Clay"),
        ))
        .child(group(
            "Rich value",
            theme,
            MetaItem::from_spec(MetaItemSpec::new().with_label("State"), theme).with_value(
                CompatRow::new()
                    .gap(6.0)
                    .child(Pill::from_spec(
                        PillSpec::new()
                            .with_label("Active")
                            .with_tone(PillTone::Success),
                        theme,
                    ))
                    .child("Ready for review"),
            ),
        ))
        .child(group(
            "Code value",
            theme,
            MetaItem::from_spec(MetaItemSpec::new().with_label("ID"), theme).with_value(
                Code::from_spec(
                    CodeSpec::new()
                        .with_content("proj_01JX9G9NVV1W3M4P6K8Q8T2D5A")
                        .with_inline(true),
                    theme,
                ),
            ),
        ))
        .child(group(
            "Inherit typography",
            theme,
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .text_size(px(20.0))
                .child(div().child("Owner"))
                .child(
                    MetaItem::from_spec(
                        MetaItemSpec::new()
                            .with_label("Team")
                            .with_typography(InlineTypographyMode::Inherit),
                        theme,
                    )
                    .with_value("Platform"),
                )
                .child(div().child("today")),
        ))
}
