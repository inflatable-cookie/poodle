//! ListCard — Jetstream list card backed by ListCardSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{LeadingFill, LeadingShape, ListCardSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

pub fn js_list_card(spec: &ListCardSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.subtitle_color_token());
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    // Spacing from tokens — no hardcoded rem literals
    let pad_x = resolve_px(theme, "space.panel.x");
    let pad_y = resolve_px(theme, "space.stack.sm");
    let gap = resolve_px(theme, "space.inline.sm");

    // Typography from tokens
    let title_font = resolve_px(theme, "typography.label.size");
    let subtitle_font = resolve_px(theme, "typography.caption.size");
    let meta_font = resolve_px(theme, "typography.caption.size");

    // Leading avatar/swatch dimensions — one icon size unit
    let leading_size = rem_to_px(1.5);
    let leading_accent = resolve_color(theme, spec.leading_tint_bg_token());
    let leading_radius = match spec.leading_shape {
        LeadingShape::Circle => leading_size / 2.0,
        LeadingShape::RoundedSquare => resolve_radius(theme, "radius.control"),
    };
    let leading_bg = match spec.leading_fill {
        LeadingFill::Tint => tint(leading_accent, 0.14),
        LeadingFill::Solid => leading_accent,
    };
    let leading_icon_color = match spec.leading_fill {
        LeadingFill::Tint => leading_accent,
        LeadingFill::Solid => resolve_color(theme, "color.text.on-accent"),
    };

    // Leading element — accent-colored circle/rounded square with an icon or initial
    let leading_el = ui_element::div()
        .size(leading_size)
        .flex_none()
        .rounded(leading_radius)
        .bg(leading_bg)
        .items_center()
        .justify_center()
        .child(
            // First character of title as placeholder when no icon is available
            ui_element::label(&spec.title.chars().next().map_or(String::new(), |c| c.to_uppercase().to_string()))
                .text_color(leading_icon_color)
                .text_size(title_font)
                .text_weight(600),
        );

    // Text block: title + optional subtitle stacked
    let mut text_block = ui_element::div()
        .flex_col()
        .gap(rem_to_px(0.125))
        .flex_grow()
        .min_w_0();

    text_block = text_block.child(
        ui_element::label(&spec.title)
            .text_color(text_primary)
            .text_size(title_font)
            .text_weight(600)
            .text_ellipsis(),
    );

    if let Some(ref subtitle) = spec.subtitle {
        text_block = text_block.child(
            ui_element::label(subtitle)
                .text_color(text_secondary)
                .text_size(subtitle_font)
                .text_ellipsis(),
        );
    }

    // Meta label (trailing, right-aligned)
    let meta_el = spec.meta.as_ref().map(|m| {
        ui_element::label(m)
            .text_color(text_secondary)
            .text_size(meta_font)
            .flex_none()
    });

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border)
        .rounded(radius)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .flex_row()
        .items_center()
        .gap(gap);

    el = el.child(leading_el).child(text_block);

    if let Some(m) = meta_el {
        el = el.child(m);
    }

    // Disabled: reduce opacity via token, not hardcoded value
    if spec.is_disabled {
        el = el.opacity(disabled_opacity);
    }

    // Interactive: show pointer cursor and focusable
    if spec.is_interactive || spec.href.is_some() {
        el = el.cursor_pointer().focusable();
    }

    el
}
