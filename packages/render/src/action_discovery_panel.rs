//! ActionDiscoveryPanel — grouped actions with shortcuts.
//!
//! Contract: `docs/contracts/components/action-discovery-panel.md`
//! Ported from: `packages/jetstream/components/src/action_discovery_panel.rs`.
//!
//! Renders grouped action sections (Eyebrow heading + action rows), the
//! active-item accent state, badge/kbd chips, and the loading/error/empty/
//! no-results discovery states. Keyboard navigation / item activation stay
//! host-driven; the component renders at the current resolved state.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, MainAxisAlignment, Node, NodeRole,
    ShadowLayer,
};
use poodle_specs::{
    ActionDiscoveryPanelSpec, ControlDensity, ControlSize, DiscoveryState, EmptyStateSpec,
    EmptyStateVariant, EyebrowSpec, SkeletonSpec,
};

use crate::color::{mix_srgb, with_alpha};
use crate::empty_state::empty_state;
use crate::eyebrow::eyebrow;
use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::skeleton::skeleton;

/// Per-size chip dimensions (`chip-height`, `chip-x`, `chip-font-size`) for
/// the badge/kbd pills — contract §9 "Chip size table" (rem).
fn chip_dims(size: ControlSize) -> (f32, f32, f32) {
    match size {
        ControlSize::Xs => (1.125, 0.375, 0.5625),
        ControlSize::Sm => (1.25, 0.5, 0.625),
        ControlSize::Md => (1.375, 0.5, 0.6875),
        ControlSize::Lg => (1.5, 0.625, 0.75),
        ControlSize::Xl => (1.75, 0.75, 0.8125),
    }
}

/// Density-aware chip gap (`--poodle-action-discovery-chip-gap`, rem).
fn chip_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.25,
        ControlDensity::Default => 0.375,
        ControlDensity::Comfortable => 0.5,
    }
}

/// Density-aware skeleton padding (`--poodle-action-discovery-skeleton-pad`, rem).
fn skeleton_pad_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.625,
        ControlDensity::Default => 0.875,
        ControlDensity::Comfortable => 1.0,
    }
}

/// Per-size row x/y padding (`--poodle-action-discovery-row-x/y`, rem).
fn row_pad_rem(size: ControlSize) -> (f32, f32) {
    match size {
        ControlSize::Xs => (0.5, 0.25),
        ControlSize::Sm => (0.5, 0.3125),
        ControlSize::Md => (0.625, 0.375),
        ControlSize::Lg => (0.75, 0.5),
        ControlSize::Xl => (0.875, 0.625),
    }
}

