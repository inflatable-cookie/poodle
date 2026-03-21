use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{ContextMenuSpec, MenuEntry, MenuItemKind};
use pug_gpui_components::ContextMenu;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let border = theme.resolve_color("semantic.color.border.default");

    // Contract: Right-click target area with items:
    // Cut (⌘X), Copy (⌘C), Paste (⌘V), separator, Select all (⌘A), separator, Delete (disabled)
    let items = vec![
        MenuEntry::new("cut", "Cut").with_shortcut_label("⌘X"),
        MenuEntry::new("copy", "Copy").with_shortcut_label("⌘C"),
        MenuEntry::new("paste", "Paste").with_shortcut_label("⌘V"),
        MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("select-all", "Select all").with_shortcut_label("⌘A"),
        MenuEntry::new("sep2", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("delete", "Delete").with_disabled(true),
    ];

    let spec = ContextMenuSpec::new(items)
        .with_default_open(true);

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("RIGHT-CLICK THE AREA BELOW", text_secondary))
        // Dashed-border target area
        .child(
            div()
                .h(px(128.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(4.0))
                .border_1()
                .border_color(color_to_hsla(border))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child("Right-click here to open context menu".to_string())
                )
        )
        // Show the context menu open below the target area
        .child(
            ContextMenu::from_spec(spec, theme)
                .with_id("specimen-context-menu")
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
