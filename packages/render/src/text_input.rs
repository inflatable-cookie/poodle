//! TextInput — a text field: affixes, icons, validation, char count.
//!
//! Contract: `docs/contracts/components/text-input.md`
//! Ported from: `packages/gpui/components/src/primitives/text_input.rs`.
//! The node declares the current value and replacement-text callback; the
//! backend owns key dispatch and the eventual native editor/IME integration.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, StylePatch,
    TextChangeHandler,
};
use poodle_specs::{
    ControlDensity, ControlSize, IconSize, IconSpec, SpinnerSize, SpinnerSpec, SpinnerTone,
    SpinnerVariant, TextInputSpec, ValidationState,
};

use crate::color::with_alpha;
use crate::icon::icon;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_height_offset_rem, size_padding_x_offset_rem,
};
use crate::spinner::spinner;

/// Render a text input without an editing callback.
///
/// `on_clear` stays in the stable call shape while the old GPUI tier remains
/// the native parity reference. That tier has no clear affordance, so this
/// slice deliberately does not invent one on the replacement path.
pub fn text_input(
    spec: &TextInputSpec,
    theme: &dyn ThemeProvider,
    _on_clear: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    text_input_with_change(spec, theme, None)
}

/// Render a text input with host-owned replacement-text updates.
pub fn text_input_with_change(
    spec: &TextInputSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<TextChangeHandler>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let control_height = theme.resolve_space(spec.control_height_token())
        + rem_to_px(size_height_offset_rem(effective_size));
    let density_offset_rem = match spec.density {
        ControlDensity::Compact => -0.125,
        ControlDensity::Default => 0.0,
        ControlDensity::Comfortable => 0.125,
    };
    let inline_padding = theme.resolve_space(spec.horizontal_padding_token())
        + rem_to_px(size_padding_x_offset_rem(effective_size) + density_offset_rem);
    let inline_gap = theme.resolve_space(spec.inline_gap_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let body_size = rem_to_px(match effective_size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.8125,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    });
    let body_line_height = theme.resolve_space(spec.body_line_height_token()) / body_size;

    let border_default = theme.resolve_color(spec.border_token());
    let surface_raw = theme.resolve_color(spec.fill_token());
    let surface_bg = with_alpha(surface_raw, surface_raw.3 * 0.82);
    let border = with_alpha(border_default, border_default.3 * 0.72);
    let hover_border = with_alpha(border_default, border_default.3 * 0.92);
    let effective_border = match spec.validation_state {
        ValidationState::Invalid => theme.resolve_color("color.status.danger"),
        ValidationState::Valid => theme.resolve_color("color.status.success"),
        ValidationState::Pending => theme.resolve_color("color.accent.base"),
        ValidationState::None => border,
    };
    let text_primary = theme.resolve_color(spec.text_color_token());
    let text_secondary = theme.resolve_color(spec.placeholder_color_token());
    let icon_color = theme.resolve_color(spec.icon_color_token());

    let current_value = spec.current_value();
    let display_text = if current_value.is_empty() {
        spec.placeholder.as_deref().unwrap_or("")
    } else {
        current_value
    };
    let display_color = if current_value.is_empty() {
        text_secondary
    } else {
        text_primary
    };

    let mut inner = Node::container();
    {
        let s = &mut inner.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = inline_gap;
        s.fill_width = true;
        s.fill_height = true;
    }

    if let Some(prefix) = &spec.prefix {
        inner = inner.child(affix(prefix, true, inline_gap, spec, theme));
    }

    if let Some(name) = &spec.leading_icon {
        let mut glyph = icon(&IconSpec::new(name).with_size(IconSize::Sm), theme);
        glyph.style.descriptor.text_color = Some(icon_color);
        inner = inner.child(glyph);
    }

    let mut value = Node::text(display_text);
    value.style.descriptor.layout.width = LayoutSizing::Grow;
    value.style.descriptor.layout.overflow_x = poodle_node::LayoutOverflow::Hidden;
    value.style.descriptor.text_color = Some(display_color);
    value.style.text_ellipsis = true;
    value.style.no_wrap = true;
    inner = inner.child(value);

    if spec.show_char_count {
        let len = current_value.len();
        let over = spec.max_length.is_some_and(|max| len > max);
        let mut count = Node::text(match spec.max_length {
            Some(max) => format!("{len}/{max}"),
            None => len.to_string(),
        });
        count.style.descriptor.text_color = Some(theme.resolve_color(if over {
            spec.char_count_over_color_token()
        } else {
            spec.char_count_color_token()
        }));
        count.style.text_size = Some(theme.resolve_space(spec.char_count_font_size_token()));
        count.style.no_wrap = true;
        inner = inner.child(count);
    }

    if spec.shows_validation_status {
        match spec.validation_state {
            ValidationState::Valid | ValidationState::Invalid => {
                let name = if spec.validation_state == ValidationState::Valid {
                    "check"
                } else {
                    "x"
                };
                let mut glyph = icon(&IconSpec::new(name).with_size(IconSize::Sm), theme);
                glyph.style.descriptor.text_color =
                    Some(theme.resolve_color(spec.validation_indicator_color_token()));
                inner = inner.child(glyph);
            }
            ValidationState::Pending => {
                let mut pending = spinner(
                    &SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_size(SpinnerSize::Sm)
                        .with_tone(SpinnerTone::Accent),
                    theme,
                );
                pending.style.descriptor.text_color =
                    Some(theme.resolve_color(spec.validation_indicator_color_token()));
                inner = inner.child(pending);
            }
            ValidationState::None => {}
        }
    }

    if let Some(name) = &spec.trailing_icon {
        let mut glyph = icon(&IconSpec::new(name).with_size(IconSize::Sm), theme);
        glyph.style.descriptor.text_color = Some(icon_color);
        inner = inner.child(glyph);
    }

    if let Some(suffix) = &spec.suffix {
        inner = inner.child(affix(suffix, false, inline_gap, spec, theme));
    }

    let mut root = Node::input(current_value, spec.placeholder.as_deref().unwrap_or(""));
    root.id = Some(match spec.id.as_deref() {
        Some(id) => format!("poodle-input-{id}"),
        None => "poodle-input".to_string(),
    });
    {
        let s = &mut root.style;
        s.descriptor.background = Some(surface_bg);
        // The native reference uses GPUI's `border_1()` directly. The token
        // can resolve to zero on the compact axis, which removes both the
        // stroke and its one-pixel content inset.
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = effective_border;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.height = LayoutSizing::Fixed(control_height);
        s.descriptor.layout.spacing.padding.left = inline_padding;
        s.descriptor.layout.spacing.padding.right = inline_padding;
        s.descriptor.text_color = Some(text_primary);
        s.text_size = Some(body_size);
        s.line_height = Some(body_line_height);
        s.fill_width = true;
        s.hover = Some(StylePatch {
            background: None,
            border_color: Some(hover_border),
            text_color: None,
            opacity: None,
        });
    }
    root.interaction.focusable = true;
    if !spec.is_disabled && !spec.is_read_only {
        root.interaction.on_text_change = on_change;
    }
    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        root.style.descriptor.cursor = CursorHint::NotAllowed;
        root.interaction.disabled = true;
    }
    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.child(inner)
}

