use crate::node_compat::Text;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

use poodle_specs::{
    TextElement, TextLeading, TextSize, TextSpacing, TextSpec, TextTone, TextWeight,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    // Spacing between specimen rows resolves from the stack-sm token, not a
    // hardcoded px value.
    let stack_gap = px(theme.resolve_space("space.stack.sm"));
    div()
        .flex()
        .flex_col()
        .gap(stack_gap)
        // Tones — all six contract values.
        .child(Text::from_spec(TextSpec::new("Default body text."), theme))
        .child(Text::from_spec(
            TextSpec::new("Secondary supporting text.").with_tone(TextTone::Secondary),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Muted hint text.").with_tone(TextTone::Muted),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Success confirmation text.").with_tone(TextTone::Success),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Danger validation text.").with_tone(TextTone::Danger),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Warning advisory text.").with_tone(TextTone::Warning),
            theme,
        ))
        // Sizes — xs / sm / md.
        .child(Text::from_spec(
            TextSpec::new("Extra-small label text.").with_size(TextSize::Xs),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Small supporting text.").with_size(TextSize::Sm),
            theme,
        ))
        // Weights — medium / semibold / bold.
        .child(Text::from_spec(
            TextSpec::new("Medium weight text.").with_weight(TextWeight::Medium),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Relaxed semibold body copy.")
                .with_leading(TextLeading::Relaxed)
                .with_weight(TextWeight::Semibold),
            theme,
        ))
        .child(Text::from_spec(
            TextSpec::new("Bold emphasis text.").with_weight(TextWeight::Bold),
            theme,
        ))
        // Inline phrase (`as="span"`).
        .child(Text::from_spec(
            TextSpec::new("Inline span phrase.").with_element(TextElement::Span),
            theme,
        ))
        // Compact spacing — grid gap between child paragraphs.
        .child(Text::from_spec(
            TextSpec::new("Compact-spaced paragraph.").with_spacing(TextSpacing::Compact),
            theme,
        ))
        // Clamp — limited to a fixed number of lines (degrades to clip in GPUI).
        .child(Text::from_spec(
            TextSpec::new("Clamped text that would otherwise overflow several lines.")
                .with_clamp(2),
            theme,
        ))
}
