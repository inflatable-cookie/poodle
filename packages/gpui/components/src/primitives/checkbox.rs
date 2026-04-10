//! Checkbox — real GPUI component backed by CheckboxSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{CheckState, CheckboxSpec, ControlSize};

use crate::presentation::{resolve_semantic_size, control_height_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

/// A real GPUI checkbox component backed by `CheckboxSpec`.
pub struct Checkbox {
    spec: CheckboxSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Checkbox {
    type Target = CheckboxSpec;
    fn deref(&self) -> &CheckboxSpec { &self.spec }
}

impl Checkbox {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CheckboxSpec::new(), theme: theme.clone(), id_suffix: None, on_change: None }
    }

    pub fn from_spec(spec: CheckboxSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), id_suffix: None, on_change: None }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn checked(mut self, v: bool) -> Self { self.spec.checked = Some(v); self }
    pub fn default_checked(mut self, v: bool) -> Self { self.spec.default_checked = v; self }
    pub fn mixed(mut self, v: bool) -> Self { self.spec.is_mixed = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn read_only(mut self, v: bool) -> Self { self.spec.is_read_only = v; self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = Some(v.into()); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn description_id(mut self, v: impl Into<String>) -> Self { self.spec.description_id = Some(v.into()); self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn size_role(mut self, v: poodle_primitives::SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn density(mut self, v: poodle_primitives::ControlDensity) -> Self { self.spec.density = v; self }

    // ── GPUI-specific builders ────────────────────────────────
    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Checkbox {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // Contract: gap = space-inline-sm
        let inline_gap = resolve_px(theme, "space.inline.sm");
        let body_size = resolve_px(theme, "typography.body.size");
        // Contract: indicator scales with effective size — base 1.125rem at Md,
        // proportional to control height ratio
        let indicator_base = 18.0_f32; // 1.125rem at Md
        let scale = control_height_rem(effective_size) / control_height_rem(ControlSize::Md);
        let indicator_size = px(indicator_base * scale);
        let indicator_radius = px(5.0 * scale); // 0.3125rem scaled
        let focus_ring_color = resolve_color(theme, "color.accent.focusRing");

        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let accent = if let Some(ref hex) = spec.selected_color {
            crate::theme_ext::parse_hex_color(hex)
                .unwrap_or_else(|| resolve_color(theme, spec.indicator_fill_token()))
        } else {
            resolve_color(theme, spec.indicator_fill_token())
        };
        let border = resolve_color(theme, "color.border.default");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_inverse = resolve_color(theme, "color.text.inverse");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let _elevated = resolve_color(theme, "color.background.elevated");

        let state = spec.current_state();
        let is_checked = matches!(state, CheckState::Checked | CheckState::Mixed);
        let is_interactive = spec.is_interactive();

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-checkbox-{}", suffix)
        } else {
            format!(
                "poodle-checkbox-{}",
                spec.label.as_deref().unwrap_or("anon")
            )
        };

        // Indicator box
        let indicator = {
            let mut ind = div()
                .w(indicator_size)
                .h(indicator_size)
                .rounded(indicator_radius)
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0();

            // Contract: check mark inside indicator = 0.875rem at Md, scaled
            let mark_size = px(14.0 * scale);

            if is_checked {
                ind = ind.bg(accent).border_1().border_color(accent);
                match state {
                    CheckState::Checked => {
                        ind = ind.child(
                            svg()
                                .path(SharedString::from("assets/icons/check.svg"))
                                .size(mark_size)
                                .flex_shrink_0()
                                .text_color(text_inverse),
                        );
                    }
                    CheckState::Mixed => {
                        ind = ind.child(
                            svg()
                                .path(SharedString::from("assets/icons/minus.svg"))
                                .size(mark_size)
                                .flex_shrink_0()
                                .text_color(text_inverse),
                        );
                    }
                    _ => {}
                }
            } else {
                ind = ind
                    .bg(surface_bg)
                    .border_1()
                    .border_color(border);
            }

            ind
        };

        // Row: indicator + label
        let mut row = div()
            .id(SharedString::from(id_str))
            .focusable()
            .flex()
            .items_center()
            .gap(inline_gap)
            // Svelte: focus-visible outline with offset — approximate with border + shadow ring
            .focus(move |s| s
                .border_color(focus_ring_color)
                .shadow(vec![gpui::BoxShadow {
                    color: Hsla { a: focus_ring_color.a * 0.28, ..focus_ring_color },
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(2.0),
                }])
            );

        if is_interactive {
            row = row.cursor_pointer();
        }

        if spec.is_disabled {
            row = row.opacity(disabled_opacity).cursor(CursorStyle::OperationNotAllowed);
        }

        row = row.child(indicator);

        if let Some(ref label) = spec.label {
            row = row.child(
                div()
                    .text_size(body_size)
                    .text_color(text_primary)
                    .child(label.clone()),
            );
        }

        // Click + keyboard handlers
        if let Some(handler) = self.on_change {
            if is_interactive {
                let next_checked = !is_checked;
                let handler = std::rc::Rc::new(handler);
                let key_handler = handler.clone();
                row = row
                    .on_click(move |_event, window, cx| {
                        handler(&next_checked, window, cx);
                    })
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                            key_handler(&next_checked, window, cx);
                        }
                    });
            }
        }

        row.into_any_element()
    }
}
