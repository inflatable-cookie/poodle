use gpui::*;
use pug_gpui_primitives::{AccordionSpec, AccordionItemSpec, AccordionSelectionValue};
use pug_gpui_components::PugAccordion;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let sections = ["Section One", "Section Two", "Section Three"];
    let contents = [
        "Expanded content for section one.",
        "This is section two's content area.",
        "Section three has its own content here.",
    ];

    let items: Vec<AccordionItemSpec> = sections.iter().zip(contents.iter()).enumerate().map(|(i, (title, content))| {
        AccordionItemSpec::new(format!("{}", i), *title)
            .with_description(*content)
    }).collect();

    let mut expanded = Vec::new();
    for i in 0..sections.len() {
        let key = format!("accordion-{}", i);
        if state.specimens.is_on(&key) {
            expanded.push(format!("{}", i));
        }
    }

    let mut spec = AccordionSpec::new(items)
        .with_allow_multiple(true)
        .with_collapsible(true);

    if !expanded.is_empty() {
        spec = spec.with_value(AccordionSelectionValue::Multiple(expanded));
    }

    div().child(
        PugAccordion::new(spec, theme)
            .with_id("specimen-accordion")
            .on_toggle(cx.listener(|this, value: &str, _w, cx| {
                let key = format!("accordion-{}", value);
                this.state.specimens.toggle(&key);
                cx.notify();
            }))
    )
}
