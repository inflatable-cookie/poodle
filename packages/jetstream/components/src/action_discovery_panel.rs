//! ActionDiscoveryPanel — Jetstream discovery surface backed by ActionDiscoveryPanelSpec.
//!
//! Renders grouped action sections (Eyebrow heading + action rows), the
//! active-item accent state, badge/kbd chips, and the loading/error/empty/
//! no-results discovery states. Mirrors the Svelte `ActionDiscoveryPanel`
//! anatomy and contract §9 token table.
//!
//! Keyboard navigation / item activation are parent (event-loop) driven and
//! are not wired here — the component renders at the current resolved state
//! (`spec.active_id`, `spec.state`). No ARIA channel exists in Jetstream.
use jetstream_runtime::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ActionDiscoveryPanelSpec, ControlDensity, ControlSize, DiscoveryState, EmptyStateSpec,
    EmptyStateVariant, EyebrowSpec, SkeletonSpec,
};

use crate::empty_state::js_empty_state;
use crate::eyebrow::js_eyebrow;
use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::skeleton::js_skeleton;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

/// Per-size chip dimensions (`chip-height`, `chip-x`, `chip-font-size`) for the
/// badge/kbd pills — contract §9 "Chip size table" (rem).
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

pub fn js_action_discovery_panel(
    spec: &ActionDiscoveryPanelSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
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

    let gap = resolve_px(theme, spec.gap_token());
    let radius_control = resolve_radius(theme, "radius.control");

    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let elevated = resolve_color(theme, "color.background.elevated");
    let surface = resolve_color(theme, "color.background.surface");
    let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
    let label_size = resolve_px(theme, "typography.label.size");
    let caption_size = resolve_px(theme, "typography.caption.size");

    // Active-item accent treatment (contract §9):
    //   bg   = accent 18% over background-elevated
    //   ring = inset accent 22% (approximated as a 1px border; JsEl has no
    //          inset box-shadow primitive — noted approximation)
    let active_bg = color_mix(accent, elevated, 0.18);
    let active_ring = tint(accent, 0.22);

    let mut panel = ui_element::div().flex_col().gap(gap).w_full();

    match spec.state {
        DiscoveryState::Loading => {
            // 5 skeleton rows; two skeletons per row (48% / 20% width).
            let mut skeletons = ui_element::div()
                .flex_col()
                .gap(list_gap)
                .w_full()
                .p(skeleton_pad);
            for _ in 0..5 {
                let wide = js_skeleton(
                    &SkeletonSpec::new().with_width("48%").with_animated(true),
                    theme,
                );
                let narrow = js_skeleton(
                    &SkeletonSpec::new().with_width("20%").with_animated(true),
                    theme,
                );
                // Skeleton width strings like "48%" aren't parsed by js_skeleton
                // (rem/px only); express the proportions via flex sizing instead.
                let row = ui_element::div()
                    .flex_row()
                    .justify_between()
                    .gap(chip_gap)
                    .child(ui_element::div().flex_basis(rem_to_px(8.0)).child(wide))
                    .child(ui_element::div().flex_basis(rem_to_px(3.5)).child(narrow));
                skeletons = skeletons.child(row);
            }
            // __state wrapper: min-height 10rem, centered.
            panel = panel.child(
                ui_element::div()
                    .items_center()
                    .justify_center()
                    .min_h(rem_to_px(10.0))
                    .w_full()
                    .child(skeletons),
            );
            return panel;
        }
        DiscoveryState::Error => {
            return panel.child(js_empty_state(
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
            return panel.child(js_empty_state(
                &EmptyStateSpec::new(title)
                    .with_message("No actions are available in this context.")
                    .with_compact(true),
                theme,
            ));
        }
        DiscoveryState::NoResults => {
            return panel.child(js_empty_state(
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
        let mut section_el = ui_element::div().flex_col().gap(group_gap);

        // Section heading via the Eyebrow primitive.
        section_el = section_el.child(js_eyebrow(
            &EyebrowSpec::new().with_content(&section.title),
            theme,
        ));

        // Optional section description.
        if let Some(ref desc) = section.description {
            section_el = section_el.child(
                ui_element::label(desc)
                    .text_color(text_secondary)
                    .text_size(label_size)
                    .pl(row_x)
                    .pr(row_x),
            );
        }

        // List of action items.
        let mut list = ui_element::div().flex_col().gap(list_gap);

        for action in &section.actions {
            let is_active = spec.active_id.as_deref() == Some(action.id.as_str());

            // Left: title + optional subtitle (description).
            let mut text_block = ui_element::div().flex_col().gap(rem_to_px(0.125)).min_w_0();
            text_block = text_block.child(
                ui_element::label(&action.title)
                    .text_color(text_primary)
                    .text_size(font_size)
                    .text_ellipsis(),
            );
            if let Some(ref desc) = action.description {
                text_block = text_block.child(
                    ui_element::label(desc)
                        .text_color(text_secondary)
                        .text_size(caption_size)
                        .text_ellipsis(),
                );
            }

            let mut row = ui_element::div()
                .id(action.id.clone())
                .flex_row()
                .items_center()
                .justify_between()
                .gap(item_gap)
                .pl(row_x)
                .pr(row_x)
                .pt(row_y)
                .pb(row_y)
                .rounded(radius_control)
                .child(text_block.flex_grow());

            // Active item (contract §9): accent-tinted bg + an inset accent ring (no
            // longer a border approximation — doesn't shift layout vs other rows).
            if is_active {
                row = row.bg(active_bg).shadow_layers(vec![jetstream_runtime::ui_element::BoxShadow {
                    offset_x: 0.0,
                    offset_y: 0.0,
                    blur: 0.0,
                    spread: rem_to_px(0.0625),
                    color: active_ring,
                    inset: true,
                }]);
            }

            // Trailing snippet — badge chip + kbd chip.
            let has_badge = action.badge.is_some();
            let has_shortcut = action.shortcut.is_some();
            if has_badge || has_shortcut {
                let mut trailing = ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(chip_gap)
                    .flex_none();

                if let Some(ref badge) = action.badge {
                    // Accent override: accent 16% bg, accent text, uppercase label.
                    trailing = trailing.child(
                        ui_element::div()
                            .items_center()
                            .justify_center()
                            .min_h(rem_to_px(chip_h))
                            .pl(rem_to_px(chip_x))
                            .pr(rem_to_px(chip_x))
                            .rounded(radius_control)
                            .bg(tint(accent, 0.16))
                            .whitespace_nowrap()
                            .child(
                                ui_element::label(badge.to_uppercase())
                                    .text_color(accent)
                                    .text_size(rem_to_px(chip_font))
                                    .text_weight(600)
                                    .letter_spacing_em(0.03), // contract §9 chip: 0.03em
                            ),
                    );
                }

                if let Some(ref shortcut) = action.shortcut {
                    // Kbd: surface 76% bg, secondary text, monospace (no uppercase).
                    trailing = trailing.child(
                        ui_element::div()
                            .items_center()
                            .justify_center()
                            .min_h(rem_to_px(chip_h))
                            .pl(rem_to_px(chip_x))
                            .pr(rem_to_px(chip_x))
                            .rounded(radius_control)
                            .bg(tint(surface, 0.76))
                            .whitespace_nowrap()
                            .child(
                                // Contract §9 kbd override: code-family (monospace).
                                ui_element::label(shortcut)
                                    .text_color(text_secondary)
                                    .text_size(rem_to_px(chip_font))
                                    .text_weight(600)
                                    .font_family(FontFamily::Mono),
                            ),
                    );
                }

                row = row.child(trailing);
            }

            // Disabled: reduce opacity via token, not a hardcoded value.
            if action.is_disabled {
                row = row.opacity(disabled_opacity);
            } else {
                row = row.cursor_pointer().focusable();
            }

            list = list.child(row);
        }

        section_el = section_el.child(list);
        panel = panel.child(section_el);
    }

    panel
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ActionDiscoverySection, CommandActionItem};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn sample_sections() -> Vec<ActionDiscoverySection> {
        vec![
            ActionDiscoverySection::new(
                "file",
                "File",
                vec![
                    CommandActionItem::new("save", "Save").with_shortcut("Cmd+S"),
                    CommandActionItem::new("open", "Open File").with_shortcut("Cmd+O"),
                ],
            ),
            ActionDiscoverySection::new(
                "ci",
                "CI/CD",
                vec![CommandActionItem::new("deploy", "Deploy to Production")
                    .with_description("Push the current build live")
                    .with_badge("Dangerous")],
            ),
        ]
    }

    #[test]
    fn renders_action_item_labels_not_just_section_titles() {
        let th = theme();
        let spec = ActionDiscoveryPanelSpec::new(sample_sections());
        let tree = probe(&js_action_discovery_panel(&spec, &th), 480.0, 320.0);

        // Section titles present (js_eyebrow uppercases — `File` → `FILE`).
        assert!(
            tree.has_text("FILE"),
            "section title missing: {:?}",
            tree.texts()
        );
        // Action item labels present — the stub only rendered section titles.
        assert!(
            tree.has_text("Save"),
            "action item label missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Open File"),
            "action item label missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Deploy to Production"),
            "action item label missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn active_item_gets_accent_background() {
        let th = theme();
        let accent = resolve_color(&th, "color.accent.base");
        let elevated = resolve_color(&th, "color.background.elevated");
        let active_bg = color_mix(accent, elevated, 0.18);

        let spec = ActionDiscoveryPanelSpec::new(sample_sections()).with_active_id("save");
        let tree = probe(&js_action_discovery_panel(&spec, &th), 480.0, 320.0);

        let target = crate::render_probe::ProbeColor {
            r: active_bg.x,
            g: active_bg.y,
            b: active_bg.z,
            a: active_bg.w,
        };
        assert!(
            tree.has_background(target, 0.02),
            "active item accent background not found in tree"
        );

        // And the non-active variant does NOT carry that accent fill.
        let inactive = ActionDiscoveryPanelSpec::new(sample_sections());
        let inactive_tree = probe(&js_action_discovery_panel(&inactive, &th), 480.0, 320.0);
        assert!(
            !inactive_tree.has_background(target, 0.001),
            "accent background leaked onto non-active item"
        );
    }

    #[test]
    fn loading_state_renders_five_skeleton_rows() {
        let th = theme();
        let spec =
            ActionDiscoveryPanelSpec::new(sample_sections()).with_state(DiscoveryState::Loading);
        let tree = probe(&js_action_discovery_panel(&spec, &th), 480.0, 320.0);

        // No action/section text in loading posture.
        assert!(
            !tree.has_text("Save"),
            "loading state leaked action items: {:?}",
            tree.texts()
        );
        // Skeleton fill panels present — two per row, five rows = 10 fills.
        // Match the skeleton's actual resolved shimmer fill (contract mid-tone),
        // not the legacy flat `fill_token()`.
        let skeleton_fill = crate::skeleton::shimmer_fill(&SkeletonSpec::new(), &th);
        let target = crate::render_probe::ProbeColor {
            r: skeleton_fill.x,
            g: skeleton_fill.y,
            b: skeleton_fill.z,
            a: skeleton_fill.w,
        };
        let fills = tree
            .nodes
            .iter()
            .filter(|n| n.bg.is_some_and(|b| b.approx(target, 0.02)))
            .count();
        assert!(
            fills >= 10,
            "expected >=10 skeleton fills (5 rows x 2), found {fills}"
        );
    }

    #[test]
    fn keyboard_shortcut_badge_renders() {
        let th = theme();
        let spec = ActionDiscoveryPanelSpec::new(sample_sections());
        let tree = probe(&js_action_discovery_panel(&spec, &th), 480.0, 320.0);

        // Kbd chip text (shortcut) present.
        assert!(
            tree.has_text("Cmd+S"),
            "keyboard shortcut kbd chip missing: {:?}",
            tree.texts()
        );
        // Accent badge text present (uppercased).
        assert!(
            tree.has_text("DANGEROUS"),
            "accent badge chip missing: {:?}",
            tree.texts()
        );
    }
}
