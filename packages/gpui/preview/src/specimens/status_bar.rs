use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{TabStripSpec, TabStripItem, SurfaceSpec, SurfaceTone, SurfaceBorder, StatusIndicatorSpec, StatusTone};
use pug_gpui_components::{PugTabStrip, PugSurface, PugStatusIndicator};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let selected = state.specimens.selected("surface-tab");

    let tabs = vec![
        TabStripItem::new("main", "main.rs").with_closable(true),
        TabStripItem::new("lib", "lib.rs").with_closable(true),
        TabStripItem::new("mod", "mod.rs").with_closable(true),
    ];
    let values = ["main", "lib", "mod"];

    let tab_spec = TabStripSpec::new(tabs)
        .with_value(values[selected]);

    let mut ready = StatusIndicatorSpec::new().with_status(StatusTone::Success);
    ready.label = Some("Ready".to_string());

    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    div().flex().flex_col().gap(px(6.0))
        .child(PugTabStrip::new(tab_spec, theme).with_id("stab"))
        .child(
            PugSurface::new(surface_spec, theme)
                .with_content(
                    div().flex().items_center().gap(px(12.0))
                        .child(PugStatusIndicator::new(ready, theme))
                        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Ln 42, Col 18"))
                        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("UTF-8"))
                )
        )
}
