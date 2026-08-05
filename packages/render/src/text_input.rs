//! TextInput — a text field: affixes, icons, validation, char count.
//!
//! Contract: `docs/contracts/components/text-input.md`
//! Ported from: `packages/jetstream/components/src/text_input.rs`, which is
//! render-only — editing, caret and focus treatment are host concerns on
//! every native target, so no native slot is needed here. The pending
//! validation state composes the ring [`crate::spinner`], the first
//! component-in-component reuse on the node path.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, StylePatch,
};
use poodle_specs::{
    SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant, TextInputSpec, ValidationState,
};

use crate::color::mix_srgb;
use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::spinner::spinner;

pub fn text_input(
    spec: &TextInputSpec,
    theme: &dyn ThemeProvider,
    on_clear: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = theme.resolve_space(spec.vertical_padding_token());
    let icon_sz = rem_to_px(size_font_rem(effective_size));
    let inline_gap = theme.resolve_space(spec.inline_gap_token());
    let border_width = theme.resolve_space(spec.border_width_token());

    let fill = theme.resolve_color(spec.fill_token());
    let border_color = theme.resolve_color(spec.border_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let text_color = theme.resolve_color(spec.text_color_token());
    let placeholder_color = theme.resolve_color(spec.placeholder_color_token());
    let icon_color = theme.resolve_color(spec.icon_color_token());
    let affix_color = theme.resolve_color(spec.affix_color_token());
    let affix_sep_color = theme.resolve_color(spec.affix_separator_solid_token());

    // Hover border: contract color-mix(border 78%, text-primary).
    let text_primary = theme.resolve_color("color.text.primary");
    let hover_border = mix_srgb(border_color, text_primary, 0.78);

    // Slug mode: value + prefix in code family so the slug reads as one unit.
    let is_slug = spec.input_type == "slug";
    let current_value = spec.current_value();
    let is_placeholder = current_value.is_empty() || spec.value.is_none();
    let show_text = if is_placeholder {
        spec.placeholder.as_deref().unwrap_or("")
    } else {
        current_value
    };
    let show_color = if is_placeholder {
        placeholder_color
    } else {
        text_color
    };

    // ── Input row ──
    let mut input_row = Node::container();
    {
        let s = &mut input_row.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border_color;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.spacing.gap = inline_gap;
        s.hover = Some(StylePatch {
            border_color: Some(hover_border),
            background: None,
            text_color: None,
            opacity: None,
        });
        if spec.is_multiline() {
            s.min_height = Some(rem_to_px(
                control_height_rem(effective_size) * spec.rows as f32,
            ));
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.padding.top = pad_y;
            s.descriptor.layout.spacing.padding.bottom = pad_y;
        } else {
            s.descriptor.layout.height = LayoutSizing::Fixed(height);
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        }
    }
    input_row.interaction.focusable = true;

    let divider = |sep: poodle_node::ColorValue| {
        let mut d = Node::container();
        d.style.descriptor.layout.width = LayoutSizing::Fixed(border_width);
        d.style.self_stretch = true;
        d.style.descriptor.background = Some(sep);
        d
    };

    // Prefix affix (left edge, right divider).
    if let Some(prefix) = &spec.prefix {
        let mut prefix_label = Node::text(prefix.as_str());
        prefix_label.style.descriptor.text_color = Some(affix_color);
        prefix_label.style.text_size = Some(font_size);
        if is_slug {
            prefix_label.style.font_family = Some(FontFamily::Mono);
        }
        input_row = input_row.child(prefix_label).child(divider(affix_sep_color));
    }

    // Leading icon.
    if let Some(icon_name) = &spec.leading_icon {
        let mut icon = Node::icon(icon_name.as_str(), icon_sz);
        icon.style.descriptor.text_color = Some(icon_color);
        input_row = input_row.child(icon);
    }

    // Value (grows); slug renders in code family.
    let mut value_label = Node::text(show_text);
    value_label.style.descriptor.text_color = Some(show_color);
    value_label.style.text_size = Some(font_size);
    value_label.style.descriptor.layout.width = LayoutSizing::Grow;
    if is_slug {
        value_label.style.font_family = Some(FontFamily::Mono);
    }
    input_row = input_row.child(value_label);

    // Trailing icon wins over the validation indicator.
    if let Some(icon_name) = &spec.trailing_icon {
        let mut icon = Node::icon(icon_name.as_str(), icon_sz);
        icon.style.descriptor.text_color = Some(icon_color);
        input_row = input_row.child(icon);
    } else {
        match spec.validation_state {
            ValidationState::Valid | ValidationState::Invalid => {
                let color = theme.resolve_color(spec.validation_indicator_color_token());
                let glyph = if spec.validation_state == ValidationState::Valid {
                    "check"
                } else {
                    "x"
                };
                let mut icon = Node::icon(glyph, icon_sz);
                icon.style.descriptor.text_color = Some(color);
                input_row = input_row.child(icon);
            }
            ValidationState::Pending => {
                input_row = input_row.child(spinner(
                    &SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_size(SpinnerSize::Sm)
                        .with_tone(SpinnerTone::Accent),
                    theme,
                ));
            }
            ValidationState::None => {}
        }
    }

    // Clear button (search type, non-empty value).
    if spec.input_type == "search" && spec.show_clear_button && !current_value.is_empty() {
        let mut clear = Node::button("");
        clear.a11y.label = Some("Clear search query".to_string());
        clear.style.descriptor.cursor = CursorHint::Pointer;
        let mut x = Node::icon("x", icon_sz);
        x.style.descriptor.text_color = Some(icon_color);
        clear = clear.child(x);

        if let (false, false, Some(handler)) = (spec.is_disabled, spec.is_read_only, &on_clear) {
            let handler = Arc::clone(handler);
            clear.interaction.on_activate = Some(Arc::new(move || handler()));
        }

        input_row = input_row.child(clear);
    }

    // Suffix affix (right edge, left divider).
    if let Some(suffix) = &spec.suffix {
        let mut suffix_label = Node::text(suffix.as_str());
        suffix_label.style.descriptor.text_color = Some(affix_color);
        suffix_label.style.text_size = Some(font_size);
        input_row = input_row.child(divider(affix_sep_color)).child(suffix_label);
    }

    if spec.is_disabled {
        input_row.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        input_row.interaction.disabled = true;
    }

    // ── Char count (wraps the row in a column) ──
    if spec.show_char_count {
        let current_len = current_value.chars().count();
        let over = spec.max_length.map_or(false, |max| current_len > max);
        let count_color = if over {
            theme.resolve_color(spec.char_count_over_color_token())
        } else {
            theme.resolve_color(spec.char_count_color_token())
        };
        let count_font = theme.resolve_space(spec.char_count_font_size_token());
        let count_text = match spec.max_length {
            Some(max) => format!("{}/{}", current_len, max),
            None => format!("{}", current_len),
        };

        let mut count_label = Node::text(&count_text);
        count_label.style.descriptor.text_color = Some(count_color);
        count_label.style.text_size = Some(count_font);

        let mut char_count_row = Node::container();
        char_count_row.style.descriptor.layout.direction = LayoutDirection::Row;
        char_count_row.style.descriptor.layout.alignment.main = MainAxisAlignment::End;
        let char_count_row = char_count_row.child(count_label);

        let mut column = Node::container();
        column.style.descriptor.layout.direction = LayoutDirection::Column;
        column.style.descriptor.layout.spacing.gap = rem_to_px(0.25);
        // Faithful to the old tier: the aria label does not survive the
        // char-count wrap there either; parity first, contract fix later on
        // both paths at once.
        return column.child(input_row).child(char_count_row);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        input_row.a11y.label = Some(label.to_string());
    }
    input_row
}
