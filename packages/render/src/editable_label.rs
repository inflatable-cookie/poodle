//! EditableLabel — a click-to-edit label.
//!
//! Contract: `docs/contracts/components/editable-label.md`
//! Ported from: `packages/jetstream/components/src/editable_label.rs`.
//!
//! Renders the correct mode for the current spec state:
//!   - display mode (`is_editing == false`): the label text (or empty-text /
//!     placeholder), an optional leading-pencil edit affordance, hover hint
//!     border + background, disabled opacity. Flush variant strips the
//!     padding / border / radius and inlines the text.
//!   - editing mode (`is_editing == true`): an input node seeded with the
//!     current value + placeholder, accent-focusRing border, surface
//!     background (flush → bottom border only). The native end-caret editing
//!     subset enforces `max_length` before reporting the controlled value.
//!
//! Edit state, commit (Enter / blur), and cancel (Escape) are host-owned: the
//! host drives `is_editing` / `value` and re-renders. Shared text transitions
//! derive each next value. `on_edit_start` fires when the display-mode label
//! is pressed; already editing or disabled, there is nothing to start.

use std::sync::Arc;

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, Node, StylePatch,
    TextChangeHandler,
};
use poodle_specs::{
    ControlDensity, ControlSize, EditableLabelActivation, EditableLabelSpec, EditableLabelVariant,
};

use crate::color::{mix_srgb, TRANSPARENT};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Per-size font-size in rem. Contract §8: xs 0.75, sm/md base label-size
/// (0.8125), lg 0.9375, xl 1.0.
fn font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm | ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Per-size padding-y offset in rem, relative to `space.control.y`.
/// Contract §8 size table (y component of the `calc(...)` declarations).
fn pad_y_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.0625,
        ControlSize::Xl => 0.125,
    }
}

/// Per-size padding-x offset in rem, relative to `space.control.x`.
/// Contract §8 size table: xs/sm -0.125/-0.0625, md 0, lg +0.125, xl +0.1875.
fn pad_x_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.125,
        ControlSize::Xl => 0.1875,
    }
}

/// Density adjusts padding-inline only (Svelte: compact -0.125rem,
/// comfortable +0.125rem). Must NOT change padding-block / height.
fn density_pad_x_offset_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => -0.125,
        ControlDensity::Default => 0.0,
        ControlDensity::Comfortable => 0.125,
    }
}

fn limit_edit_value(value: String, max_length: Option<usize>) -> String {
    let Some(max_length) = max_length else {
        return value;
    };
    value.chars().take(max_length).collect()
}

fn end_edit_state(value: &str) -> poodle_headless::text_input::EditState {
    let end = value.chars().count();
    poodle_headless::text_input::EditState {
        anchor: end,
        head: end,
    }
}

