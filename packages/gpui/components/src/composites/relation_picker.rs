//! RelationPicker — real GPUI component backed by RelationPickerSpec.

use std::rc::Rc;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    BrowseState, DrillDownItem, PickerItemSpec, PickerVariant, RelationPickerSpec, SelectionMode,
};
use poodle_specs::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::presentation::{resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem, control_space_x_rem, rem_to_px};
use crate::primitives::{Icon, Spinner};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI relation picker component backed by `RelationPickerSpec`.
///
/// Renders an entity relationship picker that shows items with selection state,
/// an optional search field, and checkmarks for selected items.
/// Argument passed to `on_drill_enter` — pairs the zero-based level
/// index with the clicked drill-down item id. Packed into a struct so
/// it can flow through a `cx.listener(|this, arg, w, cx|)` closure.
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
    /// Fired when the user clicks a drill-down row at a non-leaf level.
    on_drill_enter: Option<Rc<dyn Fn(&DrillEnterArgs, &mut Window, &mut App) + 'static>>,
    /// Legacy per-component drill path (pre-contract drill-down support).
    /// Kept so existing callers that use `.with_drill_path(...)` still
    /// render breadcrumbs the same way.
    drill_path: Vec<String>,
}

impl std::ops::Deref for RelationPicker {
    type Target = RelationPickerSpec;
    fn deref(&self) -> &RelationPickerSpec { &self.spec }
}

