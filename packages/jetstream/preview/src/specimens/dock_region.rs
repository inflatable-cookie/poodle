//! DockRegion specimen — dockable panel region with tabs, collapse postures,
//! emphasis, static stacking, and cross-region drop affordance.
//!
//! Mirrors the GPUI `dock_split` specimen's DockRegion coverage (SplitView is a
//! separate component and out of scope here). Every region is the real
//! `js_dock_region`; postures are driven entirely from `DockRegionSpec`.

use crate::compat::js_dock_region;
use crate::compat::{rem_to_px, size_font_rem};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    ControlSize, DockCollapsedPosture, DockEdge, DockEmphasis, DockRegionSpec, DockSizing,
    PanelTabItem,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));

    // Panel content snippet builder for expanded regions.
    let body = move |text: &str| -> El {
        label(text)
            .text_color(text_primary)
            .text_size(body_font)
            .p(rem_to_px(0.5))
    };

    // Icon-bearing tabs (collapsed/compact modes render icon-only).
    let editor_tabs = || {
        vec![
            PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
            PanelTabItem::new("search", "Search").with_icon("search"),
            PanelTabItem::new("source-control", "Source Control").with_icon("git-branch"),
        ]
    };
    let panel_tabs = || {
        vec![
            PanelTabItem::new("terminal", "Terminal").with_icon("terminal"),
            PanelTabItem::new("output", "Output").with_icon("file-text"),
            PanelTabItem::new("problems", "Problems").with_icon("alert-circle"),
        ]
    };

    div()
        .flex_col()
        .gap(24.0)
        // --- Static dock — horizontal (top edge): stacked panels, no tabs. ---
        .child(group(
            "Static dock \u{2014} horizontal (top edge)",
            secondary,
            div().h(rem_to_px(6.0)).child(js_dock_region(
                &DockRegionSpec::new(
                    DockEdge::Top,
                    vec![
                        PanelTabItem::new("meter", "Meter Strip"),
                        PanelTabItem::new("transport", "Transport"),
                        PanelTabItem::new("mixer", "Mixer"),
                    ],
                )
                .with_sizing(DockSizing::Static),
                theme,
                None,
            )),
        ))
        // --- Static dock — vertical (left edge): row-stacked panels. ---
        .child(group(
            "Static dock \u{2014} vertical (left edge)",
            secondary,
            div().h(rem_to_px(7.5)).child(js_dock_region(
                &DockRegionSpec::new(
                    DockEdge::Left,
                    vec![
                        PanelTabItem::new("toolbar", "Toolbar"),
                        PanelTabItem::new("inspector", "Inspector"),
                    ],
                )
                .with_sizing(DockSizing::Static),
                theme,
                None,
            )),
        ))
        // --- Flexible dock — expanded (left edge): tab strip + active body. ---
        .child(group(
            "Flexible dock \u{2014} expanded (left edge)",
            secondary,
            div().h(rem_to_px(10.0)).child(js_dock_region(
                &DockRegionSpec::new(DockEdge::Left, editor_tabs()).with_value("explorer"),
                theme,
                Some(body("Explorer: src/\n  main.rs\n  lib.rs")),
            )),
        ))
        // --- Active tab (Search) selected via with_value. ---
        .child(group(
            "Active tab (Search)",
            secondary,
            div().h(rem_to_px(10.0)).child(js_dock_region(
                &DockRegionSpec::new(DockEdge::Left, editor_tabs()).with_value("search"),
                theme,
                Some(body("Search results for \"component\"...")),
            )),
        ))
        // --- Right edge dock. ---
        .child(group(
            "Flexible dock \u{2014} right edge",
            secondary,
            div().h(rem_to_px(8.0)).child(js_dock_region(
                &DockRegionSpec::new(
                    DockEdge::Right,
                    vec![
                        PanelTabItem::new("outline", "Outline").with_icon("list"),
                        PanelTabItem::new("properties", "Properties").with_icon("settings"),
                    ],
                )
                .with_value("outline"),
                theme,
                Some(body("Active: outline")),
            )),
        ))
        // --- Bottom edge dock. ---
        .child(group(
            "Bottom edge dock",
            secondary,
            div().h(rem_to_px(8.0)).child(js_dock_region(
                &DockRegionSpec::new(DockEdge::Bottom, panel_tabs()).with_value("terminal"),
                theme,
                Some(body("Terminal: build succeeded")),
            )),
        ))
        // --- Collapsed icon-strip (left edge): vertical icon-only strip + toggle. ---
        .child(group(
            "Collapsed icon-strip (left edge)",
            secondary,
            div().h(rem_to_px(8.0)).child(js_dock_region(
                &DockRegionSpec::new(DockEdge::Left, editor_tabs())
                    .with_collapsible(true)
                    .with_collapsed(true)
                    .with_collapsed_posture(DockCollapsedPosture::IconStrip),
                theme,
                None,
            )),
        ))
        // --- Collapsed hidden posture: only the collapse toggle is visible. ---
        .child(group(
            "Collapsed hidden posture (only collapse toggle)",
            secondary,
            div().h(rem_to_px(8.0)).child(js_dock_region(
                &DockRegionSpec::new(DockEdge::Left, editor_tabs())
                    .with_collapsible(true)
                    .with_collapsed(true)
                    .with_collapsed_posture(DockCollapsedPosture::Hidden),
                theme,
                None,
            )),
        ))
        // --- Compact tabs: top-edge icon-strip → horizontal icon-only tabs. ---
        .child(group(
            "Compact tabs (top-edge icon-strip)",
            secondary,
            div().h(rem_to_px(3.0)).child(js_dock_region(
                &DockRegionSpec::new(DockEdge::Top, panel_tabs())
                    .with_value("terminal")
                    .with_collapsed(true)
                    .with_collapsed_posture(DockCollapsedPosture::IconStrip),
                theme,
                None,
            )),
        ))
        // --- Emphasis (quiet / standard / strong). ---
        .child(group(
            "Emphasis (quiet / standard / strong)",
            secondary,
            div()
                .flex_row()
                .gap(12.0)
                .child(emphasis_cell(
                    theme,
                    DockEmphasis::Quiet,
                    "Quiet",
                    body_font,
                    text_primary,
                ))
                .child(emphasis_cell(
                    theme,
                    DockEmphasis::Standard,
                    "Standard",
                    body_font,
                    text_primary,
                ))
                .child(emphasis_cell(
                    theme,
                    DockEmphasis::Strong,
                    "Strong",
                    body_font,
                    text_primary,
                )),
        ))
        // --- canAcceptPanel drop affordance: drop-zone overlay. ---
        .child(group(
            "Drop affordance (canAcceptPanel)",
            secondary,
            div().h(rem_to_px(8.0)).child(js_dock_region(
                &DockRegionSpec::new(
                    DockEdge::Left,
                    vec![
                        PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
                        PanelTabItem::new("search", "Search").with_icon("search"),
                    ],
                )
                .with_value("explorer")
                .with_aria_label("Drop target dock")
                .with_can_accept_panel(true),
                theme,
                Some(body("Accepts cross-region panel drops")),
            )),
        ))
}

fn emphasis_cell(
    theme: &JetstreamThemeProvider,
    emphasis: DockEmphasis,
    text: &str,
    body_font: f32,
    text_primary: ColorValue,
) -> El {
    div()
        .w(rem_to_px(12.5))
        .h(rem_to_px(7.5))
        .child(js_dock_region(
            &DockRegionSpec::new(
                DockEdge::Left,
                vec![
                    PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
                    PanelTabItem::new("search", "Search").with_icon("search"),
                ],
            )
            .with_value("explorer")
            .with_emphasis(emphasis),
            theme,
            Some(
                label(text)
                    .text_color(text_primary)
                    .text_size(body_font)
                    .p(rem_to_px(0.5)),
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
