use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{ButtonSpec, ButtonTone, ButtonVariant, ControlSize, EyebrowSpec};
use poodle_gpui_components::{Button, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let click_count = state.specimens.count("btn-clicks");
    let last_clicked = state.specimens.text.get("btn-last-clicked")
        .cloned()
        .unwrap_or_default();

    div().flex().flex_col().gap(px(24.0))
        // --- Variants ---
        .child(
            div().flex().flex_col().gap(px(10.0))
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
            div().flex().flex_col().gap(px(10.0))
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
            div().flex().flex_col().gap(px(10.0))
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
            div().flex().flex_col().gap(px(10.0))
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
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().gap(px(8.0)).flex_wrap().items_end()
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_size(ControlSize::Sm)
                                    .with_label("Small"),
                                theme,
                            )
                            .with_id("size-sm")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_size(ControlSize::Md)
                                    .with_label("Medium"),
                                theme,
                            )
                            .with_id("size-md")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_size(ControlSize::Lg)
                                    .with_label("Large"),
                                theme,
                            )
                            .with_id("size-lg")
                        )
                )
        )
        // --- States ---
        .child(
            div().flex().flex_col().gap(px(10.0))
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
}
