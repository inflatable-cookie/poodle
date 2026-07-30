//! ToolCallGroup — a contiguous run of tool calls, backed by `ToolCallGroupSpec`.
//!
//! Contract: `docs/contracts/components/tool-call-group.md`.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_specs::{ToolCallGroupSpec, ToolCallSpec};

use crate::presentation::rem_to_px;
use crate::primitives::icon::Icon;
use crate::primitives::tool_call::ToolCall;
use crate::theme_ext::resolve_color;

pub struct ToolCallGroup {
    spec: ToolCallGroupSpec,
    theme: GpuiThemeProvider,
    on_toggle: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_call_toggle: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl ToolCallGroup {
    pub fn from_spec(spec: ToolCallGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
            on_call_toggle: None,
        }
    }

    /// The run was expanded or collapsed.
    pub fn on_toggle(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(std::rc::Rc::new(handler));
        self
    }

    /// A call's output was opened or closed.
    pub fn on_call_toggle(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_call_toggle = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for ToolCallGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let toggle_color = match spec.status() {
            // A collapsed run whose failure is not its newest call is the whole
            // reason run status exists: without this the failure is invisible
            // until someone expands.
            ToolCallStatus::Error => resolve_color(theme, spec.danger_token()),
            _ => resolve_color(theme, spec.toggle_token()),
        };

        let font_size = px(rem_to_px(spec.font_size_rem()));
        let row_gap = px(rem_to_px(spec.row_gap_rem()));
        let gap = px(rem_to_px(spec.gap_rem()));
        let pad_x = px(rem_to_px(spec.padding_inline_rem()));

        // The container is on the run, not the row: a thirty-call run has to read
        // as one box you can skim past, and thirty boxes is a cage.
        let surface = resolve_color(theme, spec.surface_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = crate::theme_ext::resolve_radius(theme, spec.radius_token());

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .gap(row_gap)
            .py(px(rem_to_px(spec.padding_block_rem())))
            .border_1()
            .border_color(border)
            .rounded(radius)
            .bg(surface);

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
            if let Some(handler) = &self.on_call_toggle {
                let handler = handler.clone();
                row = row.on_toggle(move |id, window, cx| handler(id, window, cx));
            }
            root = root.child(row);
        }

        // Omitted entirely rather than drawn disabled when there is nothing to
        // reveal, and always last so expanding does not move the row you were
        // reading.
        if spec.shows_toggle() {
            let label = if spec.is_expanded {
                spec.fewer_label.clone()
            } else {
                spec.resolved_more_label()
            };

            let mut toggle = div()
                .id(SharedString::from(format!("poodle-tool-run-toggle-{}", spec.id)))
                .cursor_pointer();
            if let Some(handler) = &self.on_toggle {
                let handler = handler.clone();
                let id = spec.id.clone();
                toggle = toggle.on_click(move |_event, window, cx| handler(&id, window, cx));
            }

            root = root.child(
                toggle
                    .flex()
                    .items_center()
                    .gap(gap)
                    .px(pad_x)
                    .text_size(font_size)
                    .text_color(toggle_color)
                    .child(
                        Icon::new("chevron-down", theme)
                            .with_px_size(rem_to_px(spec.icon_size_rem()))
                            .with_color(toggle_color)
                            .into_any_element(),
                    )
                    .child(label),
            );
        }

        root.into_any_element()
    }
}
