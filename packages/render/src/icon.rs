//! Icon — a named SVG glyph at a token-resolved size.
//!
//! Contract: `docs/contracts/components/icon.md`
//! Ported from: `packages/jetstream/components/src/icon.rs`. Rasterisation and
//! tinting are backend concerns; the node names the glyph and its box.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::IconSpec;

pub fn icon(spec: &IconSpec, theme: &dyn ThemeProvider) -> Node {
    let size = theme.resolve_space(spec.size_token());

    let mut el = Node::icon(&spec.name, size);
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}
