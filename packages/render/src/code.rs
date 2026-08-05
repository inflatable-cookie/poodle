//! Code — inline fragments and block display with toolbar, gutter, highlights.
//!
//! Contract: `docs/contracts/components/code.md`
//! Ported from: `packages/jetstream/components/src/code.rs`. The copy button
//! renders inert here — clipboard and the 2s check swap are host interactions.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutOverflow,
    LayoutSizing, MainAxisAlignment, Node, TextAlign,
};
use poodle_specs::{CodeInlineVariant, CodeSpec, CodeTypography};

use crate::color::{mix_srgb, with_alpha, BLACK};
use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};

fn rounded_all(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

pub fn code(spec: &CodeSpec, theme: &dyn ThemeProvider) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let text_color = theme.resolve_color(spec.text_color_token());
    let text_secondary = theme.resolve_color(spec.text_secondary_token());
    let panel = theme.resolve_color(spec.panel_token());
    let elevated = theme.resolve_color(spec.elevated_token());
    let canvas = theme.resolve_color(spec.canvas_token());
    let accent = theme.resolve_color(spec.accent_token());

    // ── Inline mode ──
    if spec.is_inline {
        let ratio = poodle_tokens::typed::semantic::TYPOGRAPHY_CODE_ADJUSTMENT_RATIO;
        let base_em = match spec.typography {
            CodeTypography::Inline => 1.0,
            CodeTypography::Body => size_font_rem(effective_size),
        };
        let inline_font = rem_to_px(base_em * ratio);

        let mut el = Node::text(&spec.content);
        {
            let s = &mut el.style;
            s.text_size = Some(inline_font);
            s.descriptor.text_color = Some(text_color);
            s.font_family = Some(FontFamily::Mono);
            s.no_wrap = true;
            if spec.inline_variant == CodeInlineVariant::Default {
                let inline_bg = mix_srgb(panel, elevated, 0.72);
                s.descriptor.layout.spacing.padding.left = rem_to_px(0.375);
                s.descriptor.layout.spacing.padding.right = rem_to_px(0.375);
                s.descriptor.layout.spacing.padding.top = rem_to_px(0.125);
                s.descriptor.layout.spacing.padding.bottom = rem_to_px(0.125);
                s.descriptor.background = Some(inline_bg);
            }
        }
        if spec.inline_variant == CodeInlineVariant::Default {
            rounded_all(&mut el, rem_to_px(0.25));
        }
        return el;
    }

    // ── Block mode ──
    let pre_pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let pre_pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let source_font = rem_to_px(size_font_rem(effective_size));
    let source_line_height = rem_to_px(size_font_rem(effective_size) * 1.4);

    let border = theme.resolve_color(spec.border_token());
    let radius = theme.resolve_radius(spec.surface_radius_token());
    let border_width = rem_to_px(0.0625);

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border;
        s.descriptor.text_color = Some(text_color);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
    }
    rounded_all(&mut root, radius);

    // ── Toolbar ──
    let has_toolbar = spec.language.is_some() || spec.is_copyable;
    if has_toolbar {
        let toolbar_bg = mix_srgb(elevated, panel, 0.60);
        let mut toolbar = Node::container();
        {
            let s = &mut toolbar.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
            s.descriptor.layout.spacing.padding.left = rem_to_px(0.625);
            s.descriptor.layout.spacing.padding.right = rem_to_px(0.625);
            s.descriptor.layout.spacing.padding.top = rem_to_px(0.375);
            s.descriptor.layout.spacing.padding.bottom = rem_to_px(0.375);
            s.descriptor.background = Some(toolbar_bg);
            s.border_bottom_width = Some(1.0);
            s.descriptor.border.color = border;
        }

        if let Some(ref lang) = spec.language {
            let mut label = Node::text(lang.to_uppercase());
            let s = &mut label.style;
            s.text_size = Some(rem_to_px(0.6875));
            s.descriptor.text_color = Some(text_secondary);
            s.text_weight = Some(500);
            s.letter_spacing_em = Some(0.05);
            toolbar = toolbar.child(label);
        } else {
            // Spacer keeps the actions right-aligned. Explicit Row (see
            // switch.rs) even for an empty spacer — the old tier's default.
            let mut spacer = Node::container();
            spacer.style.descriptor.layout.direction = LayoutDirection::Row;
            toolbar = toolbar.child(spacer);
        }

        if spec.is_copyable {
            let mut copy = Node::container();
            copy.id = Some("poodle-code-copy".to_string());
            {
                let s = &mut copy.style;
                s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(1.5));
                s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(1.5));
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.descriptor.text_color = Some(text_secondary);
                s.descriptor.cursor = CursorHint::Pointer;
            }
            rounded_all(&mut copy, rem_to_px(0.25));
            let mut icon = Node::icon("copy", rem_to_px(0.875));
            icon.style.descriptor.text_color = Some(text_secondary);
            toolbar = toolbar.child(copy.child(icon));
        }

        root = root.child(toolbar);
    }

    // ── Code surface (scroll + pre + source) ──
    let pre_bg = mix_srgb(canvas, BLACK, 0.92);
    let highlight_bg = with_alpha(accent, accent.3 * 0.12);

    let mut scroll = Node::container();
    scroll.id = Some("poodle-code-scroll".to_string());
    {
        let s = &mut scroll.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.background = Some(pre_bg);
        s.descriptor.layout.spacing.padding.left = pre_pad_x;
        s.descriptor.layout.spacing.padding.right = pre_pad_x;
        s.descriptor.layout.spacing.padding.top = pre_pad_y;
        s.descriptor.layout.spacing.padding.bottom = pre_pad_y;
        s.text_size = Some(source_font);
        s.line_height = Some(source_line_height);
        s.descriptor.layout.overflow_x = LayoutOverflow::Scroll;
        s.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
        if let Some(mh) = spec.max_height {
            s.max_height = Some(mh as f32);
        }
    }

    let needs_per_line = spec.show_line_numbers || !spec.highlight_lines.is_empty();

    if needs_per_line {
        for (i, line) in spec.content.lines().enumerate() {
            let line_no = i + 1;
            let is_highlighted = spec.highlight_lines.contains(&line_no);

            let mut row = Node::container();
            {
                let s = &mut row.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
                if is_highlighted {
                    // ±1rem bleed: negative margin + matching padding.
                    s.descriptor.background = Some(highlight_bg);
                    s.descriptor.layout.spacing.margin.left = rem_to_px(-1.0);
                    s.descriptor.layout.spacing.margin.right = rem_to_px(-1.0);
                    s.descriptor.layout.spacing.padding.left = rem_to_px(1.0);
                    s.descriptor.layout.spacing.padding.right = rem_to_px(1.0);
                }
            }

            if spec.show_line_numbers {
                let mut gutter = Node::text(line_no.to_string());
                let s = &mut gutter.style;
                s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(2.5));
                s.descriptor.layout.spacing.padding.right = rem_to_px(1.0);
                s.descriptor.text_color = Some(text_secondary);
                s.font_family = Some(FontFamily::Mono);
                s.text_align = Some(TextAlign::Right);
                row = row.child(gutter);
            }

            let mut source = Node::text(line.to_string());
            source.style.font_family = Some(FontFamily::Mono);
            source.style.no_wrap = true;
            scroll = scroll.child(row.child(source));
        }
    } else {
        let mut source = Node::text(&spec.content);
        source.style.font_family = Some(FontFamily::Mono);
        scroll = scroll.child(source);
    }

    root = root.child(scroll);
    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root
}
