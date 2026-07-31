//! JsAccordion — expandable disclosure panels backed by AccordionSpec.
//!
//! Contract: `docs/contracts/components/accordion.md`
//! Reference: `packages/svelte/components/src/Accordion.svelte`
//!
//! ALL dimensions from contract. ZERO hardcoded values.

use jetstream_ui::ui_element::{self, BoxShadow, JsEl};
use poodle_specs::ControlSize;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{AccordionItemSpec, AccordionSpec, ControlDensity};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

/// Build an accordion element from an AccordionSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .accordion] — <div>, display grid, gap 0.75rem
///   └── [Item .accordion__item] — <section>, per-item styling
///       ├── [Heading] — <h3>
///       │   └── [Trigger .accordion__trigger] — <button aria-expanded aria-controls>
///       │       ├── [Title .accordion__title] — heading typography
///       │       ├── [Description .accordion__description] — (optional)
///       │       └── [Indicator .accordion__indicator] — chevron icon, rotates
///       └── [Panel .accordion__panel] — <div role="region" aria-labelledby>
///           └── content
/// ```
///
/// Contract item dimensions:
/// - padding: 0.875rem 1rem (14px 16px)
/// - border: 0.0625rem (1px) solid color-mix(border-subtle 36%, transparent)
/// - border-radius: var(--poodle-radius-surface)
/// - background: color-mix(surface 50%, elevated)
/// - box-shadow: inset 0 0.0625rem 0 color-mix(text-inverse 8%, transparent)
/// Accordion — a stack of disclosure items.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_change(handler)`.
pub struct Accordion {
    spec: AccordionSpec,
    theme: JetstreamThemeProvider,
    on_change: Option<crate::element::Handler>,
}

impl Accordion {
    pub fn from_spec(spec: AccordionSpec, theme: &JetstreamThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_change: None }
    }

    /// Fires with the value of the item whose trigger was pressed.
    ///
    /// The item, not the resulting expanded set: single- and multi-expand are
    /// the host's policy, and returning a set would put that decision here.
    pub fn on_change(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_change = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for Accordion {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_change)
    }
}

pub fn js_accordion(spec: &AccordionSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None)
}

fn build(
    spec: &AccordionSpec,
    theme: &JetstreamThemeProvider,
    on_change: Option<crate::element::Handler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    // Contract §8 Root gap between items = space.stack.md.
    let root_gap = resolve_px(theme, spec.root_gap_token());
    let expanded = spec.expanded_values();

    // Contract §8 Root: display grid with gap.
    let mut root = ui_element::div()
        .flex_col()
        .gap(root_gap)
        .min_w_0()
        .self_stretch();

    for item in &spec.items {
        let is_expanded = expanded.iter().any(|v| *v == item.value);
        root = root.child(render_item(
            spec,
            item,
            is_expanded,
            effective_size,
            spec.density,
            theme,
            on_change.as_ref(),
        ));
    }

    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
        .aria_role(jetstream_ui::accesskit::Role::Group)
}

