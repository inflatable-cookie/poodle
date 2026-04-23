//! FormLayout — form grid container with error/success messaging.
//!
//! Contract: `docs/contracts/components/form-layout.md`
//! Reference: `packages/gpui/components/src/composites/form_layout.rs`
//!
//! Renders: description → error callout → success callout → field rows → actions.
//! ALL spacing from tokens. ZERO hardcoded values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::FormLayoutSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Build a callout banner for form-level error/success messages.
fn form_callout_banner(theme: &JetstreamThemeProvider, message: &str, tone: Color) -> JsEl {
    let panel_bg: Color = resolve_color(theme, "color.background.panel").into();
    let radius = resolve_radius(theme, "radius.surface");
    let inline_pad = rem_to_px(1.0);
    let stack_pad = resolve_px(theme, "space.stack.md");
    let body_size = rem_to_px(0.8125); // ~13px body text

    // Callout background: ~12% tone over panel
    let callout_bg = tone.mix(panel_bg, 0.12);
    let callout_border = tone.mix(panel_bg, 0.30);

    ui_element::div()
        .pl(inline_pad).pr(inline_pad)
        .pt(stack_pad).pb(stack_pad)
        .rounded(radius)
        .bg(callout_bg)
        .border(1.0).border_color(callout_border)
        .child(
            ui_element::label(message)
                .text_size(body_size)
                .text_color(tone)
        )
}

/// Build a form layout from a FormLayoutSpec with field children and optional actions.
///
/// Contract anatomy:
/// ```text
/// [Root .form-layout]  flex-col, gap: space.stack.lg
///   ├── [Description]   optional, text.secondary body size
///   ├── [ErrorCallout]  optional, danger tone callout
///   ├── [SuccessCallout] optional, success tone callout
///   ├── [Grid]          flex-col, gap: space.stack.md — field children
///   └── [Actions]       optional caller-provided action row
/// ```
pub fn js_form_layout(
    spec: &FormLayoutSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
    actions: Option<JsEl>,
) -> JsEl {
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let error_tone: Color = resolve_color(theme, "color.status.danger").into();
    let success_tone: Color = resolve_color(theme, "color.status.success").into();
    let row_gap = resolve_px(theme, "space.stack.md");
    let section_gap = resolve_px(theme, "space.stack.lg");
    let body_size = rem_to_px(0.8125);

    let mut el = ui_element::div()
        .flex_col()
        .gap(section_gap);

    if let Some(ref desc) = spec.description {
        el = el.child(
            ui_element::label(desc)
                .text_size(body_size)
                .text_color(text_secondary)
        );
    }

    if let Some(ref error) = spec.error {
        el = el.child(form_callout_banner(theme, error, error_tone));
    }

    if let Some(ref success) = spec.success {
        el = el.child(form_callout_banner(theme, success, success_tone));
    }

    // Field grid — simple flex-col for single-column, wrapping row for multi
    if spec.columns <= 1 {
        let mut fields = ui_element::div().flex_col().gap(row_gap);
        for child in children {
            fields = fields.child(child);
        }
        el = el.child(fields);
    } else {
        let col_gap = resolve_px(theme, "space.inline.md");
        let mut fields = ui_element::div()
            .flex_row()
            .flex_wrap()
            .gap(col_gap);
        for child in children {
            fields = fields.child(
                ui_element::div().flex_grow().min_w(rem_to_px(11.25)).child(child)
            );
        }
        el = el.child(fields);
    }

    if let Some(actions_el) = actions {
        el = el.child(actions_el);
    }

    el
}
