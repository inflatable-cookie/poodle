//! ToolCall — one row of agent work.
//!
//! Contract: `docs/contracts/components/tool-call.md`
//! Ported from: `packages/jetstream/components/src/tool_call.rs`.
//!
//! Every dimension resolves from the spec's ladder; the only literal is the
//! hairline the contract states as an absolute.

use std::sync::Arc;

use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole, StylePatch,
};
use poodle_specs::ToolCallSpec;

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Fires with the row id when it is opened or closed. A row with no output is
/// not interactive at all, so nothing is attached to it.
#[derive(Default)]
pub struct ToolCallHandlers {
    pub on_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Stable native instance scope. Two rows with the same spec id would
    /// otherwise share one backend focus handle.
    pub instance_id: Option<String>,
}

/// The backend-state id of the row.
pub fn tool_call_focus_id(instance_id: Option<&str>, spec_id: &str) -> String {
    match instance_id {
        Some(scope) => format!("tool-call:{scope}:{spec_id}"),
        None => spec_id.to_string(),
    }
}

fn scoped(instance_id: Option<&str>, spec_id: &str) -> Option<String> {
    instance_id.map(|scope| format!("tool-call:{scope}:{spec_id}"))
}

pub fn tool_call(spec: &ToolCallSpec, ctx: &RenderContext<'_>, handlers: ToolCallHandlers) -> Node {
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);

    let label_color = ctx.theme().resolve_color(spec.label_token());
    let detail_color = ctx.theme().resolve_color(spec.detail_token());
    let icon_color = ctx.theme().resolve_color(spec.icon_token());
    let success = ctx.theme().resolve_color(spec.success_token());
    let danger = ctx.theme().resolve_color(spec.danger_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem(base_size));
    let icon_size = rem_to_px(spec.icon_size_rem(base_size));
    let row_height = rem_to_px(spec.row_height_rem(base_size));
    let pad_y = rem_to_px(spec.padding_block_rem(base_size));
    let pad_x = rem_to_px(spec.padding_inline_rem(density));
    let gap = rem_to_px(spec.gap_rem(density));

    // Only the label takes the danger colour, never the detail. The detail is
    // already the dimmest thing in the row, and colouring it red as well makes a
    // failed row read as a block of alarm rather than a line you can scan.
    let label_color = match spec.status {
        ToolCallStatus::Error => danger,
        _ => label_color,
    };
    let status_color = match spec.status {
        ToolCallStatus::Error => danger,
        ToolCallStatus::Success => success,
        ToolCallStatus::Running => icon_color,
    };

    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.min_height = Some(row_height);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
    }

    let mut glyph = Node::icon(spec.resolved_icon(), icon_size);
    glyph.style.descriptor.text_color = Some(icon_color);
    row = row.child(glyph);

    let mut label = Node::text(spec.label.clone());
    label.style.text_size = Some(font_size);
    label.style.descriptor.text_color = Some(label_color);
    label.style.flex_shrink_zero = true;
    row = row.child(label);

    if let Some(detail) = &spec.detail {
        // Grow + min-width 0 is load-bearing: without it the detail refuses to
        // shrink below its content width and a long command pushes the status
        // indicator out of the row.
        let mut d = Node::text(detail.clone());
        {
            let s = &mut d.style;
            s.text_size = Some(font_size);
            s.descriptor.text_color = Some(detail_color);
            s.descriptor.opacity = ctx.theme().resolve_opacity(spec.detail_opacity_token());
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.min_width = Some(0.0);
        }
        row = row.child(d);
    } else {
        let mut spacer = Node::container();
        // Explicit Row (see switch.rs).
        spacer.style.descriptor.layout.direction = LayoutDirection::Row;
        spacer.style.descriptor.layout.width = LayoutSizing::Grow;
        row = row.child(spacer);
    }

    if spec.has_output() {
        let mut chevron = Node::icon("chevron-down", icon_size);
        chevron.style.descriptor.text_color = Some(detail_color);
        row = row.child(chevron);
    }

    let mut status = Node::icon(spec.status_icon(), icon_size);
    status.style.descriptor.text_color = Some(status_color);
    status.style.flex_shrink_zero = true;
    row = row.child(status);

    // The visible row is the trigger, matching Svelte's button around the row;
    // the outer node only groups the trigger with its optional output.
    row.id = Some(spec.id.clone());
    row.runtime_id = scoped(handlers.instance_id.as_deref(), &spec.id);
    if spec.has_output() {
        row.a11y.role = Some(NodeRole::Button);
        row.a11y.label = Some(spec.accessible_name());
        row.a11y.expanded = Some(spec.is_expanded);
        row.a11y.controls = Some(format!("{}-output", spec.id));
        row.interaction.focusable = true;
        row.style.focus = Some(StylePatch {
            background: None,
            border_color: Some(ctx.theme().resolve_color(spec.focus_ring_token())),
            text_color: None,
            opacity: None,
        });
        row.style.hover = Some(StylePatch {
            background: Some(ctx.theme().resolve_color(spec.hover_fill_token())),
            border_color: None,
            text_color: None,
            opacity: None,
        });
        if let Some(handler) = handlers.on_toggle {
            let id = spec.id.clone();
            row.style.descriptor.cursor = CursorHint::Pointer;
            row.interaction.on_activate = Some(Arc::new(move || handler(&id)));
        }
    }

    // The outer node is only a layout wrapper; ToolCallGroup owns list-item
    // semantics for its own list rather than making standalone calls list items.
    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.fill_width = true;
    let mut root = root.child(row);

    if spec.has_output() && spec.is_expanded {
        if let Some(output) = &spec.output {
            let mut out = Node::text(output.clone());
            out.id = Some(format!("{}-output", spec.id));
            {
                let s = &mut out.style;
                s.text_size = Some(font_size);
                s.descriptor.text_color = Some(detail_color);
                s.descriptor.layout.spacing.padding.left = pad_x + icon_size + gap;
            }
            root = root.child(out);
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> ToolCallSpec {
        ToolCallSpec::new("with-output", "Ran command").with_output("ok")
    }

    #[test]
    fn an_instance_scope_isolates_backend_state_ids() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let first = tool_call(
            &spec(),
            &ctx,
            ToolCallHandlers {
                instance_id: Some("first".to_string()),
                ..ToolCallHandlers::default()
            },
        );
        let second = tool_call(
            &spec(),
            &ctx,
            ToolCallHandlers {
                instance_id: Some("second".to_string()),
                ..ToolCallHandlers::default()
            },
        );
        let expected = tool_call_focus_id(Some("first"), "with-output");
        let first_trigger = first
            .find(&|node| node.runtime_id.as_deref() == Some(expected.as_str()))
            .expect("interactive row carries the instance-scoped focus identity");
        assert!(second
            .find(&|node| node.runtime_id.as_deref() == Some(expected.as_str()))
            .is_none());
        assert_eq!(first_trigger.id.as_deref(), Some("with-output"));
    }
}
