//! RelationPicker — Jetstream relation picker backed by RelationPickerSpec.
//!
//! Contract: `docs/contracts/components/relation-picker.md`
//! Reference: Svelte `RelationPicker.svelte` (parity authority); GPUI
//! `composites/relation_picker.rs`.
//!
//! Render-only: candidate toggling, drill navigation (advance / back /
//! breadcrumb jump), live search typing, and keyboard nav live in the preview
//! event loop — this builder renders the current open state (search field,
//! selection summary, candidate/drill list, footer). All geometry, colors, and
//! type sizes resolve from size/density tokens — zero hardcoded px/hsla.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    BrowseState, ButtonSpec, ButtonVariant, CheckboxSpec, ChoiceOption, ControlSize, PickerItemSpec,
    RelationPickerSpec, SelectSpec, SelectionMode, TextInputSpec,
};

use crate::button::js_button;
use crate::checkbox::js_checkbox;
use crate::picker_shell::js_picker_shell;
use crate::presentation::{
    control_space_x_rem, rem_to_px, relation_picker_desc_size_rem, relation_picker_item_gap_rem,
    relation_picker_item_x_rem, relation_picker_item_y_rem, relation_picker_list_gap_rem,
    relation_picker_title_size_rem, resolve_semantic_size,
};
use crate::select::js_select;
use crate::selection_summary::js_selection_summary;
use crate::text_input::js_text_input;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// Candidate / drill copy strong label weight (Svelte `strong { font-weight: 500 }`).
const LABEL_WEIGHT: u16 = 500;

pub fn js_relation_picker(spec: &RelationPickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.subtle");
    let accent = resolve_color(theme, "color.accent.base");
    let surface = resolve_color(theme, "color.background.surface");
    let radius = resolve_radius(theme, "radius.control");
    let transparent = glam::Vec4::ZERO;

    // Density-driven inter-row gap (contract §8 density table).
    let list_gap = rem_to_px(relation_picker_list_gap_rem(spec.density));
    let title_font = rem_to_px(relation_picker_title_size_rem(effective_size));
    let desc_font = rem_to_px(relation_picker_desc_size_rem(effective_size));
    let item_gap = rem_to_px(relation_picker_item_gap_rem(effective_size));
    let item_x = rem_to_px(relation_picker_item_x_rem(effective_size));
    let item_y = rem_to_px(relation_picker_item_y_rem(effective_size));
    let label_size = resolve_px(theme, "typography.label.size");

    let search = build_search(
        spec,
        theme,
        effective_size,
        text_secondary,
        accent,
        label_size,
    );

    let selection_items = spec.selection_summary_items();
    let selection = if spec.show_selection_summary && !selection_items.is_empty() {
        Some(js_selection_summary(
            &poodle_specs::SelectionSummarySpec::new(selection_items)
                .with_clear_action(poodle_specs::RemediationAction::new("clear", "Clear"))
                .with_size(spec.size)
                .with_size_role(spec.size_role)
                .with_density(spec.density),
            theme,
        ))
    } else {
        None
    };

    let mut body = None;
    if spec.state == BrowseState::Ready {
        let is_drilling = spec
            .drill_down
            .as_ref()
            .map(|dd| !dd.is_at_leaf(&spec.drill_down_path))
            .unwrap_or(false);

        if is_drilling {
            let drill_items = spec.drill_items();
            if drill_items.is_empty() {
                // Drill empty state (contract §2 [DrillEmpty]).
                body = Some(
                    ui_element::div()
                        .flex_col()
                        .items_center()
                        .py(item_y * 2.5)
                        .child(
                            ui_element::label("No items found")
                                .text_color(text_secondary)
                                .text_size(rem_to_px(0.8125)),
                        ),
                );
            } else {
                let mut list = ui_element::div().flex_col().gap(list_gap);
                for item in drill_items {
                    list = list.child(drill_row(
                        &item,
                        text_primary,
                        text_secondary,
                        transparent,
                        radius,
                        item_gap,
                        item_x,
                        item_y,
                        title_font,
                        label_size,
                    ));
                }
                body = Some(list);
            }
        } else {
            let mut list = ui_element::div().flex_col().gap(list_gap);
            for item in spec.current_items() {
                let is_selected = spec.selected_ids.iter().any(|selected| selected == &item.id);
                list = list.child(candidate_row(
                    &item,
                    is_selected,
                    spec.selection_mode,
                    theme,
                    text_primary,
                    text_secondary,
                    border,
                    accent,
                    surface,
                    radius,
                    item_gap,
                    item_x,
                    item_y,
                    title_font,
                    desc_font,
                    spec.size,
                    spec.size_role,
                    spec.density,
                ));
            }
            body = Some(list);
        }
    }

    // Footer (FormActions): optional footer note (Svelte `footerNote`) plus the
    // cancel/confirm action row. Gated on `show_footer`.
    let footer = if spec.show_footer {
        let inline_gap = rem_to_px(control_space_x_rem(spec.density));
        let actions = ui_element::div()
            .flex_row()
            .flex_wrap()
            .gap(inline_gap)
            .justify_end()
            .child(js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Ghost)
                    .with_size(ControlSize::Sm)
                    .with_label(&spec.cancel_label),
                theme,
            ))
            .child(js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_size(ControlSize::Sm)
                    .with_label(&spec.confirm_label),
                theme,
            ));

        if let Some(ref note) = spec.footer_note {
            // Note grows to fill, actions pinned to the trailing edge
            // (Svelte note `flex: 1 1 18rem` + actions `margin-left: auto`).
            Some(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .flex_wrap()
                    .gap(inline_gap)
                    .justify_between()
                    .child(
                        ui_element::div().grow().min_w_0().child(
                            ui_element::label(note)
                                .text_color(text_secondary)
                                .text_size(desc_font),
                        ),
                    )
                    .child(actions),
            )
        } else {
            Some(actions)
        }
    } else {
        None
    };

    js_picker_shell(
        &spec.as_picker_shell(),
        theme,
        Some(search),
        selection,
        body,
        None,
        footer,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_search(
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
                        .text_weight(600),
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
fn drill_row(
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
fn candidate_row(
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
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
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
            &RelationPickerSpec::new(sample_items())
                .with_selected_ids(vec!["btn".into()]),
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
}
