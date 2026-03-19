use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{NavigationMenuSpec, NavigationMenuEntry};
use pug_gpui_components::PugNavigationMenu;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // Contract: Horizontal navigation with ariaLabel="Main navigation", value="components" (initially active)
    // Five items: Home, Components, Tokens, Guides, Changelog (disabled)
    let items = vec![
        NavigationMenuEntry::new("home", "Home"),
        NavigationMenuEntry::new("components", "Components"),
        NavigationMenuEntry::new("tokens", "Tokens"),
        NavigationMenuEntry::new("guides", "Guides"),
        NavigationMenuEntry::new("changelog", "Changelog").with_disabled(true),
    ];

    let selected = state.specimens.selected("navmenu-active");
    let active_value = if selected == 0 {
        "components".to_string()
    } else {
        format!("{}", selected)
    };

    let spec = NavigationMenuSpec::new(items)
        .with_value(active_value.clone())
        .with_aria_label("Main navigation");

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("HORIZONTAL NAVIGATION", text_secondary))
        .child(
            PugNavigationMenu::new(spec, theme)
                .with_id("specimen-nav")
        )
        .child(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_secondary))
                .child(format!("Active section: {}", active_value))
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
