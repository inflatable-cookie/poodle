//! HistoryCenter GPUI conformance host (g14.007), headless.
//!
//! The fixture host: it holds the machine, answers the two named host
//! commands from the case's declared host records, and records every command
//! that left with the payload it carried. This is the part a real host owns —
//! Poodle emits `loadContinuations` and the host decides what to hand back —
//! so the corpus exercises the boundary rather than pretending across it.
//!
//! The host answers a command only when the component asks. Nothing here
//! loads ahead of a disclosure, and no fork is checked out because the picker
//! showed it.

use poodle_headless::history_center::{
    history_center_keydown_event, history_center_transition, history_center_visible_rows,
    HistoryCenterContext, HistoryCenterEffect, HistoryCenterEvent, HistoryCenterOpenFork,
    HistoryCenterRejectionCode, HistoryCenterResult, HistoryCenterRowId, HistoryCenterRowKind,
    HistoryCenterState,
    HistoryContinuation, HistoryEntry, HistoryPathPage,
};
use poodle_render::{HistoryCenterRename, HistoryCenterView};
use poodle_specs::HistoryCenterSpec;
use serde_json::{json, Value};

use super::conformance_support::{history_center_spec_from_fixture, history_entry_from_json};

/// One fork the host can answer with, tagged with the anchor it hangs off.
struct CatalogueFork {
    anchor_entry_id: String,
    continuation: HistoryContinuation,
}

/// One run entry, tagged with the fork whose run it belongs to.
struct CatalogueRunEntry {
    from_entry_id: String,
    entry: HistoryEntry,
}

/// The fixture host: machine, catalogue, shell-owned UI state, and the
/// recorded command trace.
pub struct HistoryCenterFixtureHost {
    pub spec: HistoryCenterSpec,
    state: HistoryCenterState,
    context: HistoryCenterContext,
    forks: Vec<CatalogueFork>,
    run_entries: Vec<CatalogueRunEntry>,
    open_select_anchor: Option<String>,
    open_actions_anchor: Option<String>,
    rename: Option<HistoryCenterRename>,
    /// Whether the seeded name is still whole — it is selected on open, so the
    /// next keystroke replaces it.
    rename_is_untouched: bool,
    /// The part the backend has been asked to focus, until the driver has
    /// applied it against the frame that renders it. A part id rather than a
    /// row id: focus also returns to the trigger on close and to the actions
    /// menu after a rename, and neither of those is a row.
    pending_focus: Option<String>,
    trace: Vec<Value>,
}

