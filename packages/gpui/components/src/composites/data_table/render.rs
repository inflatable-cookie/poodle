//! DataTable — full render body (toolbar, filter chips, header, rows,
//! pager). Split out of `data_table/mod.rs` (god-file decomposition); the
//! selection/sort/pagination logic lives in `poodle-specs`, so this is a
//! cohesive element-tree builder. Behavior unchanged.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_specs::{
    ControlDensity, ControlSize, IconSize, IconSpec, StatusTone,
};
use poodle_specs::{
    TableColumnSpec, TablePagination,
};

use crate::presentation::{
    data_table_actions_width_rem, data_table_selection_width_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::primitives::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

use super::DataTable;

impl DataTable {
    pub(super) fn render(self) -> AnyElement {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let body_size = px(rem_to_px(size_font_rem(effective_size)));
        // Svelte: cell horizontal padding is density-based (compact=0.5, default=0.75, comfortable=1.125rem)
        let inline_padding = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.125,
        }));
        // Svelte: cell vertical padding is size-based (not compact bool)
        let cell_py = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.3125,
            ControlSize::Sm => 0.375,
            ControlSize::Md => 0.5,
            ControlSize::Lg => 0.625,
            ControlSize::Xl => 0.75,
        }));
        // Svelte: header label font is smaller than body (xs=0.5625→0.8125rem)
        let header_font = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.5625,
            ControlSize::Sm => 0.625,
            ControlSize::Md => 0.6875,
            ControlSize::Lg => 0.75,
            ControlSize::Xl => 0.8125,
        }));

        let radius_control = resolve_radius(theme, "radius.control");
        let radius_pill = resolve_radius(theme, "radius.pill");
        let label_size = resolve_px(theme, "typography.label.size");
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_md = resolve_px(theme, "space.inline.md");
        let gap_lg = resolve_px(theme, "space.inline.lg");
        let icon_sm = resolve_px(theme, "size.icon.sm");

        // Contract §11 column widths (token/contract-rem resolved, never literal px):
        //   selection column → size-table width (md 3.25rem)
        //   actions column   → fixed 3.5rem
        //   expand column    → icon + symmetric inline padding (GPUI-only affordance)
        let selection_width = px(rem_to_px(data_table_selection_width_rem(effective_size)));
        let actions_width = px(rem_to_px(data_table_actions_width_rem()));
        let expand_width = px(f32::from(icon_sm) + f32::from(inline_padding) * 2.0);
        // Small inset paddings expressed as contract-exact rem (no raw px):
        let pill_py = px(rem_to_px(0.125)); // status pill vertical inset (2px @16)
        let chip_py = px(rem_to_px(0.1875)); // filter chip vertical inset (3px @16)
        let pager_py = px(rem_to_px(0.25)); // pager button vertical inset (4px @16)
        let mixed_bar_w = px(rem_to_px(0.5)); // mixed-state bar (8px @16)
        let mixed_bar_h = px(rem_to_px(0.125)); // mixed-state bar (2px @16)

        let header_bg = resolve_color(theme, spec.header_fill_token());
        let border_color = resolve_color(theme, "color.border.subtle");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent = resolve_color(theme, "color.accent.base");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let success_color = resolve_color(theme, "color.status.success");
        let warning_color = resolve_color(theme, "color.status.warning");
        let danger_color = resolve_color(theme, "color.status.danger");

        // Wrap handlers in Rc for sharing across closures
        let on_sort: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_sort.map(|h| std::rc::Rc::from(h));
        let on_row_click: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_row_click.map(|h| std::rc::Rc::from(h));
        let on_row_select: Option<std::rc::Rc<dyn Fn(&str, bool, &mut Window, &mut App)>> =
            self.on_row_select.map(|h| std::rc::Rc::from(h));
        let on_select_all: Option<std::rc::Rc<dyn Fn(bool, &mut Window, &mut App)>> =
            self.on_select_all.map(|h| std::rc::Rc::from(h));
        let on_row_expand: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_row_expand.map(|h| std::rc::Rc::from(h));
        let on_page_change: Option<std::rc::Rc<dyn Fn(u32, &mut Window, &mut App)>> =
            self.on_page_change.map(|h| std::rc::Rc::from(h));

        // Snapshot visible columns once so the render loop iterates the
        // same set for the header and every row.
        let visible_columns: Vec<TableColumnSpec> = spec.visible_columns().cloned().collect();

        let mut outer = div().w_full().flex().flex_col().gap(gap_md);

        // ── Optional header toolbar (column visibility + export) ──
        if spec.show_column_visibility || spec.show_export {
            let mut toolbar_row = div().flex().items_center().justify_end().gap(gap_md);

            if spec.show_column_visibility {
                toolbar_row = toolbar_row.child(
                    div()
                        .id("dt-toolbar-columns")
                        .flex()
                        .items_center()
                        .gap(gap_sm)
                        .px(gap_md)
                        .py(gap_sm)
                        .border_1()
                        .border_color(border_color)
                        .rounded(radius_control)
                        .cursor_pointer()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(
                            Icon::from_spec(
                                IconSpec::new("columns").with_size(IconSize::Sm),
                                theme,
                            )
                            .with_color(text_secondary),
                        )
                        .child("Columns"),
                );
            }

            if spec.show_export {
                toolbar_row = toolbar_row.child(
                    div()
                        .id("dt-toolbar-export")
                        .flex()
                        .items_center()
                        .gap(gap_sm)
                        .px(gap_md)
                        .py(gap_sm)
                        .border_1()
                        .border_color(border_color)
                        .rounded(radius_control)
                        .cursor_pointer()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(
                            Icon::from_spec(
                                IconSpec::new("download").with_size(IconSize::Sm),
                                theme,
                            )
                            .with_color(text_secondary),
                        )
                        .child("Export"),
                );
            }

            outer = outer.child(toolbar_row);
        }

        // ── Optional filter chip row ──
        if spec.has_filters() {
            let mut filter_row = div().flex().flex_wrap().gap(gap_sm);
            for filter in &spec.filters {
                let chip_bg = Hsla { a: accent.a * 0.12, ..accent };
                filter_row = filter_row.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(gap_sm)
                        .px(gap_sm)
                        .py(chip_py)
                        .rounded(radius_pill)
                        .bg(chip_bg)
                        .text_size(label_size)
                        .text_color(accent)
                        .child(format!("{}: {}", filter.column_id, filter.value)),
                );
            }
            outer = outer.child(filter_row);
        }

        let mut table = div()
            .w_full()
            .flex()
            .flex_col()
            .border_1()
            .border_color(border_color)
            .rounded(radius_control)
            .overflow_hidden();

        // ── Header row ──
        // When spec.sticky_header is set, pin the header with
        // flex_shrink_0 so it isn't compressed out by layout pressure
        // from a vertically-constrained parent. GPUI doesn't expose
        // CSS-style `position: sticky`, so full scroll-pinned behaviour
        // would require splitting the table into header + scrollable
        // body — this gives the practical effect without that refactor.
        let mut header_row = div()
            .w_full()
            .flex()
            .bg(header_bg)
            .border_b_1()
            .border_color(border_color);
        if spec.sticky_header {
            header_row = header_row.flex_shrink_0();
        }

        // Select-all checkbox column
        if spec.selectable {
            let check_state = spec.select_all_state();
            let check_id = SharedString::from("dt-select-all");
            let mut check_cell = div()
                .id(check_id)
                .w(selection_width)
                .flex()
                .items_center()
                .justify_center()
                .py(cell_py)
                .cursor_pointer();

            let box_bg = if check_state == poodle_specs::CheckState::Checked
                || check_state == poodle_specs::CheckState::Mixed
            {
                accent
            } else {
                surface_bg
            };
            let box_border = if check_state == poodle_specs::CheckState::Unchecked {
                border_color
            } else {
                accent
            };
            let mut check_box = div()
                .w(icon_sm)
                .h(icon_sm)
                .rounded(radius_control)
                .border_1()
                .border_color(box_border)
                .bg(box_bg);
            if check_state == poodle_specs::CheckState::Mixed {
                check_box = check_box
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(div().w(mixed_bar_w).h(mixed_bar_h).bg(gpui::white()));
            }
            check_cell = check_cell.child(check_box);

            if let Some(ref handler) = on_select_all {
                let handler = handler.clone();
                let next = check_state != poodle_specs::CheckState::Checked;
                check_cell = check_cell.on_click(move |_event, window, cx| {
                    handler(next, window, cx);
                });
            }

            header_row = header_row.child(check_cell);
        }

        // Expand column placeholder when any row might expand
        let has_expandable = !spec.expanded_row_ids.is_empty() || on_row_expand.is_some();
        if has_expandable {
            header_row = header_row.child(div().w(expand_width));
        }

        for col in &visible_columns {
            let label = col.label.clone();
            let is_sorted = spec
                .sort_column_id
                .as_ref()
                .is_some_and(|sid| sid == &col.id);

            let mut header_cell = div()
                .px(inline_padding)
                .py(cell_py)
                .text_size(header_font)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_secondary)
                .when(col.align_end, |el| el.text_right());

            // Honor explicit column width; otherwise flex-grow.
            if let Some(rem_w) = col.width_rem {
                header_cell = header_cell.w(px(rem_to_px(rem_w)));
            } else {
                header_cell = header_cell.flex_1();
            }

            if is_sorted {
                let sort_icon_name = match spec.sort_direction {
                    poodle_specs::TableSortDirection::Asc => "arrow-up",
                    poodle_specs::TableSortDirection::Desc => "arrow-down",
                };
                header_cell = header_cell
                    .flex()
                    .items_center()
                    .gap(gap_sm)
                    .child(label)
                    .child(
                        Icon::from_spec(
                            IconSpec::new(sort_icon_name).with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(text_secondary),
                    );
            } else {
                header_cell = header_cell.child(label);
            }

            if col.is_sortable {
                header_cell = header_cell.cursor_pointer();
            }

            // Wrap in stateful element for sort click handling
            let header_id = SharedString::from(format!("dt-header-{}", col.id));
            let mut stateful_header = header_cell.id(header_id);
            if col.is_sortable {
                if let Some(ref handler) = on_sort {
                    let handler = handler.clone();
                    let col_id = col.id.clone();
                    stateful_header = stateful_header.on_click(move |_event, window, cx| {
                        handler(&col_id, window, cx);
                    });
                }
            }

            header_row = header_row.child(stateful_header);
        }

        // Row action column header
        if spec.show_row_actions {
            header_row = header_row.child(
                div()
                    .w(actions_width)
                    .px(inline_padding)
                    .py(cell_py)
                    .text_size(header_font)
                    .text_color(text_secondary)
                    .child("Actions"),
            );
        }

        table = table.child(header_row);

        // Data rows or empty message
        if spec.rows.is_empty() {
            let empty_msg = spec.empty_message.as_deref().unwrap_or("No data available");
            table = table.child(
                div()
                    .w_full()
                    .px(inline_padding)
                    .py(gap_lg)
                    .text_size(body_size)
                    .text_color(text_secondary)
                    .text_center()
                    .child(String::from(empty_msg)),
            );
        } else {
            let striped_bg = color_mix(elevated_bg, surface_bg, 0.40);

            for (row_index, row) in spec.rows.iter().enumerate() {
                let is_selected = spec.selected_row_ids.iter().any(|sid| sid == &row.id);
                let is_expanded = spec.is_row_expanded(&row.id);
                let is_striped = spec.striped && row_index % 2 == 1;

                let row_hover_bg = accent.opacity(0.04);
                let selected_bg = accent.opacity(0.08);

                let base_bg = if is_selected {
                    Some(selected_bg)
                } else if is_striped {
                    Some(striped_bg)
                } else {
                    None
                };

                let mut data_row = div()
                    .id(SharedString::from(format!("dt-row-{}", row.id)))
                    .w_full()
                    .flex()
                    .border_b_1()
                    .border_color(border_color.opacity(0.5))
                    .cursor_pointer()
                    .when_some(base_bg, |el, bg| el.bg(bg))
                    .when(!is_selected, move |el| {
                        el.hover(move |s| s.bg(row_hover_bg))
                    });

                if let Some(ref handler) = on_row_click {
                    let handler = handler.clone();
                    let row_id = row.id.clone();
                    data_row = data_row.on_click(move |_event, window, cx| {
                        handler(&row_id, window, cx);
                    });
                }

                // Selection checkbox column
                if spec.selectable {
                    let check_id = SharedString::from(format!("dt-row-check-{}", row.id));
                    let box_bg = if is_selected { accent } else { surface_bg };
                    let box_border = if is_selected { accent } else { border_color };
                    let mut check_cell = div()
                        .id(check_id)
                        .w(selection_width)
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(cell_py)
                        .cursor_pointer()
                        .child(
                            div()
                                .w(icon_sm)
                                .h(icon_sm)
                                .rounded(radius_control)
                                .border_1()
                                .border_color(box_border)
                                .bg(box_bg),
                        );

                    if let Some(ref handler) = on_row_select {
                        let handler = handler.clone();
                        let row_id = row.id.clone();
                        let next = !is_selected;
                        check_cell = check_cell.on_click(move |_event, window, cx| {
                            handler(&row_id, next, window, cx);
                        });
                    }

                    data_row = data_row.child(check_cell);
                }

                // Expand chevron
                if has_expandable {
                    let expand_id = SharedString::from(format!("dt-row-expand-{}", row.id));
                    let chevron_name = if is_expanded {
                        "chevron-down"
                    } else {
                        "chevron-right"
                    };
                    let mut expand_cell = div()
                        .id(expand_id)
                        .w(expand_width)
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(cell_py)
                        .child(
                            Icon::from_spec(
                                IconSpec::new(chevron_name).with_size(IconSize::Sm),
                                theme,
                            )
                            .with_color(text_secondary),
                        );

                    if let Some(ref handler) = on_row_expand {
                        let handler = handler.clone();
                        let row_id = row.id.clone();
                        expand_cell =
                            expand_cell
                                .cursor_pointer()
                                .on_click(move |_event, window, cx| {
                                    handler(&row_id, window, cx);
                                });
                    }

                    data_row = data_row.child(expand_cell);
                }

                for col in &visible_columns {
                    let cell_value = row
                        .cells
                        .iter()
                        .find(|(key, _)| key == &col.id)
                        .map(|(_, val)| val.clone())
                        .unwrap_or_default();

                    let mut cell = div()
                        .px(inline_padding)
                        .py(cell_py)
                        .text_size(body_size)
                        .text_color(text_primary)
                        .when(col.align_end, |el| el.text_right());

                    if let Some(rem_w) = col.width_rem {
                        cell = cell.w(px(rem_to_px(rem_w)));
                    } else {
                        cell = cell.flex_1();
                    }

                    // Cell-tone rendering — wrap the value in a status Pill.
                    if let Some(tone) = row.cell_tone_for(&col.id) {
                        let (fg, bg_mix) = match tone {
                            StatusTone::Success => (success_color, success_color),
                            StatusTone::Warning => (warning_color, warning_color),
                            StatusTone::Danger => (danger_color, danger_color),
                            StatusTone::Info | StatusTone::Pending => (accent, accent),
                            StatusTone::Neutral => (text_secondary, text_secondary),
                        };
                        let tone_bg = color_mix(bg_mix, surface_bg, 0.14);
                        cell = cell.child(
                            div()
                                .flex()
                                .items_center()
                                .flex_shrink_0()
                                .px(gap_sm)
                                .py(pill_py)
                                .rounded(radius_pill)
                                .bg(tone_bg)
                                .text_size(label_size)
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(fg)
                                .child(cell_value),
                        );
                    } else {
                        cell = cell.child(cell_value);
                    }

                    data_row = data_row.child(cell);
                }

                // Row action button
                if spec.show_row_actions {
                    let action_label = spec.row_action_label.clone();
                    let action_id = SharedString::from(format!("dt-action-{}", row.id));
                    let mut action_btn = div()
                        .id(action_id)
                        .w(actions_width)
                        .px(inline_padding)
                        .py(cell_py)
                        .text_size(label_size)
                        .text_color(accent)
                        .cursor_pointer()
                        .hover(|s| s.font_weight(FontWeight::SEMIBOLD))
                        .child(action_label);

                    if let Some(ref handler) = on_row_click {
                        let handler = handler.clone();
                        let row_id = row.id.clone();
                        action_btn = action_btn.on_click(move |_event, window, cx| {
                            handler(&row_id, window, cx);
                        });
                    }
                    data_row = data_row.child(action_btn);
                }

                table = table.child(data_row);

                // Expanded summary row (renders when is_expanded && summary present)
                if is_expanded {
                    if let Some(ref summary) = row.summary {
                        table = table.child(
                            div()
                                .w_full()
                                .px(inline_padding)
                                .py(gap_sm)
                                .bg(elevated_bg.opacity(0.5))
                                .border_b_1()
                                .border_color(border_color.opacity(0.5))
                                .text_size(body_size)
                                .text_color(text_secondary)
                                .child(summary.clone()),
                        );
                    }
                }
            }
        }

        outer = outer.child(table);

        // ── Pagination footer ──
        if let Some(pagination) = spec.pagination {
            let pager = render_pager(
                pagination,
                text_secondary,
                accent,
                border_color,
                inline_padding,
                label_size,
                radius_control,
                gap_sm,
                gap_md,
                pager_py,
                on_page_change.clone(),
            );
            outer = outer.child(pager);
        }

        outer.into_any_element()
    }
}

