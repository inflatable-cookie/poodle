//! CodeInput — real GPUI component backed by CodeInputSpec.
//!
//! Renders a segmented code entry field with per-slot focus
//! rings, a hidden real input, and split-after-3 gap for 6-digit codes.
//! Supports optional masking for PIN-style entry.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CodeInputSpec, ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState,
};

use crate::presentation::{control_height_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{
    focus_ring_shadow, resolve_color, resolve_opacity, resolve_px, resolve_radius,
};

/// A real GPUI code input component backed by `CodeInputSpec`.
pub struct CodeInput {
    spec: CodeInputSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    /// Index of the slot that appears "active" (focused).
    active_index: Option<usize>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_complete: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for CodeInput {
    type Target = CodeInputSpec;
    fn deref(&self) -> &CodeInputSpec {
        &self.spec
    }
}

impl CodeInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CodeInputSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            active_index: None,
            on_change: None,
            on_complete: None,
        }
    }

    pub fn from_spec(spec: CodeInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            active_index: None,
            on_change: None,
            on_complete: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn length(mut self, v: usize) -> Self {
        self.spec.length = v;
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: impl Into<String>) -> Self {
        self.spec.default_value = v.into();
        self
    }
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.spec.name = v.into();
        self
    }
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = v.into();
        self
    }
    pub fn hint(mut self, v: impl Into<String>) -> Self {
        self.spec.hint = Some(v.into());
        self
    }
    pub fn error(mut self, v: impl Into<String>) -> Self {
        self.spec.error = Some(v.into());
        self
    }
    pub fn mask(mut self, v: bool) -> Self {
        self.spec.mask = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn validation_state(mut self, v: ValidationState) -> Self {
        self.spec.validation_state = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    // ── GPUI-specific builders ────────────────────────────────
    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    /// Set which slot index appears active/focused (for preview demonstration).
    pub fn with_active_index(mut self, index: usize) -> Self {
        self.active_index = Some(index);
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    pub fn on_complete(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_complete = Some(Box::new(handler));
        self
    }
}

impl IntoElement for CodeInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve effective size ────────────────────────────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Slot dimensions based on size (from Svelte CSS) ───────
        // Svelte: slots are square, each size = control-height for that size
        // xs=1.5rem, sm=1.75rem, md=2.25rem, lg=2.75rem, xl=3.25rem
        let slot_size = rem_to_px(control_height_rem(effective_size));
        let slot_width = slot_size;
        let slot_height = slot_size;
        let font_size = match effective_size {
            ControlSize::Xs => rem_to_px(0.8125),
            ControlSize::Sm => rem_to_px(0.875),
            ControlSize::Md => rem_to_px(1.0),
            ControlSize::Lg => rem_to_px(1.125),
            ControlSize::Xl => rem_to_px(1.25),
        };

        // ── Gap between slots (density-aware) ─────────────────────
        // Svelte: compact→0.25rem, comfortable→space.inline.md; default interpolated at 0.375rem
        let slot_gap = match spec.density {
            ControlDensity::Compact => px(rem_to_px(0.25)),
            ControlDensity::Default => px(rem_to_px(0.375)),
            ControlDensity::Comfortable => resolve_px(theme, "space.inline.md"),
        };

        // ── Split gap: extra margin after middle digit for 6-digit codes ─
        // compact→space.inline.sm(8px), default→space.inline.md(12px), comfortable→space.inline.lg(16px)
        let split_extra_margin = match spec.density {
            ControlDensity::Compact => resolve_px(theme, "space.inline.sm"),
            ControlDensity::Default => resolve_px(theme, "space.inline.md"),
            ControlDensity::Comfortable => resolve_px(theme, "space.inline.lg"),
        };

        // ── Token resolution ──────────────────────────────────────
        let border_color = resolve_color(theme, "color.border.default");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent_border = resolve_color(theme, "color.accent.border");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        let danger_color = resolve_color(theme, "color.status.danger");
        let control_radius = resolve_radius(theme, "radius.control");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");

        let effective_validation = spec.effective_validation_state();
        let is_invalid = effective_validation == ValidationState::Invalid;

        let slot_border = if is_invalid {
            danger_color
        } else {
            border_color
        };
        let active_slot_border = if is_invalid {
            danger_color
        } else {
            accent_border
        };
        let active_ring_color = if is_invalid { danger_color } else { focus_ring };

        // ── Sanitize value (digits only, clamped to length) ───────
        let current_value = spec.current_value();
        let digits: Vec<char> = current_value
            .chars()
            .filter(|c| c.is_ascii_digit())
            .take(spec.length)
            .collect();

        // ── Determine active slot index ───────────────────────────
        let active_idx = self
            .active_index
            .unwrap_or(digits.len().min(spec.length.saturating_sub(1)));

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-code-input-{}", suffix)
        } else {
            format!("poodle-code-input-{}", spec.name)
        };

        // ── Build outer wrapper (label + slots + hint/error) ──────
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let mut wrapper = div().flex().flex_col().gap(gap_sm);

        // Label
        if !spec.label.is_empty() {
            wrapper = wrapper.child(
                div()
                    .text_size(px(rem_to_px(size_font_rem(effective_size))))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(text_primary)
                    .child(spec.label.clone()),
            );
        }

        // ── Slot row ──────────────────────────────────────────────
        let mut row = div()
            .id(SharedString::from(id_str))
            .focusable()
            .flex()
            .gap(slot_gap)
            .w_auto();

        // Focus ring on the whole group when focused
        let focus_color = active_ring_color;
        row = row.focus(move |s| s.shadow(focus_ring_shadow(focus_color)));

        for i in 0..spec.length {
            let digit = digits
                .get(i)
                .map(|c| {
                    if spec.mask {
                        "\u{2022}".to_string()
                    } else {
                        c.to_string()
                    }
                })
                .unwrap_or_default();
            let is_active = i == active_idx;

            let mut slot = div()
                .flex()
                .items_center()
                .justify_center()
                .w(px(slot_width))
                .h(px(slot_height))
                .border_1()
                .border_color(slot_border)
                .rounded(control_radius)
                .bg(surface_bg)
                .text_color(text_primary)
                .text_size(px(font_size))
                .font_weight(FontWeight::SEMIBOLD)
                .line_height(relative(1.0));

            // Active slot highlight
            if is_active {
                slot = slot
                    .border_color(active_slot_border)
                    .shadow(focus_ring_shadow(active_ring_color));
            }

            // Digit content
            if !digit.is_empty() {
                slot = slot.child(digit);
            }

            // Split-after gap for 6-digit codes: extra margin after index 2
            if spec.length == 6 && i == 2 {
                slot = slot.mr(split_extra_margin);
            }

            row = row.child(slot);
        }

        // Keyboard handling
        if !spec.is_disabled {
            let current_val = String::from_iter(digits.iter());
            let length = spec.length;
            let on_change_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
                self.on_change.map(|h| std::rc::Rc::from(h));
            let on_complete_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
                self.on_complete.map(|h| std::rc::Rc::from(h));

            if on_change_rc.is_some() || on_complete_rc.is_some() {
                let change_handler = on_change_rc.clone();
                let complete_handler = on_complete_rc.clone();
                row = row.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    let key = event.keystroke.key.as_str();
                    if key == "backspace" {
                        if !current_val.is_empty() {
                            let mut chars: Vec<char> = current_val.chars().collect();
                            chars.pop();
                            let new_val: String = chars.into_iter().collect();
                            if let Some(ref handler) = change_handler {
                                handler(&new_val, window, cx);
                            }
                        }
                    } else if key.len() == 1
                        && !event.keystroke.modifiers.platform
                        && !event.keystroke.modifiers.control
                    {
                        let ch = key.chars().next().unwrap();
                        if ch.is_ascii_digit() && current_val.len() < length {
                            let new_val = format!("{}{}", current_val, ch);
                            if let Some(ref handler) = change_handler {
                                handler(&new_val, window, cx);
                            }
                            if new_val.len() == length {
                                if let Some(ref handler) = complete_handler {
                                    handler(&new_val, window, cx);
                                }
                            }
                        }
                    }
                });
            }
        }

        wrapper = wrapper.child(row);

        // ── Hint text ─────────────────────────────────────────────
        if let Some(ref hint) = spec.hint {
            if spec.error.is_none() {
                wrapper = wrapper.child(
                    div()
                        .text_size(px(rem_to_px(size_font_rem(effective_size) * 0.85)))
                        .text_color(text_secondary)
                        .child(hint.clone()),
                );
            }
        }

        // ── Error message ─────────────────────────────────────────
        if let Some(ref error) = spec.error {
            wrapper = wrapper.child(
                div()
                    .text_size(px(rem_to_px(size_font_rem(effective_size) * 0.85)))
                    .text_color(danger_color)
                    .child(error.clone()),
            );
        }

        // ── Disabled state ────────────────────────────────────────
        if spec.is_disabled {
            wrapper = wrapper
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        wrapper.into_any_element()
    }
}
