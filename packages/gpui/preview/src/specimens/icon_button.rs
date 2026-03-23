use gpui::*;
use gpui::prelude::FluentBuilder;
use pug_adapter::ThemeProvider;
use pug_primitives::{ButtonVariant, ControlSize, IconButtonSpec};
use pug_gpui_components::IconButton;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let last_clicked = state.specimens.text.get("icon-btn-last")
        .cloned()
        .unwrap_or_default();

    div().flex().flex_col().gap(px(16.0))
        // --- Variants ---
        .child(section_label("VARIANTS", text_secondary))
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
        // --- Danger Tone ---
        .child(section_label("DANGER TONE", text_secondary))
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
        // --- Sizes ---
        .child(section_label("SIZES", text_secondary))
        .child(
            div().flex().gap(px(8.0)).items_center()
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
        )
        // --- States ---
        .child(section_label("STATES", text_secondary))
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
        // --- Click feedback ---
        .when(!last_clicked.is_empty(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(format!("Last action: {}", last_clicked))
            )
        })
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
