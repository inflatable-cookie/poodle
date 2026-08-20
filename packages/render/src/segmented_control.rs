//! SegmentedControl — an inline choice between exclusive options.
//!
//! Contract: `docs/contracts/components/segmented-control.md`
//! Ported from: `packages/jetstream/components/src/segmented_control.rs`,
//! reconciled against the old GPUI tier
//! (`packages/gpui/components/src/primitives/segmented_control.rs`) for the
//! g12.019 node-backend migration.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeKey, NodeRole, NodeToggled, ShadowLayer, StylePatch,
};
use poodle_specs::{IconSize, SegmentedControlSpec};

use crate::color::{mix_srgb, with_alpha, TRANSPARENT};
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size,
};

pub fn segmented_control(
    spec: &SegmentedControlSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    // Fixed per-size/per-density tables, transcribed from the old GPUI tier
    // (g12.019): unlike select/button, this component's old tier is
    // deliberately fixed-table — Svelte's `--poodle-segmented-control-height`
    // / `--poodle-segmented-control-x` are overridden by the data-size /
    // data-density stops, so the token+offset re-anchor does not apply here
    // and the visual gate expects the ladder values.
    let height = rem_to_px(control_height_rem(effective_size));
    // Contract §8: label font fixed at 0.75rem for all sizes.
    let font_size = rem_to_px(0.75);
    // Contract §8: segment padding-x is density-driven
    // (`--poodle-segmented-control-x`: 0.5/0.75/1rem), not size-offset.
    let seg_px = rem_to_px(control_space_x_rem(spec.density));
    let inner = rem_to_px(0.125);
    // Segment height = control height minus the container's 0.125rem
    // top+bottom padding. The old GPUI tier sets it explicitly so the
    // selected fill spans the full inner track, and centers the label with
    // flex instead of vertical padding.
    let segment_height = height - rem_to_px(0.25);

    let selected_fill = theme.resolve_color(spec.selected_fill_token());
    let surface = theme.resolve_color("color.background.surface");
    let elevated = theme.resolve_color("color.background.elevated");
    let text_primary = theme.resolve_color("color.text.primary");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let text_inverse = theme.resolve_color("color.text.inverse");
    let text_muted = theme.resolve_color("color.text.secondary");
    let control_radius = theme.resolve_radius("radius.control");
    let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");

    // Contract: root bg = surface 93% mix with text-primary; root border =
    // border-subtle at 84% alpha.
    let root_bg = mix_srgb(surface, text_primary, 0.93);
    let root_border = with_alpha(border_subtle, border_subtle.3 * 0.84);
    // Segment hover, the old GPUI tier's recipe: surface 84% over elevated.
    let hover_fill = mix_srgb(surface, elevated, 0.84);
    // Contract §8 selected Label: `box-shadow inset 0 0.0625rem 0
    // color-mix(white 12%, transparent)`. No `white` token exists;
    // `text.inverse` is the closest semantic (white in the dark theme),
    // mixed to 12% alpha. GPUI has no inset shadow, so the old tier
    // approximates the highlight as a 1px top edge line (offset y =
    // 0.0625rem, blur 0, spread 0) — transcribed here as a non-inset layer.
    let selected_highlight = with_alpha(text_inverse, text_inverse.3 * 0.12);
    // Contract §8 Label: inner radius = calc(radius-control - 0.125rem).
    let inner_radius = (control_radius - inner).max(0.0);
    let focus_ring = theme.resolve_color("color.accent.focusRing");
    let focus_ring_width = rem_to_px(0.125);

    let selected = spec.current_value();

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(root_bg);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = root_border;
        s.descriptor.corner_radii.top_left = control_radius;
        s.descriptor.corner_radii.top_right = control_radius;
        s.descriptor.corner_radii.bottom_right = control_radius;
        s.descriptor.corner_radii.bottom_left = control_radius;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.padding.left = inner;
        s.descriptor.layout.spacing.padding.right = inner;
        s.descriptor.layout.spacing.padding.top = inner;
        s.descriptor.layout.spacing.padding.bottom = inner;
        s.descriptor.layout.spacing.gap = inner;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        // equal_width=false: content-sized, left-aligned — Start is taffy's
        // default main alignment, so silence is the faithful emission.
    }

    let icon_size = theme
        .resolve_space(IconSize::from(resolve_supporting_visual_size(effective_size)).size_token());
    // Contract §8 label gap between icon and visible text.
    let icon_text_gap = rem_to_px(0.375);
    let roving = roving_values(spec);
    let tab_stop = tab_stop_value(spec, &roving);

    for option in &spec.options {
        let is_selected = selected == Some(option.value.as_str());
        let is_enabled = !spec.is_disabled && !option.is_disabled;
        let icon_only = option.is_icon_only();

        let text_color = if is_selected {
            text_inverse
        } else {
            text_muted
        };

        // With an icon, children carry the glyph and optional label so the
        // backend does not duplicate the Button kind's intrinsic text.
        let mut seg = Node::button(if option.icon.is_some() {
            String::new()
        } else {
            option.label.clone()
        });
        {
            let s = &mut seg.style;
            s.text_size = Some(font_size);
            s.text_weight = Some(600);
            s.descriptor.text_color = Some(text_color);
            s.descriptor.layout.height = LayoutSizing::Fixed(segment_height);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.layout.spacing.padding.left = seg_px;
            s.descriptor.layout.spacing.padding.right = seg_px;
            s.descriptor.corner_radii.top_left = inner_radius;
            s.descriptor.corner_radii.top_right = inner_radius;
            s.descriptor.corner_radii.bottom_right = inner_radius;
            s.descriptor.corner_radii.bottom_left = inner_radius;
            // Old tier: the label stays on one line and truncates with an
            // ellipsis instead of wrapping or overflowing the segment.
            s.no_wrap = true;
            s.text_ellipsis = true;
            s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            if is_selected {
                s.descriptor.background = Some(selected_fill);
                s.shadow_layers = vec![ShadowLayer {
                    offset_x: 0.0,
                    offset_y: rem_to_px(0.0625),
                    blur: 0.0,
                    spread: 0.0,
                    color: selected_highlight,
                    inset: false,
                }];
            }
            if spec.equal_width {
                s.descriptor.layout.width = LayoutSizing::Grow;
            }
            if icon_only {
                // Contract §8 icon-only label: padding-inline 0; square when
                // `equalWidth=false`.
                s.descriptor.layout.spacing.padding.left = 0.0;
                s.descriptor.layout.spacing.padding.right = 0.0;
                if !spec.equal_width {
                    s.descriptor.layout.width = LayoutSizing::Fixed(segment_height);
                }
            } else if option.icon.is_some() {
                s.descriptor.layout.spacing.gap = icon_text_gap;
            }
            if is_enabled {
                // Transparent reserve so the contracted focus ring can paint
                // without a layout shift, and so GPUI tracks a focus handle
                // (`tracks_focus` requires focusable + a focus patch).
                s.descriptor.border.width = focus_ring_width;
                s.descriptor.border.color = TRANSPARENT;
                s.focus = Some(StylePatch {
                    border_color: Some(focus_ring),
                    background: None,
                    text_color: None,
                    opacity: None,
                });
                // Old tier: pointer cursor on every enabled segment, whether or
                // not a change handler is wired.
                s.descriptor.cursor = CursorHint::Pointer;
                // Hover fill is UNSELECTED-only. The contract's selected state
                // is "accent background, inverse text, inset highlight shadow",
                // and a hover patch replaces the background outright — so
                // hovering the selected segment used to wipe the accent and
                // paint it the neutral hover fill instead. Svelte has no
                // `:hover` rule for segments at all, so nothing is owed to the
                // selected one here.
                if !is_selected {
                    s.hover = Some(StylePatch {
                        background: Some(hover_fill),
                        border_color: None,
                        text_color: None,
                        opacity: None,
                    });
                }
            }
        }

        if let Some(icon_name) = option.icon.as_deref() {
            let mut glyph = Node::icon(icon_name, icon_size);
            glyph.style.descriptor.text_color = Some(text_color);
            seg = seg.child(glyph);
            if !icon_only {
                let mut label = Node::text(&option.label);
                label.style.text_size = Some(font_size);
                label.style.text_weight = Some(600);
                label.style.descriptor.text_color = Some(text_color);
                label.style.no_wrap = true;
                label.style.min_width = Some(0.0);
                seg = seg.child(label);
            }
        }

        seg.id = Some(segment_id(&option.value));
        seg.runtime_id = Some(segment_focus_id(spec.instance_id.as_deref(), &option.value));
        seg.a11y.role = Some(NodeRole::RadioButton);
        seg.a11y.selected = Some(is_selected);
        seg.a11y.toggled = Some(if is_selected {
            NodeToggled::True
        } else {
            NodeToggled::False
        });

        if is_enabled {
            seg.interaction.focusable = true;
            seg.a11y.tab_index = Some(if tab_stop == Some(option.value.as_str()) {
                0
            } else {
                -1
            });
            // Re-picking the current segment still fires: the host asked to
            // be told about clicks, and swallowing one would hide a "confirm".
            if let Some(handler) = &on_change {
                let handler = Arc::clone(handler);
                let value = option.value.clone();
                seg.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }
            seg.interaction.on_key = roving_key_handler(
                &option.value,
                &roving,
                spec.instance_id.clone(),
                on_change.clone(),
            );
        } else {
            seg.interaction.disabled = true;
            seg.interaction.focusable = false;
            seg.a11y.tab_index = Some(-1);
            if option.is_disabled && !spec.is_disabled {
                seg.style.descriptor.opacity = disabled_opacity;
                seg.style.descriptor.cursor = CursorHint::NotAllowed;
            }
        }

        if let Some(name) = option.accessible_name_override() {
            seg.a11y.label = Some(name.to_string());
        }
        if let Some(title) = option.tooltip_text() {
            seg.tooltip = Some(title.to_string());
        }

        el = el.child(seg);
    }

    if spec.is_disabled {
        el.style.descriptor.opacity = disabled_opacity;
        el.style.descriptor.cursor = CursorHint::NotAllowed;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.role = Some(NodeRole::RadioGroup);
    el
}

