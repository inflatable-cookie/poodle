use crate::app_state::AppState;
use crate::node_compat::{AgentMessage, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_transcript::TranscriptRole;
use poodle_specs::{AgentMessageSpec, EyebrowSpec};

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

fn stack(children: impl IntoIterator<Item = AnyElement>) -> Div {
    let mut col = div().flex().flex_col().gap(px(12.0));
    for child in children {
        col = col.child(child);
    }
    col
}

fn message(markdown: &str, theme: &GpuiThemeProvider) -> AnyElement {
    AgentMessage::from_spec(AgentMessageSpec::new(markdown), theme).into_any_element()
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Assistant and user messages",
            theme,
            stack([
                message("The latest parser fixes hold.", theme),
                AgentMessage::from_spec(
                    AgentMessageSpec::new("Can you run the parity sweep again?")
                        .with_role(TranscriptRole::User),
                    theme,
                )
                .into_any_element(),
                message(
                    "The latest fixes hold: 41 parser tests pass. Remaining deltas are narrow compatibility rules, not parsing failures.",
                    theme,
                ),
            ])
            .into_any_element(),
        ))
        .child(group(
            "Inline formatting and headings",
            theme,
            stack([
                message(
                    "The `lexer` is **strict** but *forgiving*, see [the docs](https://example.com/md). Also ~~gone~~ kept.",
                    theme,
                ),
                message(
                    "# One\n\n## Two\n\n### Three\n\n#### Four\n\n##### Five\n\n###### Six",
                    theme,
                ),
            ])
            .into_any_element(),
        ))
        .child(group(
            "Code blocks",
            theme,
            stack([
                message("```rust\nfn main() {\n    println!(\"hi\");\n}\n```", theme),
                message("```\nno language given\n```", theme),
            ])
            .into_any_element(),
        ))
        .child(group(
            "List structures",
            theme,
            stack([
                message("- alpha\n- beta\n- gamma", theme),
                message("- alpha\n\n- beta", theme),
                message("3. three\n4. four", theme),
                message("- outer\n  - inner one\n  - inner two\n- second", theme),
                message("1. run this:\n\n   ```sh\n   bun test\n   ```", theme),
            ])
            .into_any_element(),
        ))
        .child(group(
            "Quotes, rules and fallback",
            theme,
            stack([
                message("> quoted **line**\n> continued", theme),
                message("before\n\n---\n\nafter", theme),
                message(
                    "A table:\n\n| a | b |\n| - | - |\n| 1 | 2 |\n\nAnd raw <div>markup</div>.",
                    theme,
                ),
            ])
            .into_any_element(),
        ))
        .child(group(
            "Streaming",
            theme,
            AgentMessage::from_spec(
                AgentMessageSpec::new("Regenerating the corpus against the cached oracle")
                    .with_streaming(true),
                theme,
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "agent-message",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                AgentMessage::from_spec(
                    AgentMessageSpec::new("Size prose measure and type scale move together.")
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                AgentMessage::from_spec(
                    AgentMessageSpec::new("Density\n\n- one\n- two").with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
