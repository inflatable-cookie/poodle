use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{CheckState, EyebrowSpec, TriStateSwitchSpec};
use poodle_gpui_components::{Eyebrow, TriStateSwitch};

use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

fn state_key(state: CheckState) -> usize {
    match state {
        CheckState::Unchecked => 0,
        CheckState::Mixed => 1,
        CheckState::Checked => 2,
    }
}

fn state_from_key(value: usize) -> CheckState {
    match value {
        0 => CheckState::Unchecked,
        2 => CheckState::Checked,
        _ => CheckState::Mixed,
    }
}

fn state_label(state: CheckState) -> &'static str {
    match state {
        CheckState::Unchecked => "excluded",
        CheckState::Mixed => "default",
        CheckState::Checked => "included",
    }
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let filter_state = state_from_key(state.specimens.selected("tri-state-filter"));
    let visibility_state = state_from_key(state.specimens.selected("tri-state-visibility"));
    let custom_state = state_from_key(state.specimens.selected("tri-state-custom"));

    div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    TriStateSwitch::from_spec(
                        TriStateSwitchSpec::new()
                            .with_state(filter_state)
                            .with_label("Filter mode"),
                        theme,
                    )
                    .on_change(cx.listener(|this, value: &CheckState, _w, cx| {
                        this.state.specimens.select("tri-state-filter", state_key(*value));
                        cx.notify();
                    }))
                )
                .child(
                    div().text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Value: {}", state_label(filter_state)))
                )
        )
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom labels"), theme))
                .child(
                    TriStateSwitch::from_spec(
                        TriStateSwitchSpec::new()
                            .with_state(visibility_state)
                            .with_label("Visibility")
                            .with_excluded_label("Hide")
                            .with_default_label("All")
                            .with_included_label("Show"),
                        theme,
                    )
                    .on_change(cx.listener(|this, value: &CheckState, _w, cx| {
                        this.state.specimens.select("tri-state-visibility", state_key(*value));
                        cx.notify();
                    }))
                )
                .child(
                    div().text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Value: {}", state_label(visibility_state)))
                )
        )
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom semantic colors"), theme))
                .child(
                    TriStateSwitch::from_spec(
                        TriStateSwitchSpec::new()
                            .with_state(custom_state)
                            .with_label("Filter mode")
                            .with_excluded_color("#ef4444")
                            .with_default_color("#64748b")
                            .with_included_color("#22c55e"),
                        theme,
                    )
                    .on_change(cx.listener(|this, value: &CheckState, _w, cx| {
                        this.state.specimens.select("tri-state-custom", state_key(*value));
                        cx.notify();
                    }))
                )
                .child(
                    div().text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Value: {}", state_label(custom_state)))
                )
        )
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    TriStateSwitch::from_spec(
                        TriStateSwitchSpec::new()
                            .with_state(CheckState::Checked)
                            .with_label("Disabled switch")
                            .with_disabled(true),
                        theme,
                    )
                )
        )
}
