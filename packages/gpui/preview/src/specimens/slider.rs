use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SliderSpec;
use pug_gpui_components::PugSlider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let spec = SliderSpec::new(50.0).with_bounds(0.0, 100.0);
    div().child(PugSlider::new(spec, theme))
}
