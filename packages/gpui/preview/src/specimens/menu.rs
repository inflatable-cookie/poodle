use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{MenuSpec, MenuEntry, MenuItemKind};
use pug_gpui_components::PugMenu;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // --- With shortcuts ---
    // Contract: New file ⌘N, Open… ⌘O, Save ⌘S, separator, Export as PDF (no shortcut), Print… ⌘P (disabled)
    let file_items = vec![
        MenuEntry::new("new", "New file").with_shortcut_label("⌘N"),
        MenuEntry::new("open", "Open…").with_shortcut_label("⌘O"),
        MenuEntry::new("save", "Save").with_shortcut_label("⌘S"),
        MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("export", "Export as PDF"),
        MenuEntry::new("print", "Print…").with_shortcut_label("⌘P").with_disabled(true),
    ];

    let file_spec = MenuSpec::new(file_items)
        .with_default_open(true)
        .with_aria_label("File menu");

    // --- With checkboxes ---
    // Contract: Dark mode (checked), Notifications (unchecked), separator, Settings…
    let settings_items = vec![
        MenuEntry::new("theme", "Dark mode").with_kind(MenuItemKind::Checkbox).with_checked(true),
        MenuEntry::new("notifications", "Notifications").with_kind(MenuItemKind::Checkbox),
        MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("settings", "Settings…"),
    ];

    let settings_spec = MenuSpec::new(settings_items)
        .with_default_open(true)
        .with_aria_label("Settings menu");

    div().flex().flex_col().gap(px(16.0))
        // With shortcuts
        .child(section_label("WITH SHORTCUTS", text_secondary))
        .child(
            PugMenu::new(file_spec, theme)
                .with_id("specimen-menu-shortcuts")
        )
        // With checkboxes
        .child(section_label("WITH CHECKBOXES", text_secondary))
        .child(
            PugMenu::new(settings_spec, theme)
                .with_id("specimen-menu-checkboxes")
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
