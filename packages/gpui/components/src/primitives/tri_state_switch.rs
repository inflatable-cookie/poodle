//! TriStateSwitch — real GPUI component backed by TriStateSwitchSpec.
//!
//! Renders as a shrink-wrapped tri-state switch with a sliding active capsule.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{CheckState, ControlSize, TriStateSwitchSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size, control_height_rem, size_font_rem};
use crate::theme_ext::{color_mix, parse_hex_color, resolve_color, resolve_opacity, resolve_px};

/// A real GPUI tri-state switch component backed by `TriStateSwitchSpec`.
///
/// Renders as a shrink-wrapped ternary switch with a shared track and a
/// sliding active capsule.
pub struct TriStateSwitch {
    spec: TriStateSwitchSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Box<dyn Fn(&CheckState, &mut Window, &mut App) + 'static>>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for TriStateSwitch {
    type Target = TriStateSwitchSpec;
    fn deref(&self) -> &TriStateSwitchSpec { &self.spec }
}

impl TriStateSwitch {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TriStateSwitchSpec::new(), theme: theme.clone(), on_change: None, on_click: None }
    }

    pub fn from_spec(spec: TriStateSwitchSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
            on_click: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn state(mut self, v: CheckState) -> Self { self.spec.state = v; self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn size_role(mut self, v: poodle_primitives::SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn density(mut self, v: poodle_primitives::ControlDensity) -> Self { self.spec.density = v; self }

    pub fn on_change(
        mut self,
        handler: impl Fn(&CheckState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for TriStateSwitch {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_gap = resolve_px(theme, "space.inline.sm");

        let border_color = resolve_color(theme, "color.border.default");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let danger = resolve_color(theme, "color.status.danger");
        let success = resolve_color(theme, "color.status.success");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let control_height = px(rem_to_px(control_height_rem(effective_size)));
        let track_padding = px(2.0);
        // Segment width scales with effective size
        let segment_min_w: Pixels = match effective_size {
            ControlSize::Xs => px(56.0),
            ControlSize::Sm => px(64.0),
            ControlSize::Md => px(72.0),
            ControlSize::Lg => px(80.0),
            ControlSize::Xl => px(88.0),
        };
        let segment_h = control_height - px(4.0);
        let segment_radius = segment_h / 2.0;
        let label_size = px(rem_to_px(size_font_rem(effective_size)));
        let track_w = segment_min_w * 3.0 + track_padding * 2.0;
        let track_radius = control_height / 2.0;

        let excluded_color = spec
            .excluded_color
            .as_deref()
            .and_then(parse_hex_color)
            .unwrap_or(danger);
        let default_color = spec
            .default_color
            .as_deref()
            .and_then(parse_hex_color)
            .unwrap_or(text_primary);
        let included_color = spec
            .included_color
            .as_deref()
            .and_then(parse_hex_color)
            .unwrap_or(success);

        let track_bg = color_mix(text_primary, surface_bg, 0.18);
        let excluded_bg = color_mix(excluded_color, surface_bg, 0.18);
        let default_bg = color_mix(default_color, surface_bg, 0.10);
        let included_bg = color_mix(included_color, surface_bg, 0.18);

        let (selected_index, selection_bg, selection_border) = match spec.state {
            CheckState::Unchecked => (0.0, excluded_bg, color_mix(excluded_color, border_color, 0.58)),
            CheckState::Mixed => (1.0, default_bg, border_color),
            CheckState::Checked => (2.0, included_bg, color_mix(included_color, border_color, 0.58)),
        };

        let selection_x = track_padding + segment_min_w * selected_index;

        let selection = div()
            .absolute()
            .top(track_padding)
            .left(selection_x)
            .w(segment_min_w)
            .h(segment_h)
            .rounded(segment_radius)
            .bg(selection_bg)
            .border_1()
            .border_color(selection_border)
            .shadow(vec![
                BoxShadow {
                    color: hsla(0.0, 0.0, 1.0, 0.08),
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(1.0),
                },
                BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.18),
                    offset: point(px(0.0), px(2.0)),
                    blur_radius: px(8.0),
                    spread_radius: px(0.0),
                },
            ]);

        let segment_base = |label: String, color: Hsla| {
            div()
                .w(segment_min_w)
                .h(segment_h)
                .flex()
                .items_center()
                .justify_center()
                .px(px(14.0))
                .rounded(segment_radius)
                .text_size(label_size)
                .font_weight(FontWeight::MEDIUM)
                .whitespace_nowrap()
                .text_color(color)
                .child(label)
        };

        let on_change: Option<std::rc::Rc<dyn Fn(&CheckState, &mut Window, &mut App)>> =
            self.on_change.map(|handler| std::rc::Rc::from(handler));
        let on_click: Option<std::rc::Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>> =
            self.on_click.map(|handler| std::rc::Rc::from(handler));

        let states = [
            (spec.excluded_label().to_string(), CheckState::Unchecked, excluded_color),
            (spec.default_label().to_string(), CheckState::Mixed, text_primary),
            (spec.included_label().to_string(), CheckState::Checked, included_color),
        ];

        let switch_id = SharedString::from("poodle-tri-state-switch");

        let mut container = div()
            .id(switch_id)
            .relative()
            .w(track_w)
            .h(control_height)
            .flex()
            .items_center()
            .rounded(track_radius)
            .border_1()
            .border_color(border_color)
            .bg(track_bg)
            .overflow_hidden()
            .child(selection)
            ;

        if !spec.is_disabled {
            for (idx, (label, state_value, selected_color)) in states.iter().cloned().enumerate() {
                let is_selected = spec.state == state_value;
                let mut segment = segment_base(
                    label,
                    if is_selected { selected_color } else { text_secondary },
                )
                .id(SharedString::from(format!("poodle-tri-state-switch-{}", idx)))
                .focusable()
                .relative()
                .border_1()
                .border_color(gpui::transparent_black())
                .cursor_pointer()
                .focus(move |s| s.border_color(focus_ring).shadow(crate::theme_ext::focus_ring_shadow(focus_ring)));

                if let Some(ref change_handler) = on_change {
                    let click_handler = change_handler.clone();
                    let key_handler = change_handler.clone();
                    let current_state = state_value;
                    let next_state = match idx {
                        0 => CheckState::Mixed,
                        1 => CheckState::Checked,
                        _ => CheckState::Unchecked,
                    };
                    let prev_state = match idx {
                        0 => CheckState::Checked,
                        1 => CheckState::Unchecked,
                        _ => CheckState::Mixed,
                    };
                    segment = segment
                        .on_click(move |_event, window, cx| {
                            click_handler(&current_state, window, cx);
                        })
                        .on_key_down(move |event: &KeyDownEvent, window, cx| {
                            if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                                key_handler(&current_state, window, cx);
                            } else if event.keystroke.key == "right" || event.keystroke.key == "down" {
                                key_handler(&next_state, window, cx);
                            } else if event.keystroke.key == "left" || event.keystroke.key == "up" {
                                key_handler(&prev_state, window, cx);
                            }
                        });
                } else if let Some(ref click_handler) = on_click {
                    let click_handler_for_click = click_handler.clone();
                    let click_handler_for_key = click_handler.clone();
                    segment = segment
                        .on_click(move |event, window, cx| {
                            click_handler_for_click(event, window, cx);
                        })
                        .on_key_down(move |event: &KeyDownEvent, window, cx| {
                            if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                                click_handler_for_key(&ClickEvent::default(), window, cx);
                            }
                        });
                }

                container = container.child(segment);
            }
        } else {
            container = container
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);

            for (idx, (label, state_value, selected_color)) in states.iter().cloned().enumerate() {
                let is_selected = spec.state == state_value;
                let segment = segment_base(
                    label,
                    if is_selected { selected_color } else { text_secondary },
                )
                .id(SharedString::from(format!("poodle-tri-state-switch-{}", idx)))
                .relative();
                container = container.child(segment);
            }
        }

        let mut el = div()
            .flex()
            .items_center()
            .gap(inline_gap)
            .child(container);

        if let Some(ref label) = spec.label {
            el = el.child(
                div()
                    .text_size(label_size)
                    .text_color(text_primary)
                    .child(label.clone()),
            );
        }

        el.into_any_element()
    }
}
