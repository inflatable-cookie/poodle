//! AgentMessage — agent prose, backed by `AgentMessageSpec`.
//!
//! Contract: `docs/contracts/components/agent-message.md`.
//!
//! Markdown is parsed by `poodle-markdown` into the block model the web target
//! also renders, so both show the same document.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_markdown::{parse_markdown, MdBlock, MdInline};
use poodle_specs::AgentMessageSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct AgentMessage {
    spec: AgentMessageSpec,
    theme: GpuiThemeProvider,
}

impl AgentMessage {
    pub fn from_spec(spec: AgentMessageSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

/// Flatten inline nodes to text.
///
/// GPUI has no rich inline runs here, so emphasis and links resolve to their
/// text rather than being dropped — losing content is the worst available
/// failure for a transcript, and this is recorded as a delta on the contract.
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

impl IntoElement for AgentMessage {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // An empty message contributes no box: a turn with nothing in it should
        // not reserve space in the transcript.
        if !spec.renders() {
            return div().into_any_element();
        }

        let text = resolve_color(theme, spec.text_token());
        let quote_text = resolve_color(theme, spec.quote_text_token());
        let quote_rule = resolve_color(theme, spec.quote_rule_token());
        let code_fill = resolve_color(theme, spec.code_span_fill_token());
        let surface = resolve_color(theme, spec.user_surface_token());
        let radius = resolve_radius(theme, spec.radius_token());

        let font_size = px(rem_to_px(spec.font_size_rem()));
        let block_gap = px(rem_to_px(spec.block_gap_rem()));
        let indent = px(rem_to_px(spec.list_indent_rem()));
        let inset = px(rem_to_px(spec.padding_inset_rem()));
        let measure = px(rem_to_px(spec.measure_rem()));

        fn render_blocks(
            blocks: &[MdBlock],
            font_size: Pixels,
            block_gap: Pixels,
            indent: Pixels,
            text: Hsla,
            quote_text: Hsla,
            quote_rule: Hsla,
            code_fill: Hsla,
        ) -> Div {
            let mut body = div().flex().flex_col().gap(block_gap);

            for block in blocks {
                match block {
                    MdBlock::Paragraph(children) => {
                        body = body.child(div().text_size(font_size).text_color(text).child(line(children)));
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
                            div()
                                .text_size(px(f32::from(font_size) * scale))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(text)
                                .child(line(children)),
                        );
                    }
                    MdBlock::Code { value, .. } => {
                        body = body.child(
                            div()
                                .p(px(6.0))
                                .bg(code_fill)
                                .text_size(font_size)
                                .text_color(text)
                                .child(value.clone()),
                        );
                    }
                    MdBlock::List { ordered, start, items } => {
                        let mut list = div().flex().flex_col().gap(px(2.0)).pl(indent);
                        for (index, item) in items.iter().enumerate() {
                            let marker = if *ordered {
                                format!("{}.", *start as usize + index)
                            } else {
                                "•".to_string()
                            };
                            list = list.child(
                                div()
                                    .flex()
                                    .gap(px(6.0))
                                    .child(div().text_size(font_size).text_color(quote_text).child(marker))
                                    .child(render_blocks(
                                        item, font_size, block_gap, indent, text, quote_text,
                                        quote_rule, code_fill,
                                    )),
                            );
                        }
                        body = body.child(list);
                    }
                    MdBlock::Blockquote(children) => {
                        body = body.child(
                            div()
                                .pl(indent)
                                .border_l_2()
                                .border_color(quote_rule)
                                .text_color(quote_text)
                                .child(render_blocks(
                                    children, font_size, block_gap, indent, quote_text, quote_text,
                                    quote_rule, code_fill,
                                )),
                        );
                    }
                    MdBlock::Rule => {
                        body = body.child(div().h(px(1.0)).w_full().bg(quote_rule));
                    }
                }
            }

            body
        }

        let blocks = parse_markdown(&spec.markdown);
        let body = render_blocks(
            &blocks, font_size, block_gap, indent, text, quote_text, quote_rule, code_fill,
        );

        let mut root = div().flex().flex_col().max_w(measure).child(body);

        if spec.is_user() {
            root = root.p(inset).rounded(radius).bg(surface);
        }

        root.into_any_element()
    }
}
