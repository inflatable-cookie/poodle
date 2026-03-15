use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder};
use pug_gpui_components::PugSurface;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let accent = theme.resolve_color("semantic.color.accent.base");
    let selected = state.specimens.selected("card-selected");

    let cards_data = [("Nav Card", "Navigate →"), ("List Card", "Description text"), ("Action Card", "Click to select")];
    let mut row = div().flex().gap(px(8.0)).flex_wrap();

    for (i, (title, desc)) in cards_data.iter().enumerate() {
        let is_selected = selected == i;

        let surface_spec = SurfaceSpec::new()
            .with_tone(SurfaceTone::Panel)
            .with_border(if is_selected { SurfaceBorder::Default } else { SurfaceBorder::Subtle });

        let card = div()
            .id(SharedString::from(format!("card-{}", i)))
            .w(px(140.0))
            .cursor_pointer()
            .hover(|s| s.border_color(color_to_hsla(accent).opacity(0.5)))
            .active(|s| s.bg(color_to_hsla(accent).opacity(0.06)))
            .child(
                PugSurface::new(surface_spec, theme)
                    .with_content(
                        div().flex().flex_col().gap(px(4.0))
                            .child(div().text_sm().text_color(color_to_hsla(accent)).child(title.to_string()))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child(desc.to_string()))
                    )
            )
            .on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.select("card-selected", i);
                cx.notify();
            }));
        row = row.child(card);
    }

    row
}
