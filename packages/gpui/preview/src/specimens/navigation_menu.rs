use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize, NavigationMenuSpec, NavigationMenuEntry, EyebrowSpec};
use poodle_gpui_components::{NavigationMenu, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

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
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child({
                    let sizes: &[(&str, ControlSize)] = &[
                        ("xs", ControlSize::Xs),
                        ("sm", ControlSize::Sm),
                        ("md", ControlSize::Md),
                        ("lg", ControlSize::Lg),
                        ("xl", ControlSize::Xl),
                    ];
                    let mut col = div().flex().flex_col().gap(px(8.0));
                    for &(key, size) in sizes {
                        let items = vec![
                            NavigationMenuEntry::new("home", "Home"),
                            NavigationMenuEntry::new("docs", "Docs"),
                            NavigationMenuEntry::new("about", "About"),
                        ];
                        let spec = NavigationMenuSpec::new(items)
                            .with_value("docs")
                            .with_aria_label(format!("Nav size {}", key));
                        col = col.child(
                            NavigationMenu::from_spec(spec, theme)
                                .with_id(format!("size-{}", key))
                                .size(size)
                        );
                    }
                    col
                })
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child({
                    let densities: &[(&str, ControlDensity)] = &[
                        ("compact", ControlDensity::Compact),
                        ("default", ControlDensity::Default),
                        ("comfortable", ControlDensity::Comfortable),
                    ];
                    let mut col = div().flex().flex_col().gap(px(8.0));
                    for &(key, density) in densities {
                        let items = vec![
                            NavigationMenuEntry::new("home", "Home"),
                            NavigationMenuEntry::new("docs", "Docs"),
                            NavigationMenuEntry::new("about", "About"),
                        ];
                        let spec = NavigationMenuSpec::new(items)
                            .with_value("docs")
                            .with_aria_label(format!("Nav density {}", key));
                        col = col.child(
                            NavigationMenu::from_spec(spec, theme)
                                .with_id(format!("density-{}", key))
                                .with_density(density)
                        );
                    }
                    col
                })
        )
}
