//! RelationPicker — Jetstream relation picker backed by RelationPickerSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    BrowseState, ButtonSpec, ButtonVariant, CheckboxSpec, PickerItemSpec, RelationPickerSpec,
    SelectionMode,
};

use crate::button::js_button;
use crate::checkbox::js_checkbox;
use crate::picker_shell::js_picker_shell;
use crate::selection_summary::js_selection_summary;
use crate::theme_ext::{resolve_color, resolve_radius, tint};

pub fn js_relation_picker(spec: &RelationPickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.subtle");
    let accent = resolve_color(theme, "color.accent.base");
    let surface = resolve_color(theme, "color.background.surface");
    let elevated = resolve_color(theme, "color.background.elevated");
    let radius = resolve_radius(theme, "radius.control");

    let search = build_search(
        spec,
        theme,
        text_primary,
        text_secondary,
        border,
        surface,
        radius,
    );

    let selection_items = spec.selection_summary_items();
    let selection = if !selection_items.is_empty() {
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
        if spec.drill_down.is_some()
            && spec
                .drill_down
                .as_ref()
                .map(|dd| !dd.is_at_leaf(&spec.drill_down_path))
                .unwrap_or(false)
        {
            let mut list = ui_element::div().flex_col().gap(2.0);
            for item in spec.drill_items() {
                list = list.child(drill_row(
                    &item,
                    theme,
                    text_primary,
                    text_secondary,
                    border,
                    surface,
                    elevated,
                    radius,
                ));
            }
            body = Some(list);
        } else {
            let mut list = ui_element::div().flex_col().gap(2.0);
            for item in spec.current_items() {
                let is_selected = spec
                    .selected_ids
                    .iter()
                    .any(|selected| selected == &item.id);
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
                    elevated,
                    radius,
                    spec.size,
                    spec.size_role,
                    spec.density,
                ));
            }
            body = Some(list);
        }
    }

    let footer_actions = ui_element::div()
        .flex_row()
        .gap(8.0)
        .child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_size(poodle_specs::ControlSize::Sm)
                .with_label(&spec.cancel_label),
            theme,
        ))
        .child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_size(poodle_specs::ControlSize::Sm)
                .with_label(&spec.confirm_label),
            theme,
        ));

    js_picker_shell(
        &spec.as_picker_shell(),
        theme,
        Some(search),
        selection,
        body,
        None,
        Some(footer_actions),
    )
}

fn build_search(
    spec: &RelationPickerSpec,
    _theme: &JetstreamThemeProvider,
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    border: glam::Vec4,
    surface: glam::Vec4,
    radius: f32,
) -> JsEl {
    let mut col = ui_element::div().flex_col().gap(8.0);

    if let Some(ref drill_down) = spec.drill_down {
        if !spec.drill_down_path.is_empty() {
            let mut crumbs = ui_element::div().flex_row().items_center().gap(4.0);
            crumbs = crumbs.child(
                ui_element::button("Back")
                    .text_color(text_secondary)
                    .text_size(12.0)
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
                            .text_size(11.0),
                    )
                    .child(
                        ui_element::label(&label)
                            .text_color(accent_or(text_primary, text_primary))
                            .text_size(12.0),
                    );
            }

            col = col.child(crumbs);
        }

        if !drill_down.is_at_leaf(&spec.drill_down_path) {
            if let Some(level) = drill_down.next_level(&spec.drill_down_path) {
                col = col.child(
                    ui_element::label(&level.label.to_uppercase())
                        .text_color(text_secondary)
                        .text_size(12.0)
                        .text_weight(600),
                );
            }
        }
    }

    col.child(
        ui_element::div()
            .flex_row()
            .items_center()
            .gap(8.0)
            .pl(10.0)
            .pr(10.0)
            .pt(6.0)
            .pb(6.0)
            .border(1.0)
            .border_color(border)
            .rounded(radius)
            .bg(surface)
            .child(
                ui_element::icon("search")
                    .w(14.0)
                    .h(14.0)
                    .text_color(text_secondary),
            )
            .child(
                ui_element::label(if spec.query.is_empty() {
                    "Search…"
                } else {
                    spec.query.as_str()
                })
                .text_color(if spec.query.is_empty() {
                    text_secondary
                } else {
                    text_primary
                })
                .text_size(13.0),
            ),
    )
}

fn drill_row(
    item: &poodle_specs::DrillDownItem,
    theme: &JetstreamThemeProvider,
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    border: glam::Vec4,
    surface: glam::Vec4,
    elevated: glam::Vec4,
    radius: f32,
) -> JsEl {
    let meta = item
        .count
        .map(|count| format!("{count} items"))
        .unwrap_or_default();
    ui_element::button("")
        .flex_row()
        .items_center()
        .justify_between()
        .gap(12.0)
        .pl(12.0)
        .pr(12.0)
        .pt(8.0)
        .pb(8.0)
        .rounded(radius)
        .border(1.0)
        .border_color(border)
        .bg(tint(elevated, 0.88))
        .focusable()
        .child(
            ui_element::div()
                .flex_col()
                .gap(2.0)
                .child(
                    ui_element::label(&item.label)
                        .text_color(text_primary)
                        .text_size(14.0)
                        .text_weight(600),
                )
                .child(
                    ui_element::label(item.description.as_deref().unwrap_or(""))
                        .text_color(text_secondary)
                        .text_size(12.0),
                ),
        )
        .child(
            ui_element::div()
                .flex_row()
                .items_center()
                .gap(8.0)
                .child(
                    ui_element::label(&meta)
                        .text_color(text_secondary)
                        .text_size(12.0),
                )
                .child(
                    ui_element::icon("chevron-right")
                        .w(14.0)
                        .h(14.0)
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
    elevated: glam::Vec4,
    radius: f32,
    size: poodle_specs::ControlSize,
    size_role: poodle_specs::SemanticControlSizeRole,
    density: poodle_specs::ControlDensity,
) -> JsEl {
    let row_bg = if is_selected {
        tint(accent, 0.10)
    } else {
        tint(surface, 0.86)
    };
    let row_border = if is_selected {
        tint(accent, 0.60)
    } else {
        border
    };

    let mut row = ui_element::button("")
        .flex_row()
        .items_center()
        .gap(8.0)
        .pl(12.0)
        .pr(12.0)
        .pt(8.0)
        .pb(8.0)
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
            .gap(2.0)
            .child(
                ui_element::label(&item.label)
                    .text_color(text_primary)
                    .text_size(14.0)
                    .text_weight(600),
            )
            .children(item.description.as_ref().map(|description| {
                ui_element::label(description)
                    .text_color(text_secondary)
                    .text_size(12.0)
            }))
            .children(item.meta.as_ref().map(|meta| {
                ui_element::label(meta)
                    .text_color(text_secondary)
                    .text_size(12.0)
            })),
    )
}

fn accent_or(accent: glam::Vec4, fallback: glam::Vec4) -> glam::Vec4 {
    if accent == glam::Vec4::ZERO {
        fallback
    } else {
        accent
    }
}
