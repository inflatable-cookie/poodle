//! RelationPicker — pick related records, with drill-down.
//!
//! Contract: `docs/contracts/components/relation-picker.md`
//! Ported from: `packages/jetstream/components/src/relation_picker/` (mod +
//! parts, merged).
//!
//! Render-only: candidate toggling, drill navigation (advance / back /
//! breadcrumb jump), live search typing, and keyboard nav are host-owned —
//! this builder renders the current open state (search field, selection
//! summary, candidate/drill list, footer). All geometry, colors, and type
//! sizes resolve from size/density tokens.

use std::sync::Arc;

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node,
};
use poodle_specs::{
    BrowseState, ButtonSpec, ButtonVariant, CheckboxSpec, ChoiceOption, ControlSize,
    PickerItemSpec, RelationPickerSpec, SelectSpec, SelectionMode, TextInputSpec,
};

use crate::button::button;
use crate::checkbox::checkbox;
use crate::color::{mix_srgb, TRANSPARENT};
use crate::context::RenderContext;
use crate::picker_shell::picker_shell;
use crate::presentation::{
    control_space_x_rem, relation_picker_desc_size_rem, relation_picker_item_gap_rem,
    relation_picker_item_x_rem, relation_picker_item_y_rem, relation_picker_list_gap_rem,
    relation_picker_title_size_rem, rem_to_px,
};
use crate::select::{select, SelectHandlers};
use crate::selection_summary::{selection_summary, SelectionSummaryHandlers};
use crate::text_input::text_input;

/// Candidate / drill copy strong label weight (Svelte `strong { font-weight: 500 }`).
const LABEL_WEIGHT: u16 = 500;

