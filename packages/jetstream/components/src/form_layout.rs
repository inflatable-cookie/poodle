//! FormLayout — form grid container with error/success messaging.
//!
//! Contract: `docs/contracts/components/form-layout.md`
//! Reference: `packages/svelte/components/src/FormLayout.svelte`
//!
//! Renders: description → error Callout → success Callout → field-errors summary
//! → field grid → actions (FormActions). Error/success delegate to the real
//! `js_callout` primitive; actions delegate to `js_form_actions` (contract §8
//! Composed Primitives). ALL spacing/colors from tokens.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CallOutSpec, FormActionsSpec, FormLayoutSpec, StatusTone};

use crate::callout::js_callout;
use crate::form_actions::js_form_actions;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Accessible field-errors summary (contract §2 FieldErrors / §6 `role="alert"`).
///
/// Background `color-mix(status-danger 8%, transparent)`, border
/// `color-mix(status-danger 40%, transparent)`, label-size body text.
/// ARIA (`role="alert"`/`aria-live`) is a platform limit on Jetstream (no a11y
/// channel) — the visual summary still renders.
fn field_errors_summary(spec: &FormLayoutSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let tone: Color = resolve_color(theme, spec.field_errors_tone_token()).into();
    let radius = resolve_radius(theme, spec.field_errors_radius_token());
    let border_width = resolve_px(theme, spec.field_errors_border_width_token());
    let pad_x = resolve_px(theme, spec.field_errors_padding_x_token());
    let pad_y = resolve_px(theme, spec.field_errors_padding_y_token());
    let font_size = resolve_px(theme, spec.field_errors_font_size_token());
    let text_color = resolve_color(theme, spec.field_errors_text_token());
    let heading_gap = resolve_px(theme, spec.field_errors_stack_gap_token());
    let item_gap = resolve_px(theme, spec.field_errors_stack_gap_token());

    // color-mix toward transparent: 8% fill, 40% border (Svelte FormLayout:84-85).
    let transparent = Color::new(tone.r, tone.g, tone.b, 0.0);
    let fill = tone.mix_srgb(transparent, 0.08);
    let border = tone.mix_srgb(transparent, 0.40);

    let mut block = ui_element::div()
        .bg(fill)
        .border(border_width)
        .border_color(border)
        .rounded(radius)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .flex_col()
        .gap(heading_gap)
        .child(
            ui_element::label(spec.field_errors_heading())
                .text_color(text_color)
                .text_size(font_size)
                // Heading is semibold (contract §8 field-errors p font-weight: 600).
                .text_weight(600),
        );

    let mut list = ui_element::div().flex_col().gap(item_gap);
    for (field, message) in &spec.field_errors {
        // Each item renders `<field>: <message>` (contract §6).
        list = list.child(
            ui_element::label(format!("{}: {}", field, message))
                .text_color(text_color)
                .text_size(font_size),
        );
    }
    block = block.child(list);
    block
}

