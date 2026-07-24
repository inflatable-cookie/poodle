//! GPU-free render probe for Jetstream components.
//!
//! Builds a component's `JsEl`, runs the *real* `GameUi` immediate-mode layout
//! (Taffy, CPU-only — no window, no GPU device), then walks the resulting
//! `UiTree` into a flat, inspectable, serializable structure.
//!
//! This is the Jetstream side of the cross-target parity harness: it lets tests
//! assert on actual rendered output (node count, anatomy, computed sizes/positions,
//! resolved background colors, text) instead of only checking that the crate
//! compiles. Because layout is CPU-only, no display or GPU is required.
//!
//! Text widths come from `Widget::intrinsic_size`'s character estimate when no
//! font atlas is present, so absolute text widths are approximate; structure,
//! colors, token keys, and non-text dimensions are exact.

use jetstream_ui::{GameUi, Widget};
use jetstream_ui::ui_element::JsEl;

/// An RGBA color sampled from a node's resolved style, components in `0.0..=1.0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProbeColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ProbeColor {
    /// True when every channel is within `eps` of `other` (tolerant compare for
    /// token-resolved colors that round-trip through `f32`).
    pub fn approx(&self, other: ProbeColor, eps: f32) -> bool {
        (self.r - other.r).abs() <= eps
            && (self.g - other.g).abs() <= eps
            && (self.b - other.b).abs() <= eps
            && (self.a - other.a).abs() <= eps
    }
}

/// One laid-out node, flattened depth-first from the tree root.
#[derive(Clone, Debug)]
pub struct ProbeNode {
    /// Depth from root (root = 0).
    pub depth: usize,
    /// Widget variant name (`Panel`, `Label`, `Icon`, `TextInput`, …).
    pub kind: &'static str,
    /// Interaction id assigned via `.id(...)`, if any (the hit-test/action key).
    pub token_key: Option<String>,
    /// Text content for text-bearing widgets (label / button / icon name / input).
    pub text: Option<String>,
    /// Computed layout rect (logical px).
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    /// Resolved flat background fill, if set.
    pub bg: Option<ProbeColor>,
    /// Resolved text size, if set.
    pub text_size: Option<f32>,
}

/// A flattened render of a component, root first, depth-first.
#[derive(Clone, Debug)]
pub struct ProbeTree {
    pub nodes: Vec<ProbeNode>,
}

/// Render `el` at the given logical viewport and flatten the resulting tree.
pub fn probe(el: &JsEl, width: f32, height: f32) -> ProbeTree {
    let mut ui = GameUi::new(width, height);
    ui.set_scale_factor(1.0);
    ui.active = true;
    ui.render_immediate(el);

    let mut nodes = Vec::new();
    if let Some(root) = ui.tree.root() {
        let mut stack = vec![(root, 0usize)];
        // Iterative DFS preserving child order (push reversed so first child pops first).
        while let Some((id, depth)) = stack.pop() {
            if let Some(node) = ui.tree.get(id) {
                let (kind, text) = widget_info(&node.widget);
                let r = node.computed_rect;
                nodes.push(ProbeNode {
                    depth,
                    kind,
                    token_key: node.style.token_key.clone(),
                    text,
                    x: r.x,
                    y: r.y,
                    w: r.width,
                    h: r.height,
                    bg: node.style.background.map(|c| ProbeColor {
                        r: c.r,
                        g: c.g,
                        b: c.b,
                        a: c.a,
                    }),
                    text_size: node.style.text_size,
                });
                let children = ui.tree.children(id);
                for child in children.into_iter().rev() {
                    stack.push((child, depth + 1));
                }
            }
        }
        // DFS-with-stack visits root then reverses sibling order back to correct via the
        // double-reverse above; re-sort is unnecessary — order is root-first preorder.
    }
    ProbeTree { nodes }
}

