use gpui::*;
use flint_adapter::ThemeProvider;
use flint_primitives::{AccordionSpec, AccordionItemSpec, AccordionSelectionValue, EyebrowSpec};
use flint_gpui_components::{Accordion, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let ts = crate::style_bridge::color_to_hsla(text_secondary);

    // --- Single selection items ---
    let single_items = vec![
        AccordionItemSpec::new("getting-started", "Getting started"),
        AccordionItemSpec::new("api-reference", "API reference"),
        AccordionItemSpec::new("accessibility", "Accessibility"),
    ];

    let single_content: Vec<(&str, &str)> = vec![
        ("getting-started", "Install the package with your preferred package manager, then import individual components as needed. Each component is tree-shakeable and ships with its own styles scoped via CSS custom properties."),
        ("api-reference", "Every component accepts an ariaLabel prop for accessible naming. Most interactive components support controlled and uncontrolled modes via value/defaultValue pairs, and emit granular events like valueChange, openChange, or requestClose."),
        ("accessibility", "All components follow WAI-ARIA authoring practices. Focus is trapped inside modal overlays, arrow keys navigate composite widgets, and Escape dismisses dismissible layers. Screen reader announcements use live regions where appropriate."),
    ];

    // Track single-selection expanded state (default: "getting-started" open)
    let single_key_prefix = "accordion-single-";
    let single_initialized = state.specimens.is_on(&format!("{single_key_prefix}__init"));
    let single_expanded: Option<String> = if !single_initialized {
        Some("getting-started".to_string())
    } else {
        single_items.iter()
            .find(|item| state.specimens.is_on(&format!("{single_key_prefix}{}", item.value)))
            .map(|item| item.value.clone())
    };

    let mut single_spec = AccordionSpec::new(single_items)
        .with_allow_multiple(false)
        .with_collapsible(true);

    if let Some(ref val) = single_expanded {
        single_spec = single_spec.with_value(AccordionSelectionValue::Single(val.clone()));
    }

    let mut single_accordion = Accordion::from_spec(single_spec, theme)
        .with_id("specimen-accordion-single")
        .on_toggle(cx.listener(|this, value: &str, _w, cx| {
            // Mark as initialized so defaults stop applying
            let init_key = "accordion-single-__init".to_string();
            if !this.state.specimens.is_on(&init_key) {
                this.state.specimens.toggle(&init_key);
            }
            let key = format!("accordion-single-{}", value);
            let is_on = this.state.specimens.is_on(&key);
            for item_val in &["getting-started", "api-reference", "accessibility"] {
                let k = format!("accordion-single-{}", item_val);
                if this.state.specimens.is_on(&k) {
                    this.state.specimens.toggle(&k);
                }
            }
            if !is_on {
                this.state.specimens.toggle(&key);
            }
            cx.notify();
        }));

    for (value, text) in &single_content {
        single_accordion = single_accordion.with_content(
            *value,
            div()
                .text_size(px(14.0))
                .line_height(relative(1.5))
                .text_color(ts)
                .child(text.to_string()),
        );
    }

    // --- Multiple selection items ---
    let multi_items = vec![
        AccordionItemSpec::new("design", "Design tokens"),
        AccordionItemSpec::new("keyboard", "Keyboard shortcuts"),
        AccordionItemSpec::new("known-issues", "Known issues"),
    ];

    let multi_content: Vec<(&str, &str)> = vec![
        ("design", "Components consume semantic tokens like --flint-color-text-primary and --flint-size-control-height rather than hard-coded values. Switching themes at runtime updates every component instantly without re-rendering."),
        ("keyboard", "Arrow keys move focus between accordion headers. Enter or Space toggles the focused panel. Home and End jump to the first and last header respectively. Tab moves focus out of the accordion entirely."),
        ("known-issues", "Animation on panel expand/collapse is not yet implemented. The component does not support nested accordions. Horizontal orientation is planned but not available in this release."),
    ];

    // Track multiple-selection expanded state (default: "design" + "keyboard" open)
    let multi_key_prefix = "accordion-multi-";
    let multi_initialized = state.specimens.is_on(&format!("{multi_key_prefix}__init"));
    let multi_expanded: Vec<String> = if !multi_initialized {
        vec!["design".to_string(), "keyboard".to_string()]
    } else {
        multi_items.iter()
            .filter(|item| state.specimens.is_on(&format!("{multi_key_prefix}{}", item.value)))
            .map(|item| item.value.clone())
            .collect()
    };

    let mut multi_spec = AccordionSpec::new(multi_items)
        .with_allow_multiple(true)
        .with_collapsible(true);

    if !multi_expanded.is_empty() {
        multi_spec = multi_spec.with_value(AccordionSelectionValue::Multiple(multi_expanded));
    }

    let mut multi_accordion = Accordion::from_spec(multi_spec, theme)
        .with_id("specimen-accordion-multi")
        .on_toggle(cx.listener(|this, value: &str, _w, cx| {
            let init_key = "accordion-multi-__init".to_string();
            if !this.state.specimens.is_on(&init_key) {
                // First interaction: initialize from defaults
                this.state.specimens.toggle(&init_key);
                for default_val in &["design", "keyboard"] {
                    let k = format!("accordion-multi-{}", default_val);
                    this.state.specimens.toggle(&k); // set defaults on
                }
            }
            let key = format!("accordion-multi-{}", value);
            this.state.specimens.toggle(&key);
            cx.notify();
        }));

    for (value, text) in &multi_content {
        multi_accordion = multi_accordion.with_content(
            *value,
            div()
                .text_size(px(14.0))
                .line_height(relative(1.5))
                .text_color(ts)
                .child(text.to_string()),
        );
    }

    div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Single selection"), theme))
                .child(single_accordion)
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Multiple selection"), theme))
                .child(multi_accordion)
        )
}
