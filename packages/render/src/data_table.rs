//! DataTable — rows, columns, sorting and selection.
//!
//! Contract: `docs/contracts/components/data-table.md`
//! Ported from: `packages/jetstream/components/src/data_table.rs`.
//!
//! Anatomy (contract §3): Toolbar (export + column-visibility buttons),
//! Header row (select-all + column labels + active sort icon + actions
//! header), Filter chip row, Body (selection checkbox / plain / status-pill
//! cells, zebra stripes, selected tint, expanded summaries, empty state),
//! Pagination footer. All flex rows — the Svelte grid resolves to fixed
//! selection/actions widths plus growing data columns.
//!
//! Sort cycling, export, column-visibility popover, filter edits and the
//! pager are host-owned. `on_sort` fires with the column's id (the column,
//! not a direction: cycling asc → desc → none is the host's rule to apply).

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, StylePatch, TextAlign,
};
use poodle_specs::{CheckState, CheckboxSpec, DataTableSpec, StatusTone, TableSortDirection};

use crate::checkbox::checkbox;
use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::skeleton::skeleton;

/// Selection-column width in rem per size (contract §11 size table).
fn selection_width_rem(size: poodle_specs::ControlSize) -> f32 {
    match size {
        poodle_specs::ControlSize::Xs => 2.5,
        poodle_specs::ControlSize::Sm => 2.75,
        poodle_specs::ControlSize::Md => 3.25,
        poodle_specs::ControlSize::Lg => 3.625,
        poodle_specs::ControlSize::Xl => 4.0,
    }
}

/// Row-actions column width in rem. Contract `.data-table__actions` is a
/// fixed `3.5rem` across all sizes.
fn actions_width_rem() -> f32 {
    3.5
}

/// Host callbacks. `on_sort` fires with the column id; `on_row_click` /
/// `on_row_select` with the row id.
#[derive(Default)]
pub struct DataTableHandlers {
    pub on_row_click: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_sort: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_row_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_select_all: Option<Arc<dyn Fn() + Send + Sync>>,
}

fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

fn row_shell(direction: LayoutDirection) -> Node {
    let mut n = Node::container();
    n.style.descriptor.layout.direction = direction;
    n
}

