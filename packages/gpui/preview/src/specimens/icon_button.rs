use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{ButtonVariant, ControlSize, IconButtonSpec, EyebrowSpec};
use poodle_gpui_components::{IconButton, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_clicked = state.specimens.text.get("icon-btn-last")
        .cloned()
        .unwrap_or_default();

    div().flex().flex_col().gap(px(24.0))
        // --- Variants ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Variants"), theme))
                .child(
                    div().flex().gap(px(8.0))
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_icon("plus")
                                    .with_aria_label("Add"),
                                theme,
                            )
                            .with_id("add")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert("icon-btn-last".to_string(), "Add".to_string());
                                cx.notify();
                            }))
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_icon("settings")
                                    .with_aria_label("Settings"),
                                theme,
                            )
                            .with_id("settings")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert("icon-btn-last".to_string(), "Settings".to_string());
                                cx.notify();
                            }))
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_icon("x")
                                    .with_aria_label("Close"),
                                theme,
                            )
                            .with_id("close")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.text.insert("icon-btn-last".to_string(), "Close".to_string());
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
                    div().flex().gap(px(8.0))
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_icon("trash-2")
                                    .with_aria_label("Delete"),
                                theme,
                            )
                            .with_id("danger-primary")
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_icon("trash-2")
                                    .with_aria_label("Delete"),
                                theme,
                            )
                            .with_id("danger-secondary")
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_icon("trash-2")
                                    .with_aria_label("Delete"),
                                theme,
                            )
                            .with_id("danger-ghost")
                        )
                )
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().gap(px(8.0)).items_center()
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_icon("star")
                                    .with_size(ControlSize::Xs)
                                    .with_aria_label("Star"),
                                theme,
                            )
                            .with_id("size-xs")
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_icon("star")
                                    .with_size(ControlSize::Sm)
                                    .with_aria_label("Star"),
                                theme,
                            )
                            .with_id("size-sm")
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_icon("star")
                                    .with_size(ControlSize::Md)
                                    .with_aria_label("Star"),
                                theme,
                            )
                            .with_id("size-md")
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_icon("star")
                                    .with_size(ControlSize::Lg)
                                    .with_aria_label("Star"),
                                theme,
                            )
                            .with_id("size-lg")
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_icon("star")
                                    .with_size(ControlSize::Xl)
                                    .with_aria_label("Star"),
                                theme,
                            )
                            .with_id("size-xl")
                        )
                )
        )
        // --- States ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("States"), theme))
                .child(
                    div().flex().gap(px(8.0)).items_center()
                        .child({
                            let pinned = state.specimens.is_on("icon-btn-pinned");
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_icon("map-pin")
                                    .with_pressed(pinned)
                                    .with_aria_label("Pin"),
                                theme,
                            )
                            .with_id("state-pressed")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.toggle("icon-btn-pinned");
                                this.state.specimens.text.insert("icon-btn-last".to_string(), "Pin toggled".to_string());
                                cx.notify();
                            }))
                        })
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_icon("settings")
                                    .with_disabled(true)
                                    .with_aria_label("Settings"),
                                theme,
                            )
                            .with_id("state-disabled")
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_icon("loader")
                                    .with_loading(true)
                                    .with_aria_label("Loading"),
                                theme,
                            )
                            .with_id("state-loading")
                        )
                )
        )
        // --- String name (built-in internals) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("String name (built-in internals)"), theme))
                .child(
                    div().flex().gap(px(8.0))
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_icon("plus")
                                    .with_aria_label("Add"),
                                theme,
                            ).with_id("str-plus")
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_icon("search")
                                    .with_aria_label("Search"),
                                theme,
                            ).with_id("str-search")
                        )
                        .child(
                            IconButton::from_spec(
                                IconButtonSpec::new()
                                    .with_variant(ButtonVariant::Ghost)
                                    .with_icon("x")
                                    .with_aria_label("Close"),
                                theme,
                            ).with_id("str-close")
                        )
                )
        )
        // --- Click feedback ---
        .when(!last_clicked.is_empty(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(format!("Last action: {}", last_clicked))
            )
        })
}
