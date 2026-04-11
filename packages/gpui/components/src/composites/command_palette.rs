//! CommandPalette — real GPUI component backed by CommandPaletteSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{CommandActionItem, CommandPaletteSpec, DiscoveryState};
use poodle_primitives::{ControlDensity, ControlSize, OverlayPlacement, SemanticControlSizeRole};

use crate::presentation::{resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem, control_space_x_rem, rem_to_px};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI command palette backed by `CommandPaletteSpec`.
///
/// Renders a searchable list of commands with grouping, shortcuts,
/// badges, and active-item highlighting.
pub struct CommandPalette {
    spec: CommandPaletteSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_select: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_query_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for CommandPalette {
    type Target = CommandPaletteSpec;
    fn deref(&self) -> &CommandPaletteSpec { &self.spec }
}

impl CommandPalette {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CommandPaletteSpec::default(), theme: theme.clone(), id_prefix: String::new(), on_select: None, on_query_change: None }
    }

    pub fn from_spec(spec: CommandPaletteSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-cmd-palette".to_string(),
            on_select: None,
            on_query_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn query(mut self, v: impl Into<String>) -> Self { self.spec.query = v.into(); self }
    pub fn actions(mut self, v: Vec<CommandActionItem>) -> Self { self.spec.actions = v; self }
    pub fn state(mut self, v: DiscoveryState) -> Self { self.spec.state = v; self }
    pub fn active_action_id(mut self, v: impl Into<String>) -> Self { self.spec.active_action_id = Some(v.into()); self }
    pub fn placement(mut self, v: OverlayPlacement) -> Self { self.spec.placement = v; self }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }


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

impl IntoElement for CommandPalette {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let panel_px = rem_to_px(panel_space_x_rem(spec.density));
        let _panel_py = rem_to_px(panel_space_y_rem(spec.density));
        let _item_gap = rem_to_px(control_space_x_rem(spec.density));

        let inline_padding = px(panel_px);
        let body_size = px(font_size);

        let results_bg = resolve_color(theme, spec.results_fill_token());
        let border = resolve_color(theme, "color.border.default");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_muted = resolve_color(theme, "color.text.muted");
        let accent = resolve_color(theme, "color.accent.base");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let hover_bg = resolve_color(theme, "color.background.elevated");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        let label_size = resolve_px(theme, "typography.label.size");
        let radius_control = resolve_radius(theme, "radius.control");
        let radius_surface = resolve_radius(theme, "radius.surface");
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_lg = resolve_px(theme, "space.inline.lg");

        let mut palette = div()
            .flex()
            .flex_col()
            .w(px(480.0))
            .max_h(px(400.0))
            .rounded(radius_surface)
            .bg(results_bg)
            .border_1()
            .border_color(border)
            .shadow(vec![
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.12),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(24.0),
                    spread_radius: px(0.0),
                },
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.08),
                    offset: point(px(0.0), px(2.0)),
                    blur_radius: px(8.0),
                    spread_radius: px(0.0),
                },
            ])
            .overflow_hidden();

        // Search input row
        let search_row = div()
            .flex()
            .items_center()
            .px(inline_padding)
            .py(gap_sm)
            .border_b_1()
            .border_color(border)
            .child(
                div()
                    .flex_1()
                    .text_size(body_size)
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
                        .p(gap_lg)
                        .text_size(body_size)
                        .text_color(text_secondary)
                        .child("Searching\u{2026}"),
                );
                return palette.into_any_element();
            }
            DiscoveryState::Error => {
                palette = palette.child(
                    div()
                        .p(gap_lg)
                        .text_size(body_size)
                        .text_color(resolve_color(theme, "color.status.danger"))
                        .child("Error loading commands"),
                );
                return palette.into_any_element();
            }
            DiscoveryState::Empty | DiscoveryState::NoResults => {
                palette = palette.child(
                    div()
                        .p(gap_lg)
                        .text_size(body_size)
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
                            .px(inline_padding)
                            .py(px(4.0))
                            .text_size(label_size)
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
                .focusable()
                .flex()
                .items_center()
                .justify_between()
                .px(inline_padding)
                .py(gap_sm)
                .mx(px(4.0))
                .rounded(radius_control)
                .text_size(body_size);

            if is_active {
                row = row.bg(accent.opacity(0.10)).text_color(accent);
            } else {
                row = row.text_color(text_primary);
            }

            row = row.focus(move |s| s.border_color(focus_ring).shadow(crate::theme_ext::focus_ring_shadow(focus_ring)));

            if action.is_disabled {
                row = row
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                row = row
                    .cursor_pointer()
                    .hover(|s| s.bg(hover_bg));
            }

            // Left: title + badge
            let mut left = div().flex().items_center().gap(gap_sm);
            left = left.child(action.title.clone());

            if let Some(ref badge) = action.badge {
                left = left.child(
                    div()
                        .text_size(label_size)
                        .px(px(4.0))
                        .py(px(1.0))
                        .rounded(radius_control)
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
                        .text_size(label_size)
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
