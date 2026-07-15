//! FilterBuilder — Jetstream filter-clause builder backed by FilterBuilderSpec.
//!
//! Contract: `docs/contracts/components/filter-builder.md`
//! Reference: `packages/svelte/components/src/FilterBuilder.svelte`
//!
//! Anatomy: popover wrapper → root (trigger-wrap with FILTER label + summary +
//! count badge + chevron, and an optional ghost reset IconButton) → clause pills
//! → dialog surface (Match all / Match any combinator when 2+ clauses, an
//! add-field Select, or "No filters" empty text).
//!
//! Interaction (menu open/close, draft field → operator → operand → Add, pill
//! edit/remove) lives in the preview event loop, not the component — render-only,
//! build/probe-verified.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ButtonVariant, ChoiceOption, ControlSize, FilterBuilderSpec, FilterCombinator, IconButtonSpec,
    SelectSpec,
};

use crate::icon_button::js_icon_button;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_padding_x_offset_rem};
use crate::select::js_select;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

pub fn js_filter_builder(spec: &FilterBuilderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Size table (contract §8) ──────────────────────────────────────────────
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
        poodle_specs::ControlDensity::Compact => 0.375,
        poodle_specs::ControlDensity::Default => 0.5,
        poodle_specs::ControlDensity::Comfortable => 0.625,
    });

    let root_gap = rem_to_px(0.375);
    let panel_gap = rem_to_px(0.5);

    // ── Colors ────────────────────────────────────────────────────────────────
    let text_primary = resolve_color(theme, spec.field_text_token());
    let text_secondary = resolve_color(theme, spec.label_color_token());
    let muted = resolve_color(theme, spec.muted_color_token());
    let border = resolve_color(theme, spec.field_border_token());
    let item_border = resolve_color(theme, spec.item_border_token());
    let surface = resolve_color(theme, spec.field_fill_token());
    let elevated = resolve_color(theme, spec.field_hover_fill_token());
    let accent = resolve_color(theme, spec.count_fill_token());
    let accent_text = resolve_color(theme, spec.count_text_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let surface_radius = resolve_radius(theme, spec.surface_radius_token());
    let item_bg = color_mix(surface, elevated, 0.90);

    // ── Single bordered field: opener + inline pills + reset ──────────────────
    let mut field = ui_element::div()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(root_gap)
        .min_w(0.0)
        .min_h(trigger_h)
        .pl(trigger_pad_x)
        .pr(trigger_pad_x)
        .pt(rem_to_px(0.25))
        .pb(rem_to_px(0.25))
        .rounded(radius)
        .border_1()
        .border_color(border)
        .bg(surface);

    // Opener (borderless): label + optional summary + chevron. Summary text
    // shows only when pills are not conveying the clauses.
    let mut opener = ui_element::div().flex_row().items_center().gap(trigger_gap);
    if !spec.is_compact {
        opener = opener.child(
            ui_element::label(spec.opener_label())
                .text_color(if spec.combinator_visible() {
                    text_primary
                } else {
                    text_secondary
                })
                .text_size(label_font)
                .text_weight(500)
                .letter_spacing_em(0.05),
        );
    }
    if !(spec.show_pills && spec.has_value()) {
        let is_placeholder = !spec.has_value();
        opener = opener.child(
            ui_element::label(&spec.summary_text())
                .text_color(if is_placeholder { muted } else { text_primary })
                .text_size(summary_font)
                .text_ellipsis()
                .whitespace_nowrap(),
        );
    }
    opener = opener.child(
        ui_element::icon("chevron-down")
            .w(summary_font)
            .h(summary_font)
            .text_color(text_secondary),
    );
    field = field.child(opener);

    // Inline clause pills.
    if spec.show_pills && spec.has_value() {
        for clause in spec.value.clauses.iter() {
            let label = spec.clause_label(clause);
            field = field.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(rem_to_px(0.375))
                    .pl(rem_to_px(0.5))
                    .pr(rem_to_px(0.375))
                    .min_h(rem_to_px(1.5))
                    .rounded(radius)
                    .border_1()
                    .border_color(item_border)
                    .bg(item_bg)
                    .child(
                        ui_element::label(&label)
                            .text_color(text_primary)
                            .text_size(rem_to_px(0.75)),
                    )
                    // Compact remove glyph (not a full 1.5rem IconButton).
                    .child(
                        ui_element::icon("x")
                            .w(rem_to_px(0.75))
                            .h(rem_to_px(0.75))
                            .text_color(muted),
                    ),
            );
        }
    }

    // Trailing controls (count badge + single clear-all).
    if spec.has_value() && (spec.show_pills || spec.show_clear_button) {
        let mut trailing = ui_element::div()
            .grow()
            .flex_row()
            .items_center()
            .justify_end()
            .gap(rem_to_px(0.375));

        if spec.show_pills {
            trailing = trailing.child(
                ui_element::label(&format!("{}", spec.active_count()))
                    .min_w(rem_to_px(1.125))
                    .min_h(rem_to_px(1.125))
                    .pl(rem_to_px(0.3125))
                    .pr(rem_to_px(0.3125))
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .rounded(rem_to_px(0.5625))
                    .bg(accent)
                    .text_color(accent_text)
                    .text_size(rem_to_px(0.6875))
                    .text_weight(600),
            );
        }

        if spec.show_clear_button {
            trailing = trailing.child(js_icon_button(
                &IconButtonSpec::new()
                    .with_icon("x")
                    .with_aria_label("Clear filters")
                    .with_variant(ButtonVariant::Ghost)
                    .with_size(effective_size)
                    .with_disabled(spec.is_disabled),
                theme,
            ));
        }

        field = field.child(trailing);
    }

    let mut root = ui_element::div().flex_col().gap(root_gap).min_w(0.0).child(field);

    // ── Dialog surface (rendered inline when open) ────────────────────────────
    if spec.is_open {
        let mut panel = ui_element::div().flex_col().gap(panel_gap);

        // Combinator (2+ clauses) — static two-option indicator.
        if spec.combinator_visible() {
            let is_and = spec.value.combinator == FilterCombinator::And;
            let mk = |text: &str, selected: bool| {
                ui_element::label(text)
                    .grow()
                    .pl(rem_to_px(0.5))
                    .pr(rem_to_px(0.5))
                    .pt(rem_to_px(0.25))
                    .pb(rem_to_px(0.25))
                    .rounded(radius)
                    .bg(if selected { accent } else { surface })
                    .text_color(if selected { accent_text } else { text_secondary })
                    .text_size(rem_to_px(0.75))
            };
            panel = panel.child(
                ui_element::div()
                    .flex_row()
                    .gap(rem_to_px(0.25))
                    .child(mk("Match all", is_and))
                    .child(mk("Match any", !is_and)),
            );
        }

        // Add-field Select.
        let available = spec.available_fields();
        if spec.can_add_more() && !available.is_empty() {
            let options: Vec<ChoiceOption> = available
                .into_iter()
                .map(|field| ChoiceOption::new(field.key.clone(), field.label.clone()))
                .collect();
            let mut select_spec = SelectSpec::new(options)
                .with_placeholder("+ Add filter")
                .with_size(effective_size)
                .with_density(spec.density);
            select_spec.aria_label = Some("Add filter field".to_string());
            select_spec.is_disabled = spec.is_disabled;
            panel = panel.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .child(js_select(&select_spec, theme)),
            );
        }

        if !spec.has_value() {
            panel = panel.child(
                ui_element::label("No filters")
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
                .pl(rem_to_px(0.375))
                .pr(rem_to_px(0.375))
                .pt(rem_to_px(0.375))
                .pb(rem_to_px(0.375))
                .child(panel),
        );
    }

    if spec.is_disabled {
        root = root.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{
        FilterClause, FilterExpression, FilterFieldDefinition, FilterFieldKind, FilterOperand,
        FilterOption,
    };

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn fields() -> Vec<FilterFieldDefinition> {
        vec![
            FilterFieldDefinition::new("format", "Format", FilterFieldKind::MultiEnum).with_options(
                vec![FilterOption::new("clap", "CLAP"), FilterOption::new("vst3", "VST3")],
            ),
            FilterFieldDefinition::new("hidden", "Hidden", FilterFieldKind::Boolean),
            FilterFieldDefinition::new("tag-count", "Tag count", FilterFieldKind::Number),
        ]
    }

    fn populated() -> FilterBuilderSpec {
        FilterBuilderSpec::new()
            .with_fields(fields())
            .with_open(true)
            .with_show_combinator(true)
            .with_value(FilterExpression {
                combinator: FilterCombinator::And,
                clauses: vec![
                    FilterClause::new(
                        "format-1",
                        "format",
                        "any_of",
                        FilterOperand::Options(vec!["clap".into(), "vst3".into()]),
                    ),
                    FilterClause::new("hidden-1", "hidden", "is", FilterOperand::Boolean(false)),
                ],
            })
    }

    #[test]
    fn empty_open_shows_no_filters_and_hides_reset() {
        let el = js_filter_builder(
            &FilterBuilderSpec::new().with_fields(fields()).with_open(true),
            &theme(),
        );
        let tree = crate::render_probe::probe(&el, 320.0, 240.0);
        assert!(tree.has_text("No filters"), "empty text wrong: {:?}", tree.texts());
        // Empty + combinator off → opener label and placeholder summary both "Filter".
        assert!(tree.has_text("Filter"), "opener/placeholder missing: {:?}", tree.texts());
    }

    #[test]
    fn populated_shows_inline_pill_labels() {
        let tree = crate::render_probe::probe(&js_filter_builder(&populated(), &theme()), 360.0, 320.0);
        assert!(
            tree.has_text("Format is any of CLAP, VST3"),
            "multi-enum pill label wrong: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("Hidden is false"), "boolean pill wrong: {:?}", tree.texts());
        // Summary count text is suppressed while pills convey the clauses (no
        // duplicate count). `populated()` opts into the combinator with 2 clauses,
        // so the opener label reflects the match mode ("All").
        assert!(tree.has_text("All"), "opener mode label missing: {:?}", tree.texts());
        assert!(!tree.has_text("2 filters"), "count must not duplicate the pills: {:?}", tree.texts());
    }

    #[test]
    fn combinator_shows_only_with_two_clauses() {
        // Two clauses → combinator visible.
        let tree = crate::render_probe::probe(&js_filter_builder(&populated(), &theme()), 360.0, 320.0);
        assert!(tree.has_text("Match all") && tree.has_text("Match any"));

        // One clause → combinator hidden (even with the toggle enabled).
        let one = FilterBuilderSpec::new().with_fields(fields()).with_open(true).with_show_combinator(true).with_value(
            FilterExpression {
                combinator: FilterCombinator::And,
                clauses: vec![FilterClause::new(
                    "hidden-1",
                    "hidden",
                    "is",
                    FilterOperand::Boolean(true),
                )],
            },
        );
        let tree = crate::render_probe::probe(&js_filter_builder(&one, &theme()), 360.0, 240.0);
        assert!(!tree.has_text("Match all"), "combinator must hide with <2 clauses: {:?}", tree.texts());
        assert!(tree.has_text("Hidden is true"), "single clause pill missing: {:?}", tree.texts());
    }
}
