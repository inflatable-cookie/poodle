use gpui::*;
use pug_gpui_workstation::{CommandPaletteSpec, CommandActionItem};
use pug_gpui_components::PugCommandPalette;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let actions = vec![
        CommandActionItem::new("open-file", "Open File")
            .with_group("File")
            .with_shortcut("\u{2318}P"),
        CommandActionItem::new("new-window", "New Window")
            .with_group("File")
            .with_shortcut("\u{21E7}\u{2318}N"),
        CommandActionItem::new("toggle-sidebar", "Toggle Sidebar")
            .with_group("View")
            .with_shortcut("\u{2318}B"),
        CommandActionItem::new("find-replace", "Find and Replace")
            .with_group("Edit")
            .with_shortcut("\u{2318}H")
            .with_badge("new"),
    ];

    let spec = CommandPaletteSpec::new(actions)
        .with_active_action_id("open-file");

    div().w(px(480.0)).child(
        PugCommandPalette::new(spec, theme)
            .with_id("cmd-palette")
    )
}
