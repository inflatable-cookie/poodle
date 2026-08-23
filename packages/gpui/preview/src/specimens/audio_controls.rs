//! The twelve audio-family specimen pages.
//!
//! Content comes from `poodle_render::audio_specimens`, which is shared with
//! Jetstream; the Examples / Sizes / Densities structure around it is GPUI's
//! own. Every one of these controls takes both `size` and `density`, so every
//! page admits both axis panes.

use gpui::*;
use poodle_render::audio_specimens::AudioSpecimen;

use crate::app_state::AppState;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;

fn to_element(node: poodle_node::Node) -> AnyElement {
    poodle_gpui_node_backend::to_gpui(&node)
}

pub(crate) fn render(
    specimen: AudioSpecimen,
    name: &str,
    state: &AppState,
    cx: &mut Context<PreviewRoot>,
) -> Div {
    let examples = to_element(specimen.examples(&poodle_render::RenderContext::new(&state.theme)));
    specimen_layout(
        state,
        cx,
        name,
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(move |size, theme| {
                to_element(specimen.size(size, &poodle_render::RenderContext::new(theme)))
            })
            .with_densities(move |density, theme| {
                to_element(specimen.density(
                    density,
                    &poodle_render::RenderContext::new(theme),
                ))
            }),
    )
}
