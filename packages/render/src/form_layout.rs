//! FormLayout — form body layout with callouts, field-errors summary,
//! field grid and actions.
//!
//! Contract: `docs/contracts/components/form-layout.md`
//! Ported from: `packages/jetstream/components/src/form_layout.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{ColorValue, LayoutDirection, Node};
use poodle_specs::{CallOutSpec, FormActionsSpec, FormLayoutSpec, StatusTone};

use crate::callout::{callout, CalloutHandlers};
use crate::color::mix_srgb;
use crate::form_actions::form_actions;

/// Accessible field-errors summary (contract §2 FieldErrors / §6
/// `role="alert"`). Background `color-mix(status-danger 8%, transparent)`,
/// border `color-mix(status-danger 40%, transparent)`, label-size body text.
fn field_errors_summary(spec: &FormLayoutSpec, theme: &dyn ThemeProvider) -> Node {
    let tone = theme.resolve_color(spec.field_errors_tone_token());
    let radius = theme.resolve_radius(spec.field_errors_radius_token());
    let border_width = theme.resolve_space(spec.field_errors_border_width_token());
    let pad_x = theme.resolve_space(spec.field_errors_padding_x_token());
    let pad_y = theme.resolve_space(spec.field_errors_padding_y_token());
    let font_size = theme.resolve_space(spec.field_errors_font_size_token());
    let text_color = theme.resolve_color(spec.field_errors_text_token());
    let heading_gap = theme.resolve_space(spec.field_errors_stack_gap_token());
    let item_gap = theme.resolve_space(spec.field_errors_stack_gap_token());

    // color-mix toward transparent: 8% fill, 40% border.
    let transparent = ColorValue(tone.0, tone.1, tone.2, 0.0);
    let fill = mix_srgb(tone, transparent, 0.08);
    let border = mix_srgb(tone, transparent, 0.40);

    let mut block = Node::container();
    {
        let s = &mut block.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = heading_gap;
    }
    let mut heading = Node::text(spec.field_errors_heading());
    heading.style.descriptor.text_color = Some(text_color);
    heading.style.text_size = Some(font_size);
    // Heading is semibold (contract §8 field-errors p font-weight: 600).
    heading.style.text_weight = Some(600);
    let block = block.child(heading);

    let mut list = Node::container();
    {
        let s = &mut list.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = item_gap;
    }
    for (field, message) in &spec.field_errors {
        // Each item renders `<field>: <message>` (contract §6).
        let mut item = Node::text(format!("{}: {}", field, message));
        item.style.descriptor.text_color = Some(text_color);
        item.style.text_size = Some(font_size);
        list = list.child(item);
    }
    block.child(list)
}

/// Build a form layout from a FormLayoutSpec with field children and optional
/// actions.
///
/// Contract anatomy: Root (flex-col, gap stack.lg) → Description? →
/// ErrorCallout? → SuccessCallout? → FieldErrors? → Grid → Actions?.
pub fn form_layout(
    spec: &FormLayoutSpec,
    theme: &dyn ThemeProvider,
    children: Vec<Node>,
    actions: Option<Node>,
) -> Node {
    let text_secondary = theme.resolve_color(spec.description_color_token());
    let row_gap = theme.resolve_space("space.stack.md");
    let section_gap = theme.resolve_space(spec.section_gap_token());
    let column_gap = theme.resolve_space(spec.column_gap_token());
    let body_size = theme.resolve_space(spec.body_size_token());

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = section_gap;
        s.fill_width = true;
        s.self_stretch = true;
    }

    if let Some(ref desc) = spec.description {
        let mut d = Node::text(desc);
        d.style.text_size = Some(body_size);
        d.style.descriptor.text_color = Some(text_secondary);
        el = el.child(d);
    }

    // Error/success delegate to the real Callout primitive (contract §8).
    if let Some(ref error) = spec.error {
        el = el.child(callout(
            &CallOutSpec::new()
                .with_tone(StatusTone::Danger)
                .with_content(error),
            theme,
            CalloutHandlers::default(),
        ));
    }

    if let Some(ref success) = spec.success {
        el = el.child(callout(
            &CallOutSpec::new()
                .with_tone(StatusTone::Success)
                .with_content(success),
            theme,
            CalloutHandlers::default(),
        ));
    }

    // Accessible field-errors summary (contract §2 / §6).
    if spec.has_field_errors() {
        el = el.child(field_errors_summary(spec, theme));
    }

    // Field grid — single-column flex-col, or wrapping flex-row for
    // multi-col. Container-query responsive collapse (contract §7) is
    // Svelte-only; the wrapping flex row approximates columns.
    if spec.columns <= 1 {
        let mut fields = Node::container();
        {
            let s = &mut fields.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = row_gap;
        }
        for child in children {
            fields = fields.child(child);
        }
        el = el.child(fields);
    } else {
        let min_w = theme.resolve_space(spec.column_min_width_token());
        let mut fields = Node::container();
        {
            let s = &mut fields.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_wrap = true;
            s.descriptor.layout.spacing.gap = column_gap;
            s.fill_width = true;
            s.self_stretch = true;
        }
        for child in children {
            let mut cell = Node::container();
            {
                let s = &mut cell.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                // Match the GPUI tier's percentage flex-basis + grow: the
                // basis admits the configured number of columns before the
                // intrinsic min-width decides whether a row wraps.
                s.flex_grow = Some(1.0);
                s.flex_shrink_zero = true;
                s.width_pct = Some(1.0 / spec.columns as f32 - 0.01);
                s.min_width = Some(min_w);
            }
            fields = fields.child(cell.child(child));
        }
        el = el.child(fields);
    }

    // Actions delegate to the FormActions primitive (contract §2 / §8).
    if let Some(actions_el) = actions {
        el = el.child(form_actions(
            &FormActionsSpec::new(),
            theme,
            vec![actions_el],
        ));
    }

    el
}
