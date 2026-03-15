use gpui::*;
use pug_gpui_primitives::{MenuSpec, MenuEntry, MenuItemKind};
use pug_gpui_components::PugMenu;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let selected = state.specimens.selected("menu-active");

    let items = vec![
        MenuEntry::new("0", "New File").with_shortcut_label("⌘N"),
        MenuEntry::new("1", "Open").with_shortcut_label("⌘O"),
        MenuEntry::new("2", "Save").with_shortcut_label("⌘S"),
        MenuEntry::new("sep", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("3", "Close").with_shortcut_label("⌘W"),
    ];

    let spec = MenuSpec::new(items);

    div().child(
        PugMenu::new(spec, theme)
            .with_id("specimen-menu")
            .with_selected(format!("{}", selected))
    )
}
