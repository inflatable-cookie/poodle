//! DockRegion specimen — dockable panel region with tabs.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::dock_region::js_dock_region;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, DockEdge, DockRegionSpec, PanelTabItem};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");

    let tabs = vec![
        PanelTabItem::new("files", "Files"),
        PanelTabItem::new("search", "Search"),
        PanelTabItem::new("git", "Git"),
    ];

    let tabs_short = vec![
        PanelTabItem::new("outline", "Outline"),
        PanelTabItem::new("problems", "Problems"),
    ];

    div().flex_col().gap(24.0)
        .child(group("Left dock with tabs", secondary,
            div().h(rem_to_px(10.0)).child(
                js_dock_region(
                    &DockRegionSpec::new(DockEdge::Left, tabs.clone()),
                    theme,
                    Some(label("Panel content area.").text_color(text_primary).text_size(body_font).p(rem_to_px(0.5))),
                )
            )
        ))
        .child(group("Empty dock", secondary,
            div().h(rem_to_px(5.0)).child(
                js_dock_region(
                    &DockRegionSpec::new(DockEdge::Right, vec![]),
                    theme,
                    None,
                )
            )
        ))
        // Active tab — "search" selected via with_value
        .child(group("Active tab (Search)", secondary,
            div().h(rem_to_px(10.0)).child(
                js_dock_region(
                    &DockRegionSpec::new(DockEdge::Left, tabs.clone())
                        .with_value("search"),
                    theme,
                    Some(label("Search panel content.").text_color(text_primary).text_size(body_font).p(rem_to_px(0.5))),
                )
            )
        ))
        // Bottom dock
        .child(group("Bottom dock", secondary,
            div().h(rem_to_px(8.0)).child(
                js_dock_region(
                    &DockRegionSpec::new(DockEdge::Bottom, tabs_short.clone()),
                    theme,
                    Some(label("Bottom panel content.").text_color(text_primary).text_size(body_font).p(rem_to_px(0.5))),
                )
            )
        ))
        // Top dock
        .child(group("Top dock", secondary,
            div().h(rem_to_px(8.0)).child(
                js_dock_region(
                    &DockRegionSpec::new(DockEdge::Top, tabs_short.clone()),
                    theme,
                    Some(label("Top panel content.").text_color(text_primary).text_size(body_font).p(rem_to_px(0.5))),
                )
            )
        ))
        // Collapsible — currently collapsed
        .child(group("Collapsible (collapsed)", secondary,
            div().h(rem_to_px(8.0)).child(
                js_dock_region(
                    &DockRegionSpec::new(DockEdge::Left, tabs.clone())
                        .with_collapsible(true)
                        .with_collapsed(true),
                    theme,
                    None,
                )
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
