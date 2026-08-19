//! HistoryCenter behaviour core. Mirror of core `history-center.ts`.
//!
//! A flat visible-row derivation over path pages plus the open forks, and the
//! machine that owns popover state, roving focus by row identity, the
//! disclosure tree and transient rejection display.
//!
//! Three rules carry the design, and every renderer depends on all three:
//!
//! - **Rows are flat and carry their topology as data.** Each row has a depth
//!   number, the entry it hangs off, and the fork it belongs to. No renderer
//!   recurses; core knows the topology, which is core's job. Two forks at one
//!   entry are never confusable with a fork off a fork, at any depth, and
//!   there is no depth cap.
//! - **Display order is reversed exactly once.** Pages arrive newest-first
//!   and display oldest-first, so joining in fetch order puts history
//!   backwards. Every level — the root and each nested run — reverses through
//!   the same join. `continuations` is not reversed: it is a picker, not a
//!   timeline.
//! - **Core holds only what is open.** Closing a level drops its
//!   continuations, pick, chosen fork, run pages and everything inner.
//!   Nothing is cached across a close and reopen.
//!
//! The record types are structural mirrors of the authority's shapes. Poodle
//! imports no authority type and no manifest gains one; the dependency runs
//! the other way. `checkout` is Poodle's own word for what a host maps onto
//! its own prefer operation.

/// Where an entry sits relative to the current position.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum HistoryEntryPosition {
    #[default]
    Past,
    Current,
    Future,
}

impl HistoryEntryPosition {
    /// The portable spelling, which is also what the renderer projects and
    /// the conformance corpus asserts. An unknown value is `Past` rather than
    /// a panic: the record shape is a structural mirror of an authority Poodle
    /// does not own, and a history that renders is better than one that dies.
    pub fn from_portable(value: &str) -> Self {
        match value {
            "current" => Self::Current,
            "future" => Self::Future,
            _ => Self::Past,
        }
    }

    pub fn as_portable(self) -> &'static str {
        match self {
            Self::Past => "past",
            Self::Current => "current",
            Self::Future => "future",
        }
    }
}

/// One history entry — a structural mirror of the authority's entry record.
/// Poodle imports no authority type and no manifest gains one; the dependency
/// runs the other way.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoryEntry {
    pub id: String,
    pub label: String,
    pub position: HistoryEntryPosition,
    /// Renders as a named pin.
    pub is_checkpoint: bool,
    pub group_id: Option<String>,
    /// Authority-supplied milliseconds since the epoch; absent renders
    /// nothing. Never invented here — this crate reads no clock. Integral,
    /// because a millisecond stamp is a count and not a measurement.
    pub recorded_at_ms: Option<u64>,
    /// Every entry continuing from this one, the run's own next row included.
    /// A fork count is one less; a run's last entry is always 0.
    pub continuation_count: usize,
}

impl HistoryEntry {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            position: HistoryEntryPosition::Past,
            is_checkpoint: false,
            group_id: None,
            recorded_at_ms: None,
            continuation_count: 0,
        }
    }

    pub fn with_position(mut self, position: HistoryEntryPosition) -> Self {
        self.position = position;
        self
    }

    pub fn with_checkpoint(mut self, is_checkpoint: bool) -> Self {
        self.is_checkpoint = is_checkpoint;
        self
    }

    pub fn with_group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    pub fn with_recorded_at_ms(mut self, recorded_at_ms: u64) -> Self {
        self.recorded_at_ms = Some(recorded_at_ms);
        self
    }

    pub fn with_continuation_count(mut self, continuation_count: usize) -> Self {
        self.continuation_count = continuation_count;
        self
    }

    /// Forks at this entry: one less than its continuation count, floored at
    /// zero. A run's terminal entry has no children at all, so the floor is
    /// what keeps the unsaturated form from yielding a negative count.
    pub fn fork_count(&self) -> usize {
        self.continuation_count.saturating_sub(1)
    }
}

/// One bounded page of a path — the root path or a continuation run — newest
/// first. `offset` counts from the newest entry, so a page at a higher offset
/// holds older entries.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct HistoryPathPage {
    pub entries: Vec<HistoryEntry>,
    pub offset: usize,
    /// Continuations at the position directly above this page's first entry,
    /// that entry included. Carried for the host and the renderer; the row
    /// derivation emits no row for it.
    pub preceding_continuation_count: usize,
    /// Newer records precede this page.
    pub is_truncated_before: bool,
    /// Older records follow this page.
    pub is_truncated_after: bool,
}

impl HistoryPathPage {
    pub fn new(entries: Vec<HistoryEntry>) -> Self {
        Self {
            entries,
            ..Self::default()
        }
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_preceding_continuation_count(mut self, count: usize) -> Self {
        self.preceding_continuation_count = count;
        self
    }

    pub fn with_truncated_before(mut self, is_truncated_before: bool) -> Self {
        self.is_truncated_before = is_truncated_before;
        self
    }

    pub fn with_truncated_after(mut self, is_truncated_after: bool) -> Self {
        self.is_truncated_after = is_truncated_after;
        self
    }
}

/// One continuation at an anchor: the operator's fork. `entry_id` is the run's
/// first entry — the continuation's stable identity, never a list index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoryContinuation {
    pub entry_id: String,
    pub label: String,
    pub recorded_at_ms: Option<u64>,
    /// Whether a redo from the anchor takes this continuation.
    pub is_preferred: bool,
    /// Entries in the run starting here, this one included.
    pub entry_count: usize,
    /// The branch a consumer lands on by taking this continuation.
    pub branch_id: String,
    pub branch_name: Option<String>,
}

impl HistoryContinuation {
    pub fn new(
        entry_id: impl Into<String>,
        label: impl Into<String>,
        branch_id: impl Into<String>,
    ) -> Self {
        Self {
            entry_id: entry_id.into(),
            label: label.into(),
            recorded_at_ms: None,
            is_preferred: false,
            entry_count: 1,
            branch_id: branch_id.into(),
            branch_name: None,
        }
    }

    pub fn with_preferred(mut self, is_preferred: bool) -> Self {
        self.is_preferred = is_preferred;
        self
    }

    pub fn with_entry_count(mut self, entry_count: usize) -> Self {
        self.entry_count = entry_count;
        self
    }

    pub fn with_branch_name(mut self, branch_name: impl Into<String>) -> Self {
        self.branch_name = Some(branch_name.into());
        self
    }

    pub fn with_recorded_at_ms(mut self, recorded_at_ms: u64) -> Self {
        self.recorded_at_ms = Some(recorded_at_ms);
        self
    }

    /// What the picker shows for this fork: the branch's name when it has one,
    /// else its id. A branch without a name is still a branch an operator has
    /// to choose between.
    pub fn display_branch(&self) -> &str {
        self.branch_name.as_deref().unwrap_or(&self.branch_id)
    }
}


// ── Visible-row model ──────────────────────────────────────────────────────

/// What kind of row an identity names. Focus is keyed by identity and never
/// by array position: a disclosure toggle changes the list shape underneath an
/// index, which is the bug this model exists to avoid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoryCenterRowKind {
    Entry,
    Picker,
    NotYetLoaded,
}

/// Stable identity of one visible row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoryCenterRowId {
    pub kind: HistoryCenterRowKind,
    pub entry_id: String,
}

impl HistoryCenterRowId {
    pub fn new(kind: HistoryCenterRowKind, entry_id: impl Into<String>) -> Self {
        Self {
            kind,
            entry_id: entry_id.into(),
        }
    }
}

