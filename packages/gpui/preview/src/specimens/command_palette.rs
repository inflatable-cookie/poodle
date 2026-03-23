use gpui::*;
use gpui::prelude::FluentBuilder;
use flint_adapter::ThemeProvider;
use flint_composites::{CommandPaletteSpec, CommandActionItem};
use flint_primitives::EyebrowSpec;
use flint_gpui_components::{CommandPalette, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

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

    let query = state.specimens.text.get("cmd-palette-query")
        .cloned()
        .unwrap_or_default();
    let last_executed = state.specimens.text.get("cmd-palette-executed")
        .cloned();

    let mut spec = CommandPaletteSpec::new(actions);
    if !query.is_empty() {
        spec = spec.with_query(&query);
    }

    div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Command Palette"), theme))
                .child(
                    div().w(px(480.0)).child(
                        CommandPalette::from_spec(spec, theme)
                            .with_id("cmd-palette")
                            .on_select(cx.listener(|this, val: &str, _w, cx| {
                                this.state.specimens.text.insert(
                                    "cmd-palette-executed".to_string(),
                                    val.to_string(),
                                );
                                cx.notify();
                            }))
                            .on_query_change(cx.listener(|this, val: &str, _w, cx| {
                                this.state.specimens.text.insert(
                                    "cmd-palette-query".to_string(),
                                    val.to_string(),
                                );
                                cx.notify();
                            }))
                    )
                )
        )
        .when(last_executed.is_some(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(format!("Last executed: {}", last_executed.as_deref().unwrap_or("")))
            )
        })
}
