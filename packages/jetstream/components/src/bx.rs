//! Box — Jetstream container component backed by BoxSpec.
//!
//! Contract: `docs/contracts/components/box.md`
//! Reference: `packages/svelte/components/src/Box.svelte`, GPUI `primitives/bx.rs`
//!
//! Box is a neutral layout container: padding, explicit sizing, and overflow
//! control only. Background / border / directional layout are out of scope per
//! contract §1.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{BoxSpec, Dimension, Overflow};

use crate::theme_ext::resolve_px;

/// Resolve a `Dimension` string to logical px (`px`, `rem` × 16, bare number).
/// Returns `None` for `%` / `calc()` / viewport units (no absolute px); `100%`
/// is handled separately by the caller via `w_full`/`h_full`.
fn parse_dimension_px(dim: &Dimension) -> Option<f32> {
    let s = dim.as_str().trim();
    if let Some(stripped) = s.strip_suffix("px") {
        stripped.trim().parse::<f32>().ok()
    } else if let Some(stripped) = s.strip_suffix("rem") {
        stripped.trim().parse::<f32>().ok().map(|rem| rem * 16.0)
    } else {
        s.parse::<f32>().ok()
    }
}

pub fn js_box(spec: &BoxSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let padding = spec.resolved_padding();

    let mut el = ui_element::div();

    // Explicit sizing constraints (contract §7). `100%` maps to full; other
    // non-px units (calc, vh, %) have no JsEl analogue and are dropped (note).
    if let Some(w) = &spec.width {
        if let Some(v) = parse_dimension_px(w) {
            el = el.w(v);
        } else if w.as_str().trim() == "100%" {
            el = el.w_full();
        }
    }
    if let Some(h) = &spec.height {
        if let Some(v) = parse_dimension_px(h) {
            el = el.h(v);
        } else if h.as_str().trim() == "100%" {
            el = el.h_full();
        }
    }
    if let Some(mw) = &spec.min_width {
        if let Some(v) = parse_dimension_px(mw) {
            el = el.min_w(v);
        }
    }
    if let Some(mh) = &spec.min_height {
        if let Some(v) = parse_dimension_px(mh) {
            el = el.min_h(v);
        }
    }

    if let Some(h) = padding.horizontal {
        let px_val = resolve_px(theme, h);
        el = el.pl(px_val).pr(px_val);
    }
    if let Some(v) = padding.vertical {
        let px_val = resolve_px(theme, v);
        el = el.pt(px_val).pb(px_val);
    }

    // Overflow (contract §3 `OverflowMode`, 4 values). JsEl exposes clip-style
    // `overflow_hidden` and scroll-style `overflow_scroll`:
    //   Hidden / Clip  -> clip (no scroll)
    //   Auto / Scroll  -> scroll container
    //   Visible        -> default (no clip)
    match spec.overflow {
        Overflow::Hidden | Overflow::Clip => {
            el = el.overflow_hidden();
        }
        Overflow::Auto | Overflow::Scroll => {
            el = el.overflow_scroll();
        }
        Overflow::Visible => {}
    }

    // `role` / `aria_label` (contract §3) have no Jetstream accessibility channel
    // — JsEl emits no a11y tree — so they are intentionally not applied here.
    // Accepted runtime limit (mirrors GPUI, which also lacks an a11y API).

    for child in children {
        el = el.child(child);
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

    #[test]
    fn renders_children() {
        let el = js_box(
            &BoxSpec::new(),
            &theme(),
            vec![ui_element::label("Inside")],
        );
        let tree = probe(&el, 200.0, 100.0);
        assert!(tree.has_text("Inside"), "child missing: {:?}", tree.texts());
    }

    #[test]
    fn fixed_rem_dimensions_resolve_to_px() {
        // width="12rem" height="6rem" -> 192 × 96 logical px (16px base).
        let el = js_box(
            &BoxSpec::new()
                .with_width("12rem")
                .with_height("6rem"),
            &theme(),
            vec![],
        );
        let tree = probe(&el, 400.0, 400.0);
        let root = &tree.nodes[0];
        assert!((root.w - 192.0).abs() < 0.5, "width: {root:?}");
        assert!((root.h - 96.0).abs() < 0.5, "height: {root:?}");
    }

    #[test]
    fn min_dimensions_constrain_root() {
        // min-width forces the box wider than its (empty) content.
        let el = js_box(
            &BoxSpec::new().with_min_width("8rem"),
            &theme(),
            vec![],
        );
        let tree = probe(&el, 400.0, 100.0);
        let root = &tree.nodes[0];
        assert!(root.w >= 128.0 - 0.5, "min-width applied: {root:?}");
    }

    #[test]
    fn padding_from_tokens_insets_child() {
        // md padding resolves from space tokens and insets the child rect.
        let el = js_box(
            &BoxSpec::new().with_padding(PaddingScale::Md),
            &theme(),
            vec![ui_element::label("x")],
        );
        let tree = probe(&el, 200.0, 100.0);
        let child = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("x"))
            .expect("child node present");
        let pad_x = resolve_px(&theme(), poodle_tokens::semantic::SPACE_INLINE_MD);
        assert!(pad_x > 0.0, "md padding token resolves > 0");
        assert!(child.x >= pad_x - 0.5, "child inset by horizontal padding: {child:?}");
    }

    #[test]
    fn overflow_scroll_becomes_scroll_container() {
        // Auto/Scroll map to a scroll container (List widget).
        let el = js_box(
            &BoxSpec::new().with_overflow(Overflow::Auto),
            &theme(),
            vec![ui_element::label("scrolly")],
        );
        let tree = probe(&el, 100.0, 50.0);
        assert_eq!(tree.nodes[0].kind, "List", "auto overflow -> scroll list: {:?}", tree.nodes[0]);
    }
}
