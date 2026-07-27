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

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    BrowseState, ButtonSpec, ButtonVariant, ControlSize,
    RelationPickerSpec,
};

use crate::button::js_button;
use crate::picker_shell::js_picker_shell;
use crate::presentation::{
    control_space_x_rem, rem_to_px, relation_picker_desc_size_rem, relation_picker_item_gap_rem,
    relation_picker_item_x_rem, relation_picker_item_y_rem, relation_picker_list_gap_rem,
    relation_picker_title_size_rem, resolve_semantic_size,
};
use crate::selection_summary::js_selection_summary;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

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

    let root = js_picker_shell(
        &spec.as_picker_shell(),
        theme,
        Some(search),
        selection,
        body,
        None,
        footer,
    );
    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
}

#[allow(clippy::too_many_arguments)]

mod parts;
use parts::{build_search, candidate_row, drill_row};
