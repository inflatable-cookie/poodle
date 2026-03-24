use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ProgressSpec, EyebrowSpec};
use poodle_gpui_components::{Progress, Eyebrow};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Determinate ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Determinate"), theme))
                .child(Progress::from_spec(ProgressSpec::new().with_value(0.0), theme))
                .child(Progress::from_spec(ProgressSpec::new().with_value(35.0), theme))
                .child(Progress::from_spec(ProgressSpec::new().with_value(72.0), theme))
                .child(Progress::from_spec(ProgressSpec::new().with_value(100.0), theme))
        )
        // --- Indeterminate ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Indeterminate"), theme))
                .child(Progress::from_spec(ProgressSpec::new().with_indeterminate(true), theme))
        )
        // --- Custom max ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom max"), theme))
                .child(Progress::from_spec({
                    let mut spec = ProgressSpec::new().with_value(3.0);
                    spec.max = 5.0;
                    spec
                }, theme))
        )
}
