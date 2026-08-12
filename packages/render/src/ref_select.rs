//! RefSelect — version-control ref chooser.
//!
//! Contract: `docs/contracts/components/ref-select.md`
//! Ported from: `packages/jetstream/components/src/ref_select.rs`.
//!
//! Anatomy: root → trigger (kind glyph + label + chevron) → dialog surface
//! (search field, ref list with the current marker, empty/loading footers).
//! Typing and clicking live in the host event loop; the render is a faithful
//! function of the spec, including its query.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole};
use poodle_specs::{ControlDensity, ControlSize, RefSelectSpec, RefSelectVariant, TextInputSpec};

use crate::color::with_alpha;
use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::text_input::text_input;

pub fn ref_select(
    spec: &RefSelectSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

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
    let trigger_gap = rem_to_px(match spec.density {
        ControlDensity::Compact => 0.25,
        ControlDensity::Default => 0.375,
        ControlDensity::Comfortable => 0.5,
    });

    // ── Colors ────────────────────────────────────────────────────────────────
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color(spec.secondary_color_token());
    let muted = theme.resolve_color(spec.muted_color_token());
    let label_color = if spec.has_selection() {
        theme.resolve_color(spec.label_color_token())
    } else {
        muted
    };
    // Subdued dims the resting trigger; hover/focus restoration is web-only
    // (contract §12).
    let subdued_opacity = if spec.emphasis.is_subdued() {
        theme.resolve_opacity(spec.subdued_opacity_token())
    } else {
        1.0
    };
    let border = theme.resolve_color(spec.trigger_border_token());
    let item_border = theme.resolve_color(spec.item_border_token());
    let surface = theme.resolve_color(spec.trigger_fill_token());
    let elevated = theme.resolve_color(spec.surface_fill_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let surface_radius = theme.resolve_radius(spec.surface_radius_token());

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
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
        if spec.variant == RefSelectVariant::Outlined {
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = border;
            s.descriptor.background = Some(surface);
        }
    }
    all_radius(&mut trigger, radius);

    let glyph_color = with_alpha(text_secondary, text_secondary.3 * subdued_opacity);
    let mut kind_glyph = Node::icon(spec.trigger_icon(), trigger_font);
    kind_glyph.style.descriptor.text_color = Some(glyph_color);
    let mut label = Node::text(spec.trigger_label());
    {
        let s = &mut label.style;
        s.descriptor.text_color = Some(label_color);
        s.text_size = Some(trigger_font);
        s.text_weight = Some(if spec.has_selection() { 500 } else { 400 });
        s.text_ellipsis = true;
        s.no_wrap = true;
    }
    let mut chevron = Node::icon("chevron-down", trigger_font);
    chevron.style.descriptor.text_color = Some(glyph_color);
    let trigger = trigger.child(kind_glyph).child(label).child(chevron);

    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.descriptor.layout.spacing.gap = rem_to_px(0.5);
    root.style.min_width = Some(0.0);
    let mut root = root.child(trigger);

    // ── Dialog surface (rendered inline when open) ────────────────────────────
    if spec.is_open {
        // Contract: the open overlay panel is a `dialog`.
        let mut panel = Node::container();
        panel.a11y.role = Some(NodeRole::Dialog);
        panel.style.descriptor.layout.direction = LayoutDirection::Column;
        panel.style.descriptor.layout.spacing.gap = rem_to_px(0.5);

        if spec.is_searchable {
            let mut search = TextInputSpec::new()
                .with_size(effective_size)
                .with_density(spec.density)
                .with_disabled(spec.is_disabled);
            if let Some(query) = &spec.search_value {
                search = search.with_value(query.clone());
            }
            search.placeholder = Some(spec.search_placeholder.clone());
            // A search field inside a panel has no visible label of its own.
            search.aria_label = Some("Search references".to_string());
            panel = panel.child(text_input(&search, theme, None));
        }

        let rows = spec.rows();
        // Contract: the results are a `listbox` of `option`s.
        let mut list = Node::container();
        list.a11y.role = Some(NodeRole::ListBox);
        list.style.descriptor.layout.direction = LayoutDirection::Column;
        list.style.descriptor.layout.spacing.gap = rem_to_px(0.125);
        for (index, option) in rows.iter().enumerate() {
            if let Some(heading) = spec.group_heading_for(&rows, index) {
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
                    pad.top = rem_to_px(if index == 0 { 0.5 } else { 0.875 });
                    pad.bottom = rem_to_px(0.25);
                }
                list = list.child(h);
            }

            let is_selected = option.value == spec.value;
            let mut row = Node::container();
            // Each result row is an `option` of the listbox above it.
            row.a11y.role = Some(NodeRole::ListBoxOption);
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
            }
            all_radius(&mut row, radius);

            let mut glyph = Node::icon(option.resolved_icon(), rem_to_px(0.75));
            glyph.style.descriptor.text_color = Some(text_secondary);
            let mut row = row.child(glyph);

            let mut copy = Node::container();
            {
                let s = &mut copy.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.width = LayoutSizing::Grow;
                s.min_width = Some(0.0);
            }
            let mut title = Node::text(&option.label);
            {
                let s = &mut title.style;
                s.descriptor.text_color = Some(text_primary);
                s.text_size = Some(rem_to_px(0.875));
                s.text_weight = Some(if is_selected { 600 } else { 400 });
                s.text_ellipsis = true;
                s.no_wrap = true;
            }
            let mut copy = copy.child(title);
            if let Some(description) = &option.description {
                let mut d = Node::text(description);
                {
                    let s = &mut d.style;
                    s.descriptor.text_color = Some(text_secondary);
                    s.text_size = Some(rem_to_px(0.75));
                    s.text_ellipsis = true;
                    s.no_wrap = true;
                }
                copy = copy.child(d);
            }
            row = row.child(copy);

            if spec.is_current(option) {
                let mut current = Node::text(&spec.current_label);
                current.style.flex_none = true;
                current.style.descriptor.text_color = Some(text_secondary);
                current.style.text_size = Some(rem_to_px(0.75));
                row = row.child(current);
            }

            if option.is_disabled {
                row.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
            } else if let Some(handler) = &on_change {
                let handler = Arc::clone(handler);
                let id = option.value.clone();
                row.style.descriptor.cursor = CursorHint::Pointer;
                row.interaction.on_activate = Some(Arc::new(move || handler(&id)));
            }

            list = list.child(row);
        }
        panel = panel.child(list);

        let status_line = |content: &str| -> Node {
            // Contract: the empty and loading lines are `status`, so a screen
            // reader is told the list is empty rather than finding nothing and
            // being left to infer why.
            let mut t = Node::text(content);
            t.a11y.role = Some(NodeRole::Status);
            t.style.descriptor.text_color = Some(text_secondary);
            t.style.text_size = Some(rem_to_px(0.75));
            t
        };
        if spec.show_empty() {
            panel = panel.child(status_line(&spec.empty_label));
        }
        if spec.is_loading {
            panel = panel.child(status_line(&spec.loading_label));
        }

        let mut dialog = Node::container();
        {
            let s = &mut dialog.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.min_width = Some(rem_to_px(16.0));
            s.max_width = Some(rem_to_px(24.0));
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
        let spec = RefSelectSpec::new().with_open(true);
        let node = ref_select(&spec, &theme(), None);
        assert!(node.find(&|n| n.interaction.on_activate.is_some()).is_none());

        // Refusal: the open surface carries the inert activation marker a
        // host keys outside-dismissal on.
        let refusing = spec.with_dismiss_on_outside_interact(false);
        let node = ref_select(&refusing, &theme(), None);
        assert!(node.find(&|n| n.interaction.on_activate.is_some()).is_some());
    }
}
