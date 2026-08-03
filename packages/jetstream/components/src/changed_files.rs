//! ChangedFiles — what a turn touched, backed by `ChangedFilesSpec`.
//!
//! Contract: `docs/contracts/components/changed-files.md`.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_headless::agent_transcript::ChangedFileNode;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ChangedFilesSpec;

use std::sync::Arc;

use crate::element::{Handler, IntoJsEl};
use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

/// ChangedFiles — what a turn touched.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_x(handler)`.
pub struct ChangedFiles {
    spec: ChangedFilesSpec,
    theme: JetstreamThemeProvider,
    on_toggle: Option<Handler>,
    on_file_select: Option<Handler>,
}

impl ChangedFiles {
    pub fn from_spec(spec: ChangedFilesSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
            on_file_select: None,
        }
    }

    /// Fires with the card id when the file tree is opened or closed.
    pub fn on_toggle(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_toggle = Some(Arc::new(handler));
        self
    }

    /// Fires with the file's full path when a chip or a tree row is chosen.
    ///
    /// The payload is the path, not the row's label: a collapsed chain shows
    /// `crates/latex` but the host needs the file it stands for.
    pub fn on_file_select(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_file_select = Some(Arc::new(handler));
        self
    }
}

impl IntoJsEl for ChangedFiles {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_toggle, self.on_file_select)
    }
}

pub fn js_changed_files(spec: &ChangedFilesSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None, None)
}

fn build(
    spec: &ChangedFilesSpec,
    theme: &JetstreamThemeProvider,
    on_toggle: Option<Handler>,
    on_file_select: Option<Handler>,
) -> JsEl {
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

    let mut header = ui_element::button("")
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
            ui_element::icon(if spec.is_expanded {
                "chevron-down"
            } else {
                "chevron-right"
            })
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

    if let Some(handler) = on_toggle {
        let id = spec.id.clone();
        header = header.cursor_pointer().on_click(move |_event| handler(&id));
    }

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

        let mut tree = ui_element::div()
            .flex_col()
            .w_full()
            .pl(inset)
            .pr(inset)
            .pb(inset)
            .aria_role(jetstream_ui::accesskit::Role::Tree);

        for (depth, node) in flat {
            let mut row = ui_element::div()
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
                );

            // Only files select. A directory row's job is to expand, and
            // handing a host a directory as a "file chosen" is a lie it would
            // have to filter out itself.
            if let (false, Some(handler)) = (node.is_directory, &on_file_select) {
                let handler = Arc::clone(handler);
                let path = node.path.clone();
                row = row.cursor_pointer().on_click(move |_event| handler(&path));
            }

            tree = tree.child(row);
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
            let leaf = file
                .path
                .rsplit('/')
                .next()
                .unwrap_or(&file.path)
                .to_string();
            let mut chip = ui_element::div()
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
                );

            // A chip shows the leaf but reports the full path, same as a tree
            // row: the two are the same event seen from two states.
            if let Some(handler) = &on_file_select {
                let handler = Arc::clone(handler);
                let path = file.path.clone();
                chip = chip.cursor_pointer().on_click(move |_event| handler(&path));
            }

            summary = summary.child(chip);
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
        ChangedFile {
            path: path.into(),
            additions: a,
            deletions: d,
            status: None,
        }
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
    fn the_header_toggles_the_card() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let spec = ChangedFilesSpec::new("card-1", vec![file("a.rs", 3, 1)]);
        let el = ChangedFiles::from_spec(spec, &theme())
            .on_toggle(move |id| {
                assert_eq!(id, "card-1");
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 720.0, 128.0, "1 changed files");

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_toggle fired exactly once"
        );
    }

    /// A chip shows the leaf and reports the path — the whole point of the
    /// event, since the leaf alone does not identify the file.
    #[test]
    fn a_chip_reports_the_full_path() {
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let paths = Arc::clone(&seen);

        let spec = ChangedFilesSpec::new("card-1", vec![file("pkg/src/main.rs", 3, 1)]);
        let el = ChangedFiles::from_spec(spec, &theme())
            .on_file_select(move |path| paths.lock().unwrap().push(path.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 720.0, 128.0, "main.rs");

        assert_eq!(seen.lock().unwrap().as_slice(), ["pkg/src/main.rs"]);
    }

    /// A directory expands; it is not a file, and reporting it as one would
    /// make every host filter directories back out.
    #[test]
    fn a_directory_row_is_not_a_file_selection() {
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let paths = Arc::clone(&seen);

        let spec = ChangedFilesSpec::new(
            "card-1",
            vec![file("pkg/a.rs", 3, 1), file("pkg/b.rs", 2, 0)],
        )
        .with_expanded(true);

        let el = ChangedFiles::from_spec(spec, &theme())
            .on_file_select(move |path| paths.lock().unwrap().push(path.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 720.0, 256.0, "pkg");
        assert!(
            seen.lock().unwrap().is_empty(),
            "a directory row selected a file: {:?}",
            seen.lock().unwrap()
        );

        crate::element::click_probe::click_text(&el, 720.0, 256.0, "a.rs");
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            ["pkg/a.rs"],
            "the file row still selects"
        );
    }

    #[test]
    fn a_single_file_scope_is_singular() {
        let spec = ChangedFilesSpec::new("c", vec![file("pkg/a.rs", 1, 0)]);
        let tree = crate::render_probe::probe(&js_changed_files(&spec, &theme()), 720.0, 128.0);

        assert!(tree.has_text("pkg 1 file"), "{:?}", tree.texts());
    }
}
