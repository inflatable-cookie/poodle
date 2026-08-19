//! HistoryCenter specimen (g15.021). Six teaching sections aligned with the
//! web catalogue; each keeps its own host-state prefix and stays closed on mount.

use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::Eyebrow;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::history_center::{
    history_center_visible_rows, HistoryCenterOpenFork, HistoryContinuation, HistoryEntry,
    HistoryEntryPosition, HistoryPathPage,
};
use poodle_render::{
    history_center, HistoryCenterDelete, HistoryCenterHandlers, HistoryCenterRename,
    HistoryCenterView,
};
use poodle_specs::{
    EyebrowSpec, HistoryCenterRejection, HistoryCenterSpec,
};

const T: u64 = 1_750_000_000_000;

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

fn entry(
    id: &str,
    label: &str,
    position: HistoryEntryPosition,
    count: usize,
) -> HistoryEntry {
    HistoryEntry::new(id, label)
        .with_position(position)
        .with_continuation_count(count)
}

fn entry_at(
    id: &str,
    label: &str,
    position: HistoryEntryPosition,
    count: usize,
    recorded_at_ms: u64,
) -> HistoryEntry {
    entry(id, label, position, count).with_recorded_at_ms(recorded_at_ms)
}

fn page(entries: Vec<HistoryEntry>) -> HistoryPathPage {
    HistoryPathPage::new(entries)
}

fn continuation(
    entry_id: &str,
    label: &str,
    branch_id: &str,
) -> HistoryContinuation {
    HistoryContinuation::new(entry_id, label, branch_id).with_entry_count(2)
}

fn linear_pages() -> Vec<HistoryPathPage> {
    vec![page(vec![
        entry_at(
            "e3",
            "Current draft",
            HistoryEntryPosition::Current,
            0,
            T + 3_600_000,
        ),
        entry_at(
            "e2",
            "Arranged intro",
            HistoryEntryPosition::Past,
            1,
            T + 600_000,
        ),
        entry_at(
            "e1",
            "Committed mix 1",
            HistoryEntryPosition::Past,
            1,
            T,
        ),
    ])]
}

fn two_fork_pages() -> Vec<HistoryPathPage> {
    vec![page(vec![
        entry_at(
            "c3",
            "Current draft",
            HistoryEntryPosition::Current,
            0,
            T + 3_600_000,
        ),
        entry_at(
            "c2",
            "Arranged intro",
            HistoryEntryPosition::Past,
            3,
            T + 600_000,
        ),
        entry_at(
            "c1",
            "Committed mix 1",
            HistoryEntryPosition::Past,
            1,
            T,
        ),
    ])]
}

fn two_fork_continuations() -> Vec<HistoryContinuation> {
    vec![
        continuation("l1", "Lead intro", "feature/lead").with_branch_name("feature/lead"),
        continuation("x1", "Alt intro", "feature/alt")
            .with_preferred(true)
            .with_entry_count(1)
            .with_branch_name("feature/alt"),
    ]
}

fn two_fork_run(fork: &str) -> Vec<HistoryPathPage> {
    match fork {
        "x1" => vec![page(vec![
            entry_at("x2", "Alt mix", HistoryEntryPosition::Past, 0, T + 2_300_000),
            entry_at("x1", "Alt intro", HistoryEntryPosition::Past, 1, T + 1_100_000),
        ])],
        "l1" => vec![page(vec![
            entry_at("l2", "Lead mix", HistoryEntryPosition::Past, 0, T + 2_400_000),
            entry_at("l1", "Lead intro", HistoryEntryPosition::Past, 1, T + 1_200_000),
        ])],
        _ => Vec::new(),
    }
}

fn nested_pages() -> Vec<HistoryPathPage> {
    two_fork_pages()
}

fn nested_continuations(anchor: &str) -> Vec<HistoryContinuation> {
    match anchor {
        "c2" => vec![continuation("l1", "Lead intro", "feature/lead")
            .with_preferred(true)
            .with_entry_count(3)
            .with_branch_name("feature/lead")],
        "l2" => vec![continuation("i1", "Inner intro", "feature/inner")
            .with_preferred(true)
            .with_entry_count(2)
            .with_branch_name("feature/inner")],
        _ => Vec::new(),
    }
}