/// `on_select` fires with the chosen action's id. Disabled actions never fire.
pub fn action_discovery_panel(
    spec: &ActionDiscoveryPanelSpec,
    theme: &dyn ThemeProvider,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let item_gap = rem_to_px(control_space_x_rem(spec.density));
    let (row_x_rem, row_y_rem) = row_pad_rem(effective_size);
    let row_x = rem_to_px(row_x_rem);
    let row_y = rem_to_px(row_y_rem);
    let (chip_h, chip_x, chip_font) = chip_dims(effective_size);
    let chip_gap = rem_to_px(chip_gap_rem(spec.density));
    let skeleton_pad = rem_to_px(skeleton_pad_rem(spec.density));
    let group_gap = rem_to_px(0.375);
    let list_gap = rem_to_px(0.25);

    let gap = theme.resolve_space(spec.gap_token());
    let radius_control = theme.resolve_radius("radius.control");

    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");
    let elevated = theme.resolve_color("color.background.elevated");
    let surface = theme.resolve_color("color.background.surface");
    let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
    let label_size = theme.resolve_space("typography.label.size");
    let caption_size = theme.resolve_space("typography.caption.size");

    // Active-item accent treatment (contract §9):
    //   bg   = accent 18% over background-elevated
    //   ring = inset accent 22%
    let active_bg = mix_srgb(accent, elevated, 0.18);
    let active_ring = with_alpha(accent, accent.3 * 0.22);

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };

    let mut panel = Node::container();
    panel.style.descriptor.layout.direction = LayoutDirection::Column;
    panel.style.descriptor.layout.spacing.gap = gap;
    panel.style.fill_width = true;
    let mut panel = panel;

    match spec.state {
        DiscoveryState::Loading => {
            // 5 skeleton rows; two skeletons per row (48% / 20% width).
            let mut skeletons = Node::container();
            {
                let s = &mut skeletons.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = list_gap;
                s.fill_width = true;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = skeleton_pad;
                pad.right = skeleton_pad;
                pad.top = skeleton_pad;
                pad.bottom = skeleton_pad;
            }
            let mut skeletons = skeletons;
            for _ in 0..5 {
                let wide = skeleton(
                    &SkeletonSpec::new().with_width("48%").with_animated(true),
                    theme,
                );
                let narrow = skeleton(
                    &SkeletonSpec::new().with_width("20%").with_animated(true),
                    theme,
                );
                // Skeleton width strings like "48%" aren't parsed by skeleton
                // (rem/px only); express the proportions via flex sizing.
                let cell = |basis: f32, child: Node| -> Node {
                    let mut c = Node::container();
                    // Explicit Row (see switch.rs).
                    c.style.descriptor.layout.direction = LayoutDirection::Row;
                    c.style.flex_basis = Some(basis);
                    c.child(child)
                };
                let mut row = Node::container();
                {
                    let s = &mut row.style;
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
                    s.descriptor.layout.spacing.gap = chip_gap;
                }
                skeletons = skeletons.child(
                    row.child(cell(rem_to_px(8.0), wide))
                        .child(cell(rem_to_px(3.5), narrow)),
                );
            }
            // __state wrapper: min-height 10rem, centered.
            let mut state = Node::container();
            {
                let s = &mut state.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.min_height = Some(rem_to_px(10.0));
                s.fill_width = true;
            }
            return panel.child(state.child(skeletons));
        }
        DiscoveryState::Error => {
            return panel.child(empty_state(
                &EmptyStateSpec::new("Could not load actions")
                    .with_message("Actions could not be loaded. Try again.")
                    .with_compact(true),
                theme,
            ));
        }
        DiscoveryState::Empty => {
            let title = spec
                .empty_message
                .as_deref()
                .unwrap_or("No actions available");
            return panel.child(empty_state(
                &EmptyStateSpec::new(title)
                    .with_message("No actions are available in this context.")
                    .with_compact(true),
                theme,
            ));
        }
        DiscoveryState::NoResults => {
            return panel.child(empty_state(
                &EmptyStateSpec::new("No matching actions")
                    .with_message("No actions match the current search.")
                    .with_variant(EmptyStateVariant::Search)
                    .with_compact(true),
                theme,
            ));
        }
        DiscoveryState::Ready => {}
    }

    // Render each section (group: Eyebrow heading + list of items).
    for section in &spec.sections {
        let mut section_el = Node::container();
        section_el.style.descriptor.layout.direction = LayoutDirection::Column;
        section_el.style.descriptor.layout.spacing.gap = group_gap;
        let mut section_el = section_el;

        // Section heading via the Eyebrow primitive.
        section_el = section_el.child(eyebrow(
            &EyebrowSpec::new().with_content(&section.title),
            theme,
        ));

        // Optional section description.
        if let Some(ref desc) = section.description {
            let mut d = Node::text(desc);
            {
                let s = &mut d.style;
                s.descriptor.text_color = Some(text_secondary);
                s.text_size = Some(label_size);
                s.descriptor.layout.spacing.padding.left = row_x;
                s.descriptor.layout.spacing.padding.right = row_x;
            }
            section_el = section_el.child(d);
        }

        // List of action items.
        // Contract: the actions in a section are a `listbox` of `option`s.
        let mut list = Node::container();
        list.a11y.role = Some(NodeRole::ListBox);
        list.style.descriptor.layout.direction = LayoutDirection::Column;
        list.style.descriptor.layout.spacing.gap = list_gap;

        for action in &section.actions {
            let is_active = spec.active_id.as_deref() == Some(action.id.as_str());

            // Left: title + optional subtitle (description).
            let mut text_block = Node::container();
            {
                let s = &mut text_block.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = rem_to_px(0.125);
                s.min_width = Some(0.0);
                // Old tier `.flex_grow()` — grow without cross-stretch.
                s.flex_fill = true;
            }
            let mut title = Node::text(&action.title);
            title.style.descriptor.text_color = Some(text_primary);
            title.style.text_size = Some(font_size);
            title.style.text_ellipsis = true;
            let mut text_block = text_block.child(title);
            if let Some(ref desc) = action.description {
                let mut d = Node::text(desc);
                d.style.descriptor.text_color = Some(text_secondary);
                d.style.text_size = Some(caption_size);
                d.style.text_ellipsis = true;
                text_block = text_block.child(d);
            }

            let mut row = Node::container();
            // Each action is an `option` of the section's listbox.
            row.a11y.role = Some(NodeRole::ListBoxOption);
            row.a11y.selected = Some(is_active);
            row.id = Some(action.id.clone());
            {
                let s = &mut row.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
                s.descriptor.layout.spacing.gap = item_gap;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = row_x;
                pad.right = row_x;
                pad.top = row_y;
                pad.bottom = row_y;
                // Active item (contract §9): accent-tinted bg + an inset accent
                // ring (doesn't shift layout vs other rows).
                if is_active {
                    s.descriptor.background = Some(active_bg);
                    s.shadow_layers = vec![ShadowLayer {
                        offset_x: 0.0,
                        offset_y: 0.0,
                        blur: 0.0,
                        spread: rem_to_px(0.0625),
                        color: active_ring,
                        inset: true,
                    }];
                }
            }
            all_radius(&mut row, radius_control);
            let mut row = row.child(text_block);

            // Trailing snippet — badge chip + kbd chip.
            let has_badge = action.badge.is_some();
            let has_shortcut = action.shortcut.is_some();
            if has_badge || has_shortcut {
                let mut trailing = Node::container();
                {
                    let s = &mut trailing.style;
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    s.descriptor.layout.spacing.gap = chip_gap;
                    s.flex_none = true;
                }
                let mut trailing = trailing;

                let chip_shell = |bg| -> Node {
                    let mut c = Node::container();
                    {
                        let s = &mut c.style;
                        // Explicit Row (see switch.rs).
                        s.descriptor.layout.direction = LayoutDirection::Row;
                        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                        s.min_height = Some(rem_to_px(chip_h));
                        let pad = &mut s.descriptor.layout.spacing.padding;
                        pad.left = rem_to_px(chip_x);
                        pad.right = rem_to_px(chip_x);
                        s.descriptor.background = Some(bg);
                        s.no_wrap = true;
                    }
                    all_radius(&mut c, radius_control);
                    c
                };

                if let Some(ref badge) = action.badge {
                    // Accent override: accent 16% bg, accent text, uppercase.
                    let mut label = Node::text(badge.to_uppercase());
                    {
                        let s = &mut label.style;
                        s.descriptor.text_color = Some(accent);
                        s.text_size = Some(rem_to_px(chip_font));
                        s.text_weight = Some(600);
                        s.letter_spacing_em = Some(0.03); // contract §9 chip: 0.03em
                    }
                    trailing = trailing
                        .child(chip_shell(with_alpha(accent, accent.3 * 0.16)).child(label));
                }

                if let Some(ref shortcut) = action.shortcut {
                    // Kbd: surface 76% bg, secondary text, monospace.
                    let mut label = Node::text(shortcut);
                    {
                        let s = &mut label.style;
                        s.descriptor.text_color = Some(text_secondary);
                        s.text_size = Some(rem_to_px(chip_font));
                        s.text_weight = Some(600);
                        // Contract §9 kbd override: code-family (monospace).
                        s.font_family = Some(FontFamily::Mono);
                    }
                    trailing = trailing
                        .child(chip_shell(with_alpha(surface, surface.3 * 0.76)).child(label));
                }

                row = row.child(trailing);
            }

            // Disabled: reduce opacity via token, not a hardcoded value.
            if action.is_disabled {
                row.style.descriptor.opacity = disabled_opacity;
            } else {
                row.style.descriptor.cursor = CursorHint::Pointer;
                row.interaction.focusable = true;
                if let Some(handler) = &on_select {
                    let handler = Arc::clone(handler);
                    let id = action.id.clone();
                    row.interaction.on_activate = Some(Arc::new(move || handler(&id)));
                }
            }

            list = list.child(row);
        }

        section_el = section_el.child(list);
        panel = panel.child(section_el);
    }

    panel
}
