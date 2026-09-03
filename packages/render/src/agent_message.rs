//! AgentMessage — agent prose.
//!
//! Contract: `docs/contracts/components/agent-message.md`
//! Ported from: `packages/jetstream/components/src/agent_message.rs`.
//!
//! Markdown parses through `poodle-markdown` into the same block model the web
//! target renders, so both show the same document.
//!
//! No `on_link_click` yet: inline nodes flatten to text here, so there is no
//! link element to hang one on. It arrives with rich inline runs, not before —
//! a handler for a link that is not drawn is exactly the dead-handler pattern
//! this shape exists to prevent.

use poodle_markdown::{parse_markdown, MdBlock, MdInline};
use poodle_node::{ColorValue, LayoutDirection, LayoutSizing, Node};
use poodle_specs::{AgentMessageSpec, PaddingScale, SurfaceBorder, SurfaceSpec};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Flatten inline nodes to text.
///
/// This tier has no rich inline runs here, so emphasis and links resolve to
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

#[derive(Clone, Copy)]
struct Style {
    font_size: f32,
    block_gap: f32,
    indent: f32,
    text: ColorValue,
    quote_text: ColorValue,
    quote_rule: ColorValue,
    code_fill: ColorValue,
}

fn render_blocks(blocks: &[MdBlock], s: &Style) -> Node {
    let mut body = Node::container();
    body.style.descriptor.layout.direction = LayoutDirection::Column;
    body.style.fill_width = true;
    body.style.descriptor.layout.spacing.gap = s.block_gap;

    for block in blocks {
        match block {
            MdBlock::Paragraph(children) => {
                let mut p = Node::text(line(children));
                p.style.text_size = Some(s.font_size);
                p.style.descriptor.text_color = Some(s.text);
                body = body.child(p);
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
                let mut h = Node::text(line(children));
                h.style.text_size = Some(s.font_size * scale);
                h.style.text_weight = Some(600);
                h.style.descriptor.text_color = Some(s.text);
                body = body.child(h);
            }
            MdBlock::Code { value, .. } => {
                let mut frame = Node::container();
                {
                    let st = &mut frame.style;
                    // Explicit Row (see switch.rs).
                    st.descriptor.layout.direction = LayoutDirection::Row;
                    st.fill_width = true;
                    let pad = &mut st.descriptor.layout.spacing.padding;
                    pad.left = rem_to_px(0.375);
                    pad.right = rem_to_px(0.375);
                    pad.top = rem_to_px(0.375);
                    pad.bottom = rem_to_px(0.375);
                    st.descriptor.background = Some(s.code_fill);
                }
                let mut code = Node::text(value.clone());
                code.style.text_size = Some(s.font_size);
                code.style.descriptor.text_color = Some(s.text);
                body = body.child(frame.child(code));
            }
            MdBlock::List {
                ordered,
                start,
                items,
            } => {
                let mut list = Node::container();
                {
                    let st = &mut list.style;
                    st.descriptor.layout.direction = LayoutDirection::Column;
                    st.fill_width = true;
                    st.descriptor.layout.spacing.padding.left = s.indent;
                    st.descriptor.layout.spacing.gap = rem_to_px(0.125);
                }
                for (index, item) in items.iter().enumerate() {
                    let marker = if *ordered {
                        format!("{}.", *start as usize + index)
                    } else {
                        "•".to_string()
                    };
                    let mut m = Node::text(marker);
                    m.style.text_size = Some(s.font_size);
                    m.style.descriptor.text_color = Some(s.quote_text);
                    m.style.flex_shrink_zero = true;

                    let mut row = Node::container();
                    row.style.descriptor.layout.direction = LayoutDirection::Row;
                    row.style.fill_width = true;
                    row.style.descriptor.layout.spacing.gap = rem_to_px(0.375);
                    list = list.child(row.child(m).child(render_blocks(item, s)));
                }
                body = body.child(list);
            }
            MdBlock::Blockquote(children) => {
                let quoted = Style {
                    text: s.quote_text,
                    ..*s
                };
                let mut quote = Node::container();
                {
                    let st = &mut quote.style;
                    // Explicit Row (see switch.rs).
                    st.descriptor.layout.direction = LayoutDirection::Row;
                    st.fill_width = true;
                    st.descriptor.layout.spacing.padding.left = s.indent;
                    st.border_left_width = Some(1.0);
                    st.descriptor.border.color = s.quote_rule;
                }
                body = body.child(quote.child(render_blocks(children, &quoted)));
            }
            MdBlock::Rule => {
                let mut rule = Node::container();
                {
                    let st = &mut rule.style;
                    // Explicit Row (see switch.rs).
                    st.descriptor.layout.direction = LayoutDirection::Row;
                    st.fill_width = true;
                    st.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.0625));
                    st.descriptor.background = Some(s.quote_rule);
                }
                body = body.child(rule);
            }
        }
    }

    body
}

