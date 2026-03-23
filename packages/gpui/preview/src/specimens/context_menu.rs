use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{ContextMenuSpec, MenuEntry, MenuItemKind};
use pug_gpui_components::ContextMenu;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let last_action = state.specimens.text.get("context-menu-action").cloned();

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
        .child(section_label("CONTEXT MENU (SHOWN INLINE)", text_secondary))
        .child(
            ContextMenu::from_spec(spec, theme)
                .with_id("specimen-context-menu")
                .on_select(cx.listener(|this, val: &str, _w, cx| {
                    this.state.specimens.text.insert("context-menu-action".to_string(), val.to_string());
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
