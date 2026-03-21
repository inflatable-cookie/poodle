use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::PaginationSummarySpec;
use pug_gpui_components::PaginationSummary;
use pug_primitives::{ButtonSpec, ButtonVariant, ControlSize};
use pug_gpui_components::Button;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    // --- Basic ---
    // title="Components", subtitle="Browse and manage your component library."
    let basic = div().flex().flex_col().gap(px(4.0))
        .child(div().text_lg().font_weight(FontWeight::SEMIBOLD).text_color(color_to_hsla(text_primary)).child("Components"))
        .child(div().text_sm().text_color(color_to_hsla(text_secondary)).child("Browse and manage your component library."));

    // --- With eyebrow and actions ---
    // title="Button", eyebrow="Primitive", subtitle="Primary interactive control for triggering actions.", actions
    let with_eyebrow_actions = div().flex().items_start().justify_between()
        .child(
            div().flex().flex_col().gap(px(2.0))
                .child(div().text_xs().font_weight(FontWeight::MEDIUM).text_color(color_to_hsla(text_secondary)).child("Primitive"))
                .child(div().text_lg().font_weight(FontWeight::SEMIBOLD).text_color(color_to_hsla(text_primary)).child("Button"))
                .child(div().text_sm().text_color(color_to_hsla(text_secondary)).child("Primary interactive control for triggering actions."))
        )
        .child(
            div().flex().gap(px(6.0))
                .child(
                    Button::from_spec(
                        ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("View source").with_size(ControlSize::Sm),
                        theme,
                    ).with_id("ph-source")
                )
                .child(
                    Button::from_spec(
                        ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Edit").with_size(ControlSize::Sm),
                        theme,
                    ).with_id("ph-edit")
                )
        );

    // --- Title only ---
    // title="Settings"
    let title_only = div().child(
        div().text_lg().font_weight(FontWeight::SEMIBOLD).text_color(color_to_hsla(text_primary)).child("Settings")
    );

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("BASIC", text_secondary))
        .child(basic)
        .child(section_label("WITH EYEBROW AND ACTIONS", text_secondary))
        .child(with_eyebrow_actions)
        .child(section_label("TITLE ONLY", text_secondary))
        .child(title_only)
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
