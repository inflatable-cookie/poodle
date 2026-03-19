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

    // --- Flexible dock -- expanded (left edge) ---
    let flex_expanded_spec = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
            PanelTabItem::new("search", "Search").with_icon("search"),
            PanelTabItem::new("source-control", "Source Control").with_icon("git-branch"),
        ],
    )
    .with_value("explorer");

    // --- Flexible dock -- collapsed icon-strip (left edge) ---
    let flex_collapsed_spec = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
            PanelTabItem::new("search", "Search").with_icon("search"),
            PanelTabItem::new("source-control", "Source Control").with_icon("git-branch"),
        ],
    )
    .with_collapsed(true);

    // --- Horizontal split ---
    let h_split_spec = SplitViewSpec::new(SplitOrientation::Horizontal)
        .with_default_ratio(0.5);

    // --- Vertical split ---
    let v_split_spec = SplitViewSpec::new(SplitOrientation::Vertical)
        .with_default_ratio(0.5);

    div().flex().flex_col().gap(px(16.0))
        // --- Flexible dock -- expanded (left edge) ---
        .child(section_label("FLEXIBLE DOCK -- EXPANDED (LEFT EDGE)", text_secondary))
        .child(
            div().h(px(140.0)).flex().child(
                div().w(px(180.0)).h_full().child(
                    PugDockRegion::new(flex_expanded_spec, theme)
                        .with_content(
                            div().p(px(8.0))
                                .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Explorer panel content"))
                        )
                )
            )
        )
        // --- Flexible dock -- collapsed icon-strip (left edge) ---
        .child(section_label("FLEXIBLE DOCK -- COLLAPSED ICON-STRIP (LEFT EDGE)", text_secondary))
        .child(
            div().h(px(100.0)).flex().child(
                PugDockRegion::new(flex_collapsed_spec, theme)
                    .with_content(div())
            )
        )
        // --- Horizontal split ---
        .child(section_label("HORIZONTAL SPLIT", text_secondary))
        .child(
            div().h(px(100.0)).child(
                PugSplitView::new(h_split_spec, theme)
                    .with_primary(
                        div().p(px(8.0))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Primary pane"))
                    )
                    .with_secondary(
                        div().p(px(8.0))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Secondary pane"))
                    )
            )
        )
        // --- Vertical split ---
        .child(section_label("VERTICAL SPLIT", text_secondary))
        .child(
            div().h(px(120.0)).child(
                PugSplitView::new(v_split_spec, theme)
                    .with_primary(
                        div().p(px(8.0))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Primary pane"))
                    )
                    .with_secondary(
                        div().p(px(8.0))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Secondary pane"))
                    )
            )
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
