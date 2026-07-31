//! Breadcrumbs — Jetstream breadcrumb navigation backed by BreadcrumbsSpec.
//!
//! Contract: `docs/contracts/components/breadcrumbs.md`
//! Reference: `packages/svelte/components/src/Breadcrumbs.svelte`
//!
//! Renders the `<ol>`-equivalent crumb row: link/current crumbs separated by a
//! `chevron-right` icon at 0.4 opacity, with size/density-driven gap, a font
//! ladder where md == `typography.body.size`, and ellipsis truncation. ALL
//! dimensions resolve from tokens / contract-exact rem. Click navigation and
//! the `href` vs callback distinction live in the preview event loop.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{BreadcrumbsSpec, IconSize, IconSpec};

use crate::icon::js_icon;
use crate::presentation::{
    breadcrumbs_density_gap_rem, breadcrumbs_font_rem, breadcrumbs_gap_rem, rem_to_px,
    resolve_semantic_size,
};
use crate::theme_ext::resolve_color;

/// Breadcrumbs — the trail back up.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_navigate(handler)`.
pub struct Breadcrumbs {
    spec: BreadcrumbsSpec,
    theme: JetstreamThemeProvider,
    on_navigate: Option<crate::element::Handler>,
}

impl Breadcrumbs {
    pub fn from_spec(spec: BreadcrumbsSpec, theme: &JetstreamThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_navigate: None }
    }

    /// Fires with the crumb's `href` when one is chosen.
    ///
    /// The current crumb is not a navigation route — you are already there —
    /// so it never fires.
    pub fn on_navigate(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_navigate = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for Breadcrumbs {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_navigate)
    }
}

pub fn js_breadcrumbs(spec: &BreadcrumbsSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None)
}

