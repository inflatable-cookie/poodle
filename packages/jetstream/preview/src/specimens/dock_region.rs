//! DockRegion specimen — dockable panel region with tabs.

use jetstream_runtime::ui_element::*;
use flint_jetstream::JetstreamThemeProvider;
use flint_jetstream_components::dock_region::js_dock_region;
use flint_jetstream_components::theme_ext::*;
use flint_composites::{DockEdge, DockRegionSpec, PanelTabItem};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");

    let tabs = vec![
        PanelTabItem::new("files", "Files"),
        PanelTabItem::new("search", "Search"),
        PanelTabItem::new("git", "Git"),
    ];

    div().flex_col().gap(24.0)
        .child(group("Left dock with tabs", secondary,
            div().h(160.0).child(
                js_dock_region(
                    &DockRegionSpec::new(DockEdge::Left, tabs),
                    theme,
                    Some(label("Panel content area.").text_color(text_primary).text_size(13.0).p(8.0)),
                )
            )
        ))
        .child(group("Empty dock", secondary,
            div().h(80.0).child(
                js_dock_region(
                    &DockRegionSpec::new(DockEdge::Right, vec![]),
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
