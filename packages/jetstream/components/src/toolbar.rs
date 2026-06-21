//! Toolbar — Jetstream toolbar container backed by ToolbarSpec.
//!
//! Contract: `docs/contracts/components/toolbar.md`
//! Reference: `packages/svelte/components/src/Toolbar.svelte`
//!
//! A semantic grouping container for compact action controls. Auto-sizes to
//! content (inline-flex equivalent), with subtle chrome (panel-mix fill,
//! border-subtle border, radius.surface). Size scales padding + gap; density
//! overrides only inline padding + gap. Roving focus is an accepted runtime
//! limit (the preview event loop owns focus; see parity doc).

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{Alignment, Orientation, ToolbarSpec};

use crate::presentation::{
    rem_to_px, resolve_semantic_size, toolbar_density_gap_rem, toolbar_density_pad_inline_rem,
    toolbar_gap_rem, toolbar_pad_block_rem, toolbar_pad_inline_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius, tint};

pub fn js_toolbar(spec: &ToolbarSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    // Contract §8 chrome: background = color-mix(panel 94%, transparent),
    // border = color-mix(border-subtle 78%, transparent), radius = radius.surface.
    let panel_raw = resolve_color(theme, spec.bg_token());
    let bg = tint(panel_raw, 0.94);
    let border_raw = resolve_color(theme, spec.border_token());
    let border = tint(border_raw, 0.78);
    let radius = resolve_radius(theme, spec.radius_token());

    // Contract §8: size scales block/inline padding + gap; density overrides
    // only inline padding + gap (block padding / height untouched by density).
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let pad_v = rem_to_px(toolbar_pad_block_rem(effective_size));
    let pad_h = rem_to_px(
        toolbar_density_pad_inline_rem(spec.density)
            .unwrap_or_else(|| toolbar_pad_inline_rem(effective_size)),
    );
    let gap = rem_to_px(
        toolbar_density_gap_rem(spec.density).unwrap_or_else(|| toolbar_gap_rem(effective_size)),
    );

    let is_vertical = spec.orientation == Orientation::Vertical;

    // Contract §8: auto-size to content with block + inline padding (no forced
    // control-height). Vertical orientation → column layout, stretch items.
    let mut el = ui_element::div()
        .when(is_vertical, |el| el.flex_col().items_stretch())
        .when(!is_vertical, |el| el.flex_row().items_center())
        .gap(gap)
        .pt(pad_v)
        .pb(pad_v)
        .pl(pad_h)
        .pr(pad_h)
        .bg(bg)
        .rounded(radius)
        .border_1()
        .border_color(border);

    // `alignment` is a Rust-only convenience (no Svelte/contract counterpart —
    // see parity doc): justifies items along the main axis when not Start.
    match spec.alignment {
        Alignment::Start => {}
        Alignment::Center => el = el.justify_center(),
        Alignment::End => el = el.justify_end(),
        Alignment::Stretch => el = el.grow(),
    }

    for child in children {
        el = el.child(child);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{Alignment, ControlDensity, ControlSize, Orientation, ToolbarSpec};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn items() -> Vec<JsEl> {
        vec![
            ui_element::label("B"),
            ui_element::label("I"),
            ui_element::label("U"),
        ]
    }

    #[test]
    fn renders_items_in_a_bordered_chrome() {
        let th = theme();
        let el = js_toolbar(&ToolbarSpec::new(), &th, items());
        let tree = probe(&el, 400.0, 80.0);
        assert!(!tree.is_empty(), "toolbar produced no nodes");
        for v in ["B", "I", "U"] {
            assert!(tree.has_text(v), "item {v:?} missing: {:?}", tree.texts());
        }
        let root = &tree.nodes[0];
        assert!(root.w > 0.0 && root.h > 0.0, "root not laid out: {root:?}");
    }

    #[test]
    fn fills_panel_mix_background() {
        let th = theme();
        let panel = resolve_color(&th, "color.background.panel");
        let expected = tint(panel, 0.94);
        let el = js_toolbar(&ToolbarSpec::new(), &th, items());
        let tree = probe(&el, 400.0, 80.0);
        let color = ProbeColor {
            r: expected.x,
            g: expected.y,
            b: expected.z,
            a: expected.w,
        };
        assert!(
            tree.has_background(color, 0.02),
            "panel-mix toolbar fill not found"
        );
    }

    /// Probe the toolbar inside a content-sized column wrapper so the toolbar
    /// node reports its intrinsic (content + block-padding) height instead of
    /// stretching to fill the probe viewport. Returns the toolbar's height.
    fn toolbar_height(spec: &ToolbarSpec) -> f32 {
        let th = theme();
        let wrapped = ui_element::div()
            .flex_col()
            .items_start()
            .child(js_toolbar(spec, &th, items()));
        let tree = probe(&wrapped, 400.0, 200.0);
        // node[0] = wrapper, node[1] = toolbar root.
        tree.nodes[1].h
    }

    #[test]
    fn density_does_not_change_block_padding() {
        // Density never touches block padding → identical toolbar height.
        let compact = toolbar_height(&ToolbarSpec::new().with_density(ControlDensity::Compact));
        let comfortable =
            toolbar_height(&ToolbarSpec::new().with_density(ControlDensity::Comfortable));
        assert!(
            (compact - comfortable).abs() < 0.01,
            "density changed toolbar height: {compact} vs {comfortable}"
        );
    }

    #[test]
    fn size_scales_block_padding() {
        // xl block padding (0.375rem) > xs (0.125rem) → taller toolbar.
        let xs = toolbar_height(&ToolbarSpec::new().with_size(ControlSize::Xs));
        let xl = toolbar_height(&ToolbarSpec::new().with_size(ControlSize::Xl));
        assert!(
            xl > xs,
            "xl toolbar height ({xl}) not greater than xs ({xs})"
        );
    }

    #[test]
    fn vertical_orientation_stacks_items() {
        let th = theme();
        let el = js_toolbar(
            &ToolbarSpec::new().with_orientation(Orientation::Vertical),
            &th,
            items(),
        );
        let tree = probe(&el, 120.0, 300.0);
        // Three item labels at depth 1; in column layout each has a distinct y.
        let ys: Vec<f32> = tree
            .nodes
            .iter()
            .filter(|n| n.kind == "Label")
            .map(|n| n.y)
            .collect();
        assert_eq!(ys.len(), 3, "expected 3 items, got {}", ys.len());
        assert!(
            ys[0] < ys[1] && ys[1] < ys[2],
            "vertical items not stacked top-to-bottom: {ys:?}"
        );
    }

    #[test]
    fn end_alignment_justifies_items() {
        let th = theme();
        // Smoke: end-aligned toolbar still renders all items + chrome.
        let el = js_toolbar(
            &ToolbarSpec::new().with_alignment(Alignment::End),
            &th,
            items(),
        );
        let tree = probe(&el, 400.0, 80.0);
        for v in ["B", "I", "U"] {
            assert!(tree.has_text(v), "item {v:?} missing under end-align");
        }
    }
}
