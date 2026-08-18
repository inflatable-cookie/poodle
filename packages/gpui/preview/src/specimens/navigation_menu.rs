use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, NavigationMenu};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ActiveEdge, ActiveFill, EyebrowSpec, NavigationMenuEntry, NavigationMenuSpec};
use std::sync::Arc;

fn change_handler(state: &AppState) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: "navmenu-active".to_string(),
            value: value.to_string(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    // Leading icons (contract §3 `icon`, §10 GPUI renders icon ahead of label)
    // plus per-item descriptions that drive the disclosed viewport content
    // (Known Delta §12 — `description` is the Rust slot-prop equivalent).
    let items = vec![
        NavigationMenuEntry::new("home", "Home")
            .with_icon("arrow-right")
            .with_description("Overview, highlights, and what's new this release."),
        NavigationMenuEntry::new("components", "Components")
            .with_icon("filter")
            .with_description("Buttons, inputs, overlays, and the full primitive catalog."),
        NavigationMenuEntry::new("tokens", "Tokens")
            .with_icon("check")
            .with_description("Color, spacing, typography, and radius semantic tokens."),
        NavigationMenuEntry::new("guides", "Guides")
            .with_icon("chevron-right")
            .with_description("Adoption guides, theming, and migration walkthroughs."),
        NavigationMenuEntry::new("changelog", "Changelog")
            .with_icon("clock")
            .with_disabled(true),
    ];

    let active_value = state
        .specimens
        .text
        .get("navmenu-active")
        .cloned()
        .unwrap_or_else(|| "components".to_string());

    let spec = NavigationMenuSpec::new(items.clone())
        .with_value(active_value.clone())
        .with_aria_label("Main navigation");

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Horizontal navigation"),
                    theme,
                ))
                .child(
                    NavigationMenu::from_spec(spec, theme)
                        .with_id("specimen-nav")
                        .on_change(change_handler(state)),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Active section: {}", active_value)),
                ),
        )
        // g13.016 switches: the default trigger is borderless; activeEdge
        // opts the border/underline back in, solid fill covers the open
        // trigger with accent-base + text-inverse and must survive hover
        // (native previews have no hover simulation — the render test proves
        // the hover patch).
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Navigation menu (active outline)"),
                    theme,
                ))
                .child(
                    NavigationMenu::from_spec(
                        NavigationMenuSpec::new(items.clone())
                            .with_value("components")
                            .with_aria_label("Outlined main navigation")
                            .with_active_edge(ActiveEdge::Outline),
                        theme,
                    )
                    .with_id("specimen-nav-outline"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Navigation menu (active underline)"),
                    theme,
                ))
                .child(
                    NavigationMenu::from_spec(
                        NavigationMenuSpec::new(items.clone())
                            .with_value("components")
                            .with_aria_label("Underlined main navigation")
                            .with_active_edge(ActiveEdge::Underline),
                        theme,
                    )
                    .with_id("specimen-nav-underline"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Navigation menu (solid fill)"),
                    theme,
                ))
                .child(
                    NavigationMenu::from_spec(
                        NavigationMenuSpec::new(items.clone())
                            .with_value("components")
                            .with_aria_label("Solid main navigation")
                            .with_active_fill(ActiveFill::Solid),
                        theme,
                    )
                    .with_id("specimen-nav-solid"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Navigation menu (solid fill — hover the open trigger)"),
                    theme,
                ))
                .child(
                    NavigationMenu::from_spec(
                        NavigationMenuSpec::new(items.clone())
                            .with_value("components")
                            .with_aria_label("Solid hovered main navigation")
                            .with_active_fill(ActiveFill::Solid),
                        theme,
                    )
                    .with_id("specimen-nav-solid-hover"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Navigation menu (no fill)"),
                    theme,
                ))
                .child(
                    NavigationMenu::from_spec(
                        NavigationMenuSpec::new(items.clone())
                            .with_value("components")
                            .with_aria_label("No-fill underlined main navigation")
                            .with_active_fill(ActiveFill::None)
                            .with_active_edge(ActiveEdge::Underline),
                        theme,
                    )
                    .with_id("specimen-nav-none"),
                ),
        )
        .into_any_element();

    let make_items = || {
        vec![
            NavigationMenuEntry::new("home", "Home"),
            NavigationMenuEntry::new("components", "Components"),
            NavigationMenuEntry::new("tokens", "Tokens"),
            NavigationMenuEntry::new("guides", "Guides"),
            NavigationMenuEntry::new("changelog", "Changelog").with_disabled(true),
        ]
    };

    specimen_layout(
        state,
        cx,
        "navigation-menu",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(move |size, theme: &GpuiThemeProvider| {
                NavigationMenu::from_spec(
                    NavigationMenuSpec::new(make_items())
                        .with_value("components")
                        .with_aria_label("Navigation"),
                    theme,
                )
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
            })
            .with_densities(move |density, theme: &GpuiThemeProvider| {
                NavigationMenu::from_spec(
                    NavigationMenuSpec::new(make_items())
                        .with_value("components")
                        .with_aria_label("Navigation"),
                    theme,
                )
                .with_id(format!("specimen-density-{:?}", density))
                .with_density(density)
                .into_any_element()
            }),
    )
}
