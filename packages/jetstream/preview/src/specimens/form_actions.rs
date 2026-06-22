//! FormActions specimen — form action bars.
//!
//! Mirrors the GPUI specimen (`packages/gpui/preview/src/specimens/form_actions.rs`)
//! against the contract (`docs/contracts/components/form-actions.md`). All rows
//! are real `js_form_actions` / `js_form_actions_full` instances composing real
//! `js_button` actions; no hand-rolled boxes. The responsive container-query
//! collapse (§8 Responsive Swap) is a host/preview-loop concern with no JsEl
//! channel, so the inline danger group + overflow trigger both render.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::form_actions::{js_form_actions, js_form_actions_full};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ButtonSpec, ButtonTone, ButtonVariant, ControlDensity, FormActionAlign, FormActionDangerItem,
    FormActionsSpec,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Action button helpers (real js_button instances).
    let secondary_btn =
        |label: &str| js_button(&ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label(label), theme);
    let primary_btn =
        |label: &str| js_button(&ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label(label), theme);
    let ghost_btn =
        |label: &str| js_button(&ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label(label), theme);

    div().flex_col().gap(24.0)
        // ── End-aligned (default) ───────────────────────────────────────
        .child(group("End-aligned (default)", secondary,
            js_form_actions(&FormActionsSpec::new(), theme, vec![
                secondary_btn("Cancel"),
                primary_btn("Save changes"),
            ])
        ))
        // ── Start-aligned ───────────────────────────────────────────────
        .child(group("Start-aligned", secondary,
            js_form_actions(&FormActionsSpec::new().with_align(FormActionAlign::Start), theme, vec![
                secondary_btn("Back"),
                primary_btn("Continue"),
            ])
        ))
        // ── Space between (danger-tone primary + primary) ───────────────
        .child(group("Space between", secondary,
            js_form_actions(&FormActionsSpec::new().with_align(FormActionAlign::Between), theme, vec![
                js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_tone(ButtonTone::Danger)
                        .with_label("Delete"),
                    theme,
                ),
                primary_btn("Save"),
            ])
        ))
        // ── Responsive danger actions (inline danger group + overflow) ──
        // `dangerItems` on the spec turns on the ghost ellipsis overflow
        // trigger (contract §2 danger menu / §8 Responsive Swap); the inline
        // danger ghost button is the `__danger` group content. Both render
        // together — the JsEl render-immediate model has no container-query
        // channel for the width-driven swap.
        .child(group("Responsive danger actions (overflow)", secondary,
            js_form_actions_full(
                &FormActionsSpec::new()
                    .with_danger_item(FormActionDangerItem::new("Discard draft"))
                    .with_danger_item(FormActionDangerItem::new("Reset form")),
                theme,
                vec![js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_tone(ButtonTone::Danger)
                        .with_label("Discard draft"),
                    theme,
                )],
                vec![secondary_btn("Cancel"), primary_btn("Save changes")],
            )
        ))
        // ── Submitting (loading primary, disabled cancel) ───────────────
        .child(group("Submitting", secondary,
            js_form_actions(&FormActionsSpec::new(), theme, vec![
                js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Cancel")
                        .with_disabled(true),
                    theme,
                ),
                js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Primary)
                        .with_label("Saving\u{2026}")
                        .with_loading(true),
                    theme,
                ),
            ])
        ))
        // ── Density ladder (compact / default / comfortable) ────────────
        .child(group("Density ladder", secondary,
            div().flex_col().gap(12.0)
                .child(density_row(theme, secondary, "Compact", ControlDensity::Compact))
                .child(density_row(theme, secondary, "Default", ControlDensity::Default))
                .child(density_row(theme, secondary, "Comfortable", ControlDensity::Comfortable))
        ))
        // ── Footer-embedded (showTopSeparation=false) ───────────────────
        .child(group("Footer-embedded (no top separation)", secondary,
            js_form_actions(&FormActionsSpec::new().with_top_separation(false), theme, vec![
                ghost_btn("Cancel"),
                primary_btn("Save changes"),
            ])
        ))
        // ── Bordered separation (showTopBorder=true) ────────────────────
        .child(group("Bordered separation", secondary,
            js_form_actions(&FormActionsSpec::new().with_top_border(true), theme, vec![
                secondary_btn("Cancel"),
                primary_btn("Save changes"),
            ])
        ))
}

/// One labelled row of the density ladder: a secondary + primary pair laid out
/// by a `FormActions` at the given density. Density changes the inline gap +
/// top separation only — child button size stays constant (contract §8).
fn density_row(
    theme: &JetstreamThemeProvider,
    text_secondary: glam::Vec4,
    label_text: &str,
    density: ControlDensity,
) -> JsEl {
    div().flex_col().gap(4.0)
        .child(label(label_text).text_color(text_secondary).text_size(11.0))
        .child(
            js_form_actions(&FormActionsSpec::new().with_density(density), theme, vec![
                js_button(&ButtonSpec::new().with_variant(ButtonVariant::Secondary).with_label("Cancel"), theme),
                js_button(&ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Save changes"), theme),
            ])
        )
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
