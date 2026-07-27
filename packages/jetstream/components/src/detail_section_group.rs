//! JsDetailSectionGroup — multi-section layout backed by DetailSectionGroupSpec.
//!
//! Contract: `docs/contracts/components/detail-section-group.md`
//! Reference: GPUI `composites/detail_section_group.rs`.
//!
//! A wrapping grid (or vertical stack) of detail sections. Gap is density-driven;
//! the column min width resolves from `min_column_width` and the column count is
//! capped at `max_columns` (contract §7) — no hardcoded px.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{DetailSectionGroupLayout, DetailSectionGroupSpec};

use crate::presentation::rem_to_px;

/// Build a detail-section-group from its spec + child sections.
pub fn js_detail_section_group(
    spec: &DetailSectionGroupSpec,
    _theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
) -> JsEl {
    let gap = rem_to_px(spec.gap_rem());

    let root = match spec.layout {
        DetailSectionGroupLayout::Stack => {
            // Single vertical column regardless of available width.
            let mut root = ui_element::div().flex_col().gap(gap);
            for child in children {
                root = root.child(ui_element::div().w_full().child(child));
            }
            root
        }
        DetailSectionGroupLayout::Grid => {
            // Wrapping grid. Each column is at least `min_column_width` wide and
            // grows to fill the row, wrapping when columns no longer fit.
            //
            // NOTE: the `max_columns` cap (contract §7) is approximated. JsEl has
            // no percentage flex-basis/max-width, so we can't bound the column
            // count to N by fraction the way the Svelte grid or the GPUI target
            // (`flex_basis(relative(1/N))`) does. Wrapping + `min_w` keeps columns
            // legible but does not hard-cap their number — a JsEl-layout gap.
            let column_min = rem_to_px(spec.min_column_width_rem());

            let mut root = ui_element::div().flex_row().flex_wrap().gap(gap);
            for child in children {
                root = root.child(ui_element::div().flex_1().min_w(column_min).child(child));
            }
            root
        }
    };
    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
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
    fn column_min_width_resolves_from_spec() {
        // Grid columns honor `min_column_width` ("14rem" default → 224px).
        let el = js_detail_section_group(
            &DetailSectionGroupSpec::new(),
            &theme(),
            vec![ui_element::label("A")],
        );
        let tree = probe(&el, 600.0, 200.0);
        // The wrapper (depth 1) carries the column min width.
        let wrapper = tree.nodes.iter().find(|n| n.depth == 1).expect("wrapper");
        assert!(
            wrapper.w >= rem_to_px(14.0) - 0.5,
            "wrapper narrower than min_column_width: {}",
            wrapper.w
        );
    }

    #[test]
    fn custom_min_column_width_resolves() {
        // A custom min_column_width string parses and constrains the column.
        let el = js_detail_section_group(
            &DetailSectionGroupSpec::new().with_min_column_width("10rem"),
            &theme(),
            vec![ui_element::label("A")],
        );
        let tree = probe(&el, 600.0, 200.0);
        let wrapper = tree.nodes.iter().find(|n| n.depth == 1).expect("wrapper");
        assert!(
            wrapper.w >= rem_to_px(10.0) - 0.5,
            "wrapper narrower than custom min: {}",
            wrapper.w
        );
    }

    #[test]
    fn stack_layout_single_column() {
        // In stack layout the three children stack vertically (distinct y rows).
        let el = js_detail_section_group(
            &DetailSectionGroupSpec::new().with_layout(DetailSectionGroupLayout::Stack),
            &theme(),
            vec![
                ui_element::label("A"),
                ui_element::label("B"),
                ui_element::label("C"),
            ],
        );
        let tree = probe(&el, 600.0, 400.0);
        let ys: Vec<f32> = ["A", "B", "C"]
            .iter()
            .map(|t| {
                tree.nodes
                    .iter()
                    .find(|n| n.text.as_deref() == Some(*t))
                    .unwrap()
                    .y
            })
            .collect();
        assert!(ys[0] < ys[1] && ys[1] < ys[2], "stack not vertical: {ys:?}");
    }
}