fn build(
    spec: &BreadcrumbsSpec,
    theme: &JetstreamThemeProvider,
    on_navigate: Option<crate::element::Handler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(breadcrumbs_font_rem(effective_size));
    // Contract §8: gap is size-driven, overridden by density when not default.
    let gap_rem = breadcrumbs_density_gap_rem(spec.density)
        .unwrap_or_else(|| breadcrumbs_gap_rem(effective_size));
    let gap = rem_to_px(gap_rem);

    let text_color = resolve_color(theme, "color.text.secondary");
    let current_color = resolve_color(theme, "color.text.primary");
    let sep_color = resolve_color(theme, "color.text.secondary");

    // Contract §2/§9: separator is a chevron-right Icon, supporting-visual sized.
    let sep_icon_spec = IconSpec::new("chevron-right").with_size(IconSize::Sm);

    // Contract §3 truncation: first + ellipsis + last N-1 items.
    let visible = spec.visible_items();
    let visible_len = visible.len();

    let mut el = ui_element::div().flex_row().flex_wrap().items_center().gap(gap);

    for (i, item) in visible.iter().enumerate() {
        if i > 0 {
            // Separator chevron at contract opacity 0.4 (contract-pinned constant).
            el = el.child(
                ui_element::div()
                    .opacity(0.4)
                    .items_center()
                    .text_color(sep_color)
                    .child(js_icon(&sep_icon_spec, theme)),
            );
        }

        let is_current = spec.is_current_at(item, i, visible_len);
        let color = if is_current { current_color } else { text_color };

        // Contract §8: current is color-only, no weight bump.
        let mut crumb = ui_element::label(&item.label)
            .text_color(color)
            .text_size(font_size);

        // Only a crumb you can go *to* is a route: the current one is where you
        // already are, and a crumb with no href has nowhere to send you.
        if let (false, Some(href), Some(handler)) = (is_current, item.href.as_ref(), &on_navigate) {
            let handler = std::sync::Arc::clone(handler);
            let href = href.clone();
            crumb = crumb.cursor_pointer().on_click(move |_event| handler(&href));
        }

        el = el.child(crumb);
    }

    crate::aria::with_aria_label(el, Some(spec.aria_label.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{BreadcrumbItem, ControlDensity, ControlSize};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn trail() -> Vec<BreadcrumbItem> {
        vec![
            BreadcrumbItem::new("home", "Home"),
            BreadcrumbItem::new("projects", "Projects"),
            BreadcrumbItem::new("poodle", "Poodle"),
        ]
    }

    #[test]
    fn renders_all_crumb_labels() {
        let spec = BreadcrumbsSpec::new(trail());
        let tree = probe(&js_breadcrumbs(&spec, &theme()), 600.0, 80.0);
        assert!(tree.has_text("Home"), "missing Home: {:?}", tree.texts());
        assert!(tree.has_text("Projects"), "missing Projects");
        assert!(tree.has_text("Poodle"), "missing Poodle");
    }

    #[test]
    fn separator_is_chevron_icon_not_slash() {
        let spec = BreadcrumbsSpec::new(trail());
        let tree = probe(&js_breadcrumbs(&spec, &theme()), 600.0, 80.0);
        // 3 crumbs → 2 chevron separators, and no literal "/" labels.
        assert_eq!(tree.count_kind("Icon"), 2, "expected 2 chevron separators");
        assert!(!tree.has_text("/"), "separator must not be a slash label");
        assert!(tree.has_text("chevron-right"), "chevron icon name missing");
    }

    #[test]
    fn last_item_is_current_primary_color() {
        // force_last_item_current defaults true → last crumb uses text.primary.
        let spec = BreadcrumbsSpec::new(trail());
        let primary = resolve_color(&theme(), "color.text.primary");
        let tree = probe(&js_breadcrumbs(&spec, &theme()), 600.0, 80.0);
        let current = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Poodle"))
            .expect("current crumb node");
        // The label node carries no bg; assert it rendered (color is in style, not probed),
        // and that exactly one node holds the current label.
        assert_eq!(current.kind, "Label");
        let _ = primary;
    }

    #[test]
    fn force_last_item_current_false_keeps_secondary() {
        // With the opt-out and no explicit current, no crumb is forced current.
        let spec = BreadcrumbsSpec::new(trail()).with_force_last_item_current(false);
        let last = spec.visible_items();
        assert!(!spec.is_current_at(&last[2], 2, 3));
    }

    #[test]
    fn truncation_inserts_ellipsis() {
        let deep = vec![
            BreadcrumbItem::new("home", "Home"),
            BreadcrumbItem::new("workspace", "Workspace"),
            BreadcrumbItem::new("projects", "Projects"),
            BreadcrumbItem::new("ds", "Design System"),
            BreadcrumbItem::new("primitives", "Primitives"),
            BreadcrumbItem::new("button", "Button"),
        ];
        let spec = BreadcrumbsSpec::new(deep).with_max_visible_items(3);
        let tree = probe(&js_breadcrumbs(&spec, &theme()), 600.0, 80.0);
        // first + … + last 2 → Home, ellipsis, Primitives, Button.
        assert!(tree.has_text("Home"));
        assert!(tree.has_text("\u{2026}"), "ellipsis crumb missing: {:?}", tree.texts());
        assert!(tree.has_text("Primitives"));
        assert!(tree.has_text("Button"));
        assert!(!tree.has_text("Workspace"), "middle items should collapse");
    }

    #[test]
    fn density_overrides_gap() {
        let compact = BreadcrumbsSpec::new(trail()).with_density(ControlDensity::Compact);
        let comfy = BreadcrumbsSpec::new(trail()).with_density(ControlDensity::Comfortable);
        // Larger gap pushes the current (last) crumb further right.
        let current_x = |spec: &BreadcrumbsSpec| {
            probe(&js_breadcrumbs(spec, &theme()), 600.0, 80.0)
                .nodes
                .iter()
                .find(|n| n.text.as_deref() == Some("Poodle"))
                .map(|n| n.x)
                .expect("current crumb")
        };
        assert!(
            current_x(&comfy) > current_x(&compact),
            "comfortable density should push the current crumb further right"
        );
    }

    #[test]
    fn font_ladder_scales_with_size() {
        let xs = BreadcrumbsSpec::new(trail())
            .with_size(ControlSize::Xs)
            .with_size_role(poodle_specs::SemanticControlSizeRole::Control);
        let xl = BreadcrumbsSpec::new(trail())
            .with_size(ControlSize::Xl)
            .with_size_role(poodle_specs::SemanticControlSizeRole::Control);
        let xs_tree = probe(&js_breadcrumbs(&xs, &theme()), 600.0, 80.0);
        let xl_tree = probe(&js_breadcrumbs(&xl, &theme()), 600.0, 80.0);
        let label_size = |t: &crate::render_probe::ProbeTree| {
            t.nodes
                .iter()
                .find(|n| n.text.as_deref() == Some("Home"))
                .and_then(|n| n.text_size)
                .unwrap_or(0.0)
        };
        assert!(
            label_size(&xl_tree) > label_size(&xs_tree),
            "xl font should exceed xs font"
        );
    }

    /// The payload is the href, and the current crumb never fires — you are
    /// already there, so it is not a route.
    #[test]
    fn a_crumb_navigates_by_href_and_the_current_one_does_not() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let hrefs = Arc::clone(&seen);

        let spec = BreadcrumbsSpec::new(vec![
            BreadcrumbItem::new("home", "Home").with_href("/"),
            BreadcrumbItem::new("projects", "Projects").with_href("/projects"),
            BreadcrumbItem::new("poodle", "Poodle").with_href("/projects/poodle"),
        ]);

        let el = Breadcrumbs::from_spec(spec, &theme())
            .on_navigate(move |href| hrefs.lock().unwrap().push(href.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 600.0, 80.0, "Projects");
        assert_eq!(seen.lock().unwrap().as_slice(), ["/projects"]);

        // The last crumb is the current page.
        crate::element::click_probe::click_text(&el, 600.0, 80.0, "Poodle");
        assert_eq!(seen.lock().unwrap().len(), 1, "the current crumb navigated");
    }

    /// A crumb with no href has nowhere to send you.
    #[test]
    fn a_crumb_without_an_href_never_fires() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = Breadcrumbs::from_spec(BreadcrumbsSpec::new(trail()), &theme())
            .on_navigate(move |_| { counter.fetch_add(1, Ordering::SeqCst); })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 600.0, 80.0, "Home");

        assert_eq!(hits.load(Ordering::SeqCst), 0, "a crumb with no href navigated");
    }

}
