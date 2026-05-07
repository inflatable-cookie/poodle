use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Code, Eyebrow, MetaBar, MetaItem, Pill};
use poodle_specs::{
    CodeSpec, EyebrowSpec, InlineTypographyMode, MetaBarSpec, MetaItemSpec, PillSpec, PillTone,
};

use crate::specimens::specimen_card;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(640.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Header metadata"),
                    theme,
                ))
                .child(
                    MetaBar::from_spec(
                        MetaBarSpec::new().with_aria_label("Project metadata"),
                        theme,
                    )
                    .with_child(
                        MetaItem::from_spec(MetaItemSpec::new().with_label("ID"), theme)
                            .with_value(Code::from_spec(
                                CodeSpec::new()
                                    .with_content("proj_01JX9G9NVV1W3M4P6K8Q8T2D5A")
                                    .with_inline(true),
                                theme,
                            )),
                    )
                    .with_child(Pill::from_spec(
                        PillSpec::new()
                            .with_label("Active")
                            .with_tone(PillTone::Success),
                        theme,
                    ))
                    .with_child(
                        MetaItem::from_spec(MetaItemSpec::new().with_label("Owner"), theme)
                            .with_value(div().child("Clay")),
                    )
                    .with_child(
                        MetaItem::from_spec(MetaItemSpec::new().with_label("Updated"), theme)
                            .with_value(div().child("2 hours ago")),
                    ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("No separators"),
                    theme,
                ))
                .child(
                    MetaBar::from_spec(MetaBarSpec::new().with_show_separators(false), theme)
                        .with_child(
                            MetaItem::from_spec(MetaItemSpec::new().with_label("Type"), theme)
                                .with_value(div().child("Media")),
                        )
                        .with_child(Pill::from_spec(
                            PillSpec::new()
                                .with_label("Public")
                                .with_tone(PillTone::Info),
                            theme,
                        ))
                        .with_child(MetaItem::new(theme).with_value(div().child("1920 x 1080"))),
                ),
        )
}

pub(crate) fn render_meta_item(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .max_w(px(480.0))
        .child(specimen_card(
            "Labeled",
            theme,
            MetaItem::from_spec(MetaItemSpec::new().with_label("Owner"), theme)
                .with_value(div().child("Clay")),
        ))
        .child(specimen_card(
            "Rich Value",
            theme,
            MetaItem::from_spec(MetaItemSpec::new().with_label("State"), theme).with_value(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(Pill::from_spec(
                        PillSpec::new()
                            .with_label("Active")
                            .with_tone(PillTone::Success),
                        theme,
                    ))
                    .child(div().child("Ready for review")),
            ),
        ))
        .child(specimen_card(
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
                    .with_value(div().child("Platform")),
                )
                .child(div().child("today")),
        ))
}
