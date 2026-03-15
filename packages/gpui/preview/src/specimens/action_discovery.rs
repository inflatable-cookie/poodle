use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{MenuSpec, MenuEntry};
use pug_gpui_components::PugMenu;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let selected = state.specimens.selected("action-disc");

    let actions = vec![
        MenuEntry::new("0", "New File").with_shortcut_label("⌘N"),
        MenuEntry::new("1", "Save").with_shortcut_label("⌘S"),
        MenuEntry::new("2", "Find").with_shortcut_label("⌘F"),
    ];

    let spec = MenuSpec::new(actions);

    let accent = theme.resolve_color("semantic.color.accent.base");

    div().flex().flex_col().gap(px(6.0))
        .child(
            div().flex().items_center().gap(px(8.0))
                .child(div().text_sm().child("Actions"))
                .child(div().px(px(6.0)).py(px(2.0)).rounded(px(3.0)).bg(color_to_hsla(accent).opacity(0.1)).text_xs().text_color(color_to_hsla(accent)).child("⌘K"))
        )
        .child(
            PugMenu::new(spec, theme)
                .with_id("action-disc")
                .with_selected(format!("{}", selected))
        )
}
