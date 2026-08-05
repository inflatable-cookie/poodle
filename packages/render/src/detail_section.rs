//! DetailSection — titled grouping of detail rows.
//!
//! Contract: `docs/contracts/components/detail-section.md`
//! Ported from: `packages/jetstream/components/src/detail_section.rs`.
//!
//! Optional separator rule, header (title + description + actions), body slot,
//! density-driven spacing, and a multi-column body (flex-wrap approximation of
//! the Svelte grid).

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
};
use poodle_specs::DetailSectionSpec;

use crate::presentation::rem_to_px;

/// Render a titled detail section.
///
/// - `content`: body children (detail rows, form fields, etc.)
/// - `actions`: optional trailing action slot in the header row (e.g. an edit button)
pub fn detail_section(
    spec: &DetailSectionSpec,
    theme: &dyn ThemeProvider,
    content: Vec<Node>,
    actions: Option<Node>,
) -> Node {
    let text_primary = theme.resolve_color(spec.title_color_token());
    let text_secondary = theme.resolve_color(spec.description_color_token());
    let border = theme.resolve_color(spec.separator_color_token());

    // Density-aware spacing resolved from the spec (contract §8).
    let root_gap = rem_to_px(spec.root_gap_rem());
    let header_gap = rem_to_px(spec.header_gap_rem());
    let title_gap = rem_to_px(spec.title_gap_rem());
    let body_gap = rem_to_px(spec.body_gap_rem());
    let separated_gap = rem_to_px(spec.separated_gap_rem());
    // Contract §8: separator rule height 0.0625rem.
    let separator_h = rem_to_px(0.0625);

    // Title font: 1.125rem heading (contract §8); description: body size token.
    let title_font = rem_to_px(1.125);
    let body_font = theme.resolve_space("typography.body.size");

    let mut el = Node::container();
    el.style.descriptor.layout.direction = LayoutDirection::Column;

    // Top separator rule — rendered when is_separated (density-driven gap).
    if spec.is_separated {
        let mut rule = Node::container();
        {
            let s = &mut rule.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.height = LayoutSizing::Fixed(separator_h);
            s.self_stretch = true;
            s.descriptor.background = Some(border);
            s.descriptor.layout.spacing.margin.bottom = separated_gap;
        }
        el = el.child(rule);
    }

    // Header row: title block on start, optional actions on end.
    let has_header = spec.title.is_some() || spec.description.is_some() || actions.is_some();
    if has_header {
        let mut header = Node::container();
        {
            let s = &mut header.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
            s.descriptor.layout.spacing.gap = header_gap;
            s.descriptor.layout.spacing.margin.bottom = root_gap;
        }

        // Title + description stacked vertically (title-gap between).
        let mut title_block = Node::container();
        {
            let s = &mut title_block.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = title_gap;
            s.flex_fill = true;
        }

        if let Some(ref title) = spec.title {
            let mut t = Node::text(title);
            t.style.descriptor.text_color = Some(text_primary);
            t.style.text_size = Some(title_font);
            t.style.text_weight = Some(700);
            title_block = title_block.child(t);
        }

        if let Some(ref desc) = spec.description {
            let mut d = Node::text(desc);
            d.style.descriptor.text_color = Some(text_secondary);
            d.style.text_size = Some(body_font);
            title_block = title_block.child(d);
        }

        let mut header = header.child(title_block);

        if let Some(actions_el) = actions {
            let mut slot = Node::container();
            {
                let s = &mut slot.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.flex_shrink_zero = true;
            }
            header = header.child(slot.child(actions_el));
        }

        el = el.child(header);
    }

    // Body: content rows.
    // columns > 1 → flex-wrap multi-column approximation of the Svelte grid.
    if !content.is_empty() {
        let mut body = Node::container();
        {
            let s = &mut body.style;
            if spec.columns > 1 {
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.flex_wrap = true;
            } else {
                s.descriptor.layout.direction = LayoutDirection::Column;
            }
            s.descriptor.layout.spacing.gap = body_gap;
            s.self_stretch = true;
        }
        for child in content {
            body = body.child(child);
        }
        el = el.child(body);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
