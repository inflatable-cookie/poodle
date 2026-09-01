use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, Spinner};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_render::RenderContext;
use poodle_specs::{
    ControlSize, EyebrowSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant,
};

fn spinner_size(size: ControlSize) -> SpinnerSize {
    match size {
        ControlSize::Xs => SpinnerSize::Xs,
        ControlSize::Sm => SpinnerSize::Sm,
        ControlSize::Md => SpinnerSize::Md,
        ControlSize::Lg => SpinnerSize::Lg,
        ControlSize::Xl => SpinnerSize::Xl,
    }
}

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

pub(crate) fn render(
    state: &AppState,
    cx: &mut Context<PreviewRoot>,
    context: &RenderContext<'_>,
) -> Div {
    let theme = &state.theme;
    let from_spec = |spec: SpinnerSpec| Spinner::from_spec_with_context(spec, context);
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Ring",
            theme,
            from_spec(SpinnerSpec::new().with_variant(SpinnerVariant::Ring)),
        ))
        .child(group(
            "CLI grid",
            theme,
            from_spec(
                SpinnerSpec::new()
                    .with_variant(SpinnerVariant::Grid)
                    .with_tone(SpinnerTone::Muted),
            ),
        ))
        .child(group(
            "Context tones",
            theme,
            div()
                .flex()
                .gap(px(16.0))
                .items_center()
                .child(from_spec(
                    SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_tone(SpinnerTone::Current),
                ))
                .child(from_spec(
                    SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_tone(SpinnerTone::Accent),
                ))
                .child(from_spec(
                    SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Grid)
                        .with_tone(SpinnerTone::Muted),
                )),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "spinner",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, _theme: &GpuiThemeProvider| {
                from_spec(
                    SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_size(spinner_size(size)),
                )
            })
            .with_densities(|density, _theme: &GpuiThemeProvider| {
                from_spec(
                    SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_density(density),
                )
            }),
    )
}
