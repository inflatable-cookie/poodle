use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{CommandPalette, Eyebrow};
use poodle_specs::{CommandActionItem, CommandPaletteSpec};
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
    let last_executed = state.specimens.text.get("cmd-palette-executed").cloned();

    let mut spec = CommandPaletteSpec::new(actions);
    if !query.is_empty() {
        spec = spec.with_query(&query);
    }

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
                    div().w(px(480.0)).child(
                        CommandPalette::from_spec(spec, theme)
                            .with_id("cmd-palette")
                            .on_select(cx.listener(|this, val: &str, _w, cx| {
                                this.state
                                    .specimens
                                    .text
                                    .insert("cmd-palette-executed".to_string(), val.to_string());
                                cx.notify();
                            }))
                            .on_query_change(cx.listener(|this, val: &str, _w, cx| {
                                this.state
                                    .specimens
                                    .text
                                    .insert("cmd-palette-query".to_string(), val.to_string());
                                cx.notify();
                            })),
                    ),
                ),
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
                                    .with_size(ControlSize::Sm)
                                    .with_density(ControlDensity::Compact),
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
                                    .with_size_role(SemanticControlSizeRole::Prominent)
                                    .with_density(ControlDensity::Compact),
                                    theme,
                                )
                                .with_id("cmd-palette-prominent"),
                            ),
                        ),
                ),
        )
        // --- With title, description, and invocation hint ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With title and invocation hint"),
                    theme,
                ))
                .child(
                    div().w(px(480.0)).child(
                        CommandPalette::from_spec(
                            CommandPaletteSpec::new(vec![
                                CommandActionItem::new("new-doc", "New document")
                                    .with_group("Create")
                                    .with_shortcut("\u{2318}N"),
                                CommandActionItem::new("new-proj", "New project")
                                    .with_group("Create")
                                    .with_shortcut("\u{2318}\u{21E7}N"),
                                CommandActionItem::new("invite", "Invite collaborator")
                                    .with_group("Team"),
                            ])
                            .with_title("Quick actions")
                            .with_description("Search and execute workspace commands.")
                            .with_invocation_hint("Search workspace\u{2026}"),
                            theme,
                        )
                        .with_id("cmd-palette-header"),
                    ),
                ),
        )
        .when(last_executed.is_some(), |d| {
            d.child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child(format!(
                        "Last executed: {}",
                        last_executed.as_deref().unwrap_or("")
                    )),
            )
        })
}
