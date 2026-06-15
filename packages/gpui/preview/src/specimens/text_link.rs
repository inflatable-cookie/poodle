use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::TextLink;
use poodle_specs::{TextLinkSpec, TextLinkTone};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(
            div()
                .flex()
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
        )
        .child(TextLink::from_spec(
            TextLinkSpec::new("Button action"),
            theme,
        ))
        .child(TextLink::from_spec(
            TextLinkSpec::new("Disabled action").with_disabled(true),
            theme,
        ))
}