fn render_pager(
    pagination: TablePagination,
    text_secondary: Hsla,
    accent: Hsla,
    border_color: Hsla,
    inline_padding: Pixels,
    label_size: Pixels,
    radius_control: Pixels,
    gap_sm: Pixels,
    gap_md: Pixels,
    pager_py: Pixels,
    on_page_change: Option<std::rc::Rc<dyn Fn(u32, &mut Window, &mut App)>>,
) -> Div {
    let first = pagination.first_item();
    let last = pagination.last_item();
    let total = pagination.total;
    let current_page = pagination.page;
    let total_pages = pagination.total_pages();

    let summary_text = format!("{first}\u{2013}{last} of {total}");

    let mut pager = div()
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .px(inline_padding)
        .py(gap_sm)
        .text_size(label_size)
        .text_color(text_secondary);

    pager = pager.child(div().child(summary_text));

    let mut controls = div().flex().items_center().gap(gap_md);

    let prev_enabled = current_page > 1;
    let next_enabled = current_page < total_pages;

    let page_button = |id: &str,
                       label: &'static str,
                       enabled: bool,
                       new_page: u32,
                       handler: Option<std::rc::Rc<dyn Fn(u32, &mut Window, &mut App)>>|
     -> AnyElement {
        let mut btn = div()
            .id(SharedString::from(id.to_string()))
            .px(gap_sm)
            .py(pager_py)
            .rounded(radius_control)
            .border_1()
            .border_color(border_color)
            .text_size(label_size)
            .child(label);

        if enabled {
            btn = btn.cursor_pointer().text_color(accent);
            if let Some(handler) = handler {
                btn = btn.on_click(move |_event, window, cx| {
                    handler(new_page, window, cx);
                });
            }
        } else {
            btn = btn.text_color(text_secondary.opacity(0.4));
        }
        btn.into_any_element()
    };

    controls = controls.child(page_button(
        "dt-pager-prev",
        "Prev",
        prev_enabled,
        current_page.saturating_sub(1).max(1),
        on_page_change.clone(),
    ));

    controls = controls.child(
        div()
            .text_size(label_size)
            .text_color(text_secondary)
            .child(format!("Page {current_page} of {total_pages}")),
    );

    controls = controls.child(page_button(
        "dt-pager-next",
        "Next",
        next_enabled,
        (current_page + 1).min(total_pages),
        on_page_change,
    ));

    pager = pager.child(controls);
    pager
}

