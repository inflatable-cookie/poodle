//! FieldSet — Jetstream fieldset grouping wrapper backed by FieldSetSpec.
//!
//! Contract: `docs/contracts/components/field-set.md`
//! Reference: `packages/svelte/components/src/FieldSet.svelte`
//!
//! Renders a fieldset wrapper with an optional legend, an optional description,
//! and a multi-column grid for slotted field content. Children are passed in as
//! a Vec of JsEl.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::FieldSetSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

/// Build the fieldset element. `children` are the slotted field controls.
pub fn js_field_set(
    spec: &FieldSetSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
) -> JsEl {
    // Column-gap = scaleToSpace(gap); None → 0 (contract §6).
    let col_gap = spec
        .column_gap_token()
        .map(|t| resolve_px(theme, t))
        .unwrap_or(0.0);
    // Row-gap = column-gap + 0.5rem (asymmetric, contract §6).
    let row_gap = col_gap + rem_to_px(FieldSetSpec::ROW_GAP_EXTRA_REM);
    let legend_color = resolve_color(theme, spec.legend_color_token());
    // Legend is a fixed 0.6875rem eyebrow scale (NOT typography-label-size,
    // contract §6).
    let legend_size = rem_to_px(FieldSetSpec::LEGEND_SIZE_REM);

    // `span` (FieldSetSpec) is a CSS-grid placement prop with no Jetstream flex
    // equivalent — accepted layout delta (contract §6 `spanned` / §12); the
    // owning layout positions the fieldset.

    // Root stacks legend → description → fields grid.
    let mut root = ui_element::div().flex_col();

    // Legend (optional). Contract §6 letter-spacing 0.12em; line-height 1.5 has
    // no JsEl API (accepted rendering delta); casing applied via to_uppercase().
    if let Some(ref legend) = spec.legend {
        root = root.child(
            ui_element::label(legend.to_uppercase())
                .text_color(legend_color)
                .text_size(legend_size)
                .text_weight(600)
                .letter_spacing_em(0.12)
                .mb(resolve_px(theme, spec.legend_margin_bottom_token())),
        );
    }

    // Description (optional) — <p> between legend and fields (contract §2).
    if let Some(ref description) = spec.description {
        root = root.child(
            ui_element::label(description)
                .text_color(resolve_color(theme, spec.description_color_token()))
                .text_size(resolve_px(theme, spec.description_size_token()))
                .mb(resolve_px(theme, spec.description_margin_bottom_token())),
        );
    }

    // Fields grid. Equal-fraction columns (≈ repeat(columns, minmax(0,1fr)))
    // via flex_wrap + per-child flex-basis. Asymmetric gaps: wrap row-gap uses
    // the larger row_gap; inline column-gap uses col_gap.
    let cols = spec.columns.max(1);
    let mut grid = ui_element::div().flex_row().flex_wrap().gap(col_gap);

    for child in children {
        let mut wrapper = ui_element::div().child(child).min_w_0();
        if cols > 1 {
            // Equal share of width: 1/cols, with grow/shrink so rows fill.
            let basis_pct = 100.0 / cols as f32;
            wrapper = wrapper.flex_basis(basis_pct).flex_grow().flex_shrink();
        } else {
            wrapper = wrapper.grow();
        }
        grid = grid.child(wrapper);
    }

    // Drive the larger row-gap on the grid when it wraps to multiple rows.
    // (flex has a single gap; column-gap is exact, the wrap row-gap reuses it —
    // applying the +0.5rem row-gap as the flex gap when multi-column.)
    if cols > 1 {
        grid = grid.gap(row_gap);
    }

    root = root.child(grid);
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{FieldSetSpec, SpaceScale};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn fieldset_renders_legend_uppercased_description_and_children() {
        let th = theme();
        let spec = FieldSetSpec::new()
            .with_legend("Contact")
            .with_description("How we reach you.");
        let children = vec![
            ui_element::label("Email").into(),
            ui_element::label("Phone").into(),
        ];
        let el = js_field_set(&spec, &th, children);
        let tree = probe(&el, 600.0, 400.0);

        // Legend rendered uppercased (text-transform applied in Rust).
        assert!(
            tree.has_text("CONTACT"),
            "uppercased legend missing: {:?}",
            tree.texts()
        );
        // Description part present (contract §2 — between legend and fields).
        assert!(
            tree.has_text("How we reach you."),
            "description missing: {:?}",
            tree.texts()
        );
        // Grouped children present.
        assert!(tree.has_text("Email") && tree.has_text("Phone"), "fields missing: {:?}", tree.texts());
    }

    #[test]
    fn fieldset_legendless_omits_legend() {
        let th = theme();
        let spec = FieldSetSpec::new();
        let el = js_field_set(&spec, &th, vec![ui_element::label("Only field").into()]);
        let tree = probe(&el, 400.0, 200.0);
        assert!(tree.has_text("Only field"));
    }

    #[test]
    fn fieldset_multi_column_lays_out_children_side_by_side() {
        let th = theme();
        let spec = FieldSetSpec::new().with_columns(2).with_gap(SpaceScale::Md);
        let children = vec![
            ui_element::label("First").w(80.0).h(20.0).into(),
            ui_element::label("Second").w(80.0).h(20.0).into(),
        ];
        let el = js_field_set(&spec, &th, children);
        let tree = probe(&el, 600.0, 400.0);

        // Both column wrappers laid out; with 2 columns at 600px width the two
        // field wrappers should sit on the same row (similar y).
        let first = tree.nodes.iter().find(|n| n.text.as_deref() == Some("First"));
        let second = tree.nodes.iter().find(|n| n.text.as_deref() == Some("Second"));
        assert!(first.is_some() && second.is_some(), "both columns present");
        let (f, s) = (first.unwrap(), second.unwrap());
        assert!(
            (f.y - s.y).abs() < 5.0,
            "two-column children should share a row: y1={}, y2={}",
            f.y,
            s.y
        );
        assert!(s.x > f.x, "second column should be to the right of first");
    }

    #[test]
    fn fieldset_gap_none_resolves_to_zero() {
        let th = theme();
        let spec = FieldSetSpec::new().with_gap(SpaceScale::None);
        // gap_token is None → builders use 0 column-gap.
        assert!(spec.column_gap_token().is_none());
        let el = js_field_set(&spec, &th, vec![ui_element::label("X").into()]);
        let tree = probe(&el, 400.0, 200.0);
        assert!(tree.has_text("X"));
    }
}