fn affix(
    text: &str,
    prefix: bool,
    inline_gap: f32,
    spec: &TextInputSpec,
    theme: &dyn ThemeProvider,
) -> Node {
    let separator_base = theme.resolve_color(spec.affix_separator_color_token());
    let separator = with_alpha(separator_base, separator_base.3 * 0.52);
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.text_color = Some(theme.resolve_color(spec.affix_color_token()));
        s.no_wrap = true;
        if prefix {
            s.descriptor.layout.spacing.padding.right = inline_gap;
            s.descriptor.layout.spacing.margin.right = inline_gap;
            s.border_right_width = Some(1.0);
        } else {
            s.descriptor.layout.spacing.padding.left = inline_gap;
            s.descriptor.layout.spacing.margin.left = inline_gap;
            s.border_left_width = Some(1.0);
        }
        s.descriptor.border.color = separator;
    }
    el.child(Node::text(text))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider;
    use poodle_node::NodeKind;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn root_is_a_full_width_input_with_inline_count_and_validation() {
        let spec = TextInputSpec::new()
            .with_id("email")
            .with_value("bad@")
            .with_max_length(12)
            .with_show_char_count(true)
            .with_validation_state(ValidationState::Invalid);
        let node = text_input(&spec, &theme(), None);

        assert!(matches!(node.kind, NodeKind::Input { .. }));
        assert_eq!(node.id.as_deref(), Some("poodle-input-email"));
        assert!(node.style.fill_width);
        assert_eq!(node.style.descriptor.border.width, 1.0);
        assert_eq!(node.children.len(), 1, "one inline content row");
        assert_eq!(node.children[0].children.len(), 3, "value + count + status");
        assert_eq!(
            node.style.descriptor.border.color,
            theme().resolve_color("color.status.danger")
        );
    }

    #[test]
    fn editable_callback_lives_on_the_input_but_read_only_suppresses_it() {
        let callback: TextChangeHandler = Arc::new(|_| {});
        let editable =
            text_input_with_change(&TextInputSpec::new(), &theme(), Some(callback.clone()));
        assert!(editable.interaction.on_text_change.is_some());

        let read_only = text_input_with_change(
            &TextInputSpec::new().with_read_only(true),
            &theme(),
            Some(callback),
        );
        assert!(read_only.interaction.on_text_change.is_none());
    }

    #[test]
    fn affixes_keep_separator_inside_the_inline_row() {
        let node = text_input(
            &TextInputSpec::new().with_prefix("$").with_suffix("/mo"),
            &theme(),
            None,
        );
        let children = &node.children[0].children;
        assert_eq!(children[0].style.border_right_width, Some(1.0));
        assert_eq!(children[2].style.border_left_width, Some(1.0));
    }
}
