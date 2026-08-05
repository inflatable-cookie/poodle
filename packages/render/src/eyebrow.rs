//! Eyebrow — small uppercase section label.
//!
//! Ported from: `packages/jetstream/components/src/eyebrow.rs`. Uppercasing
//! happens here (no CSS transform channel), matching both old tiers.

use poodle_adapter::ThemeProvider;
use poodle_node::Node;
use poodle_specs::EyebrowSpec;

use crate::presentation::rem_to_px;

pub fn eyebrow(spec: &EyebrowSpec, theme: &dyn ThemeProvider) -> Node {
    let text_color = theme.resolve_color(spec.text_color_token());
    let font_size = rem_to_px(spec.font_size_rem());
    let text = spec.content.as_deref().unwrap_or("").to_uppercase();

    let mut el = Node::text(text);
    {
        let s = &mut el.style;
        s.descriptor.text_color = Some(text_color);
        s.text_size = Some(font_size);
        s.text_weight = Some(spec.font_weight());
        s.letter_spacing_em = Some(spec.letter_spacing_em());
        let mb = spec.margin_bottom_rem();
        if mb > 0.0 {
            s.descriptor.layout.spacing.margin.bottom = rem_to_px(mb);
        }
    }
    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}
