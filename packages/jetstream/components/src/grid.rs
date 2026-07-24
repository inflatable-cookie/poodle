//! Grid — Jetstream grid layout backed by GridSpec.
//!
//! Jetstream/Taffy doesn't support CSS Grid natively, so we emulate it as a
//! flex-wrap container. Column tracks drive per-child flex:
//!   - `Fr` tracks → each child grows equally (`flex_grow`, basis 0).
//!   - `AutoFit { min_rem }` → each child is at least `min_rem` wide and grows
//!     to fill, wrapping like `repeat(auto-fit, minmax(min_rem, 1fr))`.
//!   - `Rem` fixed track → child takes that exact width, no grow/shrink.
//!
//! DELTAS vs CSS grid (JsEl runtime has no relative/weighted flex-basis):
//!   - weighted ratio tracks (`1fr 2fr`) collapse to EQUAL columns;
//!   - explicit `rows` tracks are not honored (rows emerge from flex-wrap);
//!   - CSS `gap` is one value on both axes — applied via a single `.gap()`.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{GridColumns, GridSpec, GridTrack};

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_px;

pub fn js_grid(spec: &GridSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let padding = spec.resolved_padding();
    let columns = spec.parsed_columns();

    let mut el = ui_element::div().flex_row().flex_wrap();

    // Single gap value on both axes (contract §8).
    if let Some(gap_token) = spec.resolved_column_gap() {
        el = el.gap(resolve_px(theme, gap_token));
    }

    if let Some(h) = padding.horizontal {
        let px_val = resolve_px(theme, h);
        el = el.pl(px_val).pr(px_val);
    }
    if let Some(v) = padding.vertical {
        let px_val = resolve_px(theme, v);
        el = el.pt(px_val).pb(px_val);
    }

    for (i, child) in children.into_iter().enumerate() {
        let wrapper = match &columns {
            GridColumns::AutoFit { min_rem } => {
                let min_px = rem_to_px(*min_rem);
                ui_element::div()
                    .flex_grow()
                    .flex_basis(min_px)
                    .min_w(min_px)
                    .child(child)
            }
            GridColumns::Tracks(tracks) if !tracks.is_empty() => {
                match tracks[i % tracks.len()] {
                    // Fr tracks → equal columns (weighted ratios degrade; see DELTA).
                    GridTrack::Fr(_) => ui_element::div().flex_grow().min_w_0().child(child),
                    GridTrack::Rem(rem) => {
                        ui_element::div().w(rem_to_px(rem)).flex_shrink_0().child(child)
                    }
                }
            }
            GridColumns::Tracks(_) => ui_element::div().flex_grow().min_w_0().child(child),
        };
        el = el.child(wrapper);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::PaddingScale;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn cells(n: usize) -> Vec<JsEl> {
        (0..n).map(|i| ui_element::label(&format!("c{i}"))).collect()
    }

    #[test]
    fn three_equal_columns_lay_out_in_one_row() {
        let th = theme();
        let spec = GridSpec::new()
            .with_columns("1fr 1fr 1fr")
            .with_gap(PaddingScale::Md);
        let el = js_grid(&spec, &th, cells(3));
        let tree = probe(&el, 600.0, 200.0);

        // All three cell labels survive layout.
        for i in 0..3 {
            assert!(tree.has_text(&format!("c{i}")), "missing cell c{i}: {:?}", tree.texts());
        }
        // Equal-fr wrappers each grew to roughly a third of the row, so all
        // three sit on the same row (same y) with positive width.
        let labels: Vec<_> = tree.nodes.iter().filter(|n| n.kind == "Label").collect();
        assert_eq!(labels.len(), 3, "expected 3 cell labels");
        let y0 = labels[0].y;
        assert!(
            labels.iter().all(|n| (n.y - y0).abs() < 1.0),
            "equal columns should share one row: {:?}",
            labels.iter().map(|n| (n.x, n.y, n.w)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn gap_resolves_to_panel_y_token() {
        // Contract §8: gap md → space.panel.y (the fix; was wrongly inline-md).
        // The token key is the load-bearing assertion — in the DARK theme
        // panel.y and inline.md happen to share the same px value, so the probe
        // can't distinguish them numerically; the spec key proves the scale.
        let th = theme();
        let spec = GridSpec::new().with_gap(PaddingScale::Md);
        assert_eq!(spec.resolved_column_gap(), Some("space.panel.y"));
        assert_eq!(spec.resolved_row_gap(), Some("space.panel.y"));
        let el = js_grid(&spec, &th, cells(2));
        let tree = probe(&el, 400.0, 200.0);
        assert!(!tree.is_empty(), "grid produced no nodes");
    }

    #[test]
    fn auto_fit_min_width_is_honored() {
        let th = theme();
        let spec = GridSpec::new()
            .with_columns("repeat(auto-fit, minmax(8rem, 1fr))")
            .with_gap(PaddingScale::Sm);
        // Narrow viewport forces wrapping; each cell keeps its 8rem (128px) min.
        let el = js_grid(&spec, &th, cells(5));
        let tree = probe(&el, 300.0, 400.0);
        let min_px = rem_to_px(8.0);
        let wrappers: Vec<_> = tree
            .nodes
            .iter()
            .filter(|n| n.kind == "Panel" && n.depth == 1)
            .collect();
        assert_eq!(wrappers.len(), 5, "expected 5 cell wrappers");
        for w in &wrappers {
            assert!(
                w.w + 0.5 >= min_px,
                "auto-fit cell width {} below 8rem min {min_px}",
                w.w
            );
        }
    }

    #[test]
    fn padding_applied_from_grid_scale() {
        let th = theme();
        let spec = GridSpec::new().with_padding(PaddingScale::Md);
        // md padding → space.panel.y on both axes.
        let inset = spec.resolved_padding();
        assert_eq!(inset.horizontal, Some("space.panel.y"));
        assert_eq!(inset.vertical, Some("space.panel.y"));
        let el = js_grid(&spec, &th, cells(2));
        let tree = probe(&el, 400.0, 200.0);
        // Root has padding, so first child starts inset from the left edge.
        let first_label = tree.nodes.iter().find(|n| n.kind == "Label").unwrap();
        assert!(first_label.x > 0.0, "padding should inset children: {first_label:?}");
    }
}
