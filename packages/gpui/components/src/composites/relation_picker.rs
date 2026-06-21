//! RelationPicker — GPUI relation picker backed by RelationPickerSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    BrowseState, ButtonVariant, CheckboxSpec, ControlDensity, ControlSize, DrillDownItem,
    PickerItemSpec, PickerVariant, RelationPickerSpec, SelectionMode, SemanticControlSizeRole,
};
use std::rc::Rc;

use super::{PickerShell, SelectionSummary};
use crate::presentation::{
    control_space_x_rem, rem_to_px, relation_picker_desc_size_rem, relation_picker_item_gap_rem,
    relation_picker_item_x_rem, relation_picker_item_y_rem, relation_picker_list_gap_rem,
    relation_picker_title_size_rem, resolve_semantic_size,
};
use crate::primitives::{Button, Checkbox, Icon, Select, TextInput};
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// Candidate / drill copy strong label weight (Svelte `strong { font-weight: 500 }`).
/// FontWeight::MEDIUM == 500.
const LABEL_WEIGHT: FontWeight = FontWeight::MEDIUM;

#[derive(Clone, Debug)]
pub struct DrillEnterArgs {
    pub level_index: usize,
    pub item_id: String,
}

pub struct RelationPicker {
    spec: RelationPickerSpec,
    theme: GpuiThemeProvider,
    on_select: Option<Rc<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
    on_breadcrumb_click: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
    on_drill_enter: Option<Rc<dyn Fn(&DrillEnterArgs, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for RelationPicker {
    type Target = RelationPickerSpec;
    fn deref(&self) -> &RelationPickerSpec {
        &self.spec
    }
}

impl RelationPicker {
    pub fn new(items: Vec<PickerItemSpec>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(RelationPickerSpec::new(items), theme)
    }

    pub fn from_spec(spec: RelationPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_select: None,
            on_breadcrumb_click: None,
            on_drill_enter: None,
        }
    }

    pub fn items(mut self, v: Vec<PickerItemSpec>) -> Self {
        self.spec.items = v;
        self
    }
    pub fn selected_ids(mut self, v: Vec<String>) -> Self {
        self.spec.selected_ids = v;
        self
    }
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.spec.query = v.into();
        self
    }
    pub fn selection_mode(mut self, v: SelectionMode) -> Self {
        self.spec.selection_mode = v;
        self
    }
    pub fn variant(mut self, v: PickerVariant) -> Self {
        self.spec.variant = v;
        self
    }
    pub fn state(mut self, v: BrowseState) -> Self {
        self.spec.state = v;
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    pub fn on_breadcrumb_click(
        mut self,
        handler: impl Fn(&usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_breadcrumb_click = Some(Rc::new(handler));
        self
    }

    pub fn on_drill_enter(
        mut self,
        handler: impl Fn(&DrillEnterArgs, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_drill_enter = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for RelationPicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let label_size = resolve_px(theme, "typography.label.size");
        // Toolbar/footer inline gap (density-driven).
        let gap = px(rem_to_px(control_space_x_rem(spec.density)));
        // Candidate/drill geometry from size + density tokens (contract §8).
        let row_gap = px(rem_to_px(relation_picker_list_gap_rem(spec.density)));
        let item_x = px(rem_to_px(relation_picker_item_x_rem(effective_size)));
        let item_y = px(rem_to_px(relation_picker_item_y_rem(effective_size)));
        let item_gap = px(rem_to_px(relation_picker_item_gap_rem(effective_size)));
        let title_font = px(rem_to_px(relation_picker_title_size_rem(effective_size)));
        let desc_font = px(rem_to_px(relation_picker_desc_size_rem(effective_size)));
        let border = resolve_color(theme, "color.border.subtle");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let surface = resolve_color(theme, "color.background.surface");
        let accent = resolve_color(theme, "color.accent.base");
        let radius = resolve_radius(theme, "radius.control");

        let mut search_col = div().flex().flex_col().gap(gap);

        if let Some(ref drill_down) = spec.drill_down {
            if !spec.drill_down_path.is_empty() {
                let mut crumbs = div().flex().items_center().gap(px(rem_to_px(0.25)));

                let mut back = Button::from_spec(
                    poodle_specs::ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Sm)
                        .with_leading_icon("chevron-left")
                        .with_label("Back"),
                    theme,
                );
                if let Some(ref handler) = self.on_breadcrumb_click {
                    let handler = handler.clone();
                    let depth = spec.drill_down_path.len().saturating_sub(1);
                    back = back.on_click(move |_event, window, cx| handler(&depth, window, cx));
                }
                crumbs = crumbs.child(back);

                for (idx, item_id) in spec.drill_down_path.iter().enumerate() {
                    let label = drill_down
                        .levels
                        .get(idx)
                        .and_then(|level| level.items.iter().find(|item| item.id == *item_id))
                        .map(|item| item.label.clone())
                        .unwrap_or_else(|| item_id.clone());

                    crumbs = crumbs
                        .child(
                            div()
                                .text_size(label_size)
                                .text_color(text_secondary)
                                .child("/"),
                        )
                        .child(div().text_size(label_size).text_color(accent).child(label));
                }

                search_col = search_col.child(crumbs);
            }

            if !drill_down.is_at_leaf(&spec.drill_down_path) {
                if let Some(level) = drill_down.next_level(&spec.drill_down_path) {
                    search_col = search_col.child(
                        div()
                            .text_size(label_size)
                            .text_color(text_secondary)
                            .child(level.label.to_uppercase()),
                    );
                }
            }
        }

        // Real search field — TextInput type="search" with a leading search
        // icon and clear button, current query as value. Typing/clear are
        // owned by the consumer's event loop (render-only here).
        let mut search_spec = poodle_specs::TextInputSpec::new()
            .with_id("relation-picker-search")
            .with_input_type("search")
            .with_leading_icon("search")
            .with_placeholder(spec.search_placeholder.clone())
            .with_show_clear_button(true);
        search_spec.size = effective_size;
        search_spec.size_role = spec.size_role;
        search_spec.density = spec.density;
        if !spec.query.is_empty() {
            search_spec = search_spec.with_value(spec.query.clone());
        }
        search_col =
            search_col.child(div().w_full().child(TextInput::from_spec(search_spec, theme)));

        // Toolbar filter controls — one labeled Select per `filters` entry
        // (Svelte `.poodle-relation-picker__filters`). Value change is
        // consumer-owned (render-only here).
        if !spec.filters.is_empty() {
            let mut filters_row = div()
                .flex()
                .flex_wrap()
                .gap(px(rem_to_px(control_space_x_rem(spec.density))));
            for filter in &spec.filters {
                let options = filter
                    .resolved_options()
                    .into_iter()
                    .map(|(value, label)| poodle_specs::ChoiceOption::new(value, label))
                    .collect::<Vec<_>>();
                let mut select_spec = poodle_specs::SelectSpec::new(options)
                    .with_value(spec.filter_value(&filter.key).to_string())
                    .with_size(effective_size)
                    .with_size_role(spec.size_role)
                    .with_density(spec.density);
                select_spec.aria_label = Some(format!("{} filter", filter.label));
                filters_row = filters_row.child(Select::from_spec(select_spec, theme));
            }
            search_col = search_col.child(filters_row);
        }

        let selection_items = spec.selection_summary_items();
        let selection_el = if spec.show_selection_summary && !selection_items.is_empty() {
            Some(
                SelectionSummary::from_spec(
                    poodle_specs::SelectionSummarySpec::new(selection_items)
                        .with_clear_action(poodle_specs::RemediationAction::new("clear", "Clear")),
                    theme,
                )
                .with_size(spec.size)
                .with_size_role(spec.size_role)
                .with_density(spec.density)
                .into_any_element(),
            )
        } else {
            None
        };

        let content = if spec.state != BrowseState::Ready {
            None
        } else if spec.drill_down.is_some()
            && spec
                .drill_down
                .as_ref()
                .map(|dd| !dd.is_at_leaf(&spec.drill_down_path))
                .unwrap_or(false)
        {
            let drill_items = spec.drill_items();
            let mut list = div().flex().flex_col().gap(row_gap);
            for item in drill_items {
                let mut row = drill_row(
                    &item,
                    theme,
                    title_font,
                    label_size,
                    item_gap,
                    item_x,
                    item_y,
                    text_primary,
                    text_secondary,
                    radius,
                );
                if let Some(ref handler) = self.on_drill_enter {
                    let handler = handler.clone();
                    let args = DrillEnterArgs {
                        level_index: spec.drill_down_path.len(),
                        item_id: item.id.clone(),
                    };
                    row = row.on_click(move |_event, window, cx| handler(&args, window, cx));
                }
                list = list.child(row);
            }
            Some(list.into_any_element())
        } else {
            let mut list = div().flex().flex_col().gap(row_gap);
            for item in spec.current_items() {
                let is_selected = spec
                    .selected_ids
                    .iter()
                    .any(|selected| selected == &item.id);
                let mut row = candidate_row(
                    &item,
                    is_selected,
                    spec.selection_mode,
                    theme,
                    title_font,
                    desc_font,
                    item_gap,
                    item_x,
                    item_y,
                    border,
                    text_primary,
                    text_secondary,
                    surface,
                    accent,
                    radius,
                    spec.size,
                    spec.size_role,
                    spec.density,
                );
                if let Some(ref handler) = self.on_select {
                    let handler = handler.clone();
                    let item_id = item.id.clone();
                    row =
                        row.on_click(move |event, window, cx| handler(&item_id, event, window, cx));
                }
                list = list.child(row);
            }
            Some(list.into_any_element())
        };

        // Footer (FormActions): optional footer note (Svelte `footerNote`) +
        // the cancel/confirm action row. Gated on `show_footer`.
        let footer_el = if spec.show_footer {
            let footer_actions = div()
                .flex()
                .items_center()
                .gap(gap)
                .child(Button::from_spec(
                    poodle_specs::ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Sm)
                        .with_label(spec.cancel_label.clone()),
                    theme,
                ))
                .child(Button::from_spec(
                    poodle_specs::ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_size(ControlSize::Sm)
                        .with_label(spec.confirm_label.clone()),
                    theme,
                ));

            let mut footer = div().flex().items_center().flex_wrap().gap(gap).w_full();
            if let Some(ref note) = spec.footer_note {
                footer = footer.child(
                    div()
                        .flex_grow()
                        .min_w_0()
                        .text_size(desc_font)
                        .text_color(text_secondary)
                        .child(note.clone()),
                );
            }
            // Push actions to the trailing edge (Svelte `margin-left: auto`).
            Some(footer.child(div().ml_auto().child(footer_actions)))
        } else {
            None
        };

        let mut shell =
            PickerShell::from_spec(spec.as_picker_shell(), theme).with_toolbar(search_col);
        if let Some(footer) = footer_el {
            shell = shell.with_footer(footer);
        }
        if let Some(selection) = selection_el {
            shell = shell.with_selection(selection);
        }
        if let Some(results) = content {
            shell = shell.with_body(results);
        }
        shell.into_any_element()
    }
}

#[allow(clippy::too_many_arguments)]
fn drill_row(
    item: &DrillDownItem,
    theme: &GpuiThemeProvider,
    title_font: Pixels,
    label_size: Pixels,
    item_gap: Pixels,
    item_x: Pixels,
    item_y: Pixels,
    text_primary: Hsla,
    text_secondary: Hsla,
    radius: Pixels,
) -> Stateful<Div> {
    let meta = item
        .count
        .map(|count| format!("{count} items"))
        .unwrap_or_default();

    // Drill button base is transparent (Svelte `.drill-list__button`
    // background: transparent; hover color-mix(surface 60%) is consumer-owned).
    div()
        .id(SharedString::from(format!("poodle-drill-row-{}", item.id)))
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .gap(item_gap)
        .px(item_x)
        .py(item_y)
        .rounded(radius)
        .bg(gpui::transparent_black())
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(rem_to_px(0.125)))
                .child(
                    div()
                        .text_size(title_font)
                        .font_weight(LABEL_WEIGHT)
                        .text_color(text_primary)
                        .child(item.label.clone()),
                )
                .children(item.description.clone().map(|description| {
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(description)
                })),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(rem_to_px(0.25)))
                .children((!meta.is_empty()).then(|| {
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(meta)
                }))
                .child(
                    Icon::from_spec(
                        poodle_specs::IconSpec::new("chevron-right")
                            .with_size(poodle_specs::IconSize::Sm),
                        theme,
                    )
                    .with_color(text_secondary),
                ),
        )
}

