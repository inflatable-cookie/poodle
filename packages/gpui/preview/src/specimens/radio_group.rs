use gpui::*;
use pug_gpui_primitives::{RadioGroupSpec, ChoiceOption};
use pug_gpui_components::PugRadioGroup;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let selected = state.specimens.selected("radio-group");
    let items = ["Alpha", "Beta", "Gamma"];

    let options: Vec<ChoiceOption> = items.iter().enumerate().map(|(i, label)| {
        ChoiceOption::new(format!("{}", i), label.to_string())
    }).collect();

    let spec = RadioGroupSpec::new(options)
        .with_value(format!("{}", selected));

    div().child(
        PugRadioGroup::new(spec, theme)
            .with_id("radio-group")
            .on_change(cx.listener(|this, value: &str, _w, cx| {
                if let Ok(i) = value.parse::<usize>() {
                    this.state.specimens.select("radio-group", i);
                    cx.notify();
                }
            }))
    )
}
