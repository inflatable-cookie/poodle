//! ToolCallGroup — a contiguous run of tool calls, backed by `ToolCallGroupSpec`.
//!
//! Contract: `docs/contracts/components/tool-call-group.md`.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ToolCallGroupSpec, ToolCallSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;
use crate::tool_call::js_tool_call;

pub fn js_tool_call_group(spec: &ToolCallGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // A collapsed run whose failure is not its newest call is the whole reason
    // run status exists: without this the failure is invisible until someone
    // expands.
    let toggle_color: Color = match spec.status() {
        ToolCallStatus::Error => resolve_color(theme, spec.danger_token()).into(),
        _ => resolve_color(theme, spec.toggle_token()).into(),
    };

    let font_size = rem_to_px(spec.font_size_rem());
    let icon_size = rem_to_px(spec.icon_size_rem());
    let row_gap = rem_to_px(spec.row_gap_rem());
    let gap = rem_to_px(spec.gap_rem());
    let pad_x = rem_to_px(spec.padding_inline_rem());

    let mut list = ui_element::div()
        .flex_col()
        .w_full()
        .gap(row_gap)
        .aria_role(jetstream_ui::accesskit::Role::List);

    for call in spec.rendered_calls() {
        let mut call_spec = ToolCallSpec::new(call.id.clone(), call.label.clone())
            .with_status(call.status)
            .with_expanded(spec.expanded_calls.contains(&call.id))
            .with_size(spec.size)
            .with_density(spec.density);
        if let Some(detail) = &call.detail {
            call_spec = call_spec.with_detail(detail.clone());
        }
        if let Some(icon) = &call.icon {
            call_spec = call_spec.with_icon(icon.clone());
        }
        if let Some(output) = &call.output {
            call_spec = call_spec.with_output(output.clone());
        }
        list = list.child(js_tool_call(&call_spec, theme));
    }

    let mut root = ui_element::div().flex_col().w_full().gap(row_gap).child(list);

    // Omitted entirely rather than drawn disabled when there is nothing to
    // reveal, and always last so expanding does not move the row you were
    // reading.
    if spec.shows_toggle() {
        let label = if spec.is_expanded {
            spec.fewer_label.clone()
        } else {
            spec.resolved_more_label()
        };

        root = root.child(
            ui_element::button("")
                .aria_label(spec.toggle_accessible_name())
                .aria_role(jetstream_ui::accesskit::Role::Button)
                .aria_expanded(spec.is_expanded)
                .flex_row()
                .items_center()
                .gap(gap)
                .pl(pad_x)
                .pr(pad_x)
                .bg(Color::TRANSPARENT)
                .focusable()
                .child(
                    ui_element::icon("chevron-down")
                        .w(icon_size)
                        .h(icon_size)
                        .text_color(toggle_color),
                )
                .child(
                    ui_element::label(label)
                        .text_size(font_size)
                        .text_color(toggle_color),
                ),
        );
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_transcript::TranscriptToolCall;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn call(id: &str, detail: &str, status: ToolCallStatus) -> TranscriptToolCall {
        TranscriptToolCall {
            id: id.to_string(),
            label: "Ran command".to_string(),
            detail: Some(detail.to_string()),
            status,
            ..Default::default()
        }
    }

    #[test]
    fn collapsed_shows_only_the_newest_call() {
        let spec = ToolCallGroupSpec::new(
            "run",
            vec![
                call("a", "cargo check", ToolCallStatus::Success),
                call("b", "bun test", ToolCallStatus::Success),
            ],
        );
        let tree = crate::render_probe::probe(&js_tool_call_group(&spec, &theme()), 720.0, 96.0);

        assert!(tree.has_text("bun test"), "{:?}", tree.texts());
        assert!(!tree.has_text("cargo check"), "{:?}", tree.texts());
        assert!(tree.has_text("+1 previous tool calls"), "{:?}", tree.texts());
    }

    #[test]
    fn a_single_call_run_renders_no_toggle() {
        let spec = ToolCallGroupSpec::new("run", vec![call("a", "bun test", ToolCallStatus::Success)]);
        let tree = crate::render_probe::probe(&js_tool_call_group(&spec, &theme()), 720.0, 96.0);

        assert!(!tree.texts().iter().any(|t| t.contains("previous tool calls")), "{:?}", tree.texts());
    }
}
