use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::TextLink;
use poodle_specs::{TextLinkSpec, TextLinkTone};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_primary = color_to_hsla(theme.resolve_color("color.text.primary"));
    let text_secondary = color_to_hsla(theme.resolve_color("color.text.secondary"));

    let group = |title: &str| {
        div()
            .text_xs()
            .text_color(text_secondary)
            .child(title.to_string())
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // ── Inline prose ──
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(group("Inline prose"))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .gap(px(4.0))
                        .text_sm()
                        .text_color(text_primary)
                        .child("Review the ".to_string())
                        .child(TextLink::from_spec(
                            TextLinkSpec::new("component contract").with_href("#contract"),
                            theme,
                        ))
                        .child(" before wiring the production route.".to_string()),
                ),
        )
        // ── Tones ──
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(group("Tones"))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(16.0))
                        .child(TextLink::from_spec(
                            TextLinkSpec::new("Accent link").with_href("#accent"),
                            theme,
                        ))
                        .child(TextLink::from_spec(
                            TextLinkSpec::new("Secondary link")
                                .with_href("#secondary")
                                .with_tone(TextLinkTone::Secondary),
                            theme,
                        ))
                        .child(TextLink::from_spec(
                            TextLinkSpec::new("Inherited link")
                                .with_href("#inherit")
                                .with_tone(TextLinkTone::Inherit),
                            theme,
                        )),
                ),
        )
        // ── Button action (no href) ──
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(group("Button action"))
                .child(TextLink::from_spec(
                    TextLinkSpec::new("Open inline action"),
                    theme,
                )),
        )
        // ── Disabled (anchor + button paths) ──
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(group("Disabled"))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(16.0))
                        .child(TextLink::from_spec(
                            TextLinkSpec::new("Disabled anchor")
                                .with_href("#disabled")
                                .with_disabled(true),
                            theme,
                        ))
                        .child(TextLink::from_spec(
                            TextLinkSpec::new("Disabled action").with_disabled(true),
                            theme,
                        )),
                ),
        )
}