fn nested_run(fork: &str) -> Vec<HistoryPathPage> {
    match fork {
        "l1" => vec![page(vec![
            entry_at("l3", "Lead outro", HistoryEntryPosition::Past, 0, T + 3_000_000),
            entry_at("l2", "Lead bridge", HistoryEntryPosition::Past, 2, T + 2_400_000),
            entry_at("l1", "Lead intro", HistoryEntryPosition::Past, 1, T + 1_200_000),
        ])],
        "i1" => vec![page(vec![
            entry_at("i2", "Inner mix", HistoryEntryPosition::Past, 0, T + 2_700_000),
            entry_at("i1", "Inner intro", HistoryEntryPosition::Past, 1, T + 2_500_000),
        ])],
        _ => Vec::new(),
    }
}

fn single_continuation_pages() -> Vec<HistoryPathPage> {
    vec![page(vec![
        entry_at(
            "c3",
            "Current draft",
            HistoryEntryPosition::Current,
            0,
            T + 3_600_000,
        ),
        entry_at(
            "c2",
            "Arranged intro",
            HistoryEntryPosition::Past,
            1,
            T + 600_000,
        ),
        entry_at(
            "c1",
            "Committed mix 1",
            HistoryEntryPosition::Past,
            1,
            T,
        ),
    ])]
}

fn run_tail_pages() -> Vec<HistoryPathPage> {
    two_fork_pages()
}

fn run_tail_continuations() -> Vec<HistoryContinuation> {
    vec![continuation("l1", "Lead intro", "feature/lead")
        .with_preferred(true)
        .with_entry_count(3)
        .with_branch_name("feature/lead")]
}

fn run_tail_run(fork: &str) -> Vec<HistoryPathPage> {
    match fork {
        "l1" => vec![page(vec![
            entry_at("l3", "Lead outro", HistoryEntryPosition::Past, 0, T + 3_000_000),
            entry_at("l2", "Lead bridge", HistoryEntryPosition::Past, 1, T + 2_400_000),
            entry_at("l1", "Lead intro", HistoryEntryPosition::Past, 1, T + 1_200_000),
        ])],
        _ => Vec::new(),
    }
}

fn no_timestamp_pages() -> Vec<HistoryPathPage> {
    vec![page(vec![
        entry("c3", "Current draft", HistoryEntryPosition::Current, 0),
        entry("c2", "Arranged intro", HistoryEntryPosition::Past, 2),
        entry("c1", "Committed mix 1", HistoryEntryPosition::Past, 1),
    ])]
}

fn no_timestamp_continuations() -> Vec<HistoryContinuation> {
    run_tail_continuations()
}

fn no_timestamp_run(fork: &str) -> Vec<HistoryPathPage> {
    match fork {
        "l1" => vec![page(vec![
            entry("l2", "Lead mix", HistoryEntryPosition::Past, 0),
            entry("l1", "Lead intro", HistoryEntryPosition::Past, 1),
        ])],
        _ => Vec::new(),
    }
}

struct SectionState<'a> {
    state: &'a AppState,
    prefix: &'static str,
}

impl<'a> SectionState<'a> {
    fn new(state: &'a AppState, prefix: &'static str) -> Self {
        Self { state, prefix }
    }

    fn key(&self, suffix: &str) -> String {
        format!("{}-{suffix}", self.prefix)
    }

    fn is_on(&self, suffix: &str) -> bool {
        self.state.specimens.is_on(&self.key(suffix))
    }

