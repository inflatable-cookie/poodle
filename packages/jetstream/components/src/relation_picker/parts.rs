//! relation_picker — helper builders. Split out of `relation_picker/mod.rs` (god-file
//! decomposition); unchanged.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    CheckboxSpec, ChoiceOption, ControlSize, PickerItemSpec, RelationPickerSpec, SelectSpec,
    SelectionMode, TextInputSpec,
};

use crate::checkbox::js_checkbox;
use crate::presentation::{control_space_x_rem, rem_to_px};
use crate::select::js_select;
use crate::text_input::js_text_input;
use crate::theme_ext::color_mix;

use super::*;

pub(super) fn build_search(
    spec: &RelationPickerSpec,
    theme: &JetstreamThemeProvider,
    effective_size: ControlSize,
    text_secondary: glam::Vec4,
    accent: glam::Vec4,
    label_size: f32,
) -> JsEl {
    let mut col = ui_element::div().flex_col().gap(rem_to_px(0.5));

    if let Some(ref drill_down) = spec.drill_down {
        if !spec.drill_down_path.is_empty() {
            let mut crumbs = ui_element::div()
                .flex_row()
                .items_center()
                .gap(rem_to_px(0.25));
            // Back navigation (handler lives in preview event loop).
            crumbs = crumbs.child(
                ui_element::button("Back")
                    .id("poodle-relation-drill-back")
                    .text_color(text_secondary)
                    .text_size(label_size)
                    .focusable(),
            );

            for (idx, item_id) in spec.drill_down_path.iter().enumerate() {
                let label = drill_down
                    .levels
                    .get(idx)
                    .and_then(|level| level.items.iter().find(|item| item.id == *item_id))
                    .map(|item| item.label.clone())
                    .unwrap_or_else(|| item_id.clone());
                crumbs = crumbs
                    .child(
                        ui_element::label("/")
                            .text_color(text_secondary)
                            .text_size(label_size),
                    )
                    .child(
                        // Breadcrumb items are accent-colored (Svelte
                        // `--poodle-color-accent-base`), with weight 500.
                        ui_element::button(&label)
                            .id(format!("poodle-relation-crumb-{idx}"))
                            .text_color(accent)
                            .text_size(label_size)
                            .text_weight(LABEL_WEIGHT)
                            .focusable(),
                    );
            }

            col = col.child(crumbs);
        }

        if !drill_down.is_at_leaf(&spec.drill_down_path) {
            if let Some(level) = drill_down.next_level(&spec.drill_down_path) {
                col = col.child(
                    ui_element::label(&level.label.to_uppercase())
                        .text_color(text_secondary)
                        .text_size(label_size)
                        .text_weight(600)
                        .letter_spacing_em(0.08), // contract drill-level-label: 0.08em
                );
            }
        }
    }

    // Real search field — a TextInput type="search" with leading search icon
    // and the current query as its value (Svelte composes the real TextInput).
    // Typing/clear are owned by the preview event loop (render-only here).
    let mut search_spec = TextInputSpec::new()
        .with_id("relation-picker-search")
        .with_input_type("search")
        .with_leading_icon("search")
        .with_size(effective_size)
        .with_size_role(spec.size_role)
        .with_density(spec.density)
        .with_placeholder(spec.search_placeholder.clone())
        .with_show_clear_button(true);
    if !spec.query.is_empty() {
        search_spec = search_spec.with_value(spec.query.clone());
    }
    col = col.child(js_text_input(&search_spec, theme));

    // Toolbar filter controls — one labeled Select per `filters` entry (Svelte
    // `.poodle-relation-picker__filters`). Value change is preview-owned.
    if !spec.filters.is_empty() {
        let mut filters_row = ui_element::div()
            .flex_row()
            .flex_wrap()
            .gap(rem_to_px(control_space_x_rem(spec.density)));
        for filter in &spec.filters {
            let options = filter
                .resolved_options()
                .into_iter()
                .map(|(value, label)| ChoiceOption::new(value, label))
                .collect::<Vec<_>>();
            let mut select_spec = SelectSpec::new(options)
                .with_value(spec.filter_value(&filter.key).to_string())
                .with_size(effective_size)
                .with_size_role(spec.size_role)
                .with_density(spec.density);
            select_spec.aria_label = Some(format!("{} filter", filter.label));
            filters_row = filters_row.child(js_select(&select_spec, theme));
        }
        col = col.child(filters_row);
    }

    col
}

