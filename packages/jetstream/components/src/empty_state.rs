//! EmptyState — Jetstream empty state backed by EmptyStateSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ButtonSpec, ControlSize, EmptyStateSpec, EmptyStateVariant, SemanticControlSizeRole};

use crate::button::js_button;
use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_empty_state(spec: &EmptyStateSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let gap = resolve_px(theme, spec.layout_gap_token());

    // Effective size: compact uses Sm, default uses Md
    let effective_size = if spec.compact { ControlSize::Sm } else { ControlSize::Md };

    // Title font: 1.125rem default, 0.9375rem compact — match Svelte
    let title_font = if spec.compact { rem_to_px(0.9375) } else { rem_to_px(1.125) };
    // Message font: 0.8125rem default, 0.75rem compact — match Svelte
    let message_font = if spec.compact { rem_to_px(0.75) } else { rem_to_px(0.8125) };

    // Icon: name driven by variant (matches Svelte reference)
    let icon_name = match spec.variant {
        EmptyStateVariant::Search => "search",
        EmptyStateVariant::FirstRun => "plus",
        EmptyStateVariant::Neutral => "inbox",
    };
    // Icon container size: 2.25rem default, 1.75rem compact — match Svelte
    let icon_container = if spec.compact { rem_to_px(1.75) } else { rem_to_px(2.25) };
    // Icon font size inside container: 1.125rem default, 0.9375rem compact
    let icon_font = if spec.compact { rem_to_px(0.9375) } else { rem_to_px(1.125) };

    let icon_bg = resolve_color(theme, "color.background.panel");

    // Visual affordance container (circular)
    let visual_el = ui_element::div()
        .w(icon_container).h(icon_container)
        .rounded(999.0)
        .bg(icon_bg)
        .flex_row().items_center().justify_center()
        .child(
            ui_element::icon(icon_name)
                .w(icon_font).h(icon_font)
                .text_color(text_secondary)
        );

    let mut el = ui_element::div()
        .flex_col().items_center().justify_center().gap(gap)
        .pt(rem_to_px(2.0)).pb(rem_to_px(2.0));

    el = el.child(visual_el);

    // Copy block
    let mut copy_el = ui_element::div()
        .flex_col().items_center().gap(resolve_px(theme, "space.inline.sm"));

    copy_el = copy_el.child(
        ui_element::label(&spec.title)
            .text_color(text_primary)
            .text_size(title_font)
            .text_weight(600)
    );

    if let Some(ref desc) = spec.message {
        copy_el = copy_el.child(
            ui_element::label(desc)
                .text_color(text_secondary)
                .text_size(message_font)
        );
    }

    el = el.child(copy_el);

    // Actions
    if spec.action_count() > 0 {
        let mut actions_el = ui_element::div()
            .flex_row().items_center().gap(resolve_px(theme, "space.inline.sm"));

        for action in &spec.actions {
            let btn_spec = ButtonSpec::new()
                .with_label(&action.label)
                .with_variant(action.variant)
                .with_disabled(action.is_disabled)
                .with_size(effective_size)
                .with_size_role(SemanticControlSizeRole::Control);
            actions_el = actions_el.child(js_button(&btn_spec, theme));
        }

        el = el.child(actions_el);
    }

    el
}
