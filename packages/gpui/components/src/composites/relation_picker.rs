//! RelationPicker — GPUI relation picker backed by RelationPickerSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    BrowseState, ButtonVariant, CheckboxSpec, ControlDensity, ControlSize, DrillDownItem,
    PickerItemSpec, PickerVariant, RelationPickerSpec, SelectionMode, SemanticControlSizeRole,
};
use std::rc::Rc;

use super::{PickerShell, SelectionSummary};
use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::primitives::{Button, Checkbox, Icon};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

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
        let body_size = px(rem_to_px(size_font_rem(effective_size)));
        let label_size = resolve_px(theme, "typography.label.size");
        let gap = px(rem_to_px(control_space_x_rem(spec.density)));
        let row_gap = px(rem_to_px(0.125));
        let pad_x = px(rem_to_px(0.75));
        let pad_y = px(rem_to_px(0.5));
        let border = resolve_color(theme, "color.border.subtle");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let surface = resolve_color(theme, "color.background.surface");
        let elevated = resolve_color(theme, "color.background.elevated");
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

        search_col = search_col.child(
            div()
                .w_full()
                .px(pad_x)
                .py(pad_y)
                .rounded(radius)
                .border_1()
                .border_color(border)
                .bg(surface)
                .flex()
                .items_center()
                .gap(gap)
                .child(
                    Icon::from_spec(
                        poodle_specs::IconSpec::new("search").with_size(poodle_specs::IconSize::Sm),
                        theme,
                    )
                    .with_color(text_secondary),
                )
                .child(
                    div()
                        .text_size(body_size)
                        .text_color(if spec.query.is_empty() {
                            text_secondary
                        } else {
                            text_primary
                        })
                        .child(if spec.query.is_empty() {
                            "Search…".to_string()
                        } else {
                            spec.query.clone()
                        }),
                ),
        );

        let selection_items = spec.selection_summary_items();
        let selection_el = if !selection_items.is_empty() {
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
                    body_size,
                    label_size,
                    gap,
                    pad_x,
                    pad_y,
                    border,
                    text_primary,
                    text_secondary,
                    surface,
                    elevated,
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
                    body_size,
                    label_size,
                    gap,
                    pad_x,
                    pad_y,
                    border,
                    text_primary,
                    text_secondary,
                    surface,
                    elevated,
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

        let mut shell = PickerShell::from_spec(spec.as_picker_shell(), theme)
            .with_toolbar(search_col)
            .with_footer(footer_actions);
        if let Some(selection) = selection_el {
            shell = shell.with_selection(selection);
        }
        if let Some(results) = content {
            shell = shell.with_body(results);
        }
        shell.into_any_element()
    }
}

fn drill_row(
    item: &DrillDownItem,
    theme: &GpuiThemeProvider,
    body_size: Pixels,
    label_size: Pixels,
    gap: Pixels,
    pad_x: Pixels,
    pad_y: Pixels,
    border: Hsla,
    text_primary: Hsla,
    text_secondary: Hsla,
    surface: Hsla,
    elevated: Hsla,
    radius: Pixels,
) -> Stateful<Div> {
    let meta = item
        .count
        .map(|count| format!("{count} items"))
        .unwrap_or_default();

    div()
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .gap(gap)
        .px(pad_x)
        .py(pad_y)
        .rounded(radius)
        .border_1()
        .border_color(border)
        .bg(Hsla {
            a: surface.a * 0.6 + elevated.a * 0.4,
            ..elevated
        })
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(rem_to_px(0.125)))
                .child(
                    div()
                        .text_size(body_size)
                        .font_weight(FontWeight::SEMIBOLD)
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
                .gap(px(rem_to_px(0.5)))
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
    body_size: Pixels,
    label_size: Pixels,
    gap: Pixels,
    pad_x: Pixels,
    pad_y: Pixels,
    border: Hsla,
    text_primary: Hsla,
    text_secondary: Hsla,
    surface: Hsla,
    elevated: Hsla,
    accent: Hsla,
    radius: Pixels,
    size: ControlSize,
    size_role: SemanticControlSizeRole,
    density: ControlDensity,
) -> Stateful<Div> {
    let row_bg = if is_selected {
        Hsla {
            a: accent.a * 0.10 + surface.a * 0.90,
            ..surface
        }
    } else {
        Hsla {
            a: surface.a * 0.6 + elevated.a * 0.4,
            ..elevated
        }
    };
    let row_border = if is_selected { accent } else { border };

    let mut row = div()
        .w_full()
        .flex()
        .items_center()
        .gap(gap)
        .px(pad_x)
        .py(pad_y)
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
            .gap(px(rem_to_px(0.125)))
            .child(
                div()
                    .text_size(body_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(item.label.clone()),
            )
            .children(item.description.clone().map(|description| {
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(description)
            }))
            .children(item.meta.clone().map(|meta| {
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(meta)
            })),
    )
}
