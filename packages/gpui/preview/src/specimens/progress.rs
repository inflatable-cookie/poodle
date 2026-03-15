use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::ProgressSpec;
use pug_gpui_components::PugProgress;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(6.0))
        .child(PugProgress::new(ProgressSpec::new().with_value(0.75), theme))
        .child(PugProgress::new(ProgressSpec::new().with_value(0.33), theme))
        .child(PugProgress::new(ProgressSpec::new().with_indeterminate(true), theme))
}
