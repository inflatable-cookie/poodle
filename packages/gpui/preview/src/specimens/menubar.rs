use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{MenubarSpec, MenubarEntry, MenuEntry, MenuItemKind};
use pug_gpui_components::PugMenubar;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // Contract: Application menu bar with three top-level menus: File, Edit, View
    let menubar_items = vec![
        // File menu: New ⌘N, Open... ⌘O, Save ⌘S, separator, Quit ⌘Q
        MenubarEntry::new("file", "File", vec![
            MenuEntry::new("new", "New").with_shortcut_label("⌘N"),
            MenuEntry::new("open", "Open…").with_shortcut_label("⌘O"),
            MenuEntry::new("save", "Save").with_shortcut_label("⌘S"),
            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("quit", "Quit").with_shortcut_label("⌘Q"),
        ]),
        // Edit menu: Undo ⌘Z, Redo ⇧⌘Z, separator, Cut ⌘X, Copy ⌘C, Paste ⌘V
        MenubarEntry::new("edit", "Edit", vec![
            MenuEntry::new("undo", "Undo").with_shortcut_label("⌘Z"),
            MenuEntry::new("redo", "Redo").with_shortcut_label("⇧⌘Z"),
            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("cut", "Cut").with_shortcut_label("⌘X"),
            MenuEntry::new("copy", "Copy").with_shortcut_label("⌘C"),
            MenuEntry::new("paste", "Paste").with_shortcut_label("⌘V"),
        ]),
        // View menu: Zoom in ⌘+, Zoom out ⌘-, separator, Full screen ⌃⌘F
        MenubarEntry::new("view", "View", vec![
            MenuEntry::new("zoom-in", "Zoom in").with_shortcut_label("⌘+"),
            MenuEntry::new("zoom-out", "Zoom out").with_shortcut_label("⌘-"),
            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("fullscreen", "Full screen").with_shortcut_label("⌃⌘F"),
        ]),
    ];

    let spec = MenubarSpec::new(menubar_items)
        .with_aria_label("Application menu");

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("APPLICATION MENU BAR", text_secondary))
        .child(
            PugMenubar::new(spec, theme).with_id("specimen-menubar")
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
