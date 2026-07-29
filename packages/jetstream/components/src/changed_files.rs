//! ChangedFiles — what a turn touched, backed by `ChangedFilesSpec`.
//!
//! Contract: `docs/contracts/components/changed-files.md`.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_headless::agent_transcript::ChangedFileNode;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ChangedFilesSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_changed_files(spec: &ChangedFilesSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // An empty card renders nothing rather than an empty state. A turn that
    // changed no files should not have a box saying so.
    if !spec.renders() {
        return ui_element::div();
    }

    let surface: Color = resolve_color(theme, spec.surface_token()).into();
    let border: Color = resolve_color(theme, spec.border_token()).into();
    let count_color: Color = resolve_color(theme, spec.count_token()).into();
    let additions: Color = resolve_color(theme, spec.additions_token()).into();
    let deletions: Color = resolve_color(theme, spec.deletions_token()).into();
    let scope_color: Color = resolve_color(theme, spec.scope_token()).into();
    let chip_fill: Color = resolve_color(theme, spec.chip_fill_token()).into();
    let radius = resolve_radius(theme, spec.radius_token());
    let chip_radius = resolve_radius(theme, spec.chip_radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let icon_size = rem_to_px(spec.icon_size_rem());
    let inset = rem_to_px(spec.padding_inset_rem());
    let gap = rem_to_px(spec.gap_rem());
    // Contract §8: a hairline, stated as an absolute because no border-width
    // token is finer than 1px.
    let hairline = rem_to_px(0.0625);

    let totals = spec.totals();

    let header = ui_element::button("")
        // Counts are colour-coded, and colour alone is not a signal.
        .aria_label(spec.accessible_name())
        .aria_role(jetstream_ui::accesskit::Role::Button)
        .aria_expanded(spec.is_expanded)
        .flex_row()
        .items_center()
        .gap(gap)
        .p(inset)
        .bg(Color::TRANSPARENT)
        .focusable()
        .child(
            ui_element::icon(if spec.is_expanded { "chevron-down" } else { "chevron-right" })
                .w(icon_size)
                .h(icon_size)
                .text_color(scope_color),
        )
        .child(
            ui_element::label(spec.resolved_count_label())
                .text_size(font_size)
                .text_weight(600)
                .text_color(count_color),
        )
        .child(
            ui_element::label(format!("+{}", totals.additions))
                .text_size(font_size)
                .text_color(additions),
        )
        .child(
            ui_element::label(format!("−{}", totals.deletions))
                .text_size(font_size)
                .text_color(deletions),
        );

    let mut root = ui_element::div()
        .flex_col()
        .w_full()
        .border(hairline)
        .border_color(border)
        .rounded(radius)
        .bg(surface)
        .child(header);

    if spec.is_expanded {
        // Rows are indented by depth rather than nested, because the fold has
        // already collapsed the chains that would have needed nesting to read.
        fn flatten(nodes: &[ChangedFileNode], depth: usize, out: &mut Vec<(usize, ChangedFileNode)>) {
            for node in nodes {
                out.push((depth, node.clone()));
                flatten(&node.children, depth + 1, out);
            }
        }

        let mut flat = Vec::new();
        flatten(&spec.tree(), 0, &mut flat);

        let mut tree = ui_element::div()
            .flex_col()
            .w_full()
            .pl(inset)
            .pr(inset)
            .pb(inset)
            .aria_role(jetstream_ui::accesskit::Role::Tree);

        for (depth, node) in flat {
            tree = tree.child(
                ui_element::div()
                    .flex_row()
                    .w_full()
                    .items_center()
                    .gap(gap)
                    .pl(rem_to_px(0.75) * depth as f32)
                    .aria_role(jetstream_ui::accesskit::Role::TreeItem)
                    .child(
                        ui_element::icon(if node.is_directory { "folder" } else { "file" })
                            .w(icon_size)
                            .h(icon_size)
                            .text_color(scope_color),
                    )
                    .child(
                        ui_element::label(node.label.clone())
                            .text_size(font_size)
                            .text_color(count_color)
                            .grow()
                            .min_w_0(),
                    )
                    .child(
                        ui_element::label(format!("+{}", node.additions))
                            .text_size(font_size)
                            .text_color(additions),
                    )
                    .child(
                        ui_element::label(format!("−{}", node.deletions))
                            .text_size(font_size)
                            .text_color(deletions),
                    ),
            );
        }
        root = root.child(tree);
    } else {
        let mut summary = ui_element::div()
            .flex_row()
            .w_full()
            .items_center()
            .gap(gap)
            .pl(inset)
            .pr(inset)
            .pb(inset);

        for (name, count) in spec.scopes() {
            let word = if count == 1 { "file" } else { "files" };
            summary = summary.child(
                ui_element::label(format!("{name} {count} {word}"))
                    .text_size(font_size)
                    .text_color(scope_color),
            );
        }

        for file in spec.visible_chips() {
            let leaf = file.path.rsplit('/').next().unwrap_or(&file.path).to_string();
            summary = summary.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(rem_to_px(0.25))
                    .pl(rem_to_px(0.375))
                    .pr(rem_to_px(0.375))
                    .pt(rem_to_px(0.125))
                    .pb(rem_to_px(0.125))
                    .border(hairline)
                    .border_color(border)
                    .rounded(chip_radius)
                    .bg(chip_fill)
                    .child(
                        ui_element::icon("file")
                            .w(icon_size)
                            .h(icon_size)
                            .text_color(scope_color),
                    )
                    .child(
                        ui_element::label(leaf)
                            .text_size(font_size)
                            .text_color(scope_color),
                    ),
            );
        }

        root = root.child(summary);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_transcript::ChangedFile;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn file(path: &str, a: u32, d: u32) -> ChangedFile {
        ChangedFile { path: path.into(), additions: a, deletions: d, status: None }
    }

    #[test]
    fn the_header_reports_totals() {
        let spec = ChangedFilesSpec::new("c", vec![file("a.rs", 361, 11), file("b.md", 15, 5)]);
        let tree = crate::render_probe::probe(&js_changed_files(&spec, &theme()), 720.0, 128.0);

        assert!(tree.has_text("2 changed files"), "{:?}", tree.texts());
        assert!(tree.has_text("+376"), "{:?}", tree.texts());
    }

    #[test]
    fn a_chain_with_no_forks_reads_as_one_row() {
        let spec = ChangedFilesSpec::new("c", vec![file("app/src/lib/editor/machine.ts", 12, 3)])
            .with_expanded(true);
        let tree = crate::render_probe::probe(&js_changed_files(&spec, &theme()), 720.0, 256.0);

        assert!(tree.has_text("app/src/lib/editor"), "{:?}", tree.texts());
    }

    #[test]
    fn a_single_file_scope_is_singular() {
        let spec = ChangedFilesSpec::new("c", vec![file("pkg/a.rs", 1, 0)]);
        let tree = crate::render_probe::probe(&js_changed_files(&spec, &theme()), 720.0, 128.0);

        assert!(tree.has_text("pkg 1 file"), "{:?}", tree.texts());
    }
}
