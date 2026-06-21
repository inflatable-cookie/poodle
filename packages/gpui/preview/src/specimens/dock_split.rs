use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{DockRegion, Eyebrow, SplitView};
use poodle_specs::EyebrowSpec;
use poodle_specs::{
    DockCollapsedPosture, DockEdge, DockEmphasis, DockRegionSpec, PanelTabItem, SplitOrientation,
    SplitViewSpec,
};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let hover_bg = theme.resolve_color("color.background.hover");

    let active_tab = state
        .specimens
        .text
        .get("dock-active-tab")
        .cloned()
        .unwrap_or_else(|| "explorer".to_string());

    // --- Static dock -- horizontal (top edge) ---
    let top_spec = DockRegionSpec::new(
        DockEdge::Top,
        vec![
            PanelTabItem::new("output", "Output").with_icon("terminal"),
            PanelTabItem::new("problems", "Problems").with_icon("alert-circle"),
            PanelTabItem::new("debug", "Debug Console").with_icon("play"),
        ],
    )
    .with_value("output");

    // --- Static dock -- vertical (left edge) ---
    let static_left_spec = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("files", "Files").with_icon("folder"),
            PanelTabItem::new("bookmarks", "Bookmarks").with_icon("bookmark"),
        ],
    )
    .with_value("files");

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

    // --- Interactive collapse toggle ---
    let toggle_collapsed = state.specimens.is_on("dock-toggle-collapsed");
    let toggle_tab = state
        .specimens
        .text
        .get("dock-toggle-tab")
        .cloned()
        .unwrap_or_else(|| "explorer".to_string());

    let toggle_spec = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
            PanelTabItem::new("search", "Search").with_icon("search"),
            PanelTabItem::new("extensions", "Extensions").with_icon("layout-grid"),
        ],
    )
    .with_collapsed(toggle_collapsed)
    .with_value(&toggle_tab);

    // --- Right edge dock ---
    let right_tab = state
        .specimens
        .text
        .get("dock-right-tab")
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

    // --- Bottom edge dock ---
    let bottom_collapsed = state.specimens.is_on("dock-bottom-collapsed");
    let bottom_tab = state
        .specimens
        .text
        .get("dock-bottom-tab")
        .cloned()
        .unwrap_or_else(|| "terminal".to_string());

    let bottom_spec = DockRegionSpec::new(
        DockEdge::Bottom,
        vec![
            PanelTabItem::new("terminal", "Terminal").with_icon("terminal"),
            PanelTabItem::new("output", "Output").with_icon("file-text"),
            PanelTabItem::new("problems", "Problems").with_icon("alert-circle"),
        ],
    )
    .with_collapsed(bottom_collapsed)
    .with_value(&bottom_tab);

    // --- Splits ---
    let h_split_spec = SplitViewSpec::new(SplitOrientation::Horizontal).with_default_ratio(0.5);
    let v_split_spec = SplitViewSpec::new(SplitOrientation::Vertical).with_default_ratio(0.5);

    // Content for the active tab
    let tab_content = match active_tab.as_str() {
        "explorer" => "Explorer: src/\n  main.rs\n  lib.rs\n  utils/\n    mod.rs",
        "search" => "Search results for \"component\"...",
        "source-control" => "2 files changed\n  M src/main.rs\n  A src/utils/mod.rs",
        _ => "Panel content",
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Static dock -- horizontal (top edge) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Static dock \u{2014} horizontal (top edge)"),
                    theme,
                ))
                .child(
                    div().h(px(100.0)).child(
                        DockRegion::from_spec(top_spec, theme).with_content(
                            div().p(px(8.0)).child(
                                div()
                                    .text_xs()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child("Output: Build succeeded in 2.3s"),
                            ),
                        ),
                    ),
                ),
        )
        // --- Static dock -- vertical (left edge) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Static dock \u{2014} vertical (left edge)"),
                    theme,
                ))
                .child(
                    div().h(px(120.0)).flex().child(
                        div().w(px(200.0)).h_full().child(
                            DockRegion::from_spec(static_left_spec, theme).with_content(
                                div().p(px(8.0)).child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("Files: index.ts, package.json, README.md"),
                                ),
                            ),
                        ),
                    ),
                ),
        )
        // --- Flexible dock -- expanded (left edge) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Flexible dock \u{2014} expanded (left edge)"),
                    theme,
                ))
                .child(
                    div().h(px(160.0)).flex().child(
                        div().w(px(220.0)).h_full().child(
                            DockRegion::from_spec(flex_expanded_spec, theme)
                                .on_tab_change(cx.listener(|this, tab_id: &str, _w, cx| {
                                    this.state
                                        .specimens
                                        .text
                                        .insert("dock-active-tab".to_string(), tab_id.to_string());
                                    cx.notify();
                                }))
                                .with_content(
                                    div().p(px(8.0)).child(
                                        div()
                                            .text_xs()
                                            .text_color(color_to_hsla(text_secondary))
                                            .whitespace_nowrap()
                                            .child(tab_content.to_string()),
                                    ),
                                ),
                        ),
                    ),
                ),
        )
        // --- Flexible dock -- collapsed icon-strip (left edge) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Flexible dock \u{2014} collapsed icon-strip (left edge)"),
                    theme,
                ))
                .child(
                    div().h(px(100.0)).flex().child(
                        DockRegion::from_spec(flex_collapsed_spec, theme).with_content(div()),
                    ),
                ),
        )
        // --- Interactive collapse toggle ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Interactive collapse toggle (click to toggle)"),
                    theme,
                ))
                .child(
                    div()
                        .h(px(if toggle_collapsed { 60.0 } else { 160.0 }))
                        .flex()
                        .child(
                            div()
                                .w(px(if toggle_collapsed { 48.0 } else { 220.0 }))
                                .h_full()
                                .child(
                                    DockRegion::from_spec(toggle_spec, theme)
                                        .on_tab_change(cx.listener(|this, tab_id: &str, _w, cx| {
                                            this.state.specimens.text.insert(
                                                "dock-toggle-tab".to_string(),
                                                tab_id.to_string(),
                                            );
                                            // Expand when a tab is clicked while collapsed
                                            this.state
                                                .specimens
                                                .toggles
                                                .insert("dock-toggle-collapsed".to_string(), false);
                                            cx.notify();
                                        }))
                                        .with_content(
                                            div().p(px(8.0)).child(
                                                div()
                                                    .text_xs()
                                                    .text_color(color_to_hsla(text_secondary))
                                                    .child(format!("Active: {}", toggle_tab)),
                                            ),
                                        ),
                                ),
                        )
                        .child(
                            div().ml(px(8.0)).flex().items_start().child(
                                div()
                                    .id("dock-toggle-btn")
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .text_xs()
                                    .text_color(color_to_hsla(text_secondary))
                                    .rounded(px(4.0))
                                    .cursor(CursorStyle::PointingHand)
                                    .hover(|s| s.bg(color_to_hsla(hover_bg)))
                                    .child(if toggle_collapsed {
                                        "Expand \u{25B8}"
                                    } else {
                                        "Collapse \u{25C2}"
                                    })
                                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                        this.state.specimens.toggle("dock-toggle-collapsed");
                                        cx.notify();
                                    })),
                            ),
                        ),
                ),
        )
        // --- Right edge dock ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Flexible dock \u{2014} right edge"),
                    theme,
                ))
                .child(
                    div().h(px(120.0)).flex().justify_end().child(
                        div().w(px(200.0)).h_full().child(
                            DockRegion::from_spec(flex_right_spec, theme)
                                .on_tab_change(cx.listener(|this, tab_id: &str, _w, cx| {
                                    this.state
                                        .specimens
                                        .text
                                        .insert("dock-right-tab".to_string(), tab_id.to_string());
                                    cx.notify();
                                }))
                                .with_content(
                                    div().p(px(8.0)).child(
                                        div()
                                            .text_xs()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child(format!("Active: {}", right_tab)),
                                    ),
                                ),
                        ),
                    ),
                ),
        )
        // --- Bottom edge dock ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Bottom edge dock (click to toggle)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .h(px(if bottom_collapsed { 36.0 } else { 120.0 }))
                                .child(
                                    DockRegion::from_spec(bottom_spec, theme)
                                        .on_tab_change(cx.listener(|this, tab_id: &str, _w, cx| {
                                            this.state.specimens.text.insert(
                                                "dock-bottom-tab".to_string(),
                                                tab_id.to_string(),
                                            );
                                            this.state
                                                .specimens
                                                .toggles
                                                .insert("dock-bottom-collapsed".to_string(), false);
                                            cx.notify();
                                        }))
                                        .with_content(
                                            div().p(px(8.0)).child(
                                                div()
                                                    .text_xs()
                                                    .text_color(color_to_hsla(text_secondary))
                                                    .child(format!("Terminal: {}", bottom_tab)),
                                            ),
                                        ),
                                ),
                        )
                        .child(
                            div().mt(px(4.0)).child(
                                div()
                                    .id("dock-bottom-toggle-btn")
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .text_xs()
                                    .text_color(color_to_hsla(text_secondary))
                                    .rounded(px(4.0))
                                    .cursor(CursorStyle::PointingHand)
                                    .hover(|s| s.bg(color_to_hsla(hover_bg)))
                                    .child(if bottom_collapsed {
                                        "Show panel \u{25B4}"
                                    } else {
                                        "Hide panel \u{25BE}"
                                    })
                                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                        this.state.specimens.toggle("dock-bottom-collapsed");
                                        cx.notify();
                                    })),
                            ),
                        ),
                ),
        )
        // --- Emphasis variants (quiet / standard / strong) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Emphasis (quiet / standard / strong)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(12.0))
                        .child(
                            div().w(px(200.0)).h(px(120.0)).child(
                                DockRegion::from_spec(
                                    DockRegionSpec::new(
                                        DockEdge::Left,
                                        vec![
                                            PanelTabItem::new("explorer", "Explorer")
                                                .with_icon("folder"),
                                            PanelTabItem::new("search", "Search")
                                                .with_icon("search"),
                                        ],
                                    )
                                    .with_value("explorer")
                                    .with_emphasis(DockEmphasis::Quiet),
                                    theme,
                                )
                                .with_content(
                                    div().p(px(8.0)).child(
                                        div()
                                            .text_xs()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child("Quiet"),
                                    ),
                                ),
                            ),
                        )
                        .child(
                            div().w(px(200.0)).h(px(120.0)).child(
                                DockRegion::from_spec(
                                    DockRegionSpec::new(
                                        DockEdge::Left,
                                        vec![
                                            PanelTabItem::new("explorer", "Explorer")
                                                .with_icon("folder"),
                                            PanelTabItem::new("search", "Search")
                                                .with_icon("search"),
                                        ],
                                    )
                                    .with_value("explorer")
                                    .with_emphasis(DockEmphasis::Standard),
                                    theme,
                                )
                                .with_content(
                                    div().p(px(8.0)).child(
                                        div()
                                            .text_xs()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child("Standard"),
                                    ),
                                ),
                            ),
                        )
                        .child(
                            div().w(px(200.0)).h(px(120.0)).child(
                                DockRegion::from_spec(
                                    DockRegionSpec::new(
                                        DockEdge::Left,
                                        vec![
                                            PanelTabItem::new("explorer", "Explorer")
                                                .with_icon("folder"),
                                            PanelTabItem::new("search", "Search")
                                                .with_icon("search"),
                                        ],
                                    )
                                    .with_value("explorer")
                                    .with_emphasis(DockEmphasis::Strong),
                                    theme,
                                )
                                .with_content(
                                    div().p(px(8.0)).child(
                                        div()
                                            .text_xs()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child("Strong"),
                                    ),
                                ),
                            ),
                        ),
                ),
        )
        // --- Collapsed hidden posture (only collapse toggle visible) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Collapsed hidden posture (only collapse toggle)"),
                    theme,
                ))
                .child(
                    div().h(px(120.0)).flex().child(
                        DockRegion::from_spec(
                            DockRegionSpec::new(
                                DockEdge::Left,
                                vec![
                                    PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
                                    PanelTabItem::new("search", "Search").with_icon("search"),
                                ],
                            )
                            .with_collapsed(true)
                            .with_collapsible(true)
                            .with_collapsed_posture(DockCollapsedPosture::Hidden),
                            theme,
                        )
                        .with_content(div()),
                    ),
                ),
        )
        // --- Compact tabs (collapsed icon-strip, top edge → horizontal icon-only) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Compact tabs (top-edge icon-strip)"),
                    theme,
                ))
                .child(
                    div().h(px(48.0)).child(
                        DockRegion::from_spec(
                            DockRegionSpec::new(
                                DockEdge::Top,
                                vec![
                                    PanelTabItem::new("terminal", "Terminal").with_icon("terminal"),
                                    PanelTabItem::new("output", "Output").with_icon("file-text"),
                                    PanelTabItem::new("problems", "Problems")
                                        .with_icon("alert-circle"),
                                ],
                            )
                            .with_value("terminal")
                            .with_collapsed(true)
                            .with_collapsed_posture(DockCollapsedPosture::IconStrip),
                            theme,
                        )
                        .with_content(div()),
                    ),
                ),
        )
        // --- canAcceptPanel drop affordance (drag-over drop-zone overlay) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Drop affordance (canAcceptPanel)"),
                    theme,
                ))
                .child(
                    div().h(px(120.0)).flex().child(
                        div().w(px(220.0)).h_full().child(
                            DockRegion::from_spec(
                                DockRegionSpec::new(
                                    DockEdge::Left,
                                    vec![
                                        PanelTabItem::new("explorer", "Explorer")
                                            .with_icon("folder"),
                                        PanelTabItem::new("search", "Search").with_icon("search"),
                                    ],
                                )
                                .with_value("explorer")
                                .with_aria_label("Drop target dock")
                                .with_can_accept_panel(true),
                                theme,
                            )
                            .with_content(
                                div().p(px(8.0)).child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("Accepts cross-region panel drops"),
                                ),
                            ),
                        ),
                    ),
                ),
        )
        // --- Cross-region drag-and-drop (two side-by-side regions) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Cross-region drag-and-drop"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(12.0))
                        .h(px(140.0))
                        .child(
                            div().flex_1().h_full().child(
                                DockRegion::from_spec(
                                    DockRegionSpec::new(
                                        DockEdge::Left,
                                        vec![
                                            PanelTabItem::new("explorer", "Explorer")
                                                .with_icon("folder"),
                                            PanelTabItem::new("search", "Search")
                                                .with_icon("search"),
                                            PanelTabItem::new("source-control", "Source Control")
                                                .with_icon("git-branch"),
                                        ],
                                    )
                                    .with_value("explorer")
                                    .with_aria_label("Left dock")
                                    .with_can_accept_panel(true),
                                    theme,
                                )
                                .with_content(
                                    div().p(px(8.0)).child(
                                        div()
                                            .text_xs()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child("Left dock \u{2014} 3 panels"),
                                    ),
                                ),
                            ),
                        )
                        .child(
                            div().flex_1().h_full().child(
                                DockRegion::from_spec(
                                    DockRegionSpec::new(
                                        DockEdge::Right,
                                        vec![PanelTabItem::new("outline", "Outline")
                                            .with_icon("list")],
                                    )
                                    .with_value("outline")
                                    .with_aria_label("Right dock")
                                    .with_can_accept_panel(true),
                                    theme,
                                )
                                .with_content(
                                    div().p(px(8.0)).child(
                                        div()
                                            .text_xs()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child("Right dock \u{2014} 1 panel"),
                                    ),
                                ),
                            ),
                        ),
                ),
        )
        // --- Horizontal split ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Horizontal split"),
                    theme,
                ))
                .child(
                    div().h(px(100.0)).child(
                        SplitView::from_spec(h_split_spec, theme)
                            .with_primary(
                                div().p(px(8.0)).child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("Primary pane"),
                                ),
                            )
                            .with_secondary(
                                div().p(px(8.0)).child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("Secondary pane"),
                                ),
                            ),
                    ),
                ),
        )
        // --- Vertical split ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Vertical split"),
                    theme,
                ))
                .child(
                    div().h(px(120.0)).child(
                        SplitView::from_spec(v_split_spec, theme)
                            .with_primary(
                                div().p(px(8.0)).child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("Primary pane"),
                                ),
                            )
                            .with_secondary(
                                div().p(px(8.0)).child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("Secondary pane"),
                                ),
                            ),
                    ),
                ),
        )
}
