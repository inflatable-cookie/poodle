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

    let tabs_spec = PanelTabsSpec::new(vec![
        PanelTabItem::new("explorer", "Explorer"),
        PanelTabItem::new("search", "Search"),
    ])
    .with_value(if selected == 0 { "explorer" } else { "search" });

    let header_spec = PanelHeaderSpec::new()
        .with_title("Explorer")
        .with_active(true)
        .with_collapsible(true)
        .with_tabs(true);

    let surface_spec = PanelSurfaceSpec::new()
        .with_active(true);

    let contents = ["Panel content \u{2014} Explorer view", "Panel content \u{2014} Search results"];

    let panel_tabs = PugPanelTabs::new(tabs_spec, theme).with_id("panel-tab");
    let panel_header = PugPanelHeader::new(header_spec, theme)
        .with_tabs(panel_tabs);

    div().child(
        PugPanelSurface::new(surface_spec, theme)
            .with_header(panel_header)
            .with_content(
                div().p(px(10.0)).min_h(px(60.0))
                    .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child(contents[selected].to_string()))
            )
    )
}
