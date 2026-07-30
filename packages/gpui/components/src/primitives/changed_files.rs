//! ChangedFiles — what a turn touched, backed by `ChangedFilesSpec`.
//!
//! Contract: `docs/contracts/components/changed-files.md`.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_transcript::ChangedFileNode;
use poodle_specs::ChangedFilesSpec;

use crate::presentation::rem_to_px;
use crate::primitives::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct ChangedFiles {
    spec: ChangedFilesSpec,
    theme: GpuiThemeProvider,
    on_toggle: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_file_select: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl ChangedFiles {
    pub fn from_spec(spec: ChangedFilesSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_toggle: None,
            on_file_select: None,
        }
    }

    /// The card was opened or closed.
    pub fn on_toggle(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(std::rc::Rc::new(handler));
        self
    }

    /// A file row or chip was chosen.
    pub fn on_file_select(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_file_select = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for ChangedFiles {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // An empty card renders nothing rather than an empty state. A turn that
        // changed no files should not have a box saying so.
        if !spec.renders() {
            return div().into_any_element();
        }

        let surface = resolve_color(theme, spec.surface_token());
        let border = resolve_color(theme, spec.border_token());
        let count_color = resolve_color(theme, spec.count_token());
        let additions_color = resolve_color(theme, spec.additions_token());
        let deletions_color = resolve_color(theme, spec.deletions_token());
        let scope_color = resolve_color(theme, spec.scope_token());
        let chip_fill = resolve_color(theme, spec.chip_fill_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let chip_radius = resolve_radius(theme, spec.chip_radius_token());

        let font_size = px(rem_to_px(spec.font_size_rem()));
        let inset = px(rem_to_px(spec.padding_inset_rem()));
        let gap = px(rem_to_px(spec.gap_rem()));
        let icon_px = rem_to_px(spec.icon_size_rem());

        let totals = spec.totals();

        let mut header = div()
            .id(SharedString::from(format!("poodle-changed-files-{}", spec.id)))
            .cursor_pointer()
            .flex()
            .items_center()
            .gap(gap)
            .p(inset)
            .text_size(font_size)
            .child(
                Icon::new(
                    if spec.is_expanded { "chevron-down" } else { "chevron-right" },
                    theme,
                )
                .with_px_size(icon_px)
                .with_color(scope_color)
                .into_any_element(),
            )
            .child(
                div()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(count_color)
                    .child(spec.resolved_count_label()),
            )
            .child(
                div()
                    .text_color(additions_color)
                    .child(format!("+{}", totals.additions)),
            )
            .child(
                div()
                    .text_color(deletions_color)
                    .child(format!("−{}", totals.deletions)),
            );

        if let Some(handler) = &self.on_toggle {
            let handler = handler.clone();
            let id = spec.id.clone();
            header = header.on_click(move |_event, window, cx| handler(&id, window, cx));
        }

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .border_1()
            .border_color(border)
            .rounded(radius)
            .bg(surface)
            .child(header);

        if spec.is_expanded {
            // Rows are indented by depth rather than nested, because the fold
            // has already collapsed the chains that would have needed nesting
            // to read.
            fn rows(
                nodes: &[ChangedFileNode],
                depth: usize,
                out: &mut Vec<(usize, ChangedFileNode)>,
            ) {
                for node in nodes {
                    out.push((depth, node.clone()));
                    rows(&node.children, depth + 1, out);
                }
            }

            let mut flat = Vec::new();
            rows(&spec.tree(), 0, &mut flat);

            let mut tree = div().flex().flex_col().px(inset).pb(inset).text_size(font_size);
            for (depth, node) in flat {
                tree = tree.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(gap)
                        .pl(px(rem_to_px(0.75) * depth as f32))
                        .child(
                            Icon::new(if node.is_directory { "folder" } else { "file" }, theme)
                                .with_px_size(icon_px)
                                .with_color(scope_color)
                                .into_any_element(),
                        )
                        .child(div().flex_1().min_w_0().text_color(count_color).child(node.label.clone()))
                        .child(
                            div()
                                .text_color(additions_color)
                                .child(format!("+{}", node.additions)),
                        )
                        .child(
                            div()
                                .text_color(deletions_color)
                                .child(format!("−{}", node.deletions)),
                        ),
                );
            }
            root = root.child(tree);
        } else {
            let mut summary = div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(gap)
                .px(inset)
                .pb(inset)
                .text_size(font_size);

            for (name, count) in spec.scopes() {
                summary = summary.child(
                    div()
                        .text_color(scope_color)
                        .child(format!("{name} {count} {}", if count == 1 { "file" } else { "files" })),
                );
            }

            for file in spec.visible_chips() {
                let leaf = file.path.rsplit('/').next().unwrap_or(&file.path).to_string();
                let mut chip = div()
                    .id(SharedString::from(format!("poodle-changed-file-chip-{}", file.path)))
                    .cursor_pointer();
                if let Some(handler) = &self.on_file_select {
                    let handler = handler.clone();
                    let path = file.path.clone();
                    chip = chip.on_click(move |_event, window, cx| handler(&path, window, cx));
                }

                summary = summary.child(
                    chip
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .px(px(6.0))
                        .py(px(2.0))
                        .border_1()
                        .border_color(border)
                        .rounded(chip_radius)
                        .bg(chip_fill)
                        .text_color(scope_color)
                        .child(
                            Icon::new("file", theme)
                                .with_px_size(icon_px)
                                .with_color(scope_color)
                                .into_any_element(),
                        )
                        .child(leaf),
                );
            }

            root = root.child(summary);
        }

        root.into_any_element()
    }
}
