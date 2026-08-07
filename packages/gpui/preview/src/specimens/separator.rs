use crate::node_compat::{Eyebrow, Separator};
use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, RuleTone, SeparatorOrientation, SeparatorSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

    let body = |s: &str| {
        div()
            .text_sm()
            .text_color(color_to_hsla(text_secondary))
            .child(s.to_string())
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Horizontal (default) ---
        .child(group(
            theme,
            "Horizontal (default)",
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(body("Content above"))
                .child(Separator::from_spec(SeparatorSpec::new(), theme))
                .child(body("Content below")),
        ))
        // --- Tone: subtle (default) vs default ---
        .child(group(
            theme,
            "Tone emphasis",
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(body("Subtle (default)"))
                .child(Separator::from_spec(
                    SeparatorSpec::new().with_tone(RuleTone::Subtle),
                    theme,
                ))
                .child(body("Default"))
                .child(Separator::from_spec(
                    SeparatorSpec::new().with_tone(RuleTone::Default),
                    theme,
                ))
                .child(body("Below")),
        ))
        // --- Vertical ---
        .child(group(
            theme,
            "Vertical",
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .h(px(32.0))
                .child(body("Left"))
                .child(Separator::from_spec(
                    SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical),
                    theme,
                ))
                .child(body("Center"))
                .child(Separator::from_spec(
                    SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical),
                    theme,
                ))
                .child(body("Right")),
        ))
        // --- Vertical, default tone ---
        .child(group(
            theme,
            "Vertical (default tone)",
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .h(px(32.0))
                .child(body("Left"))
                .child(Separator::from_spec(
                    SeparatorSpec::new()
                        .with_orientation(SeparatorOrientation::Vertical)
                        .with_tone(RuleTone::Default),
                    theme,
                ))
                .child(body("Right")),
        ))
        // --- Decorative (aria-hidden) ---
        .child(group(
            theme,
            "Decorative",
            Separator::from_spec(SeparatorSpec::new().with_decorative(true), theme),
        ))
        // --- Semantic (decorative=false → role="separator") ---
        .child(group(
            theme,
            "Semantic (decorative=false)",
            Separator::from_spec(SeparatorSpec::new().with_decorative(false), theme),
        ))
}

fn group(theme: &GpuiThemeProvider, title: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(title),
            theme,
        ))
        .child(content)
}
