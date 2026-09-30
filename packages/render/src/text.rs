//! Text — the text primitive: tone, size, weight, wrap, compact spacing.
//!
//! Contract: `docs/contracts/components/text.md`
//! Ported from: `packages/jetstream/components/src/text.rs`.

use poodle_node::{LayoutDirection, LayoutOverflow, Node};
use poodle_specs::{TextSpec, TextWeight, TextWrap};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub(crate) fn contains_whitespace_break(content: &str) -> bool {
    content
        .chars()
        .any(|character| matches!(character, ' ' | '\t' | '\n' | '\r' | '\u{000c}'))
}

pub fn text(spec: &TextSpec, ctx: &RenderContext<'_>) -> Node {
    let color = ctx.theme().resolve_color(spec.color_token());
    let weight: u16 = match spec.weight {
        TextWeight::Normal => 400,
        TextWeight::Medium => 500,
        TextWeight::Semibold => 600,
        TextWeight::Bold => 700,
    };

    let mut el = Node::text(&spec.content);
    {
        let s = &mut el.style;
        s.descriptor.text_color = Some(color);
        s.text_size = Some(rem_to_px(spec.font_size_rem()));
        s.text_weight = Some(weight);
        s.line_height = Some(spec.line_height());
        s.text_wrap = true;
        s.wrap_anywhere = spec.wrap == TextWrap::Anywhere;
        s.collapse_text_whitespace = true;
        if spec.wrap == TextWrap::Normal
            && !spec.content.is_empty()
            && !contains_whitespace_break(&spec.content)
        {
            // GPUI normal mode breaks oversized tokens; CSS has no normal
            // break point in this run. Keep the normal wrapping channel
            // available and let no_wrap override GPUI's emergency break.
            s.no_wrap = true;
        }
        // `clamp` degrades to wrapped text clipped at the box, as on both old
        // native tiers — the exact N-line cap + ellipsis stays a backend gap.
        if spec.clamp.is_some() {
            s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        }
    }

    // spacing="compact": stack in a column with the resolved gap.
    if let Some(token) = spec.spacing_gap_token() {
        let gap = ctx.theme().resolve_space(token);
        let mut column = Node::container();
        column.style.descriptor.layout.direction = LayoutDirection::Column;
        column.style.descriptor.layout.spacing.gap = gap;
        return column.child(el);
    }
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{TextLeading, TextSize, TextSpacing, TextTone, TextWeight, TextWrap};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn text_resolves_all_contract_tones() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        for tone in [
            TextTone::Default,
            TextTone::Secondary,
            TextTone::Muted,
            TextTone::Success,
            TextTone::Danger,
            TextTone::Warning,
        ] {
            let spec = TextSpec::new("sample").with_tone(tone);
            let expected_color = ctx.theme().resolve_color(spec.color_token());
            let node = text(&spec, &ctx);
            assert_eq!(
                node.style.descriptor.text_color,
                Some(expected_color),
                "Text tone {tone:?} must resolve to expected color token"
            );
        }
    }

    #[test]
    fn text_resolves_sizes_weights_leading_clamp_and_compact_spacing() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);

        // Sizes
        let xs = text(&TextSpec::new("xs").with_size(TextSize::Xs), &ctx);
        assert_eq!(xs.style.text_size, Some(12.0));
        let sm = text(&TextSpec::new("sm").with_size(TextSize::Sm), &ctx);
        assert_eq!(sm.style.text_size, Some(13.0));
        let md = text(&TextSpec::new("md").with_size(TextSize::Md), &ctx);
        assert_eq!(md.style.text_size, Some(14.0));

        // Weights
        let normal = text(&TextSpec::new("w").with_weight(TextWeight::Normal), &ctx);
        assert_eq!(normal.style.text_weight, Some(400));
        let medium = text(&TextSpec::new("w").with_weight(TextWeight::Medium), &ctx);
        assert_eq!(medium.style.text_weight, Some(500));
        let semibold = text(&TextSpec::new("w").with_weight(TextWeight::Semibold), &ctx);
        assert_eq!(semibold.style.text_weight, Some(600));
        let bold = text(&TextSpec::new("w").with_weight(TextWeight::Bold), &ctx);
        assert_eq!(bold.style.text_weight, Some(700));

        // Leading
        let norm_lead = text(&TextSpec::new("l").with_leading(TextLeading::Normal), &ctx);
        assert_eq!(norm_lead.style.line_height, Some(1.5));
        let relaxed_lead = text(&TextSpec::new("l").with_leading(TextLeading::Relaxed), &ctx);
        assert_eq!(relaxed_lead.style.line_height, Some(1.6));

        // Clamp
        let unclamped = text(&TextSpec::new("c"), &ctx);
        assert_eq!(unclamped.style.descriptor.layout.overflow_x, LayoutOverflow::Visible);
        assert_eq!(unclamped.style.descriptor.layout.overflow_y, LayoutOverflow::Visible);
        let clamped = text(&TextSpec::new("c").with_clamp(2), &ctx);
        assert_eq!(clamped.style.descriptor.layout.overflow_x, LayoutOverflow::Hidden);
        assert_eq!(clamped.style.descriptor.layout.overflow_y, LayoutOverflow::Hidden);

        // Compact spacing
        let compact = text(
            &TextSpec::new("compact").with_spacing(TextSpacing::Compact),
            &ctx,
        );
        assert_eq!(compact.style.descriptor.layout.direction, LayoutDirection::Column);
        assert_eq!(
            compact.style.descriptor.layout.spacing.gap,
            ctx.theme().resolve_space(poodle_tokens::semantic::SPACE_STACK_SM)
        );
        assert_eq!(compact.children.len(), 1);
        match &compact.children[0].kind {
            poodle_node::NodeKind::Text { content } => assert_eq!(content, "compact"),
            _ => panic!("Compact child must be a text node"),
        }
    }

    #[test]
    fn text_wrap_distinguishes_normal_and_anywhere() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);

        let normal_token = text(&TextSpec::new("very-long-identifier"), &ctx);
        assert!(normal_token.style.text_wrap);
        assert!(!normal_token.style.wrap_anywhere);
        assert!(normal_token.style.no_wrap);
        assert!(normal_token.style.collapse_text_whitespace);

        let normal_words = text(&TextSpec::new("first\nsecond"), &ctx);
        assert!(normal_words.style.text_wrap);
        assert!(!normal_words.style.no_wrap);

        let anywhere = text(
            &TextSpec::new("very-long-identifier").with_wrap(TextWrap::Anywhere),
            &ctx,
        );
        assert!(anywhere.style.text_wrap);
        assert!(anywhere.style.wrap_anywhere);
        assert!(!anywhere.style.no_wrap);
    }
}
