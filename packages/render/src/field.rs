//! Field — form field wrapper: header, control slot, validation messages.
//!
//! Contract: `docs/contracts/components/field.md`
//! Ported from: `packages/jetstream/components/src/field.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
};
use poodle_specs::{FieldSpec, ValidationState};

use crate::color::{mix_srgb, with_alpha};

pub fn field(spec: &FieldSpec, theme: &dyn ThemeProvider, control: Option<Node>) -> Node {
    let label_size = theme.resolve_space(spec.label_typography_token());
    let desc_color = theme.resolve_color(spec.description_color_token());
    let error_color = theme.resolve_color(spec.error_color_token());
    // Contract §8: label = color-mix(text-primary 45%, text-secondary).
    let label_primary = theme.resolve_color(spec.label_color_primary_token());
    let label_secondary = theme.resolve_color(spec.label_color_secondary_token());
    let label_color = mix_srgb(label_primary, label_secondary, FieldSpec::LABEL_COLOR_PRIMARY_RATIO);
    let row_gap = theme.resolve_space(spec.row_gap_token());
    let supporting_size = theme.resolve_space(spec.supporting_text_typography_token());

    let mut el = Node::container();
    el.style.descriptor.layout.direction = LayoutDirection::Column;
    el.style.descriptor.layout.spacing.gap = row_gap;

    // Header — space-between so the optional marker right-aligns.
    let mut header = Node::container();
    {
        let s = &mut header.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = theme.resolve_space(spec.header_gap_token());
    }

    // Label row — label + required marker + info icon.
    let mut label_row = Node::container();
    {
        let s = &mut label_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space(spec.label_row_gap_token());
    }

    let mut label = Node::text(&spec.label);
    label.style.descriptor.text_color = Some(label_color);
    label.style.text_size = Some(label_size);
    label.style.text_weight = Some(500);
    label_row = label_row.child(label);

    if spec.is_required {
        let mut star = Node::text("*");
        star.style.descriptor.text_color = Some(error_color);
        star.style.text_size = Some(label_size);
        label_row = label_row.child(star);
    }

    // Info-icon pill next to the label when a description/hint exists.
    if spec.info_text().is_some() {
        let icon_box = label_size * FieldSpec::INFO_ICON_EM;
        let icon_glyph = label_size * FieldSpec::INFO_ICON_SVG_EM;
        let info_base = theme.resolve_color(spec.info_icon_bg_token());
        let info_bg = with_alpha(info_base, info_base.3 * FieldSpec::INFO_ICON_BG_ALPHA);
        let info_color = theme.resolve_color(spec.info_icon_color_token());
        let info_radius = theme.resolve_radius(spec.info_icon_radius_token());

        let mut pill = Node::container();
        {
            let s = &mut pill.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.flex_shrink_zero = true;
            s.descriptor.layout.width = LayoutSizing::Fixed(icon_box);
            s.descriptor.layout.height = LayoutSizing::Fixed(icon_box);
            s.descriptor.corner_radii.top_left = info_radius;
            s.descriptor.corner_radii.top_right = info_radius;
            s.descriptor.corner_radii.bottom_right = info_radius;
            s.descriptor.corner_radii.bottom_left = info_radius;
            s.descriptor.background = Some(info_bg);
            s.descriptor.cursor = CursorHint::Pointer;
        }
        let mut glyph = Node::icon("info", icon_glyph);
        glyph.style.descriptor.text_color = Some(info_color);
        label_row = label_row.child(pill.child(glyph));
    }

    header = header.child(label_row);

    if spec.shows_optional_label() {
        if let Some(ref opt_label) = spec.optional_label {
            let mut opt = Node::text(opt_label);
            opt.style.descriptor.text_color = Some(desc_color);
            opt.style.text_size = Some(supporting_size);
            opt.style.flex_shrink_zero = true;
            header = header.child(opt);
        }
    }
    el = el.child(header);

    if let Some(control_el) = control {
        el = el.child(control_el);
    }

    if spec.validation_state == ValidationState::Invalid {
        if let Some(ref error) = spec.error {
            let mut e = Node::text(error);
            e.style.descriptor.text_color = Some(error_color);
            e.style.text_size = Some(supporting_size);
            el = el.child(e);
        }
    }
    if spec.validation_state == ValidationState::Pending {
        if let Some(ref pending) = spec.pending_message {
            let mut p = Node::text(pending);
            p.style.descriptor.text_color = Some(desc_color);
            p.style.text_size = Some(supporting_size);
            el = el.child(p);
        }
    }

    el
}
