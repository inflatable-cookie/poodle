//! SelectionSummary — Jetstream selection summary backed by SelectionSummarySpec.
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SelectionSummarySpec;

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// SelectionSummary — the chips a selection leaves behind.
///
/// Mirrors the GPUI target's names: `on_remove` and `on_clear`.
pub struct SelectionSummary {
    spec: SelectionSummarySpec,
    theme: JetstreamThemeProvider,
    on_remove: Option<crate::element::Handler>,
    on_clear: Option<crate::element::ActionHandler>,
}

impl SelectionSummary {
    pub fn from_spec(spec: SelectionSummarySpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_remove: None,
            on_clear: None,
        }
    }

    /// Fires with the removed item's id.
    pub fn on_remove(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_remove = Some(std::sync::Arc::new(handler));
        self
    }

    /// Fires when the clear control is pressed.
    pub fn on_clear(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_clear = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for SelectionSummary {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_remove, self.on_clear)
    }
}

pub fn js_selection_summary(spec: &SelectionSummarySpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None, None)
}

fn build(
    spec: &SelectionSummarySpec,
    theme: &JetstreamThemeProvider,
    on_remove: Option<crate::element::Handler>,
    on_clear: Option<crate::element::ActionHandler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let chip_font = rem_to_px(SelectionSummarySpec::chip_font_rem(effective_size));
    // Overflow badge carries its own font-size per size (Svelte
    // `--poodle-selection-summary-overflow-font-size`), distinct from chips.
    let overflow_font = rem_to_px(SelectionSummarySpec::overflow_font_rem(effective_size));
    let gap = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.375,
        poodle_specs::ControlDensity::Default => control_space_x_rem(spec.density),
        poodle_specs::ControlDensity::Comfortable => 0.75,
    });
    // Chip / overflow radius + border width resolve from tokens
    // (`radius.control`, `border.width.default`) — no hardcoded literals.
    let chip_radius = resolve_radius(theme, spec.radius_token());
    let chip_border_width = resolve_px(theme, spec.border_width_token());

    let text_color = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_tertiary = resolve_color(theme, "color.text.tertiary");
    let surface = resolve_color(theme, "color.background.surface");
    let elevated = resolve_color(theme, "color.background.elevated");
    let chip_bg = elevated.lerp(surface, 0.40);
    let overflow_bg = elevated.lerp(surface, 0.32);
    let chip_border = resolve_color(theme, "color.border.subtle");
    let accent = resolve_color(theme, "color.accent.base");
    let bottom_pad = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.5,
        poodle_specs::ControlDensity::Default => 0.625,
        poodle_specs::ControlDensity::Comfortable => 0.75,
    });
    let chip_px = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.625,
        poodle_specs::ControlDensity::Default => 0.75,
        poodle_specs::ControlDensity::Comfortable => 0.875,
    });
    let overflow_px = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.5,
        poodle_specs::ControlDensity::Default => 0.625,
        poodle_specs::ControlDensity::Comfortable => 0.75,
    });
    let chip_min_h = rem_to_px(SelectionSummarySpec::chip_min_height_rem(effective_size));

    let mut el = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap)
        .flex_wrap()
        .self_stretch();
    el = el.pb(bottom_pad).min_h(chip_min_h);

    if spec.items.is_empty() {
        return el.child(
            ui_element::label("No selection")
                .text_color(text_tertiary)
                .text_size(chip_font),
        );
    }

    for item in spec.items.iter().take(spec.visible_item_count()) {
        let mut chip = ui_element::button("")
            .flex_row()
            .items_center()
            .gap(gap)
            .text_color(text_color)
            .pl(chip_px)
            .pr(chip_px)
            .min_h(chip_min_h)
            .rounded(chip_radius)
            .bg(chip_bg)
            .border(chip_border_width)
            .border_color(chip_border)
            .child(
                ui_element::label(&item.label)
                    .text_color(text_color)
                    .text_size(chip_font),
            );

        // Anatomy is ChipLabel + RemoveIcon only (contract §2); item `meta` is
        // not part of the Svelte/contract surface — not rendered.

        chip = chip.child(
            ui_element::label("×")
                .text_color(text_secondary)
                .text_size(chip_font),
        );

        if let Some(handler) = &on_remove {
            let handler = std::sync::Arc::clone(handler);
            let id = item.id.clone();
            chip = chip.cursor_pointer().on_click(move |_event| handler(&id));
        }

        el = el.child(chip);
    }

    if spec.overflow_count() > 0 {
        el = el.child(
            ui_element::label(&format!("+{} more", spec.overflow_count()))
                .text_color(text_secondary)
                .text_size(overflow_font)
                .pl(overflow_px)
                .pr(overflow_px)
                .min_h(chip_min_h)
                .rounded(chip_radius)
                .bg(overflow_bg)
                .border(chip_border_width)
                .border_color(chip_border),
        );
    }

    // Clear link — Svelte renders the inline "Clear" link whenever the
    // selection is populated (contract §4), not only when a clear action is
    // configured. Label defaults to "Clear", overridable via clear_action.
    // Interaction is owned by the preview event loop (display-only here).
    let clear_label = spec
        .clear_action
        .as_ref()
        .map(|c| c.label.clone())
        .unwrap_or_else(|| "Clear".to_string());
    let mut clear = ui_element::button(&clear_label)
        .text_color(accent)
        .text_size(font_size)
        .focusable();

    if let Some(handler) = on_clear {
        clear = clear.cursor_pointer().on_click(move |_event| handler());
    }

    el = el.child(
        ui_element::div()
            .flex_grow()
            .flex_row()
            .justify_end()
            .child(clear),
    );

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{ControlSize, SelectionSummaryItem, SelectionSummarySpec};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn items(n: usize) -> Vec<SelectionSummaryItem> {
        (0..n)
            .map(|i| SelectionSummaryItem::new(format!("id-{i}"), format!("Item {i}")))
            .collect()
    }

    #[test]
    fn empty_state_shows_placeholder_no_clear() {
        let spec = SelectionSummarySpec::new(Vec::new());
        let tree = probe(&js_selection_summary(&spec, &theme()), 360.0, 80.0);
        assert!(tree.has_text("No selection"), "empty → italic placeholder");
        assert!(
            !tree.has_text("Clear"),
            "empty → no clear control, no chips"
        );
        assert!(!tree.has_text("Item 0"));
    }

    #[test]
    fn populated_renders_chips_and_clear_control() {
        let spec = SelectionSummarySpec::new(items(3));
        let tree = probe(&js_selection_summary(&spec, &theme()), 360.0, 80.0);
        assert!(tree.has_text("Item 0"));
        assert!(tree.has_text("Item 1"));
        assert!(tree.has_text("Item 2"));
        // Clear renders whenever populated even with no configured clear_action
        // (Svelte renders the inline "Clear" link unconditionally).
        assert!(
            tree.has_text("Clear"),
            "populated → unconditional Clear control"
        );
        // Each chip carries a remove "×" glyph (RemoveIcon anatomy part).
        let removes = tree
            .nodes
            .iter()
            .filter(|n| n.text.as_deref() == Some("×"))
            .count();
        assert_eq!(removes, 3, "one remove glyph per visible chip");
    }

    #[test]
    fn custom_clear_label_is_used() {
        use poodle_specs::RemediationAction;
        let spec = SelectionSummarySpec::new(items(2))
            .with_clear_action(RemediationAction::new("clear", "Clear all"));
        let tree = probe(&js_selection_summary(&spec, &theme()), 360.0, 80.0);
        assert!(
            tree.has_text("Clear all"),
            "clear_action label overrides default"
        );
    }

    #[test]
    fn truncated_shows_overflow_count_and_clear() {
        // 6 items, max 3 visible → "+3 more" overflow badge.
        let spec = SelectionSummarySpec::new(items(6)).with_max_visible_items(3);
        let tree = probe(&js_selection_summary(&spec, &theme()), 480.0, 80.0);
        assert!(tree.has_text("Item 0") && tree.has_text("Item 2"));
        assert!(!tree.has_text("Item 3"), "items past the cap are not chips");
        assert!(
            tree.has_text("+3 more"),
            "overflow badge shows hidden count"
        );
        assert!(tree.has_text("Clear"));
        // Overflow badge font-size resolves from its own per-size token, not the
        // chip font (md: overflow 0.8125rem vs chip 0.75rem).
        let overflow_node = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("+3 more"))
            .expect("overflow node present");
        let expected = rem_to_px(SelectionSummarySpec::overflow_font_rem(ControlSize::Md));
        assert_eq!(overflow_node.text_size, Some(expected));
    }

    #[test]
    fn chip_background_resolves_from_token() {
        let spec = SelectionSummarySpec::new(items(1));
        let tree = probe(&js_selection_summary(&spec, &theme()), 360.0, 80.0);
        // chip_bg = elevated.lerp(surface, 0.40), matching the component.
        let surface = resolve_color(&theme(), "color.background.surface");
        let elevated = resolve_color(&theme(), "color.background.elevated");
        let chip_bg = elevated.lerp(surface, 0.40);
        let expected = ProbeColor {
            r: chip_bg.x,
            g: chip_bg.y,
            b: chip_bg.z,
            a: chip_bg.w,
        };
        assert!(
            tree.has_background(expected, 0.01),
            "chip fill resolved from surface/elevated tokens, not a literal"
        );
    }

    #[test]
    fn removing_a_chip_reports_its_id() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let ids = Arc::clone(&seen);

        let el = SelectionSummary::from_spec(SelectionSummarySpec::new(items(3)), &theme())
            .on_remove(move |id| ids.lock().unwrap().push(id.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 640.0, 160.0, "Item 1");

        assert_eq!(seen.lock().unwrap().as_slice(), ["id-1"]);
    }

    #[test]
    fn the_clear_control_reports_a_clear() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = SelectionSummary::from_spec(SelectionSummarySpec::new(items(3)), &theme())
            .on_clear(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 640.0, 160.0, "Clear");

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_clear fired exactly once"
        );
    }
}
