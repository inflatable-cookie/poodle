//! CommandPalette — real GPUI component backed by CommandPaletteSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CommandActionItem, CommandPaletteSpec, DiscoveryState};
use poodle_specs::{ControlDensity, ControlSize, OverlayPlacement, SemanticControlSizeRole};

use crate::primitives::{Icon, TextInput};
use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
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
    /// Fired when the palette requests to close (Escape, backdrop
    /// click, or close button). Matches Svelte `onOpenChange(false)`.
    on_open_change: Option<std::rc::Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for CommandPalette {
    type Target = CommandPaletteSpec;
    fn deref(&self) -> &CommandPaletteSpec {
        &self.spec
    }
}

impl CommandPalette {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CommandPaletteSpec::default(),
            theme: theme.clone(),
            id_prefix: String::new(),
            on_select: None,
            on_query_change: None,
            on_open_change: None,
        }
    }

    pub fn from_spec(spec: CommandPaletteSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-cmd-palette".to_string(),
            on_select: None,
            on_query_change: None,
            on_open_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.spec.query = v.into();
        self
    }
    pub fn actions(mut self, v: Vec<CommandActionItem>) -> Self {
        self.spec.actions = v;
        self
    }
    pub fn state(mut self, v: DiscoveryState) -> Self {
        self.spec.state = v;
        self
    }
    pub fn active_action_id(mut self, v: impl Into<String>) -> Self {
        self.spec.active_action_id = Some(v.into());
        self
    }
    pub fn placement(mut self, v: OverlayPlacement) -> Self {
        self.spec.placement = v;
        self
    }
    pub fn open(mut self, v: bool) -> Self {
        self.spec.is_open = v;
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = Some(v.into());
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn invocation_hint(mut self, v: impl Into<String>) -> Self {
        self.spec.invocation_hint = Some(v.into());
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

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
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

    /// Fired when the palette requests to close — Escape, backdrop
    /// click, or the close button. Mirrors Svelte `onOpenChange(false)`.
    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for CommandPalette {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        // Closed: render nothing. Consumers that skip the `is_open`
        // field still render because the default is true.
        if !self.spec.is_open {
            return div().into_any_element();
        }


        // Rc so every result-row closure can hold a clone.
        let on_select_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_select.map(|h| std::rc::Rc::from(h));

        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let panel_px = px(rem_to_px(panel_space_x_rem(spec.density)));
        let panel_py = px(rem_to_px(panel_space_y_rem(spec.density)));

        let body_size = px(font_size);

        // ── Color / dimension tokens ──────────────────────────────
        // Modal surface fill = color.background.elevated (contract §9
        // mixes it 98% with transparent; resolve straight for parity
        // since GPUI has no transparent-mix surface here).
        let surface_bg = resolve_color(theme, spec.results_fill_token());
        let border_default = resolve_color(theme, "color.border.default");
        // Contract §9 dialog border = border-default mixed 42% with
        // transparent → reduce alpha to 0.42 of the base.
        let dialog_border = Hsla {
            a: border_default.a * 0.42,
            ..border_default
        };
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_muted = resolve_color(theme, "color.text.muted");
        let accent = resolve_color(theme, "color.accent.base");
        let surface_subtle = resolve_color(theme, "color.background.surface");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let hover_bg = resolve_color(theme, "color.background.elevated");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        // Scrim: contract §9 overlay background = black 44%. No token
        // matches that literal exactly, so use the semantic overlay
        // scrim color (color.background.overlay). See parity note.
        let scrim = resolve_color(theme, "color.background.overlay");

        let label_size = resolve_px(theme, "typography.label.size");
        let heading_size = resolve_px(theme, "typography.heading.size");
        let radius_control = resolve_radius(theme, "radius.control");
        let radius_surface = resolve_radius(theme, "radius.surface");
        // Contract §9 dialog radius = radius.surface + 0.125rem.
        let dialog_radius = radius_surface + px(rem_to_px(0.125));
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_md = resolve_px(theme, "space.inline.md");
        let gap_lg = resolve_px(theme, "space.inline.lg");
        let stack_md = resolve_px(theme, "space.stack.md");
        // Contract §9 close-button geometry (1.75rem square, control
        // radius − 0.0625rem). Hint pill min-height 1.5rem, x-pad
        // 0.5rem (contract §9 .command-palette__hint).
        let close_dim = px(rem_to_px(1.75));
        let close_radius = radius_control - px(rem_to_px(0.0625));
        let hint_min_h = px(rem_to_px(1.5));
        let hint_pad_x = px(rem_to_px(0.5));

        // ── Modal surface ─────────────────────────────────────────
        // Centered dialog: grid-equivalent vertical stack of header /
        // query / status / results, gap = space.stack.md (contract §9).
        let mut modal = div()
            .flex()
            .flex_col()
            .gap(stack_md)
            // Contract width: min(45rem, calc(100vw − 2rem)). GPUI has
            // no min()/vw; pin the rem cap and let max_w_full clamp to
            // the available container.
            .w(px(rem_to_px(45.0)))
            .max_w_full()
            // Contract max-height: min(78vh, 52.5rem). Pin the rem cap.
            .max_h(px(rem_to_px(52.5)))
            .px(panel_px)
            .py(panel_py)
            .rounded(dialog_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(dialog_border)
            .shadow(crate::theme_ext::elevation_dialog_shadow())
            .overflow_hidden()
            .occlude();

        // ── Header: title-group + meta (hint pill + close) ────────
        // Rendered whenever a title, description, or invocation hint is
        // present, or a close affordance is wanted (always, per §3).
        let mut title_group = div().flex().flex_col().gap(px(rem_to_px(0.125)));
        if let Some(ref title) = spec.title {
            title_group = title_group.child(
                div()
                    .text_size(heading_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }
        if let Some(ref description) = spec.description {
            title_group = title_group.child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        // Meta column: optional invocation-hint pill + close button.
        let mut meta = div().flex().items_center().gap(gap_sm);
        if let Some(ref hint) = spec.invocation_hint {
            meta = meta.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .min_h(hint_min_h)
                    .px(hint_pad_x)
                    .rounded(radius_control)
                    // Contract §9 hint bg = background.surface 76%.
                    .bg(Hsla {
                        a: surface_subtle.a * 0.76,
                        ..surface_subtle
                    })
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(hint.clone()),
            );
        }
        // Close button (contract §3 / §9): transparent, control-sized
        // square with an `x` icon. Click forwards onOpenChange(false).
        let close_id = SharedString::from(format!("{}-close", self.id_prefix));
        let close_hover_bg = Hsla {
            a: 0.08,
            ..text_secondary
        };
        let mut close_btn = div()
            .id(close_id)
            .flex()
            .items_center()
            .justify_center()
            .w(close_dim)
            .h(close_dim)
            .flex_shrink_0()
            .rounded(close_radius)
            .cursor_pointer()
            .hover(move |s| s.bg(close_hover_bg))
            .child(Icon::new("x", theme).with_color(text_secondary));
        if let Some(ref handler) = self.on_open_change {
            let handler = handler.clone();
            close_btn = close_btn.on_click(move |_event, window, cx| {
                handler(false, window, cx);
            });
        }
        meta = meta.child(close_btn);

        let header = div()
            .flex()
            .items_start()
            .justify_between()
            .gap(gap_md)
            .child(div().flex_1().min_w_0().child(title_group))
            .child(meta);
        modal = modal.child(header);

        // ── Query: real TextInput type="search" ───────────────────
        // Renders current query value + leading search icon and forwards the
        // composed input's live editing (character/backspace key handling is
        // TextInput's) as on_query_change. Caret-at-end editing only — full
        // IME/selection still waits on gpui's EntityInputHandler path.
        let placeholder = "Search commands, panels, and actions".to_string();
        let mut query_input = TextInput::new(theme)
            .with_id(format!("{}-query", self.id_prefix))
            .input_type("search")
            .leading_icon("search")
            .value(spec.query.clone())
            .placeholder(placeholder)
            .aria_label("Search commands")
            .size(effective_size)
            .density(spec.density);
        if let Some(handler) = self.on_query_change {
            query_input = query_input.on_change(move |value, window, cx| {
                handler(value, window, cx);
            });
        }
        modal = modal.child(div().min_w_0().child(query_input));

        // ── Status region (contract §3 / §7) ──────────────────────
        // Live count / active / state message. ARIA role=status is an
        // accepted GPUI limit (no a11y channel).
        let enabled: Vec<&CommandActionItem> =
            spec.actions.iter().filter(|a| !a.is_disabled).collect();
        let active_title = spec
            .active_action_id
            .as_deref()
            .and_then(|id| spec.actions.iter().find(|a| a.id == id))
            .map(|a| a.title.clone());
        let status_text = match spec.state {
            DiscoveryState::Loading => "Loading commands.".to_string(),
            DiscoveryState::Error => "Command palette unavailable.".to_string(),
            DiscoveryState::Empty => "No commands are available in this workspace.".to_string(),
            DiscoveryState::NoResults => {
                format!("No commands match \"{}\".", spec.query)
            }
            DiscoveryState::Ready => {
                let count = enabled.len();
                let plural = if count == 1 { "" } else { "s" };
                let active_suffix = active_title
                    .as_ref()
                    .map(|t| format!(" Active command: {}.", t))
                    .unwrap_or_default();
                format!("{count} command{plural} available.{active_suffix}")
            }
        };
        modal = modal.child(
            div()
                // Contract §9 status: 0.75rem font, text-secondary.
                .text_size(px(rem_to_px(0.75)))
                .text_color(text_secondary)
                .child(status_text),
        );

        // ── Results panel ─────────────────────────────────────────
        // Non-ready postures render an empty-state message in place of
        // the list; ready renders grouped rows.
        let results: AnyElement = match spec.state {
            DiscoveryState::Loading => div()
                .py(gap_lg)
                .text_size(body_size)
                .text_color(text_secondary)
                .child("Searching\u{2026}")
                .into_any_element(),
            DiscoveryState::Error => div()
                .py(gap_lg)
                .text_size(body_size)
                .text_color(resolve_color(theme, "color.status.danger"))
                .child("Error loading commands")
                .into_any_element(),
            DiscoveryState::Empty | DiscoveryState::NoResults => div()
                .py(gap_lg)
                .text_size(body_size)
                .text_color(text_secondary)
                .child("No matching commands")
                .into_any_element(),
            DiscoveryState::Ready => {
                let mut results_list = div()
                    .flex()
                    .flex_col()
                    .gap(px(rem_to_px(0.125)))
                    .overflow_y_hidden();

                let mut current_group: Option<&str> = None;

                for action in &spec.actions {
                    // Render group header if the group changed.
                    if action.group.as_deref() != current_group {
                        current_group = action.group.as_deref();
                        if let Some(group_name) = current_group {
                            results_list = results_list.child(
                                div()
                                    .px(gap_sm)
                                    .py(px(rem_to_px(0.25)))
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
                        .px(gap_sm)
                        .py(gap_sm)
                        .rounded(radius_control)
                        .text_size(body_size);

                    if is_active {
                        row = row.bg(accent.opacity(0.10)).text_color(accent);
                    } else {
                        row = row.text_color(text_primary);
                    }

                    row = row.focus(move |s| {
                        s.border_color(focus_ring)
                            .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
                    });

                    if action.is_disabled {
                        row = row
                            .opacity(disabled_opacity)
                            .cursor(CursorStyle::OperationNotAllowed);
                    } else {
                        row = row.cursor_pointer().hover(|s| s.bg(hover_bg));

                        if let Some(handler) = &on_select_rc {
                            let handler = handler.clone();
                            let id = action.id.clone();
                            row = row.on_click(move |_event, window, cx| {
                                handler(&id, window, cx);
                            });
                        }
                    }

                    // Left: title + optional badge.
                    let mut left = div().flex().items_center().gap(gap_sm);
                    left = left.child(action.title.clone());
                    if let Some(ref badge) = action.badge {
                        left = left.child(
                            div()
                                .text_size(label_size)
                                .px(px(rem_to_px(0.25)))
                                .py(px(rem_to_px(0.0625)))
                                .rounded(radius_control)
                                .bg(accent.opacity(0.12))
                                .text_color(accent)
                                .child(badge.clone()),
                        );
                    }
                    row = row.child(left);

                    // Right: optional shortcut hint.
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
                results_list.into_any_element()
            }
        };
        modal = modal.child(results);

        // ── Overlay backdrop ──────────────────────────────────────
        // Full-area scrim centering the modal. GPUI has no `fixed`/`vw`
        // or z-index; `absolute inset_0` fills the positioned ancestor
        // and paint order layers it above siblings. Backdrop click and
        // Escape forward onOpenChange(false).
        let mut backdrop = div()
            .id(SharedString::from(format!("{}-overlay", self.id_prefix)))
            .absolute()
            .inset_0()
            .bg(scrim)
            .flex()
            .items_center()
            .justify_center()
            .occlude()
            .child(modal);

        if let Some(ref handler) = self.on_open_change {
            let click_handler = handler.clone();
            backdrop = backdrop.on_click(move |_event, window, cx| {
                click_handler(false, window, cx);
            });
            let esc_handler = handler.clone();
            backdrop = backdrop.on_key_down(move |event: &KeyDownEvent, window, cx| {
                if event.keystroke.key == "escape" {
                    esc_handler(false, window, cx);
                }
            });
        }

        backdrop.into_any_element()
    }
}
