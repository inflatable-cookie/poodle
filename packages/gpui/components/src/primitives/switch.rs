//! Switch — real GPUI component backed by SwitchSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, SwitchSpec, SwitchTone};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px};

/// A real GPUI switch/toggle component backed by `SwitchSpec`.
pub struct Switch {
    spec: SwitchSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Switch {
    type Target = SwitchSpec;
    fn deref(&self) -> &SwitchSpec {
        &self.spec
    }
}

impl Switch {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SwitchSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    pub fn from_spec(spec: SwitchSpec, theme: &GpuiThemeProvider) -> Self {
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
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.spec.name = Some(v.into());
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
    pub fn left_label(mut self, v: impl Into<String>) -> Self {
        self.spec.left_label = Some(v.into());
        self
    }
    pub fn right_label(mut self, v: impl Into<String>) -> Self {
        self.spec.right_label = Some(v.into());
        self
    }
    pub fn left_tone(mut self, v: SwitchTone) -> Self {
        self.spec.left_tone = v;
        self
    }
    pub fn right_tone(mut self, v: SwitchTone) -> Self {
        self.spec.right_tone = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
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

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Switch {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // Contract: gap = space-inline-sm
        let inline_gap = resolve_px(theme, "space.inline.sm");

        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let accent = resolve_color(theme, "color.accent.base");
        let border = resolve_color(theme, "color.border.default");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let label_size = px(rem_to_px(size_font_rem(effective_size)));

        let is_checked = spec.current_checked();
        let is_interactive = !spec.is_disabled && !spec.is_read_only;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-switch-{}", suffix)
        } else {
            format!("poodle-switch-{}", spec.label.as_deref().unwrap_or("anon"))
        };

        // Per-size track geometry from the contract size table.
        // Md baseline: 34×20px track, 2px padding, 14px thumb (matches Svelte).
        // Other sizes scale from the icon-size tokens with non-linear per-size geometry.
        let icon_xs = resolve_px(theme, "size.icon.xs");
        let icon_sm = resolve_px(theme, "size.icon.sm");
        let icon_lg = resolve_px(theme, "size.icon.lg");
        let icon_xl = resolve_px(theme, "size.icon.xl");
        let (track_w, track_h, track_padding, thumb_size) = match effective_size {
            ControlSize::Xs => (icon_xs * 1.75, icon_xs * 0.875, px(1.0), icon_xs * 0.75 - px(2.0)),
            ControlSize::Sm => (icon_sm * 1.875, icon_sm + px(2.0), px(1.5), icon_sm - px(3.0)),
            ControlSize::Md => (px(34.0), px(20.0), px(2.0), px(14.0)),
            ControlSize::Lg => (icon_lg * 2.25 + px(4.0), icon_lg + px(8.0), px(3.0), icon_lg),
            ControlSize::Xl => (icon_xl * 2.5 + px(6.0), icon_xl + px(12.0), px(4.0), icon_xl + px(2.0)),
        };
        let track_radius = track_h / 2.0;
        let thumb_radius = thumb_size / 2.0;
        let knob_offset = if is_checked {
            track_w - thumb_size - track_padding
        } else {
            track_padding
        };
        let focus_ring = resolve_color(theme, "color.accent.focusRing");

        // Svelte: off-track = text-primary 18% + surface, on-track = accent 24% + surface.
        // Resolution order for each side:
        //   1. explicit hex (on_color / off_color)
        //   2. tone-derived color (right_tone / left_tone) mixed with surface
        //   3. default track color (accent / text-primary mix)
        let resolve_tone_color = |tone: SwitchTone| -> Option<Hsla> {
            tone.color_token().map(|token| resolve_color(theme, token))
        };

        let track_bg = if is_checked {
            if let Some(ref hex) = spec.on_color {
                crate::theme_ext::parse_hex_color(hex)
                    .unwrap_or_else(|| color_mix(accent, surface_bg, 0.24))
            } else if let Some(tone_color) = resolve_tone_color(spec.right_tone) {
                color_mix(tone_color, surface_bg, 0.24)
            } else {
                color_mix(accent, surface_bg, 0.24)
            }
        } else {
            if let Some(ref hex) = spec.off_color {
                crate::theme_ext::parse_hex_color(hex)
                    .unwrap_or_else(|| color_mix(text_primary, surface_bg, 0.18))
            } else if let Some(tone_color) = resolve_tone_color(spec.left_tone) {
                color_mix(tone_color, surface_bg, 0.18)
            } else {
                color_mix(text_primary, surface_bg, 0.18)
            }
        };

        // Contract: checked track border = accent-base 58% + border-default.
        // Tone-aware version: use the resolved tone color (or accent as fallback).
        let on_tone_color = resolve_tone_color(spec.right_tone).unwrap_or(accent);
        let off_tone_color = resolve_tone_color(spec.left_tone).unwrap_or(text_primary);

        let track_border = if is_checked {
            color_mix(on_tone_color, border, 0.58)
        } else if spec.left_tone != SwitchTone::Default {
            color_mix(off_tone_color, border, 0.58)
        } else {
            border
        };

        // Contract: checked thumb = accent-base (or tone), unchecked = text-primary (or tone)
        let knob_color = if is_checked {
            on_tone_color
        } else {
            off_tone_color
        };

        // Contract: track inset shadow = inset 0 0 0 1px white/8%
        let inset_shadow_color = hsla(0.0, 0.0, 1.0, 0.08);

        let track = div()
            .w(track_w)
            .h(track_h)
            .rounded(track_radius)
            .bg(track_bg)
            .border_1()
            .border_color(track_border)
            .relative()
            .flex_shrink_0()
            // Contract: inset shadow simulated via inner border highlight
            .shadow(vec![gpui::BoxShadow {
                color: inset_shadow_color,
                offset: point(px(0.0), px(0.0)),
                blur_radius: px(0.0),
                spread_radius: px(1.0),
            }])
            .child(
                div()
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    .bg(knob_color)
                    // Svelte: 0 0.125rem 0.5rem color-mix(black 18%, transparent)
                    .shadow(vec![gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.18),
                        offset: point(px(0.0), px(2.0)),
                        blur_radius: px(8.0),
                        spread_radius: px(0.0),
                    }])
                    .absolute()
                    .top(track_padding)
                    .left(knob_offset),
            );

