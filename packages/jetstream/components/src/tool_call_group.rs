//! ToolCallGroup — a contiguous run of tool calls, backed by `ToolCallGroupSpec`.
//!
//! Contract: `docs/contracts/components/tool-call-group.md`.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ToolCallGroupSpec, ToolCallSpec};

use std::sync::Arc;

use crate::element::{Handler, IntoJsEl};
use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;
use crate::tool_call::ToolCall;

/// ToolCallGroup — a contiguous run of tool calls.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_x(handler)`.
pub struct ToolCallGroup {
    spec: ToolCallGroupSpec,
    theme: JetstreamThemeProvider,
    on_toggle: Option<Handler>,
    on_call_toggle: Option<Handler>,
}

impl ToolCallGroup {
    pub fn from_spec(spec: ToolCallGroupSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
            on_call_toggle: None,
        }
    }

    /// Fires with the run id when the run is expanded or collapsed.
    pub fn on_toggle(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_toggle = Some(Arc::new(handler));
        self
    }

    /// Fires with the call id when one call's output is opened or closed.
    pub fn on_call_toggle(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_call_toggle = Some(Arc::new(handler));
        self
    }
}

impl IntoJsEl for ToolCallGroup {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_toggle, self.on_call_toggle)
    }
}

pub fn js_tool_call_group(spec: &ToolCallGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None, None)
}

fn build(
    spec: &ToolCallGroupSpec,
    theme: &JetstreamThemeProvider,
    on_toggle: Option<Handler>,
    on_call_toggle: Option<Handler>,
) -> JsEl {
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

        let mut row = ToolCall::from_spec(call_spec, theme);
        if let Some(handler) = &on_call_toggle {
            let handler = Arc::clone(handler);
            row = row.on_toggle(move |id| handler(id));
        }
        list = list.child(row.into_js_el());
    }

    // The container is on the run, not the row: a thirty-call run has to read as
    // one box you can skim past, and thirty boxes is a cage.
    let surface: Color = resolve_color(theme, spec.surface_token()).into();
    let border: Color = resolve_color(theme, spec.border_token()).into();
    let hairline = rem_to_px(0.0625);

    let mut root = ui_element::div()
        .flex_col()
        .w_full()
        .gap(row_gap)
        .pt(rem_to_px(spec.padding_block_rem()))
        .pb(rem_to_px(spec.padding_block_rem()))
        .border(hairline)
        .border_color(border)
        .rounded(crate::theme_ext::resolve_radius(theme, spec.radius_token()))
        .bg(surface)
        .child(list);

    // Omitted entirely rather than drawn disabled when there is nothing to
    // reveal, and always last so expanding does not move the row you were
    // reading.
    if spec.shows_toggle() {
        let label = if spec.is_expanded {
            spec.fewer_label.clone()
        } else {
            spec.resolved_more_label()
        };

        let mut toggle = ui_element::button("")
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
            );

        if let Some(handler) = on_toggle {
            let id = spec.id.clone();
            toggle = toggle.cursor_pointer().on_click(move |_event| handler(&id));
        }

        root = root.child(toggle);
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
        assert!(
            tree.has_text("+1 previous tool calls"),
            "{:?}",
            tree.texts()
        );
    }

    /// The run toggle carries the run id, not a call id — expanding a run and
    /// opening a row's output are different events with different payloads, and
    /// the collapsed row sits directly above the toggle.
    #[test]
    fn the_toggle_reports_the_run() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let spec = ToolCallGroupSpec::new(
            "run-7",
            vec![
                call("a", "cargo check", ToolCallStatus::Success),
                call("b", "bun test", ToolCallStatus::Success),
            ],
        );

        let el = ToolCallGroup::from_spec(spec, &theme())
            .on_toggle(move |id| {
                assert_eq!(id, "run-7", "the handler is told which run was expanded");
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 720.0, 200.0, "+1 previous tool calls");

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_toggle fired exactly once"
        );
    }

    /// A run forwards to its rows, so opening one call's output does not read as
    /// expanding the run.
    #[test]
    fn a_row_click_reaches_the_call_handler() {
        use std::sync::Arc;
        use std::sync::Mutex;

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let calls = Arc::clone(&seen);
        let runs = Arc::clone(&seen);

        let mut lead = call("b", "bun test", ToolCallStatus::Success);
        lead.output = Some("272 pass".to_string());

        let spec = ToolCallGroupSpec::new(
            "run-7",
            vec![call("a", "cargo check", ToolCallStatus::Success), lead],
        );

        let el = ToolCallGroup::from_spec(spec, &theme())
            .on_call_toggle(move |id| calls.lock().unwrap().push(format!("call:{id}")))
            .on_toggle(move |id| runs.lock().unwrap().push(format!("run:{id}")))
            .into_js_el();

        // The collapsed run's only row is the newest call.
        crate::element::click_probe::click_text(&el, 720.0, 200.0, "bun test");

        assert_eq!(
            seen.lock().unwrap().as_slice(),
            ["call:b"],
            "the row's own event fired, not the run's"
        );
    }

    #[test]
    fn a_single_call_run_renders_no_toggle() {
        let spec =
            ToolCallGroupSpec::new("run", vec![call("a", "bun test", ToolCallStatus::Success)]);
        let tree = crate::render_probe::probe(&js_tool_call_group(&spec, &theme()), 720.0, 96.0);

        assert!(
            !tree
                .texts()
                .iter()
                .any(|t| t.contains("previous tool calls")),
            "{:?}",
            tree.texts()
        );
    }
}
