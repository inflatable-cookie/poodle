use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{Button, CommandPalette, Eyebrow};
use poodle_specs::{ButtonSpec, CommandActionItem, CommandPaletteSpec};
use poodle_specs::{ControlDensity, ControlSize, EyebrowSpec, SemanticControlSizeRole};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let actions = vec![
        CommandActionItem::new("save", "Save")
            .with_group("File")
            .with_shortcut("\u{2318}S"),
        CommandActionItem::new("open-file", "Open File")
            .with_group("File")
            .with_shortcut("\u{2318}O"),
        CommandActionItem::new("close-tab", "Close Tab")
            .with_group("File")
            .with_shortcut("\u{2318}W"),
        CommandActionItem::new("find-in-files", "Find in Files")
            .with_group("Edit")
            .with_shortcut("\u{21E7}\u{2318}F"),
        CommandActionItem::new("find-and-replace", "Find and Replace")
            .with_group("Edit")
            .with_shortcut("\u{2318}H"),
        CommandActionItem::new("toggle-terminal", "Toggle Terminal")
            .with_group("View")
            .with_shortcut("\u{2318}`"),
        CommandActionItem::new("toggle-sidebar", "Toggle Sidebar")
            .with_group("View")
            .with_shortcut("\u{2318}B"),
    ];

    let query = state
        .specimens
        .text
        .get("cmd-palette-query")
        .cloned()
        .unwrap_or_default();
    let is_open = state.specimens.is_on("cmd-palette-open");
    let compact_open = state.specimens.is_on("cmd-palette-compact-open");

    let mut spec = CommandPaletteSpec::new(actions);
    if !query.is_empty() {
        spec = spec.with_query(&query);
    }
    spec = spec.with_open(is_open);

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Command Palette"),
                    theme,
                ))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(
                            "Click below to open the palette. Close with Escape, click outside, or the X button.",
                        ),
                )
                .child(
                    Button::from_spec(ButtonSpec::new().with_label("Open Command Palette"), theme)
                        .with_id("cmd-palette-open")
                        .on_click(cx.listener(|this, _event: &ClickEvent, _window, cx| {
                            if !this.state.specimens.is_on("cmd-palette-open") {
                                this.state.specimens.toggle("cmd-palette-open");
                            }
                            cx.notify();
                        })),
                )
                .child(div().w(px(480.0)).child(
                    CommandPalette::from_spec(spec, theme)
                        .with_id("cmd-palette")
                        .on_select(cx.listener(|this, val: &str, _w, cx| {
                            this.state
                                .specimens
                                .text
                                .insert("cmd-palette-query".to_string(), val.to_string());
                            cx.notify();
                        }))
                        .on_query_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state
                                .specimens
                                .text
                                .insert("cmd-palette-query".to_string(), val.to_string());
                            cx.notify();
                        })),
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Semantic presentation"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(
                            Button::from_spec(ButtonSpec::new().with_label("Open compact palette"), theme)
                                .with_id("cmd-palette-compact-open")
                                .on_click(cx.listener(|this, _event: &ClickEvent, _window, cx| {
                                    if !this.state.specimens.is_on("cmd-palette-compact-open") {
                                        this.state.specimens.toggle("cmd-palette-compact-open");
                                    }
                                    cx.notify();
                                })),
                        )
                        .child(
                            div().w(px(420.0)).child(
                                CommandPalette::from_spec(
                                    CommandPaletteSpec::new(vec![
                                        CommandActionItem::new("save", "Save")
                                            .with_group("File")
                                            .with_shortcut("\u{2318}S"),
                                        CommandActionItem::new("open", "Open File")
                                            .with_group("File")
                                            .with_shortcut("\u{2318}O"),
                                    ])
                                    .with_open(compact_open)
                                    .with_size(ControlSize::Sm)
                                    .with_density(ControlDensity::Compact)
                                    .with_invocation_hint("Cmd+K"),
                                    theme,
                                )
                                .with_id("cmd-palette-compact"),
                            ),
                        )
                        .child(
                            div().w(px(480.0)).child(
                                CommandPalette::from_spec(
                                    CommandPaletteSpec::new(vec![CommandActionItem::new(
                                        "save", "Save",
                                    )
                                    .with_group("File")
                                    .with_shortcut("\u{2318}S")])
                                    .with_open(false)
                                    .with_size_role(SemanticControlSizeRole::Prominent)
                                    .with_density(ControlDensity::Compact),
                                    theme,
                                )
                                .with_id("cmd-palette-prominent"),
                            ),
                        ),
                ),
        )
}