pub fn data_table(
    spec: &DataTableSpec,
    theme: &dyn ThemeProvider,
    handlers: DataTableHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let header_font = rem_to_px(size_font_rem(effective_size) - 0.0625);
    let body_font = rem_to_px(size_font_rem(effective_size));
    let label_size = theme.resolve_space("typography.label.size");
    let cell_px = rem_to_px(control_space_x_rem(spec.density));
    // Svelte uses the effective control size for both header and body cell
    // padding. Keep the two rows on one metric ladder so size variants do
    // not drift vertically between backends.
    let cell_py = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.3125,
        poodle_specs::ControlSize::Sm => 0.375,
        poodle_specs::ControlSize::Md => 0.5,
        poodle_specs::ControlSize::Lg => 0.625,
        poodle_specs::ControlSize::Xl => 0.75,
    });
    let gap_sm = theme.resolve_space("space.inline.sm");
    let gap_md = theme.resolve_space("space.inline.md");
    let icon_sm = theme.resolve_space("size.icon.sm");
    // Contract §11 column widths (size-table selection, fixed 3.5rem actions).
    let selection_width = rem_to_px(selection_width_rem(effective_size));
    let actions_width = rem_to_px(actions_width_rem());

    let fill = theme.resolve_color("color.background.surface");
    let border = theme.resolve_color("color.border.subtle");
    let border_default = theme.resolve_color("color.border.default");
    let header_fill = theme.resolve_color(spec.header_fill_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");
    let surface = theme.resolve_color("color.background.surface");
    let success = theme.resolve_color("color.status.success");
    let warning = theme.resolve_color("color.status.warning");
    let danger = theme.resolve_color("color.status.danger");
    let radius = theme.resolve_radius("radius.surface");
    let radius_control = theme.resolve_radius("radius.control");
    let radius_pill = theme.resolve_radius("radius.pill");
    let header_hover = with_alpha(accent, accent.3 * 0.10);

    // Zebra stripe tint for even-indexed rows
    let stripe_tint = with_alpha(surface, surface.3 * 0.04);
    // Selected row tint (accent at 8% opacity — mirrors Svelte `color-mix 8%`)
    let selected_tint = with_alpha(accent, accent.3 * 0.08);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
    }
    all_corners(&mut el, radius);

    // ── Toolbar (export + column-visibility) ──────────────────────
    // Contract §9: rendered above the table when showExport or
    // showColumnVisibility is set. Buttons render chrome; the export/popover
    // behaviour is host-owned.
    if spec.show_export || spec.show_column_visibility {
        let toolbar_btn = |icon_name: &str, text: &str| -> Node {
            let mut b = row_shell(LayoutDirection::Row);
            {
                let s = &mut b.style;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = gap_sm;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = cell_px;
                pad.right = cell_px;
                pad.top = cell_py;
                pad.bottom = cell_py;
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = border_default;
                s.descriptor.cursor = CursorHint::Pointer;
            }
            all_corners(&mut b, radius_control);
            b.interaction.focusable = true;
            let mut glyph = Node::icon(icon_name, icon_sm);
            glyph.style.descriptor.text_color = Some(text_secondary);
            let mut label = Node::text(text);
            label.style.descriptor.text_color = Some(text_secondary);
            label.style.text_size = Some(label_size);
            b.child(glyph).child(label)
        };

        let mut toolbar = row_shell(LayoutDirection::Row);
        {
            let s = &mut toolbar.style;
            s.descriptor.background = Some(header_fill);
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            s.descriptor.layout.spacing.gap = gap_md;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = cell_py;
            pad.bottom = cell_py;
            s.border_bottom_width = Some(1.0);
            s.descriptor.border.color = border;
        }

        if spec.show_export {
            toolbar = toolbar.child(toolbar_btn("download", "Export"));
        }
        if spec.show_column_visibility {
            toolbar = toolbar.child(toolbar_btn("columns-3", "Columns"));
        }
        el = el.child(toolbar);
    }

    // ── Header row ────────────────────────────────────────────────
    let mut header = row_shell(LayoutDirection::Row);
    {
        let s = &mut header.style;
        s.descriptor.background = Some(header_fill);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = cell_py;
        pad.bottom = cell_py;
    }

    // Selectable: "select all" checkbox column header
    if spec.selectable {
        let cb_spec = match spec.select_all_state() {
            CheckState::Checked => CheckboxSpec::new().with_checked(true),
            CheckState::Mixed => CheckboxSpec::new().with_mixed(true),
            CheckState::Unchecked => CheckboxSpec::new(),
        }
        // Header checkbox: selects every row, and has no caption of its own.
        .with_aria_label("Select all rows");
        let mut cell = row_shell(LayoutDirection::Row);
        {
            let s = &mut cell.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(selection_width);
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        }
        let mut cell = cell.child(checkbox(&cb_spec, theme, None));
        if let Some(handler) = &handlers.on_select_all {
            let handler = Arc::clone(handler);
            cell.style.descriptor.cursor = CursorHint::Pointer;
            cell.interaction.on_activate = Some(Arc::new(move || handler()));
        }
        header = header.child(cell);
    }

    for col in spec.visible_columns() {
        let is_sorted = spec.sort_column_id.as_deref() == Some(&*col.id);

        let mut col_cell = row_shell(LayoutDirection::Row);
        {
            let s = &mut col_cell.style;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = rem_to_px(0.25);
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.descriptor.layout.spacing.padding.left = cell_px;
            s.descriptor.layout.spacing.padding.right = cell_px;
            if col.align_end {
                s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            }
        }

        let mut label = Node::text(&col.label);
        label.style.descriptor.text_color = Some(text_secondary);
        label.style.text_size = Some(header_font);
        label.style.text_weight = Some(600);
        let mut col_cell = col_cell.child(label);

        // Active sort column shows an arrow icon (size from size.icon.sm).
        // Svelte renders NO indicator on unsorted columns — match that.
        if col.is_sortable {
            if is_sorted {
                let icon_name = match spec.sort_direction {
                    TableSortDirection::Asc => "arrow-up",
                    TableSortDirection::Desc => "arrow-down",
                };
                let mut arrow = Node::icon(icon_name, icon_sm);
                arrow.style.descriptor.text_color = Some(accent);
                col_cell = col_cell.child(arrow);
            }
            // Sortable columns show a pointer cursor and accent tint on hover.
            col_cell.style.descriptor.cursor = CursorHint::Pointer;
            col_cell.interaction.focusable = true;
            col_cell.style.hover = Some(StylePatch {
                background: Some(header_hover),
                border_color: None,
                text_color: None,
                opacity: None,
            });

            if let Some(handler) = &handlers.on_sort {
                let handler = Arc::clone(handler);
                let id = col.id.clone();
                col_cell.interaction.on_activate = Some(Arc::new(move || handler(&id)));
            }
        }

        header = header.child(col_cell);
    }

    // Actions column header (fixed 3.5rem) when row actions are shown.
    if spec.show_row_actions {
        let mut cell = row_shell(LayoutDirection::Row);
        {
            let s = &mut cell.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(actions_width);
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            s.descriptor.layout.spacing.padding.left = cell_px;
            s.descriptor.layout.spacing.padding.right = cell_px;
        }
        let mut label = Node::text("Actions");
        label.style.descriptor.text_color = Some(text_secondary);
        label.style.text_size = Some(header_font);
        label.style.text_weight = Some(600);
        header = header.child(cell.child(label));
    }

    el = el.child(header);

    // ── Filter chip row ───────────────────────────────────────────
    // Contract §3 Filter Row. Active filters render as accent chips; the
    // per-column filter inputs are host-owned.
    if spec.has_filters() {
        let mut filter_row = row_shell(LayoutDirection::Row);
        {
            let s = &mut filter_row.style;
            s.descriptor.background = Some(header_fill);
            s.flex_wrap = true;
            s.descriptor.layout.spacing.gap = gap_sm;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = cell_px;
            pad.right = cell_px;
            pad.top = cell_py;
            pad.bottom = cell_py;
            s.border_bottom_width = Some(1.0);
            s.descriptor.border.color = border;
        }
        for filter in &spec.filters {
            let mut chip = Node::text(format!("{}: {}", filter.column_id, filter.value));
            {
                let s = &mut chip.style;
                s.descriptor.text_color = Some(accent);
                s.text_size = Some(label_size);
                s.descriptor.background = Some(with_alpha(accent, accent.3 * 0.12));
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = gap_sm;
                pad.right = gap_sm;
                pad.top = rem_to_px(0.1875);
                pad.bottom = rem_to_px(0.1875);
            }
            all_corners(&mut chip, radius_pill);
            filter_row = filter_row.child(chip);
        }
        el = el.child(filter_row);
    }

    // ── Body ──────────────────────────────────────────────────────
    if spec.rows.is_empty() {
        // Empty state
        let empty_msg = spec.empty_message.as_deref().unwrap_or("No results");
        let mut empty = row_shell(LayoutDirection::Row);
        {
            let s = &mut empty.style;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = cell_px;
            pad.right = cell_px;
            pad.top = cell_py;
            pad.bottom = cell_py;
        }
        let mut msg = Node::text(empty_msg);
        msg.style.descriptor.text_color = Some(text_secondary);
        msg.style.text_size = Some(body_font);
        el = el.child(empty.child(msg));
    } else {
        let visible_cols: Vec<_> = spec.visible_columns().collect();

        for (row_index, row) in spec.rows.iter().enumerate() {
            let is_selected = spec.selected_row_ids.iter().any(|id| id == &row.id);

            // Row background: selected > default/stripe
            let row_bg = if is_selected {
                selected_tint
            } else if spec.striped && row_index % 2 == 0 {
                stripe_tint
            } else {
                // Transparent — a zero-alpha version of surface
                with_alpha(surface, 0.0)
            };

            let mut row_el = row_shell(LayoutDirection::Row);
            {
                let s = &mut row_el.style;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.top = cell_py;
                pad.bottom = cell_py;
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = border;
                s.descriptor.background = Some(row_bg);
            }

            // Row-selection checkbox
            if spec.selectable {
                // Per-row selection: the row's cells are siblings, not
                // children, so there is nothing to name it from.
                let cb_spec = CheckboxSpec::new()
                    .with_checked(is_selected)
                    .with_aria_label("Select row");
                let mut cell = row_shell(LayoutDirection::Row);
                {
                    let s = &mut cell.style;
                    s.descriptor.layout.width = LayoutSizing::Fixed(selection_width);
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                }
                let mut cell = cell.child(checkbox(&cb_spec, theme, None));

                // Its own handler, always: selecting a row and opening it are
                // different intents, and an unwired checkbox that bubbled
                // would open the row you were only ticking.
                if let Some(handler) = &handlers.on_row_select {
                    let handler = Arc::clone(handler);
                    let id = row.id.clone();
                    cell.style.descriptor.cursor = CursorHint::Pointer;
                    cell.interaction.on_activate = Some(Arc::new(move || handler(&id)));
                } else {
                    cell.interaction.on_activate = Some(Arc::new(|| {}));
                }

                row_el = row_el.child(cell);
            }

            for col in &visible_cols {
                let value = row
                    .cells
                    .iter()
                    .find(|(k, _)| k == &col.id)
                    .map(|(_, v)| v.as_str())
                    .unwrap_or("—");

                // Status-pill cell (Svelte custom-cell pattern via cell_tones).
                if let Some(tone) = row.cell_tone_for(&col.id) {
                    let base: ColorValue = match tone {
                        StatusTone::Success => success,
                        StatusTone::Warning => warning,
                        StatusTone::Danger => danger,
                        StatusTone::Info | StatusTone::Pending => accent,
                        StatusTone::Neutral => text_secondary,
                    };
                    let pill_bg = mix_srgb(base, surface, 0.14);
                    let mut cell = row_shell(LayoutDirection::Row);
                    {
                        let s = &mut cell.style;
                        s.descriptor.layout.width = LayoutSizing::Grow;
                        s.descriptor.layout.spacing.padding.left = cell_px;
                        s.descriptor.layout.spacing.padding.right = cell_px;
                        if col.align_end {
                            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
                        }
                    }
                    let mut pill = Node::text(value);
                    {
                        let s = &mut pill.style;
                        s.descriptor.text_color = Some(base);
                        s.text_size = Some(label_size);
                        s.text_weight = Some(600);
                        s.descriptor.background = Some(pill_bg);
                        let pad = &mut s.descriptor.layout.spacing.padding;
                        pad.left = gap_sm;
                        pad.right = gap_sm;
                        pad.top = rem_to_px(0.125);
                        pad.bottom = rem_to_px(0.125);
                    }
                    all_corners(&mut pill, radius_pill);
                    row_el = row_el.child(cell.child(pill));
                } else {
                    let mut cell = Node::text(value);
                    cell.style.descriptor.text_color = Some(text_primary);
                    cell.style.text_size = Some(body_font);
                    cell.style.descriptor.layout.width = LayoutSizing::Grow;
                    cell.style.descriptor.layout.spacing.padding.left = cell_px;
                    cell.style.descriptor.layout.spacing.padding.right = cell_px;
                    if col.align_end {
                        cell.style.text_align = Some(TextAlign::Right);
                    }
                    row_el = row_el.child(cell);
                }
            }

            // Row-actions cell (fixed 3.5rem) — legacy single action label.
            if spec.show_row_actions {
                let mut cell = row_shell(LayoutDirection::Row);
                {
                    let s = &mut cell.style;
                    s.descriptor.layout.width = LayoutSizing::Fixed(actions_width);
                    s.descriptor.layout.alignment.main = MainAxisAlignment::End;
                    s.descriptor.layout.spacing.padding.left = cell_px;
                    s.descriptor.layout.spacing.padding.right = cell_px;
                }
                let mut action = Node::text(&spec.row_action_label);
                action.style.descriptor.text_color = Some(accent);
                action.style.text_size = Some(label_size);
                action.style.descriptor.cursor = CursorHint::Pointer;
                action.interaction.focusable = true;
                row_el = row_el.child(cell.child(action));
            }

            if let Some(handler) = &handlers.on_row_click {
                let handler = Arc::clone(handler);
                let id = row.id.clone();
                row_el.style.descriptor.cursor = CursorHint::Pointer;
                row_el.interaction.on_activate = Some(Arc::new(move || handler(&id)));
            }

            el = el.child(row_el);

            // Expanded row summary
            if spec.is_row_expanded(&row.id) {
                if let Some(ref summary) = row.summary {
                    let mut expand = row_shell(LayoutDirection::Row);
                    {
                        let s = &mut expand.style;
                        let pad = &mut s.descriptor.layout.spacing.padding;
                        pad.left = cell_px;
                        pad.right = cell_px;
                        pad.top = cell_py;
                        pad.bottom = cell_py;
                        s.descriptor.border.width = 1.0;
                        s.descriptor.border.color = border;
                    }
                    let mut text = Node::text(summary);
                    text.style.descriptor.text_color = Some(text_secondary);
                    text.style.text_size = Some(body_font);
                    el = el.child(expand.child(text));
                }
            }
        }
    }

    // ── Pagination footer ─────────────────────────────────────────
    // Contract §3 Footer: summary + (limit selector host-owned) + prev/next
    // controls. Pager clicks are host-owned.
    if let Some(pagination) = spec.pagination {
        let summary = format!(
            "{}\u{2013}{} of {}",
            pagination.first_item(),
            pagination.last_item(),
            pagination.total
        );
        let total_pages = pagination.total_pages();
        let page_label = format!("Page {} of {}", pagination.page, total_pages);

        let pager_btn = |text: &str, enabled: bool| -> Node {
            let color = if enabled {
                accent
            } else {
                with_alpha(text_secondary, text_secondary.3 * 0.4)
            };
            let mut b = Node::text(text);
            {
                let s = &mut b.style;
                s.descriptor.text_color = Some(color);
                s.text_size = Some(label_size);
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = border_default;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = gap_sm;
                pad.right = gap_sm;
                pad.top = rem_to_px(0.25);
                pad.bottom = rem_to_px(0.25);
            }
            all_corners(&mut b, radius_control);
            if enabled {
                b.style.descriptor.cursor = CursorHint::Pointer;
                b.interaction.focusable = true;
            }
            b
        };

        let mut controls = row_shell(LayoutDirection::Row);
        {
            let s = &mut controls.style;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = gap_md;
        }
        let mut page_text = Node::text(&page_label);
        page_text.style.descriptor.text_color = Some(text_secondary);
        page_text.style.text_size = Some(label_size);
        let controls = controls
            .child(pager_btn("Prev", pagination.page > 1))
            .child(page_text)
            .child(pager_btn("Next", pagination.page < total_pages));

        let mut footer = row_shell(LayoutDirection::Row);
        {
            let s = &mut footer.style;
            s.descriptor.background = Some(header_fill);
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = cell_px;
            pad.right = cell_px;
            pad.top = cell_py;
            pad.bottom = cell_py;
            s.border_top_width = Some(1.0);
            s.descriptor.border.color = border;
        }
        let mut summary_text = Node::text(&summary);
        summary_text.style.descriptor.text_color = Some(text_secondary);
        summary_text.style.text_size = Some(label_size);
        el = el.child(footer.child(summary_text).child(controls));
    }

    if !spec.aria_label.is_empty() {
        el.a11y.label = Some(spec.aria_label.clone());
    }
    el
}

