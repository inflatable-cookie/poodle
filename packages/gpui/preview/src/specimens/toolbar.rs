use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlSize, EyebrowSpec, SeparatorSpec,
    SeparatorOrientation, RuleTone, ToolbarSpec,
};
use poodle_gpui_components::{Button, Eyebrow, Separator, Toolbar};
use poodle_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_action = state.specimens.text.get("toolbar-last")
        .cloned()
        .unwrap_or_default();

    let examples = div().flex().flex_col().gap(px(24.0))
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
        // --- Last action feedback ---
        .when(!last_action.is_empty(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(format!("Last action: {}", last_action))
            )
        })
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "toolbar",
        examples,
        |size, theme: &GpuiThemeProvider| {
            Toolbar::from_spec(
                ToolbarSpec::new().with_size(size).with_aria_label("Toolbar"),
                theme,
            )
            .child(Button::from_spec(
                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(size).with_label("B"),
                theme,
            ).with_id(format!("specimen-toolbar-size-{:?}-b", size)))
            .child(Button::from_spec(
                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(size).with_label("I"),
                theme,
            ).with_id(format!("specimen-toolbar-size-{:?}-i", size)))
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            Toolbar::from_spec(
                ToolbarSpec::new().with_density(density).with_aria_label("Toolbar"),
                theme,
            )
            .child(Button::from_spec(
                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("B"),
                theme,
            ).with_id(format!("specimen-toolbar-density-{:?}-b", density)))
            .child(Button::from_spec(
                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_size(ControlSize::Sm).with_label("I"),
                theme,
            ).with_id(format!("specimen-toolbar-density-{:?}-i", density)))
            .into_any_element()
        },
    )
}
