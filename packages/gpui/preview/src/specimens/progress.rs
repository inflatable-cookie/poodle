use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ProgressSpec, EyebrowSpec};
use poodle_gpui_components::{Progress, Eyebrow};
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let examples = div().flex().flex_col().gap(px(24.0))
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
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "progress",
        examples,
        |size, theme: &GpuiThemeProvider| {
            Progress::from_spec(ProgressSpec::new().with_value(60.0), theme)
                .size(size)
                .into_any_element()
        },
        // Progress has no density axis; empty row.
        |_density, theme: &GpuiThemeProvider| {
            Progress::from_spec(ProgressSpec::new().with_value(60.0), theme)
                .into_any_element()
        },
    )
}
