//! Stack — Jetstream flex stack layout backed by StackSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{Alignment, LayoutJustify, StackDirection, StackSpec};

use crate::theme_ext::resolve_px;

pub fn js_stack(spec: &StackSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let padding = spec.resolved_padding();

    // Direction sets the main axis (was hardcoded column).
    let mut el = match spec.direction {
        StackDirection::Column => ui_element::div().flex_col(),
        StackDirection::Row => ui_element::div().flex_row(),
    };

    if spec.wrap {
        el = el.flex_wrap();
    }

    if let Some(gap_token) = spec.resolved_gap() {
        el = el.gap(resolve_px(theme, gap_token));
    }

    match spec.align {
        Alignment::Start => { el = el.items_start(); }
        Alignment::Center => { el = el.items_center(); }
        Alignment::End => { el = el.items_end(); }
        Alignment::Stretch => {} // default flex behavior
    }

    if let Some(justify) = spec.justify {
        el = match justify {
            LayoutJustify::Start => el.justify_start(),
            LayoutJustify::End => el.justify_end(),
            LayoutJustify::Center => el.justify_center(),
            LayoutJustify::SpaceBetween => el.justify_between(),
        };
    }

    if let Some(h) = padding.horizontal {
        let px_val = resolve_px(theme, h);
        el = el.pl(px_val).pr(px_val);
    }
    if let Some(v) = padding.vertical {
        let px_val = resolve_px(theme, v);
        el = el.pt(px_val).pb(px_val);
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

    fn pos<'a>(tree: &'a crate::render_probe::ProbeTree, text: &str) -> &'a crate::render_probe::ProbeNode {
        tree.nodes.iter().find(|n| n.text.as_deref() == Some(text)).expect("node")
    }

    #[test]
    fn row_direction_lays_children_horizontally() {
        let el = js_stack(
            &StackSpec::new()
                .with_direction(StackDirection::Row)
                .with_gap(PaddingScale::Md),
            &theme(),
            vec![ui_element::label("A"), ui_element::label("B")],
        );
        let tree = probe(&el, 300.0, 80.0);
        assert!(
            pos(&tree, "B").x > pos(&tree, "A").x,
            "row should place B to the right of A"
        );
    }

    #[test]
    fn column_direction_stacks_vertically() {
        let el = js_stack(
            &StackSpec::new().with_gap(PaddingScale::Md),
            &theme(),
            vec![ui_element::label("A"), ui_element::label("B")],
        );
        let tree = probe(&el, 300.0, 200.0);
        assert!(
            pos(&tree, "B").y > pos(&tree, "A").y,
            "column should place B below A"
        );
    }
}
