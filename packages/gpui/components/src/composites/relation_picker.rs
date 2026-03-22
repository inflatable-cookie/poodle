//! RelationPicker — real GPUI component backed by RelationPickerSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::{BrowseState, PickerItemSpec, PickerVariant, RelationPickerSpec, SelectionMode};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI relation picker component backed by `RelationPickerSpec`.
///
/// Renders an entity relationship picker that shows items with selection state,
/// an optional search field, and checkmarks for selected items.
pub struct RelationPicker {
    spec: RelationPickerSpec,
    theme: GpuiThemeProvider,
    on_select: Option<Box<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for RelationPicker {
    type Target = RelationPickerSpec;
    fn deref(&self) -> &RelationPickerSpec { &self.spec }
}

impl RelationPicker {
    pub fn new(items: Vec<PickerItemSpec>, theme: &GpuiThemeProvider) -> Self {
        Self { spec: RelationPickerSpec::new(items), theme: theme.clone(), on_select: None }
    }

    pub fn from_spec(spec: RelationPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_select: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<PickerItemSpec>) -> Self { self.spec.items = v; self }
    pub fn selected_ids(mut self, v: Vec<String>) -> Self { self.spec.selected_ids = v; self }
    pub fn query(mut self, v: impl Into<String>) -> Self { self.spec.query = v.into(); self }
    pub fn selection_mode(mut self, v: SelectionMode) -> Self { self.spec.selection_mode = v; self }
    pub fn variant(mut self, v: PickerVariant) -> Self { self.spec.variant = v; self }
    pub fn state(mut self, v: BrowseState) -> Self { self.spec.state = v; self }


    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl IntoElement for RelationPicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let border = resolve_color(theme, "semantic.color.border.subtle");
        let bg = resolve_color(theme, "semantic.color.background.elevated");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let mut container = div()
            .flex()
            .flex_col()
            .bg(bg)
            .border_1()
            .border_color(border)
            .rounded(control_radius)
            .overflow_hidden();

        // Search area (if query exists)
        if !spec.query.is_empty() {
            container = container.child(
                div()
                    .w_full()
                    .px(inline_padding)
                    .py(px(8.0))
                    .border_b_1()
                    .border_color(border)
                    .text_size(px(14.0))
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
                        .items_center()
                        .justify_center()
                        .py(px(24.0))
                        .child(
                            div()
                                .text_size(px(14.0))
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
                                .text_size(px(14.0))
                                .text_color(resolve_color(theme, "semantic.color.status.danger"))
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
                                .text_size(px(14.0))
                                .text_color(text_secondary)
                                .child("No items found."),
                        ),
                );
            }
            BrowseState::Ready => {
                // Item list
                let mut list = div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .max_h(px(240.0))
                    .overflow_hidden();

                for item in &spec.items {
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
                            .text_size(px(14.0))
                            .text_color(if is_selected { accent } else { text_secondary })
                            .child(check),
                    );

                    // Item content
                    let mut item_content = div().flex().flex_col().gap(px(1.0)).flex_grow();

                    item_content = item_content.child(
                        div()
                            .text_size(px(14.0))
                            .text_color(text_primary)
                            .child(item.label.clone()),
                    );

                    if let Some(ref desc) = item.description {
                        item_content = item_content.child(
                            div()
                                .text_xs()
                                .text_color(text_secondary)
                                .child(desc.clone()),
                        );
                    }

                    item_el = item_el.child(item_content);

                    // Meta
                    if let Some(ref meta) = item.meta {
                        item_el = item_el.child(
                            div()
                                .text_xs()
                                .text_color(text_secondary.opacity(0.7))
                                .child(meta.clone()),
                        );
                    }

                    list = list.child(item_el);
                }

                container = container.child(list);
            }
        }

        // Footer with count
        let count = spec.selected_item_count();
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
                        .text_xs()
                        .text_color(text_secondary)
                        .child(format!(
                            "{} of {} selected",
                            count,
                            spec.items.len()
                        )),
                ),
        );

        container.into_any_element()
    }
}
