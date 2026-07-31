//! JsCardToggleGroup — multi-select card group backed by CardToggleGroupSpec.
//!
//! Contract: `docs/contracts/components/card-toggle-group.md`
//! Reference: GPUI `composites/card_toggle_group.rs`.
//!
//! Render-only: toggling lives in the preview event loop. Each option composes
//! the `Card` primitive (interactive, and selected when its value is in
//! `values`) so the selected fill/border come from Card's own token-resolved
//! treatment — never a hand-rolled accent tint. The Card body carries the title
//! (weight 600, text-primary) and an optional description (text-secondary). The
//! options reflow in a flex-wrap grid with a density-driven gap; per-item and
//! group `disabled` dim via the disabled-opacity token. ZERO hardcoded values.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CardSpec, CardToggleGroupSpec};

use crate::card::js_card;
use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity};

/// Build a card-toggle-group from its spec.
/// CardToggleGroup — several choices, rendered as cards.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_change(handler)`.
///
/// Multi-select, so the event carries the option that was pressed rather than
/// the resulting set — the host owns the set and decides whether a press adds
/// or removes.
pub struct CardToggleGroup {
    spec: CardToggleGroupSpec,
    theme: JetstreamThemeProvider,
    on_change: Option<crate::element::Handler>,
}

impl CardToggleGroup {
    pub fn from_spec(spec: CardToggleGroupSpec, theme: &JetstreamThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_change: None }
    }

    /// Fires with the chosen option's value. Disabled options never fire.
    pub fn on_change(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_change = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for CardToggleGroup {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_change)
    }
}

pub fn js_card_toggle_group(spec: &CardToggleGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None)
}

