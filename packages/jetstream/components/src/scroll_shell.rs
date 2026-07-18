//! ScrollShell — Jetstream scrollable container backed by ScrollShellSpec.
//!
//! Contract: `docs/contracts/components/scroll-shell.md`
//!
//! Three-layer anatomy per contract §2:
//!   Root (.scroll-shell)       → clip boundary: radius-surface
//!   Viewport (.scroll-shell__viewport) → scroll owner: per-axis overflow, padding
//!   Content (.scroll-shell__content)   → sizing wrapper: horizontal max-content
//!
//! Honors `direction` (scroll axis) and `padding` (token-resolved inset). Role/
//! label/focus are ARIA/interaction concerns with no Jetstream render surface,
//! and keyboard scroll lives in the preview event loop (accepted deltas).

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{Direction, ScrollShellSpec};

use crate::theme_ext::{resolve_px, resolve_radius};

/// Build a scroll-shell from its spec + children.
pub fn js_scroll_shell(
    spec: &ScrollShellSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
) -> JsEl {
    let needs_horizontal = matches!(spec.direction, Direction::Horizontal | Direction::Both);

    // ── Content (.scroll-shell__content) — sizing wrapper ──
    // For horizontal/both the content must not collapse: a non-shrinking row
    // sized to its children is JsEl's `min-width: max-content` analogue
    // (contract §8 Content). Vertical content stacks and fills the width.
    let mut content = if needs_horizontal {
        ui_element::div().flex_row().flex_shrink_0()
    } else {
        ui_element::div().flex_col().w_full()
    };
    for child in children {
        content = content.child(child);
    }

    // ── Viewport (.scroll-shell__viewport) — scroll owner ──
    // Direction sets the layout axis + which overflow scrolls.
    let mut viewport = match spec.direction {
        Direction::Horizontal => ui_element::div()
            .flex_row()
            .overflow_scroll()
            .overflow_y_hidden(),
        Direction::Vertical => ui_element::div()
            .flex_col()
            .overflow_scroll()
            .overflow_x_hidden(),
        Direction::Both => ui_element::div().flex_col().overflow_scroll(),
    }
    .grow();

    // Token-resolved padding inset on the viewport (contract §8 padding scale).
    let inset = spec.resolved_padding();
    if let Some(h) = inset.horizontal {
        let p = resolve_px(theme, h);
        viewport = viewport.pl(p).pr(p);
    }
    if let Some(v) = inset.vertical {
        let p = resolve_px(theme, v);
        viewport = viewport.pt(p).pb(p);
    }

    viewport = viewport.child(content);

    // ── Root (.scroll-shell) — clip boundary ──
    ui_element::div()
        .grow()
        .rounded(resolve_radius(theme, "radius.surface"))
        .child(viewport)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::PaddingScale;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_children() {
        let el = js_scroll_shell(
            &ScrollShellSpec::new(),
            &theme(),
            vec![ui_element::label("row 1"), ui_element::label("row 2")],
        );
        let tree = probe(&el, 200.0, 300.0);
        assert!(tree.has_text("row 1") && tree.has_text("row 2"));
    }

    #[test]
    fn padding_offsets_first_child() {
        // With a non-zero padding scale, the first child is inset from the edge.
        let el = js_scroll_shell(
            &ScrollShellSpec::new().with_padding(PaddingScale::Lg),
            &theme(),
            vec![ui_element::label("x")],
        );
        let tree = probe(&el, 200.0, 300.0);
        let child = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("x"))
            .unwrap();
        assert!(
            child.x > 0.0 || child.y > 0.0,
            "padding not applied: child at ({}, {})",
            child.x,
            child.y
        );
    }

    #[test]
    fn three_layer_anatomy() {
        // Root → Viewport → Content → child: the labeled child sits at depth >= 3.
        let el = js_scroll_shell(
            &ScrollShellSpec::new(),
            &theme(),
            vec![ui_element::label("deep")],
        );
        let tree = probe(&el, 200.0, 300.0);
        let child = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("deep"))
            .unwrap();
        assert!(
            child.depth >= 3,
            "expected root/viewport/content wrapping, child at depth {}",
            child.depth
        );
    }

    #[test]
    fn horizontal_lays_out_in_a_row() {
        // Horizontal direction: two children sit side by side (distinct x).
        let el = js_scroll_shell(
            &ScrollShellSpec::new().with_direction(Direction::Horizontal),
            &theme(),
            vec![ui_element::label("a"), ui_element::label("b")],
        );
        let tree = probe(&el, 400.0, 100.0);
        let ax = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("a"))
            .unwrap()
            .x;
        let bx = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("b"))
            .unwrap()
            .x;
        assert!(bx > ax, "horizontal children not in a row: a.x={ax} b.x={bx}");
    }
}
