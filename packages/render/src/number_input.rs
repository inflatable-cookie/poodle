//! NumberInput — numeric field with optional steppers and boxed affixes.
//!
//! Contract: `docs/contracts/components/number-input.md`
//! Declares one editable SpinButton value node with text/selection/focus/
//! submit/cancel/replacement channels. Value, draft, and commit effects come
//! from `poodle_headless::number_input`. The GPUI host stores draft, caret,
//! and focus between rebuilds and routes mounted dispatch through the same
//! transition results as web.

use std::sync::Arc;

use poodle_headless::number_input::{
    number_input_display_text, number_input_transition, NumberInputEffect, NumberInputEvent,
};
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeRole, StylePatch, TextChangeHandler,
};
use poodle_specs::{NumberInputSpec, ValidationState};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::{
    control_height_rem, rem_to_px, resolve_supporting_visual_size, size_font_rem,
    size_padding_x_offset_rem,
};

const SELECTION_ALPHA: f32 = 0.30;

/// Host callbacks matching the number-input machine effects, plus caret/focus
/// channels the host stores between rebuilds.
#[derive(Clone, Default)]
pub struct NumberInputHandlers {
    pub on_draft_value_change: Option<Arc<dyn Fn(Option<String>) + Send + Sync>>,
    pub on_value_change: Option<Arc<dyn Fn(Option<f64>) + Send + Sync>>,
    pub on_commit: Option<Arc<dyn Fn(Option<f64>) + Send + Sync>>,
    pub on_selection_change: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
    pub on_focus_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

fn apply_effects(effects: &[NumberInputEffect], handlers: &NumberInputHandlers) {
    for effect in effects {
        match effect {
            NumberInputEffect::EmitDraftValueChange { draft } => {
                if let Some(on_draft) = &handlers.on_draft_value_change {
                    on_draft(draft.clone());
                }
            }
            NumberInputEffect::EmitValueChange { value } => {
                if let Some(on_value) = &handlers.on_value_change {
                    on_value(*value);
                }
            }
            NumberInputEffect::EmitCommit { value } => {
                if let Some(on_commit) = &handlers.on_commit {
                    on_commit(*value);
                }
            }
        }
    }
}

fn dispatch_event(spec: &NumberInputSpec, event: NumberInputEvent, handlers: &NumberInputHandlers) {
    let (next, effects) = number_input_transition(spec.to_context(), event);
    apply_effects(&effects, handlers);
    // Step/Home/End/Enter/Escape/Blur change the visible text without going
    // through the text edit model, so the host caret has to follow the new
    // display. Raw keystroke paths report their own selection afterward and
    // overwrite this when the caret is not simply "end of field".
    let next_display = number_input_display_text(&next);
    if next_display != spec.display_text() {
        let len = next_display.chars().count();
        if let Some(on_selection) = &handlers.on_selection_change {
            on_selection(len, len);
        }
    }
}

fn report_text_edit(
    outcome: poodle_headless::text_input::EditOutcome,
    current_text: &str,
    selection: (usize, usize),
    spec: &NumberInputSpec,
    handlers: &NumberInputHandlers,
) {
    let moved = (outcome.state.anchor, outcome.state.head);
    if let Some(next) = outcome.value {
        if next != current_text {
            dispatch_event(
                spec,
                NumberInputEvent::RawEdit { text: next.clone() },
                handlers,
            );
            if let Some(on_selection) = &handlers.on_selection_change {
                on_selection(moved.0, moved.1);
            }
            return;
        }
    }
    if moved != selection {
        if let Some(on_selection) = &handlers.on_selection_change {
            on_selection(moved.0, moved.1);
        }
    }
}

/// A boxed prefix/suffix affix: bordered box with surface bg + muted text,
/// full control height.
#[expect(
    clippy::too_many_arguments,
    reason = "affix rendering keeps resolved token metrics explicit"
)]
fn affix_box(
    text: &str,
    text_color: ColorValue,
    bg: ColorValue,
    border_color: ColorValue,
    border_width: f32,
    font_size: f32,
    pad_x: f32,
    height: f32,
) -> Node {
    let mut el = Node::container();
    {
        let s = &mut el.style;
        // GPUI's `.h_full()` fills the value row's cross axis; using a fixed
        // height here leaves the border-box one pixel outside that row in the
        // node backend and produces a doubled vertical rule.
        let _ = height;
        s.fill_height = true;
        s.descriptor.background = Some(bg);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border_color;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    let mut label = Node::text(text);
    label.style.descriptor.text_color = Some(text_color);
    label.style.text_size = Some(font_size);
    el.child(label)
}

pub fn number_input(
    spec: &NumberInputSpec,
    ctx: &RenderContext<'_>,
    handlers: NumberInputHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    // The old GPUI component resolves this token through the active theme
    // density (the visual axis), then applies the semantic size offset. The
    // spec density is for callers' standalone contracts and is not the
    // preview theme's density override.
    let pad_x = ctx.theme().resolve_space(spec.horizontal_padding_token())
        + rem_to_px(size_padding_x_offset_rem(effective_size));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(
        effective_size,
    )));
    let border_width = ctx.theme().resolve_border_width(spec.border_width_token());

    let border = ctx.theme().resolve_color(spec.border_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    let fill = ctx.theme().resolve_color(spec.fill_token());
    let text_color = ctx.theme().resolve_color(spec.text_color_token());
    let placeholder_color = ctx.theme().resolve_color(spec.placeholder_color_token());
    let stepper_icon_color = ctx.theme().resolve_color(spec.stepper_icon_color_token());
    // Boxed-affix chrome (border-default box + surface bg + muted text).
    let affix_text = ctx.theme().resolve_color(spec.affix_text_token());
    let affix_bg = ctx.theme().resolve_color(spec.affix_fill_token());
    let affix_border = ctx.theme().resolve_color(spec.affix_border_token());
    let disabled_opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
    let selection_fill = ctx.theme().resolve_color("color.accent.base");

    let invalid_draft = spec.is_invalid_draft();
    let effective_border = match (invalid_draft, spec.validation_state) {
        (true, _) | (_, ValidationState::Invalid) => {
            ctx.theme().resolve_color("color.status.danger")
        }
        (_, ValidationState::Valid) => ctx.theme().resolve_color("color.status.success"),
        (_, ValidationState::Pending) => ctx.theme().resolve_color("color.accent.base"),
        (_, ValidationState::None) => border,
    };

    let display_text = spec.display_text();
    let showing_placeholder = display_text.is_empty();
    let visible_text = if showing_placeholder {
        spec.placeholder.as_deref().unwrap_or("").to_string()
    } else {
        display_text.clone()
    };
    let display_color = if showing_placeholder {
        placeholder_color
    } else {
        text_color
    };

    let field_id = match spec.id.as_deref() {
        Some(id) => format!("poodle-number-input-{id}"),
        None => {
            let descriptor = [
                spec.aria_label.as_deref(),
                spec.placeholder.as_deref(),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join("-");
            if descriptor.is_empty() {
                "poodle-number-input".to_string()
            } else {
                format!("poodle-number-input-{descriptor}")
            }
        }
    };

    // ── Root container ─────────────────────────────────────────────────────
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = effective_border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.fill_width = true;
    }

    let stepper_bg = ctx.theme().resolve_color(spec.stepper_fill_token());
    let stepper_bg = with_alpha(stepper_bg, stepper_bg.3 * 0.88);
    let stepper = |icon: &str,
                   label: &str,
                   id: &str,
                   blocked: bool,
                   direction: i32|
     -> Node {
        let mut btn = Node::button("");
        btn.a11y.label = Some(label.to_string());
        btn.id = Some(id.to_string());
        {
            let s = &mut btn.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.flex_grow = Some(1.0);
            s.descriptor.background = Some(stepper_bg);
            let inner_radius = (radius - rem_to_px(0.125)).max(0.0);
            s.descriptor.corner_radii.top_left = inner_radius;
            s.descriptor.corner_radii.top_right = inner_radius;
            s.descriptor.corner_radii.bottom_right = inner_radius;
            s.descriptor.corner_radii.bottom_left = inner_radius;
            let pad = &mut s.descriptor.layout.spacing.padding;
            s.descriptor.cursor = CursorHint::Pointer;
            pad.top = 0.0;
            pad.bottom = 0.0;
        }
        // Pointer-activatable only: the field root owns the one focus
        // treatment. A focusable stepper would steal the root handle on click
        // and fire blur/commit before the step landed.
        btn.interaction.focusable = false;
        let mut glyph = Node::icon(icon, icon_size);
        glyph.style.descriptor.text_color = Some(stepper_icon_color);
        let mut btn = btn.child(glyph);
        if blocked {
            btn.style.descriptor.opacity = disabled_opacity;
            btn.interaction.disabled = true;
        } else {
            let spec = spec.clone();
            let handlers = handlers.clone();
            btn.interaction.on_activate = Some(Arc::new(move || {
                dispatch_event(&spec, NumberInputEvent::Step { direction }, &handlers);
            }));
        }
        btn
    };

    // ── Value row ──────────────────────────────────────────────────────────
    let mut value_row = Node::container();
    {
        let s = &mut value_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.5);
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
        s.min_width = Some(0.0);
    }

    if let Some(prefix) = &spec.prefix {
        value_row = value_row.child(affix_box(
            prefix,
            affix_text,
            affix_bg,
            affix_border,
            border_width,
            font_size,
            pad_x,
            height,
        ));
    }

    // Editable value node (not a static label).
    let mut value = Node::text(&visible_text);
    value.id = Some(format!("{field_id}-value"));
    {
        let s = &mut value.style;
        s.descriptor.text_color = Some(display_color);
        s.text_size = Some(font_size);
        s.line_height = Some(1.4);
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
        s.min_width = Some(0.0);
        s.no_wrap = true;
        s.text_ellipsis = true;
    }

    let mut value_caret = None;
    let mut value_select: Option<
        Arc<dyn Fn(usize, usize, poodle_node::SelectGranularity) + Send + Sync>,
    > = None;

    if !spec.is_disabled {
        let caret_color = if spec.is_read_only {
            with_alpha(text_color, 0.0)
        } else {
            text_color
        };
        value = value.with_caret(
            spec.selection_range(),
            caret_color,
            with_alpha(selection_fill, selection_fill.3 * SELECTION_ALPHA),
        );
        if let Some(caret) = &mut value.caret {
            caret.showing_placeholder = showing_placeholder;
        }
        value_caret = value.caret;

        if let Some(on_selection_change) = handlers.on_selection_change.clone() {
            let text = display_text.clone();
            let handler: Arc<dyn Fn(usize, usize, poodle_node::SelectGranularity) + Send + Sync> =
                Arc::new(
                    move |start: usize, end: usize, granularity: poodle_node::SelectGranularity| {
                        let (start, end) = match granularity {
                            poodle_node::SelectGranularity::Character => (start, end),
                            poodle_node::SelectGranularity::Word => {
                                let (a, _) =
                                    poodle_headless::text_input::word_range_at(&text, start);
                                let (_, b) = poodle_headless::text_input::word_range_at(&text, end);
                                (a, b)
                            }
                            poodle_node::SelectGranularity::Line => (0, text.chars().count()),
                        };
                        on_selection_change(start, end);
                    },
                );
            value.interaction.on_select_range = Some(Arc::clone(&handler));
            value_select = Some(handler);
        }

        if !spec.is_read_only {
            let text = display_text.clone();
            let (start, end) = spec.selection_range();
            let spec_owned = spec.clone();
            let handlers_owned = handlers.clone();
            value.interaction.on_edit_insert = Some(Arc::new(move |inserted: &str| {
                let outcome = poodle_headless::text_input::insert_transition(
                    &text,
                    poodle_headless::text_input::EditState {
                        anchor: start,
                        head: end,
                    },
                    inserted,
                    None,
                );
                report_text_edit(outcome, &text, (start, end), &spec_owned, &handlers_owned);
            }));
        }
    }
    value_row = value_row.child(value);

    if let Some(suffix) = &spec.suffix {
        value_row = value_row.child(affix_box(
            suffix,
            affix_text,
            affix_bg,
            affix_border,
            border_width,
            font_size,
            pad_x,
            height,
        ));
    }

    el = el.child(value_row);

    // ── Vertical steppers (only when enabled) ─────────────────────────────
    if spec.show_steppers {
        let stepper_width = rem_to_px(1.25);
        let mut steppers = Node::container();
        {
            let s = &mut steppers.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.width = LayoutSizing::Fixed(stepper_width);
            s.fill_height = true;
            s.descriptor.layout.spacing.padding.top = 1.0;
            s.descriptor.layout.spacing.padding.right = 1.0;
            s.descriptor.layout.spacing.padding.bottom = 1.0;
            s.descriptor.layout.spacing.padding.left = 1.0;
        }
        steppers = steppers
            .child(stepper(
                "plus",
                "Increment",
                &format!("{field_id}-inc"),
                !spec.can_step(1),
                1,
            ))
            .child(stepper(
                "minus",
                "Decrement",
                &format!("{field_id}-dec"),
                !spec.can_step(-1),
                -1,
            ));
        el = el.child(steppers);
    }

    el.id = Some(field_id);
    el.interaction.focusable = true;

    if !spec.is_disabled {
        el.style.focus = Some(StylePatch {
            background: None,
            border_color: Some(ctx.theme().resolve_color(spec.focus_ring_color_token())),
            text_color: None,
            opacity: None,
        });
        el.caret = value_caret;
        el.interaction.on_select_range = value_select;

        let on_focus = handlers.on_focus_change.clone();
        let spec_blur = spec.clone();
        let handlers_blur = handlers.clone();
        el.interaction.on_focus_change = Some(Arc::new(move |focused: bool| {
            if let Some(on_focus) = &on_focus {
                on_focus(focused);
            }
            if !focused {
                dispatch_event(&spec_blur, NumberInputEvent::Blur, &handlers_blur);
            }
        }));

        if !spec.is_read_only {
            let text = display_text.clone();
            let (start, end) = spec.selection_range();
            let spec_keys = spec.clone();
            let handlers_keys = handlers.clone();
            el.interaction.on_edit_key = Some(Arc::new(move |key: &str, mods| {
                let event = match key {
                    "up" => Some(NumberInputEvent::Step { direction: 1 }),
                    "down" => Some(NumberInputEvent::Step { direction: -1 }),
                    "home" => Some(NumberInputEvent::Home),
                    "end" => Some(NumberInputEvent::End),
                    _ => None,
                };
                if let Some(event) = event {
                    dispatch_event(&spec_keys, event, &handlers_keys);
                    return;
                }
                let state = poodle_headless::text_input::EditState {
                    anchor: start,
                    head: end,
                };
                let Some(outcome) = poodle_headless::text_input::edit_transition(
                    &text, state, key, mods.shift, mods.accel, None,
                ) else {
                    return;
                };
                report_text_edit(outcome, &text, (start, end), &spec_keys, &handlers_keys);
            }));

            let text = display_text.clone();
            let (start, end) = spec.selection_range();
            let spec_insert = spec.clone();
            let handlers_insert = handlers.clone();
            el.interaction.on_edit_insert = Some(Arc::new(move |inserted: &str| {
                let outcome = poodle_headless::text_input::insert_transition(
                    &text,
                    poodle_headless::text_input::EditState {
                        anchor: start,
                        head: end,
                    },
                    inserted,
                    None,
                );
                report_text_edit(outcome, &text, (start, end), &spec_insert, &handlers_insert);
            }));

            let replacement: TextChangeHandler = {
                let spec_replace = spec.clone();
                let handlers_replace = handlers.clone();
                Arc::new(move |text: &str| {
                    dispatch_event(
                        &spec_replace,
                        NumberInputEvent::RawEdit {
                            text: text.to_string(),
                        },
                        &handlers_replace,
                    );
                })
            };
            el.interaction.on_text_change = Some(replacement);

            let spec_submit = spec.clone();
            let handlers_submit = handlers.clone();
            el.interaction.on_submit = Some(Arc::new(move || {
                dispatch_event(&spec_submit, NumberInputEvent::Enter, &handlers_submit);
            }));

            let spec_cancel = spec.clone();
            let handlers_cancel = handlers.clone();
            el.interaction.on_cancel = Some(Arc::new(move || {
                dispatch_event(&spec_cancel, NumberInputEvent::Escape, &handlers_cancel);
            }));
        }
    }

    if spec.is_disabled {
        el.style.descriptor.opacity = disabled_opacity;
        el.interaction.disabled = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el.a11y.role = Some(NodeRole::SpinButton);
    if let Some(now) = spec.accessible_value_now() {
        el.a11y.value = Some(now);
        el.a11y.value_text = Some(spec.display_text());
    }
    el.a11y.value_min = spec.min;
    el.a11y.value_max = spec.max;
    let unresolved_invalid = invalid_draft || spec.validation_state == ValidationState::Invalid;
    if unresolved_invalid {
        el.a11y.invalid = Some(true);
    }
    if spec.validation_state == ValidationState::Pending {
        el.a11y.busy = Some(true);
    }
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::RenderContext;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn editable_spin_button_declares_edit_channels() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = number_input(
            &NumberInputSpec::new(Some(5.0)).with_aria_label("Qty"),
            &ctx,
            NumberInputHandlers::default(),
        );
        assert_eq!(node.a11y.role, Some(NodeRole::SpinButton));
        assert_eq!(node.a11y.value, Some(5.0));
        assert_eq!(node.a11y.label.as_deref(), Some("Qty"));
        assert!(node.interaction.focusable);
        assert!(node.interaction.on_text_change.is_some());
        assert!(node.interaction.on_edit_key.is_some());
        assert!(node.interaction.on_edit_insert.is_some());
        assert!(node.interaction.on_submit.is_some());
        assert!(node.interaction.on_cancel.is_some());
        assert!(node.interaction.on_focus_change.is_some());
        assert!(node.caret.is_some());
    }

    #[test]
    fn empty_committed_omits_valuenow_and_shows_placeholder() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = number_input(
            &NumberInputSpec::new(None).with_placeholder("n"),
            &ctx,
            NumberInputHandlers::default(),
        );
        assert_eq!(node.a11y.value, None);
        assert!(node.interaction.on_text_change.is_some());
    }

    #[test]
    fn bounds_project_to_valuemin_max() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = number_input(
            &NumberInputSpec::new(Some(3.0))
                .with_min(Some(0.0))
                .with_max(Some(10.0)),
            &ctx,
            NumberInputHandlers::default(),
        );
        assert_eq!(node.a11y.value_min, Some(0.0));
        assert_eq!(node.a11y.value_max, Some(10.0));
    }

    #[test]
    fn read_only_keeps_focus_but_drops_mutation_channels() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = number_input(
            &NumberInputSpec::new(Some(1.0)).with_read_only(true),
            &ctx,
            NumberInputHandlers::default(),
        );
        assert!(node.interaction.focusable);
        assert!(node.interaction.on_text_change.is_none());
        assert!(node.interaction.on_edit_key.is_none());
        assert!(node.caret.is_some());
    }

    #[test]
    fn steppers_wire_machine_not_increment_callbacks() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let committed = Arc::new(std::sync::Mutex::new(None));
        let committed_c = Arc::clone(&committed);
        let node = number_input(
            &NumberInputSpec::new(Some(1.0))
                .with_min(Some(0.0))
                .with_max(Some(10.0))
                .with_steppers(true),
            &ctx,
            NumberInputHandlers {
                on_value_change: Some(Arc::new(move |v| {
                    *committed_c.lock().unwrap() = v;
                })),
                on_commit: Some(Arc::new(|_| {})),
                ..NumberInputHandlers::default()
            },
        );
        let steppers = &node.children[1];
        let inc = &steppers.children[0];
        assert!(!inc.interaction.focusable, "steppers stay out of focus order");
        assert!(inc.style.focus.is_none());
        assert!(node.style.focus.is_some(), "the field owns the focus treatment");
        let activate = inc.interaction.on_activate.as_ref().expect("inc active");
        activate();
        assert_eq!(*committed.lock().unwrap(), Some(2.0));
    }

    #[test]
    fn unresolved_draft_and_pending_validation_project_a11y_flags() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let invalid = number_input(
            &NumberInputSpec::new(Some(1.0))
                .with_draft_value(Some("-".into()))
                .with_aria_label("Qty"),
            &ctx,
            NumberInputHandlers::default(),
        );
        assert_eq!(invalid.a11y.invalid, Some(true));
        assert_eq!(invalid.a11y.value, None);

        let busy = number_input(
            &NumberInputSpec::new(Some(1.0)).with_validation_state(ValidationState::Pending),
            &ctx,
            NumberInputHandlers::default(),
        );
        assert_eq!(busy.a11y.busy, Some(true));
    }
}