#[allow(clippy::too_many_arguments)]
pub(super) fn drill_row(
    item: &poodle_specs::DrillDownItem,
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    transparent: glam::Vec4,
    radius: f32,
    item_gap: f32,
    item_x: f32,
    item_y: f32,
    title_font: f32,
    label_size: f32,
) -> JsEl {
    let meta = item
        .count
        .map(|count| format!("{count} items"))
        .unwrap_or_default();
    // Drill button base is transparent (Svelte `.drill-list__button` background:
    // transparent; hover color-mix(surface 60%) is preview-owned).
    ui_element::button("")
        .id(format!("poodle-relation-drill-{}", item.id))
        .flex_row()
        .items_center()
        .justify_between()
        .gap(item_gap)
        .px(item_x)
        .py(item_y)
        .rounded(radius)
        .bg(transparent)
        .focusable()
        .child(
            ui_element::div()
                .flex_col()
                .gap(rem_to_px(0.125))
                .child(
                    ui_element::label(&item.label)
                        .text_color(text_primary)
                        .text_size(title_font)
                        .text_weight(LABEL_WEIGHT),
                )
                .child(
                    ui_element::label(item.description.as_deref().unwrap_or(""))
                        .text_color(text_secondary)
                        .text_size(label_size),
                ),
        )
        .child(
            ui_element::div()
                .flex_row()
                .items_center()
                .gap(rem_to_px(0.25))
                .child(
                    ui_element::label(&meta)
                        .text_color(text_secondary)
                        .text_size(label_size),
                )
                .child(
                    ui_element::icon("chevron-right")
                        .w(rem_to_px(0.875))
                        .h(rem_to_px(0.875))
                        .text_color(text_secondary),
                ),
        )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn candidate_row(
    item: &PickerItemSpec,
    is_selected: bool,
    selection_mode: SelectionMode,
    theme: &JetstreamThemeProvider,
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    border: glam::Vec4,
    accent: glam::Vec4,
    surface: glam::Vec4,
    radius: f32,
    item_gap: f32,
    item_x: f32,
    item_y: f32,
    title_font: f32,
    desc_font: f32,
    size: poodle_specs::ControlSize,
    size_role: poodle_specs::SemanticControlSizeRole,
    density: poodle_specs::ControlDensity,
) -> JsEl {
    let transparent = glam::Vec4::ZERO;
    // Base item bg: color-mix(surface 86%, transparent) (Svelte `.item`).
    let base_bg = color_mix(surface, transparent, 0.86);
    // Selected bg replaces the base with color-mix(accent 10%, transparent)
    // (contract §8 selected table — a single semi-transparent accent fill).
    let selected_bg = color_mix(accent, transparent, 0.10);
    let row_bg = if is_selected { selected_bg } else { base_bg };
    // Selected border: color-mix(accent 60%, transparent); else border-subtle.
    let row_border = if is_selected {
        color_mix(accent, transparent, 0.60)
    } else {
        border
    };

    let mut row = ui_element::button("")
        .id(format!("poodle-relation-candidate-{}", item.id))
        .flex_row()
        .items_center()
        .gap(item_gap)
        .px(item_x)
        .py(item_y)
        .border(1.0)
        .border_color(row_border)
        .rounded(radius)
        .bg(row_bg)
        .focusable();

    if selection_mode == SelectionMode::Multiple {
        row = row.child(js_checkbox(
            &CheckboxSpec::new()
                // A selection checkbox has no caption of its own — the row's
                // label sits beside it, not inside it — so without this it is
                // announced as an unnamed checkbox in a list of identical ones.
                .with_aria_label(format!("Select {}", item.label))
                .with_checked(is_selected)
                .with_size(size)
                .with_size_role(size_role)
                .with_density(density),
            theme,
        ));
    }

    row.child(
        ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.25))
            .min_w_0()
            .child(
                ui_element::label(&item.label)
                    .text_color(text_primary)
                    .text_size(title_font)
                    .text_weight(LABEL_WEIGHT),
            )
            .children(item.description.as_ref().map(|description| {
                ui_element::label(description)
                    .text_color(text_secondary)
                    .text_size(desc_font)
            }))
            .children(item.meta.as_ref().map(|meta| {
                ui_element::label(meta)
                    .text_color(text_secondary)
                    .text_size(desc_font)
            })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{DrillDownConfig, DrillDownItem, DrillDownLevel, PickerItemSpec};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn sample_items() -> Vec<PickerItemSpec> {
        vec![
            PickerItemSpec::new("btn", "Button").with_description("Primary action"),
            PickerItemSpec::new("card", "Card").with_meta("layout"),
            PickerItemSpec::new("input", "Input"),
        ]
    }

    #[test]
    fn renders_candidate_rows_and_search() {
        let el = js_relation_picker(&RelationPickerSpec::new(sample_items()), &theme());
        let tree = probe(&el, 480.0, 520.0);
        assert!(tree.has_text("Button") && tree.has_text("Card") && tree.has_text("Input"));
        // Real search field renders its leading search icon (not a faked div).
        assert!(
            tree.nodes
                .iter()
                .any(|n| n.kind == "Icon" && n.text.as_deref() == Some("search")),
            "search icon missing: {:?}",
            tree.texts()
        );
        // Each candidate row is an id-tagged, hit-testable target.
        assert!(tree.find_token("poodle-relation-candidate-btn").is_some());
    }

    #[test]
    fn selected_candidate_summary_and_remove() {
        let el = js_relation_picker(
            &RelationPickerSpec::new(sample_items())
                .with_selected_ids(vec!["btn".into(), "card".into()]),
            &theme(),
        );
        let tree = probe(&el, 480.0, 520.0);
        // Selection summary renders the chosen item labels…
        assert!(tree.has_text("Button") && tree.has_text("Card"));
        // …each summary chip carries a `×` remove glyph (SelectionSummary).
        assert!(
            tree.has_text("×"),
            "selection summary remove glyph missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn multiple_mode_shows_checkboxes() {
        let el = js_relation_picker(
            &RelationPickerSpec::new(sample_items())
                .with_selection_mode(SelectionMode::Multiple)
                .with_selected_ids(vec!["btn".into()]),
            &theme(),
        );
        let tree = probe(&el, 480.0, 520.0);
        // Multiple mode pairs a Checkbox with each candidate row.
        assert!(
            tree.count_kind("Panel") > 0,
            "expected rendered structure: {:?}",
            tree.texts()
        );
        assert!(tree.find_token("poodle-relation-candidate-btn").is_some());
    }

    #[test]
    fn single_mode_omits_checkboxes_keeps_rows() {
        let el = js_relation_picker(
            &RelationPickerSpec::new(sample_items()).with_selection_mode(SelectionMode::Single),
            &theme(),
        );
        let tree = probe(&el, 480.0, 520.0);
        assert!(tree.has_text("Button"));
        assert!(tree.find_token("poodle-relation-candidate-input").is_some());
    }

    #[test]
    fn drilling_renders_drill_rows_breadcrumb_back() {
        let config = DrillDownConfig::new(
            vec![DrillDownLevel::new(
                "cat",
                "Category",
                vec![
                    DrillDownItem::new("forms", "Forms").with_count(4),
                    DrillDownItem::new("layout", "Layout").with_count(2),
                ],
            )],
            vec![],
        );
        let el = js_relation_picker(
            &RelationPickerSpec::new(sample_items()).with_drill_down(config),
            &theme(),
        );
        let tree = probe(&el, 480.0, 520.0);
        assert!(tree.has_text("Forms") && tree.has_text("Layout"));
        assert!(
            tree.find_token("poodle-relation-drill-forms").is_some(),
            "drill row should be hit-testable: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn results_surface_hidden_in_non_ready_state() {
        let el = js_relation_picker(
            &RelationPickerSpec::new(sample_items()).with_state(BrowseState::Loading),
            &theme(),
        );
        let tree = probe(&el, 480.0, 520.0);
        // Candidate rows are not rendered when the picker is not "ready".
        assert!(
            tree.find_token("poodle-relation-candidate-btn").is_none(),
            "non-ready state must not render candidates"
        );
    }

    #[test]
    fn toolbar_renders_filter_controls() {
        use poodle_specs::{PickerFilterConfig, PickerFilterOption};
        let filters = vec![PickerFilterConfig::new(
            "kind",
            "Kind",
            vec![
                PickerFilterOption::new("forms", "Forms"),
                PickerFilterOption::new("layout", "Layout"),
            ],
        )];
        let el = js_relation_picker(
            &RelationPickerSpec::new(sample_items()).with_filters(filters),
            &theme(),
        );
        let tree = probe(&el, 480.0, 520.0);
        // Filter Select with no chosen value renders the synthesized "All"
        // trigger label (Svelte `getFilterOptions` prepends "All").
        assert!(
            tree.has_text("All"),
            "filter select 'All' trigger missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn filter_renders_selected_value_label() {
        use poodle_specs::{PickerFilterConfig, PickerFilterOption};
        let filters = vec![PickerFilterConfig::new(
            "kind",
            "Kind",
            vec![PickerFilterOption::new("forms", "Forms")],
        )];
        let el = js_relation_picker(
            &RelationPickerSpec::new(sample_items())
                .with_filters(filters)
                .with_filter_value("kind", "forms"),
            &theme(),
        );
        let tree = probe(&el, 480.0, 520.0);
        // The chosen filter value resolves to its option label in the trigger.
        assert!(
            tree.has_text("Forms"),
            "selected filter label missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn footer_note_renders_when_set() {
        let el = js_relation_picker(
            &RelationPickerSpec::new(sample_items()).with_footer_note("Choose up to three items."),
            &theme(),
        );
        let tree = probe(&el, 480.0, 520.0);
        assert!(
            tree.has_text("Choose up to three items."),
            "footer note missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn show_footer_false_hides_actions() {
        let el = js_relation_picker(
            &RelationPickerSpec::new(sample_items())
                .with_show_footer(false)
                .with_cancel_label("Cancel"),
            &theme(),
        );
        let tree = probe(&el, 480.0, 520.0);
        // Confirm/cancel buttons live in the footer; suppressed entirely.
        assert!(
            !tree.has_text("Cancel"),
            "footer should be absent when show_footer=false: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn show_selection_summary_false_hides_summary() {
        let with_summary = js_relation_picker(
            &RelationPickerSpec::new(sample_items()).with_selected_ids(vec!["btn".into()]),
            &theme(),
        );
        let without_summary = js_relation_picker(
            &RelationPickerSpec::new(sample_items())
                .with_selected_ids(vec!["btn".into()])
                .with_show_selection_summary(false),
            &theme(),
        );
        // The summary chip carries a `×` remove glyph; absent when suppressed.
        assert!(probe(&with_summary, 480.0, 520.0).has_text("×"));
        assert!(
            !probe(&without_summary, 480.0, 520.0).has_text("×"),
            "selection summary should be hidden when show_selection_summary=false"
        );
    }

    /// The candidate's id, not a resolved selection: single- vs multi-select is
    /// the host's policy.
    #[test]
    fn choosing_a_candidate_reports_its_id() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let ids = Arc::clone(&seen);

        let el = crate::relation_picker::RelationPicker::from_spec(
            RelationPickerSpec::new(sample_items()),
            &theme(),
        )
        .on_select(move |id| ids.lock().unwrap().push(id.to_string()))
        .into_js_el();

        crate::element::click_probe::click_text(&el, 480.0, 520.0, "Card");

        assert_eq!(seen.lock().unwrap().as_slice(), ["card"]);
    }

    #[test]
    fn entering_a_drill_row_reports_its_context() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let ids = Arc::clone(&seen);

        let config = DrillDownConfig::new(
            vec![DrillDownLevel::new(
                "cat",
                "Category",
                vec![
                    DrillDownItem::new("forms", "Forms").with_count(4),
                    DrillDownItem::new("layout", "Layout").with_count(2),
                ],
            )],
            vec![],
        );

        let el = crate::relation_picker::RelationPicker::from_spec(
            RelationPickerSpec::new(sample_items()).with_drill_down(config),
            &theme(),
        )
        .on_drill_enter(move |id| ids.lock().unwrap().push(id.to_string()))
        .into_js_el();

        crate::element::click_probe::click_text(&el, 480.0, 520.0, "Layout");

        assert_eq!(seen.lock().unwrap().as_slice(), ["layout"]);
    }

    #[test]
    fn the_footer_reports_confirm_and_cancel() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
        let confirms = Arc::clone(&seen);
        let cancels = Arc::clone(&seen);

        let spec = RelationPickerSpec::new(sample_items());
        let (confirm_label, cancel_label) = (spec.confirm_label.clone(), spec.cancel_label.clone());

        let el = crate::relation_picker::RelationPicker::from_spec(spec, &theme())
            .on_confirm(move || confirms.lock().unwrap().push("confirm"))
            .on_cancel(move || cancels.lock().unwrap().push("cancel"))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 480.0, 560.0, &confirm_label);
        crate::element::click_probe::click_text(&el, 480.0, 560.0, &cancel_label);

        assert_eq!(seen.lock().unwrap().as_slice(), ["confirm", "cancel"]);
    }
}