fn build(
    spec: &CardToggleGroupSpec,
    theme: &JetstreamThemeProvider,
    on_change: Option<crate::element::Handler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Contract §7 size scale — resolved through the spec helpers, not literals.
    let title_font = rem_to_px(CardToggleGroupSpec::title_font_rem(effective_size));
    let description_font = rem_to_px(CardToggleGroupSpec::description_font_rem(effective_size));

    // Density-driven grid gap (contract §7 density table) + Card body rhythm.
    let grid_gap = rem_to_px(control_space_x_rem(spec.density));
    let body_gap = rem_to_px(0.25);

    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");

    // Contract §6: options lay out in a grid capped at `columns` (1–4). Collect each cell,
    // then assemble rows of `column_count()`.
    let cols = spec.column_count();
    let mut cells: Vec<JsEl> = Vec::new();

    for option in &spec.options {
        let is_selected = spec.is_selected(&option.value);
        let is_option_disabled = spec.disabled || option.disabled;

        // Card body: title + optional description, both token-resolved.
        let mut body = ui_element::div().flex_col().gap(body_gap).child(
            ui_element::label(option.title.clone())
                .text_size(title_font)
                .text_weight(600)
                .text_color(text_primary),
        );
        if let Some(description) = &option.description {
            body = body.child(
                ui_element::label(description.clone())
                    .text_size(description_font)
                    .text_color(text_secondary),
            );
        }

        // Compose the Card primitive — selected state owns the fill/border.
        let mut card_spec = CardSpec::new().interactive();
        if is_selected {
            card_spec = card_spec.selected();
        }
        let card = js_card(&card_spec, theme, vec![body]);

        // Wrap each Card so it can grow within the wrap-grid; dim per-item disabled.
        let mut option_el = ui_element::div().flex_1().min_w_0().child(card);
        if is_option_disabled {
            option_el = option_el.opacity(disabled_opacity);
        } else if let Some(handler) = &on_change {
            let handler = std::sync::Arc::clone(handler);
            let value = option.value.clone();
            option_el = option_el.cursor_pointer().on_click(move |_event| handler(&value));
        }

        cells.push(option_el);
    }

    // Assemble rows of `cols` cells; pad a short final row with flex spacers so card
    // widths stay aligned across rows (matches the auto-fit grid's column track).
    let mut root = ui_element::div().flex_col().gap(grid_gap);
    let mut iter = cells.into_iter();
    let mut remaining = spec.options.len();
    while remaining > 0 {
        let take = cols.min(remaining);
        let mut row = ui_element::div().flex_row().gap(grid_gap);
        for _ in 0..take {
            if let Some(cell) = iter.next() {
                row = row.child(cell);
            }
        }
        for _ in take..cols {
            row = row.child(ui_element::div().flex_1());
        }
        root = root.child(row);
        remaining -= take;
    }

    if spec.disabled {
        root = root.opacity(disabled_opacity);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::CardToggleOption;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn options() -> Vec<CardToggleOption> {
        vec![
            CardToggleOption::new("a", "Option A").with_description("First"),
            CardToggleOption::new("b", "Option B"),
            CardToggleOption::new("c", "Option C"),
        ]
    }

    #[test]
    fn renders_each_option_title_and_description() {
        let el = js_card_toggle_group(&CardToggleGroupSpec::new(options()), &theme());
        let tree = probe(&el, 480.0, 240.0);
        assert!(
            tree.has_text("Option A") && tree.has_text("Option B") && tree.has_text("Option C"),
            "titles missing: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("First"), "description missing");
    }

    #[test]
    fn columns_chunk_renders_all_options_in_rows() {
        // Contract §3 columns: 4 options at columns=2 → two rows of 2; chunking must
        // not drop any cell, and column_count clamps to 1..4.
        let four = vec![
            CardToggleOption::new("a", "Alpha"),
            CardToggleOption::new("b", "Bravo"),
            CardToggleOption::new("c", "Charlie"),
            CardToggleOption::new("d", "Delta"),
        ];
        let spec = CardToggleGroupSpec::new(four).with_columns(2);
        assert_eq!(spec.column_count(), 2);
        let tree = probe(&js_card_toggle_group(&spec, &theme()), 480.0, 320.0);
        for t in ["Alpha", "Bravo", "Charlie", "Delta"] {
            assert!(tree.has_text(t), "{t} missing with columns=2: {:?}", tree.texts());
        }
        // Clamp: 0 → 1, 9 → 4.
        assert_eq!(CardToggleGroupSpec::new(vec![]).with_columns(0).column_count(), 1);
        assert_eq!(CardToggleGroupSpec::new(vec![]).with_columns(9).column_count(), 4);
    }

    #[test]
    fn multi_select_marks_each_selected_value() {
        // Multi-select: both "a" and "c" selected at once. The spec reports both,
        // and all option titles still render as cards.
        let spec = CardToggleGroupSpec::new(options())
            .with_values(vec!["a".into(), "c".into()]);
        assert!(spec.is_selected("a"));
        assert!(spec.is_selected("c"));
        assert!(!spec.is_selected("b"));

        let el = js_card_toggle_group(&spec, &theme());
        let tree = probe(&el, 480.0, 240.0);
        assert!(!tree.is_empty());
        assert!(
            tree.has_text("Option A")
                && tree.has_text("Option B")
                && tree.has_text("Option C"),
            "selected treatment dropped an option: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn renders_all_options_when_group_disabled() {
        let spec = CardToggleGroupSpec::new(options()).with_disabled(true);
        let el = js_card_toggle_group(&spec, &theme());
        let tree = probe(&el, 480.0, 240.0);
        assert!(
            tree.has_text("Option A") && tree.has_text("Option B"),
            "disabled group dropped options: {:?}",
            tree.texts()
        );
    }

    /// Multi-select, so the event names the option pressed rather than the
    /// resulting set: the host owns the set and decides add or remove.
    #[test]
    fn pressing_a_card_reports_that_option() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let values = Arc::clone(&seen);

        let el = CardToggleGroup::from_spec(CardToggleGroupSpec::new(options()), &theme())
            .on_change(move |value| values.lock().unwrap().push(value.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 640.0, 240.0, "Option B");

        assert_eq!(seen.lock().unwrap().as_slice(), ["b"]);
    }

}
