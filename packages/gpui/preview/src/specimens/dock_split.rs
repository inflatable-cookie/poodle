use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{
    DockRegionSpec, DockEdge, PanelTabItem,
    SplitViewSpec, SplitOrientation,
};
use pug_gpui_components::{DockRegion, SplitView};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let active_tab = state.specimens.text.get("dock-active-tab")
        .cloned()
        .unwrap_or_else(|| "explorer".to_string());

    // --- Flexible dock -- expanded (left edge) ---
    let flex_expanded_spec = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
            PanelTabItem::new("search", "Search").with_icon("search"),
            PanelTabItem::new("source-control", "Source Control").with_icon("git-branch"),
        ],
    )
    .with_value(&active_tab);

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

    // --- Right edge dock ---
    let right_tab = state.specimens.text.get("dock-right-tab")
        .cloned()
        .unwrap_or_else(|| "outline".to_string());

    let flex_right_spec = DockRegionSpec::new(
        DockEdge::Right,
        vec![
            PanelTabItem::new("outline", "Outline").with_icon("list"),
            PanelTabItem::new("properties", "Properties").with_icon("settings"),
        ],
    )
    .with_value(&right_tab);

    // --- Horizontal split ---
    let h_split_spec = SplitViewSpec::new(SplitOrientation::Horizontal)
        .with_default_ratio(0.5);

    // --- Vertical split ---
    let v_split_spec = SplitViewSpec::new(SplitOrientation::Vertical)
        .with_default_ratio(0.5);

    // Content for the active tab
    let tab_content = match active_tab.as_str() {
        "explorer" => "Explorer: src/\n  main.rs\n  lib.rs\n  utils/\n    mod.rs",
        "search" => "Search results for \"component\"...",
        "source-control" => "2 files changed\n  M src/main.rs\n  A src/utils/mod.rs",
        _ => "Panel content",
    };

    div().flex().flex_col().gap(px(16.0))
        // --- Flexible dock -- expanded (left edge) ---
        .child(section_label("FLEXIBLE DOCK -- EXPANDED (LEFT EDGE)", text_secondary))
        .child(
            div().h(px(160.0)).flex().child(
                div().w(px(220.0)).h_full().child(
                    DockRegion::from_spec(flex_expanded_spec, theme)
                        .on_tab_change(cx.listener(|this, tab_id: &str, _w, cx| {
                            this.state.specimens.text.insert("dock-active-tab".to_string(), tab_id.to_string());
                            cx.notify();
                        }))
                        .with_content(
                            div().p(px(8.0))
                                .child(
                                    div().text_xs().text_color(color_to_hsla(text_secondary))
                                        .whitespace_nowrap()
                                        .child(tab_content.to_string())
                                )
                        )
                )
            )
        )
        // --- Flexible dock -- collapsed icon-strip (left edge) ---
        .child(section_label("FLEXIBLE DOCK -- COLLAPSED ICON-STRIP (LEFT EDGE)", text_secondary))
        .child(
            div().h(px(100.0)).flex().child(
                DockRegion::from_spec(flex_collapsed_spec, theme)
                    .with_content(div())
            )
        )
        // --- Right edge dock ---
        .child(section_label("FLEXIBLE DOCK -- RIGHT EDGE", text_secondary))
        .child(
            div().h(px(120.0)).flex().justify_end().child(
                div().w(px(200.0)).h_full().child(
                    DockRegion::from_spec(flex_right_spec, theme)
                        .on_tab_change(cx.listener(|this, tab_id: &str, _w, cx| {
                            this.state.specimens.text.insert("dock-right-tab".to_string(), tab_id.to_string());
                            cx.notify();
                        }))
                        .with_content(
                            div().p(px(8.0))
                                .child(
                                    div().text_xs().text_color(color_to_hsla(text_secondary))
                                        .child(format!("Active: {}", right_tab))
                                )
                        )
                )
            )
        )
        // --- Horizontal split ---
        .child(section_label("HORIZONTAL SPLIT", text_secondary))
        .child(
            div().h(px(100.0)).child(
                SplitView::from_spec(h_split_spec, theme)
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
                SplitView::from_spec(v_split_spec, theme)
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