/// Render a loading skeleton for the data table body (used when data is
/// in-flight). Renders `row_count` skeleton rows, each with a skeleton per
/// visible column.
pub fn data_table_loading(
    spec: &DataTableSpec,
    theme: &dyn ThemeProvider,
    row_count: usize,
) -> Node {
    use poodle_specs::SkeletonSpec;

    let cell_gap = rem_to_px(0.5);
    let cell_px = rem_to_px(control_space_x_rem(spec.density));
    let row_py = rem_to_px(panel_space_y_rem(spec.density) - 0.125);
    let border = theme.resolve_color("color.border.subtle");
    let skel_spec = SkeletonSpec::new();

    let mut el = Node::container();
    el.style.descriptor.layout.direction = LayoutDirection::Column;

    for _ in 0..row_count {
        let mut row_el = row_shell(LayoutDirection::Row);
        {
            let s = &mut row_el.style;
            s.descriptor.layout.spacing.gap = cell_gap;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = cell_px;
            pad.right = cell_px;
            pad.top = row_py;
            pad.bottom = row_py;
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = border;
        }

        if spec.selectable {
            row_el = row_el.child(skeleton(&skel_spec, theme));
        }
        for _ in spec.visible_columns() {
            let mut cell = skeleton(&skel_spec, theme);
            cell.style.descriptor.layout.width = LayoutSizing::Grow;
            row_el = row_el.child(cell);
        }
        el = el.child(row_el);
    }

    if !spec.aria_label.is_empty() {
        el.a11y.label = Some(spec.aria_label.clone());
    }
    el
}
