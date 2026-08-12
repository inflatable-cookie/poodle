//! FilterBuilder — filter clauses behind a trigger. Last of the big three.
//!
//! Contract: `docs/contracts/components/filter-builder.md`
//! Ported from: `packages/jetstream/components/src/filter_builder.rs`.
//!
//! Events name the intent so the host applies it to the state it holds:
//! remove/reset on the pills, toggle on the opener, picker_toggle +
//! field_pick / operator_change / operand_change / combinator_change +
//! commit/cancel in the open panel. Typed operands (text, number, range)
//! stay host-side.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole,
};
use poodle_specs::{
    ButtonSpec, ButtonVariant, CheckboxSpec, ChoiceOption, ControlDensity, ControlSize,
    FilterBuilderPicker, FilterBuilderSpec, FilterCombinator, FilterFieldDefinition,
    FilterFieldKind, FilterOperand, FilterOperandKind, IconButtonSpec, NumberInputSpec,
    SegmentedControlSpec, SelectSpec, TextInputSpec,
};

use crate::button::button;
use crate::checkbox::checkbox;
use crate::color::mix_srgb;
use crate::icon_button::icon_button;
use crate::number_input::{number_input, NumberInputHandlers};
use crate::presentation::{rem_to_px, resolve_semantic_size, size_padding_x_offset_rem};
use crate::segmented_control::segmented_control;
use crate::select::{select, SelectHandlers};
use crate::text_input::text_input;

/// The full intent surface, threaded through as one bundle.
#[derive(Default, Clone)]
pub struct FilterBuilderHandlers {
    pub on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_reset: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_picker_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_field_pick: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_operator_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_operand_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_combinator_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_commit: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

fn row(gap: f32) -> Node {
    let mut n = Node::container();
    let s = &mut n.style;
    s.descriptor.layout.direction = LayoutDirection::Row;
    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    s.descriptor.layout.spacing.gap = gap;
    n
}

/// Operand editor for a draft clause, chosen by the operator's operand kind.
#[expect(
    clippy::too_many_arguments,
    reason = "operand rendering keeps contract state and handlers explicit"
)]
fn operand_editor(
    theme: &dyn ThemeProvider,
    field: &FilterFieldDefinition,
    operand_kind: FilterOperandKind,
    operand: &FilterOperand,
    size: ControlSize,
    density: ControlDensity,
    disabled: bool,
    operand_picker_open: bool,
    handlers: &FilterBuilderHandlers,
) -> Node {
    match operand_kind {
        FilterOperandKind::Boolean => {
            let on = matches!(operand, FilterOperand::Boolean(true));
            let mut s = SegmentedControlSpec::new(vec![
                ChoiceOption::new("true", "True"),
                ChoiceOption::new("false", "False"),
            ])
            .with_size(size)
            .with_density(density);
            s.value = Some(if on { "true" } else { "false" }.to_string());
            s.is_disabled = disabled;
            segmented_control(&s, theme, handlers.on_operand_change.clone())
        }
        FilterOperandKind::Text => {
            let value = match operand {
                FilterOperand::Text(t) => t.clone(),
                _ => String::new(),
            };
            text_input(
                &TextInputSpec::new()
                    .with_value(value)
                    // The clause's own field/operator selects sit beside it,
                    // not as its label.
                    .with_aria_label("Filter value")
                    .with_size(size)
                    .with_density(density)
                    .with_disabled(disabled),
                theme,
                None,
            )
        }
        FilterOperandKind::Number => {
            let value = match operand {
                FilterOperand::Number(n) if n.is_finite() => *n,
                _ => 0.0,
            };
            number_input(
                &NumberInputSpec::new(value)
                    .with_size(size)
                    .with_density(density)
                    .with_disabled(disabled),
                theme,
                NumberInputHandlers::default(),
            )
        }
        FilterOperandKind::Options => {
            let selected: Vec<String> = match operand {
                FilterOperand::Options(v) => v.clone(),
                _ => Vec::new(),
            };
            if field.kind == FilterFieldKind::Enum {
                let options: Vec<ChoiceOption> = field
                    .options
                    .iter()
                    .map(|o| ChoiceOption::new(o.value.clone(), o.label.clone()))
                    .collect();
                let mut s = SelectSpec::new(options)
                    .with_placeholder("Select…")
                    .with_size(size)
                    .with_density(density);
                if let Some(v) = selected.first() {
                    s = s.with_value(v.clone());
                }
                s.is_disabled = disabled;
                s = s.with_open(operand_picker_open);
                let toggle = handlers.on_picker_toggle.as_ref().map(|handler| {
                    let handler = Arc::clone(handler);
                    Arc::new(move || handler("operand")) as Arc<dyn Fn() + Send + Sync>
                });
                select(
                    &s,
                    theme,
                    &SelectHandlers {
                        toggle,
                        change: handlers.on_operand_change.clone(),
                        clear: None,
                    },
                )
            } else {
                let mut list = Node::container();
                {
                    let s = &mut list.style;
                    s.descriptor.layout.direction = LayoutDirection::Column;
                    s.descriptor.layout.spacing.gap = rem_to_px(0.25);
                }
                for option in field.options.iter() {
                    let on_change = handlers.on_operand_change.as_ref().map(|handler| {
                        let handler = Arc::clone(handler);
                        let value = option.value.clone();
                        Arc::new(move |_next: bool| handler(&value))
                            as Arc<dyn Fn(bool) + Send + Sync>
                    });
                    list = list.child(checkbox(
                        &CheckboxSpec::new()
                            .with_label(option.label.clone())
                            .with_checked(selected.contains(&option.value))
                            .with_size(size)
                            .with_disabled(disabled || option.is_disabled),
                        theme,
                        on_change,
                    ));
                }
                list
            }
        }
        FilterOperandKind::Range => {
            let (min, max) = match operand {
                FilterOperand::Range { min, max } => (*min, *max),
                _ => (None, None),
            };
            let sep_color = theme.resolve_color("color.text.secondary");
            let mut sep = Node::text("–");
            sep.style.descriptor.text_color = Some(sep_color);
            row(rem_to_px(0.375))
                .child(number_input(
                    &NumberInputSpec::new(min.unwrap_or(0.0))
                        .with_size(size)
                        .with_density(density)
                        .with_disabled(disabled),
                    theme,
                    NumberInputHandlers::default(),
                ))
                .child(sep)
                .child(number_input(
                    &NumberInputSpec::new(max.unwrap_or(0.0))
                        .with_size(size)
                        .with_density(density)
                        .with_disabled(disabled),
                    theme,
                    NumberInputHandlers::default(),
                ))
        }
        FilterOperandKind::None => {
            let mut empty = Node::container();
            // Explicit Row (see switch.rs).
            empty.style.descriptor.layout.direction = LayoutDirection::Row;
            empty
        }
    }
}