impl HistoryCenterFixtureHost {
    pub fn new(fixture: &Value) -> Self {
        let spec = history_center_spec_from_fixture(fixture);
        // The mount state is not a transition: a fixture that opens on mount
        // has not emitted an open change, and asserting one would be a lie
        // about what the operator did.
        let state = if spec.open.unwrap_or(spec.default_open) {
            HistoryCenterState::Open
        } else {
            HistoryCenterState::Closed
        };
        let context = HistoryCenterContext {
            pages: spec.pages.clone(),
            ..HistoryCenterContext::default()
        };

        let host = fixture.get("host").cloned().unwrap_or_else(|| json!({}));
        let forks = host
            .get("continuations")
            .and_then(Value::as_array)
            .map(|records| records.iter().map(catalogue_fork).collect())
            .unwrap_or_default();
        let run_entries = host
            .get("runEntries")
            .and_then(Value::as_array)
            .map(|records| {
                records
                    .iter()
                    .map(|record| CatalogueRunEntry {
                        from_entry_id: record
                            .get("fromEntryId")
                            .and_then(Value::as_str)
                            .unwrap_or_default()
                            .to_owned(),
                        entry: history_entry_from_json(record),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let rejection = spec.rejection;
        let mut host = Self {
            spec,
            state,
            context,
            forks,
            run_entries,
            open_select_anchor: None,
            open_actions_anchor: None,
            rename: None,
            rename_is_untouched: true,
            pending_focus: None,
            trace: Vec::new(),
        };
        // A host that was handed a rejection has already had one mapped for
        // it; the machine owns the copy, so it has to be told. The web
        // adapter delivers the same thing through its prop watch.
        if let Some(code) = rejection {
            let code = match code {
                poodle_specs::HistoryCenterRejection::UnknownEntry => {
                    HistoryCenterRejectionCode::UnknownEntry
                }
                poodle_specs::HistoryCenterRejection::AlreadyAtTarget => {
                    HistoryCenterRejectionCode::AlreadyAtTarget
                }
            };
            host.send(HistoryCenterEvent::ShowRejection(code));
        }
        host
    }

    pub fn is_open(&self) -> bool {
        self.state == HistoryCenterState::Open
    }

    pub fn trace(&self) -> Vec<Value> {
        self.trace.clone()
    }

    /// Take the focus the machine asked for, if any.
    pub fn take_pending_focus(&mut self) -> Option<String> {
        self.pending_focus.take()
    }

    /// The view the composition renders: the derived rows plus the state the
    /// machine and the shell own between them.
    pub fn view(&self) -> HistoryCenterView {
        HistoryCenterView {
            is_open: self.is_open(),
            rows: history_center_visible_rows(self.context.pages.as_ref(), &self.context.open),
            focus_row: self.context.focus_row.clone(),
            open_anchors: open_anchors(&self.context.open),
            rejection: self.context.rejection.clone(),
            open_select_anchor: self.open_select_anchor.clone(),
            open_actions_anchor: self.open_actions_anchor.clone(),
            rename: self.rename.clone(),
        }
    }

    // ── Command routing ────────────────────────────────────────────────────

    pub fn send(&mut self, event: HistoryCenterEvent) {
        let result = history_center_transition(self.state, self.context.clone(), event);
        self.apply(result);
    }

    /// Apply a transition and drain its effects, answering the two host
    /// commands as they arrive. Answers are queued rather than recursed, so
    /// the recorded order is the order the operator would see: the request
    /// first, then whatever the answer set off.
    fn apply(&mut self, result: HistoryCenterResult) {
        self.state = result.state;
        self.context = result.context;
        let mut queue: std::collections::VecDeque<HistoryCenterEffect> =
            result.effects.into_iter().collect();

        while let Some(effect) = queue.pop_front() {
            match effect {
                HistoryCenterEffect::EmitOpenChange { open } => {
                    self.record("openChange", json!({ "open": open }));
                    if !open {
                        // Nothing survives a close: the shell's own state goes
                        // with the machine's, and focus returns to the trigger
                        // that opened the surface.
                        self.open_select_anchor = None;
                        self.open_actions_anchor = None;
                        self.rename = None;
                        self.pending_focus = Some("list-trigger".to_owned());
                    }
                }
                // Roving focus is not just a tab stop: the backend has to be
                // asked to move focus, or the next keystroke still reaches
                // the row the operator left.
                HistoryCenterEffect::FocusRow { row } => {
                    let part = match row.kind {
                        // An entry row's focus target is its button; the other
                        // row kinds are focus stops themselves.
                        HistoryCenterRowKind::Entry => format!("entry:{}", row.entry_id),
                        HistoryCenterRowKind::Picker => format!("picker:{}", row.entry_id),
                        HistoryCenterRowKind::NotYetLoaded => {
                            format!("not-yet-loaded:{}", row.entry_id)
                        }
                    };
                    self.pending_focus = Some(part);
                }
                HistoryCenterEffect::EmitNavigateEntry {
                    branch_id,
                    entry_id,
                } => self.record(
                    "navigateEntry",
                    // The spine has no branch. The corpus pins the empty
                    // string so "no branch" is asserted rather than skipped.
                    json!({ "branchId": branch_id.unwrap_or_default(), "entryId": entry_id }),
                ),
                HistoryCenterEffect::EmitRenameBranch { branch_id, name } => {
                    self.record("renameBranch", json!({ "branchId": branch_id, "name": name }))
                }
                HistoryCenterEffect::CheckoutContinuation { entry_id } => {
                    self.record("checkoutContinuation", json!({ "entryId": entry_id }))
                }
                HistoryCenterEffect::LoadContinuations { entry_id } => {
                    self.record("loadContinuations", json!({ "entryId": entry_id }));
                    let continuations = self.continuations_at(&entry_id);
                    let answered = history_center_transition(
                        self.state,
                        self.context.clone(),
                        HistoryCenterEvent::ContinuationsLoaded {
                            entry_id,
                            continuations,
                        },
                    );
                    self.state = answered.state;
                    self.context = answered.context;
                    queue.extend(answered.effects);
                }
                HistoryCenterEffect::LoadContinuationRun { from_entry_id } => {
                    self.record(
                        "loadContinuationRun",
                        json!({ "fromEntryId": from_entry_id }),
                    );
                    let pages = self.run_pages_for(&from_entry_id);
                    let answered = history_center_transition(
                        self.state,
                        self.context.clone(),
                        HistoryCenterEvent::RunLoaded {
                            from_entry_id,
                            pages,
                        },
                    );
                    self.state = answered.state;
                    self.context = answered.context;
                    queue.extend(answered.effects);
                }
            }
        }
    }

    fn record(&mut self, event: &str, payload: Value) {
        self.trace.push(json!({ "event": event, "payload": payload }));
    }

    /// Every continuation at the anchor, the child already on the list
    /// included — filtering that one out by id is the component's job, and a
    /// host that pre-filtered would hide a bug rather than expose one.
    fn continuations_at(&self, anchor_entry_id: &str) -> Vec<HistoryContinuation> {
        self.forks
            .iter()
            .filter(|fork| fork.anchor_entry_id == anchor_entry_id)
            .map(|fork| fork.continuation.clone())
            .collect()
    }

    /// The run starting at a fork's first entry, as one page. Entries are
    /// supplied newest first, which is the order a real page arrives in.
    fn run_pages_for(&self, from_entry_id: &str) -> Vec<HistoryPathPage> {
        let entries: Vec<HistoryEntry> = self
            .run_entries
            .iter()
            .filter(|record| record.from_entry_id == from_entry_id)
            .map(|record| record.entry.clone())
            .collect();
        if entries.is_empty() {
            Vec::new()
        } else {
            vec![HistoryPathPage::new(entries)]
        }
    }

    // ── Shell-owned interactions ───────────────────────────────────────────

    pub fn undo(&mut self) {
        if !self.spec.undo_is_disabled() {
            self.record("undo", json!({}));
        }
    }

    pub fn redo(&mut self) {
        if !self.spec.redo_is_disabled() {
            self.record("redo", json!({}));
        }
    }

    pub fn toggle_open(&mut self) {
        self.send(HistoryCenterEvent::Toggle);
    }

    pub fn close(&mut self) {
        self.send(HistoryCenterEvent::Close);
    }

    pub fn activate_row(&mut self, row: HistoryCenterRowId) {
        self.send(HistoryCenterEvent::ActivateRow(Some(row)));
    }

    pub fn disclose(&mut self, entry_id: &str) {
        self.send(HistoryCenterEvent::Disclose {
            entry_id: entry_id.to_owned(),
        });
    }

    /// The select's listbox is shell state, not machine state: opening it
    /// commits nothing and asks the host for nothing.
    pub fn toggle_select(&mut self, anchor_entry_id: &str) {
        self.open_select_anchor = match self.open_select_anchor.as_deref() {
            Some(open) if open == anchor_entry_id => None,
            _ => Some(anchor_entry_id.to_owned()),
        };
    }

    pub fn pick(&mut self, fork_entry_id: &str) {
        self.open_select_anchor = None;
        self.send(HistoryCenterEvent::PickContinuation {
            entry_id: fork_entry_id.to_owned(),
        });
    }

    pub fn toggle_actions(&mut self, anchor_entry_id: &str) {
        self.open_actions_anchor = match self.open_actions_anchor.as_deref() {
            Some(open) if open == anchor_entry_id => None,
            _ => Some(anchor_entry_id.to_owned()),
        };
    }

    pub fn checkout(&mut self) {
        self.open_actions_anchor = None;
        self.send(HistoryCenterEvent::Confirm);
    }

    /// Open the inline rename on whichever fork the picker currently shows.
    pub fn open_rename(&mut self, anchor_entry_id: &str) {
        self.open_actions_anchor = None;
        let Some(shown) = self.shown_fork_at(anchor_entry_id) else {
            return;
        };
        self.rename = Some(HistoryCenterRename {
            anchor_entry_id: anchor_entry_id.to_owned(),
            branch_id: shown.branch_id.clone(),
            value: shown.display_branch().to_owned(),
        });
        self.rename_is_untouched = true;
    }

    /// Insert content at the caret, one character at a time — the same path a
    /// keystroke takes.
    pub fn insert_rename(&mut self, text: &str) {
        for ch in text.chars() {
            self.append_rename(&ch.to_string());
        }
    }

    /// One keystroke of content. The seeded name is selected when the rename
    /// opens, so the first keystroke replaces it and the rest append — what
    /// typing over a selected field does. The cap is the spec's client-side
    /// affordance and enforces no protocol rule.
    pub fn append_rename(&mut self, text: &str) {
        let cap = self.spec.max_branch_name_bytes;
        let replace = self.rename_is_untouched;
        if let Some(rename) = self.rename.as_mut() {
            if replace {
                rename.value.clear();
            }
            rename.value.push_str(text);
            rename.value = rename.value.chars().take(cap).collect();
            self.rename_is_untouched = false;
        }
    }

    pub fn commit_rename(&mut self) {
        let Some(rename) = self.rename.take() else {
            return;
        };
        self.rename_is_untouched = true;
        self.pending_focus = Some(format!("picker-actions:{}", rename.anchor_entry_id));
        self.send(HistoryCenterEvent::Rename {
            branch_id: rename.branch_id,
            name: rename.value,
        });
    }

    /// Escape cancels without emitting anything at all.
    pub fn cancel_rename(&mut self) {
        if let Some(rename) = self.rename.take() {
            self.pending_focus = Some(format!("picker-actions:{}", rename.anchor_entry_id));
        }
        self.rename_is_untouched = true;
    }

    /// Whether an inline rename is open. Escape belongs to the innermost
    /// thing that can cancel, so a rename claims it before the popover does.
    pub fn is_renaming(&self) -> bool {
        self.rename.is_some()
    }

    pub fn key(&mut self, key: &str) {
        if let Some(event) = history_center_keydown_event(key) {
            self.send(event);
        }
    }

    /// Focus a row by identity, the way a pointer landing on it would.
    pub fn focus_row(&mut self, row: HistoryCenterRowId) {
        self.context.focus_row = Some(row);
    }

    fn shown_fork_at(&self, anchor_entry_id: &str) -> Option<HistoryContinuation> {
        find_level(&self.context.open, anchor_entry_id)
            .and_then(|level| level.shown().cloned())
    }
}

fn catalogue_fork(record: &Value) -> CatalogueFork {
    let mut continuation = HistoryContinuation::new(
        record.get("entryId").and_then(Value::as_str).unwrap_or_default(),
        record.get("label").and_then(Value::as_str).unwrap_or_default(),
        record.get("branchId").and_then(Value::as_str).unwrap_or_default(),
    )
    .with_preferred(
        record
            .get("preferred")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    )
    .with_entry_count(record.get("entryCount").and_then(Value::as_u64).unwrap_or(1) as usize);
    if let Some(name) = record.get("branchName").and_then(Value::as_str) {
        continuation = continuation.with_branch_name(name);
    }
    if let Some(stamp) = record.get("recordedAtMs").and_then(Value::as_u64) {
        continuation = continuation.with_recorded_at_ms(stamp);
    }
    CatalogueFork {
        anchor_entry_id: record
            .get("anchorEntryId")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
        continuation,
    }
}

fn find_level<'a>(
    open: &'a [HistoryCenterOpenFork],
    entry_id: &str,
) -> Option<&'a HistoryCenterOpenFork> {
    for level in open {
        if level.anchor_entry_id == entry_id {
            return Some(level);
        }
        if let Some(found) = find_level(&level.inner, entry_id) {
            return Some(found);
        }
    }
    None
}

fn open_anchors(open: &[HistoryCenterOpenFork]) -> Vec<String> {
    let mut out = Vec::new();
    for level in open {
        out.push(level.anchor_entry_id.clone());
        out.extend(open_anchors(&level.inner));
    }
    out
}

/// The row identity a repeated part id names (`entry:e1` → the entry row `e1`).
pub fn row_id_for_part(part: &str) -> Option<HistoryCenterRowId> {
    let (base, key) = part.split_once(':')?;
    let kind = match base {
        "row" | "entry" | "disclosure" => HistoryCenterRowKind::Entry,
        "picker" | "picker-select" | "picker-actions" | "action-rename" | "action-checkout"
        | "rename-input" => HistoryCenterRowKind::Picker,
        "not-yet-loaded" => HistoryCenterRowKind::NotYetLoaded,
        _ => return None,
    };
    Some(HistoryCenterRowId::new(kind, key))
}

/// The native element id a corpus part id resolves to. The semantic ids the
/// composition stamps are `history-center:<part>`, so the corpus's own part
/// vocabulary is the mapping — no per-part table lives here.
pub fn element_id_for_part(instance_id: &str, part: &str) -> String {
    if part == "root" {
        return instance_id.to_owned();
    }
    format!("{instance_id}:history-center:{part}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(id: &str) -> Value {
        let corpus: Value = serde_json::from_str(super::super::conformance_support::HISTORY_CENTER_CASES)
            .expect("the corpus parses");
        corpus
            .get("cases")
            .and_then(Value::as_array)
            .expect("cases")
            .iter()
            .find(|case| case.get("id").and_then(Value::as_str) == Some(id))
            .and_then(|case| case.get("fixture").cloned())
            .unwrap_or_else(|| panic!("case {id} exists"))
    }

    fn events(host: &HistoryCenterFixtureHost) -> Vec<String> {
        host.trace()
            .iter()
            .filter_map(|entry| {
                entry
                    .get("event")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            })
            .collect()
    }

    /// A fixture that opens on mount has not emitted an open change: the
    /// operator did nothing.
    #[test]
    fn mounting_open_emits_no_command() {
        let host = HistoryCenterFixtureHost::new(&fixture("history-center/linear-rows"));
        assert!(host.is_open());
        assert!(host.trace().is_empty());
    }

    /// The spine renders oldest first with every entry once, at one level.
    #[test]
    fn the_spine_derives_from_the_fixture_pages() {
        let host = HistoryCenterFixtureHost::new(&fixture("history-center/linear-page-seam"));
        let view = host.view();
        let ids: Vec<String> = view.rows.iter().map(|row| row.id().entry_id).collect();
        assert_eq!(ids, ["e1", "e2", "e3", "e4"]);
        assert!(view.rows.iter().all(|row| row.depth() == 0));
    }

    /// Disclosure asks, the host answers, and the answer sets off exactly one
    /// run request — in that order.
    #[test]
    fn the_host_answers_a_disclosure_in_command_order() {
        let mut host =
            HistoryCenterFixtureHost::new(&fixture("history-center/multiple-fork-disclosure"));
        host.disclose("e2");
        assert_eq!(events(&host), ["loadContinuations", "loadContinuationRun"]);
        assert_eq!(
            host.trace()[1]["payload"]["fromEntryId"],
            json!("f1"),
            "the first offered fork shows: the preferred one is the row already on the list",
        );
    }

    /// The host hands back every child of the anchor. Filtering the one
    /// already on the list is the component's job — a host that pre-filtered
    /// would hide the bug instead of exposing it.
    #[test]
    fn the_host_does_not_pre_filter_the_child_on_the_list() {
        let mut host =
            HistoryCenterFixtureHost::new(&fixture("history-center/multiple-fork-disclosure"));
        assert_eq!(
            host.continuations_at("e2").len(),
            3,
            "e3, f1 and f2 all come back from the host",
        );
        host.disclose("e2");
        let offered: Vec<String> = host
            .view()
            .rows
            .iter()
            .find_map(|row| match row {
                poodle_headless::history_center::HistoryCenterRow::Picker {
                    continuations,
                    ..
                } => Some(
                    continuations
                        .iter()
                        .map(|fork| fork.entry_id.clone())
                        .collect(),
                ),
                _ => None,
            })
            .expect("the picker renders");
        assert_eq!(offered, ["f1", "f2"], "e3 is already on the list");
    }

    /// Picking another fork previews it and drops the run that was showing;
    /// nothing is checked out.
    #[test]
    fn picking_another_fork_previews_without_committing() {
        let mut host = HistoryCenterFixtureHost::new(&fixture("history-center/select-other-fork"));
        host.disclose("e2");
        host.toggle_select("e2");
        host.pick("f2");
        assert_eq!(
            events(&host),
            [
                "loadContinuations",
                "loadContinuationRun",
                "loadContinuationRun"
            ],
        );
        let view = host.view();
        let ids: Vec<String> = view.rows.iter().map(|row| row.id().entry_id).collect();
        assert!(ids.contains(&"f2".to_owned()));
        assert!(!ids.contains(&"f1b".to_owned()), "the other run is gone");
    }

    /// Checkout commits the shown fork alone: the run was already previewed
    /// by the pick, and no navigation travels with it.
    #[test]
    fn checkout_commits_the_shown_fork_alone() {
        let mut host =
            HistoryCenterFixtureHost::new(&fixture("history-center/checkout-selected-fork"));
        host.disclose("e2");
        host.toggle_actions("e2");
        host.checkout();
        assert_eq!(
            events(&host),
            [
                "loadContinuations",
                "loadContinuationRun",
                "checkoutContinuation"
            ],
        );
        assert_eq!(host.trace()[2]["payload"]["entryId"], json!("f1"));
    }

    /// Rename commits on the branch the picker shows, and Escape commits
    /// nothing at all.
    #[test]
    fn rename_commits_the_shown_branch_and_escape_commits_nothing() {
        let mut host =
            HistoryCenterFixtureHost::new(&fixture("history-center/rename-selected-branch"));
        host.disclose("e2");
        host.toggle_actions("e2");
        host.open_rename("e2");
        assert_eq!(
            host.view().rename.map(|rename| rename.value),
            Some("Wide mix".to_owned()),
            "the input is seeded with the shown fork's current name",
        );
        host.insert_rename("Wide mix v2");
        host.commit_rename();
        assert_eq!(
            events(&host),
            ["loadContinuations", "loadContinuationRun", "renameBranch"],
        );
        assert_eq!(host.trace()[2]["payload"]["branchId"], json!("branch-wide"));
        assert_eq!(host.trace()[2]["payload"]["name"], json!("Wide mix v2"));

        let mut cancelled =
            HistoryCenterFixtureHost::new(&fixture("history-center/rename-escape-cancels"));
        cancelled.disclose("e2");
        cancelled.toggle_actions("e2");
        cancelled.open_rename("e2");
        cancelled.insert_rename("Discarded");
        cancelled.cancel_rename();
        assert_eq!(
            events(&cancelled),
            ["loadContinuations", "loadContinuationRun"],
        );
        assert!(cancelled.is_open(), "the surface stays open");
    }

    /// A row reports its own branch and its own entry.
    #[test]
    fn a_fork_run_row_navigates_on_its_own_branch() {
        let mut host =
            HistoryCenterFixtureHost::new(&fixture("history-center/fork-run-navigation"));
        host.disclose("e2");
        host.activate_row(HistoryCenterRowId::new(HistoryCenterRowKind::Entry, "f1b"));
        let last = host.trace().pop().expect("a command left");
        assert_eq!(last["event"], json!("navigateEntry"));
        assert_eq!(last["payload"]["branchId"], json!("branch-wide"));
        assert_eq!(last["payload"]["entryId"], json!("f1b"));
    }

    /// Disabled triggers are inert, not merely dimmed.
    #[test]
    fn busy_triggers_emit_nothing() {
        let mut host =
            HistoryCenterFixtureHost::new(&fixture("history-center/busy-disables-commands"));
        host.undo();
        host.redo();
        assert!(host.trace().is_empty());
    }

    /// Three disclosures, four levels, nothing saturated.
    #[test]
    fn depth_is_never_clamped() {
        let mut host =
            HistoryCenterFixtureHost::new(&fixture("history-center/deep-depth-uncapped"));
        host.disclose("e2");
        host.disclose("f1b");
        host.disclose("f1b1");
        let view = host.view();
        let depth_of = |id: &str| {
            view.rows
                .iter()
                .find(|row| row.id().entry_id == id)
                .map(|row| row.depth())
        };
        assert_eq!(depth_of("e2"), Some(0));
        assert_eq!(depth_of("f1b"), Some(1));
        assert_eq!(depth_of("f1b1"), Some(2));
        assert_eq!(depth_of("f1b1a"), Some(3));

        // Closing the outermost level drops every level inside it.
        host.disclose("e2");
        let view = host.view();
        assert!(view.rows.iter().all(|row| row.depth() == 0));
    }

    /// Part ids resolve to the composition's own semantic ids without a
    /// per-part table.
    #[test]
    fn part_ids_map_onto_the_compositions_semantic_ids() {
        assert_eq!(
            element_id_for_part("case", "entry:e1"),
            "case:history-center:entry:e1",
        );
        assert_eq!(
            element_id_for_part("case", "list-trigger"),
            "case:history-center:list-trigger",
        );
        assert_eq!(element_id_for_part("case", "root"), "case");
        assert_eq!(
            row_id_for_part("disclosure:e2"),
            Some(HistoryCenterRowId::new(HistoryCenterRowKind::Entry, "e2")),
        );
    }
}

// ── Driver ─────────────────────────────────────────────────────────────────

use std::sync::{Arc, Mutex};

use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::conformance::{
    assert_events, assert_part, expected_events, observe_tree_with_context, InterfaceDoc,
    ObserveContext,
};
use poodle_render::{history_center, HistoryCenterHandlers};

use super::conformance_button::CaseOutcome;
use super::conformance_driver::HeadlessDriver;

/// What a rendered control asked the host to do. The composition's handlers
/// push these; the driver drains them after every real input, so a click that
/// reaches nothing stays observably inert.
#[derive(Clone, Debug)]
enum Intent {
    Undo,
    Redo,
    OpenChange(bool),
    ActivateRow(HistoryCenterRowId),
    Disclose(String),
    Pick(String),
    ToggleSelect(String),
    ToggleActions(String),
    Checkout,
    OpenRename(String),
    RenameKey(String),
    RenameInsert(String),
    RowKey(poodle_node::NodeKey),
    Dismiss,
}

struct CaseHost {
    host: HistoryCenterFixtureHost,
    node: Arc<Mutex<Node>>,
    intents: Arc<Mutex<Vec<Intent>>>,
    instance_id: String,
    theme: GpuiThemeProvider,
}

impl CaseHost {
    fn rebuild(&mut self) {
        let handlers = self.handlers();
        let node = history_center(
            &self.host.spec,
            &self.theme,
            &self.host.view(),
            &handlers,
        );
        *self.node.lock().expect("node lock") = node;
    }

    fn handlers(&self) -> HistoryCenterHandlers {
        let push = |intents: &Arc<Mutex<Vec<Intent>>>| Arc::clone(intents);
        let queue = push(&self.intents);
        let undo = Arc::clone(&queue);
        let redo = Arc::clone(&queue);
        let open = Arc::clone(&queue);
        let activate = Arc::clone(&queue);
        let disclose = Arc::clone(&queue);
        let pick = Arc::clone(&queue);
        let select = Arc::clone(&queue);
        let actions = Arc::clone(&queue);
        let checkout = Arc::clone(&queue);
        let rename_open = Arc::clone(&queue);
        let rename_key = Arc::clone(&queue);
        let rename_insert = Arc::clone(&queue);
        let row_key = Arc::clone(&queue);
        let dismiss = Arc::clone(&queue);

        HistoryCenterHandlers {
            on_undo: Some(Arc::new(move || {
                undo.lock().expect("intents").push(Intent::Undo)
            })),
            on_redo: Some(Arc::new(move || {
                redo.lock().expect("intents").push(Intent::Redo)
            })),
            on_open_change: Some(Arc::new(move |next| {
                open.lock().expect("intents").push(Intent::OpenChange(next))
            })),
            on_activate_row: Some(Arc::new(move |row| {
                activate
                    .lock()
                    .expect("intents")
                    .push(Intent::ActivateRow(row.clone()))
            })),
            on_disclose: Some(Arc::new(move |entry_id| {
                disclose
                    .lock()
                    .expect("intents")
                    .push(Intent::Disclose(entry_id.to_owned()))
            })),
            on_pick: Some(Arc::new(move |entry_id| {
                pick.lock()
                    .expect("intents")
                    .push(Intent::Pick(entry_id.to_owned()))
            })),
            on_select_toggle: Some(Arc::new(move |anchor| {
                select
                    .lock()
                    .expect("intents")
                    .push(Intent::ToggleSelect(anchor.to_owned()))
            })),
            on_actions_toggle: Some(Arc::new(move |anchor| {
                actions
                    .lock()
                    .expect("intents")
                    .push(Intent::ToggleActions(anchor.to_owned()))
            })),
            on_checkout: Some(Arc::new(move |_anchor| {
                checkout.lock().expect("intents").push(Intent::Checkout)
            })),
            on_rename_open: Some(Arc::new(move |anchor| {
                rename_open
                    .lock()
                    .expect("intents")
                    .push(Intent::OpenRename(anchor.to_owned()))
            })),
            on_rename_key: Some(Arc::new(move |key| {
                rename_key
                    .lock()
                    .expect("intents")
                    .push(Intent::RenameKey(key.to_owned()))
            })),
            on_rename_insert: Some(Arc::new(move |text| {
                rename_insert
                    .lock()
                    .expect("intents")
                    .push(Intent::RenameInsert(text.to_owned()))
            })),
            on_row_key: Some(Arc::new(move |key| {
                row_key.lock().expect("intents").push(Intent::RowKey(key))
            })),
            on_dismiss: Some(Arc::new(move |_reason| {
                dismiss.lock().expect("intents").push(Intent::Dismiss)
            })),
            instance_id: Some(self.instance_id.clone()),
        }
    }

    /// Drain whatever the last real input reached. Nothing is applied that a
    /// control did not ask for.
    fn drain(&mut self) {
        let intents = std::mem::take(&mut *self.intents.lock().expect("intents"));
        // Escape belongs to the innermost thing that can cancel. The rename
        // input and the window's dismiss route both see the same keystroke,
        // so the decision reads the state as it was when the key landed —
        // otherwise the rename cancels first and the popover then closes
        // because it no longer sees one.
        let was_renaming = self.host.is_renaming();
        for intent in intents {
            match intent {
                Intent::Undo => self.host.undo(),
                Intent::Redo => self.host.redo(),
                Intent::OpenChange(_) => self.host.toggle_open(),
                Intent::ActivateRow(row) => self.host.activate_row(row),
                Intent::Disclose(entry_id) => self.host.disclose(&entry_id),
                Intent::Pick(entry_id) => self.host.pick(&entry_id),
                Intent::ToggleSelect(anchor) => self.host.toggle_select(&anchor),
                Intent::ToggleActions(anchor) => self.host.toggle_actions(&anchor),
                Intent::Checkout => self.host.checkout(),
                Intent::OpenRename(anchor) => self.host.open_rename(&anchor),
                Intent::RenameInsert(text) => self.host.insert_rename(&text),
                Intent::RowKey(key) => self.host.key(match key {
                    poodle_node::NodeKey::ArrowDown => "ArrowDown",
                    poodle_node::NodeKey::ArrowUp => "ArrowUp",
                    poodle_node::NodeKey::Home => "Home",
                    poodle_node::NodeKey::End => "End",
                    poodle_node::NodeKey::Space => " ",
                    _ => return,
                }),
                Intent::RenameKey(key) => match key.as_str() {
                    // Enter commits, Escape cancels without emitting. Every
                    // other keystroke is content the host appends.
                    "enter" => self.host.commit_rename(),
                    "escape" => self.host.cancel_rename(),
                    other => self.host.append_rename(other),
                },
                Intent::Dismiss => {
                    if was_renaming {
                        self.host.cancel_rename();
                    } else {
                        self.host.close();
                    }
                }
            }
        }
        self.rebuild();
    }
}

fn observe_case(host: &CaseHost, iface: &InterfaceDoc) -> Value {
    let node = host.node.lock().expect("node lock").clone();
    let focus_by_id = |id: &str| poodle_gpui_node_backend::focus_state_for(id);
    let layer_count = || Some(poodle_gpui_node_backend::open_layer_count());
    let bounds_by_id = |id: &str| {
        poodle_gpui_node_backend::bounds_for(id).map(|bounds| {
            (
                f32::from(bounds.origin.y),
                f32::from(bounds.origin.x),
                f32::from(bounds.size.width),
                f32::from(bounds.size.height),
            )
        })
    };
    let mut observation = observe_tree_with_context(
        "gpui",
        "history-center",
        iface,
        &node,
        &ObserveContext {
            focus_by_id: &focus_by_id,
            layer_count: &layer_count,
            bounds_by_id: &bounds_by_id,
        },
    );
    observation["trace"] = json!(host.host.trace());
    observation
}

fn gpui_key(key: &str) -> Option<&'static str> {
    Some(match key {
        "ArrowDown" => "down",
        "ArrowUp" => "up",
        "ArrowRight" => "right",
        "ArrowLeft" => "left",
        "Home" => "home",
        "End" => "end",
        "Enter" => "enter",
        "Escape" => "escape",
        _ => return None,
    })
}

pub fn drive_history_center_cases(
    driver: &mut HeadlessDriver<'_>,
    iface: InterfaceDoc,
    cases: Vec<Value>,
    only: Option<String>,
) -> Vec<CaseOutcome> {
    let mut outcomes = Vec::new();
    for case in &cases {
        let case_id = case
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("?")
            .to_owned();
        if only.as_deref().is_some_and(|only| only != case_id.as_str()) {
            continue;
        }
        let fixture = case.get("fixture").cloned().unwrap_or_else(|| json!({}));
        let steps = case
            .get("steps")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let node = Arc::new(Mutex::new(Node::container()));
        let instance_id = format!("conformance-{case_id}");
        let mut host = CaseHost {
            host: HistoryCenterFixtureHost::new(&fixture),
            node: Arc::clone(&node),
            intents: Arc::new(Mutex::new(Vec::new())),
            instance_id: instance_id.clone(),
            theme: GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE),
        };
        host.rebuild();
        driver.mount_node(Arc::clone(&node));
        driver.draw_frame();

        let mut failures = Vec::new();
        let mut assertions = Vec::new();
        let mut observations = vec![observe_case(&host, &iface)];

        for (index, step) in steps.iter().enumerate() {
            match step.get("kind").and_then(Value::as_str).unwrap_or("") {
                "action" => {
                    let name = step.get("name").and_then(Value::as_str).unwrap_or("");
                    let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                    let element = element_id_for_part(&instance_id, part);
                    match name {
                        // Activation goes through the window's real dispatch
                        // tree: focus the element, then confirm. Hit testing a
                        // row nested inside a scrolling overlay would test the
                        // harness's arithmetic rather than the component.
                        // A press on an element the composition never rendered
                        // is a silent no-op otherwise, which would let an
                        // inert control pass by doing nothing at all.
                        "press" => {
                            driver.wait_for_focus_handle(&element);
                            driver.keyboard_activate(&element);
                        }
                        "focus" => {
                            if let Some(row) = row_id_for_part(part) {
                                host.host.focus_row(row);
                                host.rebuild();
                                driver.draw_frame();
                            }
                            driver.wait_for_focus_handle(&element);
                            driver.focus_element(&element);
                        }
                        "key" => {
                            let key = step.get("key").and_then(Value::as_str).unwrap_or("");
                            if let Some(key) = gpui_key(key) {
                                driver.keyboard_key(&element, key);
                            }
                        }
                        // The real dismissal route: Escape through the
                        // window, which the overlay host routes to the layer.
                        "dismiss" => driver.dispatch_key("escape"),
                        "insert" => {
                            let text = step.get("text").and_then(Value::as_str).unwrap_or("");
                            driver.wait_for_focus_handle(&element);
                            for ch in text.chars() {
                                let stroke = if ch.is_uppercase() {
                                    format!("shift-{}", ch.to_lowercase())
                                } else if ch == ' ' {
                                    "space".to_owned()
                                } else {
                                    ch.to_string()
                                };
                                driver.keyboard_key(&element, &stroke);
                                host.drain();
                                driver.draw_frame();
                            }
                        }
                        _ => {}
                    }
                    host.drain();
                    driver.draw_frame();
                    // Apply whatever focus the machine asked for, against the
                    // frame that now renders the row.
                    if let Some(part) = host.host.take_pending_focus() {
                        let target = element_id_for_part(&host.instance_id, &part);
                        driver.wait_for_focus_handle(&target);
                        driver.focus_element(&target);
                    }
                    observations.push(observe_case(&host, &iface));
                }
                "expectPart" => {
                    let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                    let expect = step.get("expect").cloned().unwrap_or(Value::Null);
                    let mut results = Vec::new();
                    assert_part(
                        &iface,
                        part,
                        &expect,
                        index,
                        observe_case(&host, &iface),
                        "gpui",
                        &mut results,
                    );
                    collect(results, &mut failures, &mut assertions);
                }
                "expectEvents" => {
                    let mut results = Vec::new();
                    assert_events(
                        &expected_events(step),
                        &host.host.trace(),
                        index,
                        &mut results,
                    );
                    collect(results, &mut failures, &mut assertions);
                }
                _ => {}
            }
        }

        driver.drain();
        outcomes.push(CaseOutcome {
            case_id,
            pass: failures.is_empty(),
            failures,
            assertions,
            observations,
        });
    }
    outcomes
}

