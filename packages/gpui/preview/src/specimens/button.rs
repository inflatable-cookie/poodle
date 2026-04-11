use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant, EyebrowSpec};
use poodle_gpui_components::{Button, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let click_count = state.specimens.count("btn-clicks");
    let last_clicked = state.specimens.text.get("btn-last-clicked")
        .cloned()
        .unwrap_or_default();

    let examples = div().flex().flex_col().gap(px(24.0))
        // --- Variants ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Variants"), theme))
                .child(
                    div().flex().gap(px(8.0)).flex_wrap()
                        .child(
                            Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Primary"),
                                theme,
                            )
                            .with_id("primary")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.increment("btn-clicks");
                                this.state.specimens.text.insert("btn-last-clicked".to_string(), "Primary".to_string());
                                cx.notify();
                            }))
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Secondary"),
                                theme,
                            )
                            .with_id("secondary")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.increment("btn-clicks");
                                this.state.specimens.text.insert("btn-last-clicked".to_string(), "Secondary".to_string());
                                cx.notify();
                            }))
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Ghost"),
                                theme,
                            )
                            .with_id("ghost")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.increment("btn-clicks");
                                this.state.specimens.text.insert("btn-last-clicked".to_string(), "Ghost".to_string());
                                cx.notify();
                            }))
                        )
                )
        )
        // --- Danger tone ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Danger tone"), theme))
                .child(
                    div().flex().gap(px(8.0)).flex_wrap()
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_tone(ButtonTone::Danger)
                                    .with_label("Danger primary"),
                                theme,
                            )
                            .with_id("danger-primary")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_tone(ButtonTone::Danger)
                                    .with_label("Danger secondary"),
                                theme,
                            )
                            .with_id("danger-secondary")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_tone(ButtonTone::Danger)
                                    .with_label("Danger ghost"),
                                theme,
                            )
                            .with_id("danger-ghost")
                        )
                )
        )
        // --- With icons ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With icons"), theme))
                .child(
                    div().flex().gap(px(8.0)).flex_wrap()
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_leading_icon("plus")
                                    .with_label("Create"),
                                theme,
                            )
                            .with_id("icon-create")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_trailing_icon("external-link")
                                    .with_label("Open"),
                                theme,
                            )
                            .with_id("icon-open")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_leading_icon("save")
                                    .with_trailing_icon("check")
                                    .with_label("Save"),
                                theme,
                            )
                            .with_id("icon-save")
                        )
                )
        )
        // --- With chevron ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With chevron"), theme))
                .child(
                    div().flex().gap(px(8.0)).flex_wrap()
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_chevron(true)
                                    .with_label("Options"),
                                theme,
                            )
                            .with_id("chevron-options")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_chevron(true)
                                    .with_label("Actions"),
                                theme,
                            )
                            .with_id("chevron-actions")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_leading_icon("filter")
                                    .with_chevron(true)
                                    .with_label("Filter"),
                                theme,
                            )
                            .with_id("chevron-filter")
                        )
                )
        )
        // (Sizes and Densities moved into the SpecimenLayout tabs below.)
        // --- States ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("States"), theme))
                .child(
                    div().flex().gap(px(8.0)).flex_wrap()
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_disabled(true)
                                    .with_label("Disabled"),
                                theme,
                            )
                            .with_id("disabled")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_loading(true)
                                    .with_label("Loading"),
                                theme,
                            )
                            .with_id("loading")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_disabled(true)
                                    .with_label("Disabled secondary"),
                                theme,
                            )
                            .with_id("disabled-secondary")
                        )
                )
        )
        // --- Toggle (pressed state) ---
        .child({
            let bold = state.specimens.is_on("btn-bold");
            let italic = state.specimens.is_on("btn-italic");
            let underline = !state.specimens.is_on("btn-underline-off");
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Toggle (pressed state)"), theme))
                .child(
                    div().flex().gap(px(8.0))
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_leading_icon("bold")
                                    .with_pressed(bold)
                                    .with_label("B"),
                                theme,
                            )
                            .with_id("btn-bold")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.toggle("btn-bold");
                                cx.notify();
                            }))
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_leading_icon("italic")
                                    .with_pressed(italic)
                                    .with_label("I"),
                                theme,
                            )
                            .with_id("btn-italic")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.toggle("btn-italic");
                                cx.notify();
                            }))
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_leading_icon("underline")
                                    .with_pressed(underline)
                                    .with_label("U"),
                                theme,
                            )
                            .with_id("btn-underline")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.toggle("btn-underline-off");
                                cx.notify();
                            }))
                        )
                )
        })
        // --- Form overrides ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Form overrides"), theme))
                .child(
                    div().flex().gap(px(8.0))
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("Save"),
                                theme,
                            )
                            .with_id("form-save")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert("btn-last".to_string(), "Save (submit)".to_string());
                                cx.notify();
                            }))
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Publish"),
                                theme,
                            )
                            .with_id("form-publish")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert("btn-last".to_string(), "Publish (formaction)".to_string());
                                cx.notify();
                            }))
                        )
                )
        )
        // --- Click counter ---
        .child(
            div().flex().flex_col().gap(px(2.0))
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child(format!("Clicks: {}", click_count))
                )
                .when(!last_clicked.is_empty(), |d| {
                    d.child(
                        div().text_sm().text_color(color_to_hsla(text_secondary))
                            .child(format!("Last clicked: {}", last_clicked))
                    )
                })
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "button",
        examples,
        // Sizes pane: one button per size.
        |size, theme: &GpuiThemeProvider| {
            Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_size(size)
                    .with_label("Enabled"),
                theme,
            )
            .with_id(format!("specimen-size-{:?}", size))
            .into_any_element()
        },
        // Densities pane: one button per density.
        |density, theme: &GpuiThemeProvider| {
            Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Toggle"),
                theme,
            )
            .with_id(format!("specimen-density-{:?}", density))
            .density(density)
            .into_any_element()
        },
    )
}
