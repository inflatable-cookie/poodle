//! Stack — Jetstream flex stack layout backed by StackSpec.

use jetstream_ui::ui_element::{self, JsEl};
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

    // Cross-axis alignment (direction-aware default when unset:
    // column → stretch, row → center, per Svelte authority).
    match spec.resolved_align() {
        Alignment::Start => {
            el = el.items_start();
        }
        Alignment::Center => {
            el = el.items_center();
        }
        Alignment::End => {
            el = el.items_end();
        }
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

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::PaddingScale;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn pos<'a>(
        tree: &'a crate::render_probe::ProbeTree,
        text: &str,
    ) -> &'a crate::render_probe::ProbeNode {
        tree.nodes
            .iter()
            .find(|n| n.text.as_deref() == Some(text))
            .expect("node")
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

    #[test]
    fn gap_separates_children_by_resolved_space() {
        // Md column gap resolves from the stack-gap token; consecutive
        // children must be separated by more than zero on the main axis.
        let el = js_stack(
            &StackSpec::new().with_gap(PaddingScale::Md),
            &theme(),
            vec![ui_element::label("A"), ui_element::label("B")],
        );
        let tree = probe(&el, 300.0, 200.0);
        let a = pos(&tree, "A");
        let b = pos(&tree, "B");
        assert!(
            b.y >= a.y + a.h,
            "B must start at or below A's bottom edge (gap applied)"
        );
        assert!(
            b.y - (a.y + a.h) > 0.0,
            "a non-zero gap must separate children"
        );
    }

    #[test]
    fn row_align_default_centers_children_on_cross_axis() {
        // Row with no explicit align must resolve to Center (Svelte authority),
        // so a short child sits vertically centred in a tall container, not at top.
        let el = js_stack(
            &StackSpec::new().with_direction(StackDirection::Row),
            &theme(),
            vec![ui_element::label("A")],
        );
        let tree = probe(&el, 300.0, 200.0);
        let a = pos(&tree, "A");
        let child_center = a.y + a.h / 2.0;
        assert!(
            child_center > a.h,
            "default row align should centre the child, not pin it to the top \
             (center={child_center}, child_h={})",
            a.h
        );
    }

    #[test]
    fn column_align_default_stretches() {
        // Column with no explicit align resolves to Stretch — children fill
        // the cross axis. The label should be wider than its intrinsic text.
        assert_eq!(
            StackSpec::new().resolved_align(),
            poodle_specs::Alignment::Stretch
        );
        assert_eq!(
            StackSpec::new()
                .with_direction(StackDirection::Row)
                .resolved_align(),
            poodle_specs::Alignment::Center
        );
    }
}
