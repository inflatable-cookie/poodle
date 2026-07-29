//! The markdown block model shared by every Poodle target — Rust side.
//!
//! Contract: `docs/contracts/components/agent-message.md`, "Markdown Subset".
//!
//! Agent output is markdown, and Poodle renders it on four targets that cannot
//! share a parser: the web has `marked`, the natives have `pulldown-cmark`.
//! Left alone the two would disagree — on tight vs loose lists, on soft breaks,
//! on what an unannotated fence means — and the disagreement would surface as
//! "the desktop build renders this answer differently", months later, with no
//! obvious cause.
//!
//! So neither parser's output is the model. Both normalise into the blocks
//! below, and `packages/contracts/headless/vectors/markdown-blocks.json` pins
//! what each source must produce in both languages. The vectors are generated
//! from the web target, which is the parity authority.
//!
//! Mirror of core `markdown-blocks.ts`.

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MdInline {
    Text(String),
    Code(String),
    Strong(Vec<MdInline>),
    Em(Vec<MdInline>),
    Del(Vec<MdInline>),
    Link { href: String, children: Vec<MdInline> },
    Break,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MdBlock {
    Paragraph(Vec<MdInline>),
    Heading { level: u8, children: Vec<MdInline> },
    Code { lang: Option<String>, value: String },
    List { ordered: bool, start: u64, items: Vec<Vec<MdBlock>> },
    Blockquote(Vec<MdBlock>),
    Rule,
}

/// Where the walker is currently appending. Blocks and inlines are collected
/// separately because a paragraph can close while a list is still open.
#[derive(Debug)]
enum Frame {
    Blocks(Vec<MdBlock>),
    Inlines(Vec<MdInline>),
}

struct Walker {
    stack: Vec<Frame>,
    /// (ordered, start) for each open list, innermost last.
    lists: Vec<(bool, u64)>,
    /// Item block-lists for each open list, innermost last.
    list_items: Vec<Vec<Vec<MdBlock>>>,
    /// Pending code fence language, held between Start and End.
    code_lang: Vec<Option<String>>,
    heading_level: Vec<u8>,
    link_href: Vec<String>,
}

impl Walker {
    fn new() -> Self {
        Self {
            stack: vec![Frame::Blocks(Vec::new())],
            lists: Vec::new(),
            list_items: Vec::new(),
            code_lang: Vec::new(),
            heading_level: Vec::new(),
            link_href: Vec::new(),
        }
    }

    fn push_block(&mut self, block: MdBlock) {
        for frame in self.stack.iter_mut().rev() {
            if let Frame::Blocks(blocks) = frame {
                blocks.push(block);
                return;
            }
        }
    }

    fn push_inline(&mut self, inline: MdInline) {
        if let Some(Frame::Inlines(inlines)) = self.stack.last_mut() {
            // Contiguous text merges into one node. `pulldown-cmark` splits text
            // on entity and escape boundaries where `marked` does not, and an
            // unmerged split would be a structural difference the vectors catch
            // even though the rendered result is identical.
            if let (MdInline::Text(next), Some(MdInline::Text(prev))) =
                (&inline, inlines.last_mut())
            {
                prev.push_str(next);
                return;
            }
            inlines.push(inline);
        }
    }

    fn open_inlines(&mut self) {
        self.stack.push(Frame::Inlines(Vec::new()));
    }

    fn close_inlines(&mut self) -> Vec<MdInline> {
        match self.stack.pop() {
            Some(Frame::Inlines(inlines)) => inlines,
            Some(other) => {
                self.stack.push(other);
                Vec::new()
            }
            None => Vec::new(),
        }
    }

    fn open_blocks(&mut self) {
        self.stack.push(Frame::Blocks(Vec::new()));
    }

    fn close_blocks(&mut self) -> Vec<MdBlock> {
        match self.stack.pop() {
            Some(Frame::Blocks(blocks)) => blocks,
            Some(other) => {
                self.stack.push(other);
                Vec::new()
            }
            None => Vec::new(),
        }
    }

    /// Close a bare inline run and file it as a paragraph.
    ///
    /// A tight list item emits no paragraph events, so its text arrives with no
    /// inline frame open and one gets opened on demand. That frame has to be
    /// closed before anything block-level starts inside the same item, or a
    /// nested list would be filed ahead of the text introducing it.
    fn flush_pending_inlines(&mut self) {
        if matches!(self.stack.last(), Some(Frame::Inlines(_))) {
            let children = self.close_inlines();
            if !children.is_empty() {
                self.push_block(MdBlock::Paragraph(children));
            }
        }
    }
}

fn heading_level_number(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Parse markdown into the shared block model.
pub fn parse_markdown(source: &str) -> Vec<MdBlock> {
    let mut options = Options::empty();
    // `marked` has GFM on by default, so strikethrough must be enabled here or
    // `~~gone~~` would arrive as literal tildes on the natives only.
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let mut walker = Walker::new();

    for event in Parser::new_ext(source, options) {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => walker.open_inlines(),
                Tag::Heading { level, .. } => {
                    walker.heading_level.push(heading_level_number(level));
                    walker.open_inlines();
                }
                Tag::CodeBlock(kind) => {
                    walker.flush_pending_inlines();
                    let lang = match kind {
                        CodeBlockKind::Fenced(info) => {
                            // Only the language word; `marked` drops the rest of
                            // the info string, and an unannotated fence is
                            // `None` rather than `Some("")` so a renderer can
                            // tell "no language" from "empty language".
                            info.split_whitespace().next().map(str::to_string)
                        }
                        CodeBlockKind::Indented => None,
                    };
                    walker.code_lang.push(lang.filter(|s| !s.is_empty()));
                    walker.open_inlines();
                }
                Tag::List(start) => {
                    walker.flush_pending_inlines();
                    walker.lists.push((start.is_some(), start.unwrap_or(1)));
                    walker.list_items.push(Vec::new());
                }
                Tag::Item => walker.open_blocks(),
                Tag::BlockQuote(_) => {
                    walker.flush_pending_inlines();
                    walker.open_blocks();
                }
                Tag::Strong | Tag::Emphasis | Tag::Strikethrough => walker.open_inlines(),
                Tag::Link { dest_url, .. } => {
                    walker.link_href.push(dest_url.to_string());
                    walker.open_inlines();
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Paragraph => {
                    let children = walker.close_inlines();
                    walker.push_block(MdBlock::Paragraph(children));
                }
                TagEnd::Heading(_) => {
                    let children = walker.close_inlines();
                    let level = walker.heading_level.pop().unwrap_or(1);
                    walker.push_block(MdBlock::Heading { level, children });
                }
                TagEnd::CodeBlock => {
                    let inlines = walker.close_inlines();
                    let mut value = String::new();
                    for inline in inlines {
                        if let MdInline::Text(text) = inline {
                            value.push_str(&text);
                        }
                    }
                    // `marked` reports fence content without its trailing
                    // newline; `pulldown-cmark` keeps it.
                    while value.ends_with('\n') {
                        value.pop();
                    }
                    let lang = walker.code_lang.pop().flatten();
                    walker.push_block(MdBlock::Code { lang, value });
                }
                TagEnd::Item => {
                    // A tight list emits no paragraph events, so item content
                    // arrives as bare inlines. The web model always wraps item
                    // content in blocks, and normalising here is what removes
                    // tight-vs-loose as a source of divergence.
                    //
                    // Order matters: the inline run closes first so it lands in
                    // the item's own block frame. Closing the blocks first finds
                    // the inline frame on top, leaves the block frame orphaned
                    // on the stack, and the enclosing list then files itself
                    // into the orphan instead of the document.
                    walker.flush_pending_inlines();
                    let blocks = walker.close_blocks();
                    if let Some(items) = walker.list_items.last_mut() {
                        items.push(blocks);
                    }
                }
                TagEnd::List(_) => {
                    let items = walker.list_items.pop().unwrap_or_default();
                    let (ordered, start) = walker.lists.pop().unwrap_or((false, 1));
                    walker.push_block(MdBlock::List { ordered, start, items });
                }
                TagEnd::BlockQuote(_) => {
                    let blocks = walker.close_blocks();
                    walker.push_block(MdBlock::Blockquote(blocks));
                }
                TagEnd::Strong => {
                    let children = walker.close_inlines();
                    walker.push_inline(MdInline::Strong(children));
                }
                TagEnd::Emphasis => {
                    let children = walker.close_inlines();
                    walker.push_inline(MdInline::Em(children));
                }
                TagEnd::Strikethrough => {
                    let children = walker.close_inlines();
                    walker.push_inline(MdInline::Del(children));
                }
                TagEnd::Link => {
                    let children = walker.close_inlines();
                    let href = walker.link_href.pop().unwrap_or_default();
                    walker.push_inline(MdInline::Link { href, children });
                }
                _ => {}
            },
            Event::Text(text) => {
                if matches!(walker.stack.last(), Some(Frame::Inlines(_))) {
                    walker.push_inline(MdInline::Text(text.to_string()));
                } else {
                    // Text outside any paragraph — a tight list item. Open an
                    // inline frame so `TagEnd::Item` can wrap it.
                    walker.open_inlines();
                    walker.push_inline(MdInline::Text(text.to_string()));
                }
            }
            Event::Code(code) => {
                if !matches!(walker.stack.last(), Some(Frame::Inlines(_))) {
                    walker.open_inlines();
                }
                walker.push_inline(MdInline::Code(code.to_string()));
            }
            // A soft break is a source line wrap, which `marked` keeps as a
            // newline inside the text rather than as a node.
            Event::SoftBreak => walker.push_inline(MdInline::Text("\n".to_string())),
            Event::HardBreak => walker.push_inline(MdInline::Break),
            Event::Rule => {
                walker.flush_pending_inlines();
                walker.push_block(MdBlock::Rule);
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                // Degrade to text rather than dropping it. An agent explaining
                // HTML must not have the explanation disappear because the
                // parser classified part of it as raw HTML.
                let trimmed = html.trim().to_string();
                if !trimmed.is_empty() {
                    if matches!(walker.stack.last(), Some(Frame::Inlines(_))) {
                        walker.push_inline(MdInline::Text(trimmed));
                    } else {
                        walker.push_block(MdBlock::Paragraph(vec![MdInline::Text(trimmed)]));
                    }
                }
            }
            _ => {}
        }
    }

    match walker.stack.into_iter().next() {
        Some(Frame::Blocks(blocks)) => blocks,
        _ => Vec::new(),
    }
}

/// Plain text of a block tree — for accessible names, copy, and measurement.
///
/// Code blocks contribute their source: a screen reader announcing a message
/// needs to know code was there, and skipping it would make an answer that is
/// mostly a snippet read as almost empty.
pub fn markdown_plain_text(blocks: &[MdBlock]) -> String {
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

    fn block_text(blocks: &[MdBlock], lines: &mut Vec<String>) {
        for block in blocks {
            match block {
                MdBlock::Paragraph(children) | MdBlock::Heading { children, .. } => {
                    let mut line = String::new();
                    inline_text(children, &mut line);
                    lines.push(line);
                }
                MdBlock::Code { value, .. } => lines.push(value.clone()),
                MdBlock::Blockquote(children) => block_text(children, lines),
                MdBlock::List { items, .. } => {
                    for item in items {
                        block_text(item, lines);
                    }
                }
                MdBlock::Rule => {}
            }
        }
    }

    let mut lines = Vec::new();
    block_text(blocks, &mut lines);
    lines.retain(|line| !line.is_empty());
    lines.join("\n")
}