/// Build a form layout from a FormLayoutSpec with field children and optional actions.
///
/// Contract anatomy:
/// ```text
/// [Root .poodle-form-layout]  flex-col, gap: space.stack.lg
///   ├── [Description]    optional, text.secondary body size
///   ├── [ErrorCallout]   optional, danger-tone Callout primitive
///   ├── [SuccessCallout] optional, success-tone Callout primitive
///   ├── [FieldErrors]    optional, accessible danger-toned error summary
///   ├── [Grid]           field children (single-col or wrapping multi-col)
///   └── [Actions]        optional, FormActions primitive
/// ```
pub fn js_form_layout(
    spec: &FormLayoutSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
    actions: Option<JsEl>,
) -> JsEl {
    let text_secondary = resolve_color(theme, spec.description_color_token());
    let row_gap = resolve_px(theme, "space.stack.md");
    let section_gap = resolve_px(theme, spec.section_gap_token());
    let column_gap = resolve_px(theme, spec.column_gap_token());
    let body_size = resolve_px(theme, spec.body_size_token());

    let mut el = ui_element::div().flex_col().gap(section_gap);

    if let Some(ref desc) = spec.description {
        el = el.child(
            ui_element::label(desc)
                .text_size(body_size)
                .text_color(text_secondary),
        );
    }

    // Error/success delegate to the real Callout primitive (contract §8).
    if let Some(ref error) = spec.error {
        el = el.child(js_callout(
            &CallOutSpec::new().with_tone(StatusTone::Danger).with_content(error),
            theme,
        ));
    }

    if let Some(ref success) = spec.success {
        el = el.child(js_callout(
            &CallOutSpec::new().with_tone(StatusTone::Success).with_content(success),
            theme,
        ));
    }

    // Accessible field-errors summary (contract §2 / §6).
    if spec.has_field_errors() {
        el = el.child(field_errors_summary(spec, theme));
    }

    // Field grid — single-column flex-col, or wrapping flex-row for multi-col.
    // Container-query responsive collapse (contract §7) is Svelte-only; the
    // wrapping flex row approximates columns at desktop widths.
    if spec.columns <= 1 {
        let mut fields = ui_element::div().flex_col().gap(row_gap);
        for child in children {
            fields = fields.child(child);
        }
        el = el.child(fields);
    } else {
        let min_w = resolve_px(theme, spec.column_min_width_token());
        let mut fields = ui_element::div().flex_row().flex_wrap().gap(column_gap);
        for child in children {
            fields = fields.child(
                ui_element::div().flex_grow().min_w(min_w).child(child),
            );
        }
        el = el.child(fields);
    }

    // Actions delegate to the FormActions primitive (contract §2 Actions / §8).
    if let Some(actions_el) = actions {
        el = el.child(js_form_actions(&FormActionsSpec::new(), theme, vec![actions_el]));
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn field(label: &str) -> JsEl {
        ui_element::label(label)
    }

    #[test]
    fn single_column_stacks_fields() {
        let th = theme();
        let spec = FormLayoutSpec::new().with_columns(1);
        let el = js_form_layout(&spec, &th, vec![field("Name"), field("Email")], None);
        let tree = probe(&el, 600.0, 400.0);
        assert!(tree.has_text("Name") && tree.has_text("Email"), "fields missing: {:?}", tree.texts());
    }

    #[test]
    fn description_renders_above_grid() {
        let th = theme();
        let spec = FormLayoutSpec::new().with_columns(1).with_description("Fill in the details.");
        let el = js_form_layout(&spec, &th, vec![field("Name")], None);
        let tree = probe(&el, 600.0, 400.0);
        assert!(tree.has_text("Fill in the details."), "description missing: {:?}", tree.texts());
    }

    #[test]
    fn error_composes_callout_primitive() {
        let th = theme();
        let spec = FormLayoutSpec::new().with_columns(1).with_error("Unable to save changes.");
        let el = js_form_layout(&spec, &th, vec![field("Name")], None);
        let tree = probe(&el, 600.0, 400.0);
        // Callout renders the message plus an icon badge (its anatomy) — proves
        // composition rather than a bare label.
        assert!(tree.has_text("Unable to save changes."), "error message missing");
        assert!(tree.count_kind("Icon") >= 1, "callout icon badge missing — not composed");
    }

    #[test]
    fn field_errors_summary_renders_heading_and_items() {
        let th = theme();
        let spec = FormLayoutSpec::new()
            .with_columns(1)
            .with_field_error("Email", "Required")
            .with_field_error("Role", "Pick one");
        let el = js_form_layout(&spec, &th, vec![field("Name")], None);
        let tree = probe(&el, 600.0, 400.0);
        assert!(
            tree.has_text("Please fix the following errors:"),
            "field-errors heading missing: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("Email: Required"), "field-error item 1 missing: {:?}", tree.texts());
        assert!(tree.has_text("Role: Pick one"), "field-error item 2 missing: {:?}", tree.texts());
    }

    #[test]
    fn no_field_errors_means_no_summary() {
        let th = theme();
        let spec = FormLayoutSpec::new().with_columns(1);
        let el = js_form_layout(&spec, &th, vec![field("Name")], None);
        let tree = probe(&el, 600.0, 400.0);
        assert!(
            !tree.has_text("Please fix the following errors:"),
            "summary rendered with no field errors"
        );
    }

    #[test]
    fn actions_wrapped_in_form_actions() {
        let th = theme();
        let spec = FormLayoutSpec::new().with_columns(1);
        let actions = ui_element::button("Save");
        let el = js_form_layout(&spec, &th, vec![field("Name")], Some(actions));
        let tree = probe(&el, 600.0, 400.0);
        assert!(tree.has_text("Save"), "action button missing: {:?}", tree.texts());
    }

    #[test]
    fn multi_column_lays_out_fields_in_a_row() {
        let th = theme();
        let spec = FormLayoutSpec::new().with_columns(2);
        let el = js_form_layout(&spec, &th, vec![field("First"), field("Last")], None);
        let tree = probe(&el, 600.0, 400.0);
        // Both fields present; with a wrapping row they share the first row, so
        // the second field's x-offset is greater than the first's.
        let first = tree.nodes.iter().find(|n| n.text.as_deref() == Some("First"));
        let last = tree.nodes.iter().find(|n| n.text.as_deref() == Some("Last"));
        let (first, last) = (first.expect("First missing"), last.expect("Last missing"));
        assert!(last.x > first.x, "two-column fields not laid out side by side");
    }
}
