//! AgentMessage — agent prose, backed by `AgentMessageSpec`.
//!
//! Contract: `docs/contracts/components/agent-message.md`.
//!
//! Markdown parses through `poodle-markdown` into the same block model the web
//! target renders, so both show the same document.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_markdown::{parse_markdown, MdBlock, MdInline};
use poodle_specs::AgentMessageSpec;

use crate::element::IntoJsEl;
use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

/// AgentMessage — agent prose.
///
/// No `on_link_click` yet, unlike the web target: inline nodes flatten to text
/// here, so there is no link element to hang one on. It arrives with rich
/// inline runs, not before — a handler for a link that is not drawn is exactly
/// the dead-handler pattern this shape exists to prevent.
pub struct AgentMessage {
    spec: AgentMessageSpec,
    theme: JetstreamThemeProvider,
}

impl AgentMessage {
    pub fn from_spec(spec: AgentMessageSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoJsEl for AgentMessage {
    fn into_js_el(self) -> JsEl {
        js_agent_message(&self.spec, &self.theme)
    }
}

/// Flatten inline nodes to text.
///
/// Jetstream has no rich inline runs here, so emphasis and links resolve to
/// their text rather than being dropped — losing content is the worst available
/// failure for a transcript. Recorded as a delta on the contract.
fn inline_text(nodes: &[MdInline], out: &mut String) {
    for node in nodes {
        match node {
            MdInline::Text(value) | MdInline::Code(value) => out.push_str(value),
            MdInline::Break => out.push(' '),
            MdInline::Strong(children)
            | MdInline::Em(children)
            | MdInline::Del(children)
            | MdInline::Link { children, .. } => inline_text(children, out),
        }
    }
}

fn line(nodes: &[MdInline]) -> String {
    let mut out = String::new();
    inline_text(nodes, &mut out);
    out
}

struct Style {
    font_size: f32,
    block_gap: f32,
    indent: f32,
    text: Color,
    quote_text: Color,
    quote_rule: Color,
    code_fill: Color,
}

fn render_blocks(blocks: &[MdBlock], s: &Style) -> JsEl {
    let mut body = ui_element::div().flex_col().w_full().gap(s.block_gap);

    for block in blocks {
        match block {
            MdBlock::Paragraph(children) => {
                body = body.child(
                    ui_element::label(line(children))
                        .text_size(s.font_size)
                        .text_color(s.text),
                );
            }
            MdBlock::Heading { level, children } => {
                // The heading ramp mirrors the web target's `em` scale.
                let scale = match level {
                    1 => 1.5,
                    2 => 1.3,
                    3 => 1.15,
                    4 => 1.05,
                    5 => 1.0,
                    _ => 0.9,
                };
                body = body.child(
                    ui_element::label(line(children))
                        .text_size(s.font_size * scale)
                        .text_weight(600)
                        .text_color(s.text),
                );
            }
            MdBlock::Code { value, .. } => {
                body = body.child(
                    ui_element::div()
                        .w_full()
                        .p(rem_to_px(0.375))
                        .bg(s.code_fill)
                        .child(
                            ui_element::label(value.clone())
                                .text_size(s.font_size)
                                .text_color(s.text),
                        ),
                );
            }
            MdBlock::List { ordered, start, items } => {
                let mut list = ui_element::div().flex_col().w_full().pl(s.indent).gap(rem_to_px(0.125));
                for (index, item) in items.iter().enumerate() {
                    let marker = if *ordered {
                        format!("{}.", *start as usize + index)
                    } else {
                        "•".to_string()
                    };
                    list = list.child(
                        ui_element::div()
                            .flex_row()
                            .w_full()
                            .gap(rem_to_px(0.375))
                            .child(
                                ui_element::label(marker)
                                    .text_size(s.font_size)
                                    .text_color(s.quote_text)
                                    .flex_shrink_0(),
                            )
                            .child(render_blocks(item, s)),
                    );
                }
                body = body.child(list);
            }
            MdBlock::Blockquote(children) => {
                let quoted = Style {
                    text: s.quote_text,
                    ..*s
                };
                body = body.child(
                    ui_element::div()
                        .w_full()
                        .pl(s.indent)
                        .border_l_1()
                        .border_color(s.quote_rule)
                        .child(render_blocks(children, &quoted)),
                );
            }
            MdBlock::Rule => {
                body = body.child(ui_element::div().w_full().h(rem_to_px(0.0625)).bg(s.quote_rule));
            }
        }
    }

    body
}

impl Clone for Style {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}
impl Copy for Style {}

pub fn js_agent_message(spec: &AgentMessageSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // An empty message contributes no box: a turn with nothing in it should not
    // reserve space in the transcript.
    if !spec.renders() {
        return ui_element::div();
    }

    let style = Style {
        font_size: rem_to_px(spec.font_size_rem()),
        block_gap: rem_to_px(spec.block_gap_rem()),
        indent: rem_to_px(spec.list_indent_rem()),
        text: resolve_color(theme, spec.text_token()).into(),
        quote_text: resolve_color(theme, spec.quote_text_token()).into(),
        quote_rule: resolve_color(theme, spec.quote_rule_token()).into(),
        code_fill: resolve_color(theme, spec.code_span_fill_token()).into(),
    };

    let blocks = parse_markdown(&spec.markdown);
    let mut root = render_blocks(&blocks, &style);

    if spec.is_user() {
        let surface: Color = resolve_color(theme, spec.user_surface_token()).into();
        root = ui_element::div()
            .flex_col()
            .w_full()
            .p(rem_to_px(spec.padding_inset_rem()))
            .rounded(resolve_radius(theme, spec.radius_token()))
            .bg(surface)
            .child(root);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn inline_markup_survives_as_text_rather_than_vanishing() {
        let spec = AgentMessageSpec::new("run `bun test` and **stop**");
        let tree = crate::render_probe::probe(&js_agent_message(&spec, &theme()), 720.0, 96.0);

        let joined = tree.texts().join(" ");
        assert!(joined.contains("bun test"), "{joined:?}");
        assert!(joined.contains("stop"), "{joined:?}");
    }

    #[test]
    fn an_empty_message_contributes_nothing() {
        let tree = crate::render_probe::probe(
            &js_agent_message(&AgentMessageSpec::new(""), &theme()),
            720.0,
            96.0,
        );
        assert!(tree.texts().is_empty(), "{:?}", tree.texts());
    }

    #[test]
    fn list_items_render_their_content() {
        let spec = AgentMessageSpec::new("- alpha\n- beta");
        let tree = crate::render_probe::probe(&js_agent_message(&spec, &theme()), 720.0, 128.0);

        assert!(tree.has_text("alpha"), "{:?}", tree.texts());
        assert!(tree.has_text("beta"), "{:?}", tree.texts());
    }
}
