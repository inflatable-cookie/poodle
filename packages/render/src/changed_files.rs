//! ChangedFiles — a collapsed card summarising a turn's file changes.
//!
//! Contract: `docs/contracts/components/changed-files.md`
//! Ported from: `packages/jetstream/components/src/changed_files.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_headless::agent_transcript::ChangedFileNode;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole, StylePatch,
};
use poodle_specs::ChangedFilesSpec;

use crate::color::TRANSPARENT;
use crate::presentation::rem_to_px;

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct ChangedFilesHandlers {
    /// Fires with the card id when it is opened or closed.
    pub on_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with a file's path when one is chosen (tree row or chip).
    pub on_file_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn changed_files(
    spec: &ChangedFilesSpec,
    theme: &dyn ThemeProvider,
    handlers: ChangedFilesHandlers,
) -> Node {
    // An empty card renders nothing rather than an empty state. A turn that
    // changed no files should not have a box saying so.
    if !spec.renders() {
        let mut empty = Node::container();
        // Explicit Row (see switch.rs) — the old tier returns a bare div.
        empty.style.descriptor.layout.direction = LayoutDirection::Row;
        return empty;
    }

    let surface = theme.resolve_color(spec.surface_token());
    let border = theme.resolve_color(spec.border_token());
    let count_color = theme.resolve_color(spec.count_token());
    let additions = theme.resolve_color(spec.additions_token());
    let deletions = theme.resolve_color(spec.deletions_token());
    let scope_color = theme.resolve_color(spec.scope_token());
    let chip_fill = theme.resolve_color(spec.chip_fill_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let chip_radius = theme.resolve_radius(spec.chip_radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let icon_size = rem_to_px(spec.icon_size_rem());
    let inset = rem_to_px(spec.padding_inset_rem());
    let gap = rem_to_px(spec.gap_rem());
    // Contract §8: a hairline, stated as an absolute because no border-width
    // token is finer than 1px.
    let hairline = rem_to_px(0.0625);

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };
    let text = |content: String, color, size, weight: Option<u16>| -> Node {
        let mut t = Node::text(content);
        t.style.descriptor.text_color = Some(color);
        t.style.text_size = Some(size);
        t.style.text_weight = weight;
        t
    };

    let totals = spec.totals();

    let mut header = Node::button("");
    header.id = Some(format!("changed-files-toggle-{}", spec.id));
    // Counts are colour-coded, and colour alone is not a signal.
    header.a11y.label = Some(spec.accessible_name());
    header.a11y.role = Some(NodeRole::Button);
    header.a11y.expanded = Some(spec.is_expanded);
    {
        let s = &mut header.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = inset;
        pad.right = inset;
        pad.top = inset;
        pad.bottom = inset;
        s.descriptor.background = Some(TRANSPARENT);
    }
    header.interaction.focusable = true;
    header.style.focus = Some(StylePatch {
        background: None,
        border_color: Some(theme.resolve_color("color.accent.focusRing")),
        text_color: None,
        opacity: None,
    });

    let mut chevron = Node::icon(
        if spec.is_expanded {
            "chevron-down"
        } else {
            "chevron-right"
        },
        icon_size,
    );
    chevron.style.descriptor.text_color = Some(scope_color);
    let mut header = header
        .child(chevron)
        .child(text(
            spec.resolved_count_label(),
            count_color,
            font_size,
            Some(600),
        ))
        .child(text(
            format!("+{}", totals.additions),
            additions,
            font_size,
            None,
        ))
        .child(text(
            format!("−{}", totals.deletions),
            deletions,
            font_size,
            None,
        ));

    if let Some(handler) = &handlers.on_toggle {
        let handler = Arc::clone(handler);
        let id = spec.id.clone();
        header.style.descriptor.cursor = CursorHint::Pointer;
        header.interaction.on_activate = Some(Arc::new(move || handler(&id)));
    }

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.fill_width = true;
        s.descriptor.border.width = hairline;
        s.descriptor.border.color = border;
        s.descriptor.background = Some(surface);
    }
    all_radius(&mut root, radius);
    let mut root = root.child(header);

    if spec.is_expanded {
        // Rows are indented by depth rather than nested, because the fold has
        // already collapsed the chains that would have needed nesting to read.
        fn flatten(
            nodes: &[ChangedFileNode],
            depth: usize,
            out: &mut Vec<(usize, ChangedFileNode)>,
        ) {
            for node in nodes {
                out.push((depth, node.clone()));
                flatten(&node.children, depth + 1, out);
            }
        }

        let mut flat = Vec::new();
        flatten(&spec.tree(), 0, &mut flat);

        let mut tree = Node::container();
        tree.a11y.role = Some(NodeRole::Tree);
        {
            let s = &mut tree.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.fill_width = true;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = inset;
            pad.right = inset;
            pad.bottom = inset;
        }

        for (depth, node) in flat {
            let mut row = Node::container();
            row.id = Some(format!(
                "changed-files-file-{}-{}",
                spec.id,
                node.path.replace('/', ":")
            ));
            row.a11y.role = Some(NodeRole::TreeItem);
            {
                let s = &mut row.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.fill_width = true;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = gap;
                s.descriptor.layout.spacing.padding.left = rem_to_px(0.75) * depth as f32;
            }

            let mut glyph =
                Node::icon(if node.is_directory { "folder" } else { "file" }, icon_size);
            glyph.style.descriptor.text_color = Some(scope_color);

            let mut label = text(node.label.clone(), count_color, font_size, None);
            label.style.descriptor.layout.width = LayoutSizing::Grow;
            label.style.min_width = Some(0.0);

            let mut row = row
                .child(glyph)
                .child(label)
                .child(text(
                    format!("+{}", node.additions),
                    additions,
                    font_size,
                    None,
                ))
                .child(text(
                    format!("−{}", node.deletions),
                    deletions,
                    font_size,
                    None,
                ));

            // Only files select. A directory row's job is to expand, and
            // handing a host a directory as a "file chosen" is a lie it would
            // have to filter out itself.
            if let (false, Some(handler)) = (node.is_directory, &handlers.on_file_select) {
                let handler = Arc::clone(handler);
                let path = node.path.clone();
                row.style.descriptor.cursor = CursorHint::Pointer;
                row.interaction.focusable = true;
                row.style.focus = Some(StylePatch {
                    background: None,
                    border_color: Some(theme.resolve_color("color.accent.focusRing")),
                    text_color: None,
                    opacity: None,
                });
                row.interaction.on_activate = Some(Arc::new(move || handler(&path)));
            }

            tree = tree.child(row);
        }
        root = root.child(tree);
    } else {
        let mut summary = Node::container();
        {
            let s = &mut summary.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.fill_width = true;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = gap;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = inset;
            pad.right = inset;
            pad.bottom = inset;
        }

        for (name, count) in spec.scopes() {
            let word = if count == 1 { "file" } else { "files" };
            summary = summary.child(text(
                format!("{name} {count} {word}"),
                scope_color,
                font_size,
                None,
            ));
        }

        for file in spec.visible_chips() {
            let leaf = file
                .path
                .rsplit('/')
                .next()
                .unwrap_or(&file.path)
                .to_string();
            let mut chip = Node::container();
            chip.id = Some(format!(
                "changed-files-chip-{}-{}",
                spec.id,
                file.path.replace('/', ":")
            ));
            {
                let s = &mut chip.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = rem_to_px(0.25);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.375);
                pad.right = rem_to_px(0.375);
                pad.top = rem_to_px(0.125);
                pad.bottom = rem_to_px(0.125);
                s.descriptor.border.width = hairline;
                s.descriptor.border.color = border;
                s.descriptor.background = Some(chip_fill);
            }
            all_radius(&mut chip, chip_radius);

            let mut glyph = Node::icon("file", icon_size);
            glyph.style.descriptor.text_color = Some(scope_color);
            let mut chip = chip
                .child(glyph)
                .child(text(leaf, scope_color, font_size, None));

            // A chip shows the leaf but reports the full path, same as a tree
            // row: the two are the same event seen from two states.
            if let Some(handler) = &handlers.on_file_select {
                let handler = Arc::clone(handler);
                let path = file.path.clone();
                chip.style.descriptor.cursor = CursorHint::Pointer;
                chip.interaction.focusable = true;
                chip.style.focus = Some(StylePatch {
                    background: None,
                    border_color: Some(theme.resolve_color("color.accent.focusRing")),
                    text_color: None,
                    opacity: None,
                });
                chip.interaction.on_activate = Some(Arc::new(move || handler(&path)));
            }

            summary = summary.child(chip);
        }

        root = root.child(summary);
    }

    root
}
