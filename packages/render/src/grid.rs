//! Grid — grid layout emulated as a flex-wrap container.
//!
//! Contract: `docs/contracts/components/grid.md`
//! Ported from: `packages/jetstream/components/src/grid.rs`.
//!
//! Column tracks drive per-child flex:
//!   - `Fr` tracks → each child grows equally (grow 1, min-width 0).
//!   - `AutoFit { min_rem }` → each child is at least `min_rem` wide and grows
//!     to fill, wrapping like `repeat(auto-fit, minmax(min_rem, 1fr))`.
//!   - `Rem` fixed track → child takes that exact width, no grow/shrink.
//!
//! DELTAS vs CSS grid: weighted ratio tracks (`1fr 2fr`) collapse to EQUAL
//! columns; explicit `rows` tracks are not honored (rows emerge from
//! flex-wrap); `gap` is one value on both axes.

use poodle_adapter::ThemeProvider;
use poodle_node::{LayoutDirection, LayoutSizing, Node};
use poodle_specs::{GridColumns, GridSpec, GridTrack};

use crate::presentation::rem_to_px;

pub fn grid(spec: &GridSpec, theme: &dyn ThemeProvider, children: Vec<Node>) -> Node {
    let padding = spec.resolved_padding();
    let columns = spec.parsed_columns();

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;

        // Single gap value on both axes (contract §8).
        if let Some(gap_token) = spec.resolved_column_gap() {
            s.descriptor.layout.spacing.gap = theme.resolve_space(gap_token);
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

    for (i, child) in children.into_iter().enumerate() {
        let mut wrapper = Node::container();
        // Explicit Row (see switch.rs).
        wrapper.style.descriptor.layout.direction = LayoutDirection::Row;
        {
            let s = &mut wrapper.style;
            match &columns {
                GridColumns::AutoFit { min_rem } => {
                    let min_px = rem_to_px(*min_rem);
                    s.flex_fill = true;
                    s.flex_basis = Some(min_px);
                    s.min_width = Some(min_px);
                }
                GridColumns::Tracks(tracks) if !tracks.is_empty() => {
                    match tracks[i % tracks.len()] {
                        // Fr tracks → equal columns (weighted ratios degrade).
                        GridTrack::Fr(_) => {
                            s.flex_fill = true;
                            s.min_width = Some(0.0);
                        }
                        GridTrack::Rem(rem) => {
                            s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(rem));
                            s.flex_shrink_zero = true;
                        }
                    }
                }
                GridColumns::Tracks(_) => {
                    s.flex_fill = true;
                    s.min_width = Some(0.0);
                }
            }
        }
        el = el.child(wrapper.child(child));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
