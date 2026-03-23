use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{MenubarSpec, MenubarEntry, MenuEntry, MenuItemKind};
use pug_gpui_components::Menubar;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    // Track which menu is open and last selected action
    let active_menu = state.specimens.text.get("menubar-active").cloned();
    let last_action = state.specimens.text.get("menubar-last-action").cloned();

    // Contract: Application menu bar with three top-level menus: File, Edit, View
    let menubar_items = vec![
        MenubarEntry::new("file", "File", vec![
            MenuEntry::new("new", "New").with_shortcut_label("⌘N"),
            MenuEntry::new("open", "Open…").with_shortcut_label("⌘O"),
            MenuEntry::new("save", "Save").with_shortcut_label("⌘S"),
            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("quit", "Quit").with_shortcut_label("⌘Q"),
        ]),
        MenubarEntry::new("edit", "Edit", vec![
            MenuEntry::new("undo", "Undo").with_shortcut_label("⌘Z"),
            MenuEntry::new("redo", "Redo").with_shortcut_label("⇧⌘Z"),
            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("cut", "Cut").with_shortcut_label("⌘X"),
            MenuEntry::new("copy", "Copy").with_shortcut_label("⌘C"),
            MenuEntry::new("paste", "Paste").with_shortcut_label("⌘V"),
        ]),
        MenubarEntry::new("view", "View", vec![
            MenuEntry::new("zoom-in", "Zoom in").with_shortcut_label("⌘+"),
            MenuEntry::new("zoom-out", "Zoom out").with_shortcut_label("⌘-"),
            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("fullscreen", "Full screen").with_shortcut_label("⌃⌘F"),
        ]),
    ];

    let mut spec = MenubarSpec::new(menubar_items)
        .with_aria_label("Application menu");
    if let Some(ref active) = active_menu {
        spec = spec.with_value(active.as_str());
    }

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("APPLICATION MENU BAR", text_secondary))
        .child(
            Menubar::from_spec(spec, theme)
                .with_id("specimen-menubar")
                .on_trigger(cx.listener(|this, val: &str, _w, cx| {
                    let current = this.state.specimens.text.get("menubar-active").cloned();
                    if current.as_deref() == Some(val) {
                        this.state.specimens.text.remove("menubar-active");
                    } else {
                        this.state.specimens.text.insert("menubar-active".to_string(), val.to_string());
                    }
                    cx.notify();
                }))
                .on_select(cx.listener(|this, val: &str, _w, cx| {
                    this.state.specimens.text.insert("menubar-last-action".to_string(), val.to_string());
                    this.state.specimens.text.remove("menubar-active");
                    cx.notify();
                }))
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(color_to_hsla(text_primary))
                .child(format!("Last action: {}", last_action.as_deref().unwrap_or("(none)")))
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