fn segment_id(value: &str) -> String {
    format!("segmented:{value}")
}

fn segment_focus_id(instance_id: Option<&str>, value: &str) -> String {
    match instance_id {
        Some(scope) => format!("segmented:{scope}:option:{value}"),
        None => segment_id(value),
    }
}

fn roving_values(spec: &SegmentedControlSpec) -> Vec<String> {
    spec.options
        .iter()
        .filter(|option| !spec.is_disabled && !option.is_disabled)
        .map(|option| option.value.clone())
        .collect()
}

fn tab_stop_value<'a>(spec: &'a SegmentedControlSpec, roving: &'a [String]) -> Option<&'a str> {
    let selected = spec.current_value();
    if selected.is_some_and(|value| roving.iter().any(|candidate| candidate == value)) {
        selected
    } else {
        roving.first().map(String::as_str)
    }
}

fn roving_key_handler(
    value: &str,
    roving: &[String],
    instance_id: Option<String>,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Option<Arc<dyn Fn(NodeKey, poodle_node::NodeModifiers) -> Option<String> + Send + Sync>> {
    let index = roving.iter().position(|candidate| candidate == value)?;
    let ids = roving.to_vec();
    Some(Arc::new(move |key, _modifiers| {
        if ids.is_empty() {
            return None;
        }
        let last = ids.len() - 1;
        let next = match key {
            NodeKey::ArrowRight | NodeKey::ArrowDown => {
                Some(if index == last { 0 } else { index + 1 })
            }
            NodeKey::ArrowLeft | NodeKey::ArrowUp => {
                Some(if index == 0 { last } else { index - 1 })
            }
            NodeKey::Home => Some(0),
            NodeKey::End => Some(last),
            _ => None,
        }?;
        let target = ids[next].clone();
        if let Some(handler) = &on_change {
            handler(&target);
        }
        Some(segment_focus_id(instance_id.as_deref(), &target))
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeModifiers;
    use poodle_specs::{ControlDensity, ControlSize, SegmentedControlOption};

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn view_options() -> Vec<SegmentedControlOption> {
        vec![
            SegmentedControlOption::new("grid", "Grid"),
            SegmentedControlOption::new("list", "List"),
            SegmentedControlOption::new("table", "Table"),
        ]
    }

    fn find_segment<'a>(node: &'a Node, label: &str) -> &'a Node {
        node.find(&|n| matches!(&n.kind, poodle_node::NodeKind::Button { label: l } if l == label))
            .unwrap_or_else(|| panic!("segment {label:?} exists"))
    }

    #[test]
    fn track_metrics_follow_the_old_tiers_fixed_tables() {
        // height = the fixed per-size ladder (`control_height_rem` — the
        // old GPUI tier's deliberate form for this component, matching
        // Svelte's data-size stops, not select's token+offset); segment
        // height = track height minus the container's 0.125rem top+bottom
        // padding. Label font is a fixed 0.75rem for every size
        // (contract §8).
        let cases = [
            (ControlSize::Xs, 24.0),
            (ControlSize::Sm, 28.0),
            (ControlSize::Md, 36.0),
            (ControlSize::Lg, 44.0),
            (ControlSize::Xl, 52.0),
        ];
        for (size, expected) in cases {
            let spec = SegmentedControlSpec::new(view_options()).with_size(size);
            let node = segmented_control(&spec, &theme(), None);
            match node.style.descriptor.layout.height {
                LayoutSizing::Fixed(h) => assert_eq!(h, expected, "track height for {size:?}"),
                ref other => panic!("expected fixed track height, got {other:?}"),
            }
            let seg = find_segment(&node, "List");
            match seg.style.descriptor.layout.height {
                LayoutSizing::Fixed(h) => {
                    assert_eq!(h, expected - 4.0, "segment height for {size:?}")
                }
                ref other => panic!("expected fixed segment height, got {other:?}"),
            }
            assert_eq!(seg.style.text_size, Some(12.0), "font for {size:?}");
        }
    }

    #[test]
    fn segment_padding_is_density_driven() {
        // padding-x = the fixed per-density table (`control_space_x_rem`:
        // 0.5/0.75/1rem = 8/12/16px), the old GPUI tier's form, matching
        // Svelte's data-density stops.
        let cases = [
            (ControlDensity::Compact, 8.0),
            (ControlDensity::Default, 12.0),
            (ControlDensity::Comfortable, 16.0),
        ];
        for (density, expected) in cases {
            let spec = SegmentedControlSpec::new(view_options()).with_density(density);
            let node = segmented_control(&spec, &theme(), None);
            let seg = find_segment(&node, "List");
            assert_eq!(
                seg.style.descriptor.layout.spacing.padding.left, expected,
                "padding-x for {density:?}"
            );
            assert_eq!(seg.style.descriptor.layout.spacing.padding.right, expected);
        }
    }

    #[test]
    fn selected_segment_gets_accent_fill_and_top_highlight() {
        let theme = theme();
        let accent = theme.resolve_color("color.accent.base");
        let text_inverse = theme.resolve_color("color.text.inverse");
        let text_secondary = theme.resolve_color("color.text.secondary");

        let spec = SegmentedControlSpec::new(view_options()).with_default_value("list");
        let node = segmented_control(&spec, &theme, None);

        let selected_seg = find_segment(&node, "List");
        assert_eq!(selected_seg.style.descriptor.background, Some(accent));
        assert_eq!(selected_seg.style.descriptor.text_color, Some(text_inverse));
        // The contract's inset highlight, transcribed the old tier's way: a
        // non-inset 1px top line in text.inverse at 12% alpha (GPUI has no
        // inset shadow).
        assert_eq!(
            selected_seg.style.shadow_layers,
            vec![ShadowLayer {
                offset_x: 0.0,
                offset_y: 1.0,
                blur: 0.0,
                spread: 0.0,
                color: with_alpha(text_inverse, text_inverse.3 * 0.12),
                inset: false,
            }]
        );

        let unselected_seg = find_segment(&node, "Grid");
        assert_eq!(unselected_seg.style.descriptor.background, None);
        assert_eq!(
            unselected_seg.style.descriptor.text_color,
            Some(text_secondary)
        );
        assert!(unselected_seg.style.shadow_layers.is_empty());
    }

    #[test]
    fn enabled_segments_show_pointer_cursor_and_hover_fill_without_a_handler() {
        let theme = theme();
        let surface = theme.resolve_color("color.background.surface");
        let elevated = theme.resolve_color("color.background.elevated");
        let hover_fill = mix_srgb(surface, elevated, 0.84);

        // No change handler wired: the old tier still shows the affordances.
        let spec = SegmentedControlSpec::new(view_options()).with_default_value("grid");
        let node = segmented_control(&spec, &theme, None);
        // "Grid" is the selected segment and is deliberately excluded below.
        for label in ["List", "Table"] {
            let seg = find_segment(&node, label);
            assert_eq!(seg.style.descriptor.cursor, CursorHint::Pointer, "{label}");
            assert_eq!(
                seg.style.hover,
                Some(StylePatch {
                    background: Some(hover_fill),
                    border_color: None,
                    text_color: None,
                    opacity: None,
                }),
                "{label}"
            );
            assert!(seg.interaction.focusable, "{label}");
            assert!(seg.interaction.on_activate.is_none(), "{label}");
        }
    }

    #[test]
    fn hovering_the_selected_segment_keeps_its_accent_fill() {
        // Regression: the hover patch was applied to every enabled segment,
        // including the selected one. A patch replaces the background outright,
        // so hovering the selected segment swapped its accent fill for the
        // neutral hover fill and the selection visually vanished.
        let theme = theme();
        let accent = theme.resolve_color("color.accent.base");

        let spec = SegmentedControlSpec::new(view_options()).with_default_value("grid");
        let node = segmented_control(&spec, &theme, None);

        let selected = find_segment(&node, "Grid");
        assert_eq!(selected.style.descriptor.background, Some(accent));
        assert!(
            selected.style.hover.is_none(),
            "the selected segment must not carry a hover patch that would \
             replace its accent fill"
        );
        // The affordance still exists everywhere it does not conflict.
        assert!(find_segment(&node, "List").style.hover.is_some());
        assert_eq!(
            selected.style.descriptor.cursor,
            CursorHint::Pointer,
            "the selected segment is still clickable"
        );
    }

    #[test]
    fn disabled_option_is_out_of_traversal_and_dimmed() {
        let theme = theme();
        let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
        let mut options = view_options();
        options.push(SegmentedControlOption::new("draft", "Draft").with_disabled(true));
        let handlers: Arc<dyn Fn(&str) + Send + Sync> = Arc::new(|_| {});
        let spec = SegmentedControlSpec::new(options).with_default_value("grid");
        let node = segmented_control(&spec, &theme, Some(handlers));

        let draft = find_segment(&node, "Draft");
        assert!(draft.interaction.on_activate.is_none());
        assert!(draft.interaction.disabled);
        assert!(!draft.interaction.focusable);
        assert_eq!(draft.a11y.tab_index, Some(-1));
        assert_eq!(draft.a11y.role, Some(NodeRole::RadioButton));
        assert_eq!(draft.style.descriptor.cursor, CursorHint::NotAllowed);
        assert!(draft.style.hover.is_none());
        assert_eq!(draft.style.descriptor.opacity, disabled_opacity);

        // Siblings stay wired and in the roving set.
        let list = find_segment(&node, "List");
        assert!(list.interaction.on_activate.is_some());
        assert_eq!(list.a11y.tab_index, Some(-1));
        assert!(list.interaction.focusable);
    }

    #[test]
    fn disabled_control_dims_the_track_and_shows_not_allowed() {
        let theme = theme();
        let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
        let handlers: Arc<dyn Fn(&str) + Send + Sync> = Arc::new(|_| {});
        let spec = SegmentedControlSpec {
            is_disabled: true,
            ..SegmentedControlSpec::new(view_options()).with_default_value("grid")
        };
        let node = segmented_control(&spec, &theme, Some(handlers));

        assert_eq!(node.style.descriptor.opacity, disabled_opacity);
        assert_eq!(node.style.descriptor.cursor, CursorHint::NotAllowed);
        for label in ["Grid", "List", "Table"] {
            let seg = find_segment(&node, label);
            assert!(seg.interaction.on_activate.is_none(), "{label}");
            assert!(seg.interaction.disabled, "{label}");
            assert!(!seg.interaction.focusable, "{label}");
            assert_eq!(seg.a11y.tab_index, Some(-1), "{label}");
            assert!(seg.style.hover.is_none(), "{label}");
            assert_ne!(seg.style.descriptor.cursor, CursorHint::Pointer, "{label}");
        }
    }

    #[test]
    fn choosing_a_segment_reports_its_value_through_the_node_handler() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()));
        let spec = SegmentedControlSpec::new(view_options()).with_default_value("grid");
        let node = segmented_control(&spec, &theme(), Some(on_change));

        let list = find_segment(&node, "List");
        (list.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), ["list"]);
    }

    #[test]
    fn equal_width_grows_segments_content_fit_leaves_them_alone() {
        let spec = SegmentedControlSpec::new(view_options());
        let node = segmented_control(&spec, &theme(), None);
        let seg = find_segment(&node, "List");
        assert_eq!(seg.style.descriptor.layout.width, LayoutSizing::Grow);

        let spec = SegmentedControlSpec::new(view_options()).with_equal_width(false);
        let node = segmented_control(&spec, &theme(), None);
        let seg = find_segment(&node, "List");
        assert_ne!(seg.style.descriptor.layout.width, LayoutSizing::Grow);
    }

    #[test]
    fn radiogroup_role_and_aria_label_ride_the_root() {
        let spec = SegmentedControlSpec::new(view_options());
        let mut spec_with_label = spec.clone();
        spec_with_label.aria_label = Some("View mode".to_string());
        let node = segmented_control(&spec_with_label, &theme(), None);
        assert_eq!(node.a11y.role, Some(NodeRole::RadioGroup));
        assert_eq!(node.a11y.label.as_deref(), Some("View mode"));
        let grid = find_segment(&node, "Grid");
        assert_eq!(grid.a11y.role, Some(NodeRole::RadioButton));
        assert_eq!(grid.a11y.selected, Some(false));
        assert_eq!(grid.a11y.toggled, Some(NodeToggled::False));
        assert_eq!(grid.a11y.tab_index, Some(0));
        let list = find_segment(&node, "List");
        assert_eq!(list.a11y.tab_index, Some(-1));
        assert!(list.interaction.focusable);
    }

    #[test]
    fn selected_option_is_the_roving_tab_stop() {
        let spec = SegmentedControlSpec::new(view_options()).with_default_value("list");
        let node = segmented_control(&spec, &theme(), None);
        assert_eq!(find_segment(&node, "Grid").a11y.tab_index, Some(-1));
        let list = find_segment(&node, "List");
        assert_eq!(list.a11y.tab_index, Some(0));
        assert_eq!(list.a11y.selected, Some(true));
        assert_eq!(list.a11y.toggled, Some(NodeToggled::True));
        assert_eq!(find_segment(&node, "Table").a11y.tab_index, Some(-1));
    }

    #[test]
    fn arrow_keys_move_selection_through_enabled_options_and_skip_disabled() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()));
        let spec = SegmentedControlSpec::new(vec![
            SegmentedControlOption::new("grid", "Grid"),
            SegmentedControlOption::new("list", "List").with_disabled(true),
            SegmentedControlOption::new("table", "Table"),
        ])
        .with_default_value("grid")
        .with_instance_id("view");
        let node = segmented_control(&spec, &theme(), Some(on_change));
        let keys = find_segment(&node, "Grid")
            .interaction
            .on_key
            .as_ref()
            .expect("roving handler");
        let modifiers = NodeModifiers::default();
        assert_eq!(
            keys(NodeKey::ArrowRight, modifiers),
            Some(segment_focus_id(Some("view"), "table"))
        );
        assert_eq!(seen.lock().unwrap().as_slice(), ["table"]);
        assert!(find_segment(&node, "List").interaction.on_key.is_none());
    }

    #[test]
    fn enabled_segments_carry_a_focus_patch_so_gpui_tracks_handles() {
        let spec = SegmentedControlSpec::new(view_options()).with_default_value("grid");
        let node = segmented_control(&spec, &theme(), None);
        let grid = find_segment(&node, "Grid");
        assert!(grid.style.focus.is_some());
        assert_eq!(
            grid.style.focus.and_then(|patch| patch.border_color),
            Some(theme().resolve_color("color.accent.focusRing"))
        );
    }

    #[test]
    fn instance_scope_keeps_roving_focus_inside_the_originating_control() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()));
        let a = segmented_control(
            &SegmentedControlSpec::new(view_options())
                .with_default_value("grid")
                .with_instance_id("a"),
            &theme(),
            Some(Arc::clone(&on_change)),
        );
        let b = segmented_control(
            &SegmentedControlSpec::new(view_options())
                .with_default_value("grid")
                .with_instance_id("b"),
            &theme(),
            Some(on_change),
        );
        let a_grid = find_segment(&a, "Grid");
        let b_grid = find_segment(&b, "Grid");
        assert_eq!(a_grid.id.as_deref(), Some("segmented:grid"));
        assert_eq!(b_grid.id.as_deref(), Some("segmented:grid"));
        assert_eq!(
            a_grid.runtime_id.as_deref(),
            Some("segmented:a:option:grid")
        );
        assert_eq!(
            b_grid.runtime_id.as_deref(),
            Some("segmented:b:option:grid")
        );
        let modifiers = NodeModifiers::default();
        assert_eq!(
            (a_grid.interaction.on_key.as_ref().unwrap())(NodeKey::ArrowRight, modifiers),
            Some("segmented:a:option:list".to_string())
        );
        assert_eq!(
            (b_grid.interaction.on_key.as_ref().unwrap())(NodeKey::ArrowRight, modifiers),
            Some("segmented:b:option:list".to_string())
        );
        assert_eq!(seen.lock().unwrap().as_slice(), ["list", "list"]);
        assert!(a_grid.style.focus.is_some());
        assert!(b_grid.style.focus.is_some());
    }

    #[test]
    fn custom_option_icon_names_are_preserved() {
        let spec = SegmentedControlSpec::new(vec![SegmentedControlOption::new("grid", "Grid")
            .with_icon("company-logo")
            .with_icon_only(true)]);
        let node = segmented_control(&spec, &theme(), None);
        assert!(find_icon_segment(&node, "company-logo")
            .find(
                &|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "company-logo")
            )
            .is_some());
    }

    fn plugin_kind_options() -> Vec<SegmentedControlOption> {
        vec![
            SegmentedControlOption::new("effects", "Effects")
                .with_icon("audio-waveform")
                .with_icon_only(true),
            SegmentedControlOption::new("instruments", "Instruments")
                .with_icon("piano")
                .with_icon_only(true),
        ]
    }

    fn find_icon_segment<'a>(node: &'a Node, icon: &str) -> &'a Node {
        node.children
            .iter()
            .find(|seg| {
                seg.find(
                    &|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == icon),
                )
                .is_some()
            })
            .unwrap_or_else(|| panic!("segment with icon {icon:?} exists"))
    }

    fn icon_size_of(node: &Node, name: &str) -> f32 {
        match &node
            .find(&|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name: n, .. } if n == name))
            .expect("icon exists")
            .kind
        {
            poodle_node::NodeKind::Icon { size, .. } => *size,
            _ => unreachable!(),
        }
    }

    #[test]
    fn icon_only_hides_visible_label_and_falls_back_for_name_and_tooltip() {
        let spec = SegmentedControlSpec::new(plugin_kind_options())
            .with_default_value("effects")
            .with_equal_width(false);
        let node = segmented_control(&spec, &theme(), None);

        let effects = find_icon_segment(&node, "audio-waveform");
        assert!(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets/icons/audio-waveform.svg")
                .is_file(),
            "the Effects icon must have a native paint asset"
        );
        assert!(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets/icons/piano.svg")
                .is_file(),
            "the Instruments icon must have a native paint asset"
        );
        assert!(
            !node.has_text("Effects"),
            "icon-only must not expose the required label as visible text"
        );
        assert!(
            !node.has_text("Instruments"),
            "icon-only must not expose the required label as visible text"
        );
        assert_eq!(effects.a11y.label.as_deref(), Some("Effects"));
        assert_eq!(effects.tooltip.as_deref(), Some("Effects"));
        assert!(matches!(
            &effects.kind,
            poodle_node::NodeKind::Button { label } if label.is_empty()
        ));
        assert!(effects
            .find(&|n| matches!(
                &n.kind,
                poodle_node::NodeKind::Text { content } if content == "Effects"
            ))
            .is_none());
    }

    #[test]
    fn icon_only_without_an_icon_keeps_the_visible_label() {
        let spec = SegmentedControlSpec::new(vec![
            SegmentedControlOption::new("grid", "Grid").with_icon_only(true)
        ]);
        let node = segmented_control(&spec, &theme(), None);
        assert!(node.has_text("Grid"));
        let seg = find_segment(&node, "Grid");
        assert!(seg.a11y.label.is_none());
        assert!(seg.tooltip.is_none());
        assert!(seg
            .find(&|n| matches!(&n.kind, poodle_node::NodeKind::Icon { .. }))
            .is_none());
    }

    #[test]
    fn labelled_icon_renders_before_text_with_supporting_visual_size_and_gap() {
        let theme = theme();
        let expected = theme.resolve_space(
            poodle_specs::IconSize::from(crate::presentation::resolve_supporting_visual_size(
                ControlSize::Md,
            ))
            .size_token(),
        );
        let spec = SegmentedControlSpec::new(vec![
            SegmentedControlOption::new("grid", "Grid").with_icon("list")
        ]);
        let node = segmented_control(&spec, &theme, None);
        let seg = find_icon_segment(&node, "list");
        assert!(node.has_text("Grid"));
        assert_eq!(icon_size_of(&node, "list"), expected);
        assert_eq!(seg.style.descriptor.layout.spacing.gap, rem_to_px(0.375));
        assert_eq!(
            seg.children
                .iter()
                .map(|child| match &child.kind {
                    poodle_node::NodeKind::Icon { name, .. } => name.as_str(),
                    poodle_node::NodeKind::Text { content } => content.as_str(),
                    _ => "other",
                })
                .collect::<Vec<_>>(),
            ["list", "Grid"]
        );
    }

    #[test]
    fn icon_only_content_fit_is_square_with_no_inline_padding() {
        let spec = SegmentedControlSpec::new(plugin_kind_options())
            .with_default_value("effects")
            .with_equal_width(false);
        let node = segmented_control(&spec, &theme(), None);
        let effects = find_icon_segment(&node, "audio-waveform");
        assert_eq!(
            effects.style.descriptor.layout.width,
            LayoutSizing::Fixed(32.0),
            "md track 36px minus 0.25rem padding"
        );
        assert_eq!(
            effects.style.descriptor.layout.height,
            LayoutSizing::Fixed(32.0)
        );
        assert_eq!(effects.style.descriptor.layout.spacing.padding.left, 0.0);
        assert_eq!(effects.style.descriptor.layout.spacing.padding.right, 0.0);
    }

    #[test]
    fn explicit_aria_label_and_title_win_over_the_required_label() {
        let spec = SegmentedControlSpec::new(vec![SegmentedControlOption::new("fx", "FX")
            .with_icon("audio-waveform")
            .with_icon_only(true)
            .with_aria_label("Audio effects")
            .with_title("Effects plugins")]);
        let node = segmented_control(&spec, &theme(), None);
        let seg = find_icon_segment(&node, "audio-waveform");
        assert_eq!(seg.a11y.label.as_deref(), Some("Audio effects"));
        assert_eq!(seg.tooltip.as_deref(), Some("Effects plugins"));
        assert!(!node.has_text("FX"));
    }

    #[test]
    fn icon_only_activation_still_reports_the_option_value() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()));
        let spec = SegmentedControlSpec::new(plugin_kind_options())
            .with_default_value("effects")
            .with_equal_width(false);
        let node = segmented_control(&spec, &theme(), Some(on_change));
        let instruments = find_icon_segment(&node, "piano");
        (instruments.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), ["instruments"]);
    }
}
