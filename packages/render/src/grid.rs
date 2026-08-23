//! Grid — grid layout emulated as a flex-wrap container.
//!
//! Contract: `docs/contracts/components/grid.md`
//! Ported from: `packages/jetstream/components/src/grid.rs`.
//!
//! Column tracks drive per-child flex:
//!   - `Fr` tracks → each child grows by its declared track weight.
//!   - `AutoFit { min_rem }` → each child is at least `min_rem` wide and grows
//!     to fill, wrapping like `repeat(auto-fit, minmax(min_rem, 1fr))`.
//!   - `Rem` fixed track → child takes that exact width, no grow/shrink.
//!
//! DELTAS vs CSS grid: explicit `rows` tracks are not honored (rows emerge
//! from flex-wrap); `gap` is one value on both axes.

use poodle_node::{LayoutDirection, LayoutSizing, Node};
use poodle_specs::{GridColumns, GridSpec, GridTrack};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn grid(spec: &GridSpec, ctx: &RenderContext<'_>, children: Vec<Node>) -> Node {
    let padding = spec.resolved_padding();
    let columns = spec.parsed_columns();
    let fr_total = match &columns {
        GridColumns::Tracks(tracks) => tracks
            .iter()
            .filter_map(|track| match track {
                GridTrack::Fr(weight) => Some(*weight),
                GridTrack::Rem(_) => None,
            })
            .sum::<f32>()
            .max(1.0),
        GridColumns::AutoFit { .. } => 1.0,
    };

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;

        // Single gap value on both axes (contract §8).
        if let Some(gap_token) = spec.resolved_column_gap() {
            s.descriptor.layout.spacing.gap = ctx.theme().resolve_space(gap_token);
        }

        if let Some(h) = padding.horizontal {
            let px_val = ctx.theme().resolve_space(h);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = px_val;
            pad.right = px_val;
        }
        if let Some(v) = padding.vertical {
            let px_val = ctx.theme().resolve_space(v);
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
                        GridTrack::Fr(weight) => {
                            s.flex_grow = Some(weight);
                            s.width_pct = Some(weight / fr_total - 0.001);
                            s.min_width = Some(0.0);
                            s.flex_shrink_zero = true;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fractional_tracks_keep_their_declared_weight() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let node = grid(
            &GridSpec::new().with_columns("1fr 2fr"),
            &ctx,
            vec![Node::text("one"), Node::text("two")],
        );

        assert_eq!(node.children[0].style.flex_grow, Some(1.0));
        assert_eq!(node.children[1].style.flex_grow, Some(2.0));
        assert!(node.children[0].style.width_pct < node.children[1].style.width_pct);
    }
}