#[derive(Default, Clone)]
pub struct EditableLabelHandlers {
    pub on_edit_start: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_change: Option<TextChangeHandler>,
    pub on_commit: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub fn editable_label(
    spec: &EditableLabelSpec,
    ctx: &RenderContext<'_>,
    on_edit_start: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    editable_label_with_handlers(
        spec,
        ctx,
        EditableLabelHandlers {
            on_edit_start,
            ..EditableLabelHandlers::default()
        },
    )
}

pub fn editable_label_with_handlers(
    spec: &EditableLabelSpec,
    ctx: &RenderContext<'_>,
    handlers: EditableLabelHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    // ── Token-resolved geometry ──────────────────────────────────────────
    let text_color = ctx.theme().resolve_color(spec.text_color_token());
    let placeholder_color = ctx.theme().resolve_color(spec.placeholder_color_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    let font_size = rem_to_px(font_rem(effective_size));

    let base_pad_y = ctx.theme().resolve_space("space.control.y");
    let base_pad_x = ctx.theme().resolve_space("space.control.x");
    let pad_y = base_pad_y + rem_to_px(pad_y_offset_rem(effective_size));
    let pad_x = base_pad_x
        + rem_to_px(pad_x_offset_rem(effective_size))
        + rem_to_px(density_pad_x_offset_rem(density));

    // Focus / editing border width = `border.width.focus` token.
    let focus_width = ctx.theme().resolve_space("border.width.focus");
    let display_gap = ctx.theme().resolve_space("space.inline.sm");

    let is_flush = spec.variant == EditableLabelVariant::Flush;
    let is_empty = spec.value.is_empty();

    let mut el = if spec.is_editing {
        // ── Editing mode: input node seeded with value + placeholder ────
        // The native subset edits at the end of the controlled value. The
        // component owns max_length because the host sees only the result.
        let edit_border = ctx.theme().resolve_color(spec.edit_border_token());
        let surface_bg = ctx.theme().resolve_color(spec.fill_token());

        let mut input = Node::input(
            spec.value.clone(),
            spec.placeholder.clone().unwrap_or_default(),
        );
        input.a11y.label = Some(
            spec.aria_label
                .clone()
                .unwrap_or_else(|| "Edit label".to_string()),
        );
        {
            let s = &mut input.style;
            s.fill_width = true;
            s.descriptor.text_color = Some(text_color);
            s.text_size = Some(font_size);
            if is_flush {
                // Flush editing: bottom accent border only, no fill of its own
                // (the backend's input default applies).
                s.border_bottom_width = Some(1.0);
                s.descriptor.border.color = edit_border;
            } else {
                // Default editing: accent border + surface bg + padding + radius.
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = pad_x;
                pad.right = pad_x;
                pad.top = pad_y;
                pad.bottom = pad_y;
                s.descriptor.corner_radii.top_left = radius;
                s.descriptor.corner_radii.top_right = radius;
                s.descriptor.corner_radii.bottom_right = radius;
                s.descriptor.corner_radii.bottom_left = radius;
                s.descriptor.border.width = focus_width;
                s.descriptor.border.color = edit_border;
                s.descriptor.background = Some(surface_bg);
            }
        }
        input.interaction.focusable = true;
        if !spec.is_disabled {
            if let Some(change) = handlers.on_change.clone() {
                let value = spec.value.clone();
                let max_length = spec.max_length;
                input.interaction.on_text_change = Some(Arc::new(move |next: &str| {
                    let next = limit_edit_value(next.to_string(), max_length);
                    if next != value {
                        change(&next);
                    }
                }));
            }
            input.interaction.on_cancel = handlers.on_cancel.clone();
            // The editing input rings in the accent focus colour when
            // focused — the same focus-visible signal a TextInput carries —
            // which is also what lets the GPUI backend track its focus.
            input.style.focus = Some(StylePatch {
                background: None,
                border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
                text_color: None,
                opacity: None,
            });
            // Typing reaches the controlled draft through the shared edit
            // transitions. GPUI's admitted subset keeps the caret at the end;
            // the host re-renders with the reported value after each edit.
            if let Some(change) = handlers.on_change.clone() {
                let value = spec.value.clone();
                let max_length = spec.max_length;
                let insert_change = change.clone();
                let insert_value = value.clone();
                input.interaction.on_edit_insert = Some(Arc::new(move |text: &str| {
                    let outcome = poodle_headless::text_input::insert_transition(
                        &insert_value,
                        end_edit_state(&insert_value),
                        text,
                    );
                    if let Some(next) = outcome.value {
                        let next = limit_edit_value(next, max_length);
                        if next != insert_value {
                            insert_change(&next);
                        }
                    }
                }));
                input.interaction.on_edit_key = Some(Arc::new(move |key, mods| {
                    let key = if key == "space" {
                        " ".to_string()
                    } else if mods.shift && key.chars().count() == 1 {
                        key.to_uppercase()
                    } else {
                        key.to_string()
                    };
                    let Some(outcome) = poodle_headless::text_input::edit_transition(
                        &value,
                        end_edit_state(&value),
                        &key,
                        false,
                        mods.accel,
                    ) else {
                        return;
                    };
                    if let Some(next) = outcome.value {
                        let next = limit_edit_value(next, max_length);
                        if next != value {
                            change(&next);
                        }
                    }
                }));
            }
            if let Some(handler) = &handlers.on_commit {
                let handler = Arc::clone(handler);
                let value = spec.value.clone();
                input.interaction.on_submit = Some(Arc::new(move || handler(&value)));
            }
        }
        input
    } else {
        // ── Display mode: label text + optional edit affordance ─────────
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

        // Text span + optional pencil icon (contract: 0.75rem icon, color
        // text-secondary, shown on hover/focus — here always present when set).
        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = display_gap;
            s.fill_width = true;
        }
        let mut label = Node::text(&display_text);
        label.style.descriptor.text_color = Some(text_col);
        label.style.text_size = Some(font_size);
        let mut row = row.child(label);

        if spec.show_edit_icon && !spec.is_disabled {
            let mut pencil = Node::icon("pencil", rem_to_px(0.75));
            pencil.style.descriptor.text_color = Some(placeholder_color);
            row = row.child(pencil);
        }

        if is_flush {
            // Flush display: no padding / border / radius, transparent bg.
            row.style.descriptor.cursor = CursorHint::Pointer;
        } else {
            // Default display: padding + radius + transparent border, hover hint.
            // Svelte hover = color-mix(border-default 72%, transparent) border
            //              + color-mix(surface 52%, transparent) bg.
            let border_default = ctx.theme().resolve_color("color.border.default");
            let surface = ctx.theme().resolve_color(spec.fill_token());
            let hover_border = mix_srgb(border_default, TRANSPARENT, 0.72);
            let hover_bg = mix_srgb(surface, TRANSPARENT, 0.52);
            let s = &mut row.style;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pad_x;
            pad.right = pad_x;
            pad.top = pad_y;
            pad.bottom = pad_y;
            s.descriptor.corner_radii.top_left = radius;
            s.descriptor.corner_radii.top_right = radius;
            s.descriptor.corner_radii.bottom_right = radius;
            s.descriptor.corner_radii.bottom_left = radius;
            s.descriptor.border.width = focus_width;
            s.descriptor.border.color = ColorValue(0.0, 0.0, 0.0, 0.0);
            s.descriptor.cursor = CursorHint::Pointer;
            s.hover = Some(StylePatch {
                background: Some(hover_bg),
                border_color: Some(hover_border),
                text_color: None,
                opacity: None,
            });
        }
        row
    };

    if spec.is_disabled {
        el.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
        el.interaction.disabled = true;
    }

    if !spec.is_editing
        && !spec.is_disabled
        && spec.activation_mode != EditableLabelActivation::Programmatic
    {
        if let Some(handler) = handlers.on_edit_start {
            el.interaction.focusable = true;
            el.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn key_change(
        spec: EditableLabelSpec,
        key: &str,
        modifiers: poodle_node::NodeModifiers,
    ) -> Vec<String> {
        let values = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&values);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = editable_label_with_handlers(
            &spec.with_editing(true),
            &ctx,
            EditableLabelHandlers {
                on_change: Some(Arc::new(move |value| {
                    sink.lock().unwrap().push(value.to_string());
                })),
                ..EditableLabelHandlers::default()
            },
        );
        (node.interaction.on_edit_key.as_ref().expect("key handler"))(key, modifiers);
        let result = values.lock().unwrap().clone();
        result
    }

    fn inserted_change(spec: EditableLabelSpec, inserted: &str) -> Vec<String> {
        let values = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&values);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = editable_label_with_handlers(
            &spec.with_editing(true),
            &ctx,
            EditableLabelHandlers {
                on_change: Some(Arc::new(move |value| {
                    sink.lock().unwrap().push(value.to_string());
                })),
                ..EditableLabelHandlers::default()
            },
        );
        (node
            .interaction
            .on_edit_insert
            .as_ref()
            .expect("insert handler"))(inserted);
        let result = values.lock().unwrap().clone();
        result
    }

    #[test]
    fn native_end_editing_maps_space_shift_and_backspace() {
        assert_eq!(
            key_change(
                EditableLabelSpec::new().with_value("Studio"),
                "space",
                poodle_node::NodeModifiers::default(),
            ),
            ["Studio "]
        );
        assert_eq!(
            key_change(
                EditableLabelSpec::new().with_value("Studio"),
                "m",
                poodle_node::NodeModifiers {
                    shift: true,
                    ..poodle_node::NodeModifiers::default()
                },
            ),
            ["StudioM"]
        );
        assert_eq!(
            key_change(
                EditableLabelSpec::new().with_value("Studio"),
                "backspace",
                poodle_node::NodeModifiers::default(),
            ),
            ["Studi"]
        );
    }

    #[test]
    fn inserted_text_appends_and_max_length_is_enforced() {
        assert_eq!(
            inserted_change(EditableLabelSpec::new().with_value("Studio"), " rig"),
            ["Studio rig"]
        );
        assert_eq!(
            inserted_change(
                EditableLabelSpec::new()
                    .with_value("Studio")
                    .with_max_length(8),
                " rig",
            ),
            ["Studio r"]
        );
        assert!(inserted_change(
            EditableLabelSpec::new()
                .with_value("Studio")
                .with_max_length(6),
            " rig",
        )
        .is_empty());
    }
}
