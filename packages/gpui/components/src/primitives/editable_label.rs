//! EditableLabel — real GPUI component backed by EditableLabelSpec.
//!
//! Contract: `docs/contracts/components/editable-label.md`
//! Reference: `packages/svelte/components/src/EditableLabel.svelte`
//!
//! A read-mostly label that transitions into a single-line text input on
//! activation. This component RENDERS the correct mode for the current spec
//! state:
//!   - display mode (`is_editing == false`): the label text (or empty-text /
//!     placeholder), an optional leading-pencil edit affordance, hover/focus
//!     hint border + background, disabled opacity.
//!   - editing mode (`is_editing == true`): the composed input element seeded
//!     with the current value + placeholder, accent-focusRing border, surface
//!     background and focus box-shadow (flush variant → bottom-border only).
//!
//! The activation gesture (click / double-click per `activation_mode`), the
//! commit (Enter / blur) and cancel (Escape) keys, `select_on_focus`, and the
//! `onCommit`/`onCancel`/`onEditStart` callbacks are PREVIEW-EVENT-LOOP bound:
//! the host drives `is_editing` / `value` and re-renders. This component does
//! NOT synthesize key-by-key editing or fake interactivity — it renders the
//! mode the spec asks for. Commit/cancel handlers are forwarded so a host that
//! owns a real text editor can wire them; no characters are synthesized here.
//!
//! ARIA: GPUI has no accessibility API, so `aria-label` / button-role /
//! `aria-hidden` icon are accepted gaps (per parity doc).

use gpui::{prelude::FluentBuilder, *};
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlSize, EditableLabelActivation, EditableLabelSpec, EditableLabelVariant, IconSize,
    IconSpec,
};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Per-size padding-y offset in rem, relative to the `space.control.y` token.
/// Contract §8 "Size adjustments": xs/sm subtract, lg/xl add (the y component
/// of the `calc(space-control-y +/- …)` declarations). No shared presentation
/// helper exists for the y axis, so the contract-exact rem offsets live here.
fn size_pad_y_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.0625,
        ControlSize::Xl => 0.125,
    }
}

/// Per-size padding-x offset in rem, relative to the `space.control.x` token.
/// Contract §8 "Size adjustments": xs/sm -0.125, md 0, lg +0.125, xl +0.1875.
/// (Distinct from `presentation::size_padding_x_offset_rem`, whose sm value is
/// -0.125; EditableLabel's contract sm x-offset is -0.0625.)
fn size_pad_x_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.125,
        ControlSize::Xl => 0.1875,
    }
}

/// Per-size font-size in rem. Contract §8: xs 0.75, sm/md base label-size,
/// lg 0.9375, xl 1.0. (The base label-size is `size_font_rem(Md)` = 0.8125.)
fn size_font_override_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm | ControlSize::Md => size_font_rem(ControlSize::Md),
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// A real GPUI editable label component backed by `EditableLabelSpec`.
pub struct EditableLabel {
    spec: EditableLabelSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    /// Host-driven change handler. Stored for consumers that own a real text
    /// buffer (e.g. tree inline-rename) and re-render with the new value. It is
    /// NOT used to synthesize key-by-key editing inside this component.
    #[allow(clippy::type_complexity)]
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_commit: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_cancel: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_edit_start: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for EditableLabel {
    type Target = EditableLabelSpec;
    fn deref(&self) -> &EditableLabelSpec {
        &self.spec
    }
}

impl EditableLabel {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: EditableLabelSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
            on_commit: None,
            on_cancel: None,
            on_edit_start: None,
        }
    }

    pub fn from_spec(spec: EditableLabelSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
            on_commit: None,
            on_cancel: None,
            on_edit_start: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = v.into();
        self
    }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self {
        self.spec.placeholder = Some(v.into());
        self
    }
    pub fn empty_text(mut self, v: impl Into<String>) -> Self {
        self.spec.empty_text = Some(v.into());
        self
    }
    pub fn editing(mut self, v: bool) -> Self {
        self.spec.is_editing = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn variant(mut self, v: EditableLabelVariant) -> Self {
        self.spec.variant = v;
        self
    }
    pub fn activation_mode(mut self, v: EditableLabelActivation) -> Self {
        self.spec.activation_mode = v;
        self
    }
    pub fn select_on_focus(mut self, v: bool) -> Self {
        self.spec.select_on_focus = v;
        self
    }
    pub fn max_length(mut self, v: usize) -> Self {
        self.spec.max_length = Some(v);
        self
    }
    pub fn show_edit_icon(mut self, v: bool) -> Self {
        self.spec.show_edit_icon = v;
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

    /// Host-driven change handler. The host owns the text buffer and re-renders
    /// the spec with the new `value`; this component does not synthesize edits.
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    /// Called when editing is committed (Enter / blur in the host event loop).
    pub fn on_commit(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_commit = Some(Box::new(handler));
        self
    }

    /// Called when editing is cancelled (Escape in the host event loop).
    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }

    /// Called when edit mode begins (host-driven activation gesture).
    pub fn on_edit_start(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_edit_start = Some(Box::new(handler));
        self
    }
}

