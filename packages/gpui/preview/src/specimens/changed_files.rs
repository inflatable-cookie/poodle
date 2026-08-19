use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{ChangedFiles, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_transcript::ChangedFile;
use poodle_specs::{ChangedFilesSpec, EyebrowSpec};
use std::sync::Arc;

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

fn stack(children: impl IntoIterator<Item = AnyElement>) -> Div {
    let mut col = div().flex().flex_col().gap(px(12.0));
    for child in children {
        col = col.child(child);
    }
    col
}

fn file(path: &str, additions: u32, deletions: u32) -> ChangedFile {
    ChangedFile {
        path: path.to_string(),
        additions,
        deletions,
        status: None,
    }
}

fn worked() -> Vec<ChangedFile> {
    vec![
        file("cp-api/Cargo.toml", 1, 0),
        file("cp-api/crates/latex/src/lexer.rs", 140, 6),
        file("cp-api/crates/latex/src/parser.rs", 131, 4),
        file("cp-api/tools/export_fixture.rs", 60, 1),
        file("cp-api/tools/build.rs", 29, 0),
        file("cp-api/effigy.toml", 1, 0),
        file("cp-api/crates/latex/src/tests/lexer_tests.rs", 0, 0),
        file("cp-docs/book-port-and-production.md", 14, 5),
        file("cp-docs/notes.md", 1, 0),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let expanded = state.specimens.is_on("changed-files-worked");
    let selected = state
        .specimens
        .text
        .get("changed-files-selected")
        .cloned()
        .unwrap_or_default();
    let toggle_events = state.node_events.clone();
    let select_events = state.node_events.clone();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Worked change set",
            theme,
            stack([
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new("worked", worked()).with_expanded(expanded),
                    theme,
                )
                .on_toggle(Arc::new(move |id| {
                    toggle_events
                        .lock()
                        .unwrap()
                        .push(NodeSpecimenEvent::Toggle(format!("changed-files-{id}")));
                }))
                .on_file_select(Arc::new(move |path| {
                    select_events
                        .lock()
                        .unwrap()
                        .push(NodeSpecimenEvent::SetText {
                            key: "changed-files-selected".to_string(),
                            value: path.to_string(),
                        });
                }))
                .with_instance_id("worked")
                .into_any_element(),
                div()
                    .child(if selected.is_empty() {
                        "no file selected".to_string()
                    } else {
                        format!("selected: {selected}")
                    })
                    .into_any_element(),
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new("worked-open", worked()).with_expanded(true),
                    theme,
                )
                .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            "Paths and scopes",
            theme,
            stack([
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new(
                        "deep",
                        vec![file("app/src/lib/features/editor/state/machine.ts", 12, 3)],
                    )
                    .with_expanded(true),
                    theme,
                )
                .into_any_element(),
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new(
                        "scopes",
                        vec![
                            file("cp-api/src/main.rs", 4, 1),
                            file("cp-docs/book.md", 2, 0),
                            file("packages/core/index.ts", 8, 3),
                            file("apps/preview/App.svelte", 5, 1),
                            file("tools/export.ts", 1, 0),
                        ],
                    ),
                    theme,
                )
                .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            "Count variations",
            theme,
            stack([
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new("single", vec![file("README.md", 4, 1)]),
                    theme,
                )
                .into_any_element(),
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new("adds", vec![file("src/new.ts", 88, 0)]),
                    theme,
                )
                .into_any_element(),
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new("dels", vec![file("src/old.ts", 0, 45)]),
                    theme,
                )
                .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            "Overflow and actions",
            theme,
            stack([
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new(
                        "long",
                        vec![
                            file(
                                "packages/contracts/components/src/a_very_long_component_spec_name.rs",
                                3,
                                2,
                            ),
                            file("b.rs", 1, 0),
                            file("c.rs", 1, 0),
                            file("d.rs", 1, 0),
                        ],
                    )
                    .with_chip_limit(2),
                    theme,
                )
                .into_any_element(),
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new("nodiff", vec![file("README.md", 4, 1)])
                        .with_show_open_diff(false),
                    theme,
                )
                .into_any_element(),
            ])
            .into_any_element(),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "changed-files",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new(format!("sz-{size:?}"), worked()).with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                ChangedFiles::from_spec(
                    ChangedFilesSpec::new(format!("dn-{density:?}"), worked())
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
