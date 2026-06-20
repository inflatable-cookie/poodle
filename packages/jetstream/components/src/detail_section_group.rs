//! JsDetailSectionGroup — multi-section layout backed by DetailSectionGroupSpec.
//!
//! Contract: `docs/contracts/components/detail-section-group.md`
//! Reference: GPUI `composites/detail_section_group.rs`.
//!
//! A wrapping grid (or vertical stack) of detail sections. Gap is density-driven;
//! each item's min column width resolves from the spec, not a hardcoded px.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{DetailSectionGroupLayout, DetailSectionGroupSpec};

use crate::presentation::rem_to_px;

/// Parse a `"<n>rem"` dimension string to logical px (falls back to 12rem).
fn rem_string_px(s: &str) -> f32 {
    s.trim()
        .strip_suffix("rem")
        .and_then(|n| n.trim().parse::<f32>().ok())
        .map(rem_to_px)
        .unwrap_or_else(|| rem_to_px(12.0))
}

/// Build a detail-section-group from its spec + child sections.
pub fn js_detail_section_group(
    spec: &DetailSectionGroupSpec,
    _theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
) -> JsEl {
    let gap = rem_to_px(spec.gap_rem());
    let item_min = rem_string_px(&spec.item_min_column_width);

    let mut root = ui_element::div().gap(gap);
    root = match spec.layout {
        DetailSectionGroupLayout::Grid => root.flex_row().flex_wrap(),
        DetailSectionGroupLayout::Stack => root.flex_col(),
    };
    for child in children {
        root = root.child(ui_element::div().flex_1().min_w(item_min).child(child));
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn wraps_all_children() {
        let el = js_detail_section_group(
            &DetailSectionGroupSpec::new(),
            &theme(),
            vec![ui_element::label("A"), ui_element::label("B"), ui_element::label("C")],
        );
        let tree = probe(&el, 600.0, 200.0);
        assert!(tree.has_text("A") && tree.has_text("B") && tree.has_text("C"));
    }

    #[test]
    fn item_min_width_resolves_from_spec() {
        // item_min_column_width default "12rem" → 192px wrappers.
        let el = js_detail_section_group(
            &DetailSectionGroupSpec::new(),
            &theme(),
            vec![ui_element::label("A")],
        );
        let tree = probe(&el, 600.0, 200.0);
        // The wrapper (depth 1) carries the min width.
        let wrapper = tree.nodes.iter().find(|n| n.depth == 1).expect("wrapper");
        assert!(wrapper.w >= rem_to_px(12.0) - 0.5, "wrapper too narrow: {}", wrapper.w);
    }
}
