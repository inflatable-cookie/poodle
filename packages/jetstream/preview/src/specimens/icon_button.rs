//! IconButton specimen — icon-only command triggers.
//!
//! Full contract-state coverage: variant × tone matrix (default/danger/success
//! across ghost/primary/secondary), all five sizes (xs–xl), pressed/active,
//! loading (spinner), and disabled. Every example is a real `js_icon_button`
//! resolving all visuals from tokens.

use crate::compat::js_icon_button;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ButtonTone, ButtonVariant, ControlSize, IconButtonSpec};

/// One icon button with variant + tone + icon + accessible name.
fn ib(
    theme: &JetstreamThemeProvider,
    variant: ButtonVariant,
    tone: ButtonTone,
    icon: &str,
    aria: &str,
) -> El {
    js_icon_button(
        &IconButtonSpec::new()
            .with_variant(variant)
            .with_tone(tone)
            .with_icon(icon)
            .with_aria_label(aria),
        theme,
    )
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // ── Variants (ghost / primary / secondary) ──
        .child(group(
            "Variants (ghost / primary / secondary)",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(ib(
                    theme,
                    ButtonVariant::Ghost,
                    ButtonTone::Default,
                    "x",
                    "Close",
                ))
                .child(ib(
                    theme,
                    ButtonVariant::Primary,
                    ButtonTone::Default,
                    "plus",
                    "Add",
                ))
                .child(ib(
                    theme,
                    ButtonVariant::Secondary,
                    ButtonTone::Default,
                    "settings",
                    "Settings",
                )),
        ))
        // ── Variant × tone: default ──
        .child(group(
            "Default tone",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(ib(
                    theme,
                    ButtonVariant::Ghost,
                    ButtonTone::Default,
                    "search",
                    "Search",
                ))
                .child(ib(
                    theme,
                    ButtonVariant::Primary,
                    ButtonTone::Default,
                    "check",
                    "Confirm",
                ))
                .child(ib(
                    theme,
                    ButtonVariant::Secondary,
                    ButtonTone::Default,
                    "filter",
                    "Filter",
                )),
        ))
        // ── Variant × tone: danger ──
        .child(group(
            "Danger tone (ghost / primary / secondary)",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(ib(
                    theme,
                    ButtonVariant::Ghost,
                    ButtonTone::Danger,
                    "trash-2",
                    "Delete",
                ))
                .child(ib(
                    theme,
                    ButtonVariant::Primary,
                    ButtonTone::Danger,
                    "trash-2",
                    "Delete",
                ))
                .child(ib(
                    theme,
                    ButtonVariant::Secondary,
                    ButtonTone::Danger,
                    "trash-2",
                    "Delete",
                )),
        ))
        // ── Variant × tone: success ──
        .child(group(
            "Success tone (ghost / primary / secondary)",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(ib(
                    theme,
                    ButtonVariant::Ghost,
                    ButtonTone::Success,
                    "check",
                    "Approve",
                ))
                .child(ib(
                    theme,
                    ButtonVariant::Primary,
                    ButtonTone::Success,
                    "check",
                    "Approve",
                ))
                .child(ib(
                    theme,
                    ButtonVariant::Secondary,
                    ButtonTone::Success,
                    "check",
                    "Approve",
                )),
        ))
        // ── Sizes (xs–xl) ──
        .child(group(
            "Sizes (xs–xl)",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(size_btn(theme, ControlSize::Xs))
                .child(size_btn(theme, ControlSize::Sm))
                .child(size_btn(theme, ControlSize::Md))
                .child(size_btn(theme, ControlSize::Lg))
                .child(size_btn(theme, ControlSize::Xl)),
        ))
        // ── Pressed / active (toggle state) ──
        .child(group(
            "Pressed (toggle on)",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(js_icon_button(
                    &IconButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_icon("bold")
                        .with_aria_label("Bold")
                        .with_pressed(true),
                    theme,
                ))
                .child(js_icon_button(
                    &IconButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_icon("map-pin")
                        .with_aria_label("Pin")
                        .with_pressed(true),
                    theme,
                ))
                // resting counterpart for contrast
                .child(js_icon_button(
                    &IconButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_icon("italic")
                        .with_aria_label("Italic")
                        .with_pressed(false),
                    theme,
                )),
        ))
        // ── Loading (spinner replaces glyph) ──
        .child(group(
            "Loading (spinner)",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(js_icon_button(
                    &IconButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_icon("refresh-cw")
                        .with_aria_label("Refresh")
                        .with_loading(true),
                    theme,
                ))
                .child(js_icon_button(
                    &IconButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_icon("refresh-cw")
                        .with_aria_label("Refresh")
                        .with_loading(true),
                    theme,
                )),
        ))
        // ── Disabled ──
        .child(group(
            "Disabled",
            secondary,
            div()
                .flex_row()
                .gap(8.0)
                .items_center()
                .child(js_icon_button(
                    &IconButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_icon("ban")
                        .with_aria_label("Block")
                        .with_disabled(true),
                    theme,
                ))
                .child(js_icon_button(
                    &IconButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_icon("plus")
                        .with_aria_label("Add")
                        .with_disabled(true),
                    theme,
                ))
                .child(js_icon_button(
                    &IconButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_icon("settings")
                        .with_aria_label("Settings")
                        .with_disabled(true),
                    theme,
                )),
        ))
}

fn size_btn(theme: &JetstreamThemeProvider, size: ControlSize) -> El {
    js_icon_button(
        &IconButtonSpec::new()
            .with_variant(ButtonVariant::Secondary)
            .with_icon("star")
            .with_aria_label("Favorite")
            .with_size(size),
        theme,
    )
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
