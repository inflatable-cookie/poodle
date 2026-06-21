//! Checkbox — real GPUI component backed by CheckboxSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CheckState, CheckboxSpec, ControlDensity, ControlSize, IconSpec};

use super::icon::Icon;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

/// Per-size icon token (Svelte resolves `--poodle-size-icon-{size}`).
fn icon_token(size: ControlSize) -> &'static str {
    match size {
        ControlSize::Xs => "size.icon.xs",
        ControlSize::Sm => "size.icon.sm",
        ControlSize::Md => "size.icon.md",
        ControlSize::Lg => "size.icon.lg",
        ControlSize::Xl => "size.icon.xl",
    }
}

/// Indicator border-radius in rem per size.
///
/// Svelte ladder (`Checkbox.svelte` lines 193/204/216/227): xs `0.1875`, sm
/// `0.25`, md `0.3125`, lg `0.375`, xl `0.4375rem`. These are contract-exact rem
/// literals (no semantic radius token matches them) applied via `rem_to_px`.
fn indicator_radius_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.1875,
        ControlSize::Sm => 0.25,
        ControlSize::Md => 0.3125,
        ControlSize::Lg => 0.375,
        ControlSize::Xl => 0.4375,
    }
}

/// Mark glyph offset from the per-size icon token, in rem.
///
/// Svelte: xs/sm/md mark = `icon-{size} − 0.125rem`; lg/xl mark =
/// `icon-{size} − 0.25rem` (`Checkbox.svelte` lines 197/208/162/219/230).
fn mark_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs | ControlSize::Sm | ControlSize::Md => -0.125,
        ControlSize::Lg | ControlSize::Xl => -0.25,
    }
}

/// A real GPUI checkbox component backed by `CheckboxSpec`.
pub struct Checkbox {
    spec: CheckboxSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Checkbox {
    type Target = CheckboxSpec;
    fn deref(&self) -> &CheckboxSpec {
        &self.spec
    }
}

impl Checkbox {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CheckboxSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    pub fn from_spec(spec: CheckboxSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn checked(mut self, v: bool) -> Self {
        self.spec.checked = Some(v);
        self
    }
    pub fn default_checked(mut self, v: bool) -> Self {
        self.spec.default_checked = v;
        self
    }
    pub fn mixed(mut self, v: bool) -> Self {
        self.spec.is_mixed = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn read_only(mut self, v: bool) -> Self {
        self.spec.is_read_only = v;
        self
    }
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = Some(v.into());
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn description_id(mut self, v: impl Into<String>) -> Self {
        self.spec.description_id = Some(v.into());
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    // ── GPUI-specific builders ────────────────────────────────
    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
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

        // Svelte gap ladder: compact=0.375rem (literal), default=space-inline-sm,
        // comfortable=space-inline-md (`Checkbox.svelte` lines 113/176/179).
        let inline_gap = match spec.density {
            ControlDensity::Compact    => px(rem_to_px(0.375)),
            ControlDensity::Default    => resolve_px(theme, "space.inline.sm"),
            ControlDensity::Comfortable => resolve_px(theme, "space.inline.md"),
        };
        let label_size = px(rem_to_px(size_font_rem(effective_size)));
        // Indicator size = per-size icon token + 0.125rem (Svelte adds this offset
        // at every size, e.g. md = icon-md + 0.125rem = 1.125rem). Resolving the
        // raw icon token without the offset undersizes the box ~2px per size.
        let indicator_size = resolve_px(theme, icon_token(effective_size)) + px(rem_to_px(0.125));
        let indicator_radius = px(rem_to_px(indicator_radius_rem(effective_size)));
        // Mark glyph = per-size icon token + offset (xs/sm/md −0.125, lg/xl −0.25).
        // Rendered at the exact px size, not a discrete IconSize step.
        let mark_size_px =
            f32::from(resolve_px(theme, icon_token(effective_size))) + rem_to_px(mark_offset_rem(effective_size));
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

            if is_checked {
                ind = ind.bg(accent).border_1().border_color(accent);
                match state {
                    CheckState::Checked => {
                        ind = ind.child(
                            Icon::from_spec(IconSpec::new("check"), theme)
                                .with_px_size(mark_size_px)
                                .with_color(text_inverse),
                        );
                    }
                    CheckState::Mixed => {
                        ind = ind.child(
                            Icon::from_spec(IconSpec::new("minus"), theme)
                                .with_px_size(mark_size_px)
                                .with_color(text_inverse),
                        );
                    }
                    _ => {}
                }
            } else {
                ind = ind.bg(surface_bg).border_1().border_color(border);
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
            .focus(move |s| {
                s.border_color(focus_ring_color)
                    .shadow(vec![gpui::BoxShadow {
                        color: Hsla {
                            a: focus_ring_color.a * 0.28,
                            ..focus_ring_color
                        },
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(2.0),
                    }])
            });

        if spec.is_disabled {
            row = row
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        } else if spec.is_read_only {
            row = row.cursor_default();
        } else {
            row = row.cursor_pointer();
        }

        row = row.child(indicator);

        if let Some(ref label) = spec.label {
            row = row.child(
                div()
                    .text_size(label_size)
                    .font_weight(FontWeight::MEDIUM)
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
