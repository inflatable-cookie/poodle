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
    ColorValue, CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, Node, NodeRole,
    StylePatch, TextChangeHandler,
};
use poodle_specs::{
    ControlDensity, ControlSize, EditableLabelActivation, EditableLabelSpec, EditableLabelVariant,
};

use crate::color::{mix_srgb, with_alpha, TRANSPARENT};
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

/// The whole-value channel's clamp. The edit transitions enforce the limit
/// themselves; this is for `on_text_change`, where a backend hands down a
/// complete replacement value rather than an edit.
fn limit_edit_value(value: String, max_length: Option<usize>) -> String {
    let Some(max_length) = max_length else {
        return value;
    };
    value.chars().take(max_length).collect()
}

fn live_edit_state(spec: &EditableLabelSpec) -> poodle_headless::text_input::EditState {
    let (anchor, head) = spec.selection_range();
    poodle_headless::text_input::EditState { anchor, head }
}

#[derive(Default, Clone)]
pub struct EditableLabelHandlers {
    pub on_edit_start: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_change: Option<TextChangeHandler>,
    pub on_selection_change: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
    pub on_commit: Option<Arc<dyn Fn(&str, &str) + Send + Sync>>,
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_restore_display_focus: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub(crate) fn adapt_commit(
    handler: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Option<Arc<dyn Fn(&str, &str) + Send + Sync>> {
    handler.map(|handler| {
        Arc::new(move |value: &str, _previous: &str| handler(value))
            as Arc<dyn Fn(&str, &str) + Send + Sync>
    })
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
    let live = spec.live_text().to_string();
    let accessible_name = spec.resolved_accessible_name();
    let selection = spec.selection_range();

    let mut el = if spec.is_editing {
        // ── Editing mode: input node seeded with the session draft ────
        let edit_border = ctx.theme().resolve_color(spec.edit_border_token());
        let surface_bg = ctx.theme().resolve_color(spec.fill_token());
        let selection_fill = ctx.theme().resolve_color("color.accent.base");

        let mut input = Node::input(live.clone(), spec.placeholder.clone().unwrap_or_default());
        input.a11y.role = Some(NodeRole::TextInput);
        input.a11y.label = Some(accessible_name.clone());
        input = input.with_caret(
            selection,
            text_color,
            with_alpha(selection_fill, selection_fill.3 * 0.30),
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
        if spec.request_focus {
            input.interaction.request_focus = true;
        }
        if !spec.is_disabled {
            let live_for_change = live.clone();
            if let Some(change) = handlers.on_change.clone() {
                let max_length = spec.max_length;
                input.interaction.on_text_change = Some(Arc::new(move |next: &str| {
                    let next = limit_edit_value(next.to_string(), max_length);
                    if next != live_for_change {
                        change(&next);
                    }
                }));
            }
            let cancel = handlers.on_cancel.clone();
            let restore = handlers.on_restore_display_focus.clone();
            if cancel.is_some() || restore.is_some() {
                input.interaction.on_cancel = Some(Arc::new(move || {
                    if let Some(cancel) = &cancel {
                        cancel();
                    }
                    if let Some(restore) = &restore {
                        restore();
                    }
                }));
            }
            input.style.focus = Some(StylePatch {
                background: None,
                border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
                text_color: None,
                opacity: None,
            });
            let report_edit = {
                let change = handlers.on_change.clone();
                let select = handlers.on_selection_change.clone();
                let live = live.clone();
                move |outcome: poodle_headless::text_input::EditOutcome| {
                    if let Some(next) = outcome.value.clone() {
                        if next != live {
                            if let Some(change) = &change {
                                change(&next);
                            }
                        }
                    }
                    let moved = (outcome.state.anchor, outcome.state.head);
                    if moved != selection {
                        if let Some(select) = &select {
                            select(moved.0, moved.1);
                        }
                    }
                }
            };
            if handlers.on_change.is_some() || handlers.on_selection_change.is_some() {
                let live_insert = live.clone();
                let max_length = spec.max_length;
                let state = live_edit_state(spec);
                let report = report_edit.clone();
                input.interaction.on_edit_insert = Some(Arc::new(move |text: &str| {
                    report(poodle_headless::text_input::insert_transition(
                        &live_insert,
                        state,
                        text,
                        max_length,
                    ));
                }));
                let live_key = live.clone();
                let report = report_edit;
                input.interaction.on_edit_key = Some(Arc::new(move |key, mods| {
                    let key = if key == "space" {
                        " ".to_string()
                    } else if mods.shift && key.chars().count() == 1 {
                        key.to_uppercase()
                    } else {
                        key.to_string()
                    };
                    let Some(outcome) = poodle_headless::text_input::edit_transition(
                        &live_key, state, &key, false, mods.accel, max_length,
                    ) else {
                        return;
                    };
                    report(outcome);
                }));
            }
            if let Some(handler) = handlers.on_commit.clone() {
                let committed = spec.value.clone();
                let draft = live.clone();
                let restore = handlers.on_restore_display_focus.clone();
                let submit_handler = Arc::clone(&handler);
                input.interaction.on_submit = Some(Arc::new(move || {
                    let value = poodle_headless::edit::trim_editable_label(&draft);
                    submit_handler(&value, &committed);
                    if let Some(restore) = &restore {
                        restore();
                    }
                }));
                let blur_committed = spec.value.clone();
                let blur_draft = live.clone();
                input.interaction.on_focus_change = Some(Arc::new(move |focused: bool| {
                    if !focused {
                        let value = poodle_headless::edit::trim_editable_label(&blur_draft);
                        handler(&value, &blur_committed);
                    }
                }));
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
        row.a11y.role = Some(NodeRole::Button);
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
            row.style.descriptor.cursor = CursorHint::Text;
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
            s.descriptor.cursor = CursorHint::Text;
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
        el.style.descriptor.cursor = CursorHint::NotAllowed;
        el.interaction.disabled = true;
    } else if !spec.is_editing {
        el.interaction.focusable = true;
        el.a11y.tab_index = Some(0);
        el.style.focus_ring = Some(FocusRing {
            color: ctx.theme().resolve_color(spec.focus_ring_color_token()),
            width: focus_width,
            offset: rem_to_px(0.0625),
        });
        if spec.request_focus {
            el.interaction.request_focus = true;
        }
        if let Some(handler) = handlers.on_edit_start {
            match spec.activation_mode {
                EditableLabelActivation::EnterOrSpace => {
                    el.interaction.on_activate = Some(handler);
                }
                EditableLabelActivation::DoubleClick => {
                    let key_handler = Arc::clone(&handler);
                    el.interaction.on_double_activate = Some(Arc::new(move |_mods| handler()));
                    el.interaction.on_edit_key = Some(Arc::new(move |key, _mods| {
                        if key == "enter" || key == "space" {
                            key_handler();
                        }
                    }));
                }
                EditableLabelActivation::Programmatic => {}
            }
        }
    }

    el.a11y.label = Some(accessible_name);
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
        let spec = spec.with_editing(true);
        let len = spec.live_text().chars().count();
        let node = editable_label_with_handlers(
            &spec.with_selection(len, len),
            &ctx,
            EditableLabelHandlers {
                on_change: Some(Arc::new(move |value| {
                    sink.lock().unwrap().push(value.to_string());
                })),
                ..EditableLabelHandlers::default()
            },
        );
        (node.interaction.on_edit_key.as_ref().expect("key handler"))(key, modifiers);
        let recorded = values.lock().unwrap().clone();
        recorded
    }

    fn inserted_change(spec: EditableLabelSpec, inserted: &str) -> Vec<String> {
        let values = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&values);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = spec.with_editing(true);
        let len = spec.live_text().chars().count();
        let node = editable_label_with_handlers(
            &spec.with_selection(len, len),
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
        let recorded = values.lock().unwrap().clone();
        recorded
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

    #[test]
    fn live_draft_paints_without_moving_committed_value() {
        let theme = theme();
        const CLEF: &str = "𝄞";
        let spec = EditableLabelSpec::new()
            .with_value("Kick")
            .with_draft_value(Some("Kicks".to_string()))
            .with_editing(true);
        let node = editable_label_with_handlers(
            &spec,
            &RenderContext::new(&theme),
            EditableLabelHandlers::default(),
        );
        match &node.kind {
            poodle_node::NodeKind::Input { value, .. } => assert_eq!(value, "Kicks"),
            _ => panic!("expected an input node"),
        }

        let mut spec = EditableLabelSpec::new()
            .with_value("Kick")
            .with_draft_value(Some(String::new()))
            .with_editing(true)
            .with_max_length(1)
            .with_selection(0, 0);
        let first = inserted_change(spec.clone(), CLEF);
        assert_eq!(first, [CLEF]);
        spec = spec
            .with_draft_value(Some(CLEF.to_string()))
            .with_selection(1, 1);
        assert!(inserted_change(spec, "A").is_empty());
    }

    #[test]
    fn commit_reports_trimmed_value_and_committed_previous() {
        let theme = theme();
        let payload = Arc::new(Mutex::new(None));
        let sink = Arc::clone(&payload);
        let spec = EditableLabelSpec::new()
            .with_value("Kick")
            .with_draft_value(Some("\u{0085}Take\u{FEFF}".to_string()))
            .with_editing(true);
        let node = editable_label_with_handlers(
            &spec,
            &RenderContext::new(&theme),
            EditableLabelHandlers {
                on_commit: Some(Arc::new(move |value, previous| {
                    *sink.lock().unwrap() = Some((value.to_string(), previous.to_string()));
                })),
                ..EditableLabelHandlers::default()
            },
        );
        (node.interaction.on_submit.as_ref().expect("submit"))();
        assert_eq!(
            payload.lock().unwrap().clone(),
            Some(("Take".to_string(), "Kick".to_string()))
        );
    }
}
