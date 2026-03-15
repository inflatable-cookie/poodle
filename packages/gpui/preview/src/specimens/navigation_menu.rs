use gpui::*;
use pug_gpui_primitives::{NavigationMenuSpec, NavigationMenuEntry};
use pug_gpui_components::PugNavigationMenu;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let selected = state.specimens.selected("navmenu-active");

    let items = vec![
        NavigationMenuEntry::new("0", "File").with_description("File operations"),
        NavigationMenuEntry::new("1", "Edit").with_description("Edit commands"),
        NavigationMenuEntry::new("2", "View").with_description("View options"),
        NavigationMenuEntry::new("3", "Help").with_description("Help & support"),
    ];

    let spec = NavigationMenuSpec::new(items)
        .with_value(format!("{}", selected));

    div().child(
        PugNavigationMenu::new(spec, theme)
            .with_id("specimen-nav")
    )
}