pub fn agent_message(spec: &AgentMessageSpec, ctx: &RenderContext<'_>) -> Node {
    // An empty message contributes no box: a turn with nothing in it should not
    // reserve space in the transcript.
    if !spec.renders() {
        let mut empty = Node::container();
        // Explicit Row (see switch.rs) — the old tier returns a bare div.
        empty.style.descriptor.layout.direction = LayoutDirection::Row;
        return empty;
    }

    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let style = Style {
        font_size: rem_to_px(spec.font_size_rem(base_size)),
        block_gap: rem_to_px(spec.block_gap_rem(density)),
        indent: rem_to_px(spec.list_indent_rem(density)),
        text: ctx.theme().resolve_color(spec.text_token()),
        quote_text: ctx.theme().resolve_color(spec.quote_text_token()),
        quote_rule: ctx.theme().resolve_color(spec.quote_rule_token()),
        code_fill: ctx.theme().resolve_color(spec.code_span_fill_token()),
    };

    let blocks = parse_markdown(&spec.markdown);
    let mut root = render_blocks(&blocks, &style);
    root.roles
        .insert("role".to_owned(), spec.role.as_str().to_owned());
    root.roles.insert(
        "streaming".to_owned(),
        spec.is_streaming.to_string(),
    );
    root.roles.insert(
        "size".to_owned(),
        format!("{base_size:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "density".to_owned(),
        format!("{density:?}").to_ascii_lowercase(),
    );

    if spec.is_user() {
        let surface = ctx.theme().resolve_color(spec.user_surface_token());
        let radius = ctx.theme().resolve_radius(spec.radius_token());
        // AgentMessage owns an elevated fill without elevation. Start from the
        // production Surface shell that carries no shadow, then apply the
        // message contract's exact fill, radius, and inset.
        let surface_spec = SurfaceSpec::new()
            .with_border(SurfaceBorder::None)
            .with_padding(PaddingScale::None);
        let mut bubble = crate::surface::surface(&surface_spec, ctx, vec![root]);
        {
            let s = &mut bubble.style;
            s.fill_width = true;
            let pad = &mut s.descriptor.layout.spacing.padding;
            let inset = rem_to_px(spec.padding_inset_rem(density));
            pad.left = inset;
            pad.right = inset;
            pad.top = inset;
            pad.bottom = inset;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
            s.descriptor.background = Some(surface);
        }
        bubble
            .roles
            .insert("role".to_owned(), spec.role.as_str().to_owned());
        bubble.roles.insert(
            "streaming".to_owned(),
            spec.is_streaming.to_string(),
        );
        bubble.roles.insert(
            "size".to_owned(),
            format!("{base_size:?}").to_ascii_lowercase(),
        );
        bubble.roles.insert(
            "density".to_owned(),
            format!("{density:?}").to_ascii_lowercase(),
        );
        root = bubble;
    }

    root
}
