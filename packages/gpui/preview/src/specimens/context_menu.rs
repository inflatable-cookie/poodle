use gpui::*;
use gpui::prelude::FluentBuilder;
use pug_adapter::ThemeProvider;
use pug_primitives::{ContextMenuSpec, MenuEntry, MenuItemKind, EyebrowSpec};
use pug_gpui_components::{ContextMenu, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let border = theme.resolve_color("semantic.color.border.default");

    let last_action = state.specimens.text.get("context-menu-action").cloned();

    let items = vec![
        MenuEntry::new("cut", "Cut").with_shortcut_label("\u{2318}X"),
        MenuEntry::new("copy", "Copy").with_shortcut_label("\u{2318}C"),
        MenuEntry::new("paste", "Paste").with_shortcut_label("\u{2318}V"),
        MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("select-all", "Select all").with_shortcut_label("\u{2318}A"),
        MenuEntry::new("sep2", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("delete", "Delete").with_disabled(true),
    ];

    let spec = ContextMenuSpec::new(items)
        .with_default_open(true);

    // Build a right-click target area matching Svelte's dashed bordered zone
    let target_area = div()
        .h(px(128.0))
        .w_full()
        .border_1()
        .border_color(color_to_hsla(border))
        .rounded(px(6.0))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child("Right-click this area".to_string())
        );

    div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Right-click the area below"), theme))
                .child(
                    ContextMenu::from_spec(spec, theme)
                        .with_id("specimen-context-menu")
                        .with_trigger(target_area)
                        .on_select(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert(
                                "context-menu-action".to_string(),
                                val.to_string(),
                            );
                            cx.notify();
                        }))
                )
        )
        .when(last_action.is_some(), |d| {
            d.child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child(format!("Last action: {}", last_action.as_deref().unwrap_or("")))
            )
        })
}