/// One row of the flat visible list. `depth`, `parent_entry_id` and `fork_id`
/// travel as identifiers, not as indentation.
#[derive(Clone, Debug, PartialEq)]
pub enum HistoryCenterRow {
    Entry {
        entry: HistoryEntry,
        depth: usize,
        parent_entry_id: Option<String>,
        /// The run's first entry; `None` on the spine — the trunk is not a fork.
        fork_id: Option<String>,
        /// The branch this row's run lands on; `None` on the spine, where the
        /// host knows its own branch.
        branch_id: Option<String>,
        fork_count: usize,
    },
    Picker {
        anchor_entry_id: String,
        depth: usize,
        parent_entry_id: Option<String>,
        /// The forks at the anchor, the child already on the list filtered
        /// out. Empty while the level's continuations are in flight.
        continuations: Vec<HistoryContinuation>,
        /// The select's value: the tentative pick, else the auto-chosen single
        /// fork. `None` only while continuations are loading.
        picked_entry_id: Option<String>,
        /// True when the level has exactly one fork — nothing to choose
        /// between. It governs the select alone: the actions menu never
        /// inherits it, because the auto-chosen fork still counts as picked.
        is_disabled: bool,
    },
    NotYetLoaded {
        anchor_entry_id: String,
        depth: usize,
        parent_entry_id: Option<String>,
        fork_id: Option<String>,
        branch_id: Option<String>,
    },
}

impl HistoryCenterRow {
    pub fn id(&self) -> HistoryCenterRowId {
        match self {
            Self::Entry { entry, .. } => {
                HistoryCenterRowId::new(HistoryCenterRowKind::Entry, entry.id.clone())
            }
            Self::Picker {
                anchor_entry_id, ..
            } => HistoryCenterRowId::new(HistoryCenterRowKind::Picker, anchor_entry_id.clone()),
            Self::NotYetLoaded {
                anchor_entry_id, ..
            } => {
                HistoryCenterRowId::new(HistoryCenterRowKind::NotYetLoaded, anchor_entry_id.clone())
            }
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            Self::Entry { depth, .. }
            | Self::Picker { depth, .. }
            | Self::NotYetLoaded { depth, .. } => *depth,
        }
    }
}

/// One level of the disclosure tree: a fork open at an entry. Levels nest
/// because forks fork. A `Vec` rather than a map: insertion order is the
/// display order the TypeScript `Map` also preserves, and a level carries its
/// own anchor id, so there is no key to keep in step with the value.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HistoryCenterOpenFork {
    pub anchor_entry_id: String,
    /// Forks at the anchor in supplied order; `None` until loaded. The child
    /// already on the list is filtered by the derivation, never here.
    pub continuations: Option<Vec<HistoryContinuation>>,
    /// The tentatively picked fork. One tentative pick at a time across levels.
    pub pick: Option<HistoryContinuation>,
    /// The auto-chosen single fork. The multi-fork picker never commits one:
    /// checkout clears the level and the host supplies the new root.
    pub chosen: Option<HistoryContinuation>,
    /// The displayed fork's run pages in fetch order; empty until loaded.
    pub run_pages: Vec<HistoryPathPage>,
    pub inner: Vec<HistoryCenterOpenFork>,
}

impl HistoryCenterOpenFork {
    pub fn opening(anchor_entry_id: impl Into<String>) -> Self {
        Self {
            anchor_entry_id: anchor_entry_id.into(),
            ..Self::default()
        }
    }

    /// The fork whose run renders below the select: the tentative pick, else
    /// the auto-chosen single fork.
    pub fn shown(&self) -> Option<&HistoryContinuation> {
        self.pick.as_ref().or(self.chosen.as_ref())
    }
}

// ── Derivation ─────────────────────────────────────────────────────────────

/// Forks at an entry: one less than its continuation count, floored at zero.
///
/// The floor matters. A run's terminal entry has no children at all, so its
/// continuation count is zero and the unsaturated form would yield -1.
pub fn history_center_fork_count(continuation_count: usize) -> usize {
    continuation_count.saturating_sub(1)
}

/// Join path pages — the root or a run — into display order, oldest first.
///
/// Pages arrive newest-first in fetch order, so the later-fetched (older) page
/// renders before the first page. This is the only reversal and every level
/// reverses through it. Overlapping page seams dedupe by entry id; entries are
/// immutable graph nodes, so an id names exactly one entry.
pub fn history_center_join_pages(pages: &[HistoryPathPage]) -> Vec<HistoryEntry> {
    let mut newest_first: Vec<HistoryEntry> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for page in pages {
        for entry in &page.entries {
            if seen.contains(&entry.id.as_str()) {
                continue;
            }
            seen.push(&entry.id);
            newest_first.push(entry.clone());
        }
    }
    newest_first.reverse();
    newest_first
}

/// The forks at an anchor: the loaded continuations minus the child already on
/// the list. That child is the anchor's successor in the run — filtered by id,
/// never by array position. When paging truncated the successor away, the
/// preferred flag identifies the same record, because a run follows preferred
/// children.
pub fn history_center_forks_at(
    continuations: Option<&Vec<HistoryContinuation>>,
    run_entries: &[HistoryEntry],
    anchor_index: usize,
) -> Vec<HistoryContinuation> {
    let Some(continuations) = continuations else {
        return Vec::new();
    };
    let own_id = run_entries.get(anchor_index + 1).map(|entry| entry.id.as_str());
    continuations
        .iter()
        .filter(|continuation| match own_id {
            None => !continuation.is_preferred,
            Some(own_id) => continuation.entry_id != own_id,
        })
        .cloned()
        .collect()
}

/// The visible-row derivation: one flat array of rows in display order over
/// the root path pages plus the open forks.
pub fn history_center_visible_rows(
    pages: Option<&Vec<HistoryPathPage>>,
    open: &[HistoryCenterOpenFork],
) -> Vec<HistoryCenterRow> {
    let mut rows = Vec::new();
    let Some(pages) = pages else {
        return rows;
    };
    if pages.is_empty() {
        return rows;
    }
    let root_entries = history_center_join_pages(pages);
    push_run(
        &mut rows,
        &root_entries,
        0,
        None,
        None,
        None,
        open,
        &root_entries,
    );
    rows
}

#[allow(clippy::too_many_arguments)]
fn push_run(
    rows: &mut Vec<HistoryCenterRow>,
    entries: &[HistoryEntry],
    depth: usize,
    parent_of_first: Option<&str>,
    fork_id: Option<&str>,
    branch_id: Option<&str>,
    open: &[HistoryCenterOpenFork],
    root_entries: &[HistoryEntry],
) {
    for (index, entry) in entries.iter().enumerate() {
        rows.push(HistoryCenterRow::Entry {
            entry: entry.clone(),
            depth,
            parent_entry_id: if index == 0 {
                parent_of_first.map(str::to_owned)
            } else {
                Some(entries[index - 1].id.clone())
            },
            fork_id: fork_id.map(str::to_owned),
            branch_id: branch_id.map(str::to_owned),
            fork_count: entry.fork_count(),
        });

        if let Some(level) = open
            .iter()
            .find(|level| level.anchor_entry_id == entry.id)
        {
            push_disclosed(rows, level, entry, entries, index, depth, root_entries);
        }
    }
}

