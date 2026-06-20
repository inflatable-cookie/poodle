//! JsCardToggleGroup — multi-select card group backed by CardToggleGroupSpec.
//!
//! Contract: `docs/contracts/components/card-toggle-group.md`
//! Reference: GPUI `composites/card_toggle_group.rs`.
//!
//! Render-only: toggling lives in the preview event loop. Each option renders an
//! interactive Card (selected when its value is in `values`) with a title +
//! optional description. ZERO hardcoded values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CardSpec, CardToggleGroupSpec, TextSpec, TextTone};

use crate::card::js_card;
use crate::presentation::rem_to_px;
use crate::text::js_text;

/// Build a card-toggle-group from its spec.
pub fn js_card_toggle_group(spec: &CardToggleGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let mut root = ui_element::div().flex_col().gap(rem_to_px(0.5));

    for option in &spec.options {
        let mut card_spec = CardSpec::new().interactive();
        if spec.is_selected(&option.value) {
            card_spec = card_spec.selected();
        }

        let mut body = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.25))
            .child(js_text(&TextSpec::new(option.title.clone()), theme));
        if let Some(description) = &option.description {
            body = body.child(js_text(
                &TextSpec::new(description.clone()).with_tone(TextTone::Secondary),
                theme,
            ));
        }

        root = root.child(js_card(&card_spec, theme, vec![body]));
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::CardToggleOption;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn options() -> Vec<CardToggleOption> {
        vec![
            CardToggleOption::new("a", "Option A").with_description("First"),
            CardToggleOption::new("b", "Option B"),
        ]
    }

    #[test]
    fn renders_each_option_title_and_description() {
        let el = js_card_toggle_group(&CardToggleGroupSpec::new(options()), &theme());
        let tree = probe(&el, 320.0, 240.0);
        assert!(tree.has_text("Option A") && tree.has_text("Option B"), "titles missing");
        assert!(tree.has_text("First"), "description missing");
    }

    #[test]
    fn renders_all_options_as_cards() {
        // Two options → at least two card subtrees with their titles present.
        let el = js_card_toggle_group(
            &CardToggleGroupSpec::new(options()).with_values(vec!["a".into()]),
            &theme(),
        );
        let tree = probe(&el, 320.0, 240.0);
        assert!(!tree.is_empty());
        assert!(tree.has_text("Option A") && tree.has_text("Option B"));
    }
}
