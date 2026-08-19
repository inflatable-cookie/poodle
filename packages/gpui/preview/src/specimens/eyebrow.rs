use crate::node_compat::Eyebrow;
use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

use crate::app_state::AppState;
use crate::specimens::specimen_axes::EYEBROW_SIZES;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use poodle_specs::{EyebrowElement, EyebrowSize, EyebrowSpacing, EyebrowSpec};

fn group(theme: &GpuiThemeProvider, label: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_primary = theme.resolve_color("color.text.primary");
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            theme,
            "Above a page title",
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Section label"),
                    theme,
                ))
                .child(
                    div()
                        .text_xl()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(color_to_hsla(text_primary))
                        .child("Page Title".to_string()),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(
                            "Eyebrow renders small uppercase text used for categorizing content above headings."
                                .to_string(),
                        ),
                ),
        ))
        .child(group(
            theme,
            "Primitive category",
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Primitive"),
                    theme,
                ))
                .child(
                    div()
                        .text_xl()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(color_to_hsla(text_primary))
                        .child("Button".to_string()),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child("Primary interactive control for triggering actions.".to_string()),
                ),
        ))
        .child(group(
            theme,
            "Composite category",
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Composite"),
                    theme,
                ))
                .child(
                    div()
                        .text_xl()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(color_to_hsla(text_primary))
                        .child("DataTable".to_string()),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(
                            "Feature-rich table with sorting, selection, and pagination."
                                .to_string(),
                        ),
                ),
        ))
        .child(group(
            theme,
            "Semantic heading",
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Semantic section heading")
                        .with_element(EyebrowElement::H3)
                        .with_size(EyebrowSize::Md)
                        .with_spacing(EyebrowSpacing::Bottom),
                    theme,
                ))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(
                            "Eyebrow can render as a heading when it labels a real subsection."
                                .to_string(),
                        ),
                ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "eyebrow",
        examples,
        SpecimenAxes::examples_only()
            .with_named_sizes(EYEBROW_SIZES, |value, theme: &GpuiThemeProvider| {
                let eyebrow_size = match value {
                    "xs" => EyebrowSize::Xs,
                    "sm" => EyebrowSize::Sm,
                    _ => EyebrowSize::Md,
                };
                Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Section label")
                        .with_size(eyebrow_size),
                    theme,
                )
                .into_any_element()
            }),
    )
}
