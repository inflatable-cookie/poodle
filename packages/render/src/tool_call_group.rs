//! ToolCallGroup — a contiguous run of tool calls.
//!
//! Contract: `docs/contracts/components/tool-call-group.md`
//! Ported from: `packages/jetstream/components/src/tool_call_group.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeRole, StylePatch};
use poodle_specs::{ToolCallGroupSpec, ToolCallSpec};

use crate::color::TRANSPARENT;
use crate::presentation::rem_to_px;
use crate::tool_call::tool_call;

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct ToolCallGroupHandlers {
    /// Fires with the run id when the run is expanded or collapsed.
    pub on_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the call id when one call's output is opened or closed.
    pub on_call_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn tool_call_group(
    spec: &ToolCallGroupSpec,
    theme: &dyn ThemeProvider,
    handlers: ToolCallGroupHandlers,
) -> Node {
    // A collapsed run whose failure is not its newest call is the whole reason
    // run status exists: without this the failure is invisible until someone
    // expands.
    let toggle_color = match spec.status() {
        ToolCallStatus::Error => theme.resolve_color(spec.danger_token()),
        _ => theme.resolve_color(spec.toggle_token()),
    };

    let font_size = rem_to_px(spec.font_size_rem());
    let icon_size = rem_to_px(spec.icon_size_rem());
    let row_gap = rem_to_px(spec.row_gap_rem());
    let gap = rem_to_px(spec.gap_rem());
    let pad_x = rem_to_px(spec.padding_inline_rem());

    let mut list = Node::container();
    list.style.descriptor.layout.direction = LayoutDirection::Column;
    list.style.fill_width = true;
    list.style.descriptor.layout.spacing.gap = row_gap;
    list.a11y.role = Some(NodeRole::List);

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

        let on_call = handlers.on_call_toggle.as_ref().map(Arc::clone);
        list = list.child(tool_call(&call_spec, theme, on_call));
    }

    // The container is on the run, not the row: a thirty-call run has to read as
    // one box you can skim past, and thirty boxes is a cage.
    let surface = theme.resolve_color(spec.surface_token());
    let border = theme.resolve_color(spec.border_token());
    let hairline = rem_to_px(0.0625);
    let radius = theme.resolve_radius(spec.radius_token());

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.fill_width = true;
        s.descriptor.layout.spacing.gap = row_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = rem_to_px(spec.padding_block_rem());
        pad.bottom = rem_to_px(spec.padding_block_rem());
        s.descriptor.border.width = hairline;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.background = Some(surface);
    }
    let mut root = root.child(list);

    // Omitted entirely rather than drawn disabled when there is nothing to
    // reveal, and always last so expanding does not move the row you were
    // reading.
    if spec.shows_toggle() {
        let label_text = if spec.is_expanded {
            spec.fewer_label.clone()
        } else {
            spec.resolved_more_label()
        };

        let mut toggle = Node::button("");
        toggle.id = Some(format!("tool-call-group-toggle-{}", spec.id));
        toggle.a11y.label = Some(spec.toggle_accessible_name());
        toggle.a11y.role = Some(NodeRole::Button);
        toggle.a11y.expanded = Some(spec.is_expanded);
        {
            let s = &mut toggle.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = gap;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pad_x;
            pad.right = pad_x;
            s.descriptor.background = Some(TRANSPARENT);
        }
        toggle.interaction.focusable = true;
        toggle.style.focus = Some(StylePatch {
            background: None,
            border_color: Some(theme.resolve_color("color.accent.focusRing")),
            text_color: None,
            opacity: None,
        });

        let mut chevron = Node::icon("chevron-down", icon_size);
        chevron.style.descriptor.text_color = Some(toggle_color);
        let mut label = Node::text(label_text);
        label.style.text_size = Some(font_size);
        label.style.descriptor.text_color = Some(toggle_color);
        let mut toggle = toggle.child(chevron).child(label);

        if let Some(handler) = handlers.on_toggle {
            let id = spec.id.clone();
            toggle.style.descriptor.cursor = CursorHint::Pointer;
            toggle.interaction.on_activate = Some(Arc::new(move || handler(&id)));
        }

        root = root.child(toggle);
    }

    root
}