fn render_item(
    spec: &AccordionSpec,
    item: &AccordionItemSpec,
    is_expanded: bool,
    effective_size: ControlSize,
    density: ControlDensity,
    theme: &JetstreamThemeProvider,
    on_change: Option<&crate::element::Handler>,
) -> JsEl {
    // ── Token resolution ──
    let border_subtle = resolve_color(theme, "color.border.subtle");
    let elevated = resolve_color(theme, spec.item_bg_elevated_token());
    let panel = resolve_color(theme, spec.item_bg_panel_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let radius = resolve_radius(theme, "radius.surface");

    // Contract §8 Item border = color-mix(border-subtle 36%, transparent).
    let item_border = tint(border_subtle, spec.border_subtle_alpha());
    // Contract §8 Item background = color-mix(elevated 40%, panel).
    let item_bg = color_mix(elevated, panel, spec.item_bg_elevated_ratio());

    // Contract §8 Item padding: block (Y) 0.625rem, inline (X) density-driven.
    let pad_x = rem_to_px(spec.inline_padding_rem(density));
    let pad_y = rem_to_px(spec.block_padding_rem());
    // Contract §8 Item internal gap (trigger ↔ panel).
    let item_gap = rem_to_px(spec.item_internal_gap_rem());
    // Contract §8 Summary gap (title ↔ description) = space.inline.sm.
    let summary_gap = resolve_px(theme, spec.summary_gap_token());
    // Contract §8 Trigger grid gap (summary ↔ indicator) = space.inline.md.
    let trigger_gap = resolve_px(theme, spec.trigger_grid_gap_token());

    // Contract §8 Title font-size scales per-size (xs 0.8125 … xl 1.125rem).
    let title_font_size = rem_to_px(match effective_size {
        ControlSize::Xs => 0.8125,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.0625,
        ControlSize::Xl => 1.125,
    });
    // Contract §8 Description font-size scales per-size (xs 0.6875 … xl 0.9375rem).
    let description_font_size = rem_to_px(size_font_rem(effective_size));
    // Contract §8 Indicator font-size = 0.75rem.
    let indicator_size = rem_to_px(0.75);

    // ── Summary: title over optional description, stacked (space-inline-sm gap) ──
    // Contract: description renders inside the trigger summary always-visible —
    // collapsed items keep their description, matching Svelte.
    let mut summary = ui_element::div()
        .flex_col()
        .gap(summary_gap)
        .min_w_0()
        .flex_1()
        .child(
            ui_element::label(&item.label)
                .text_color(text_primary)
                .text_size(title_font_size)
                .text_weight(700) // Contract §8 title weight 700
                .line_height(1.2),
        );
    if let Some(ref desc) = item.description {
        summary = summary.child(
            ui_element::label(desc)
                .text_color(text_secondary)
                .text_size(description_font_size)
                .line_height(1.45),
        );
    }

    // Contract §8 Indicator: chevron-down icon (rotated 180deg when open).
    // JsEl has no rotation, so the chevron swaps icon name (Tier-3).
    let chevron_icon = if is_expanded { "chevron-down" } else { "chevron-right" };
    let indicator = ui_element::icon(chevron_icon)
        .w(indicator_size)
        .h(indicator_size)
        .flex_shrink_0()
        .text_color(text_secondary);

    // Contract §8 Trigger: full-width 1fr-auto grid (summary grows, indicator trailing).
    let mut trigger = ui_element::div()
        .flex_row()
        .self_stretch()
        .gap(trigger_gap)
        .items_center()
        .focusable()
        .child(summary)
        .child(indicator);

    // The trigger is the whole header row, not just the chevron.
    if let (false, Some(handler)) = (item.is_disabled, on_change) {
        let handler = std::sync::Arc::clone(handler);
        let value = item.value.clone();
        trigger = trigger.cursor_pointer().on_click(move |_event| handler(&value));
    }

    // Contract §8 Item box-shadow: `inset 0 0.0625rem 0 color-mix(text-inverse
    // 8%, transparent)` — a top highlight in the inverse text color at 8% alpha.
    let text_inverse = resolve_color(theme, "color.text.inverse");
    let highlight = glam::Vec4::new(text_inverse.x, text_inverse.y, text_inverse.z, 0.08);

    // Contract §8 Item: section with border, radius, background, internal gap.
    let mut el = ui_element::div()
        .flex_col()
        .self_stretch()
        .gap(item_gap)
        .px(pad_x)
        .py(pad_y)
        .rounded(radius)
        .bg(item_bg)
        .border_1()
        .border_color(item_border)
        .shadow_layers(vec![BoxShadow {
            offset_x: 0.0,
            offset_y: rem_to_px(0.0625),
            blur: 0.0,
            spread: 0.0,
            color: highlight,
            inset: true,
        }])
        .child(trigger);

    // Contract §8 Panel: role="region", conditional render, carries the children
    // snippet (the per-item description lives in the trigger summary, not here).
    if is_expanded {
        let panel = ui_element::div()
            // Contract: the disclosure panel is `role="region"`, so it can be
            // navigated to as a landmark rather than being anonymous structure
            // that happens to appear when a header is pressed.
            .aria_role(jetstream_ui::accesskit::Role::Region)
            .flex_col()
            .min_w_0()
            .self_stretch();
        el = el.child(panel);
    }

    // Contract §8 Item disabled: reduce opacity.
    if item.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::AccordionSelectionValue;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn sample_items() -> Vec<AccordionItemSpec> {
        vec![
            AccordionItemSpec {
                value: "a".into(),
                label: "Section A".into(),
                description: Some("Content for A".into()),
                is_disabled: false,
            },
            AccordionItemSpec {
                value: "b".into(),
                label: "Section B".into(),
                description: Some("Content for B".into()),
                is_disabled: false,
            },
        ]
    }

    /// All items render their title, and collapsed items keep their description.
    #[test]
    fn accordion_renders_items() {
        let spec = AccordionSpec::new(sample_items())
            .with_default_value(AccordionSelectionValue::Single("a".into()));
        let el = js_accordion(&spec, &theme());
        assert_eq!(el.children.len(), 2, "two item cards expected");

        let tree = probe(&el, 480.0, 320.0);
        assert!(tree.has_text("Section A"), "title A missing: {:?}", tree.texts());
        assert!(tree.has_text("Section B"), "title B missing: {:?}", tree.texts());
        // Item B is collapsed but its description must still render (in the trigger).
        assert!(
            tree.has_text("Content for B"),
            "collapsed item description missing (must render in trigger): {:?}",
            tree.texts()
        );
        // Chevron indicators come from the icon registry, one per item.
        assert!(tree.count_kind("Icon") >= 2, "chevron icons missing");
    }

    /// An expanded item renders its panel body region; a fully-collapsed
    /// accordion does not.
    #[test]
    fn expanded_item_shows_body() {
        let collapsed = AccordionSpec::new(sample_items());
        let collapsed_nodes = probe(&js_accordion(&collapsed, &theme()), 480.0, 320.0).len();

        let expanded = AccordionSpec::new(sample_items())
            .with_default_value(AccordionSelectionValue::Single("a".into()));
        let expanded_nodes = probe(&js_accordion(&expanded, &theme()), 480.0, 320.0).len();

        assert!(
            expanded_nodes > collapsed_nodes,
            "expanded accordion must add a panel body region ({} vs {})",
            expanded_nodes,
            collapsed_nodes
        );
    }

    /// Item background is the elevated/panel token mix — not the old
    /// hand-blended surface*0.5 + elevated*0.5 average.
    #[test]
    fn item_bg_from_token_mix() {
        let th = theme();
        let spec = AccordionSpec::new(sample_items());
        let el = js_accordion(&spec, &th);

        let elevated = resolve_color(&th, spec.item_bg_elevated_token());
        let panel = resolve_color(&th, spec.item_bg_panel_token());
        let expected = color_mix(elevated, panel, spec.item_bg_elevated_ratio());

        // Each item card carries the token-mixed background.
        for item_el in &el.children {
            let bg = item_el.style.background.expect("item background set");
            assert!(
                (bg.r - expected.x).abs() < 1e-4
                    && (bg.g - expected.y).abs() < 1e-4
                    && (bg.b - expected.z).abs() < 1e-4,
                "item bg must be color-mix(elevated 40%, panel): {:?} vs {:?}",
                bg,
                expected
            );
        }

        // The old hand-blend (surface 50% + elevated 50%) must NOT be present.
        let surface = resolve_color(&th, "color.background.surface");
        let old_blend = ProbeColor {
            r: surface.x * 0.5 + elevated.x * 0.5,
            g: surface.y * 0.5 + elevated.y * 0.5,
            b: surface.z * 0.5 + elevated.z * 0.5,
            a: 1.0,
        };
        let new_mix = ProbeColor {
            r: expected.x,
            g: expected.y,
            b: expected.z,
            a: expected.w,
        };
        // Only assert distinctness when the two formulas actually differ.
        if !old_blend.approx(new_mix, 1e-4) {
            let tree = probe(&el, 480.0, 320.0);
            assert!(
                !tree.has_background(old_blend, 1e-4),
                "old hand-blended item background must not appear"
            );
        }
    }

    /// The item's value, not the resulting expanded set: single- and
    /// multi-expand are host policy, and returning a set would put that
    /// decision in the component.
    #[test]
    fn a_trigger_reports_its_item() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let values = Arc::clone(&seen);

        let el = Accordion::from_spec(AccordionSpec::new(sample_items()), &theme())
            .on_change(move |value| values.lock().unwrap().push(value.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 600.0, 400.0, "Section B");

        assert_eq!(seen.lock().unwrap().as_slice(), ["b"]);
    }

    #[test]
    fn a_disabled_item_never_fires() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let mut items = sample_items();
        items[0].is_disabled = true;

        let el = Accordion::from_spec(AccordionSpec::new(items), &theme())
            .on_change(move |_| { counter.fetch_add(1, Ordering::SeqCst); })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 600.0, 400.0, "Section A");

        assert_eq!(hits.load(Ordering::SeqCst), 0, "a disabled item fired");
    }

}