#[allow(clippy::too_many_arguments)]
fn candidate_row(
    item: &PickerItemSpec,
    is_selected: bool,
    selection_mode: SelectionMode,
    theme: &GpuiThemeProvider,
    title_font: Pixels,
    desc_font: Pixels,
    item_gap: Pixels,
    item_x: Pixels,
    item_y: Pixels,
    border: Hsla,
    text_primary: Hsla,
    text_secondary: Hsla,
    surface: Hsla,
    accent: Hsla,
    radius: Pixels,
    size: ControlSize,
    size_role: SemanticControlSizeRole,
    density: ControlDensity,
) -> Stateful<Div> {
    let transparent = gpui::transparent_black();
    // Base item bg: color-mix(surface 86%, transparent) (Svelte `.item`).
    let base_bg = color_mix(surface, transparent, 0.86);
    // Selected bg replaces the base with color-mix(accent 10%, transparent)
    // (contract §8 selected table — a single semi-transparent accent fill).
    let row_bg = if is_selected {
        color_mix(accent, transparent, 0.10)
    } else {
        base_bg
    };
    // Selected border: color-mix(accent 60%, transparent); else border-subtle.
    let row_border = if is_selected {
        color_mix(accent, transparent, 0.60)
    } else {
        border
    };

    let mut row = div()
        .id(SharedString::from(format!("poodle-picker-row-{}", item.id)))
        .w_full()
        .flex()
        .items_center()
        .gap(item_gap)
        .px(item_x)
        .py(item_y)
        .rounded(radius)
        .border_1()
        .border_color(row_border)
        .bg(row_bg);

    if selection_mode == SelectionMode::Multiple {
        row = row.child(Checkbox::from_spec(
            CheckboxSpec::new()
                .with_checked(is_selected)
                .with_size(size)
                .with_size_role(size_role)
                .with_density(density),
            theme,
        ));
    }

    row.child(
        div()
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(rem_to_px(0.25)))
            .child(
                div()
                    .text_size(title_font)
                    .font_weight(LABEL_WEIGHT)
                    .text_color(text_primary)
                    .child(item.label.clone()),
            )
            .children(item.description.clone().map(|description| {
                div()
                    .text_size(desc_font)
                    .text_color(text_secondary)
                    .child(description)
            }))
            .children(item.meta.clone().map(|meta| {
                div()
                    .text_size(desc_font)
                    .text_color(text_secondary)
                    .child(meta)
            })),
    )
}
