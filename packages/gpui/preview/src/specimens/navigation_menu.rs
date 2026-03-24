use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{NavigationMenuSpec, NavigationMenuEntry, EyebrowSpec};
use poodle_gpui_components::{NavigationMenu, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let items = vec![
        NavigationMenuEntry::new("home", "Home"),
        NavigationMenuEntry::new("components", "Components"),
        NavigationMenuEntry::new("tokens", "Tokens"),
        NavigationMenuEntry::new("guides", "Guides"),
        NavigationMenuEntry::new("changelog", "Changelog").with_disabled(true),
    ];

    let active_value = state.specimens.text.get("navmenu-active")
        .cloned()
        .unwrap_or_else(|| "components".to_string());

    let spec = NavigationMenuSpec::new(items)
        .with_value(active_value.clone())
        .with_aria_label("Main navigation");

    div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Horizontal navigation"), theme))
                .child(
                    NavigationMenu::from_spec(spec, theme)
                        .with_id("specimen-nav")
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert(
                                "navmenu-active".to_string(),
                                val.to_string(),
                            );
                            cx.notify();
                        }))
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Active section: {}", active_value))
                )
        )
}
