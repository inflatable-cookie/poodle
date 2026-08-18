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

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Inline markup",
            theme,
            AgentMessage::from_spec(
                AgentMessageSpec::new(
                    "The `lexer` is **strict** but *forgiving*, see [the docs](https://example.com/md). Also ~~gone~~ kept.",
                ),
                theme,
            ),
        ))
        .child(group(
            "Headings",
            theme,
            AgentMessage::from_spec(AgentMessageSpec::new("# One\n\n### Three\n\n###### Six"), theme),
        ))
        .child(group(
            "Code blocks",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(AgentMessage::from_spec(
                    AgentMessageSpec::new("```rust\nfn main() {\n    println!(\"hi\");\n}\n```"),
                    theme,
                ))
                .child(AgentMessage::from_spec(
                    AgentMessageSpec::new("```\nno language given\n```"),
                    theme,
                )),
        ))
        .child(group(
            "Lists",
            theme,
            AgentMessage::from_spec(AgentMessageSpec::new("- alpha\n- beta\n- gamma"), theme),
        ))
        .child(group(
            "Quotes and rules",
            theme,
            AgentMessage::from_spec(
                AgentMessageSpec::new("> quoted **line**\n> continued\n\nbefore\n\n---\n\nafter"),
                theme,
            ),
        ))
        .child(group(
            "Outside the subset",
            theme,
            AgentMessage::from_spec(
                AgentMessageSpec::new(
                    "A table:\n\n| a | b |\n| - | - |\n| 1 | 2 |\n\nAnd raw <div>markup</div>.",
                ),
                theme,
            ),
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
        .child(group(
            "Roles",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(AgentMessage::from_spec(
                    AgentMessageSpec::new("Can you run the parity sweep again?")
                        .with_role(TranscriptRole::User),
                    theme,
                ))
                .child(AgentMessage::from_spec(
                    AgentMessageSpec::new(
                        "The latest fixes hold: 41 parser tests pass. Remaining deltas are narrow compatibility rules, not parsing failures.",
                    ),
                    theme,
                )),
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