pub fn filter_builder(
    spec: &FilterBuilderSpec,
    theme: &dyn ThemeProvider,
    handlers: &FilterBuilderHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Size table (contract §8) ──────────────────────────────────────────
    let trigger_h = rem_to_px(match effective_size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.25,
    });
    let label_font = rem_to_px(match effective_size {
        ControlSize::Xs => 0.5625,
        ControlSize::Sm => 0.625,
        ControlSize::Md => 0.75,
        ControlSize::Lg => 0.8125,
        ControlSize::Xl => 0.875,
    });
    let summary_font = rem_to_px(match effective_size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.8125,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    });
    let trigger_pad_x = rem_to_px(match effective_size {
        ControlSize::Xs => 0.5,
        ControlSize::Lg => 1.0,
        ControlSize::Xl => 1.125,
        _ => 0.75 + size_padding_x_offset_rem(effective_size),
    });
    let trigger_gap = rem_to_px(match spec.density {
        ControlDensity::Compact => 0.375,
        ControlDensity::Default => 0.5,
        ControlDensity::Comfortable => 0.625,
    });

    let root_gap = rem_to_px(0.375);
    let panel_gap = rem_to_px(0.5);

    // ── Colors ────────────────────────────────────────────────────────────
    let text_primary = theme.resolve_color(spec.field_text_token());
    let text_secondary = theme.resolve_color(spec.label_color_token());
    let muted = theme.resolve_color(spec.muted_color_token());
    let border = theme.resolve_color(spec.field_border_token());
    let item_border = theme.resolve_color(spec.item_border_token());
    let surface = theme.resolve_color(spec.field_fill_token());
    let elevated = theme.resolve_color(spec.field_hover_fill_token());
    let accent = theme.resolve_color(spec.count_fill_token());
    let accent_text = theme.resolve_color(spec.count_text_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let surface_radius = theme.resolve_radius(spec.surface_radius_token());
    let item_bg = mix_srgb(surface, elevated, 0.90);

    // ── Single bordered field: opener + inline pills + reset ──────────────
    let mut field_el = Node::container();
    {
        let s = &mut field_el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = root_gap;
        s.min_width = Some(0.0);
        s.min_height = Some(trigger_h);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = trigger_pad_x;
        pad.right = trigger_pad_x;
        pad.top = rem_to_px(0.25);
        pad.bottom = rem_to_px(0.25);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.background = Some(surface);
    }
    all_corners(&mut field_el, radius);

    // Opener (borderless): label + optional summary + chevron.
    let mut opener = row(trigger_gap);
    if !spec.is_compact {
        let mut label = Node::text(spec.opener_label());
        label.style.descriptor.text_color = Some(if spec.combinator_visible() {
            text_primary
        } else {
            text_secondary
        });
        label.style.text_size = Some(label_font);
        label.style.text_weight = Some(500);
        label.style.letter_spacing_em = Some(0.05);
        opener = opener.child(label);
    }
    if !(spec.show_pills && spec.has_value()) {
        let is_placeholder = !spec.has_value();
        let mut summary = Node::text(spec.summary_text());
        summary.style.descriptor.text_color =
            Some(if is_placeholder { muted } else { text_primary });
        summary.style.text_size = Some(summary_font);
        summary.style.text_ellipsis = true;
        summary.style.no_wrap = true;
        opener = opener.child(summary);
    }
    let mut chevron = Node::icon("chevron-down", summary_font);
    chevron.style.descriptor.text_color = Some(text_secondary);
    let mut opener = opener.child(chevron);
    if let (false, Some(handler)) = (spec.is_disabled, &handlers.on_toggle) {
        let handler = Arc::clone(handler);
        opener.style.descriptor.cursor = CursorHint::Pointer;
        opener.interaction.on_activate = Some(Arc::new(move || handler()));
    }
    let mut field_el = field_el.child(opener);

    // Inline clause pills.
    if spec.show_pills && spec.has_value() {
        for clause in spec.value.clauses.iter() {
            let label_text = spec.clause_label(clause);
            let mut pill = row(rem_to_px(0.375));
            {
                let s = &mut pill.style;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.5);
                pad.right = rem_to_px(0.375);
                s.min_height = Some(rem_to_px(1.5));
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = item_border;
                s.descriptor.background = Some(item_bg);
            }
            all_corners(&mut pill, radius);
            let mut label = Node::text(&label_text);
            label.style.descriptor.text_color = Some(text_primary);
            label.style.text_size = Some(rem_to_px(0.75));
            // Compact remove glyph (not a full 1.5rem icon button).
            let mut x = Node::icon("x", rem_to_px(0.75));
            x.style.descriptor.text_color = Some(muted);
            let mut pill = pill.child(label).child(x);

            if let (false, Some(handler)) = (spec.is_disabled, &handlers.on_remove) {
                let handler = Arc::clone(handler);
                let id = clause.id.clone();
                pill.style.descriptor.cursor = CursorHint::Pointer;
                pill.interaction.on_activate = Some(Arc::new(move || handler(&id)));
            }

            field_el = field_el.child(pill);
        }
    }

    // Trailing controls (count badge + single clear-all).
    if spec.has_value() && (spec.show_pills || spec.show_clear_button) {
        let mut trailing = row(rem_to_px(0.375));
        {
            let s = &mut trailing.style;
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
        }

        if spec.show_pills {
            let mut count = Node::text(format!("{}", spec.active_count()));
            {
                let s = &mut count.style;
                s.min_width = Some(rem_to_px(1.125));
                s.min_height = Some(rem_to_px(1.125));
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.3125);
                pad.right = rem_to_px(0.3125);
                s.flex_none = true;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.descriptor.background = Some(accent);
                s.descriptor.text_color = Some(accent_text);
                s.text_size = Some(rem_to_px(0.6875));
                s.text_weight = Some(600);
            }
            all_corners(&mut count, rem_to_px(0.5625));
            trailing = trailing.child(count);
        }

        if spec.show_clear_button {
            trailing = trailing.child(icon_button(
                &IconButtonSpec::new()
                    .with_icon("x")
                    .with_aria_label("Clear filters")
                    .with_variant(ButtonVariant::Ghost)
                    .with_size(effective_size)
                    .with_disabled(spec.is_disabled),
                theme,
                handlers.on_reset.clone(),
            ));
        }

        field_el = field_el.child(trailing);
    }

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
        s.min_width = Some(0.0);
    }
    let mut root = root.child(field_el);

    // ── Dialog surface (rendered inline when open) ────────────────────────
    if spec.is_open {
        // Contract: the open overlay panel is a `dialog`.
        let mut panel = Node::container();
        panel.a11y.role = Some(NodeRole::Dialog);
        {
            let s = &mut panel.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = panel_gap;
        }

        // Combinator (2+ clauses) — two-option indicator.
        if spec.combinator_visible() {
            let is_and = spec.value.combinator == FilterCombinator::And;
            let mk = |text: &str, selected: bool, combinator: &'static str| -> Node {
                let mut segment = Node::text(text);
                {
                    let s = &mut segment.style;
                    s.descriptor.layout.width = LayoutSizing::Grow;
                    let pad = &mut s.descriptor.layout.spacing.padding;
                    pad.left = rem_to_px(0.5);
                    pad.right = rem_to_px(0.5);
                    pad.top = rem_to_px(0.25);
                    pad.bottom = rem_to_px(0.25);
                    s.descriptor.background = Some(if selected { accent } else { surface });
                    s.descriptor.text_color = Some(if selected {
                        accent_text
                    } else {
                        text_secondary
                    });
                    s.text_size = Some(rem_to_px(0.75));
                }
                all_corners(&mut segment, radius);
                if let (false, Some(handler)) = (spec.is_disabled, &handlers.on_combinator_change) {
                    let handler = Arc::clone(handler);
                    segment.style.descriptor.cursor = CursorHint::Pointer;
                    segment.interaction.on_activate = Some(Arc::new(move || handler(combinator)));
                }
                segment
            };
            let mut combi = Node::container();
            {
                let s = &mut combi.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.spacing.gap = rem_to_px(0.25);
            }
            panel = panel.child(combi.child(mk("Match all", is_and, "and")).child(mk(
                "Match any",
                !is_and,
                "or",
            )));
        }

        // Draft editor (adding or editing a clause).
        if let (Some(draft), Some(field)) = (spec.draft.as_ref(), spec.draft_field()) {
            let operators = field.resolved_operators();
            let mut editor = Node::container();
            {
                let s = &mut editor.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = rem_to_px(0.5);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.5);
                pad.right = rem_to_px(0.5);
                pad.top = rem_to_px(0.5);
                pad.bottom = rem_to_px(0.5);
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = item_border;
                s.descriptor.background = Some(item_bg);
            }
            all_corners(&mut editor, radius);
            let mut field_label = Node::text(&field.label);
            field_label.style.descriptor.text_color = Some(text_primary);
            field_label.style.text_size = Some(rem_to_px(0.8125));
            let mut editor = editor.child(field_label);

            if operators.len() > 1 {
                let options: Vec<ChoiceOption> = operators
                    .iter()
                    .map(|op| ChoiceOption::new(op.key.clone(), op.label.clone()))
                    .collect();
                let mut op_spec = SelectSpec::new(options)
                    .with_value(draft.operator.clone())
                    .with_size(effective_size)
                    .with_density(spec.density);
                op_spec.is_disabled = spec.is_disabled;
                op_spec =
                    op_spec.with_open(spec.open_picker == Some(FilterBuilderPicker::Operator));
                let toggle = handlers.on_picker_toggle.as_ref().map(|handler| {
                    let handler = Arc::clone(handler);
                    Arc::new(move || handler("operator")) as Arc<dyn Fn() + Send + Sync>
                });
                editor = editor.child(select(
                    &op_spec,
                    theme,
                    &SelectHandlers {
                        toggle,
                        change: handlers.on_operator_change.clone(),
                        clear: None,
                    },
                ));
            }

            if let Some(op) = field.find_operator(&draft.operator) {
                editor = editor.child(operand_editor(
                    theme,
                    field,
                    op.operand_kind,
                    &draft.operand,
                    effective_size,
                    spec.density,
                    spec.is_disabled,
                    spec.open_picker == Some(FilterBuilderPicker::Operand),
                    handlers,
                ));
            }

            let commit_label = if draft.editing_id.is_some() {
                "Update"
            } else {
                "Add"
            };
            let commit = button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label(commit_label)
                    .with_size(effective_size)
                    .with_disabled(spec.is_disabled || !spec.is_draft_valid()),
                theme,
                handlers.on_commit.clone(),
            );
            let cancel = button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Ghost)
                    .with_label("Cancel")
                    .with_size(effective_size)
                    .with_disabled(spec.is_disabled),
                theme,
                handlers.on_cancel.clone(),
            );
            editor = editor.child(row(rem_to_px(0.375)).child(commit).child(cancel));
            panel = panel.child(editor);
        }

        // Add-field Select (only when not drafting, room remains).
        if spec.show_add_row() {
            let options: Vec<ChoiceOption> = spec
                .available_fields()
                .into_iter()
                .map(|field| ChoiceOption::new(field.key.clone(), field.label.clone()))
                .collect();
            let mut select_spec = SelectSpec::new(options)
                .with_placeholder("+ Add filter")
                .with_size(effective_size)
                .with_density(spec.density);
            select_spec.aria_label = Some("Add filter field".to_string());
            select_spec.is_disabled = spec.is_disabled;
            select_spec =
                select_spec.with_open(spec.open_picker == Some(FilterBuilderPicker::AddField));
            let toggle = handlers.on_picker_toggle.as_ref().map(|handler| {
                let handler = Arc::clone(handler);
                Arc::new(move || handler("add-field")) as Arc<dyn Fn() + Send + Sync>
            });
            panel = panel.child(row(0.0).child(select(
                &select_spec,
                theme,
                &SelectHandlers {
                    toggle,
                    change: handlers.on_field_pick.clone(),
                    clear: None,
                },
            )));
        }

        if !spec.has_value() && !spec.is_drafting() {
            let mut empty = Node::text("No filters");
            empty.style.descriptor.text_color = Some(text_secondary);
            empty.style.text_size = Some(rem_to_px(0.75));
            panel = panel.child(empty);
        }

        let mut surface_el = Node::container();
        {
            let s = &mut surface_el.style;
            // Explicit Row (see switch.rs): one panel child.
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.min_width = Some(rem_to_px(16.0));
            s.max_width = Some(rem_to_px(24.0));
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = item_border;
            s.descriptor.background = Some(elevated);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.375);
            pad.right = rem_to_px(0.375);
            pad.top = rem_to_px(0.375);
            pad.bottom = rem_to_px(0.375);
        }
        all_corners(&mut surface_el, surface_radius);

        // Contract `dismissOnOutsideInteract` (default `true`): a *refusal*
        // flag — native overlays dismiss on outside interact by default. The
        // refusal rides the surface's interaction as an inert activation: a
        // host implementing outside-dismissal must not dismiss a panel
        // carrying this marker (see menu.rs for the full contract note).
        if !spec.dismiss_on_outside_interact {
            surface_el.interaction.on_activate = Some(Arc::new(|| {}));
        }

        root = root.child(surface_el.child(panel));
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
    }

    if !spec.aria_label.is_empty() {
        root.a11y.label = Some(spec.aria_label.clone());
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn outside_interact_refusal_marks_the_open_surface() {
        // Web default `true` + open: no refusal marker anywhere in the tree.
        let spec = FilterBuilderSpec::new().with_open(true);
        let node = filter_builder(&spec, &theme(), &FilterBuilderHandlers::default());
        assert!(node.find(&|n| n.interaction.on_activate.is_some()).is_none());

        // Refusal: the open surface carries the inert activation marker a
        // host keys outside-dismissal on.
        let refusing = spec.with_dismiss_on_outside_interact(false);
        let node = filter_builder(&refusing, &theme(), &FilterBuilderHandlers::default());
        assert!(node.find(&|n| n.interaction.on_activate.is_some()).is_some());
    }
}
