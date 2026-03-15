use gpui::*;
use pug_gpui_primitives::{ButtonSpec, ButtonVariant, SeparatorSpec, SeparatorOrientation, RuleTone, SurfaceSpec, SurfaceTone, SurfaceBorder};
use pug_gpui_components::{PugButton, PugSeparator, PugSurface};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let active_tool = state.specimens.selected("toolbar-active");

    let tools = ["Bold", "Italic", "Link"];

    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    let mut bar = div().flex().items_center().gap(px(4.0));

    for (i, tool) in tools.iter().enumerate() {
        let is_active = active_tool == i;
        let variant = if is_active { ButtonVariant::Primary } else { ButtonVariant::Ghost };

        bar = bar.child(
            PugButton::new(
                ButtonSpec::new()
                    .with_variant(variant)
                    .with_label(*tool),
                theme,
            )
            .with_id(format!("toolbar-{}", tool))
            .on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.select("toolbar-active", i);
                cx.notify();
            }))
        );

        if i == 1 {
            bar = bar.child(
                PugSeparator::new(
                    SeparatorSpec::new()
                        .with_orientation(SeparatorOrientation::Vertical)
                        .with_tone(RuleTone::Subtle),
                    theme,
                )
            );
        }
    }

    div().child(
        PugSurface::new(surface_spec, theme)
            .with_content(bar)
    )
}