    fn text(&self, suffix: &str, default: &str) -> String {
        self.state
            .specimens
            .text
            .get(&self.key(suffix))
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    fn open_fork(
        &self,
        anchor: &str,
        continuations: Vec<HistoryContinuation>,
        pick: Option<HistoryContinuation>,
        run_pages: Vec<HistoryPathPage>,
        inner: Vec<HistoryCenterOpenFork>,
    ) -> HistoryCenterOpenFork {
        HistoryCenterOpenFork {
            anchor_entry_id: anchor.to_string(),
            continuations: Some(continuations),
            pick,
            chosen: None,
            run_pages,
            inner,
        }
    }

    fn fork_open_levels(
        &self,
        anchor: &str,
        continuations: Vec<HistoryContinuation>,
        default_pick: &str,
        run_for: fn(&str) -> Vec<HistoryPathPage>,
    ) -> Vec<HistoryCenterOpenFork> {
        if !self.is_on("disclosed") {
            return Vec::new();
        }
        let pick_id = self.text("pick", default_pick);
        let pick = continuations
            .iter()
            .find(|fork| fork.entry_id == pick_id)
            .cloned();
        vec![self.open_fork(
            anchor,
            continuations,
            pick.clone(),
            pick.as_ref()
                .map(|fork| run_for(&fork.entry_id))
                .unwrap_or_default(),
            Vec::new(),
        )]
    }

    fn nested_open_levels(&self) -> Vec<HistoryCenterOpenFork> {
        if !self.is_on("disclosed") {
            return Vec::new();
        }
        let pick_id = self.text("pick", "l1");
        let outer_pick = nested_continuations("c2")
            .into_iter()
            .find(|fork| fork.entry_id == pick_id)
            .or_else(|| nested_continuations("c2").into_iter().next());
        let mut inner = Vec::new();
        if self.is_on("inner-disclosed") {
            let inner_pick = nested_continuations("l2").into_iter().next();
            inner.push(self.open_fork(
                "l2",
                nested_continuations("l2"),
                inner_pick.clone(),
                inner_pick
                    .as_ref()
                    .map(|fork| nested_run(&fork.entry_id))
                    .unwrap_or_default(),
                Vec::new(),
            ));
        }
        vec![self.open_fork(
            "c2",
            nested_continuations("c2"),
            outer_pick.clone(),
            outer_pick
                .as_ref()
                .map(|fork| nested_run(&fork.entry_id))
                .unwrap_or_default(),
            inner,
        )]
    }

    fn view(
        &self,
        pages: &[HistoryPathPage],
        open_levels: &[HistoryCenterOpenFork],
        rejection: Option<String>,
    ) -> HistoryCenterView {
        HistoryCenterView {
            is_open: self.is_on("open"),
            rows: history_center_visible_rows(Some(&pages.to_vec()), open_levels),
            open_anchors: open_levels
                .iter()
                .map(|level| level.anchor_entry_id.clone())
                .collect(),
            open_select_anchor: self
                .is_on("select-open")
                .then(|| "c2".to_string()),
            open_actions_anchor: self
                .is_on("actions-open")
                .then(|| "c2".to_string()),
            rename: self.is_on("renaming").then(|| HistoryCenterRename {
                anchor_entry_id: "c2".to_string(),
                branch_id: "feature/lead".to_string(),
                value: self.text("rename", "Wide mix"),
            }),
            delete_target: self.is_on("deleting").then(|| HistoryCenterDelete {
                entry_id: self.text("delete-entry", "l1"),
                label: self.text("delete-label", "Lead intro"),
            }),
            rejection,
            ..HistoryCenterView::default()
        }
    }

    fn handlers(
        &self,
        queue: Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
        anchor: &'static str,
        on_disclose_extra: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    ) -> HistoryCenterHandlers {
        let prefix = self.prefix.to_string();
        let disclosed = self.is_on("disclosed");
        let select_open = self.is_on("select-open");
        let actions_open = self.is_on("actions-open");
        HistoryCenterHandlers {
            on_undo: Some({
                let queue = queue.clone();
                let prefix = prefix.clone();
                Arc::new(move || {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                        key: format!("{prefix}-command"),
                        value: "undo".to_string(),
                    });
                })
            }),
            on_open_change: Some({
                let queue = queue.clone();
                let prefix = prefix.clone();
                Arc::new(move |open| {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                        key: format!("{prefix}-open"),
                        value: open,
                    });
                })
            }),
            on_disclose: Some({
                let queue = queue.clone();
                let prefix = prefix.clone();
                Arc::new(move |entry| {
                    if entry == anchor {
                        queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                            key: format!("{prefix}-disclosed"),
                            value: !disclosed,
                        });
                    }
                    if let Some(extra) = &on_disclose_extra {
                        extra(entry);
                    }
                })
            }),
            on_select_toggle: Some({
                let queue = queue.clone();
                let prefix = prefix.clone();
                Arc::new(move |_anchor| {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                        key: format!("{prefix}-select-open"),
                        value: !select_open,
                    });
                })
            }),
            on_actions_toggle: Some({
                let queue = queue.clone();
                let prefix = prefix.clone();
                Arc::new(move |_anchor| {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                        key: format!("{prefix}-actions-open"),
                        value: !actions_open,
                    });
                })
            }),
            on_pick: Some({
                let queue = queue.clone();
                let prefix = prefix.clone();
                Arc::new(move |entry_id| {
                    let mut events = queue.lock().unwrap();
                    events.push(NodeSpecimenEvent::SetText {
                        key: format!("{prefix}-pick"),
                        value: entry_id.to_string(),
                    });
                    events.push(NodeSpecimenEvent::SetToggle {
                        key: format!("{prefix}-select-open"),
                        value: false,
                    });
                })
            }),
            on_checkout: Some({
                let queue = queue.clone();
                let prefix = prefix.clone();
                Arc::new(move |_anchor| {
                    let mut events = queue.lock().unwrap();
                    events.push(NodeSpecimenEvent::SetText {
                        key: format!("{prefix}-command"),
                        value: "checkout".to_string(),
                    });
                    events.push(NodeSpecimenEvent::SetToggle {
                        key: format!("{prefix}-actions-open"),
                        value: false,
                    });
                })
            }),
            on_rename_open: Some({
                let queue = queue.clone();
                let prefix = prefix.clone();
                Arc::new(move |_anchor| {
                    let mut events = queue.lock().unwrap();
                    events.push(NodeSpecimenEvent::SetToggle {
                        key: format!("{prefix}-renaming"),
                        value: true,
                    });
                    events.push(NodeSpecimenEvent::SetToggle {
                        key: format!("{prefix}-actions-open"),
                        value: false,
                    });
                })
            }),
            on_rename_key: Some({
                let queue = queue.clone();
                let prefix = prefix.clone();
                Arc::new(move |key| {
                    if key == "enter" || key == "escape" {
                        queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                            key: format!("{prefix}-renaming"),
                            value: false,
                        });
                    }
                })
            }),
            on_activate_row: Some({
                let queue = queue.clone();
                let prefix = prefix.clone();
                Arc::new(move |row| {
                    queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
                        key: format!("{prefix}-command"),
                        value: format!("navigate {}", row.entry_id),
                    });
                })
            }),
            ..HistoryCenterHandlers::default()
        }
    }

    fn with_delete_handlers(
        &self,
        mut handlers: HistoryCenterHandlers,
        queue: Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
    ) -> HistoryCenterHandlers {
        let prefix = self.prefix.to_string();
        handlers.on_delete_request = Some({
            let queue = queue.clone();
            let prefix = prefix.clone();
            Arc::new(move |target: &HistoryContinuation| {
                let mut events = queue.lock().unwrap();
                events.push(NodeSpecimenEvent::SetText {
                    key: format!("{prefix}-delete-entry"),
                    value: target.entry_id.clone(),
                });
                events.push(NodeSpecimenEvent::SetText {
                    key: format!("{prefix}-delete-label"),
                    value: target.label.clone(),
                });
                events.push(NodeSpecimenEvent::SetToggle {
                    key: format!("{prefix}-deleting"),
                    value: true,
                });
                events.push(NodeSpecimenEvent::SetToggle {
                    key: format!("{prefix}-actions-open"),
                    value: false,
                });
            })
        });
        handlers.on_delete_confirm = Some({
            let queue = queue.clone();
            let prefix = prefix.clone();
            Arc::new(move |entry_id| {
                let mut events = queue.lock().unwrap();
                events.push(NodeSpecimenEvent::SetText {
                    key: format!("{prefix}-command"),
                    value: format!("delete {entry_id}"),
                });
                events.push(NodeSpecimenEvent::SetToggle {
                    key: format!("{prefix}-deleting"),
                    value: false,
                });
            })
        });
        handlers.on_delete_cancel = Some({
            let prefix = prefix.clone();
            Arc::new(move || {
                queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                    key: format!("{prefix}-deleting"),
                    value: false,
                });
            })
        });
        handlers
    }
}

