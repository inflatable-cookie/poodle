//! ToolCall — one row of agent work.
//!
//! Contract: `docs/contracts/components/tool-call.md`
//! Ported from: `packages/jetstream/components/src/tool_call.rs`.
//!
//! Every dimension resolves from the spec's ladder; the only literal is the
//! hairline the contract states as an absolute.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole};
use poodle_specs::ToolCallSpec;

use crate::presentation::rem_to_px;

/// Fires with the row id when it is opened or closed. A row with no output is
/// not interactive at all, so nothing is attached to it.
pub fn tool_call(
    spec: &ToolCallSpec,
    theme: &dyn ThemeProvider,
    on_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let label_color = theme.resolve_color(spec.label_token());
    let detail_color = theme.resolve_color(spec.detail_token());
    let icon_color = theme.resolve_color(spec.icon_token());
    let success = theme.resolve_color(spec.success_token());
    let danger = theme.resolve_color(spec.danger_token());
    let radius = theme.resolve_radius(spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let icon_size = rem_to_px(spec.icon_size_rem());
    let row_height = rem_to_px(spec.row_height_rem());
    let pad_y = rem_to_px(spec.padding_block_rem());
    let pad_x = rem_to_px(spec.padding_inline_rem());
    let gap = rem_to_px(spec.gap_rem());

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
            s.descriptor.opacity = theme.resolve_opacity(spec.detail_opacity_token());
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

    // Status reaches assistive technology through the name; colour and glyph do
    // not.
    let mut root = Node::container();
    root.id = Some(spec.id.clone());
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.fill_width = true;
    root.a11y.role = Some(NodeRole::ListItem);
    root.a11y.label = Some(spec.accessible_name());
    let mut root = root.child(row);

    if spec.has_output() && spec.is_expanded {
        if let Some(output) = &spec.output {
            let mut out = Node::text(output.clone());
            {
                let s = &mut out.style;
                s.text_size = Some(font_size);
                s.descriptor.text_color = Some(detail_color);
                s.descriptor.layout.spacing.padding.left = pad_x + icon_size + gap;
            }
            root = root.child(out);
        }
    }

    // Only a row with output can be opened, so only that row is clickable.
    if spec.has_output() {
        if let Some(handler) = on_toggle {
            let id = spec.id.clone();
            root.style.descriptor.cursor = CursorHint::Pointer;
            root.interaction.on_activate = Some(Arc::new(move || handler(&id)));
        }
    }

    root
}
