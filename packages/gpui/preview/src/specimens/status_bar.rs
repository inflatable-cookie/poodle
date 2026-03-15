use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_workstation::{ShellStatusBarSpec, SurfaceTabsSpec, SurfaceTabItem};
use pug_gpui_components::{PugShellStatusBar, PugSurfaceTabs};
use pug_gpui_primitives::{StatusIndicatorSpec, StatusTone};
use pug_gpui_components::PugStatusIndicator;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let selected = state.specimens.selected("surface-tab");

    let surface_tabs_spec = SurfaceTabsSpec::new(vec![
        SurfaceTabItem::new("main", "main.rs").with_closable(true),
        SurfaceTabItem::new("lib", "lib.rs").with_closable(true),
        SurfaceTabItem::new("mod", "mod.rs").with_closable(true),
    ])
    .with_value(["main", "lib", "mod"][selected]);

    let mut ready = StatusIndicatorSpec::new().with_status(StatusTone::Success);
    ready.label = Some("Ready".to_string());

    let status_spec = ShellStatusBarSpec::new()
        .with_summary("Ln 42, Col 18");

    div().flex().flex_col().gap(px(6.0))
        .child(PugSurfaceTabs::new(surface_tabs_spec, theme).with_id("stab"))
        .child(
            PugShellStatusBar::new(status_spec, theme)
                .with_leading_items(PugStatusIndicator::new(ready, theme))
                .with_trailing_items(
                    div().text_xs().text_color(color_to_hsla(text_secondary)).child("UTF-8")
                )
        )
}
