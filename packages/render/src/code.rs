//! Code — inline fragments and block display with toolbar, gutter, highlights.
//!
//! Contract: `docs/contracts/components/code.md`
//! Ported from: `packages/jetstream/components/src/code.rs`. The copy button
//! exposes its press through [`CodeHandlers::on_copy`]; clipboard and the 2s
//! check swap stay host interactions.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, FocusRing, FontFamily, LayoutDirection, LayoutOverflow,
    LayoutSizing, MainAxisAlignment, Node, NodeRole, TextAlign,
};
use poodle_specs::{CodeInlineVariant, CodeSpec, CodeTypography, CodeWrap};

use crate::color::{mix_srgb, with_alpha, BLACK};
use crate::context::RenderContext;
use crate::presentation::{panel_space_x_rem, panel_space_y_rem, rem_to_px, size_font_rem};
use crate::text::contains_whitespace_break;

fn rounded_all(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

/// Copy affordance shared by the block toolbar and the inline fragment.
/// Svelte renders a real `<button>` with an accessible label (contract §6);
/// clipboard and the 2s check swap stay host-owned, but the affordance
/// itself is a focusable button in every tier. `id` keeps the block and
/// inline buttons distinctly addressable when both mount together.
fn copy_button(
    theme: &dyn ThemeProvider,
    id: &str,
    size_rem: f32,
    icon_rem: f32,
    source: &str,
    copied: bool,
    on_copy: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let mut copy = Node::container();
    copy.id = Some(id.to_string());
    copy.a11y.role = Some(NodeRole::Button);
    // Svelte swaps the label and glyph for 2s after a clipboard write;
    // the host flips `copied` through the spec and owns the reset timer.
    copy.a11y.label = Some(if copied {
        "Copied".to_string()
    } else {
        "Copy to clipboard".to_string()
    });
    copy.a11y.tab_index = Some(0);
    copy.interaction.focusable = true;
    // The backend writes this to the platform clipboard on activation, so
    // the default path copies with zero host code (contract §5/§10).
    copy.interaction.copy_text = Some(source.to_string());
    copy.style.focus_ring = Some(FocusRing {
        color: theme.resolve_color("color.accent.focusRing"),
        width: theme.resolve_border_width("border.width.focus"),
        offset: rem_to_px(0.125),
    });
    {
        let s = &mut copy.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(size_rem));
        s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(size_rem));
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.text_color = Some(text_secondary);
        s.descriptor.cursor = CursorHint::Pointer;
    }
    rounded_all(&mut copy, rem_to_px(0.25));
    if let Some(on_copy) = on_copy {
        copy.interaction.on_activate = Some(Arc::new(move || on_copy()));
    }
    let mut icon = Node::icon(if copied { "check" } else { "copy" }, rem_to_px(icon_rem));
    icon.style.descriptor.text_color = Some(text_secondary);
    copy.child(icon)
}

pub struct CodeHandlers {
    /// Fired when the copy affordance is activated. The host owns the
    /// platform clipboard write and the 2s feedback swap (contract §4:
    /// adapter-owned interaction); without a handler the button still
    /// renders, focuses, and labels itself, but presses go nowhere.
    pub on_copy: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub fn code(spec: &CodeSpec, ctx: &RenderContext<'_>) -> Node {
    code_with_handlers(spec, ctx, CodeHandlers { on_copy: None })
}

pub fn code_with_handlers(
    spec: &CodeSpec,
    ctx: &RenderContext<'_>,
    handlers: CodeHandlers,
) -> Node {
    let theme = ctx.theme();
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

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
            s.text_wrap = true;
            s.wrap_anywhere = spec.wrap == CodeWrap::Anywhere;
            s.collapse_text_whitespace = true;
            s.no_wrap = spec.wrap == CodeWrap::Normal
                && !spec.content.is_empty()
                && !contains_whitespace_break(&spec.content);
            // Inside the inline wrap row the fragment needs the same shrink
            // contract as a block source line, or the row width never
            // constrains the wrapping text.
            if spec.wrap == CodeWrap::Anywhere {
                s.flex_grow = Some(1.0);
                s.min_width = Some(0.0);
            }
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
        // Svelte always wraps the inline fragment in an inline-flex span
        // with a 0.25rem gap and shows the compact copy button beside it
        // when copyable (contract §2/§8: 1.25rem button, 0.75rem icon).
        let mut wrap = Node::container();
        {
            let s = &mut wrap.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = rem_to_px(0.25);
        }
        wrap = wrap.child(el);
        if spec.is_copyable {
            wrap = wrap.child(copy_button(
                theme,
                "poodle-code-copy-inline",
                1.25,
                0.75,
                &spec.content,
                spec.copied,
                handlers.on_copy,
            ));
        }
        return wrap;
    }

