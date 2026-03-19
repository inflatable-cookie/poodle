use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_workstation::{PanelHeaderSpec, PanelSurfaceSpec, PanelTabsSpec, PanelTabItem};
use pug_gpui_components::{PugPanelHeader, PugPanelSurface, PugPanelTabs};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let selected = state.specimens.selected("panel-tab");

    // --- Panel Tabs: Default ---
    let tabs_spec = PanelTabsSpec::new(vec![
        PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
        PanelTabItem::new("search", "Search").with_icon("search"),
        PanelTabItem::new("source-control", "Source Control").with_icon("git-branch"),
        PanelTabItem::new("debug", "Debug").with_icon("bug"),
    ])
    .with_value(["explorer", "search", "source-control", "debug"][selected.min(3)])
    .with_aria_label("Panel tabs");

    // --- Standard panel ---
    let standard_header = PanelHeaderSpec::new()
        .with_title("Explorer")
        .with_tabs(true);
    let standard_surface = PanelSurfaceSpec::new()
        .with_title("Explorer");

    let panel_tabs = PugPanelTabs::new(tabs_spec, theme).with_id("panel-tab");
    let panel_header = PugPanelHeader::new(standard_header, theme)
        .with_tabs(panel_tabs);

    // --- Active panel ---
    let active_surface = PanelSurfaceSpec::new()
        .with_active(true);
    let active_header = PanelHeaderSpec::new()
        .with_title("Active Panel")
        .with_active(true);

    // --- Elevated panel ---
    let elevated_surface = PanelSurfaceSpec::new()
        .with_elevated(true);
    let elevated_header = PanelHeaderSpec::new()
        .with_title("Elevated Panel");

    // --- Headerless panel ---
    let headerless_surface = PanelSurfaceSpec::new()
        .with_header(false);

    let body_content = |text: &str| {
        div().p(px(10.0)).min_h(px(40.0))
            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child(text.to_string()))
    };

    div().flex().flex_col().gap(px(16.0))
        // --- Standard panel ---
        .child(section_label("STANDARD PANEL", text_secondary))
        .child(
            PugPanelSurface::new(standard_surface, theme)
                .with_header(panel_header)
                .with_content(body_content("Panel body content"))
        )
        // --- Active panel ---
        .child(section_label("ACTIVE PANEL", text_secondary))
        .child(
            PugPanelSurface::new(active_surface, theme)
                .with_header(PugPanelHeader::new(active_header, theme))
                .with_content(body_content("Active panel body content"))
        )
        // --- Elevated panel ---
        .child(section_label("ELEVATED PANEL", text_secondary))
        .child(
            PugPanelSurface::new(elevated_surface, theme)
                .with_header(PugPanelHeader::new(elevated_header, theme))
                .with_content(body_content("Elevated panel body content"))
        )
        // --- Headerless panel ---
        .child(section_label("HEADERLESS PANEL", text_secondary))
        .child(
            PugPanelSurface::new(headerless_surface, theme)
                .with_content(body_content("Body region fills top boundary with no header chrome"))
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