fn widget_info(w: &Widget) -> (&'static str, Option<String>) {
    match w {
        Widget::Panel => ("Panel", None),
        Widget::Label { text } => ("Label", Some(text.clone())),
        Widget::Button { label, .. } => ("Button", Some(label.clone())),
        Widget::Slider { .. } => ("Slider", None),
        Widget::ProgressBar { .. } => ("ProgressBar", None),
        Widget::Image { path, .. } => ("Image", Some(path.clone())),
        Widget::Icon { name } => ("Icon", Some(name.clone())),
        Widget::RichText { runs } => (
            "RichText",
            Some(runs.iter().map(|r| r.text.clone()).collect::<String>()),
        ),
        Widget::List { .. } => ("List", None),
        Widget::TextInput {
            text, placeholder, ..
        } => (
            "TextInput",
            Some(if text.is_empty() {
                placeholder.clone()
            } else {
                text.clone()
            }),
        ),
    }
}

impl ProbeTree {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// First node whose interaction id equals `key`.
    pub fn find_token(&self, key: &str) -> Option<&ProbeNode> {
        self.nodes.iter().find(|n| n.token_key.as_deref() == Some(key))
    }

    /// Any node whose interaction id starts with `prefix` (e.g. `"tree:"`).
    pub fn has_token_prefix(&self, prefix: &str) -> bool {
        self.nodes
            .iter()
            .any(|n| n.token_key.as_deref().is_some_and(|k| k.starts_with(prefix)))
    }

    /// Any node rendering exactly this text.
    pub fn has_text(&self, text: &str) -> bool {
        self.nodes.iter().any(|n| n.text.as_deref() == Some(text))
    }

    /// All non-empty text content, in document order.
    pub fn texts(&self) -> Vec<&str> {
        self.nodes.iter().filter_map(|n| n.text.as_deref()).collect()
    }

    /// Count nodes of a given widget kind.
    pub fn count_kind(&self, kind: &str) -> usize {
        self.nodes.iter().filter(|n| n.kind == kind).count()
    }

    /// True if any node has a flat background within `eps` of `color`.
    pub fn has_background(&self, color: ProbeColor, eps: f32) -> bool {
        self.nodes
            .iter()
            .any(|n| n.bg.is_some_and(|b| b.approx(color, eps)))
    }

    /// Minimal JSON dump for cross-target diffing / debugging.
    pub fn to_json(&self) -> String {
        let mut out = String::from("[\n");
        for (i, n) in self.nodes.iter().enumerate() {
            let indent = "  ".repeat(n.depth + 1);
            let token = n
                .token_key
                .as_deref()
                .map(|t| format!("{t:?}"))
                .unwrap_or_else(|| "null".into());
            let text = n
                .text
                .as_deref()
                .map(|t| format!("{t:?}"))
                .unwrap_or_else(|| "null".into());
            let bg = n
                .bg
                .map(|c| format!("[{:.3},{:.3},{:.3},{:.3}]", c.r, c.g, c.b, c.a))
                .unwrap_or_else(|| "null".into());
            out.push_str(&format!(
                "{indent}{{\"d\":{},\"kind\":{:?},\"token\":{token},\"text\":{text},\"rect\":[{:.1},{:.1},{:.1},{:.1}],\"bg\":{bg}}}{}\n",
                n.depth,
                n.kind,
                n.x,
                n.y,
                n.w,
                n.h,
                if i + 1 < self.nodes.len() { "," } else { "" },
            ));
        }
        out.push(']');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::button::js_button;
    use poodle_jetstream::JetstreamThemeProvider;
    use poodle_specs::ButtonSpec;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn probe_renders_button_headless() {
        let th = theme();
        let el = js_button(&ButtonSpec::new().with_label("Save"), &th);
        let tree = probe(&el, 400.0, 200.0);

        // Tree is non-empty and the label text made it through layout.
        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(
            tree.has_text("Save"),
            "button label missing from rendered tree: {:?}",
            tree.texts()
        );
        // Root has positive computed size (layout actually ran).
        let root = &tree.nodes[0];
        assert!(root.w > 0.0 && root.h > 0.0, "root not laid out: {root:?}");
    }
}
