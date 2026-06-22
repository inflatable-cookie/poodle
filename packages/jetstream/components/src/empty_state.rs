//! EmptyState — Jetstream empty state backed by EmptyStateSpec.
//!
//! Contract: `docs/contracts/components/empty-state.md`
//! Reference: `packages/svelte/components/src/EmptyState.svelte`
use jetstream_runtime::ui_element::{self, BorderStyle, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ButtonSpec, ControlDensity, ControlSize, EmptyStateSpec, EmptyStateVariant,
    SemanticControlSizeRole,
};

use crate::button::js_button;
use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius, tint};

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

    // Visual circle bg: background.panel @ 90% alpha (contract §8).
    let icon_bg = tint(resolve_color(theme, "color.background.panel"), 0.90);

    // ── Root: dashed border, variant background tint, radius (contract §8) ──
    let border_default = resolve_color(theme, "color.border.default");
    // radius.surface - 0.125rem.
    let root_radius = (resolve_radius(theme, "radius.surface") - rem_to_px(0.125)).max(0.0);

    // Variant background tint — color-mix(in srgb, BASE X%, transparent) ≈ alpha scale.
    let root_bg = match spec.variant {
        EmptyStateVariant::Neutral => tint(resolve_color(theme, "color.background.surface"), 0.76),
        EmptyStateVariant::Search => tint(resolve_color(theme, "color.accent.base"), 0.07),
        EmptyStateVariant::FirstRun => tint(resolve_color(theme, "color.status.success"), 0.07),
    };

    // Vertical padding: density-aware (contract §8). compact → stack.lg,
    // default → panel.y * 1.5, comfortable → panel.y * 2. Horizontal → panel.x.
    let vertical_padding = match spec.density {
        ControlDensity::Compact => resolve_px(theme, "space.stack.lg"),
        ControlDensity::Default => resolve_px(theme, "space.panel.y") * 1.5,
        ControlDensity::Comfortable => resolve_px(theme, "space.panel.y") * 2.0,
    };
    let horiz_padding = resolve_px(theme, "space.panel.x");

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
        .pt(vertical_padding).pb(vertical_padding)
        .pl(horiz_padding).pr(horiz_padding)
        .border(1.0).border_style(BorderStyle::Dashed).border_color(border_default)
        .rounded(root_radius)
        .bg(root_bg);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ButtonVariant, EmptyStateVariant, RemediationAction};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_icon_title_and_message() {
        let th = theme();
        let spec = EmptyStateSpec::new("No projects yet")
            .with_message("Create your first project to get started.");
        let tree = probe(&js_empty_state(&spec, &th), 480.0, 320.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(tree.has_text("No projects yet"), "title missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Create your first project to get started."),
            "message missing: {:?}",
            tree.texts()
        );
        // Default variant → inbox icon.
        assert!(tree.has_text("inbox"), "default variant icon missing: {:?}", tree.texts());
    }

    #[test]
    fn variant_selects_icon_and_tints_root() {
        let th = theme();
        // search → search icon + accent-base @ 7% root tint.
        let search = EmptyStateSpec::new("No results").with_variant(EmptyStateVariant::Search);
        let tree = probe(&js_empty_state(&search, &th), 480.0, 320.0);
        assert!(tree.has_text("search"), "search icon missing: {:?}", tree.texts());

        let accent = resolve_color(&th, "color.accent.base");
        let expected = crate::theme_ext::tint(accent, 0.07);
        let want = crate::render_probe::ProbeColor {
            r: expected.x,
            g: expected.y,
            b: expected.z,
            a: expected.w,
        };
        assert!(
            tree.has_background(want, 0.02),
            "search root tint (accent @ 7%) not found in render"
        );

        // firstRun → plus icon.
        let first = EmptyStateSpec::new("Welcome").with_variant(EmptyStateVariant::FirstRun);
        let first_tree = probe(&js_empty_state(&first, &th), 480.0, 320.0);
        assert!(first_tree.has_text("plus"), "firstRun icon missing: {:?}", first_tree.texts());
    }

    #[test]
    fn renders_action_buttons() {
        let th = theme();
        let spec = EmptyStateSpec::new("No projects yet").with_actions(vec![RemediationAction {
            id: "create".into(),
            label: "Create project".into(),
            variant: ButtonVariant::Primary,
            is_disabled: false,
        }]);
        let tree = probe(&js_empty_state(&spec, &th), 480.0, 320.0);

        assert!(
            tree.has_text("Create project"),
            "action button label missing: {:?}",
            tree.texts()
        );
        assert!(tree.count_kind("Button") >= 1, "expected a composed Button node");
    }

    #[test]
    fn compact_shrinks_title_font() {
        let th = theme();
        let default = EmptyStateSpec::new("Title");
        let compact = EmptyStateSpec::new("Title").with_compact(true);

        let d = probe(&js_empty_state(&default, &th), 480.0, 320.0);
        let c = probe(&js_empty_state(&compact, &th), 480.0, 320.0);

        let d_font = d
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Title"))
            .and_then(|n| n.text_size)
            .expect("default title font");
        let c_font = c
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Title"))
            .and_then(|n| n.text_size)
            .expect("compact title font");

        assert!(c_font < d_font, "compact title font {c_font} should be < default {d_font}");
    }
}