/// Handlers mirror the GPUI target's names. No `on_query_change` /
/// `on_filter_change`: both are typed, and the vocabulary raises no key
/// events.
#[derive(Default)]
pub struct RelationPickerHandlers {
    /// Fires with the candidate's id. The host resolves the click into the
    /// next selection — single- and multi-select are its policy.
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the drill context's id when a drill row is entered.
    pub on_drill_enter: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the retained path depth when back/breadcrumb navigation is
    /// activated. The host truncates its drill path to that depth.
    pub on_breadcrumb_click: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub on_confirm: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

fn all_radius(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

fn label_node(content: &str, color: ColorValue, size: f32, weight: Option<u16>) -> Node {
    let mut t = Node::text(content);
    t.style.descriptor.text_color = Some(color);
    t.style.text_size = Some(size);
    t.style.text_weight = weight;
    t
}

pub fn relation_picker(
    spec: &RelationPickerSpec,
    ctx: &RenderContext<'_>,
    handlers: RelationPickerHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let border = ctx.theme().resolve_color("color.border.subtle");
    let accent = ctx.theme().resolve_color("color.accent.base");
    let surface = ctx.theme().resolve_color("color.background.surface");
    let radius = ctx.theme().resolve_radius("radius.control");

    // Density-driven inter-row gap (contract §8 density table).
    let list_gap = rem_to_px(relation_picker_list_gap_rem(density));
    let title_font = rem_to_px(relation_picker_title_size_rem(effective_size));
    let desc_font = rem_to_px(relation_picker_desc_size_rem(effective_size));
    let item_gap = rem_to_px(relation_picker_item_gap_rem(effective_size));
    let item_x = rem_to_px(relation_picker_item_x_rem(effective_size));
    let item_y = rem_to_px(relation_picker_item_y_rem(effective_size));
    let label_size = ctx.theme().resolve_space("typography.label.size");

    let search = build_search(
        spec,
        ctx,
        effective_size,
        text_secondary,
        accent,
        label_size,
        &handlers,
    );

    let selection_items = spec.selection_summary_items();
    let selection = if spec.show_selection_summary && !selection_items.is_empty() {
        Some(selection_summary(
            &poodle_specs::SelectionSummarySpec::new(selection_items)
                .with_clear_action(poodle_specs::RemediationAction::new("clear", "Clear"))
                .with_size(ctx.base_size(spec.size))
                .with_size_role(spec.size_role)
                .with_density(density),
            ctx,
            SelectionSummaryHandlers::default(),
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
                let mut state = Node::container();
                {
                    let s = &mut state.style;
                    s.descriptor.layout.direction = LayoutDirection::Column;
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    let pad = &mut s.descriptor.layout.spacing.padding;
                    pad.top = item_y * 2.5;
                    pad.bottom = item_y * 2.5;
                }
                body = Some(state.child(label_node(
                    "No items found",
                    text_secondary,
                    rem_to_px(0.8125),
                    None,
                )));
            } else {
                let mut list = Node::container();
                list.style.descriptor.layout.direction = LayoutDirection::Column;
                list.style.descriptor.layout.spacing.gap = list_gap;
                let mut list = list;
                for item in drill_items {
                    let mut row = drill_row(
                        &item,
                        text_primary,
                        text_secondary,
                        radius,
                        item_gap,
                        item_x,
                        item_y,
                        title_font,
                        label_size,
                    );
                    if let Some(handler) = &handlers.on_drill_enter {
                        let handler = Arc::clone(handler);
                        let id = item.id.clone();
                        row.style.descriptor.cursor = CursorHint::Pointer;
                        row.interaction.on_activate = Some(Arc::new(move || handler(&id)));
                    }
                    list = list.child(row);
                }
                body = Some(list);
            }
        } else {
            let mut list = Node::container();
            list.style.descriptor.layout.direction = LayoutDirection::Column;
            list.style.descriptor.layout.spacing.gap = list_gap;
            let mut list = list;
            for item in spec.current_items() {
                let is_selected = spec
                    .selected_ids
                    .iter()
                    .any(|selected| selected == &item.id);
                let mut row = candidate_row(
                    &item,
                    is_selected,
                    spec.selection_mode,
                    ctx,
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
                    spec,
                );
                if let Some(handler) = &handlers.on_select {
                    let handler = Arc::clone(handler);
                    let id = item.id.clone();
                    row.style.descriptor.cursor = CursorHint::Pointer;
                    row.interaction.on_activate = Some(Arc::new(move || handler(&id)));
                }
                list = list.child(row);
            }
            body = Some(list);
        }
    }

    // Footer (FormActions): optional footer note (Svelte `footerNote`) plus
    // the cancel/confirm action row. Gated on `show_footer`.
    let footer = if spec.show_footer {
        let inline_gap = rem_to_px(control_space_x_rem(density));
        let mut actions = Node::container();
        {
            let s = &mut actions.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_wrap = true;
            s.descriptor.layout.spacing.gap = inline_gap;
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
        }
        let actions = actions
            .child(button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Ghost)
                    .with_size(ControlSize::Sm)
                    .with_label(&spec.cancel_label),
                ctx,
                handlers.on_cancel.as_ref().map(Arc::clone),
            ))
            .child(button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_size(ControlSize::Sm)
                    .with_label(&spec.confirm_label),
                ctx,
                handlers.on_confirm.as_ref().map(Arc::clone),
            ));

        if let Some(ref note) = spec.footer_note {
            // Note grows to fill, actions pinned to the trailing edge
            // (Svelte note `flex: 1 1 18rem` + actions `margin-left: auto`).
            let mut bar = Node::container();
            {
                let s = &mut bar.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.flex_wrap = true;
                s.descriptor.layout.spacing.gap = inline_gap;
                s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
            }
            let mut note_wrap = Node::container();
            {
                let s = &mut note_wrap.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.width = LayoutSizing::Grow;
                s.min_width = Some(0.0);
            }
            Some(
                bar.child(note_wrap.child(label_node(note, text_secondary, desc_font, None)))
                    .child(actions),
            )
        } else {
            Some(actions)
        }
    } else {
        None
    };

    let mut root = picker_shell(
        &spec.as_picker_shell(),
        ctx,
        Some(search),
        selection,
        body,
        None,
        footer,
    );
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}

