//! ValidationSummary — the roll-up of active validation entries.
//!
//! Contract: `docs/contracts/components/validation-summary.md`
//! Ported from: `packages/jetstream/components/src/validation_summary.rs`.
//!
//! Empty state (no active entries) renders nothing per contract §4. The
//! `<a href="#field-id">` focus-jump (contract §5) is web-only; the field id
//! is carried as an interaction id on each entry so the host can emulate
//! focus imperatively.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, Node, NodeRole,
};
use poodle_specs::ValidationSummarySpec;

/// Semibold label weight (typography constant; see form_shell).
const SEMIBOLD: u16 = 600;
const MEDIUM: u16 = 500;

pub fn validation_summary(spec: &ValidationSummarySpec, theme: &dyn ThemeProvider) -> Node {
    let entries = spec.active_entries();
    if entries.is_empty() {
        // Contract §4 empty state — a well-formed but visually empty node.
        let mut empty = Node::container();
        // Explicit Row (see switch.rs).
        empty.style.descriptor.layout.direction = LayoutDirection::Row;
        return empty;
    }

    let border = theme.resolve_color(spec.border_token());
    let fill = theme.resolve_color(spec.fill_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let danger_color = theme.resolve_color("color.status.danger");
    let accent_color = theme.resolve_color("color.accent.base");

    let title_size = theme.resolve_space(spec.title_size_token());
    let entry_size = theme.resolve_space(spec.entry_text_size_token());
    let pad_x = theme.resolve_space(spec.padding_x_token());
    let pad_y = theme.resolve_space(spec.padding_y_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let list_gap = theme.resolve_space(spec.list_gap_token());
    let entry_gap = theme.resolve_space(spec.entry_gap_token());
    let entry_text_gap = theme.resolve_space(spec.entry_text_gap_token());

    // Tone indicator dot sizing derives from the entry text size (a leading
    // bullet, not a contract-anatomy part). Half-text size.
    let dot_size = entry_size * 0.5;

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = list_gap;
    }

    // ── Title (optional) ─────────────────────────────────────
    if let Some(ref title) = spec.title {
        let mut t = Node::text(title);
        t.style.descriptor.text_color = Some(text_primary);
        t.style.text_size = Some(title_size);
        t.style.text_weight = Some(SEMIBOLD);
        el = el.child(t);
    }

    // ── List → Entry rows ────────────────────────────────────
    let mut list = Node::container();
    {
        let s = &mut list.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = list_gap;
    }
    for entry in &entries {
        let tone_color = if entry.is_blocking() {
            danger_color
        } else {
            accent_color
        };

        let mut row = Node::container();
        // Field id carried as the interaction id for host focus-jump emulation.
        row.id = Some(format!("validation-summary-entry:{}", entry.field_id));
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = entry_gap;
        }

        // Leading tone indicator dot (visual enrichment, token-derived).
        let mut dot = Node::container();
        {
            let s = &mut dot.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(dot_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(dot_size);
            let r = dot_size * 0.5;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = r;
            c.top_right = r;
            c.bottom_right = r;
            c.bottom_left = r;
            s.descriptor.background = Some(tone_color);
        }
        let row = row.child(dot);

        // Text column: Label + Message.
        let mut text_col = Node::container();
        {
            let s = &mut text_col.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = entry_text_gap;
        }
        let mut label = Node::text(&entry.label);
        label.style.descriptor.text_color = Some(text_primary);
        label.style.text_size = Some(entry_size);
        label.style.text_weight = Some(MEDIUM);
        let mut message = Node::text(&entry.message);
        message.style.descriptor.text_color = Some(text_secondary);
        message.style.text_size = Some(entry_size);
        let text_col = text_col.child(label).child(message);

        list = list.child(row.child(text_col));
    }
    let mut el = el.child(list);

    el.a11y.role = Some(NodeRole::Alert);
    el
}
