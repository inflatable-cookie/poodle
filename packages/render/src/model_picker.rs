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
    ControlDensity, ControlSize, ModelAxisControlKind, ModelAxisKind, ModelAxisValue,
    ModelPickerSpec, ModelPickerVariant, SegmentedControlOption, SegmentedControlSpec, SwitchSpec,
};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;
use crate::segmented_control::segmented_control;
use crate::switch::switch;

pub fn model_picker(
    spec: &ModelPickerSpec,
    ctx: &RenderContext<'_>,
    instance_id: &str,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

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
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.descriptor.layout.spacing.gap = rem_to_px(0.5);
    root.style.min_width = Some(0.0);
    let mut root = root.child(trigger);

    // ── Dialog surface (rendered inline when open) ────────────────────────────
    if spec.is_open {
        // Two columns (models | axes) whenever the selected model has
        // applicable axes; a plain list otherwise (contract §7).
        let applicable = spec.applicable_axes();
        let is_split = !applicable.is_empty();

        let mut models = Node::container();
        {
            let s = &mut models.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.min_width = Some(0.0);
            s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        }
        let mut models = models;
        for (index, model) in spec.models.iter().enumerate() {
            if let Some(heading) = spec.group_heading_for(index) {
                let mut h = Node::text(heading);
                {
                    let s = &mut h.style;
                    s.flex_none = true;
                    s.descriptor.text_color = Some(text_secondary);
                    s.text_size = Some(rem_to_px(0.6875));
                    s.text_weight = Some(500);
                    s.letter_spacing_em = Some(0.05);
                    let pad = &mut s.descriptor.layout.spacing.padding;
                    pad.left = rem_to_px(0.5);
                    // Space above every heading but the first, so group runs
                    // read as sections.
                    pad.top = rem_to_px(if index == 0 { 0.5 } else { 0.875 });
                    pad.bottom = rem_to_px(0.25);
                }
                models = models.child(h);
            }

            let is_selected = model.value == spec.value.model;
            // Never shrink: the list is height-capped and scrolls, so a
            // shrinkable row would squash below its own content.
            let mut row = Node::container();
            {
                let s = &mut row.style;
                s.flex_none = true;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
                s.descriptor.layout.spacing.gap = rem_to_px(0.5);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.5);
                pad.right = rem_to_px(0.5);
                pad.top = rem_to_px(0.375);
                pad.bottom = rem_to_px(0.375);
                if is_selected {
                    s.descriptor.background = Some(row_selected_bg);
                }
            }
            all_radius(&mut row, radius);
            let mut row = row;

            if let Some(image) = &model.image {
                row = row.child(image_node(&image.src, rem_to_px(1.0)));
            } else if let Some(icon) = &model.icon {
                let mut glyph = Node::icon(icon, rem_to_px(0.875));
                glyph.style.descriptor.text_color = Some(text_secondary);
                row = row.child(glyph);
            }

            let mut copy = Node::container();
            {
                let s = &mut copy.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.width = LayoutSizing::Grow;
                s.min_width = Some(0.0);
            }
            let mut title = Node::text(&model.label);
            {
                let s = &mut title.style;
                s.descriptor.text_color = Some(text_primary);
                s.text_size = Some(rem_to_px(0.875));
                s.text_weight = Some(if is_selected { 600 } else { 400 });
                s.text_ellipsis = true;
                s.no_wrap = true;
            }
            let mut copy = copy.child(title);
            if spec.show_model_descriptions {
                if let Some(description) = &model.description {
                    let mut d = Node::text(description);
                    d.style.descriptor.text_color = Some(text_secondary);
                    d.style.text_size = Some(rem_to_px(0.75));
                    copy = copy.child(d);
                }
            }
            row = row.child(copy);

            if let Some(badge) = &model.badge {
                let mut b = Node::text(badge);
                {
                    let s = &mut b.style;
                    s.flex_none = true;
                    let pad = &mut s.descriptor.layout.spacing.padding;
                    pad.left = rem_to_px(0.375);
                    pad.right = rem_to_px(0.375);
                    s.descriptor.border.width = 1.0;
                    s.descriptor.border.color = item_border;
                    s.descriptor.text_color = Some(text_secondary);
                    s.text_size = Some(rem_to_px(0.6875));
                }
                all_radius(&mut b, rem_to_px(0.5));
                row = row.child(b);
            }

            if is_selected {
                let mut check = Node::icon("check", rem_to_px(0.75));
                check.style.descriptor.text_color = Some(accent);
                row = row.child(check);
            }

            if model.is_disabled {
                row.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
            } else if let Some(handler) = &on_change {
                let handler = Arc::clone(handler);
                let id = model.value.clone();
                row.style.descriptor.cursor = CursorHint::Pointer;
                row.interaction.on_activate = Some(Arc::new(move || handler(&id)));
            }

            models = models.child(row);
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
                    list.style.descriptor.layout.direction = LayoutDirection::Column;
                    list.style.descriptor.layout.spacing.gap = rem_to_px(0.0625);
                    list.style.min_width = Some(0.0);
                    for option in axis.options.iter() {
                        let is_option_selected = option.value == selected;
                        let mut row = Node::container();
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
                        format!("{instance_id}:axis:{}", axis.key),
                        options,
                    )
                    .with_size(effective_size)
                    .with_density(density);
                    control.value = current.as_text().map(|value| value.to_string());
                    control.is_disabled = spec.is_disabled || axis.is_disabled;
                    section.child(segmented_control(&control, ctx, None))
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
                    section.child(switch(&control, ctx, None))
                }
            };

            axes_column = axes_column.child(section);
        }

        // Stretch (the flex default), not items_start: the rail's left rule
        // must run the panel's full height.
        // Contract: the open picker panel is a `dialog`.
        let mut panel = Node::container();
        panel.a11y.role = Some(NodeRole::Dialog);
        panel.style.descriptor.layout.direction = LayoutDirection::Row;
        panel.style.descriptor.layout.spacing.gap = rem_to_px(0.75);
        let mut panel = panel.child(models);
        if is_split {
            panel = panel.child(axes_column);
        }

        let mut dialog = Node::container();
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