fn build_search(
    spec: &RelationPickerSpec,
    ctx: &RenderContext<'_>,
    effective_size: ControlSize,
    text_secondary: ColorValue,
    accent: ColorValue,
    label_size: f32,
    handlers: &RelationPickerHandlers,
) -> Node {
    let density = ctx.resolve_density(spec.density);
    let mut col = Node::container();
    col.style.descriptor.layout.direction = LayoutDirection::Column;
    col.style.descriptor.layout.spacing.gap = rem_to_px(0.5);
    let mut col = col;

    if let Some(ref drill_down) = spec.drill_down {
        if !spec.drill_down_path.is_empty() {
            let mut crumbs = Node::container();
            crumbs.style.descriptor.layout.direction = LayoutDirection::Row;
            crumbs.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            crumbs.style.descriptor.layout.spacing.gap = rem_to_px(0.25);
            let mut crumbs = crumbs;

            // Back navigation (handler is host-owned).
            let mut back = Node::button("Back");
            back.id = Some("poodle-relation-drill-back".to_string());
            back.style.descriptor.text_color = Some(text_secondary);
            back.style.text_size = Some(label_size);
            back.interaction.focusable = true;
            if let Some(handler) = &handlers.on_breadcrumb_click {
                let handler = Arc::clone(handler);
                let depth = spec.drill_down_path.len().saturating_sub(1);
                back.interaction.on_activate = Some(Arc::new(move || handler(depth)));
            }
            crumbs = crumbs.child(back);

            for (idx, item_id) in spec.drill_down_path.iter().enumerate() {
                let label = drill_down
                    .levels
                    .get(idx)
                    .and_then(|level| level.items.iter().find(|item| item.id == *item_id))
                    .map(|item| item.label.clone())
                    .unwrap_or_else(|| item_id.clone());
                // Breadcrumb items are accent-colored (Svelte
                // `--poodle-color-accent-base`), with weight 500.
                let mut crumb = Node::button(&label);
                crumb.id = Some(format!("poodle-relation-crumb-{idx}"));
                crumb.style.descriptor.text_color = Some(accent);
                crumb.style.text_size = Some(label_size);
                crumb.style.text_weight = Some(LABEL_WEIGHT);
                crumb.interaction.focusable = true;
                crumbs = crumbs
                    .child(label_node("/", text_secondary, label_size, None))
                    .child(crumb);
            }

            col = col.child(crumbs);
        }

        if !drill_down.is_at_leaf(&spec.drill_down_path) {
            if let Some(level) = drill_down.next_level(&spec.drill_down_path) {
                let mut heading = label_node(
                    &level.label.to_uppercase(),
                    text_secondary,
                    label_size,
                    Some(600),
                );
                heading.style.letter_spacing_em = Some(0.08); // contract drill-level-label
                col = col.child(heading);
            }
        }
    }

    // Real search field — a TextInput type="search" with leading search icon
    // and the current query as its value. Typing/clear are host-owned.
    let mut search_spec = TextInputSpec::new()
        .with_id("relation-picker-search")
        .with_input_type("search")
        .with_leading_icon("search")
        .with_size(effective_size)
        .with_size_role(spec.size_role)
        .with_density(density)
        .with_placeholder(spec.search_placeholder.clone())
        // The panel's search field carries no visible label.
        .with_aria_label("Search relations")
        .with_show_clear_button(true);
    if !spec.query.is_empty() {
        search_spec = search_spec.with_value(spec.query.clone());
    }
    col = col.child(text_input(&search_spec, ctx, None));

    // Toolbar filter controls — one labeled Select per `filters` entry.
    if !spec.filters.is_empty() {
        let mut filters_row = Node::container();
        filters_row.style.descriptor.layout.direction = LayoutDirection::Row;
        filters_row.style.flex_wrap = true;
        filters_row.style.descriptor.layout.spacing.gap =
            rem_to_px(control_space_x_rem(density));
        let mut filters_row = filters_row;
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
                .with_density(density);
            select_spec.aria_label = Some(format!("{} filter", filter.label));
            filters_row =
                filters_row.child(select(&select_spec, ctx, &SelectHandlers::default()));
        }
        col = col.child(filters_row);
    }

    col
}

