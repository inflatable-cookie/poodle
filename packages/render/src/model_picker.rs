//! ModelPicker — model + capability-axis picker.
//!
//! Contract: `docs/contracts/components/model-picker.md`
//! Ported from: `packages/jetstream/components/src/model_picker.rs`.
//!
//! Anatomy: root → trigger (optional model icon + label + axis summary +
//! chevron) → dialog surface (model rows with group headings, badges and a
//! selection mark, then one section per applicable axis).

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeKind, NodeRole,
    NodeToggled,
};
use poodle_specs::{
    ChoiceOption, ControlDensity, ControlSize, ModelAxisControlKind, ModelAxisKind, ModelAxisValue,
    ModelPickerSpec, ModelPickerVariant, SegmentedControlOption, SegmentedControlSpec, SelectSpec,
    SwitchSpec,
};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;
use crate::segmented_control::segmented_control;
use crate::select::{select, select_option_id, SelectHandlers};
use crate::switch::switch;

fn find_runtime_id_mut<'a>(node: &'a mut Node, runtime_id: &str) -> Option<&'a mut Node> {
    if node.runtime_id.as_deref() == Some(runtime_id) {
        return Some(node);
    }
    node.children
        .iter_mut()
        .find_map(|child| find_runtime_id_mut(child, runtime_id))
}

