//! FormActions — Jetstream form action bar backed by FormActionsSpec.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ButtonVariant, FormActionAlign, FormActionsSpec, IconButtonSpec, SemanticControlSizeRole,
};

use crate::icon_button::js_icon_button;
use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_px;

/// Render a form-actions row with no inline danger content (the common case).
/// Delegates to [`js_form_actions_full`] with an empty danger group.
pub fn js_form_actions(spec: &FormActionsSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    js_form_actions_full(spec, theme, Vec::new(), children)
}

/// Render a form-actions row with optional inline danger content (contract
/// `danger` snippet, §2/§8 Danger Inline) plus the collapsed overflow danger
/// menu trigger when `spec.danger_items` is non-empty (§8 Responsive Swap).
pub fn js_form_actions_full(
    spec: &FormActionsSpec,
    theme: &JetstreamThemeProvider,
    danger: Vec<JsEl>,
    children: Vec<JsEl>,
) -> JsEl {
    // Density-keyed gap: compact/comfortable use contract-exact rems,
    // default inherits the inline-md token (contract §8).
    let gap = match spec.gap_rem() {
        Some(rem) => rem_to_px(rem),
        None => resolve_px(theme, spec.action_gap_token()),
    };
    // Density-keyed top separation; default inherits the stack-sm token.
    let separation = if spec.shows_top_separation() {
        match spec.top_separation_rem() {
            Some(rem) => rem_to_px(rem),
            None => resolve_px(theme, spec.stack_separation_token()),
        }
    } else {
        0.0
    };
    // Density-keyed divider offset (contract §8 Divider Offset Variants).
    let border_gap = if spec.shows_top_border() {
        rem_to_px(spec.border_gap_rem())
    } else {
        0.0
    };

    let mut el = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap)
        .flex_wrap()
        .pt(separation)
        .mt(border_gap);

    if spec.shows_top_border() {
        let border_color = crate::theme_ext::resolve_color(theme, spec.border_token());
        el = el.border_t_1().border_color(border_color);
    }

    match spec.align {
        FormActionAlign::Start => { el = el.justify_start(); }
        FormActionAlign::End => { el = el.justify_end(); }
        FormActionAlign::Between => { el = el.justify_between(); }
    }

    // Inline danger group (contract §2 danger snippet, §8 Danger Inline:
    // inline-flex, gap == form-actions gap). Rendered before the primary
    // actions so destructive/cancel content stays visually separated.
    if !danger.is_empty() {
        let mut danger_group = ui_element::div().flex_row().items_center().gap(gap);
        for child in danger {
            danger_group = danger_group.child(child);
        }
        el = el.child(danger_group);
    }

    // Overflow danger menu trigger (contract §2 danger menu / §8 Responsive
    // Swap). Shown when `dangerItems` is present. The container-query collapse
    // — hiding the inline group and showing only this trigger below 31.25rem —
    // is a runtime concern the JsEl render-immediate model can't express (no
    // container-query channel), so both render here; the trigger is a real
    // ghost ellipsis IconButton with an aria-label per contract §6.
    if spec.has_danger_menu() {
        let trigger = js_icon_button(
            &IconButtonSpec::new()
                .with_icon("ellipsis")
                .with_variant(ButtonVariant::Ghost)
                .with_aria_label("More actions")
                .with_size_role(SemanticControlSizeRole::Chrome),
            theme,
        );
        el = el.child(ui_element::div().flex_row().items_center().child(trigger));
    }

    for child in children {
        el = el.child(child);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::button::js_button;
    use crate::render_probe::probe;
    use poodle_specs::{ButtonSpec, ButtonVariant, ControlDensity};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn primary_secondary(theme: &JetstreamThemeProvider) -> Vec<JsEl> {
        vec![
            js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Ghost)
                    .with_label("Cancel"),
                theme,
            ),
            js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Save changes"),
                theme,
            ),
        ]
    }

    #[test]
    fn renders_primary_and_secondary_buttons() {
        let th = theme();
        let el = js_form_actions(&FormActionsSpec::new(), &th, primary_secondary(&th));
        let tree = probe(&el, 600.0, 100.0);
        assert!(tree.has_text("Cancel"), "secondary button missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Save changes"),
            "primary button missing: {:?}",
            tree.texts()
        );
        // Two Button widgets present.
        assert_eq!(tree.count_kind("Button"), 2, "expected 2 action buttons");
    }

    #[test]
    fn end_aligned_pushes_actions_to_the_right() {
        let th = theme();
        let spec = FormActionsSpec::new().with_align(FormActionAlign::End);
        let el = js_form_actions(&spec, &th, primary_secondary(&th));
        let tree = probe(&el, 600.0, 100.0);
        // First button's left edge should sit well past the left margin when end-aligned.
        let first_btn = tree.nodes.iter().find(|n| n.kind == "Button").unwrap();
        assert!(
            first_btn.x > 100.0,
            "end-aligned actions should be pushed right, first btn x={}",
            first_btn.x
        );
    }

    #[test]
    fn start_aligned_keeps_actions_left() {
        let th = theme();
        let spec = FormActionsSpec::new().with_align(FormActionAlign::Start);
        let el = js_form_actions(&spec, &th, primary_secondary(&th));
        let tree = probe(&el, 600.0, 100.0);
        let first_btn = tree.nodes.iter().find(|n| n.kind == "Button").unwrap();
        assert!(
            first_btn.x < 50.0,
            "start-aligned first btn should be near left, x={}",
            first_btn.x
        );
    }

    #[test]
    fn comfortable_density_widens_gap_vs_compact() {
        let th = theme();
        let compact = js_form_actions(
            &FormActionsSpec::new().with_density(ControlDensity::Compact),
            &th,
            primary_secondary(&th),
        );
        let comfortable = js_form_actions(
            &FormActionsSpec::new().with_density(ControlDensity::Comfortable),
            &th,
            primary_secondary(&th),
        );
        // Measure horizontal gap between the two buttons (left edge of 2nd minus
        // right edge of 1st). Comfortable density should produce a larger gap.
        fn gap_between_buttons(tree: &crate::render_probe::ProbeTree) -> f32 {
            let btns: Vec<_> = tree.nodes.iter().filter(|n| n.kind == "Button").collect();
            assert_eq!(btns.len(), 2);
            btns[1].x - (btns[0].x + btns[0].w)
        }
        let g_compact = gap_between_buttons(&probe(&compact, 600.0, 100.0));
        let g_comfy = gap_between_buttons(&probe(&comfortable, 600.0, 100.0));
        assert!(
            g_comfy > g_compact,
            "comfortable gap ({g_comfy}) should exceed compact gap ({g_compact})"
        );
    }

    #[test]
    fn danger_inline_content_and_overflow_trigger_render() {
        let th = theme();
        // Inline danger content = a destructive ghost button.
        let danger = vec![js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("Delete"),
            &th,
        )];
        // dangerItems present -> overflow trigger should render.
        let spec = FormActionsSpec::new()
            .with_danger_item(poodle_specs::FormActionDangerItem::new("Delete account"));
        let el = js_form_actions_full(&spec, &th, danger, primary_secondary(&th));
        let tree = probe(&el, 600.0, 100.0);
        // Inline danger button is present alongside the two primary actions.
        assert!(tree.has_text("Delete"), "inline danger missing: {:?}", tree.texts());
        assert!(tree.has_text("Save changes"), "primary missing: {:?}", tree.texts());
        // Delete + overflow-trigger + Cancel + Save = 4 Button widgets (the icon
        // button is itself a Button containing an Icon glyph).
        assert_eq!(tree.count_kind("Button"), 4, "expected danger + trigger + 2 primary");
        // The overflow trigger renders the ellipsis glyph icon.
        assert!(
            tree.nodes.iter().any(|n| n.kind == "Icon" && n.text.as_deref() == Some("ellipsis")),
            "overflow danger trigger (ellipsis Icon) missing: {:?}",
            tree.nodes.iter().filter(|n| n.kind == "Icon").map(|n| n.text.clone()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn no_danger_items_means_no_overflow_trigger() {
        let th = theme();
        // No dangerItems -> no overflow trigger; only the two plain action buttons.
        let el = js_form_actions(&FormActionsSpec::new(), &th, primary_secondary(&th));
        let tree = probe(&el, 600.0, 100.0);
        assert_eq!(tree.count_kind("Button"), 2);
        assert!(
            !tree.nodes.iter().any(|n| n.kind == "Icon" && n.text.as_deref() == Some("ellipsis")),
            "no overflow trigger expected without dangerItems"
        );
    }

    #[test]
    fn submitting_primary_button_dims_and_disables() {
        // The submitting state is owned by the buttons composed into the row;
        // a disabled primary button reports reduced opacity + disabled flag.
        let th = theme();
        let submit = js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_label("Submitting\u{2026}")
                .with_disabled(true),
            &th,
        );
        let el = js_form_actions(&FormActionsSpec::new(), &th, vec![submit]);
        let tree = probe(&el, 600.0, 100.0);
        assert!(
            tree.has_text("Submitting\u{2026}"),
            "submitting label missing: {:?}",
            tree.texts()
        );
    }
}