fn render_instance(
    theme: &GpuiThemeProvider,
    spec: &HistoryCenterSpec,
    view: &HistoryCenterView,
    handlers: HistoryCenterHandlers,
    instance_id: &str,
) -> AnyElement {
    let mut handlers = handlers;
    handlers.instance_id = Some(instance_id.to_string());
    poodle_gpui_node_backend::to_gpui(&history_center(spec, theme, view, &handlers))
}

fn hint(theme: &GpuiThemeProvider, label: &str, value: &str) -> Div {
    div().when(!value.is_empty(), |d| {
        d.child(
            div()
                .text_sm()
                .text_color(color_to_hsla(theme.resolve_color("color.text.secondary")))
                .child(format!("{label}: {value}")),
        )
    })
}

fn inner_disclose_handler(
    queue: Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
    prefix: &'static str,
    inner_disclosed: bool,
) -> Arc<dyn Fn(&str) + Send + Sync> {
    Arc::new(move |entry: &str| {
        if entry == "l2" {
            queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                key: format!("{prefix}-inner-disclosed"),
                value: !inner_disclosed,
            });
        }
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let queue = state.node_events.clone();

    let linear = SectionState::new(state, "hc-linear");
    let forks = SectionState::new(state, "hc-forks");
    let nested = SectionState::new(state, "hc-nested");
    let single = SectionState::new(state, "hc-single");
    let run_tail = SectionState::new(state, "hc-run-tail");
    let manage = SectionState::new(state, "hc-manage");
    let rename = SectionState::new(state, "hc-rename");
    let failure = SectionState::new(state, "hc-failure");
    let no_ts = SectionState::new(state, "hc-no-ts");

    let fork_levels = forks.fork_open_levels("c2", two_fork_continuations(), "x1", two_fork_run);
    let run_tail_levels =
        run_tail.fork_open_levels("c2", run_tail_continuations(), "l1", run_tail_run);
    let manage_levels =
        manage.fork_open_levels("c2", run_tail_continuations(), "l1", run_tail_run);
    let rename_levels = rename.nested_open_levels();
    let no_ts_levels = no_ts.fork_open_levels("c2", no_timestamp_continuations(), "l1", no_timestamp_run);

    let rejection_spec = HistoryCenterSpec::new()
        .with_can_undo(true)
        .with_rejection(HistoryCenterRejection::AlreadyAtTarget);
    let rejection_message = rejection_spec.rejection_message().map(str::to_string);

    let nested_extra = inner_disclose_handler(
        queue.clone(),
        "hc-nested",
        nested.is_on("inner-disclosed"),
    );
    let rename_extra = inner_disclose_handler(
        queue.clone(),
        "hc-rename",
        rename.is_on("inner-disclosed"),
    );

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Linear history",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .w_full()
                        .max_w(px(672.0))
                        .child(render_instance(
                            theme,
                            &HistoryCenterSpec::new().with_can_undo(true),
                            &linear.view(&linear_pages(), &[], None),
                            linear.handlers(queue.clone(), "c2", None),
                            "hc-linear",
                        )),
                )
                .child(hint(
                    theme,
                    "Last host command",
                    &linear.text("command", ""),
                )),
        ))
        .child(group(
            "Choosing between continuations",
            theme,
            div()
                .flex()
                .justify_end()
                .w_full()
                .max_w(px(672.0))
                .child(render_instance(
                    theme,
                    &HistoryCenterSpec::new().with_can_undo(true),
                    &forks.view(&two_fork_pages(), &fork_levels, None),
                    forks.handlers(queue.clone(), "c2", None),
                    "hc-forks",
                )),
        ))
        .child(group(
            "Nested continuation runs",
            theme,
            div()
                .flex()
                .justify_end()
                .w_full()
                .max_w(px(672.0))
                .child(render_instance(
                    theme,
                    &HistoryCenterSpec::new().with_can_undo(true),
                    &nested.view(&nested_pages(), &nested.nested_open_levels(), None),
                    nested.handlers(queue.clone(), "c2", Some(nested_extra)),
                    "hc-nested",
                )),
        ))
        .child(group(
            "Single continuation and run boundaries",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .w_full()
                        .max_w(px(672.0))
                        .child(render_instance(
                            theme,
                            &HistoryCenterSpec::new().with_can_undo(true),
                            &single.view(&single_continuation_pages(), &[], None),
                            single.handlers(queue.clone(), "c2", None),
                            "hc-single",
                        )),
                )
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .w_full()
                        .max_w(px(672.0))
                        .child(render_instance(
                            theme,
                            &HistoryCenterSpec::new().with_can_undo(true),
                            &run_tail.view(&run_tail_pages(), &run_tail_levels, None),
                            run_tail.handlers(queue.clone(), "c2", None),
                            "hc-run-tail",
                        )),
                ),
        ))
        .child(
            group(
                "Rename and manage a continuation",
                theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .w_full()
                            .max_w(px(672.0))
                            .child(render_instance(
                                theme,
                                &HistoryCenterSpec::new().with_can_undo(true),
                                &manage.view(&run_tail_pages(), &manage_levels, None),
                                manage.with_delete_handlers(
                                    manage.handlers(queue.clone(), "c2", None),
                                    queue.clone(),
                                ),
                                "hc-manage",
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .w_full()
                            .max_w(px(672.0))
                            .child(render_instance(
                                theme,
                                &HistoryCenterSpec::new().with_can_undo(true),
                                &rename.view(&nested_pages(), &rename_levels, None),
                                rename.handlers(queue.clone(), "c2", Some(rename_extra)),
                                "hc-rename",
                            )),
                    )
                    .child(hint(
                        theme,
                        "Last command",
                        &{
                            let rename_command = rename.text("command", "");
                            if rename_command.is_empty() {
                                manage.text("command", "")
                            } else {
                                rename_command
                            }
                        },
                    )),
            ),
        )
        .child(
            group(
                "Failure and incomplete metadata",
                theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .w_full()
                            .max_w(px(672.0))
                            .child(render_instance(
                                theme,
                                &rejection_spec,
                                &failure.view(&two_fork_pages(), &[], rejection_message.clone()),
                                failure.handlers(queue.clone(), "c2", None),
                                "hc-failure",
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .w_full()
                            .max_w(px(672.0))
                            .child(render_instance(
                                theme,
                                &HistoryCenterSpec::new().with_can_undo(true),
                                &no_ts.view(&no_timestamp_pages(), &no_ts_levels, None),
                                no_ts.handlers(queue.clone(), "c2", None),
                                "hc-no-ts",
                            )),
                    )
                    .child(hint(
                        theme,
                        "Last command",
                        &failure.text("command", ""),
                    )),
            ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "history-center",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                render_instance(
                    theme,
                    &HistoryCenterSpec::new().with_size(size),
                    &HistoryCenterView {
                        rows: history_center_visible_rows(Some(&linear_pages()), &[]),
                        ..HistoryCenterView::default()
                    },
                    HistoryCenterHandlers::default(),
                    "hc-linear-size",
                )
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                render_instance(
                    theme,
                    &HistoryCenterSpec::new().with_density(density),
                    &HistoryCenterView {
                        rows: history_center_visible_rows(Some(&linear_pages()), &[]),
                        ..HistoryCenterView::default()
                    },
                    HistoryCenterHandlers::default(),
                    "hc-linear-density",
                )
            }),
    )
}
