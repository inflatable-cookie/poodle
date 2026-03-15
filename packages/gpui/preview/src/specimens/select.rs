use gpui::*;
use pug_gpui_primitives::{SelectSpec, ChoiceOption};
use pug_gpui_components::PugSelect;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let open = state.specimens.is_on("select-open");
    let selected = state.specimens.selected("select-option");
    let option_labels = ["Option A", "Option B", "Option C"];

    let options: Vec<ChoiceOption> = option_labels.iter().enumerate().map(|(i, label)| {
        ChoiceOption::new(format!("{}", i), label.to_string())
    }).collect();

    let spec = SelectSpec::new(options)
        .with_value(format!("{}", selected))
        .with_open(open);

    div().child(
        PugSelect::new(spec, theme)
            .with_id("select-specimen")
            .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                this.state.specimens.toggle("select-open");
                cx.notify();
            }))
    )
}
