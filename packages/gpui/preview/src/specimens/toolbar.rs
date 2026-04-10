use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, EyebrowSpec, SeparatorSpec,
    SeparatorOrientation, RuleTone, ToolbarSpec,
};
use poodle_gpui_components::{Button, Eyebrow, Separator, Toolbar};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_action = state.specimens.text.get("toolbar-last")
        .cloned()
        .unwrap_or_default();

    div().flex().flex_col().gap(px(24.0))
        // --- Horizontal (default) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Horizontal (default)"), theme))
                .child(
                    Toolbar::from_spec(
                        ToolbarSpec::new()
                            .with_aria_label("Formatting toolbar"),
                        theme,
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(ControlSize::Sm)
                                .with_label("B"),
                            theme,
                        )
                        .with_id("toolbar-bold")
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.text.insert("toolbar-last".to_string(), "Bold".to_string());
                            cx.notify();
                        }))
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(ControlSize::Sm)
                                .with_label("I"),
                            theme,
                        )
                        .with_id("toolbar-italic")
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.text.insert("toolbar-last".to_string(), "Italic".to_string());
                            cx.notify();
                        }))
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(ControlSize::Sm)
                                .with_label("U"),
                            theme,
                        )
                        .with_id("toolbar-underline")
                    )
                    .child(
                        Separator::from_spec(
                            SeparatorSpec::new()
                                .with_orientation(SeparatorOrientation::Vertical)
                                .with_tone(RuleTone::Subtle),
                            theme,
                        )
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(ControlSize::Sm)
                                .with_label("\u{2190}"),
                            theme,
                        )
                        .with_id("toolbar-align-left")
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(ControlSize::Sm)
                                .with_label("\u{2194}"),
                            theme,
                        )
                        .with_id("toolbar-align-center")
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(ControlSize::Sm)
                                .with_label("\u{2192}"),
                            theme,
                        )
                        .with_id("toolbar-align-right")
                    )
                )
        )
        // --- With primary action ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With primary action"), theme))
                .child(
                    Toolbar::from_spec(
                        ToolbarSpec::new()
                            .with_aria_label("Actions toolbar"),
                        theme,
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_size(ControlSize::Sm)
                                .with_label("Discard"),
                            theme,
                        )
                        .with_id("toolbar-discard")
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_size(ControlSize::Sm)
                                .with_label("Save draft"),
                            theme,
                        )
                        .with_id("toolbar-save-draft")
                    )
                    .child(
                        Separator::from_spec(
                            SeparatorSpec::new()
                                .with_orientation(SeparatorOrientation::Vertical)
                                .with_tone(RuleTone::Subtle),
                            theme,
                        )
                    )
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Primary)
                                .with_size(ControlSize::Sm)
                                .with_label("Publish"),
                            theme,
                        )
                        .with_id("toolbar-publish")
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.text.insert("toolbar-last".to_string(), "Publish".to_string());
                            cx.notify();
                        }))
                    )
                )
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().flex_col().gap(px(12.0))
                        .child(
                            Toolbar::from_spec(
                                ToolbarSpec::new()
                                    .with_size(ControlSize::Xs)
                                    .with_aria_label("Xs toolbar"),
                                theme,
                            )
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Xs).with_label("B"),
                                theme,
                            ).with_id("toolbar-size-xs-b"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Xs).with_label("I"),
                                theme,
                            ).with_id("toolbar-size-xs-i"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Xs).with_label("U"),
                                theme,
                            ).with_id("toolbar-size-xs-u"))
                        )
                        .child(
                            Toolbar::from_spec(
                                ToolbarSpec::new()
                                    .with_size(ControlSize::Sm)
                                    .with_aria_label("Sm toolbar"),
                                theme,
                            )
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("B"),
                                theme,
                            ).with_id("toolbar-size-sm-b"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("I"),
                                theme,
                            ).with_id("toolbar-size-sm-i"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("U"),
                                theme,
                            ).with_id("toolbar-size-sm-u"))
                        )
                        .child(
                            Toolbar::from_spec(
                                ToolbarSpec::new()
                                    .with_size(ControlSize::Md)
                                    .with_aria_label("Md toolbar"),
                                theme,
                            )
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Md).with_label("B"),
                                theme,
                            ).with_id("toolbar-size-md-b"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Md).with_label("I"),
                                theme,
                            ).with_id("toolbar-size-md-i"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Md).with_label("U"),
                                theme,
                            ).with_id("toolbar-size-md-u"))
                        )
                        .child(
                            Toolbar::from_spec(
                                ToolbarSpec::new()
                                    .with_size(ControlSize::Lg)
                                    .with_aria_label("Lg toolbar"),
                                theme,
                            )
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Lg).with_label("B"),
                                theme,
                            ).with_id("toolbar-size-lg-b"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Lg).with_label("I"),
                                theme,
                            ).with_id("toolbar-size-lg-i"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Lg).with_label("U"),
                                theme,
                            ).with_id("toolbar-size-lg-u"))
                        )
                        .child(
                            Toolbar::from_spec(
                                ToolbarSpec::new()
                                    .with_size(ControlSize::Xl)
                                    .with_aria_label("Xl toolbar"),
                                theme,
                            )
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Xl).with_label("B"),
                                theme,
                            ).with_id("toolbar-size-xl-b"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Xl).with_label("I"),
                                theme,
                            ).with_id("toolbar-size-xl-i"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Xl).with_label("U"),
                                theme,
                            ).with_id("toolbar-size-xl-u"))
                        )
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            Toolbar::from_spec(
                                ToolbarSpec::new()
                                    .with_density(ControlDensity::Compact)
                                    .with_aria_label("Compact toolbar"),
                                theme,
                            )
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("B"),
                                theme,
                            ).with_id("toolbar-density-compact-b"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("I"),
                                theme,
                            ).with_id("toolbar-density-compact-i"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("U"),
                                theme,
                            ).with_id("toolbar-density-compact-u"))
                        )
                        .child(
                            Toolbar::from_spec(
                                ToolbarSpec::new()
                                    .with_density(ControlDensity::Default)
                                    .with_aria_label("Default toolbar"),
                                theme,
                            )
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("B"),
                                theme,
                            ).with_id("toolbar-density-default-b"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("I"),
                                theme,
                            ).with_id("toolbar-density-default-i"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("U"),
                                theme,
                            ).with_id("toolbar-density-default-u"))
                        )
                        .child(
                            Toolbar::from_spec(
                                ToolbarSpec::new()
                                    .with_density(ControlDensity::Comfortable)
                                    .with_aria_label("Comfortable toolbar"),
                                theme,
                            )
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("B"),
                                theme,
                            ).with_id("toolbar-density-comfortable-b"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("I"),
                                theme,
                            ).with_id("toolbar-density-comfortable-i"))
                            .child(Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("U"),
                                theme,
                            ).with_id("toolbar-density-comfortable-u"))
                        )
                )
        )
        // --- Last action feedback ---
        .when(!last_action.is_empty(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(format!("Last action: {}", last_action))
            )
        })
}
