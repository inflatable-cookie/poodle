use gpui::*;
use pug_gpui_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder};
use pug_gpui_components::PugSurface;
use pug_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let surface_spec = SurfaceSpec::new()
        .with_tone(SurfaceTone::Elevated)
        .with_border(SurfaceBorder::Default);

    let digits = ["1", "2", "3", "·"];
    let mut row = div().flex().gap(px(6.0));

    for d in &digits {
        row = row.child(
            div().w(px(36.0)).h(px(40.0)).child(
                PugSurface::new(surface_spec.clone(), theme)
                    .with_content(
                        div().w_full().h_full()
                            .flex().items_center().justify_center().text_base()
                            .child(d.to_string())
                    )
            )
        );
    }
    row
}
