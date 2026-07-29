//! Cross-runtime conformance for the markdown block model: runs the shared
//! vectors in `packages/contracts/headless/vectors/markdown-blocks.json`
//! against `pulldown-cmark`. The TypeScript core runs the same vectors against
//! `marked` (packages/core/test/markdown-blocks.test.ts).
//!
//! The vectors are generated from the web target, which is the parity
//! authority. A failure here means the natives would render a document
//! differently from the web — the exact drift this crate exists to prevent.

use serde_json::{json, Value};

use poodle_markdown::{markdown_plain_text, parse_markdown, MdBlock, MdInline};

fn vectors() -> Value {
    let raw = include_str!("../../headless/vectors/markdown-blocks.json");
    serde_json::from_str(raw).expect("vectors parse")
}

/// Serialise to the vector JSON shape so the comparison is order-sensitive and
/// reports a readable diff.
fn inline_value(node: &MdInline) -> Value {
    match node {
        MdInline::Text(value) => json!({ "type": "text", "value": value }),
        MdInline::Code(value) => json!({ "type": "code", "value": value }),
        MdInline::Strong(children) => json!({ "type": "strong", "children": inlines_value(children) }),
        MdInline::Em(children) => json!({ "type": "em", "children": inlines_value(children) }),
        MdInline::Del(children) => json!({ "type": "del", "children": inlines_value(children) }),
        MdInline::Link { href, children } => {
            json!({ "type": "link", "href": href, "children": inlines_value(children) })
        }
        MdInline::Break => json!({ "type": "break" }),
    }
}

fn inlines_value(nodes: &[MdInline]) -> Value {
    Value::Array(nodes.iter().map(inline_value).collect())
}

fn block_value(block: &MdBlock) -> Value {
    match block {
        MdBlock::Paragraph(children) => {
            json!({ "type": "paragraph", "children": inlines_value(children) })
        }
        MdBlock::Heading { level, children } => {
            json!({ "type": "heading", "level": level, "children": inlines_value(children) })
        }
        MdBlock::Code { lang, value } => json!({
            "type": "code",
            "lang": match lang { Some(l) => Value::String(l.clone()), None => Value::Null },
            "value": value,
        }),
        MdBlock::List { ordered, start, items } => json!({
            "type": "list",
            "ordered": ordered,
            "start": start,
            "items": Value::Array(items.iter().map(|item| blocks_value(item)).collect()),
        }),
        MdBlock::Blockquote(children) => {
            json!({ "type": "blockquote", "children": blocks_value(children) })
        }
        MdBlock::Rule => json!({ "type": "rule" }),
    }
}

fn blocks_value(blocks: &[MdBlock]) -> Value {
    Value::Array(blocks.iter().map(block_value).collect())
}

#[test]
fn blocks_match_the_shared_vectors() {
    let vectors = vectors();
    let cases = vectors.as_array().expect("vector array");

    assert!(!cases.is_empty(), "vectors file is empty");

    for case in cases {
        let name = case["name"].as_str().unwrap_or("");
        let markdown = case["markdown"].as_str().unwrap_or("");
        let want = &case["blocks"];

        let got = blocks_value(&parse_markdown(markdown));

        assert_eq!(
            got,
            *want,
            "{name}: pulldown-cmark diverged from the web target\n  source: {markdown:?}\n  got:  {got}\n  want: {want}"
        );
    }
}

#[test]
fn plain_text_includes_code() {
    let blocks = parse_markdown("Try this:\n\n```sh\nbun test\n```");
    let text = markdown_plain_text(&blocks);

    assert!(text.contains("Try this:"), "{text:?}");
    assert!(text.contains("bun test"), "{text:?}");
}

#[test]
fn unknown_html_degrades_to_text_rather_than_vanishing() {
    let blocks = parse_markdown("<div>hello</div>");

    assert!(!blocks.is_empty(), "html block dropped entirely");
    assert!(markdown_plain_text(&blocks).contains("hello"));
}