fn collect(
    results: Vec<poodle_render::conformance::AssertionResult>,
    failures: &mut Vec<Value>,
    assertions: &mut Vec<Value>,
) {
    for result in results {
        let value = serde_json::to_value(&result).expect("result serializes");
        if result.verdict == "fail" {
            failures.push(value.clone());
        }
        assertions.push(value);
    }
}

pub fn history_center_report(component: &str, outcomes: &[CaseOutcome]) -> Value {
    json!({
        "runtime": "gpui",
        "component": component,
        "results": outcomes.iter().map(|outcome| json!({
            "caseId": outcome.case_id,
            "pass": outcome.pass,
            "failures": outcome.failures,
            "assertions": outcome.assertions,
            "observations": outcome.observations,
        })).collect::<Vec<_>>(),
    })
}

#[cfg(test)]
mod planted {
    //! Planted defects (g14.007). Each one is a change a reviewer could
    //! plausibly wave through, and each has to fail the corpus's own
    //! machinery — otherwise a passing board proves nothing.

    use super::*;
    use poodle_gpui::GpuiThemeProvider;
    use poodle_headless::history_center::{HistoryCenterOpenFork, HistoryPathPage};
    use poodle_node::{LayoutOverflow, Node, NodeRole};
    use poodle_render::conformance::{assert_events, assert_part, observe_tree, InterfaceDoc};
    use poodle_render::history_center::{history_center_disclosure_id, history_center_entry_id};
    use poodle_render::{history_center, HistoryCenterHandlers};

