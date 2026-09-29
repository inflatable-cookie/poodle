use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, Text};
use crate::specimens::specimen_axes::TEXT_SIZES;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;

use poodle_specs::{
    EyebrowSpec, TextElement, TextLeading, TextSize, TextSpacing, TextSpec, TextTone,
    TextWeight,
};

/// One captioned example group: an Eyebrow caption over its content.
fn group(theme: &GpuiThemeProvider, caption: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(caption),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    // Spacing between specimen rows resolves from the stack-sm token, not a
    // hardcoded px value.
    let stack_gap = px(theme.resolve_space("space.stack.sm"));
    let stack = || div().flex().flex_col().gap(stack_gap);

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            theme,
            "Tones — all six contract values",
            stack()
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
                )),
        ))
        .child(group(
            theme,
            "Weight and leading",
            stack()
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
                )),
        ))
        .child(group(
            theme,
            "Inline phrase — renders as a span",
            Text::from_spec(
                TextSpec::new("Inline span phrase.").with_element(TextElement::Span),
                theme,
            ),
        ))
        .child(group(
            theme,
            "Compact spacing — grid gap between child paragraphs",
            Text::from_spec(
                TextSpec::new("Compact-spaced paragraph.").with_spacing(TextSpacing::Compact),
                theme,
            ),
        ))
        .child(group(
            theme,
            "Clamp — degrades to clip in GPUI",
            Text::from_spec(
                TextSpec::new("Clamped text that would otherwise overflow several lines.")
                    .with_clamp(2),
                theme,
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "text",
        examples,
        SpecimenAxes::examples_only()
            .with_named_sizes(TEXT_SIZES, |value, theme: &GpuiThemeProvider| {
                let text_size = match value {
                    "xs" => TextSize::Xs,
                    "sm" => TextSize::Sm,
                    _ => TextSize::Md,
                };
                Text::from_spec(
                    TextSpec::new("Default body text for admin and product surfaces.")
                        .with_size(text_size),
                    theme,
                )
                .into_any_element()
            }),
    )
}
