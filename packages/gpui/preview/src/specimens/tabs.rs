use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{TabsSpec, TabDefinition};
use pug_gpui_components::PugTabs;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let selected = state.specimens.selected("tabs-active");
    let tab_labels = ["Tab 1", "Tab 2", "Tab 3"];
    let tab_contents = ["Content for Tab 1", "Content for Tab 2", "Content for Tab 3"];

    let tabs: Vec<TabDefinition> = tab_labels.iter().enumerate().map(|(i, label)| {
        TabDefinition::new(format!("{}", i), *label)
    }).collect();

    let spec = TabsSpec::new(tabs).with_value(format!("{}", selected));

    let mut pug_tabs = PugTabs::new(spec, theme)
        .with_id("specimen-tabs");

    for (i, content) in tab_contents.iter().enumerate() {
        pug_tabs = pug_tabs.with_content(
            format!("{}", i),
            div().text_xs().text_color(color_to_hsla(text_secondary)).child(content.to_string()),
        );
    }

    div().child(pug_tabs)
}
