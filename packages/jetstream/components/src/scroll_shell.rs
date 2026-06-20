//! ScrollShell — Jetstream scrollable container backed by ScrollShellSpec.
//!
//! Contract: `docs/contracts/components/scroll-shell.md`
//!
//! Honors `direction` (scroll axis) and `padding` (token-resolved inset). Role/
//! label/focus are ARIA/interaction concerns with no Jetstream render surface
//! (accepted deltas). Previously a stub that ignored the spec entirely.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{Direction, ScrollShellSpec};

use crate::theme_ext::{resolve_px, resolve_radius};

/// Build a scroll-shell from its spec + children.
pub fn js_scroll_shell(
    spec: &ScrollShellSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
) -> JsEl {
    // Direction sets the layout axis + which overflow scrolls.
    let mut el = match spec.direction {
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
    .grow()
    .rounded(resolve_radius(theme, "radius.surface"));

    // Token-resolved padding inset.
    let inset = spec.resolved_padding();
    if let Some(h) = inset.horizontal {
        let p = resolve_px(theme, h);
        el = el.pl(p).pr(p);
    }
    if let Some(v) = inset.vertical {
        let p = resolve_px(theme, v);
        el = el.pt(p).pb(p);
    }

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
}