impl IntoElement for EditableLabel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Token-resolved geometry ─────────────────────────────
        let text_color = resolve_color(theme, spec.text_color_token());
        let placeholder_color = resolve_color(theme, spec.placeholder_color_token());
        let edit_border = resolve_color(theme, spec.edit_border_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let control_radius = resolve_radius(theme, spec.radius_token());
        let surface_bg = resolve_color(theme, spec.fill_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let body_size = px(rem_to_px(size_font_override_rem(effective_size)));

        // Padding: contract §8 `calc(space.control.{y,x} +/- size-offset)`.
        // Density adjusts padding-inline only (Svelte: compact -0.125rem,
        // comfortable +0.125rem); it must NOT change padding-block / height.
        let base_pad_y = resolve_px(theme, "space.control.y");
        let base_pad_x = resolve_px(theme, "space.control.x");
        let density_offset = match spec.density {
            poodle_specs::ControlDensity::Compact => px(rem_to_px(-0.125)),
            poodle_specs::ControlDensity::Default => px(0.0),
            poodle_specs::ControlDensity::Comfortable => px(rem_to_px(0.125)),
        };
        let pad_y = base_pad_y + px(rem_to_px(size_pad_y_offset_rem(effective_size)));
        let pad_x =
            base_pad_x + px(rem_to_px(size_pad_x_offset_rem(effective_size))) + density_offset;

        // Focus / editing box-shadow spread = `border.width.focus` token.
        let focus_width = resolve_px(theme, "border.width.focus");
        let make_focus_shadow = move || {
            vec![gpui::BoxShadow {
                color: Hsla {
                    a: focus_ring.a * 0.28,
                    ..focus_ring
                },
                offset: point(px(0.0), px(0.0)),
                blur_radius: px(0.0),
                spread_radius: focus_width,
            }]
        };

        // Display gap = `space.inline.sm` (Svelte/contract display gap 0.375rem).
        let display_gap = resolve_px(theme, "space.inline.sm");

        // Hover hint: Svelte `color-mix(border-default 72%, transparent)` border
        // + `color-mix(surface 52%, transparent)` bg.
        let default_border = resolve_color(theme, "color.border.default");
        let hover_border = Hsla {
            a: default_border.a * 0.72,
            ..default_border
        };
        let hover_bg = Hsla {
            a: surface_bg.a * 0.52,
            ..surface_bg
        };

        let is_flush = spec.variant == EditableLabelVariant::Flush;
        let is_empty = spec.value.is_empty();

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-editable-label-{}", suffix)
        } else {
            "poodle-editable-label".to_string()
        };

        // Shared base: width, font, radius, padding (unless flush).
        let base = move |el: Div| -> Div {
            el.w_full()
                .text_size(body_size)
                .when(!is_flush, |el| {
                    el.px(pad_x).py(pad_y).rounded(control_radius).border_1()
                })
                .when(is_flush, |el| el.border_0().rounded(px(0.0)))
        };

        if spec.is_editing {
            // ── Editing mode: composed single-line input ─────────
            // Seeded with the current value (or placeholder when empty).
            // maxLength is enforced by the host editor; we seed the visible
            // text only — no key-by-key synthesis here.
            let draft = spec.value.clone();
            let input_text = if draft.is_empty() {
                spec.placeholder.clone().unwrap_or_default()
            } else {
                draft.clone()
            };
            let input_col = if draft.is_empty() {
                placeholder_color
            } else {
                text_color
            };

            let mut input = base(div())
                .id(SharedString::from(format!("{id_str}-input")))
                .focusable()
                .text_color(input_col)
                .child(SharedString::from(input_text));

            if is_flush {
                // Flush editing: bottom border only, no box-shadow, transparent bg.
                input = input
                    .border_b_1()
                    .border_color(edit_border)
                    .focus(|s| s.border_color(focus_ring));
            } else {
                // Default editing: accent border + surface bg + focus shadow.
                let shadow = make_focus_shadow();
                input = input
                    .border_color(edit_border)
                    .bg(surface_bg)
                    .focus(move |s| s.border_color(focus_ring).shadow(shadow));
            }

            if spec.is_disabled {
                input = input
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            }

            // Commit on Enter/Tab, cancel on Escape, and live edits through
            // on_change: single characters append, backspace deletes — the
            // same caret-at-end editing the composed TextInput does. The host
            // owns the buffer and re-renders with the new value; max_length
            // is enforced here because the host cannot see the keystroke.
            if !spec.is_disabled {
                let current_value = draft;
                let max_length = spec.max_length;
                let on_commit = self.on_commit;
                let on_cancel = self.on_cancel;
                let on_change = self.on_change;
                input = input.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    let keystroke = &event.keystroke;
                    match keystroke.key.as_str() {
                        "enter" | "tab" => {
                            if let Some(ref handler) = on_commit {
                                handler(&current_value, window, cx);
                            }
                        }
                        "escape" => {
                            if let Some(ref handler) = on_cancel {
                                handler(window, cx);
                            }
                        }
                        "backspace" => {
                            if let (Some(handler), false) = (&on_change, current_value.is_empty())
                            {
                                let mut chars: Vec<char> = current_value.chars().collect();
                                chars.pop();
                                let next: String = chars.into_iter().collect();
                                handler(&next, window, cx);
                            }
                        }
                        key if key.len() == 1
                            && !keystroke.modifiers.platform
                            && !keystroke.modifiers.control =>
                        {
                            if let Some(ref handler) = on_change {
                                let within_limit = max_length
                                    .map(|limit| current_value.chars().count() < limit)
                                    .unwrap_or(true);
                                if within_limit {
                                    let next = format!("{current_value}{key}");
                                    handler(&next, window, cx);
                                }
                            }
                        }
                        _ => {}
                    }
                });
            }

            input.into_any_element()
        } else {
            // ── Display mode: label text + optional edit affordance ──
            // empty_text takes precedence over placeholder in display mode.
            let display_text = if is_empty {
                spec.empty_text
                    .clone()
                    .or_else(|| spec.placeholder.clone())
                    .unwrap_or_default()
            } else {
                spec.value.clone()
            };
            let text_col = if is_empty {
                placeholder_color
            } else {
                text_color
            };

            // Text span + optional pencil edit icon (contract: icon 0.75rem =
            // IconSize::Sm = 12px, color text-secondary, shown on hover/focus).
            let mut content_row = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(display_gap)
                .child(display_text);

            if spec.show_edit_icon && !spec.is_disabled {
                let icon_color = resolve_color(theme, spec.placeholder_color_token());
                content_row = content_row.child(
                    super::icon::Icon::from_spec(
                        IconSpec::new("pencil").with_size(IconSize::Sm),
                        theme,
                    )
                    .with_color(icon_color),
                );
            }

            let mut display = base(div())
                .id(SharedString::from(id_str))
                .focusable()
                .text_color(text_col)
                .border_color(gpui::transparent_black())
                .child(content_row);

            if !spec.is_disabled {
                // Hover / focus hint — suppressed for flush variant (no border
                // or background change), focus ring still applies via shadow.
                let shadow = make_focus_shadow();
                if is_flush {
                    display = display
                        .cursor_pointer()
                        .focus(move |s| s.shadow(shadow));
                } else {
                    display = display.cursor_pointer().hover(move |s| {
                        s.border_color(hover_border).bg(hover_bg)
                    });
                    let focus_shadow = make_focus_shadow();
                    display = display
                        .focus(move |s| s.border_color(focus_ring).shadow(focus_shadow));
                }
            }

            if spec.is_disabled {
                display = display
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            }

            // Activation gesture is host-driven (click / double-click per
            // `activation_mode`). We forward a click to `on_edit_start` so the
            // host can flip `is_editing`; the precise gesture mapping lives in
            // the preview event loop. Programmatic mode wires no gesture.
            if !spec.is_disabled
                && spec.activation_mode != EditableLabelActivation::Programmatic
            {
                if let Some(handler) = self.on_edit_start {
                    display = display.on_click(move |_event, window, cx| {
                        handler(window, cx);
                    });
                }
            }

            display.into_any_element()
        }
    }
}