fn push_disclosed(
    rows: &mut Vec<HistoryCenterRow>,
    level: &HistoryCenterOpenFork,
    entry: &HistoryEntry,
    run_entries: &[HistoryEntry],
    anchor_index: usize,
    depth: usize,
    root_entries: &[HistoryEntry],
) {
    let child_depth = depth + 1;
    let fork_count = entry.fork_count();
    let shown = level.shown();
    // A level is stale when its shown fork's first entry now sits on the root
    // spine: the host navigated into the fork and supplied root pages that
    // already contain the run. Staleness is a data fact, never an
    // array-identity fact, so a host that rebuilds its pages every render
    // cannot loop it. A stale level never splices its cached run — that would
    // duplicate spine entries — and renders the not-yet-loaded row until the
    // machine drops its data and re-requests.
    let is_stale = shown.is_some_and(|shown| {
        root_entries
            .iter()
            .any(|root_entry| root_entry.id == shown.entry_id)
    });

    if fork_count >= 1 {
        // The picker serves every open level, the single fork included, and
        // persists for as long as the level is open: the current selection
        // stays visible and a second fork is one interaction away.
        rows.push(HistoryCenterRow::Picker {
            anchor_entry_id: entry.id.clone(),
            depth: child_depth,
            parent_entry_id: Some(entry.id.clone()),
            continuations: if is_stale {
                Vec::new()
            } else {
                history_center_forks_at(level.continuations.as_ref(), run_entries, anchor_index)
            },
            picked_entry_id: if is_stale {
                None
            } else {
                shown.map(|shown| shown.entry_id.clone())
            },
            is_disabled: fork_count <= 1,
        });
    }

    if is_stale {
        let shown = shown.expect("staleness implies a shown fork");
        rows.push(HistoryCenterRow::NotYetLoaded {
            anchor_entry_id: entry.id.clone(),
            depth: child_depth,
            parent_entry_id: Some(entry.id.clone()),
            fork_id: Some(shown.entry_id.clone()),
            branch_id: Some(shown.branch_id.clone()),
        });
        return;
    }

    let Some(shown) = shown else {
        if fork_count <= 1 {
            // A single fork, or its continuations still in flight: the run is
            // what will show, so mark it not-yet-loaded rather than leave a gap.
            rows.push(HistoryCenterRow::NotYetLoaded {
                anchor_entry_id: entry.id.clone(),
                depth: child_depth,
                parent_entry_id: Some(entry.id.clone()),
                fork_id: None,
                branch_id: None,
            });
        }
        return;
    };

    if level.run_pages.is_empty() {
        rows.push(HistoryCenterRow::NotYetLoaded {
            anchor_entry_id: entry.id.clone(),
            depth: child_depth,
            parent_entry_id: Some(entry.id.clone()),
            fork_id: Some(shown.entry_id.clone()),
            branch_id: Some(shown.branch_id.clone()),
        });
        return;
    }

    let run = history_center_join_pages(&level.run_pages);
    push_run(
        rows,
        &run,
        child_depth,
        Some(&entry.id),
        Some(&shown.entry_id),
        Some(&shown.branch_id),
        &level.inner,
        root_entries,
    );
}

// ── Machine ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum HistoryCenterState {
    #[default]
    Closed,
    Open,
}

/// The rejections the machine can display, declared structurally. The host's
/// bridge maps its protocol onto these two; the machine owns the copy, so the
/// protocol's vocabulary never reaches an operator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoryCenterRejectionCode {
    AlreadyAtTarget,
    UnknownEntry,
}

