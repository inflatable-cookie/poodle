use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SliderSpec;
use pug_gpui_components::PugSlider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(8.0))
        .child(
            PugSlider::new(SliderSpec::new(20.0).with_bounds(0.0, 100.0), theme)
        )
        .child(
            PugSlider::new(SliderSpec::new(80.0).with_bounds(0.0, 100.0), theme)
        )
}
