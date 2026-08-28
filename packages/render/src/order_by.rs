//! OrderBy — ordered list of sort fields behind a trigger.
//!
//! Contract: `docs/contracts/components/order-by.md`
//! Ported from: `packages/jetstream/components/src/order_by.rs`.
//!
//! The contract has a single `onChange` carrying the whole ordering. A pointer
//! cannot produce an ordering; it produces one of two intents on one row, so
//! that is what the events say: `on_direction_toggle` and `on_remove`, each
//! carrying the field. The host applies the intent to the ordering it already
//! holds and passes the result back through the spec. Reordering is a drag,
//! and the rows carry a drag handle with no handler yet.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodeRole,
};
use poodle_specs::{
    ButtonVariant, ChoiceOption, ControlDensity, ControlSize, IconButtonSpec, OrderBySpec,
    OrderByTriggerVariant, SelectSpec, SortDirection,
};

use crate::color::mix_srgb;
use crate::context::RenderContext;
use crate::icon_button::icon_button;
use crate::presentation::{rem_to_px, size_padding_x_offset_rem};
use crate::select::{select, SelectHandlers};

/// Host-owned native interaction for one OrderBy instance.
///
/// `instance_id` is the lifetime-stable scope. It is not a web public prop, and
/// the renderer never invents one from render order or selected value.
pub struct OrderByHandlers {
    pub instance_id: String,
    /// Fires with the field whose direction arrow was pressed.
    pub on_direction_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the field that was removed.
    pub on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl OrderByHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.trim().is_empty(),
            "OrderByHandlers requires a non-empty lifetime-stable instance_id"
        );
        Self {
            instance_id,
            on_direction_toggle: None,
            on_remove: None,
        }
    }
}