#[expect(
    clippy::too_many_arguments,
    reason = "drill-row rendering keeps resolved token metrics explicit"
)]
fn drill_row(
    item: &poodle_specs::DrillDownItem,
    text_primary: ColorValue,
    text_secondary: ColorValue,
    radius: f32,
    item_gap: f32,
    item_x: f32,
    item_y: f32,
    title_font: f32,
    label_size: f32,
) -> Node {
    let meta = item
        .count
        .map(|count| format!("{count} items"))
        .unwrap_or_default();

    // Drill button base is transparent (Svelte `.drill-list__button`
    // background: transparent; hover color-mix is host-owned).
    let mut row = Node::button("");
    row.id = Some(format!("poodle-relation-drill-{}", item.id));
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = item_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = item_x;
        pad.right = item_x;
        pad.top = item_y;
        pad.bottom = item_y;
        s.descriptor.background = Some(TRANSPARENT);
    }
    all_radius(&mut row, radius);
    row.interaction.focusable = true;

    let mut copy = Node::container();
    copy.style.descriptor.layout.direction = LayoutDirection::Column;
    copy.style.descriptor.layout.spacing.gap = rem_to_px(0.125);
    let copy = copy
        .child(label_node(
            &item.label,
            text_primary,
            title_font,
            Some(LABEL_WEIGHT),
        ))
        .child(label_node(
            item.description.as_deref().unwrap_or(""),
            text_secondary,
            label_size,
            None,
        ));

    let mut trailing = Node::container();
    trailing.style.descriptor.layout.direction = LayoutDirection::Row;
    trailing.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    trailing.style.descriptor.layout.spacing.gap = rem_to_px(0.25);
    let mut chevron = Node::icon("chevron-right", rem_to_px(0.875));
    chevron.style.descriptor.text_color = Some(text_secondary);
    let trailing = trailing
        .child(label_node(&meta, text_secondary, label_size, None))
        .child(chevron);

    row.child(copy).child(trailing)
}

#[expect(
    clippy::too_many_arguments,
    reason = "candidate rendering keeps picker state and resolved metrics explicit"
)]
fn candidate_row(
    item: &PickerItemSpec,
    is_selected: bool,
    selection_mode: SelectionMode,
    ctx: &RenderContext<'_>,
    text_primary: ColorValue,
    text_secondary: ColorValue,
    border: ColorValue,
    accent: ColorValue,
    surface: ColorValue,
    radius: f32,
    item_gap: f32,
    item_x: f32,
    item_y: f32,
    title_font: f32,
    desc_font: f32,
    spec: &RelationPickerSpec,
) -> Node {
    // Base item bg: color-mix(surface 86%, transparent) (Svelte `.item`).
    let base_bg = mix_srgb(surface, TRANSPARENT, 0.86);
    // Selected bg replaces the base with color-mix(accent 10%, transparent)
    // (contract §8 selected table — a single semi-transparent accent fill).
    let selected_bg = mix_srgb(accent, TRANSPARENT, 0.10);
    let row_bg = if is_selected { selected_bg } else { base_bg };
    // Selected border: color-mix(accent 60%, transparent); else border-subtle.
    let row_border = if is_selected {
        mix_srgb(accent, TRANSPARENT, 0.60)
    } else {
        border
    };

    let mut row = Node::button("");
    row.id = Some(format!("poodle-relation-candidate-{}", item.id));
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = item_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = item_x;
        pad.right = item_x;
        pad.top = item_y;
        pad.bottom = item_y;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = row_border;
        s.descriptor.background = Some(row_bg);
    }
    all_radius(&mut row, radius);
    row.interaction.focusable = true;
    let mut row = row;

    if selection_mode == SelectionMode::Multiple {
        row = row.child(checkbox(
            &CheckboxSpec::new()
                // A selection checkbox has no caption of its own — the row's
                // label sits beside it, not inside it — so without this it is
                // announced as an unnamed checkbox in a list of identical ones.
                .with_aria_label(format!("Select {}", item.label))
                .with_checked(is_selected)
                .with_size(ctx.base_size(spec.size))
                .with_size_role(spec.size_role)
                .with_density(ctx.resolve_density(spec.density)),
            ctx,
            None,
        ));
    }

    let mut copy = Node::container();
    copy.style.descriptor.layout.direction = LayoutDirection::Column;
    copy.style.descriptor.layout.spacing.gap = rem_to_px(0.25);
    copy.style.min_width = Some(0.0);
    let mut copy = copy.child(label_node(
        &item.label,
        text_primary,
        title_font,
        Some(LABEL_WEIGHT),
    ));
    if let Some(description) = item.description.as_ref() {
        copy = copy.child(label_node(description, text_secondary, desc_font, None));
    }
    if let Some(meta) = item.meta.as_ref() {
        copy = copy.child(label_node(meta, text_secondary, desc_font, None));
    }

    row.child(copy)
}
