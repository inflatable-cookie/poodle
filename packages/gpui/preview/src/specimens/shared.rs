use gpui::{Context, Div};
use poodle_gpui_node_backend::to_gpui;
use poodle_render::RenderContext;
use poodle_specimens::{Pane, Specimen, View};

use super::specimen_layout::{specimen_layout_lazy, SpecimenAxes};
use crate::{app_state::AppState, PreviewRoot};

pub(crate) fn render(
    specimen: Specimen,
    state: &AppState,
    cx: &mut Context<PreviewRoot>,
    context: &RenderContext<'_>,
) -> Div {
    let build = |view| to_gpui(&state.shared_specimens.render(specimen, view, context));
    let mut axes = SpecimenAxes::examples_only();
    for pane in specimen.panes() {
        axes = match pane {
            Pane::Examples => axes,
            Pane::Sizes(values) => {
                axes.with_size_values(values, |value, _| build(View::Size(value)))
            }
            Pane::Densities(values) => {
                axes.with_density_values(values, |value, _| build(View::Density(value)))
            }
        };
    }
    specimen_layout_lazy(state, cx, specimen.slug(), || build(View::Examples), axes)
}