pub fn model_picker(
    spec: &ModelPickerSpec,
    ctx: &RenderContext<'_>,
    instance_id: &str,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let is_open = spec.is_open && !spec.is_disabled;
    let select_scope = format!("model-picker:{instance_id}");
    let mut select_spec = SelectSpec::new(
        spec.models
            .iter()
            .map(|model| {
                let mut option = ChoiceOption::new(&model.value, &model.label)
                    .with_disabled(model.is_disabled);
                if spec.show_model_descriptions {
                    if let Some(description) = &model.description {
                        option = option.with_description(description);
                    }
                }
                if let Some(group) = &model.group {
                    option = option.with_group(group);
                }
                option
            })
            .collect(),
    )
    .with_value(&spec.value.model)
    .with_placeholder(&spec.placeholder)
    .with_aria_label(spec.trigger_aria_label())
    .with_open(spec.is_open)
    .with_dismiss_on_outside_interact(spec.dismiss_on_outside_interact)
    .with_size(effective_size)
    .with_density(density);
    select_spec.is_disabled = spec.is_disabled;
    let mut select_handlers = SelectHandlers::new(&select_scope);
    if let Some(handler) = on_change.clone() {
        let current = spec.value.model.clone();
        select_handlers = select_handlers.on_transition(Arc::new(move |result| {
            let next = result.context.value.as_str();
            if !next.is_empty() && next != current {
                handler(next);
            }
        }));
    }
    let mut select_tree = select(&select_spec, ctx, &select_handlers);
    let (select_trigger, mut models) = if is_open {
        let mut children = std::mem::take(&mut select_tree.children);
        assert_eq!(
            children.len(),
            2,
            "open ModelPicker Select composition requires trigger and listbox"
        );
        let listbox = children.pop().expect("Select listbox");
        let trigger = children.pop().expect("Select trigger");
        (trigger, Some(listbox))
    } else {
        (select_tree, None)
    };

    // ── Size table (contract §8) ──────────────────────────────────────────────
    let trigger_h = rem_to_px(match effective_size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.25,
    });
    let trigger_font = rem_to_px(match effective_size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.8125,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    });
    let trigger_gap = rem_to_px(match density {
        ControlDensity::Compact => 0.25,
        ControlDensity::Default => 0.375,
        ControlDensity::Comfortable => 0.5,
    });

    // ── Colors ────────────────────────────────────────────────────────────────
    let text_primary = ctx.theme().resolve_color(spec.label_color_token());
    let text_secondary = ctx.theme().resolve_color(spec.secondary_color_token());
    let muted = ctx.theme().resolve_color(spec.muted_color_token());
    let border = ctx.theme().resolve_color(spec.trigger_border_token());
    let item_border = ctx.theme().resolve_color(spec.item_border_token());
    let surface = ctx.theme().resolve_color(spec.trigger_fill_token());
    let elevated = ctx.theme().resolve_color(spec.surface_fill_token());
    let accent = ctx.theme().resolve_color(spec.selected_color_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    let surface_radius = ctx.theme().resolve_radius(spec.surface_radius_token());
    let row_selected_bg = mix_srgb(elevated, accent, 0.86);

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };
    let image_node = |src: &str, size: f32| -> Node {
        let mut img = Node::container();
        img.kind = NodeKind::Image {
            source: src.to_string(),
        };
        // Explicit Row (see switch.rs).
        img.style.descriptor.layout.direction = LayoutDirection::Row;
        img.style.descriptor.layout.width = LayoutSizing::Fixed(size);
        img.style.descriptor.layout.height = LayoutSizing::Fixed(size);
        img.style.flex_none = true;
        img
    };

    // ── Trigger ───────────────────────────────────────────────────────────────
    let mut trigger = Node::container();
    trigger.id = select_trigger.id;
    trigger.runtime_id = select_trigger.runtime_id;
    trigger.a11y = select_trigger.a11y;
    trigger.a11y.expanded = Some(is_open);
    trigger.a11y.controls = is_open.then(|| format!("model-picker-{instance_id}-dialog"));
    trigger.interaction = select_trigger.interaction;
    {
        let s = &mut trigger.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = trigger_gap;
        s.min_width = Some(0.0);
        s.min_height = Some(trigger_h);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = rem_to_px(0.375);
        pad.right = rem_to_px(0.375);
        if spec.variant == ModelPickerVariant::Outlined {
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = border;
            s.descriptor.background = Some(surface);
        }
        s.focus_ring = select_trigger.style.focus_ring;
        s.hover = select_trigger.style.hover;
    }
    all_radius(&mut trigger, radius);
    let mut trigger = trigger;

    // An arbitrary image (a provider logo) wins over a registry icon name.
    if let Some(image) = spec.selected_model().and_then(|model| model.image.clone()) {
        trigger = trigger.child(image_node(&image.src, trigger_font));
    } else if let Some(icon) = spec.selected_model().and_then(|model| model.icon.clone()) {
        let mut glyph = Node::icon(&icon, trigger_font);
        glyph.style.descriptor.text_color = Some(text_secondary);
        trigger = trigger.child(glyph);
    }

    // Subdued emphasis dims the resting trigger so the picker recedes beside a
    // more important control; hover/focus restoration is web-only (§12).
    let label_color = if spec.has_selection() {
        ctx.theme().resolve_color(spec.trigger_label_color_token())
    } else {
        muted
    };
    let subdued_opacity = if spec.emphasis.is_subdued() {
        ctx.theme().resolve_opacity(spec.trigger_subdued_opacity_token())
    } else {
        1.0
    };
    let dimmed = with_alpha(text_secondary, text_secondary.3 * subdued_opacity);

    let mut label = Node::text(spec.trigger_label());
    {
        let s = &mut label.style;
        s.descriptor.text_color = Some(label_color);
        s.text_size = Some(trigger_font);
        s.text_weight = Some(if spec.has_selection() { 500 } else { 400 });
        s.text_ellipsis = true;
        s.no_wrap = true;
    }
    trigger = trigger.child(label);

    let summary = spec.summary_text();
    if !summary.is_empty() {
        // The hairline before the summary is the web's ::before rule.
        let mut rule = Node::container();
        {
            let s = &mut rule.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(0.0625));
            s.descriptor.layout.height = LayoutSizing::Fixed(trigger_font);
            s.flex_none = true;
            s.descriptor.background = Some(item_border);
        }
        let mut summary_label = Node::text(&summary);
        {
            let s = &mut summary_label.style;
            s.descriptor.text_color = Some(dimmed);
            s.text_size = Some(trigger_font);
            s.text_ellipsis = true;
            s.no_wrap = true;
        }
        trigger = trigger.child(rule).child(summary_label);
    }

    let mut chevron = Node::icon("chevron-down", trigger_font);
    chevron.style.descriptor.text_color = Some(dimmed);
    trigger = trigger.child(chevron);

    let mut root = Node::container();
    root.id = Some(format!("model-picker-{instance_id}"));
    root.runtime_id = Some(select_scope.clone());
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.descriptor.layout.spacing.gap = rem_to_px(0.5);
    root.style.min_width = Some(0.0);
    root.roles
        .insert("dependency".to_owned(), "select".to_owned());
    root.roles.insert(
        "size".to_owned(),
        format!("{effective_size:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "density".to_owned(),
        format!("{density:?}").to_ascii_lowercase(),
    );
    root.roles
        .insert("variant".to_owned(), spec.variant.as_str().to_owned());
    root.roles
        .insert("emphasis".to_owned(), spec.emphasis.as_str().to_owned());
    root.roles
        .insert("open".to_owned(), is_open.to_string());
    root.roles
        .insert("disabled".to_owned(), spec.is_disabled.to_string());
    let mut root = root.child(trigger);

    // ── Dialog surface (rendered inline when open) ────────────────────────────
    if is_open {
        // Two columns (models | axes) whenever the selected model has
        // applicable axes; a plain list otherwise (contract §7).
        let applicable = spec.applicable_axes();
        let is_split = !applicable.is_empty();

        let mut models = models.take().expect("open ModelPicker Select listbox");
        models.style.descriptor.layout.width = LayoutSizing::Grow;
        models.style.min_width = Some(0.0);
        models.style.descriptor.border.width = 0.0;
        models.style.descriptor.background = None;
        models.style.descriptor.shadow = None;
        models.style.shadow_layers.clear();
        models.style.overlay = false;
        models.position = poodle_node::NodePosition::InFlow;
        models.a11y.role = Some(NodeRole::RadioGroup);
        models.a11y.label = Some("Model".to_owned());
        models.roles.insert("dependency".to_owned(), "select".to_owned());
        let initial_focus_value = spec
            .models
            .iter()
            .find(|model| model.value == spec.value.model && !model.is_disabled)
            .or_else(|| spec.models.iter().find(|model| !model.is_disabled))
            .map(|model| model.value.as_str());
        for model in &spec.models {
            let row_id = select_option_id(&select_scope, &model.value);
            if let Some(row) = find_runtime_id_mut(&mut models, &row_id) {
                let is_selected = model.value == spec.value.model;
                row.a11y.role = Some(NodeRole::RadioButton);
                row.a11y.label = Some(if spec.show_model_descriptions {
                    match model.description.as_deref() {
                        Some(description) if !description.is_empty() => {
                            format!("{} {}", model.label, description)
                        }
                        _ => model.label.clone(),
                    }
                } else {
                    model.label.clone()
                });
                row.a11y.selected = Some(is_selected);
                row.a11y.toggled = Some(if is_selected {
                    NodeToggled::True
                } else {
                    NodeToggled::False
                });
                let is_initial_focus = Some(model.value.as_str()) == initial_focus_value;
                row.a11y.initial_focus = is_initial_focus;
                row.roles
                    .insert("selected".to_owned(), is_selected.to_string());
                row.roles
                    .insert("disabled".to_owned(), model.is_disabled.to_string());
                if (is_selected || is_initial_focus) && !spec.is_disabled && !model.is_disabled {
                    // The selected enabled radio is normally the single
                    // sequential stop. If controlled state points at a
                    // disabled model, the first enabled fallback is both the
                    // initial-focus target and the Svelte tab stop. Select's
                    // ordinary listbox options remain pointer targets.
                    row.interaction.focusable = true;
                    row.a11y.tab_index = Some(0);
                }
                if is_selected {
                    row.style.descriptor.background = Some(row_selected_bg);
                }
                if let Some(image) = &model.image {
                    row.children.insert(0, image_node(&image.src, rem_to_px(1.0)));
                } else if let Some(icon) = &model.icon {
                    let mut glyph = Node::icon(icon, rem_to_px(0.875));
                    glyph.style.descriptor.text_color = Some(text_secondary);
                    row.children.insert(0, glyph);
                }
                if let Some(badge) = &model.badge {
                    let mut badge_node = Node::text(badge);
                    badge_node.style.flex_none = true;
                    badge_node.style.descriptor.border.width = 1.0;
                    badge_node.style.descriptor.border.color = item_border;
                    badge_node.style.descriptor.text_color = Some(text_secondary);
                    badge_node.style.text_size = Some(rem_to_px(0.6875));
                    all_radius(&mut badge_node, rem_to_px(0.5));
                    let insert_at = row.children.len().saturating_sub(usize::from(is_selected));
                    row.children.insert(insert_at, badge_node);
                }
            }
        }

        // One section per applicable axis, in declaration order, stacked in
        // the right-hand column.
        let mut axes_column = Node::container();
        {
            let s = &mut axes_column.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.75);
            s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(13.0));
            s.flex_none = true;
            s.descriptor.layout.spacing.padding.left = rem_to_px(0.75);
            s.border_left_width = Some(rem_to_px(0.0625));
            s.descriptor.border.color = item_border;
        }
        let mut axes_column = axes_column;

        for (index, axis) in applicable.iter().enumerate() {
            let current = spec.axis_value(axis);
            let mut section = Node::container();
            section.runtime_id = Some(format!("{select_scope}:axis:{}", axis.key));
            section
                .roles
                .insert("kind".to_owned(), axis.kind.as_str().to_owned());
            section.roles.insert(
                "control".to_owned(),
                match axis.control_kind() {
                    ModelAxisControlKind::Segmented => "segmented",
                    ModelAxisControlKind::List => "list",
                }
                .to_owned(),
            );
            {
                let s = &mut section.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = rem_to_px(0.375);
                // The column rule already separates the axes from the list, so
                // only sections after the first carry a top rule.
                if index > 0 {
                    s.descriptor.layout.spacing.padding.top = rem_to_px(0.5);
                    s.border_top_width = Some(1.0);
                    s.descriptor.border.color = item_border;
                }
            }
            let mut heading = Node::text(&axis.label);
            {
                let s = &mut heading.style;
                s.descriptor.text_color = Some(text_secondary);
                s.text_size = Some(rem_to_px(0.6875));
                s.text_weight = Some(500);
                s.letter_spacing_em = Some(0.05);
            }
            let mut section = section.child(heading);

            if let Some(description) = &axis.description {
                let mut d = Node::text(description);
                d.style.descriptor.text_color = Some(text_secondary);
                d.style.text_size = Some(rem_to_px(0.75));
                section = section.child(d);
            }

            section = match axis.kind {
                // Many-level scales render as a vertical list; short ones stay
                // segmented (contract §4).
                ModelAxisKind::Select if axis.control_kind() == ModelAxisControlKind::List => {
                    let selected = current.as_text().unwrap_or_default().to_string();
                    let mut list = Node::container();
                    list.runtime_id = Some(format!("{select_scope}:axis:{}:list", axis.key));
                    list.a11y.role = Some(NodeRole::RadioGroup);
                    list.a11y.label = Some(axis.label.clone());
                    list.style.descriptor.layout.direction = LayoutDirection::Column;
                    list.style.descriptor.layout.spacing.gap = rem_to_px(0.0625);
                    list.style.min_width = Some(0.0);
                    for option in axis.options.iter() {
                        let is_option_selected = option.value == selected;
                        let mut row = Node::container();
                        row.runtime_id = Some(format!(
                            "{select_scope}:axis:{}:option:{}",
                            axis.key, option.value
                        ));
                        // Contract: axis options are mutually exclusive, so
                        // each is a `radio` with its own checked state.
                        row.a11y.role = Some(NodeRole::RadioButton);
                        row.a11y.label = Some(option.label.clone());
                        row.a11y.toggled = Some(if is_option_selected {
                            NodeToggled::True
                        } else {
                            NodeToggled::False
                        });
                        {
                            let s = &mut row.style;
                            s.descriptor.layout.direction = LayoutDirection::Row;
                            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                            s.descriptor.layout.spacing.gap = rem_to_px(0.5);
                            let pad = &mut s.descriptor.layout.spacing.padding;
                            pad.left = rem_to_px(0.375);
                            pad.right = rem_to_px(0.375);
                            pad.top = rem_to_px(0.25);
                            pad.bottom = rem_to_px(0.25);
                            if option.is_disabled {
                                s.descriptor.opacity =
                                    ctx.theme().resolve_opacity(spec.disabled_opacity_token());
                            }
                        }
                        all_radius(&mut row, radius);
                        let mut opt_label = Node::text(&option.label);
                        {
                            let s = &mut opt_label.style;
                            s.descriptor.layout.width = LayoutSizing::Grow;
                            s.min_width = Some(0.0);
                            s.descriptor.text_color = Some(if is_option_selected {
                                text_primary
                            } else {
                                text_secondary
                            });
                            s.text_size = Some(rem_to_px(0.8125));
                            s.text_weight = Some(if is_option_selected { 600 } else { 400 });
                            s.text_ellipsis = true;
                            s.no_wrap = true;
                        }
                        let mut row = row.child(opt_label);
                        if is_option_selected {
                            let mut check = Node::icon("check", rem_to_px(0.75));
                            check.style.descriptor.text_color = Some(accent);
                            row = row.child(check);
                        }
                        if spec.is_disabled || axis.is_disabled || option.is_disabled {
                            row.interaction.disabled = true;
                        } else if let Some(handler) = &on_change {
                            let handler = Arc::clone(handler);
                            let value = option.value.clone();
                            row.interaction.focusable = true;
                            row.a11y.tab_index = Some(0);
                            row.style.descriptor.cursor = CursorHint::Pointer;
                            row.interaction.on_activate =
                                Some(Arc::new(move || handler(value.as_str())));
                        }
                        list = list.child(row);
                    }
                    section.child(list)
                }
                ModelAxisKind::Select => {
                    let options: Vec<SegmentedControlOption> = axis
                        .options
                        .iter()
                        .map(|option| {
                            SegmentedControlOption::new(option.value.clone(), option.label.clone())
                        })
                        .collect();
                    let mut control = SegmentedControlSpec::new(
                        format!("{select_scope}:axis:{}", axis.key),
                        options,
                    )
                    .with_size(effective_size)
                    .with_density(density);
                    control.value = current.as_text().map(|value| value.to_string());
                    control.is_disabled = spec.is_disabled || axis.is_disabled;
                    section.child(segmented_control(&control, ctx, on_change.clone()))
                }
                ModelAxisKind::Toggle => {
                    let mut control = SwitchSpec::new()
                        // The axis names itself; the switch renders bare, so
                        // without this it is one of several unnamed toggles.
                        .with_aria_label(axis.label.clone())
                        .with_checked(matches!(current, ModelAxisValue::Flag(true)))
                        .with_size(effective_size)
                        .with_density(density);
                    control.is_disabled = spec.is_disabled || axis.is_disabled;
                    let toggle_handler = on_change.clone().map(|handler| {
                        Arc::new(move |next| handler(if next { "true" } else { "false" }))
                            as Arc<dyn Fn(bool) + Send + Sync>
                    });
                    let mut toggle = switch(&control, ctx, toggle_handler);
                    toggle.runtime_id = Some(format!("{select_scope}:axis:{}:toggle", axis.key));
                    section.child(toggle)
                }
            };

            axes_column = axes_column.child(section);
        }

        // Stretch (the flex default), not items_start: the rail's left rule
        // must run the panel's full height.
        // The panel owns layout only; the containing surface owns dialog
        // semantics and overlay chrome.
        let mut panel = Node::container();
        panel.runtime_id = Some(format!("{select_scope}:panel"));
        panel.style.descriptor.layout.direction = LayoutDirection::Row;
        panel.style.descriptor.layout.spacing.gap = rem_to_px(0.75);
        let mut panel = panel.child(models);
        if is_split {
            panel = panel.child(axes_column);
        }

        let mut dialog = Node::container();
        dialog.id = Some(format!("model-picker-{instance_id}-dialog"));
        dialog.runtime_id = Some(format!("{select_scope}:dialog"));
        dialog.a11y.role = Some(NodeRole::Dialog);
        dialog.a11y.label = Some(spec.aria_label.clone());
        {
            let s = &mut dialog.style;
            // The GPUI surface is a block wrapper around the horizontal panel.
            // Keep the wrapper column-oriented so its panel stretches to the
            // constrained surface width; the panel itself owns the row split.
            s.descriptor.layout.direction = LayoutDirection::Column;
            // The split layout needs room for both columns (contract §7).
            s.min_width = Some(rem_to_px(if is_split { 32.0 } else { 18.0 }));
            s.max_width = Some(rem_to_px(if is_split { 40.0 } else { 26.0 }));
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = item_border;
            s.descriptor.background = Some(elevated);
            s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.5);
            pad.right = rem_to_px(0.5);
            pad.top = rem_to_px(0.5);
            pad.bottom = rem_to_px(0.5);
        }
        all_radius(&mut dialog, surface_radius);

        // Contract `dismissOnOutsideInteract` (default `true`): a *refusal*
        // flag — native overlays dismiss on outside interact by default. The
        // refusal rides the surface's interaction as an inert activation: a
        // host implementing outside-dismissal must not dismiss a panel
        // carrying this marker (see menu.rs for the full contract note).
        if !spec.dismiss_on_outside_interact {
            dialog.interaction.on_activate = Some(Arc::new(|| {}));
        }

        root = root.child(dialog.child(panel));
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
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
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        // Web default `true` + open: no refusal marker anywhere in the tree.
        let spec = ModelPickerSpec::new().with_open(true);
        let node = model_picker(&spec, &ctx, "test", None);
        assert!(node
            .find(&|n| n.interaction.on_activate.is_some())
            .is_none());

        // Refusal: the open surface carries the inert activation marker a
        // host keys outside-dismissal on.
        let refusing = spec.with_dismiss_on_outside_interact(false);
        let node = model_picker(&refusing, &ctx, "test-refusing", None);
        assert!(node
            .find(&|n| n.interaction.on_activate.is_some())
            .is_some());
    }
}
