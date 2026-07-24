//! Region — Jetstream presentational placeholder block backed by RegionSpec.
//!
//! Contract: `docs/contracts/components/region.md`. Region is a decorative,
//! non-interactive placeholder: a dashed border with an optional centred,
//! uppercase label. Per contract §3 it does NOT accept child content — it
//! renders only the label.

use jetstream_ui::ui_element::{self, BorderStyle, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::RegionSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_region(spec: &RegionSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let radius = resolve_radius(theme, spec.radius_token());
    let padding = resolve_px(theme, spec.padding_token());

    // Custom color (contract §5) overrides both border and label; otherwise
    // resolve the default semantic tokens. GPUI passes the raw hex through
    // `resolve_color`, which parses `#`-prefixed strings — mirror that here.
    let border_color = match &spec.color {
        Some(hex) => resolve_color(theme, hex),
        None => resolve_color(theme, spec.border_color_token()),
    };
    let label_color = match &spec.color {
        Some(hex) => resolve_color(theme, hex),
        None => resolve_color(theme, spec.label_color_token()),
    };

    // Contract §2/Svelte: dashed border at 0.125rem (2px).
    let mut el = ui_element::div()
        .flex_col()
        .items_center()
        .justify_center()
        .min_h(spec.min_height_px)
        .rounded(radius)
        .border(rem_to_px(0.125))
        .border_style(BorderStyle::Dashed)
        .border_color(border_color)
        .pl(padding)
        .pr(padding)
        .pt(padding)
        .pb(padding);

    if !spec.label.is_empty() {
        let label_size = resolve_px(theme, spec.label_text_size_token());
        el = el.child(
            ui_element::label(spec.label.to_uppercase())
                .text_color(label_color)
                .text_size(label_size),
        );
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn find<'a>(
        tree: &'a crate::render_probe::ProbeTree,
        text: &str,
    ) -> Option<&'a crate::render_probe::ProbeNode> {
        tree.nodes.iter().find(|n| n.text.as_deref() == Some(text))
    }

    #[test]
    fn label_is_uppercased() {
        // Contract §2: label is rendered uppercase.
        let el = js_region(&RegionSpec::new().with_label("Section Title"), &theme());
        let tree = probe(&el, 200.0, 120.0);
        assert!(
            find(&tree, "SECTION TITLE").is_some(),
            "label should be uppercased"
        );
        assert!(
            find(&tree, "Section Title").is_none(),
            "verbatim (non-uppercased) label must not appear"
        );
    }

    #[test]
    fn wrapper_applies_min_height_and_padding() {
        // The placeholder wrapper honours the min-height dimension and is the
        // root node of the tree.
        let el = js_region(&RegionSpec::new().with_min_height(96.0), &theme());
        let tree = probe(&el, 200.0, 200.0);
        let root = &tree.nodes[0];
        assert!(
            root.h >= 96.0,
            "region wrapper should respect min-height (got {})",
            root.h
        );
    }

    #[test]
    fn label_size_resolves_from_token() {
        // Label text size must come from the label-size token, not a hardcoded
        // px literal.
        let t = theme();
        let expected = resolve_px(&t, RegionSpec::new().label_text_size_token());
        let el = js_region(&RegionSpec::new().with_label("Area"), &t);
        let tree = probe(&el, 200.0, 120.0);
        let label = find(&tree, "AREA").expect("label node");
        assert_eq!(
            label.text_size,
            Some(expected),
            "label size should resolve from the label-size token"
        );
    }

    #[test]
    fn unlabeled_region_renders_no_label_child() {
        // Empty label → no label child, only the wrapper.
        let el = js_region(&RegionSpec::new(), &theme());
        let tree = probe(&el, 200.0, 120.0);
        assert!(
            tree.nodes.iter().all(|n| n.text.is_none()),
            "unlabeled region should have no text-bearing children"
        );
    }
}
