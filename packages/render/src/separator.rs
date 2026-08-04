//! Separator — horizontal or vertical divider.
//!
//! Contract: `docs/contracts/components/separator.md`
//! Ported from: `packages/jetstream/components/src/separator.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{LayoutSizing, Node, NodeRole};
use poodle_specs::{SeparatorOrientation, SeparatorSpec};

use crate::color::with_alpha;

pub fn separator(spec: &SeparatorSpec, theme: &dyn ThemeProvider) -> Node {
    // Subtle tone: base colour at the spec's mix ratio of its own alpha;
    // default resolves ratio 1.0.
    let base = theme.resolve_color(spec.resolved_color());
    let color = with_alpha(base, base.3 * spec.subtle_mix_ratio());
    let stroke = theme.resolve_space(spec.resolved_stroke_width());

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(color);
        s.flex_none = true; // contract: flex 0 0 auto
        match spec.orientation {
            SeparatorOrientation::Horizontal => {
                s.min_height = Some(stroke);
                s.self_stretch = true; // width 100%
            }
            SeparatorOrientation::Vertical => {
                s.descriptor.layout.width = LayoutSizing::Fixed(stroke);
                s.fill_height = true; // min-height 100%
            }
        }
    }
    el.a11y.role = Some(NodeRole::Splitter);
    el
}
