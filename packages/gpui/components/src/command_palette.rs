//! PugCommandPalette — real GPUI component backed by CommandPaletteSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_workstation::{CommandPaletteSpec, DiscoveryState};

use crate::theme_ext::resolve_color;

/// A real GPUI command palette backed by `CommandPaletteSpec`.
///
/// Renders a searchable list of commands with grouping, shortcuts,
/// badges, and active-item highlighting.
pub struct PugCommandPalette {
    spec: CommandPaletteSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_select: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_query_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl PugCommandPalette {
    pub fn new(spec: CommandPaletteSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-cmd-palette".to_string(),
            on_select: None,
            on_query_change: None,
        }
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }

    pub fn on_query_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_query_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugCommandPalette {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let results_bg = resolve_color(theme, spec.results_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let text_muted = resolve_color(theme, "semantic.color.text.muted");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let mut palette = div()
            .flex()
            .flex_col()
            .w(px(480.0))
            .max_h(px(400.0))
            .rounded(px(8.0))
            .bg(results_bg)
            .border_1()
            .border_color(border)
            .shadow_lg()
            .overflow_hidden();

        // Search input row
        let search_row = div()
            .flex()
            .items_center()
            .px(px(12.0))
            .py(px(8.0))
            .border_b_1()
            .border_color(border)
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(text_primary)
                    .when(spec.query.is_empty(), |el| {
                        el.text_color(text_muted).child("Type a command\u{2026}")
                    })
                    .when(!spec.query.is_empty(), |el| {
                        el.child(spec.query.clone())
                    }),
            );
        palette = palette.child(search_row);

        // Results area
        match spec.state {
            DiscoveryState::Loading => {
                palette = palette.child(
                    div()
                        .p(px(16.0))
                        .text_sm()
                        .text_color(text_secondary)
                        .child("Searching\u{2026}"),
                );
                return palette.into_any_element();
            }
            DiscoveryState::Error => {
                palette = palette.child(
                    div()
                        .p(px(16.0))
                        .text_sm()
                        .text_color(resolve_color(theme, "semantic.color.status.danger"))
                        .child("Error loading commands"),
                );
                return palette.into_any_element();
            }
            DiscoveryState::Empty | DiscoveryState::NoResults => {
                palette = palette.child(
                    div()
                        .p(px(16.0))
                        .text_sm()
                        .text_color(text_secondary)
                        .child("No matching commands"),
                );
                return palette.into_any_element();
            }
            DiscoveryState::Ready => {}
        }

        // Group actions by group name
        let mut results_list = div()
            .flex()
            .flex_col()
            .py(px(4.0))
            .overflow_y_hidden();

        let mut current_group: Option<&str> = None;

        for action in &spec.actions {
            // Render group header if group changed
            if action.group.as_deref() != current_group {
                current_group = action.group.as_deref();
                if let Some(group_name) = current_group {
                    results_list = results_list.child(
                        div()
                            .px(px(12.0))
                            .py(px(4.0))
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_muted)
                            .child(group_name.to_string()),
                    );
                }
            }

            let is_active = spec
                .active_action_id
                .as_deref()
                .map_or(false, |id| id == action.id);
            let action_el_id =
                SharedString::from(format!("{}-{}", self.id_prefix, action.id));

            let mut row = div()
                .id(action_el_id)
                .flex()
                .items_center()
                .justify_between()
                .px(px(12.0))
                .py(px(6.0))
                .mx(px(4.0))
                .rounded(px(4.0))
                .text_sm();

            if is_active {
                row = row.bg(accent.opacity(0.10)).text_color(accent);
            } else {
                row = row.text_color(text_primary);
            }

            if action.is_disabled {
                row = row.opacity(0.48);
            } else {
                row = row
                    .cursor_pointer()
                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.06)));
            }

            // Left: title + badge
            let mut left = div().flex().items_center().gap(px(6.0));
            left = left.child(action.title.clone());

            if let Some(ref badge) = action.badge {
                left = left.child(
                    div()
                        .text_xs()
                        .px(px(4.0))
                        .py(px(1.0))
                        .rounded(px(3.0))
                        .bg(accent.opacity(0.12))
                        .text_color(accent)
                        .child(badge.clone()),
                );
            }

            row = row.child(left);

            // Right: shortcut
            if let Some(ref shortcut) = action.shortcut {
                row = row.child(
                    div()
                        .text_xs()
                        .text_color(text_muted)
                        .child(shortcut.clone()),
                );
            }

            results_list = results_list.child(row);
        }

        palette = palette.child(results_list);

        palette.into_any_element()
    }
}
