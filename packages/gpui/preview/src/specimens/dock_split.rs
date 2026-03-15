use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_workstation::{
    DockRegionSpec, DockEdge, PanelTabItem,
    SplitViewSpec, SplitOrientation,
};
use pug_gpui_components::{PugDockRegion, PugSplitView};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let dock_spec = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("explorer", "Explorer"),
            PanelTabItem::new("search", "Search"),
        ],
    )
    .with_value("explorer");

    let split_spec = SplitViewSpec::new(SplitOrientation::Horizontal)
        .with_default_ratio(0.5);

    div().flex().h(px(120.0))
        .child(
            div().w(px(120.0)).h_full().child(
                PugDockRegion::new(dock_spec, theme)
                    .with_content(
                        div().p(px(8.0))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Dock content"))
                    )
            )
        )
        .child(
            div().flex_1().h_full().child(
                PugSplitView::new(split_spec, theme)
                    .with_primary(
                        div().p(px(8.0))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Split A"))
                    )
                    .with_secondary(
                        div().p(px(8.0))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Split B"))
                    )
            )
        )
}