    fn iface() -> InterfaceDoc {
        InterfaceDoc::parse(
            &serde_json::from_str::<Value>(super::super::conformance_support::HISTORY_CENTER_INTERFACE)
                .expect("interface parses"),
        )
        .expect("interface loads")
    }

    fn theme() -> GpuiThemeProvider {
        GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn entry(id: &str, count: usize) -> HistoryEntry {
        HistoryEntry::new(id, format!("Entry {id}")).with_continuation_count(count)
    }

    fn open_view(rows: Vec<poodle_headless::history_center::HistoryCenterRow>) -> HistoryCenterView {
        HistoryCenterView {
            is_open: true,
            rows,
            ..HistoryCenterView::default()
        }
    }

    fn spine() -> Vec<HistoryPathPage> {
        vec![HistoryPathPage::new(vec![
            entry("e3", 0),
            entry("e2", 3),
            entry("e1", 1),
        ])]
    }

    fn rendered(open: &[HistoryCenterOpenFork]) -> Node {
        let rows = history_center_visible_rows(Some(&spine()), open);
        history_center(
            &HistoryCenterSpec::new(),
            &theme(),
            &open_view(rows),
            &HistoryCenterHandlers::default(),
        )
    }

    fn failures(part: &str, expect: Value, node: &Node) -> Vec<String> {
        let observation = observe_tree("gpui", "history-center", &iface(), node, Some(false));
        let mut out = Vec::new();
        assert_part(&iface(), part, &expect, 0, observation, "gpui", &mut out);
        out.into_iter()
            .filter(|r| r.verdict == "fail")
            .map(|r| r.field)
            .collect()
    }

    /// A run rendered at its anchor's level instead of one below it. The rows
    /// are all present and the list looks right; only the hierarchy is a lie.
    #[test]
    fn a_flattened_hierarchy_fails() {
        let level = HistoryCenterOpenFork {
            anchor_entry_id: "e2".to_owned(),
            continuations: Some(vec![HistoryContinuation::new("f1", "Widen", "wide")]),
            pick: Some(HistoryContinuation::new("f1", "Widen", "wide")),
            chosen: None,
            run_pages: vec![HistoryPathPage::new(vec![entry("f1", 0)])],
            inner: Vec::new(),
        };
        let node = rendered(&[level]);
        // The corpus asserts level 2 for a disclosed run; claiming level 1
        // — the spine's level — has to fail.
        assert_eq!(failures("row:f1", json!({ "level": 1 }), &node), ["level"]);
        assert!(failures("row:f1", json!({ "level": 2 }), &node).is_empty());
    }

    /// A navigate command carrying the anchor's branch rather than the row's
    /// own. The command fires, the name is right, and the operator lands on
    /// the wrong branch.
    #[test]
    fn a_wrong_command_payload_fails() {
        let trace = vec![json!({
            "event": "navigateEntry",
            "payload": { "branchId": "main", "entryId": "f1b" },
        })];
        let expected = vec![json!({
            "name": "navigateEntry",
            "payload": { "branchId": "branch-wide", "entryId": "f1b" },
        })];
        let mut out = Vec::new();
        assert_events(&expected, &trace, 0, &mut out);
        assert_eq!(out[0].verdict, "fail");

        // The same trace passes a name-only expectation, which is exactly why
        // the payload form exists.
        let mut names = Vec::new();
        assert_events(&[json!("navigateEntry")], &trace, 0, &mut names);
        assert_eq!(names[0].verdict, "pass");
    }

    /// A row that renders but cannot be reached: the entry button lost its
    /// focusability, so the keyboard walks straight past it.
    #[test]
    fn an_unfocusable_row_fails() {
        let mut node = rendered(&[]);
        let target = history_center_entry_id("e2");
        fn strip(node: &mut Node, id: &str) {
            if node.id.as_deref() == Some(id) {
                node.interaction.focusable = false;
            }
            for child in &mut node.children {
                strip(child, id);
            }
        }
        strip(&mut node, &target);
        assert_eq!(
            failures("entry:e2", json!({ "focusable": true }), &node),
            ["focusable"],
        );
    }

    /// A list that grows with the history instead of scrolling inside its own
    /// bounds. Nothing is missing; the surface simply runs off the screen.
    #[test]
    fn an_unbounded_list_fails() {
        let mut node = rendered(&[]);
        fn unbound(node: &mut Node, id: &str) {
            if node.id.as_deref() == Some(id) {
                node.style.descriptor.layout.overflow_y = LayoutOverflow::Visible;
                node.style.max_height = None;
            }
            for child in &mut node.children {
                unbound(child, id);
            }
        }
        unbound(&mut node, "history-center:list");
        assert_eq!(
            failures("list", json!({ "scrollable": true }), &node),
            ["scrollable"],
        );
    }

    /// A disclosure bound to nothing: it renders, it is focusable, it carries
    /// the right label and expanded state, and pressing it asks the host for
    /// nothing at all.
    #[test]
    fn an_inert_disclosure_fails() {
        let node = rendered(&[]);
        let disclosure = node
            .find(&|candidate| candidate.id.as_deref() == Some(&history_center_disclosure_id("e2")))
            .expect("the disclosure renders");
        // The composition was handed no handlers, so nothing is bound.
        assert!(disclosure.interaction.on_activate.is_none());
        assert_eq!(disclosure.a11y.role, Some(NodeRole::Button));

        // The corpus catches it through the trace: a press that reaches an
        // unbound control leaves no command behind.
        let mut out = Vec::new();
        assert_events(
            &[json!({ "name": "loadContinuations", "payload": { "entryId": "e2" } })],
            &[],
            0,
            &mut out,
        );
        assert_eq!(out[0].verdict, "fail");
    }
}
