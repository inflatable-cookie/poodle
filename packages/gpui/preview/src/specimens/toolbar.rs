use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{
    ButtonSpec, ButtonVariant, ControlSize, SeparatorSpec, SeparatorOrientation,
    RuleTone, ToolbarSpec,
};
use pug_gpui_components::{PugButton, PugSeparator, PugToolbar};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Horizontal (default) ---
        .child(section_label("HORIZONTAL (DEFAULT)", text_secondary))
        .child(
            PugToolbar::new(
                ToolbarSpec::new()
                    .with_aria_label("Formatting toolbar"),
                theme,
            )
            .child(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Sm)
                        .with_label("B"),
                    theme,
                )
                .with_id("toolbar-bold")
            )
            .child(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Sm)
                        .with_label("I"),
                    theme,
                )
                .with_id("toolbar-italic")
            )
            .child(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Sm)
                        .with_label("U"),
                    theme,
                )
                .with_id("toolbar-underline")
            )
            .child(
                PugSeparator::new(
                    SeparatorSpec::new()
                        .with_orientation(SeparatorOrientation::Vertical)
                        .with_tone(RuleTone::Subtle),
                    theme,
                )
            )
            .child(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Sm)
                        .with_label("\u{2190}"),
                    theme,
                )
                .with_id("toolbar-align-left")
            )
            .child(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Sm)
                        .with_label("\u{2194}"),
                    theme,
                )
                .with_id("toolbar-align-center")
            )
            .child(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Sm)
                        .with_label("\u{2192}"),
                    theme,
                )
                .with_id("toolbar-align-right")
            )
        )
        // --- With primary action ---
        .child(section_label("WITH PRIMARY ACTION", text_secondary))
        .child(
            PugToolbar::new(
                ToolbarSpec::new()
                    .with_aria_label("Actions toolbar"),
                theme,
            )
            .child(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_size(ControlSize::Sm)
                        .with_label("Discard"),
                    theme,
                )
                .with_id("toolbar-discard")
            )
            .child(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_size(ControlSize::Sm)
                        .with_label("Save draft"),
                    theme,
                )
                .with_id("toolbar-save-draft")
            )
            .child(
                PugSeparator::new(
                    SeparatorSpec::new()
                        .with_orientation(SeparatorOrientation::Vertical)
                        .with_tone(RuleTone::Subtle),
                    theme,
                )
            )
            .child(
                PugButton::new(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_size(ControlSize::Sm)
                        .with_label("Publish"),
                    theme,
                )
                .with_id("toolbar-publish")
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
