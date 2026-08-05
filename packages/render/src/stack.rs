//! Stack — flex stack layout.
//!
//! Contract: `docs/contracts/components/stack.md`
//! Ported from: `packages/jetstream/components/src/stack.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::{Alignment, LayoutJustify, StackDirection, StackSpec};

pub fn stack(spec: &StackSpec, theme: &dyn ThemeProvider, children: Vec<Node>) -> Node {
    let padding = spec.resolved_padding();

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = match spec.direction {
            StackDirection::Column => LayoutDirection::Column,
            StackDirection::Row => LayoutDirection::Row,
        };

        if spec.wrap {
            s.flex_wrap = true;
        }

        if let Some(gap_token) = spec.resolved_gap() {
            s.descriptor.layout.spacing.gap = theme.resolve_space(gap_token);
        }

        // Cross-axis alignment (direction-aware default when unset:
        // column → stretch, row → center, per Svelte authority).
        match spec.resolved_align() {
            Alignment::Start => s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start,
            Alignment::Center => s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center,
            Alignment::End => s.descriptor.layout.alignment.cross = CrossAxisAlignment::End,
            Alignment::Stretch => {} // default flex behavior
        }

        if let Some(justify) = spec.justify {
            s.descriptor.layout.alignment.main = match justify {
                LayoutJustify::Start => MainAxisAlignment::Start,
                LayoutJustify::End => MainAxisAlignment::End,
                LayoutJustify::Center => MainAxisAlignment::Center,
                LayoutJustify::SpaceBetween => MainAxisAlignment::SpaceBetween,
            };
        }

        if let Some(h) = padding.horizontal {
            let px_val = theme.resolve_space(h);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = px_val;
            pad.right = px_val;
        }
        if let Some(v) = padding.vertical {
            let px_val = theme.resolve_space(v);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = px_val;
            pad.bottom = px_val;
        }
    }

    for child in children {
        el = el.child(child);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