pub fn order_by(spec: &OrderBySpec, ctx: &RenderContext<'_>, handlers: OrderByHandlers) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);

    // ── Size table (matches corrected contract §8) ────────────────────────────
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

    // ── Density (trigger gap only) ────────────────────────────────────────────
    let density = ctx.resolve_density(spec.density);
    let trigger_gap = rem_to_px(match density {
        ControlDensity::Compact => 0.375,
        ControlDensity::Default => 0.5,
        ControlDensity::Comfortable => 0.625,
    });

    // ── Panel/list/item spacing (contract §8) ─────────────────────────────────
    let root_gap = rem_to_px(0.375);
    let panel_gap = rem_to_px(0.375);
    let list_gap = rem_to_px(0.25);
    let item_gap = rem_to_px(0.375);
    let item_pad_x = rem_to_px(0.5);
    let item_pad_y = rem_to_px(0.3125);
    let item_label_font = rem_to_px(0.8125);

    // ── Colors ────────────────────────────────────────────────────────────────
    let text_primary = ctx.theme().resolve_color(spec.field_text_token());
    let text_secondary = ctx.theme().resolve_color(spec.label_color_token());
    let muted = ctx.theme().resolve_color(spec.muted_color_token());
    let border = ctx.theme().resolve_color(spec.field_border_token());
    let item_border = ctx.theme().resolve_color(spec.item_border_token());
    let surface = ctx.theme().resolve_color(spec.field_fill_token());
    let elevated = ctx.theme().resolve_color(spec.field_hover_fill_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    let surface_radius = ctx.theme().resolve_radius(spec.surface_radius_token());
    // Item bg: color-mix(surface 90%, elevated) per contract.
    let item_bg = mix_srgb(surface, elevated, 0.90);

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };

    // ── Trigger ───────────────────────────────────────────────────────────────
    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.descriptor.layout.spacing.gap = root_gap;
    root.style.min_width = Some(0.0);
    let mut root = root;

    match spec.trigger_variant {
        OrderByTriggerVariant::Icon => {
            root = root.child(icon_button(
                &IconButtonSpec::new()
                    .with_icon("arrow-up-down")
                    .with_aria_label(spec.aria_label.clone())
                    .with_tooltip(spec.aria_label.clone())
                    .with_variant(ButtonVariant::Secondary)
                    .with_size(effective_size)
                    .with_expanded(spec.is_open)
                    .with_controls("order-by-surface")
                    .with_disabled(spec.is_disabled),
                ctx,
                None,
            ));
        }
        OrderByTriggerVariant::Summary => {
            let mut trigger = Node::container();
            {
                let s = &mut trigger.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = trigger_gap;
                s.descriptor.layout.width = LayoutSizing::Grow;
                s.min_width = Some(0.0);
                s.min_height = Some(trigger_h);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = trigger_pad_x;
                pad.right = trigger_pad_x;
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = border;
                s.descriptor.background = Some(surface);
            }
            all_radius(&mut trigger, radius);
            let mut trigger = trigger;

            if !spec.compact {
                let mut eyebrow = Node::text("SORT BY");
                {
                    let s = &mut eyebrow.style;
                    s.descriptor.text_color = Some(text_secondary);
                    s.text_size = Some(label_font);
                    s.text_weight = Some(500);
                    s.letter_spacing_em = Some(0.05);
                }
                trigger = trigger.child(eyebrow);
            }

            let summary_is_placeholder = !spec.has_value();
            let mut summary = Node::text(spec.summary_text());
            {
                let s = &mut summary.style;
                s.descriptor.text_color = Some(if summary_is_placeholder {
                    muted
                } else {
                    text_primary
                });
                s.text_size = Some(summary_font);
                s.descriptor.layout.width = LayoutSizing::Grow;
                s.min_width = Some(0.0);
                s.text_ellipsis = true;
                s.no_wrap = true;
            }
            trigger = trigger.child(summary);

            if spec.show_clear_button && spec.has_value() {
                trigger = trigger.child(icon_button(
                    &IconButtonSpec::new()
                        .with_icon("x")
                        .with_aria_label("Clear sort")
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(effective_size)
                        .with_disabled(spec.is_disabled),
                    ctx,
                    None,
                ));
            }

            let mut chevron = Node::icon("chevron-down", summary_font);
            chevron.style.descriptor.text_color = Some(text_secondary);
            root = root.child(trigger.child(chevron));
        }
    }

    // ── Dialog surface (rendered inline when open) ───────────────────────────
    if spec.is_open {
        let current_value = spec.current_value();
        // Contract: the open overlay panel is a `dialog`.
        let mut panel = Node::container();
        panel.a11y.role = Some(NodeRole::Dialog);
        panel.style.descriptor.layout.direction = LayoutDirection::Column;
        panel.style.descriptor.layout.spacing.gap = panel_gap;
        let mut panel = panel;

        if matches!(spec.trigger_variant, OrderByTriggerVariant::Icon) {
            let mut header = Node::container();
            {
                let s = &mut header.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
                s.descriptor.layout.spacing.gap = rem_to_px(0.5);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.25);
                pad.right = rem_to_px(0.25);
            }
            let mut title = Node::text("Sort order");
            title.style.descriptor.text_color = Some(text_secondary);
            title.style.text_size = Some(rem_to_px(0.75));
            let mut header = header.child(title);
            if spec.show_clear_button && spec.has_value() {
                header = header.child(icon_button(
                    &IconButtonSpec::new()
                        .with_icon("x")
                        .with_aria_label("Clear sort")
                        .with_tooltip("Clear sort")
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Xs)
                        .with_disabled(spec.is_disabled),
                    ctx,
                    None,
                ));
            }
            panel = panel.child(header);
        }

        if current_value.is_empty() {
            let mut empty = Node::text("No sort fields");
            empty.style.descriptor.text_color = Some(text_secondary);
            empty.style.text_size = Some(rem_to_px(0.75));
            panel = panel.child(empty);
        } else {
            // Contract: the active sort clauses are a `list` of `listitem`s.
            let mut list = Node::container();
            list.a11y.role = Some(NodeRole::List);
            list.style.descriptor.layout.direction = LayoutDirection::Column;
            list.style.descriptor.layout.spacing.gap = list_gap;
            for item in current_value.iter() {
                let field_label = spec.active_label(&item.key);
                let (dir_icon, dir_tooltip, dir_word) = match item.direction {
                    SortDirection::Asc => ("arrow-up", "Asc", "ascending"),
                    SortDirection::Desc => ("arrow-down", "Desc", "descending"),
                };

                let mut row = Node::container();
                row.a11y.role = Some(NodeRole::ListItem);
                {
                    let s = &mut row.style;
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    s.descriptor.layout.spacing.gap = item_gap;
                    let pad = &mut s.descriptor.layout.spacing.padding;
                    pad.left = item_pad_x;
                    pad.right = item_pad_x;
                    pad.top = item_pad_y;
                    pad.bottom = item_pad_y;
                    s.descriptor.border.width = 1.0;
                    s.descriptor.border.color = item_border;
                    s.descriptor.background = Some(item_bg);
                }
                all_radius(&mut row, radius);

                // Drag handle (focusable button carrying the braille glyph).
                let mut handle = Node::button("⠿");
                {
                    let s = &mut handle.style;
                    s.min_width = Some(rem_to_px(1.5));
                    s.min_height = Some(rem_to_px(1.5));
                    s.flex_none = true;
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                    s.descriptor.text_color = Some(muted);
                    s.text_size = Some(rem_to_px(0.75));
                }
                handle.interaction.focusable = true;

                // Field label (single-line ellipsis).
                let mut label = Node::text(&field_label);
                {
                    let s = &mut label.style;
                    s.descriptor.layout.width = LayoutSizing::Grow;
                    s.min_width = Some(0.0);
                    s.text_ellipsis = true;
                    s.no_wrap = true;
                    s.descriptor.text_color = Some(text_primary);
                    s.text_size = Some(item_label_font);
                }

                // Direction toggle (xs ghost IconButton).
                let direction = icon_button(
                    &IconButtonSpec::new()
                        .with_icon(dir_icon)
                        .with_aria_label(format!("{field_label}: {dir_word}. Click to toggle."))
                        .with_tooltip(dir_tooltip)
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Xs)
                        .with_disabled(spec.is_disabled),
                    ctx,
                    handlers.on_direction_toggle.as_ref().map(|handler| {
                        let handler = Arc::clone(handler);
                        let field = item.key.clone();
                        Arc::new(move || handler(&field)) as Arc<dyn Fn() + Send + Sync>
                    }),
                );

                // Remove (xs ghost IconButton, no danger tone).
                let remove = icon_button(
                    &IconButtonSpec::new()
                        .with_icon("x")
                        .with_aria_label(format!("Remove {field_label}"))
                        .with_tooltip("Remove")
                        .with_variant(ButtonVariant::Ghost)
                        .with_size(ControlSize::Xs)
                        .with_disabled(spec.is_disabled),
                    ctx,
                    handlers.on_remove.as_ref().map(|handler| {
                        let handler = Arc::clone(handler);
                        let field = item.key.clone();
                        Arc::new(move || handler(&field)) as Arc<dyn Fn() + Send + Sync>
                    }),
                );

                list = list.child(
                    row.child(handle)
                        .child(label)
                        .child(direction)
                        .child(remove),
                );
            }
            panel = panel.child(list);
        }

        // Add-field Select (hidden when no fields remain or maxFields reached).
        let can_add_more = spec
            .max_fields
            .map(|max| spec.active_count() < max)
            .unwrap_or(true);
        if can_add_more && !spec.available_fields().is_empty() {
            let options: Vec<ChoiceOption> = spec
                .available_fields()
                .into_iter()
                .map(|field| {
                    ChoiceOption::new(field.resolved_key().to_string(), field.label.clone())
                })
                .collect();
            let mut select_spec = SelectSpec::new(options)
                .with_placeholder("+ Add field")
                .with_size(effective_size)
                .with_density(density);
            select_spec.aria_label = Some("Add sort field".to_string());
            select_spec.is_disabled = spec.is_disabled;

            let mut row = Node::container();
            row.style.descriptor.layout.direction = LayoutDirection::Row;
            row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            panel = panel.child(row.child(select(
                &select_spec,
                ctx,
                &SelectHandlers::new(&handlers.instance_id),
            )));
        }

        // Dialog surface chrome.
        let mut dialog = Node::container();
        {
            let s = &mut dialog.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.min_width = Some(rem_to_px(14.0));
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
    use poodle_specs::SortField;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn outside_interact_refusal_marks_the_open_surface() {
        // Web default `true` + open: no refusal marker anywhere in the tree.
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = OrderBySpec::new().with_open(true);
        let node = order_by(&spec, &ctx, OrderByHandlers::new("order-by"));
        assert!(node
            .find(&|n| n.interaction.on_activate.is_some())
            .is_none());

        // Refusal: the open surface carries the inert activation marker a
        // host keys outside-dismissal on.
        let refusing = spec.with_dismiss_on_outside_interact(false);
        let node = order_by(&refusing, &ctx, OrderByHandlers::new("order-by"));
        assert!(node
            .find(&|n| n.interaction.on_activate.is_some())
            .is_some());
    }

    #[test]
    fn two_order_bys_do_not_share_select_runtime_ids() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = OrderBySpec::new()
            .with_fields(vec![
                SortField::new("name", "Name"),
                SortField::new("created", "Created"),
            ])
            .with_open(true);
        let left = order_by(&spec, &ctx, OrderByHandlers::new("sort-a"));
        let right = order_by(&spec, &ctx, OrderByHandlers::new("sort-b"));
        let mut tree = Node::container();
        tree = tree.child(left).child(right);
        assert!(tree
            .find(&|n| n.runtime_id.as_deref() == Some("select:sort-a:trigger"))
            .is_some());
        assert!(tree
            .find(&|n| n.runtime_id.as_deref() == Some("select:sort-b:trigger"))
            .is_some());
    }

    #[test]
    #[should_panic(expected = "OrderByHandlers requires a non-empty lifetime-stable instance_id")]
    fn empty_instance_scope_is_rejected() {
        let _ = OrderByHandlers::new("");
    }
}
