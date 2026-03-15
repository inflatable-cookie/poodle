use gpui::*;
use pug_gpui_primitives::{SegmentedControlSpec, ChoiceOption};
use pug_gpui_components::PugSegmentedControl;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let selected = state.specimens.selected("segmented");
    let items = ["Daily", "Weekly", "Monthly"];

    let options: Vec<ChoiceOption> = items.iter().enumerate().map(|(i, label)| {
        ChoiceOption::new(format!("{}", i), label.to_string())
    }).collect();

    let spec = SegmentedControlSpec::new(options)
        .with_default_value(format!("{}", selected));

    div().child(
        PugSegmentedControl::new(spec, theme)
            .with_id("seg-specimen")
            .on_change(cx.listener(|this, value: &str, _w, cx| {
                if let Ok(i) = value.parse::<usize>() {
                    this.state.specimens.select("segmented", i);
                    cx.notify();
                }
            }))
    )
}
