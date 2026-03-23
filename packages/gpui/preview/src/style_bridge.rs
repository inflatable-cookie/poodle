//! Bridge between flint-gpui style types and real gpui style types.
//!
//! Converts `GpuiColor` / `ColorValue` to `gpui::Hsla` and applies
//! `GpuiStyle` fields to gpui `Div` elements via `Styled` trait methods.

use gpui::*;
use flint_gpui::{
    GpuiAlignItems, GpuiColor, GpuiFlexDirection, GpuiJustifyContent, GpuiLength, GpuiOverflow,
    GpuiStyle,
};
use flint_tokens::typed::ColorValue;

/// Convert a Flint `ColorValue` (RGBA f32) to gpui's `Hsla`.
pub fn color_to_hsla(c: ColorValue) -> Hsla {
    let rgba = gpui::Rgba {
        r: c.0,
        g: c.1,
        b: c.2,
        a: c.3,
    };
    rgba.into()
}

/// Convert a `GpuiColor` to gpui's `Hsla`.
pub fn gpui_color_to_hsla(c: &GpuiColor) -> Hsla {
    let rgba = gpui::Rgba {
        r: c.r,
        g: c.g,
        b: c.b,
        a: c.a,
    };
    rgba.into()
}

/// Apply a complete `GpuiStyle` to a gpui `Div`, returning the styled element.
///
/// Maps all layout, visual, and typography properties from the adapter's
/// intermediate style representation to real gpui styling calls.
pub fn apply_style(element: Div, style: &GpuiStyle) -> Div {
    let mut el = element.flex();

    // Flex direction
    el = match style.flex_direction {
        GpuiFlexDirection::Row => el.flex_row(),
        GpuiFlexDirection::Column => el.flex_col(),
    };

    // Dimensions
    el = apply_length_w(el, &style.width);
    el = apply_length_h(el, &style.height);
    if let Some(min_w) = style.min_width {
        el = el.min_w(px(min_w));
    }
    if let Some(max_w) = style.max_width {
        el = el.max_w(px(max_w));
    }
    if let Some(min_h) = style.min_height {
        el = el.min_h(px(min_h));
    }
    if let Some(max_h) = style.max_height {
        el = el.max_h(px(max_h));
    }

    // Flex grow/shrink
    if style.flex_grow > 0.0 {
        el = el.flex_grow();
    }
    if style.flex_shrink == 0.0 {
        el = el.flex_shrink_0();
    }

    // Gap
    if style.gap > 0.0 {
        el = el.gap(px(style.gap));
    }

    // Padding
    if style.padding.top > 0.0 {
        el = el.pt(px(style.padding.top));
    }
    if style.padding.right > 0.0 {
        el = el.pr(px(style.padding.right));
    }
    if style.padding.bottom > 0.0 {
        el = el.pb(px(style.padding.bottom));
    }
    if style.padding.left > 0.0 {
        el = el.pl(px(style.padding.left));
    }

    // Margin
    if style.margin.top > 0.0 {
        el = el.mt(px(style.margin.top));
    }
    if style.margin.right > 0.0 {
        el = el.mr(px(style.margin.right));
    }
    if style.margin.bottom > 0.0 {
        el = el.mb(px(style.margin.bottom));
    }
    if style.margin.left > 0.0 {
        el = el.ml(px(style.margin.left));
    }

    // Justify content
    el = match style.justify_content {
        GpuiJustifyContent::Start => el.justify_start(),
        GpuiJustifyContent::Center => el.justify_center(),
        GpuiJustifyContent::End => el.justify_end(),
        GpuiJustifyContent::SpaceBetween => el.justify_between(),
    };

    // Align items
    el = match style.align_items {
        GpuiAlignItems::Start => el.items_start(),
        GpuiAlignItems::Center => el.items_center(),
        GpuiAlignItems::End => el.items_end(),
        GpuiAlignItems::Stretch => el,
    };

    // Overflow (non-scroll variants via Styled trait)
    el = apply_overflow(el, &style.overflow_x, &style.overflow_y);

    // Background
    if let Some(ref bg) = style.background {
        el = el.bg(gpui_color_to_hsla(bg));
    }

    // Text color
    if let Some(ref tc) = style.text_color {
        el = el.text_color(gpui_color_to_hsla(tc));
    }

    // Border
    if style.border_width > 0.0 {
        el = el.border_1();
        if let Some(ref bc) = style.border_color {
            el = el.border_color(gpui_color_to_hsla(bc));
        }
    }

    // Corner radii (use uniform if all corners are equal)
    let cr = &style.corner_radii;
    if cr.top_left > 0.0 || cr.top_right > 0.0 || cr.bottom_right > 0.0 || cr.bottom_left > 0.0 {
        let uniform = cr.top_left == cr.top_right
            && cr.top_right == cr.bottom_right
            && cr.bottom_right == cr.bottom_left;
        if uniform {
            el = el.rounded(px(cr.top_left));
        } else {
            el = el
                .rounded_tl(px(cr.top_left))
                .rounded_tr(px(cr.top_right))
                .rounded_br(px(cr.bottom_right))
                .rounded_bl(px(cr.bottom_left));
        }
    }

    // Shadow — use predefined sizes as approximation
    if let Some(ref shadow) = style.shadow {
        if shadow.blur > 12.0 {
            el = el.shadow_lg();
        } else if shadow.blur > 4.0 {
            el = el.shadow_md();
        } else if shadow.blur > 0.0 {
            el = el.shadow_sm();
        }
    }

    // Opacity
    if style.opacity < 1.0 {
        el = el.opacity(style.opacity);
    }

    // Visibility — gpui doesn't have a direct visible() setter;
    // hidden elements are handled at the parent level via conditional rendering.

    // Typography
    if let Some(ref typo) = style.typography {
        el = el.text_size(px(typo.font_size));
    }

    el
}

fn apply_length_w(el: Div, length: &GpuiLength) -> Div {
    match length {
        GpuiLength::Auto => el,
        GpuiLength::Definite(v) => el.w(px(*v)),
    }
}

fn apply_length_h(el: Div, length: &GpuiLength) -> Div {
    match length {
        GpuiLength::Auto => el,
        GpuiLength::Definite(v) => el.h(px(*v)),
    }
}

fn apply_overflow(el: Div, overflow_x: &GpuiOverflow, overflow_y: &GpuiOverflow) -> Div {
    match (overflow_x, overflow_y) {
        (GpuiOverflow::Hidden, GpuiOverflow::Hidden) => el.overflow_hidden(),
        (GpuiOverflow::Hidden, _) => el.overflow_x_hidden(),
        (_, GpuiOverflow::Hidden) => el.overflow_y_hidden(),
        _ => el,
    }
}