impl RelationPicker {
    pub fn new(items: Vec<PickerItemSpec>, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: RelationPickerSpec::new(items),
            theme: theme.clone(),
            on_select: None,
            on_breadcrumb_click: None,
            on_drill_enter: None,
            drill_path: Vec::new(),
        }
    }

    pub fn from_spec(spec: RelationPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_select: None,
            on_breadcrumb_click: None,
            on_drill_enter: None,
            drill_path: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<PickerItemSpec>) -> Self { self.spec.items = v; self }
    pub fn selected_ids(mut self, v: Vec<String>) -> Self { self.spec.selected_ids = v; self }
    pub fn query(mut self, v: impl Into<String>) -> Self { self.spec.query = v.into(); self }
    pub fn selection_mode(mut self, v: SelectionMode) -> Self { self.spec.selection_mode = v; self }
    pub fn variant(mut self, v: PickerVariant) -> Self { self.spec.variant = v; self }
    pub fn state(mut self, v: BrowseState) -> Self { self.spec.state = v; self }
    pub fn with_drill_path(mut self, path: Vec<String>) -> Self { self.drill_path = path; self }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }


    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    /// Fired when a breadcrumb segment is clicked. The argument is the
    /// target depth the user wants to return to (0 = root).
    /// Signature matches `cx.listener` so specimen code can pass it
    /// directly.
    pub fn on_breadcrumb_click(
        mut self,
        handler: impl Fn(&usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_breadcrumb_click = Some(Rc::new(handler));
        self
    }

    /// Fired when the user clicks a drill-down row at a non-leaf level.
    /// The handler receives the level index and the clicked item id
    /// packed into a `DrillEnterArgs` so it flows cleanly through
    /// `cx.listener(...)`, and is responsible for pushing the new id
    /// onto `spec.drill_down_path`.
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
        let font_size = rem_to_px(size_font_rem(effective_size));
        let panel_px = rem_to_px(panel_space_x_rem(spec.density));
        let _panel_py = rem_to_px(panel_space_y_rem(spec.density));
        let item_gap = rem_to_px(control_space_x_rem(spec.density));

        let inline_padding = px(panel_px);
        let inline_gap = px(item_gap);
        let body_size = px(font_size);
        let control_radius = resolve_radius(theme, "radius.control");
        let label_size = resolve_px(theme, "typography.label.size");
        let gap_sm = resolve_px(theme, "space.inline.sm");

        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let border = resolve_color(theme, "color.border.subtle");
        let bg = resolve_color(theme, "color.background.elevated");
        let accent = resolve_color(theme, "color.accent.base");

        let mut container = div()
            .flex()
            .flex_col()
            .bg(bg)
            .border_1()
            .border_color(border)
            .rounded(control_radius)
            .overflow_hidden();

        // Drill-down breadcrumb navigation — prefer the spec-backed
        // path when drill-down config is present; otherwise fall back
        // to the legacy component-local `drill_path` field.
        let breadcrumb_segments: Vec<String> = if let Some(ref dd) = spec.drill_down {
            spec.drill_down_path
                .iter()
                .enumerate()
                .map(|(level_idx, item_id)| {
                    let level = dd.levels.get(level_idx);
                    level
                        .and_then(|l| l.items.iter().find(|i| &i.id == item_id))
                        .map(|i| i.label.clone())
                        .unwrap_or_else(|| item_id.clone())
                })
                .collect()
        } else {
            self.drill_path.clone()
        };

        if !breadcrumb_segments.is_empty() {
            let mut breadcrumb_row = div()
                .w_full()
                .flex()
                .items_center()
                .gap(gap_sm)
                .px(inline_padding)
                .py(px(6.0))
                .border_b_1()
                .border_color(border);

            // Root label
            {
                let mut root_el = div()
                    .id("relation-picker-breadcrumb-root")
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .cursor_pointer()
                    .child("Root");
                if let Some(ref handler) = self.on_breadcrumb_click {
                    let handler = handler.clone();
                    root_el = root_el.on_click(move |_ev, win, app| {
                        handler(&0, win, app);
                    });
                }
                breadcrumb_row = breadcrumb_row.child(root_el);
            }

            for (idx, segment) in breadcrumb_segments.iter().enumerate() {
                // Chevron separator
                breadcrumb_row = breadcrumb_row.child(
                    Icon::from_spec(
                        IconSpec::new("chevron-right").with_size(IconSize::Sm),
                        &self.theme,
                    )
                    .with_color(text_secondary.opacity(0.6)),
                );

                // Segment label
                let seg_id = SharedString::from(format!("relation-picker-breadcrumb-{}", idx));
                let mut seg_el = div()
                    .id(seg_id)
                    .text_size(label_size)
                    .text_color(text_primary)
                    .cursor_pointer()
                    .child(segment.clone());
                if let Some(ref handler) = self.on_breadcrumb_click {
                    let handler = handler.clone();
                    let depth = idx + 1;
                    seg_el = seg_el.on_click(move |_ev, win, app| {
                        handler(&depth, win, app);
                    });
                }
                breadcrumb_row = breadcrumb_row.child(seg_el);
            }

            container = container.child(breadcrumb_row);
        }

        // Search area (if query exists)
        if !spec.query.is_empty() {
            container = container.child(
                div()
                    .w_full()
                    .px(inline_padding)
                    .py(px(8.0))
                    .border_b_1()
                    .border_color(border)
                    .text_size(body_size)
                    .text_color(text_primary)
                    .child(format!("Search: {}", spec.query)),
            );
        }

        // State-dependent content
        match spec.state {
            BrowseState::Loading => {
                container = container.child(
                    div()
                        .w_full()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(8.0))
                        .py(px(24.0))
                        .child(
                            Spinner::from_spec(
                                SpinnerSpec::new()
                                    .with_variant(SpinnerVariant::Grid)
                                    .with_size(SpinnerSize::Sm)
                                    .with_tone(SpinnerTone::Accent),
                                theme,
                            ),
                        )
                        .child(
                            div()
                                .text_size(body_size)
                                .text_color(text_secondary)
                                .child("Loading\u{2026}"),
                        ),
                );
            }
            BrowseState::Error => {
                container = container.child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(24.0))
                        .child(
                            div()
                                .text_size(body_size)
                                .text_color(resolve_color(theme, "color.status.danger"))
                                .child("Failed to load items."),
                        ),
                );
            }
            BrowseState::Empty | BrowseState::NoResults => {
                container = container.child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(24.0))
                        .child(
                            div()
                                .text_size(body_size)
                                .text_color(text_secondary)
                                .child("No items found."),
                        ),
                );
            }
            BrowseState::Ready => {
                // Drill-down mode: render either the next level's
                // DrillDownItem rows or the current leaf PickerItemSpecs.
                if let Some(ref dd) = spec.drill_down {
                    if !dd.is_at_leaf(&spec.drill_down_path) {
                        let level_idx = spec.drill_down_path.len();
                        let level_items: &[DrillDownItem] = dd
                            .next_level(&spec.drill_down_path)
                            .map(|l| l.items.as_slice())
                            .unwrap_or(&[]);

                        let mut list = div()
                            .id("relation-picker-drill-list")
                            .w_full()
                            .flex()
                            .flex_col()
                            .max_h(px(240.0))
                            .overflow_y_scroll();

                        for item in level_items {
                            let row_id = SharedString::from(
                                format!("relation-picker-drill-{}-{}", level_idx, item.id),
                            );
                            let mut row = div()
                                .id(row_id)
                                .w_full()
                                .flex()
                                .items_center()
                                .gap(inline_gap)
                                .px(inline_padding)
                                .py(px(8.0))
                                .cursor_pointer()
                                .border_b_1()
                                .border_color(border.opacity(0.3));

                            let mut content = div().flex().flex_col().gap(px(1.0)).flex_grow();
                            content = content.child(
                                div()
                                    .text_size(body_size)
                                    .text_color(text_primary)
                                    .child(item.label.clone()),
                            );
                            if let Some(ref desc) = item.description {
                                content = content.child(
                                    div()
                                        .text_size(label_size)
                                        .text_color(text_secondary)
                                        .child(desc.clone()),
                                );
                            }
                            row = row.child(content);

                            if let Some(count) = item.count {
                                row = row.child(
                                    div()
                                        .text_size(label_size)
                                        .text_color(text_secondary.opacity(0.7))
                                        .child(format!("{} items", count)),
                                );
                            }

                            // Chevron pointing into the next level
                            row = row.child(
                                Icon::from_spec(
                                    IconSpec::new("chevron-right").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(text_secondary),
                            );

                            if let Some(ref handler) = self.on_drill_enter {
                                let handler = handler.clone();
                                let id = item.id.clone();
                                let level_copy = level_idx;
                                row = row.on_click(move |_ev, win, app| {
                                    let args = DrillEnterArgs {
                                        level_index: level_copy,
                                        item_id: id.clone(),
                                    };
                                    handler(&args, win, app);
                                });
                            }

                            list = list.child(row);
                        }

                        container = container.child(list);
                        container = container.child(
                            div()
                                .w_full()
                                .flex()
                                .items_center()
                                .px(inline_padding)
                                .py(px(6.0))
                                .border_t_1()
                                .border_color(border)
                                .child(
                                    div()
                                        .text_size(label_size)
                                        .text_color(text_secondary)
                                        .child(format!(
                                            "{} {}",
                                            level_items.len(),
                                            dd.levels
                                                .get(level_idx)
                                                .map(|l| l.label.to_lowercase())
                                                .unwrap_or_else(|| "items".to_string())
                                        )),
                                ),
                        );
                        return container.into_any_element();
                    }
                    // At a leaf: fall through to the regular item loop
                    // below, but using the leaf group's items instead
                    // of the top-level `spec.items`.
                }

                // Compute the source slice once: leaf items in drill-down
                // mode, otherwise the top-level spec items.
                let source_items: &[PickerItemSpec] = if let Some(ref dd) = spec.drill_down {
                    dd.leaf_items_for(&spec.drill_down_path)
                } else {
                    spec.items.as_slice()
                };

                // Item list
                let mut list = div()
                    .id("relation-picker-list")
                    .w_full()
                    .flex()
                    .flex_col()
                    .max_h(px(240.0))
                    .overflow_y_scroll();

                for item in source_items {
                    let is_selected = spec
                        .selected_ids
                        .iter()
                        .any(|sid| sid == &item.id);

                    let item_id = SharedString::from(format!("relation-picker-{}", item.id));

                    let mut item_el = div()
                        .id(item_id)
                        .w_full()
                        .flex()
                        .items_center()
                        .gap(inline_gap)
                        .px(inline_padding)
                        .py(px(8.0))
                        .cursor_pointer()
                        .border_b_1()
                        .border_color(border.opacity(0.3))
                        .when(is_selected, |el| el.bg(accent.opacity(0.08)));

                    // Selection indicator
                    let check = if is_selected { "\u{2713}" } else { "\u{25CB}" };
                    item_el = item_el.child(
                        div()
                            .text_size(body_size)
                            .text_color(if is_selected { accent } else { text_secondary })
                            .child(check),
                    );

                    // Item content
                    let mut item_content = div().flex().flex_col().gap(px(1.0)).flex_grow();

                    item_content = item_content.child(
                        div()
                            .text_size(body_size)
                            .text_color(text_primary)
                            .child(item.label.clone()),
                    );

                    if let Some(ref desc) = item.description {
                        item_content = item_content.child(
                            div()
                                .text_size(label_size)
                                .text_color(text_secondary)
                                .child(desc.clone()),
                        );
                    }

                    item_el = item_el.child(item_content);

                    // Meta
                    if let Some(ref meta) = item.meta {
                        item_el = item_el.child(
                            div()
                                .text_size(label_size)
                                .text_color(text_secondary.opacity(0.7))
                                .child(meta.clone()),
                        );
                    }

                    if let Some(ref handler) = self.on_select {
                        let handler = handler.clone();
                        let id = item.id.clone();
                        item_el = item_el.on_click(move |ev, win, app| {
                            handler(&id, ev, win, app);
                        });
                    }

                    list = list.child(item_el);
                }

                container = container.child(list);
            }
        }

        // Footer with count — use leaf items when drilled all the
        // way in, or the top-level items otherwise.
        let count = spec.selected_item_count();
        let total_for_footer = if let Some(ref dd) = spec.drill_down {
            if dd.is_at_leaf(&spec.drill_down_path) {
                dd.leaf_items_for(&spec.drill_down_path).len()
            } else {
                spec.items.len()
            }
        } else {
            spec.items.len()
        };
        container = container.child(
            div()
                .w_full()
                .flex()
                .items_center()
                .px(inline_padding)
                .py(px(6.0))
                .border_t_1()
                .border_color(border)
                .child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(format!("{} of {} selected", count, total_for_footer)),
                ),
        );

        container.into_any_element()
    }
}