pub fn history_center_rejection_message(code: HistoryCenterRejectionCode) -> &'static str {
    match code {
        HistoryCenterRejectionCode::AlreadyAtTarget => "Already at the requested target",
        HistoryCenterRejectionCode::UnknownEntry => "Entry does not exist",
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HistoryCenterContext {
    /// Root path pages in fetch order, newest page first. `None` disables the
    /// list: the machine has no rows and every row event is inert.
    pub pages: Option<Vec<HistoryPathPage>>,
    /// Open forks at root entries. Holds only what is open.
    pub open: Vec<HistoryCenterOpenFork>,
    /// Roving focus identity over the visible rows.
    pub focus_row: Option<HistoryCenterRowId>,
    /// Currently displayed rejection message.
    pub rejection: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoryCenterFocusDirection {
    Next,
    Prev,
    First,
    Last,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HistoryCenterEvent {
    Toggle,
    Open,
    Close,
    FocusMove(HistoryCenterFocusDirection),
    /// Activate a row; `None` activates the focused row.
    ActivateRow(Option<HistoryCenterRowId>),
    /// Toggle the fork disclosure at an entry.
    Disclose { entry_id: String },
    ContinuationsLoaded {
        entry_id: String,
        continuations: Vec<HistoryContinuation>,
    },
    PickContinuation { entry_id: String },
    Confirm,
    DeleteContinuation { entry_id: String },
    RunLoaded {
        from_entry_id: String,
        pages: Vec<HistoryPathPage>,
    },
    Rename { branch_id: String, name: String },
    ShowRejection(HistoryCenterRejectionCode),
    DismissRejection,
    /// The host supplied new root pages. Carries nothing and changes nothing
    /// on its own: its only job is to let the standing stale-level reconcile
    /// run, which needs a transition to ride and a pages change dispatches
    /// none. Idempotent — the reconcile drops a stale level's data once and
    /// then has no shown fork to find.
    PagesChanged,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HistoryCenterEffect {
    EmitOpenChange { open: bool },
    FocusRow { row: HistoryCenterRowId },
    /// The clicked row's own branch and entry — never an ancestor, never
    /// another branch's divergence entry. `branch_id` is `None` on the spine.
    EmitNavigateEntry {
        branch_id: Option<String>,
        entry_id: String,
    },
    EmitRenameBranch { branch_id: String, name: String },
    LoadContinuations { entry_id: String },
    LoadContinuationRun { from_entry_id: String },
    /// The picker's commit: the selected fork becomes primary. Poodle does not
    /// build the new root — it emits the command and renders whatever root
    /// pages the host supplies afterwards.
    CheckoutContinuation { entry_id: String },
    /// The host deletes the selected continuation. The machine invalidates
    /// the affected level and re-requests its continuations separately.
    DeleteContinuation { entry_id: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryCenterResult {
    pub state: HistoryCenterState,
    pub context: HistoryCenterContext,
    pub effects: Vec<HistoryCenterEffect>,
}

fn stay(state: HistoryCenterState, context: HistoryCenterContext) -> HistoryCenterResult {
    HistoryCenterResult {
        state,
        context,
        effects: Vec::new(),
    }
}

// ── Disclosure-tree helpers ────────────────────────────────────────────────

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

/// Remove the level anchored at the entry, dropping its whole subtree.
fn without_level(open: &[HistoryCenterOpenFork], entry_id: &str) -> Vec<HistoryCenterOpenFork> {
    open.iter()
        .filter(|level| level.anchor_entry_id != entry_id)
        .map(|level| HistoryCenterOpenFork {
            inner: without_level(&level.inner, entry_id),
            ..level.clone()
        })
        .collect()
}

/// Replace the level anchored at the updated level's entry, wherever it sits.
fn replace_level(
    open: &[HistoryCenterOpenFork],
    updated: &HistoryCenterOpenFork,
) -> Vec<HistoryCenterOpenFork> {
    open.iter()
        .map(|level| {
            if level.anchor_entry_id == updated.anchor_entry_id {
                updated.clone()
            } else {
                HistoryCenterOpenFork {
                    inner: replace_level(&level.inner, updated),
                    ..level.clone()
                }
            }
        })
        .collect()
}

/// Add a level under the run that contains the anchor, or at the root when no
/// open run does. The container is always found when the anchor's entry row is
/// visible, which is what the disclosure guard already proved.
fn with_added_level(
    open: &[HistoryCenterOpenFork],
    added: HistoryCenterOpenFork,
) -> Vec<HistoryCenterOpenFork> {
    fn insert(levels: &[HistoryCenterOpenFork], added: &HistoryCenterOpenFork) -> Option<Vec<HistoryCenterOpenFork>> {
        for (index, level) in levels.iter().enumerate() {
            if run_contains(&level.run_pages, &added.anchor_entry_id) {
                let mut next = levels.to_vec();
                let mut container = level.clone();
                container.inner.push(added.clone());
                next[index] = container;
                return Some(next);
            }
            if let Some(inner) = insert(&level.inner, added) {
                let mut next = levels.to_vec();
                next[index] = HistoryCenterOpenFork {
                    inner,
                    ..level.clone()
                };
                return Some(next);
            }
        }
        None
    }

    match insert(open, &added) {
        Some(next) => next,
        None => {
            let mut next = open.to_vec();
            next.push(added);
            next
        }
    }
}

fn run_contains(pages: &[HistoryPathPage], entry_id: &str) -> bool {
    pages
        .iter()
        .any(|page| page.entries.iter().any(|entry| entry.id == entry_id))
}

/// Every open level, outermost first, flattened.
fn walk_levels(open: &[HistoryCenterOpenFork]) -> Vec<HistoryCenterOpenFork> {
    let mut out = Vec::new();
    for level in open {
        out.push(level.clone());
        out.extend(walk_levels(&level.inner));
    }
    out
}

/// The anchor's run in display order plus its index there.
fn anchor_run_context(
    context: &HistoryCenterContext,
    entry_id: &str,
) -> Option<(Vec<HistoryEntry>, usize)> {
    let root = history_center_join_pages(context.pages.as_deref().unwrap_or(&[]));
    if let Some(index) = root.iter().position(|entry| entry.id == entry_id) {
        return Some((root, index));
    }
    for level in walk_levels(&context.open) {
        let run = history_center_join_pages(&level.run_pages);
        if let Some(index) = run.iter().position(|entry| entry.id == entry_id) {
            return Some((run, index));
        }
    }
    None
}

fn index_of_row(rows: &[HistoryCenterRow], id: Option<&HistoryCenterRowId>) -> Option<usize> {
    let id = id?;
    rows.iter().position(|row| &row.id() == id)
}

/// Keep focus on the same row identity after a shape change. When the focused
/// row vanished, fall back to the toggled anchor's entry row, then the first
/// row — never a stale identity into a list that changed shape.
fn clamp_focus(mut context: HistoryCenterContext, anchor_entry_id: Option<&str>) -> HistoryCenterContext {
    let rows = history_center_visible_rows(context.pages.as_ref(), &context.open);
    if rows.is_empty() {
        context.focus_row = None;
        return context;
    }
    if index_of_row(&rows, context.focus_row.as_ref()).is_some() {
        return context;
    }
    if let Some(anchor_entry_id) = anchor_entry_id {
        let anchor = rows.iter().find(|row| {
            matches!(row, HistoryCenterRow::Entry { entry, .. } if entry.id == anchor_entry_id)
        });
        if let Some(anchor) = anchor {
            context.focus_row = Some(anchor.id());
            return context;
        }
    }
    context.focus_row = Some(rows[0].id());
    context
}

// ── Transitions ────────────────────────────────────────────────────────────

fn move_focus(
    context: HistoryCenterContext,
    direction: HistoryCenterFocusDirection,
) -> HistoryCenterResult {
    let rows = history_center_visible_rows(context.pages.as_ref(), &context.open);
    if rows.is_empty() {
        return stay(HistoryCenterState::Open, context);
    }
    let current = index_of_row(&rows, context.focus_row.as_ref());
    let next = match (current, direction) {
        // No focus yet, or the focused row is gone: land on a boundary.
        (None, HistoryCenterFocusDirection::Prev | HistoryCenterFocusDirection::Last) => {
            rows.len() - 1
        }
        (None, _) => 0,
        (Some(_), HistoryCenterFocusDirection::First) => 0,
        (Some(_), HistoryCenterFocusDirection::Last) => rows.len() - 1,
        (Some(current), HistoryCenterFocusDirection::Next) => (current + 1) % rows.len(),
        (Some(current), HistoryCenterFocusDirection::Prev) => {
            (current + rows.len() - 1) % rows.len()
        }
    };
    let row = rows[next].id();
    HistoryCenterResult {
        state: HistoryCenterState::Open,
        context: HistoryCenterContext {
            focus_row: Some(row.clone()),
            ..context
        },
        effects: vec![HistoryCenterEffect::FocusRow { row }],
    }
}

fn activate_row(
    context: HistoryCenterContext,
    row_id: Option<HistoryCenterRowId>,
) -> HistoryCenterResult {
    let Some(row_id) = row_id else {
        return stay(HistoryCenterState::Open, context);
    };
    let rows = history_center_visible_rows(context.pages.as_ref(), &context.open);
    let Some(index) = index_of_row(&rows, Some(&row_id)) else {
        return stay(HistoryCenterState::Open, context);
    };
    // Focus syncs to the activated row either way; only entry rows navigate.
    let effects = match &rows[index] {
        HistoryCenterRow::Entry {
            entry, branch_id, ..
        } => vec![HistoryCenterEffect::EmitNavigateEntry {
            branch_id: branch_id.clone(),
            entry_id: entry.id.clone(),
        }],
        _ => Vec::new(),
    };
    HistoryCenterResult {
        state: HistoryCenterState::Open,
        context: HistoryCenterContext {
            focus_row: Some(row_id),
            ..context
        },
        effects,
    }
}

fn disclose(context: HistoryCenterContext, entry_id: &str) -> HistoryCenterResult {
    let rows = history_center_visible_rows(context.pages.as_ref(), &context.open);
    let anchor = rows.iter().find_map(|row| match row {
        HistoryCenterRow::Entry { entry, .. } if entry.id == entry_id => Some(entry.clone()),
        _ => None,
    });
    let Some(anchor) = anchor else {
        return stay(HistoryCenterState::Open, context);
    };

    // A fork already open at this entry closes, dropping its subtree.
    if find_level(&context.open, entry_id).is_some() {
        let open = without_level(&context.open, entry_id);
        return HistoryCenterResult {
            state: HistoryCenterState::Open,
            context: clamp_focus(
                HistoryCenterContext { open, ..context },
                Some(entry_id),
            ),
            effects: Vec::new(),
        };
    }

    if anchor.fork_count() < 1 {
        return stay(HistoryCenterState::Open, context);
    }

    let open = with_added_level(&context.open, HistoryCenterOpenFork::opening(entry_id));
    HistoryCenterResult {
        state: HistoryCenterState::Open,
        context: clamp_focus(HistoryCenterContext { open, ..context }, Some(entry_id)),
        effects: vec![HistoryCenterEffect::LoadContinuations {
            entry_id: entry_id.to_owned(),
        }],
    }
}

fn continuations_loaded(
    context: HistoryCenterContext,
    entry_id: &str,
    continuations: Vec<HistoryContinuation>,
) -> HistoryCenterResult {
    // A response for an entry that is not open is stale; drop it.
    let Some(level) = find_level(&context.open, entry_id).cloned() else {
        return stay(HistoryCenterState::Open, context);
    };

    let mut updated = HistoryCenterOpenFork {
        continuations: Some(continuations.clone()),
        ..level
    };
    let mut effects = Vec::new();

    if let Some((entries, index)) = anchor_run_context(&context, entry_id) {
        let fork_count = entries[index].fork_count();
        let forks = history_center_forks_at(Some(&continuations), &entries, index);
        if fork_count == 1 {
            // Exactly one fork: nothing to choose between, so choose it and
            // request its run.
            if let Some(only) = forks.first() {
                updated.chosen = Some(only.clone());
                effects.push(HistoryCenterEffect::LoadContinuationRun {
                    from_entry_id: only.entry_id.clone(),
                });
            }
        } else if fork_count > 1 {
            // Select the current fork — preferred, else first in supplied
            // order — and show its run. The select previews; checkout commits.
            let initial = forks
                .iter()
                .find(|fork| fork.is_preferred)
                .or_else(|| forks.first());
            if let Some(initial) = initial {
                updated.pick = Some(initial.clone());
                effects.push(HistoryCenterEffect::LoadContinuationRun {
                    from_entry_id: initial.entry_id.clone(),
                });
            }
        }
    }

    let open = replace_level(&context.open, &updated);
    HistoryCenterResult {
        state: HistoryCenterState::Open,
        context: clamp_focus(HistoryCenterContext { open, ..context }, Some(entry_id)),
        effects,
    }
}

fn pick_continuation(context: HistoryCenterContext, entry_id: &str) -> HistoryCenterResult {
    // The tentative pick lands on the level whose picker offers this fork.
    let mut picked: Option<(HistoryCenterOpenFork, HistoryContinuation)> = None;
    for level in walk_levels(&context.open) {
        let Some(continuations) = level.continuations.clone() else {
            continue;
        };
        let Some((entries, index)) = anchor_run_context(&context, &level.anchor_entry_id) else {
            continue;
        };
        if entries[index].fork_count() <= 1 {
            continue;
        }
        let forks = history_center_forks_at(Some(&continuations), &entries, index);
        if let Some(candidate) = forks.iter().find(|fork| fork.entry_id == entry_id) {
            picked = Some((level, candidate.clone()));
            break;
        }
    }
    let Some((level, candidate)) = picked else {
        return stay(HistoryCenterState::Open, context);
    };

    // The entries below the select follow the pick. When the loaded run
    // belongs to another fork, drop it and load the picked fork's run; the
    // pick itself commits nothing and emits no host operation.
    let is_switching = level
        .shown()
        .is_none_or(|previous| previous.entry_id != candidate.entry_id);
    let mut updated = HistoryCenterOpenFork {
        pick: Some(candidate.clone()),
        ..level
    };
    if is_switching {
        updated.run_pages = Vec::new();
    }

    // One tentative pick at a time: clear every other level's pick.
    let mut open = replace_level(&context.open, &updated);
    for other in walk_levels(&open) {
        if other.anchor_entry_id != updated.anchor_entry_id && other.pick.is_some() {
            open = replace_level(
                &open,
                &HistoryCenterOpenFork {
                    pick: None,
                    ..other
                },
            );
        }
    }

    HistoryCenterResult {
        state: HistoryCenterState::Open,
        context: clamp_focus(HistoryCenterContext { open, ..context }, None),
        effects: if is_switching {
            vec![HistoryCenterEffect::LoadContinuationRun {
                from_entry_id: candidate.entry_id,
            }]
        } else {
            Vec::new()
        },
    }
}

fn confirm(context: HistoryCenterContext) -> HistoryCenterResult {
    // The auto-chosen single fork counts as picked: confirm commits the
    // displayed fork, whichever way it came to be displayed.
    let picked = walk_levels(&context.open)
        .into_iter()
        .find_map(|level| level.shown().cloned().map(|shown| (level.anchor_entry_id.clone(), shown)));
    let Some((anchor_entry_id, shown)) = picked else {
        return stay(HistoryCenterState::Open, context);
    };

    // The fork is becoming the root, so the open level no longer describes
    // anything: clear the anchor's disclosure state and let the host supply
    // the new root pages.
    let open = without_level(&context.open, &anchor_entry_id);
    HistoryCenterResult {
        state: HistoryCenterState::Open,
        context: clamp_focus(
            HistoryCenterContext { open, ..context },
            Some(&anchor_entry_id),
        ),
        effects: vec![HistoryCenterEffect::CheckoutContinuation {
            entry_id: shown.entry_id,
        }],
    }
}

fn delete_continuation(context: HistoryCenterContext, entry_id: &str) -> HistoryCenterResult {
    for level in walk_levels(&context.open) {
        let Some(continuations) = level.continuations.as_ref() else {
            continue;
        };
        let Some((entries, index)) = anchor_run_context(&context, &level.anchor_entry_id) else {
            continue;
        };
        let forks = history_center_forks_at(Some(continuations), &entries, index);
        if !forks.iter().any(|fork| fork.entry_id == entry_id) {
            continue;
        }

        // A deleted fork never becomes part of the root spine, so the normal
        // stale-level reconciliation cannot invalidate its cached run. Keep
        // the disclosure open, clear every cached claim below it, and ask the
        // host for the authority's new continuation list.
        let invalidated = HistoryCenterOpenFork {
            anchor_entry_id: level.anchor_entry_id.clone(),
            continuations: None,
            pick: None,
            chosen: None,
            run_pages: Vec::new(),
            inner: Vec::new(),
        };
        let open = replace_level(&context.open, &invalidated);
        return HistoryCenterResult {
            state: HistoryCenterState::Open,
            context: HistoryCenterContext { open, ..context },
            effects: vec![
                HistoryCenterEffect::DeleteContinuation {
                    entry_id: entry_id.to_owned(),
                },
                HistoryCenterEffect::LoadContinuations {
                    entry_id: level.anchor_entry_id,
                },
            ],
        };
    }
    stay(HistoryCenterState::Open, context)
}

fn run_loaded(
    context: HistoryCenterContext,
    from_entry_id: &str,
    pages: Vec<HistoryPathPage>,
) -> HistoryCenterResult {
    let updated = walk_levels(&context.open).into_iter().find_map(|level| {
        // The run below the select follows the displayed fork.
        let is_shown = level
            .shown()
            .is_some_and(|shown| shown.entry_id == from_entry_id);
        is_shown.then(|| {
            let mut run_pages = level.run_pages.clone();
            // Pages arrive in fetch order; append so the join can reverse once.
            run_pages.extend(pages.iter().cloned());
            HistoryCenterOpenFork { run_pages, ..level }
        })
    });
    let Some(updated) = updated else {
        return stay(HistoryCenterState::Open, context);
    };
    let open = replace_level(&context.open, &updated);
    HistoryCenterResult {
        state: HistoryCenterState::Open,
        context: clamp_focus(HistoryCenterContext { open, ..context }, None),
        effects: Vec::new(),
    }
}

/// Reconcile open levels against the current root pages. A level whose shown
/// fork's first entry now sits on the joined root pages is stale: it drops its
/// loaded data and its subtree — they describe a run the host already made
/// primary — but stays open at its anchor, because disclosure is UI state and
/// persists. This is not a close. The re-request rides the existing
/// `LoadContinuations` effect, so exactly one leaves, never one per derivation.
fn reconcile_stale_levels(
    context: HistoryCenterContext,
) -> (HistoryCenterContext, Vec<HistoryCenterEffect>) {
    let Some(pages) = context.pages.as_ref() else {
        return (context, Vec::new());
    };
    if pages.is_empty() || context.open.is_empty() {
        return (context, Vec::new());
    }
    let root_ids: Vec<String> = history_center_join_pages(pages)
        .into_iter()
        .map(|entry| entry.id)
        .collect();
    let mut effects = Vec::new();
    let open = drop_stale_levels(&context.open, &root_ids, &mut effects);
    if effects.is_empty() && open == context.open {
        return (context, Vec::new());
    }
    (HistoryCenterContext { open, ..context }, effects)
}

fn drop_stale_levels(
    open: &[HistoryCenterOpenFork],
    root_ids: &[String],
    effects: &mut Vec<HistoryCenterEffect>,
) -> Vec<HistoryCenterOpenFork> {
    open.iter()
        .map(|level| {
            let is_stale = level
                .shown()
                .is_some_and(|shown| root_ids.iter().any(|id| id == &shown.entry_id));
            if is_stale {
                effects.push(HistoryCenterEffect::LoadContinuations {
                    entry_id: level.anchor_entry_id.clone(),
                });
                // The same shape a fresh disclosure has, minus the anchor, so
                // the level renders the not-yet-loaded row until the answer
                // lands.
                HistoryCenterOpenFork::opening(&level.anchor_entry_id)
            } else {
                HistoryCenterOpenFork {
                    inner: drop_stale_levels(&level.inner, root_ids, effects),
                    ..level.clone()
                }
            }
        })
        .collect()
}

pub fn history_center_transition(
    state: HistoryCenterState,
    context: HistoryCenterContext,
    event: HistoryCenterEvent,
) -> HistoryCenterResult {
    // Reconcile before every open-state event except the two that close the
    // popover: both drop the whole tree anyway and must not emit a re-request
    // on the way out.
    if state == HistoryCenterState::Open
        && !matches!(
            event,
            HistoryCenterEvent::Close | HistoryCenterEvent::Toggle
        )
    {
        let (reconciled, effects) = reconcile_stale_levels(context.clone());
        if !effects.is_empty() || reconciled != context {
            let mut next = dispatch(state, reconciled, event);
            let mut all = effects;
            all.append(&mut next.effects);
            next.effects = all;
            return next;
        }
    }
    dispatch(state, context, event)
}

fn dispatch(
    state: HistoryCenterState,
    context: HistoryCenterContext,
    event: HistoryCenterEvent,
) -> HistoryCenterResult {
    let is_open = state == HistoryCenterState::Open;
    match event {
        // Deliberately inert: the reconcile above this call is the whole
        // point, and returning the context unchanged keeps an adapter's state
        // write-back a no-op when nothing was stale.
        HistoryCenterEvent::PagesChanged => stay(state, context),
        HistoryCenterEvent::Toggle => {
            if is_open {
                close_result(context)
            } else {
                open_result(context)
            }
        }
        HistoryCenterEvent::Open => {
            if is_open {
                stay(state, context)
            } else {
                open_result(context)
            }
        }
        HistoryCenterEvent::Close => {
            if is_open {
                close_result(context)
            } else {
                stay(state, context)
            }
        }
        HistoryCenterEvent::FocusMove(direction) => {
            if is_open {
                move_focus(context, direction)
            } else {
                stay(state, context)
            }
        }
        HistoryCenterEvent::ActivateRow(row) => {
            if is_open {
                let target = row.or_else(|| context.focus_row.clone());
                activate_row(context, target)
            } else {
                stay(state, context)
            }
        }
        HistoryCenterEvent::Disclose { entry_id } => {
            if is_open {
                disclose(context, &entry_id)
            } else {
                stay(state, context)
            }
        }
        HistoryCenterEvent::ContinuationsLoaded {
            entry_id,
            continuations,
        } => {
            if is_open {
                continuations_loaded(context, &entry_id, continuations)
            } else {
                stay(state, context)
            }
        }
        HistoryCenterEvent::PickContinuation { entry_id } => {
            if is_open {
                pick_continuation(context, &entry_id)
            } else {
                stay(state, context)
            }
        }
        HistoryCenterEvent::Confirm => {
            if is_open {
                confirm(context)
            } else {
                stay(state, context)
            }
        }
        HistoryCenterEvent::DeleteContinuation { entry_id } => {
            if is_open {
                delete_continuation(context, &entry_id)
            } else {
                stay(state, context)
            }
        }
        HistoryCenterEvent::RunLoaded {
            from_entry_id,
            pages,
        } => {
            if is_open {
                run_loaded(context, &from_entry_id, pages)
            } else {
                stay(state, context)
            }
        }
        // Rename is a pass-through client-side affordance in either state: it
        // enforces no protocol rule and owns no machine state.
        HistoryCenterEvent::Rename { branch_id, name } => HistoryCenterResult {
            state,
            context,
            effects: vec![HistoryCenterEffect::EmitRenameBranch { branch_id, name }],
        },
        HistoryCenterEvent::ShowRejection(code) => {
            let message = history_center_rejection_message(code);
            if context.rejection.as_deref() == Some(message) {
                stay(state, context)
            } else {
                HistoryCenterResult {
                    state,
                    context: HistoryCenterContext {
                        rejection: Some(message.to_owned()),
                        ..context
                    },
                    effects: Vec::new(),
                }
            }
        }
        HistoryCenterEvent::DismissRejection => {
            if context.rejection.is_none() {
                stay(state, context)
            } else {
                HistoryCenterResult {
                    state,
                    context: HistoryCenterContext {
                        rejection: None,
                        ..context
                    },
                    effects: Vec::new(),
                }
            }
        }
    }
}

fn open_result(context: HistoryCenterContext) -> HistoryCenterResult {
    HistoryCenterResult {
        state: HistoryCenterState::Open,
        context,
        effects: vec![HistoryCenterEffect::EmitOpenChange { open: true }],
    }
}

fn close_result(context: HistoryCenterContext) -> HistoryCenterResult {
    // Nothing is cached across a close and reopen: the disclosure tree and the
    // loaded pages it holds go with the popover.
    HistoryCenterResult {
        state: HistoryCenterState::Closed,
        context: HistoryCenterContext {
            open: Vec::new(),
            focus_row: None,
            ..context
        },
        effects: vec![HistoryCenterEffect::EmitOpenChange { open: false }],
    }
}

/// Map a row-list keydown to a machine event. `None` for keys the machine does
/// not own, which the adapter then lets propagate.
pub fn history_center_keydown_event(key: &str) -> Option<HistoryCenterEvent> {
    Some(match key {
        "ArrowDown" => HistoryCenterEvent::FocusMove(HistoryCenterFocusDirection::Next),
        "ArrowUp" => HistoryCenterEvent::FocusMove(HistoryCenterFocusDirection::Prev),
        "Home" => HistoryCenterEvent::FocusMove(HistoryCenterFocusDirection::First),
        "End" => HistoryCenterEvent::FocusMove(HistoryCenterFocusDirection::Last),
        "Enter" | " " => HistoryCenterEvent::ActivateRow(None),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, continuation_count: usize) -> HistoryEntry {
        HistoryEntry::new(id, format!("Entry {id}")).with_continuation_count(continuation_count)
    }

    fn page(entries: Vec<HistoryEntry>, offset: usize) -> HistoryPathPage {
        HistoryPathPage::new(entries).with_offset(offset)
    }

    fn fork(entry_id: &str, branch_id: &str) -> HistoryContinuation {
        HistoryContinuation::new(entry_id, format!("Fork {entry_id}"), branch_id)
    }

    fn open_context(pages: Vec<HistoryPathPage>) -> HistoryCenterContext {
        HistoryCenterContext {
            pages: Some(pages),
            ..HistoryCenterContext::default()
        }
    }

    fn row_ids(rows: &[HistoryCenterRow]) -> Vec<(HistoryCenterRowKind, String, usize)> {
        rows.iter()
            .map(|row| {
                let id = row.id();
                (id.kind, id.entry_id, row.depth())
            })
            .collect()
    }

    /// Pages arrive newest-first and display oldest-first. Joining in fetch
    /// order is the paging trap: the later-fetched (older) page has to render
    /// before the first page, or history reads backwards.
    #[test]
    fn pages_join_into_one_oldest_first_timeline() {
        let rows = history_center_visible_rows(
            Some(&vec![
                page(vec![entry("e4", 0), entry("e3", 1)], 0),
                page(vec![entry("e2", 1), entry("e1", 1)], 2),
            ]),
            &[],
        );
        assert_eq!(
            row_ids(&rows)
                .into_iter()
                .map(|(_, id, _)| id)
                .collect::<Vec<_>>(),
            ["e1", "e2", "e3", "e4"],
        );
    }

    /// An overlapping seam repeats an entry across two pages. It renders once:
    /// an id names exactly one immutable graph node.
    #[test]
    fn an_overlapping_page_seam_renders_each_entry_once() {
        let rows = history_center_visible_rows(
            Some(&vec![
                page(vec![entry("e3", 1), entry("e2", 1)], 0),
                page(vec![entry("e2", 1), entry("e1", 1)], 1),
            ]),
            &[],
        );
        assert_eq!(
            row_ids(&rows)
                .into_iter()
                .map(|(_, id, _)| id)
                .collect::<Vec<_>>(),
            ["e1", "e2", "e3"],
        );
    }

    /// A run's terminal entry has no children at all, so the unsaturated form
    /// would yield -1.
    #[test]
    fn a_terminal_entry_has_no_forks() {
        assert_eq!(history_center_fork_count(0), 0);
        assert_eq!(history_center_fork_count(1), 0);
        assert_eq!(history_center_fork_count(3), 2);
    }

    /// The continuations page returns the child already on the list. It is
    /// filtered by id, never by position.
    #[test]
    fn the_child_already_on_the_list_is_filtered_by_id() {
        let run = vec![entry("e1", 3), entry("e2", 0)];
        let forks = history_center_forks_at(
            Some(&vec![fork("e2", "main"), fork("f1", "wide"), fork("f2", "duck")]),
            &run,
            0,
        );
        assert_eq!(
            forks.iter().map(|f| f.entry_id.as_str()).collect::<Vec<_>>(),
            ["f1", "f2"],
        );
    }

    /// When paging truncated the successor away, the preferred flag names the
    /// same record: a run follows preferred children.
    #[test]
    fn a_truncated_successor_is_recognised_by_preference() {
        let run = vec![entry("e1", 3)];
        let forks = history_center_forks_at(
            Some(&vec![
                fork("e2", "main").with_preferred(true),
                fork("f1", "wide"),
            ]),
            &run,
            0,
        );
        assert_eq!(
            forks.iter().map(|f| f.entry_id.as_str()).collect::<Vec<_>>(),
            ["f1"],
        );
    }

    /// Disclosure asks the host and renders a placeholder; it never loads
    /// speculatively and never leaves a gap.
    #[test]
    fn disclosure_requests_continuations_and_holds_a_place() {
        let context = open_context(vec![page(vec![entry("e2", 3), entry("e1", 1)], 0)]);
        let result = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::Disclose {
                entry_id: "e2".to_owned(),
            },
        );
        assert_eq!(
            result.effects,
            [HistoryCenterEffect::LoadContinuations {
                entry_id: "e2".to_owned()
            }],
        );
        let rows = history_center_visible_rows(result.context.pages.as_ref(), &result.context.open);
        assert_eq!(
            row_ids(&rows),
            [
                (HistoryCenterRowKind::Entry, "e1".to_owned(), 0),
                (HistoryCenterRowKind::Entry, "e2".to_owned(), 0),
                (HistoryCenterRowKind::Picker, "e2".to_owned(), 1),
            ],
        );
    }

    /// More than one fork selects the current one — preferred, else first in
    /// supplied order — and previews its run.
    #[test]
    fn several_forks_select_the_current_one_and_preview_it() {
        // e3 is e2's successor on the spine, so it is the continuation the
        // derivation filters out; the picker offers the other two.
        let context = open_context(vec![page(
            vec![entry("e3", 0), entry("e2", 3), entry("e1", 1)],
            0,
        )]);
        let opened = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::Disclose {
                entry_id: "e2".to_owned(),
            },
        );
        let loaded = history_center_transition(
            opened.state,
            opened.context,
            HistoryCenterEvent::ContinuationsLoaded {
                entry_id: "e2".to_owned(),
                continuations: vec![
                    fork("e3", "main").with_preferred(true),
                    fork("f1", "wide"),
                    fork("f2", "duck"),
                ],
            },
        );
        // None of the offered forks is preferred — the preferred one is the
        // row already on the list — so the first in supplied order shows.
        assert_eq!(
            loaded.effects,
            [HistoryCenterEffect::LoadContinuationRun {
                from_entry_id: "f1".to_owned()
            }],
        );
    }

    /// One fork leaves nothing to choose between, so it is chosen outright —
    /// and the picker still renders, disabled, rather than vanishing.
    #[test]
    fn a_single_fork_is_chosen_and_still_shows_a_disabled_picker() {
        let context = open_context(vec![page(vec![entry("e2", 2), entry("e1", 1)], 0)]);
        let opened = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::Disclose {
                entry_id: "e2".to_owned(),
            },
        );
        let loaded = history_center_transition(
            opened.state,
            opened.context,
            HistoryCenterEvent::ContinuationsLoaded {
                entry_id: "e2".to_owned(),
                continuations: vec![fork("g1", "only")],
            },
        );
        assert_eq!(
            loaded.effects,
            [HistoryCenterEffect::LoadContinuationRun {
                from_entry_id: "g1".to_owned()
            }],
        );
        let rows = history_center_visible_rows(loaded.context.pages.as_ref(), &loaded.context.open);
        let picker = rows
            .iter()
            .find(|row| matches!(row, HistoryCenterRow::Picker { .. }))
            .expect("the picker persists for as long as the level is open");
        assert!(matches!(
            picker,
            HistoryCenterRow::Picker {
                is_disabled: true,
                ..
            }
        ));
    }

    /// A run row reports its own branch and its own entry — never the anchor,
    /// never the branch's divergence entry.
    #[test]
    fn a_run_row_navigates_on_its_own_branch() {
        let context = open_context(vec![page(vec![entry("e2", 2), entry("e1", 1)], 0)]);
        let opened = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::Disclose {
                entry_id: "e2".to_owned(),
            },
        );
        let loaded = history_center_transition(
            opened.state,
            opened.context,
            HistoryCenterEvent::ContinuationsLoaded {
                entry_id: "e2".to_owned(),
                continuations: vec![fork("f1", "branch-wide")],
            },
        );
        let ran = history_center_transition(
            loaded.state,
            loaded.context,
            HistoryCenterEvent::RunLoaded {
                from_entry_id: "f1".to_owned(),
                pages: vec![page(vec![entry("f1b", 0), entry("f1", 1)], 0)],
            },
        );
        let activated = history_center_transition(
            ran.state,
            ran.context,
            HistoryCenterEvent::ActivateRow(Some(HistoryCenterRowId::new(
                HistoryCenterRowKind::Entry,
                "f1b",
            ))),
        );
        assert_eq!(
            activated.effects,
            [HistoryCenterEffect::EmitNavigateEntry {
                branch_id: Some("branch-wide".to_owned()),
                entry_id: "f1b".to_owned(),
            }],
        );
    }

    /// The spine reports no branch: the host knows its own.
    #[test]
    fn a_spine_row_reports_no_branch() {
        let context = open_context(vec![page(vec![entry("e2", 1), entry("e1", 1)], 0)]);
        let result = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::ActivateRow(Some(HistoryCenterRowId::new(
                HistoryCenterRowKind::Entry,
                "e1",
            ))),
        );
        assert_eq!(
            result.effects,
            [HistoryCenterEffect::EmitNavigateEntry {
                branch_id: None,
                entry_id: "e1".to_owned(),
            }],
        );
    }

    /// Closing a level drops its whole subtree; nothing survives to be
    /// restored on reopen.
    #[test]
    fn closing_a_level_drops_its_subtree() {
        let context = open_context(vec![page(vec![entry("e2", 3), entry("e1", 1)], 0)]);
        let opened = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::Disclose {
                entry_id: "e2".to_owned(),
            },
        );
        let closed = history_center_transition(
            opened.state,
            opened.context,
            HistoryCenterEvent::Disclose {
                entry_id: "e2".to_owned(),
            },
        );
        assert!(closed.context.open.is_empty());
        assert!(closed.effects.is_empty());
    }

    /// A level whose shown fork now sits on the spine never splices its cached
    /// run — that would render the same entries twice. It keeps its anchor
    /// open, drops its data and re-requests exactly once.
    #[test]
    fn a_stale_level_re_requests_once_and_never_duplicates_a_row() {
        let level = HistoryCenterOpenFork {
            anchor_entry_id: "e2".to_owned(),
            continuations: Some(vec![fork("f1", "wide")]),
            pick: Some(fork("f1", "wide")),
            chosen: None,
            run_pages: vec![page(vec![entry("f1", 0)], 0)],
            inner: Vec::new(),
        };
        // The host navigated into the fork: f1 is on the root spine now.
        let context = HistoryCenterContext {
            pages: Some(vec![page(
                vec![entry("f1", 0), entry("e2", 3), entry("e1", 1)],
                0,
            )]),
            open: vec![level],
            ..HistoryCenterContext::default()
        };
        let result = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::PagesChanged,
        );
        assert_eq!(
            result.effects,
            [HistoryCenterEffect::LoadContinuations {
                entry_id: "e2".to_owned()
            }],
        );
        let rows = history_center_visible_rows(result.context.pages.as_ref(), &result.context.open);
        let f1_rows = rows
            .iter()
            .filter(|row| matches!(row, HistoryCenterRow::Entry { entry, .. } if entry.id == "f1"))
            .count();
        assert_eq!(f1_rows, 1, "the run must not be spliced under its anchor");

        // Idempotent: once dropped, the level has no shown fork to find.
        let again = history_center_transition(
            result.state,
            result.context,
            HistoryCenterEvent::PagesChanged,
        );
        assert!(again.effects.is_empty());
    }

    /// Checkout commits the displayed fork and clears the anchor's disclosure;
    /// Poodle does not build the new root.
    #[test]
    fn checkout_commits_the_shown_fork_and_clears_the_level() {
        let context = HistoryCenterContext {
            pages: Some(vec![page(vec![entry("e2", 3), entry("e1", 1)], 0)]),
            open: vec![HistoryCenterOpenFork {
                anchor_entry_id: "e2".to_owned(),
                continuations: Some(vec![fork("f1", "wide"), fork("f2", "duck")]),
                pick: Some(fork("f2", "duck")),
                chosen: None,
                run_pages: Vec::new(),
                inner: Vec::new(),
            }],
            ..HistoryCenterContext::default()
        };
        let result =
            history_center_transition(HistoryCenterState::Open, context, HistoryCenterEvent::Confirm);
        assert_eq!(
            result.effects,
            [HistoryCenterEffect::CheckoutContinuation {
                entry_id: "f2".to_owned()
            }],
        );
        assert!(result.context.open.is_empty());
    }

    #[test]
    fn delete_invalidates_the_offered_fork_and_reloads_its_anchor() {
        let context = HistoryCenterContext {
            pages: Some(vec![page(vec![entry("e2", 3), entry("e1", 1)], 0)]),
            open: vec![HistoryCenterOpenFork {
                anchor_entry_id: "e2".to_owned(),
                continuations: Some(vec![fork("f1", "wide"), fork("f2", "duck")]),
                pick: Some(fork("f2", "duck")),
                chosen: None,
                run_pages: vec![page(vec![entry("f2", 1)], 0)],
                inner: vec![HistoryCenterOpenFork::opening("f2")],
            }],
            ..HistoryCenterContext::default()
        };
        let result = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::DeleteContinuation {
                entry_id: "f2".to_owned(),
            },
        );

        assert_eq!(
            result.effects,
            [
                HistoryCenterEffect::DeleteContinuation {
                    entry_id: "f2".to_owned(),
                },
                HistoryCenterEffect::LoadContinuations {
                    entry_id: "e2".to_owned(),
                },
            ],
        );
        let level = &result.context.open[0];
        assert_eq!(level.anchor_entry_id, "e2");
        assert!(level.continuations.is_none());
        assert!(level.pick.is_none());
        assert!(level.chosen.is_none());
        assert!(level.run_pages.is_empty());
        assert!(level.inner.is_empty());
    }

    #[test]
    fn delete_is_inert_for_an_unoffered_fork_and_while_closed() {
        let context = HistoryCenterContext {
            pages: Some(vec![page(vec![entry("e2", 2), entry("e1", 1)], 0)]),
            open: vec![HistoryCenterOpenFork {
                anchor_entry_id: "e2".to_owned(),
                continuations: Some(vec![fork("f1", "wide")]),
                chosen: Some(fork("f1", "wide")),
                ..HistoryCenterOpenFork::default()
            }],
            ..HistoryCenterContext::default()
        };

        let unknown = history_center_transition(
            HistoryCenterState::Open,
            context.clone(),
            HistoryCenterEvent::DeleteContinuation {
                entry_id: "ghost".to_owned(),
            },
        );
        assert!(unknown.effects.is_empty());
        assert_eq!(unknown.context, context);

        let closed = history_center_transition(
            HistoryCenterState::Closed,
            context.clone(),
            HistoryCenterEvent::DeleteContinuation {
                entry_id: "f1".to_owned(),
            },
        );
        assert!(closed.effects.is_empty());
        assert_eq!(closed.context, context);
    }

    /// Focus is the row, not an index, so it survives a shape change under it.
    #[test]
    fn focus_survives_a_disclosure_toggle() {
        let context = HistoryCenterContext {
            focus_row: Some(HistoryCenterRowId::new(HistoryCenterRowKind::Entry, "e1")),
            ..open_context(vec![page(vec![entry("e2", 3), entry("e1", 1)], 0)])
        };
        let result = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::Disclose {
                entry_id: "e2".to_owned(),
            },
        );
        assert_eq!(
            result.context.focus_row,
            Some(HistoryCenterRowId::new(HistoryCenterRowKind::Entry, "e1")),
        );
    }

    /// Roving focus wraps, and a picker row is a focus stop that navigates
    /// nowhere.
    #[test]
    fn focus_wraps_and_a_picker_row_navigates_nowhere() {
        let context = open_context(vec![page(vec![entry("e2", 1), entry("e1", 1)], 0)]);
        let last = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::FocusMove(HistoryCenterFocusDirection::Last),
        );
        assert_eq!(
            last.context.focus_row,
            Some(HistoryCenterRowId::new(HistoryCenterRowKind::Entry, "e2")),
        );
        let wrapped = history_center_transition(
            last.state,
            last.context,
            HistoryCenterEvent::FocusMove(HistoryCenterFocusDirection::Next),
        );
        assert_eq!(
            wrapped.context.focus_row,
            Some(HistoryCenterRowId::new(HistoryCenterRowKind::Entry, "e1")),
        );
    }

    /// Absence is the signal: no pages means no rows and every row event is
    /// inert, rather than an empty list pretending to be a history.
    #[test]
    fn without_pages_every_row_event_is_inert() {
        let context = HistoryCenterContext::default();
        let result = history_center_transition(
            HistoryCenterState::Open,
            context,
            HistoryCenterEvent::Disclose {
                entry_id: "e2".to_owned(),
            },
        );
        assert!(result.effects.is_empty());
        assert!(history_center_visible_rows(None, &[]).is_empty());
    }

    /// Closing drops the disclosure tree and the focus with it.
    #[test]
    fn closing_the_popover_caches_nothing() {
        let context = HistoryCenterContext {
            open: vec![HistoryCenterOpenFork::opening("e2")],
            focus_row: Some(HistoryCenterRowId::new(HistoryCenterRowKind::Entry, "e1")),
            ..open_context(vec![page(vec![entry("e2", 3), entry("e1", 1)], 0)])
        };
        let result =
            history_center_transition(HistoryCenterState::Open, context, HistoryCenterEvent::Close);
        assert_eq!(result.state, HistoryCenterState::Closed);
        assert!(result.context.open.is_empty());
        assert_eq!(result.context.focus_row, None);
        assert_eq!(
            result.effects,
            [HistoryCenterEffect::EmitOpenChange { open: false }],
        );
    }

    /// Depth is a number the renderer uses, and it is never saturated.
    #[test]
    fn depth_nests_without_a_cap() {
        let inner = HistoryCenterOpenFork {
            anchor_entry_id: "f1b".to_owned(),
            continuations: Some(vec![fork("f1b1", "tame")]),
            pick: None,
            chosen: Some(fork("f1b1", "tame")),
            run_pages: vec![page(vec![entry("f1b1", 0)], 0)],
            inner: Vec::new(),
        };
        let outer = HistoryCenterOpenFork {
            anchor_entry_id: "e2".to_owned(),
            continuations: Some(vec![fork("f1", "wide")]),
            pick: None,
            chosen: Some(fork("f1", "wide")),
            run_pages: vec![page(vec![entry("f1b", 2), entry("f1", 1)], 0)],
            inner: vec![inner],
        };
        let rows = history_center_visible_rows(
            Some(&vec![page(vec![entry("e2", 2), entry("e1", 1)], 0)]),
            &[outer],
        );
        // Each disclosure adds exactly one level, and nothing saturates: the
        // anchor's run sits one below the anchor, and the run's own fork one
        // below that.
        let depth_of = |id: &str| {
            rows.iter()
                .find(|row| matches!(row, HistoryCenterRow::Entry { entry, .. } if entry.id == id))
                .expect("the run renders")
                .depth()
        };
        assert_eq!(depth_of("e2"), 0);
        assert_eq!(depth_of("f1b"), 1);
        assert_eq!(depth_of("f1b1"), 2);
    }
}
