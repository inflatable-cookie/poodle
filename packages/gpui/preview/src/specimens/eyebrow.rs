use crate::node_compat::Eyebrow;
use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

use crate::app_state::AppState;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use poodle_specs::{ControlSize, EyebrowSize, EyebrowSpec};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_primary = theme.resolve_color("color.text.primary");
    let examples = div().flex().flex_col().gap(px(24.0))
        // --- Section label ---
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Section label"), theme))
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Eyebrow renders small uppercase text used for categorizing content above headings.".to_string())
                )
        )
        // --- Primitive category ---
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Primitive"), theme))
                .child(
                    div().text_xl().font_weight(FontWeight::SEMIBOLD)
                        .text_color(color_to_hsla(text_primary))
                        .child("Button".to_string())
                )
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Primary interactive control for triggering actions.".to_string())
                )
        )
        // --- Composite category ---
        .child(
            div().flex().flex_col().gap(px(4.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Composite"), theme))
                .child(
                    div().text_xl().font_weight(FontWeight::SEMIBOLD)
                        .text_color(color_to_hsla(text_primary))
                        .child("DataTable".to_string())
                )
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child("Feature-rich table with sorting, selection, and pagination.".to_string())
                )
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "eyebrow",
        examples,
        SpecimenAxes::examples_only()
            // Eyebrow's own size enum stops at `md`; the larger control steps have
            // no representative to show, exactly as in the Svelte specimen.
            .with_sizes_where(|size, theme: &GpuiThemeProvider| {
                let eyebrow_size = match size {
                    ControlSize::Xs => EyebrowSize::Xs,
                    ControlSize::Sm => EyebrowSize::Sm,
                    ControlSize::Md => EyebrowSize::Md,
                    ControlSize::Lg | ControlSize::Xl => return None,
                };
                Some(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Section label")
                        .with_size(eyebrow_size),
                    theme,
                ))
            }),
    )
}
