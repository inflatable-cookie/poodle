//! RefSelect — Jetstream version-control ref chooser backed by RefSelectSpec.
//!
//! Contract: `docs/contracts/components/ref-select.md`
//! Reference: `packages/svelte/components/src/RefSelect.svelte`
//!
//! Anatomy: root → trigger (kind glyph + label + chevron) → dialog surface
//! (search field, scrolling ref list with the current marker, empty/loading
//! footers).
//!
//! Typing and clicking live in the preview event loop, not the component: the
//! render is a faithful function of the spec, including its query.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, RefSelectSpec, RefSelectVariant, TextInputSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::text_input::js_text_input;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

pub fn js_ref_select(spec: &RefSelectSpec, theme: &JetstreamThemeProvider) -> JsEl {
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
        poodle_specs::ControlDensity::Compact => 0.25,
        poodle_specs::ControlDensity::Default => 0.375,
        poodle_specs::ControlDensity::Comfortable => 0.5,
    });

    // ── Colors ────────────────────────────────────────────────────────────────
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, spec.secondary_color_token());
    let muted = resolve_color(theme, spec.muted_color_token());
    let label_color = if spec.has_selection() {
        resolve_color(theme, spec.label_color_token())
    } else {
        muted
    };
    // Subdued dims the resting trigger; hover/focus restoration is web-only
    // (contract §12).
    let subdued_opacity = if spec.emphasis.is_subdued() {
        resolve_opacity(theme, spec.subdued_opacity_token())
    } else {
        1.0
    };
    let border = resolve_color(theme, spec.trigger_border_token());
    let item_border = resolve_color(theme, spec.item_border_token());
    let surface = resolve_color(theme, spec.trigger_fill_token());
    let elevated = resolve_color(theme, spec.surface_fill_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let surface_radius = resolve_radius(theme, spec.surface_radius_token());

    // ── Trigger ───────────────────────────────────────────────────────────────
    let mut trigger = ui_element::div()
        .flex_row()
        .items_center()
        .gap(trigger_gap)
        .min_w(0.0)
        .min_h(trigger_h)
        .pl(rem_to_px(0.375))
        .pr(rem_to_px(0.375))
        .rounded(radius);

    if spec.variant == RefSelectVariant::Outlined {
        trigger = trigger.border_1().border_color(border).bg(surface);
    }

    trigger = trigger
        .child(
            ui_element::icon(&spec.trigger_icon())
                .w(trigger_font)
                .h(trigger_font)
                .text_color(tint(text_secondary, subdued_opacity)),
        )
        .child(
            ui_element::label(&spec.trigger_label())
                .text_color(label_color)
                .text_size(trigger_font)
                .text_weight(if spec.has_selection() { 500 } else { 400 })
                .text_ellipsis()
                .whitespace_nowrap(),
        )
        .child(
            ui_element::icon("chevron-down")
                .w(trigger_font)
                .h(trigger_font)
                .text_color(tint(text_secondary, subdued_opacity)),
        );

    let mut root = ui_element::div()
        .flex_col()
        .gap(rem_to_px(0.5))
        .min_w(0.0)
        .child(trigger);

    // ── Dialog surface (rendered inline when open) ────────────────────────────
    if spec.is_open {
        // Contract: the open overlay panel is a `dialog`.
        let mut panel = ui_element::div().flex_col().gap(rem_to_px(0.5))
            .aria_role(jetstream_ui::accesskit::Role::Dialog);

        if spec.is_searchable {
            let mut search = TextInputSpec::new()
                .with_size(effective_size)
                .with_density(spec.density)
                .with_disabled(spec.is_disabled);
            if let Some(query) = &spec.search_value {
                search = search.with_value(query.clone());
            }
            search.placeholder = Some(spec.search_placeholder.clone());
            panel = panel.child(js_text_input(&search, theme));
        }

        let rows = spec.rows();
        // Contract: the results are a `listbox` of `option`s.
        let mut list = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.125))
            .aria_role(jetstream_ui::accesskit::Role::ListBox);
        for (index, option) in rows.iter().enumerate() {
            if let Some(heading) = spec.group_heading_for(&rows, index) {
                list = list.child(
                    ui_element::label(heading)
                        .flex_none()
                        .text_color(text_secondary)
                        .text_size(rem_to_px(0.6875))
                        .text_weight(500)
                        .letter_spacing_em(0.05)
                        .pl(rem_to_px(0.5))
                        .pt(rem_to_px(if index == 0 { 0.5 } else { 0.875 }))
                        .pb(rem_to_px(0.25)),
                );
            }

            let is_selected = option.value == spec.value;
            let mut row = ui_element::div()
                // Each result row is an `option` of the listbox above it.
                .aria_role(jetstream_ui::accesskit::Role::ListBoxOption)
                .flex_none()
                .flex_row()
                .items_start()
                .gap(rem_to_px(0.5))
                .pl(rem_to_px(0.5))
                .pr(rem_to_px(0.5))
                .pt(rem_to_px(0.375))
                .pb(rem_to_px(0.375))
                .rounded(radius)
                .child(
                    ui_element::icon(option.resolved_icon())
                        .w(rem_to_px(0.75))
                        .h(rem_to_px(0.75))
                        .text_color(text_secondary),
                );

            let mut text = ui_element::div().flex_col().grow().min_w(0.0).child(
                ui_element::label(&option.label)
                    .text_color(text_primary)
                    .text_size(rem_to_px(0.875))
                    .text_weight(if is_selected { 600 } else { 400 })
                    .text_ellipsis()
                    .whitespace_nowrap(),
            );
            if let Some(description) = &option.description {
                text = text.child(
                    ui_element::label(description)
                        .text_color(text_secondary)
                        .text_size(rem_to_px(0.75))
                        .text_ellipsis()
                        .whitespace_nowrap(),
                );
            }
            row = row.child(text);

            if spec.is_current(option) {
                row = row.child(
                    ui_element::label(&spec.current_label)
                        .flex_none()
                        .text_color(text_secondary)
                        .text_size(rem_to_px(0.75)),
                );
            }

            if option.is_disabled {
                row = row.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
            }

            list = list.child(row);
        }
        panel = panel.child(list);

        if spec.show_empty() {
            panel = panel.child(
                ui_element::label(&spec.empty_label)
                    // Contract: the empty and loading lines are `status`, so a
                    // screen reader is told the list is empty rather than
                    // finding nothing and being left to infer why.
                    .aria_role(jetstream_ui::accesskit::Role::Status)
                    .text_color(text_secondary)
                    .text_size(rem_to_px(0.75)),
            );
        }

        if spec.is_loading {
            panel = panel.child(
                ui_element::label(&spec.loading_label)
                    .aria_role(jetstream_ui::accesskit::Role::Status)
                    .text_color(text_secondary)
                    .text_size(rem_to_px(0.75)),
            );
        }

        root = root.child(
            ui_element::div()
                .min_w(rem_to_px(16.0))
                .max_w(rem_to_px(24.0))
                .rounded(surface_radius)
                .border_1()
                .border_color(item_border)
                .bg(elevated)
                .pl(rem_to_px(0.5))
                .pr(rem_to_px(0.5))
                .pt(rem_to_px(0.5))
                .pb(rem_to_px(0.5))
                .child(panel),
        );
    }

    if spec.is_disabled {
        root = root.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    }

    crate::aria::with_aria_label(root, Some(spec.aria_label.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{RefKind, RefOption};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn sample() -> RefSelectSpec {
        RefSelectSpec::new()
            .with_refs(vec![
                RefOption::new("main", "main")
                    .with_group("Branches")
                    .with_description("a1b2c3d"),
                RefOption::new("tree-component", "tree-component").with_group("Branches"),
                RefOption::new("v1.4.0", "v1.4.0")
                    .with_kind(RefKind::Tag)
                    .with_group("Tags"),
            ])
            .with_value("main")
            .with_current_ref("main")
    }

    #[test]
    fn trigger_shows_the_selected_ref() {
        let tree = crate::render_probe::probe(&js_ref_select(&sample(), &theme()), 320.0, 80.0);
        assert!(tree.has_text("main"), "trigger label missing: {:?}", tree.texts());
    }

    #[test]
    fn open_panel_lists_refs_groups_and_the_current_marker() {
        let tree = crate::render_probe::probe(
            &js_ref_select(&sample().with_open(true), &theme()),
            360.0,
            420.0,
        );
        assert!(tree.has_text("Branches"), "group heading missing: {:?}", tree.texts());
        assert!(tree.has_text("Tags"), "second group missing: {:?}", tree.texts());
        assert!(tree.has_text("tree-component"), "row missing: {:?}", tree.texts());
        assert!(tree.has_text("a1b2c3d"), "description missing: {:?}", tree.texts());
        assert!(tree.has_text("current"), "current marker missing: {:?}", tree.texts());
    }

    #[test]
    fn loading_replaces_the_empty_message() {
        let empty = RefSelectSpec::new().with_open(true);
        let tree = crate::render_probe::probe(&js_ref_select(&empty, &theme()), 320.0, 200.0);
        assert!(tree.has_text("No refs found"), "empty text missing: {:?}", tree.texts());

        let loading = empty.with_loading(true);
        let tree = crate::render_probe::probe(&js_ref_select(&loading, &theme()), 320.0, 200.0);
        assert!(
            !tree.has_text("No refs found"),
            "must not claim empty while loading: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("Loading more refs…"), "loading text missing: {:?}", tree.texts());
    }


}
