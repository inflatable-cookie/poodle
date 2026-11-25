use jetstream_ui::GameUi;
use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_render::RenderContext;
use poodle_specimens::{Effect, Pane, Specimen, View};
use poodle_specs::TabDefinition;

use crate::app_state::{AppState, SpecimenView};
use crate::jsx::resolve_color;

pub(super) fn tabs(specimen: Specimen) -> Vec<TabDefinition> {
    specimen
        .panes()
        .iter()
        .map(|pane| match pane {
            Pane::Examples => TabDefinition::new("examples", "Examples"),
            Pane::Sizes(_) => TabDefinition::new("sizes", "Sizes"),
            Pane::Densities(_) => TabDefinition::new("densities", "Densities"),
        })
        .collect()
}

pub(super) fn render(specimen: Specimen, theme: &JetstreamThemeProvider, state: &AppState) -> JsEl {
    let context = RenderContext::new(theme);
    let build =
        |view| jetstream_poodle::to_js_el(&state.shared_specimens.render(specimen, view, &context));
    let caption = resolve_color(theme, "color.text.secondary");
    let row = |name: &str, view| {
        div()
            .flex_row()
            .gap(16.0)
            .child(label(name).w(80.0).text_size(11.0).text_color(caption))
            .child(div().grow().child(build(view)))
    };

    for pane in specimen.panes() {
        match (state.specimen_view, pane) {
            (SpecimenView::Sizes, Pane::Sizes(values)) => {
                return div().flex_col().gap(12.0).children(
                    values
                        .iter()
                        .map(|&(value, name)| row(name, View::Size(value))),
                );
            }
            (SpecimenView::Densities, Pane::Densities(values)) => {
                return div().flex_col().gap(12.0).children(
                    values
                        .iter()
                        .map(|&(value, name)| row(name, View::Density(value))),
                );
            }
            _ => {}
        }
    }
    build(View::Examples)
}

pub fn apply_effects(ui: &mut GameUi, effects: Vec<Effect>) {
    for effect in effects {
        match effect {
            Effect::Focus(id) => {
                // The converter preserves Node IDs as element IDs, which
                // materialization stores in the runtime's token_key field.
                let target = ui.tree.nodes.iter().enumerate().find(|(_, node)| {
                    node.alive
                        && node.style.focusable
                        && node.style.token_key.as_deref() == Some(id.as_str())
                });
                if let Some((index, node)) = target {
                    ui.focus.set_focus(Some(jetstream_ui::UiNodeId {
                        index,
                        generation: node.generation,
                    }));
                }
            }
        }
    }
}