        // Row: track + label
        let mut row = div()
            .id(SharedString::from(id_str))
            .focusable()
            .flex()
            .items_center()
            .gap(inline_gap)
            .focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
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

        // Dual-label mode: left label | track | right label. Tones tint the
        // corresponding label text using the resolved tone color (unchecked
        // side is muted via alpha). Otherwise: track | optional single label.
        if spec.is_dual_label() {
            let text_secondary = resolve_color(theme, "color.text.secondary");

            let label_color = |tone_color: Hsla, is_active: bool| -> Hsla {
                if is_active {
                    tone_color
                } else {
                    // Inactive side fades toward secondary text.
                    let mut dim = text_secondary;
                    dim.a *= 0.85;
                    dim
                }
            };

            if let Some(ref left) = spec.left_label {
                let color = label_color(off_tone_color, !is_checked);
                row = row.child(
                    div()
                        .text_size(label_size)
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(color)
                        .child(left.clone()),
                );
            }

            row = row.child(track);

            if let Some(ref right) = spec.right_label {
                let color = label_color(on_tone_color, is_checked);
                row = row.child(
                    div()
                        .text_size(label_size)
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(color)
                        .child(right.clone()),
                );
            }
        } else {
            row = row.child(track);

            if let Some(ref label) = spec.label {
                row = row.child(
                    div()
                        .text_size(label_size)
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_primary)
                        .child(label.clone()),
                );
            }
        }

        // Click + keyboard handlers
        if let Some(handler) = self.on_change {
            if is_interactive {
                let next = !is_checked;
                let handler = std::rc::Rc::new(handler);
                let key_handler = handler.clone();
                row = row
                    .on_click(move |_event, window, cx| {
                        handler(&next, window, cx);
                    })
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                            key_handler(&next, window, cx);
                        }
                    });
            }
        }

        row.into_any_element()
    }
}