    // ── Block mode ──
    let pre_pad_x = rem_to_px(panel_space_x_rem(density));
    let pre_pad_y = rem_to_px(panel_space_y_rem(density));
    let source_font = rem_to_px(size_font_rem(effective_size));

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
            toolbar = toolbar.child(copy_button(
                theme,
                "poodle-code-copy",
                1.5,
                0.875,
                &spec.content,
                spec.copied,
                handlers.on_copy,
            ));
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
        s.line_height = Some(1.4);
        s.fill_width = true;
        s.descriptor.layout.overflow_x = LayoutOverflow::Scroll;
        s.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
        if let Some(mh) = spec.max_height {
            s.max_height = Some(mh as f32);
        }
    }

    let needs_per_line =
        spec.show_line_numbers || !spec.highlight_lines.is_empty() || spec.content.ends_with('\n');

    if needs_per_line {
        // Svelte renders each block line as its own block span. Empty source
        // spans, including the one after a trailing LF, have zero height
        // unless a line-number gutter supplies content.
        for (i, line) in spec.content.split('\n').enumerate() {
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

            if line.is_empty() {
                scroll = scroll.child(row);
                continue;
            }

            let mut source = Node::text(line.to_string());
            source.style.font_family = Some(FontFamily::Mono);
            if spec.wrap == CodeWrap::Anywhere {
                source.style.text_wrap = true;
                source.style.wrap_anywhere = true;
                source.style.flex_grow = Some(1.0);
                source.style.min_width = Some(0.0);
            } else {
                source.style.no_wrap = true;
            }
            scroll = scroll.child(row.child(source));
        }
    } else {
        let mut source = Node::text(&spec.content);
        source.style.font_family = Some(FontFamily::Mono);
        if spec.wrap == CodeWrap::Anywhere {
            source.style.text_wrap = true;
            source.style.wrap_anywhere = true;
            source.style.fill_width = true;
        } else {
            source.style.no_wrap = true;
        }
        scroll = scroll.child(source);
    }

    root = root.child(scroll);
    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_source_uses_contract_relative_line_height() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let node = code(&CodeSpec::new().with_content("let value = 1;"), &ctx);
        let scroll = node
            .children
            .last()
            .expect("block code always renders a source surface");

        assert_eq!(scroll.style.line_height, Some(1.4));
    }

    #[test]
    fn inline_wrap_preserves_normal_breaks_and_anywhere_breaks_tokens() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        // Svelte wraps every inline fragment with the adjacent copy button
        // by default; the fragment carries the wrap directives.
        let normal_token = code(
            &CodeSpec::new()
                .with_content("very-long-identifier")
                .with_inline(true),
            &ctx,
        );
        assert_eq!(normal_token.children.len(), 2);
        let fragment = &normal_token.children[0];
        assert!(fragment.style.no_wrap);
        assert!(!fragment.style.wrap_anywhere);
        assert!(fragment.style.collapse_text_whitespace);
        let copy = &normal_token.children[1];
        assert_eq!(copy.id.as_deref(), Some("poodle-code-copy-inline"));
        assert_eq!(copy.a11y.role, Some(NodeRole::Button));
        assert_eq!(copy.a11y.label.as_deref(), Some("Copy to clipboard"));
        assert!(copy.interaction.focusable);

        let bare = code(
            &CodeSpec::new()
                .with_content("very-long-identifier")
                .with_inline(true)
                .with_copyable(false),
            &ctx,
        );
        assert_eq!(bare.children.len(), 1);

        let normal_words = code(
            &CodeSpec::new()
                .with_content("first\nsecond")
                .with_inline(true)
                .with_copyable(false),
            &ctx,
        );
        let words_fragment = &normal_words.children[0];
        assert!(words_fragment.style.text_wrap);
        assert!(!words_fragment.style.no_wrap);

        let anywhere = code(
            &CodeSpec::new()
                .with_content("very-long-identifier")
                .with_inline(true)
                .with_wrap(CodeWrap::Anywhere)
                .with_copyable(false),
            &ctx,
        );
        let anywhere_fragment = &anywhere.children[0];
        assert!(anywhere_fragment.style.text_wrap);
        assert!(anywhere_fragment.style.wrap_anywhere);
        assert!(!anywhere_fragment.style.no_wrap);
    }

    #[test]
    fn block_wrap_preserves_newlines_and_controls_long_tokens() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let normal = code(&CodeSpec::new().with_content("first\n"), &ctx);
        let normal_scroll = normal.children.last().expect("block code scroll");
        assert_eq!(normal_scroll.children.len(), 2);
        let normal_source = normal_scroll.children[0]
            .children
            .first()
            .expect("first block source line");
        assert!(normal_source.style.no_wrap);
        assert!(!normal_source.style.collapse_text_whitespace);
        assert!(matches!(
            &normal_source.kind,
            poodle_node::NodeKind::Text { content } if content == "first"
        ));
        assert!(normal_scroll.children[1].children.is_empty());

        let numbered = code(
            &CodeSpec::new()
                .with_content("first\n")
                .with_show_line_numbers(true),
            &ctx,
        );
        let numbered_scroll = numbered.children.last().expect("numbered code scroll");
        assert_eq!(numbered_scroll.children.len(), 2);

        let anywhere = code(
            &CodeSpec::new()
                .with_content("first\n")
                .with_wrap(CodeWrap::Anywhere),
            &ctx,
        );
        let anywhere_source = anywhere
            .children
            .last()
            .and_then(|scroll| scroll.children.first())
            .and_then(|row| row.children.first())
            .expect("block source");
        assert!(anywhere_source.style.text_wrap);
        assert!(anywhere_source.style.wrap_anywhere);
        assert!(!anywhere_source.style.no_wrap);
    }
}
