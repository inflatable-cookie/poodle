//! Agent transcript machinery. Mirror of core `agent-transcript.ts`:
//! the transcript item model, contiguous tool-run grouping, variable-height
//! scroll windowing, and the bottom-anchoring predicate.
//!
//! Contract: `docs/contracts/components/agent-transcript.md`.
//!
//! `AgentChatInput` owns the composer and scopes out the transcript; this is
//! the other side of that boundary. Parity with the TS core is enforced by
//! `vectors/agent-transcript.json`, run by both runtimes.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranscriptRole {
    User,
    Assistant,
}

impl TranscriptRole {
    pub fn as_str(self) -> &'static str {
        match self {
            TranscriptRole::User => "user",
            TranscriptRole::Assistant => "assistant",
        }
    }
}

/// How a tool call ended. `Running` is the live case — no result yet.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToolCallStatus {
    Running,
    #[default]
    Success,
    Error,
}

impl ToolCallStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ToolCallStatus::Running => "running",
            ToolCallStatus::Success => "success",
            ToolCallStatus::Error => "error",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "running" => ToolCallStatus::Running,
            "error" => ToolCallStatus::Error,
            _ => ToolCallStatus::Success,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TranscriptMessage {
    pub id: String,
    pub role: Option<TranscriptRole>,
    /// Raw markdown. Parsed by the renderer, never pre-rendered by the host.
    pub markdown: String,
    /// True while tokens are still arriving; drives the caret.
    pub is_streaming: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TranscriptToolCall {
    pub id: String,
    /// What kind of work this was — "Ran command", "File change", "Searched".
    pub label: String,
    /// The argument line, truncated to one line when collapsed.
    pub detail: Option<String>,
    pub status: ToolCallStatus,
    pub icon: Option<String>,
    /// Full output, revealed when the row is expanded.
    pub output: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangedFileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
}

impl ChangedFileStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ChangedFileStatus::Added => "added",
            ChangedFileStatus::Modified => "modified",
            ChangedFileStatus::Deleted => "deleted",
            ChangedFileStatus::Renamed => "renamed",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChangedFile {
    pub path: String,
    pub additions: u32,
    pub deletions: u32,
    pub status: Option<ChangedFileStatus>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TranscriptChangedFiles {
    pub id: String,
    pub files: Vec<ChangedFile>,
}

/// The live footer — "Working for 1h 1m". Present only while the turn runs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TranscriptActivity {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TranscriptItem {
    Message(TranscriptMessage),
    ToolCall(TranscriptToolCall),
    ChangedFiles(TranscriptChangedFiles),
    Activity(TranscriptActivity),
}

impl TranscriptItem {
    pub fn id(&self) -> &str {
        match self {
            TranscriptItem::Message(item) => &item.id,
            TranscriptItem::ToolCall(item) => &item.id,
            TranscriptItem::ChangedFiles(item) => &item.id,
            TranscriptItem::Activity(item) => &item.id,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            TranscriptItem::Message(_) => "message",
            TranscriptItem::ToolCall(_) => "tool-call",
            TranscriptItem::ChangedFiles(_) => "changed-files",
            TranscriptItem::Activity(_) => "activity",
        }
    }
}

/// A run of adjacent tool calls, presented as one collapsible unit.
///
/// Collapsed, the run shows its *last* call and hides the rest behind
/// "+N previous tool calls" — the newest call is the one still telling you
/// something. Expanded, the run lists every call in order, ending on the same
/// call that was visible while collapsed, so expanding never moves the row you
/// were reading.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TranscriptToolRun {
    /// Stable across appends: the id of the run's first call.
    pub id: String,
    pub calls: Vec<TranscriptToolCall>,
}

impl TranscriptToolRun {
    /// The call a collapsed run shows: the newest one.
    pub fn lead_call(&self) -> Option<&TranscriptToolCall> {
        self.calls.last()
    }

    /// How many calls "+N previous tool calls" is offering.
    pub fn hidden_count(&self) -> usize {
        self.calls.len().saturating_sub(1)
    }

    /// The run's status, for the collapsed summary's indicator.
    ///
    /// A single failure anywhere wins over any number of successes: the point
    /// of the summary is to tell you whether you need to open it, and one
    /// failed command inside eight successful ones is exactly when you do.
    /// `Running` ranks below `Error` for the same reason — a run that already
    /// broke is not "in progress" in any sense the reader cares about.
    pub fn status(&self) -> ToolCallStatus {
        if self.calls.iter().any(|c| c.status == ToolCallStatus::Error) {
            return ToolCallStatus::Error;
        }
        if self.calls.iter().any(|c| c.status == ToolCallStatus::Running) {
            return ToolCallStatus::Running;
        }
        ToolCallStatus::Success
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TranscriptBlock {
    Message(TranscriptMessage),
    ToolRun(TranscriptToolRun),
    ChangedFiles(TranscriptChangedFiles),
    Activity(TranscriptActivity),
}

impl TranscriptBlock {
    pub fn kind(&self) -> &'static str {
        match self {
            TranscriptBlock::Message(_) => "message",
            TranscriptBlock::ToolRun(_) => "tool-run",
            TranscriptBlock::ChangedFiles(_) => "changed-files",
            TranscriptBlock::Activity(_) => "activity",
        }
    }

    pub fn id(&self) -> &str {
        match self {
            TranscriptBlock::Message(item) => &item.id,
            TranscriptBlock::ToolRun(item) => &item.id,
            TranscriptBlock::ChangedFiles(item) => &item.id,
            TranscriptBlock::Activity(item) => &item.id,
        }
    }
}

/// Collapse contiguous tool calls into runs, leaving every other item alone.
///
/// Adjacency is the whole rule: anything that is not a tool call ends the run.
/// A changed-files card between two commands therefore splits them into two
/// runs, which is what the transcript should say — those commands happened
/// either side of an edit, not as one stretch of work.
///
/// Pure function of the list, so a streaming run regroups correctly as calls
/// land with no incremental state to get out of step.
pub fn group_transcript_items(items: &[TranscriptItem]) -> Vec<TranscriptBlock> {
    let mut blocks: Vec<TranscriptBlock> = Vec::new();
    let mut in_run = false;

    for item in items {
        match item {
            TranscriptItem::ToolCall(call) => {
                if in_run {
                    if let Some(TranscriptBlock::ToolRun(run)) = blocks.last_mut() {
                        run.calls.push(call.clone());
                        continue;
                    }
                }
                blocks.push(TranscriptBlock::ToolRun(TranscriptToolRun {
                    id: call.id.clone(),
                    calls: vec![call.clone()],
                }));
                in_run = true;
            }
            TranscriptItem::Message(item) => {
                in_run = false;
                blocks.push(TranscriptBlock::Message(item.clone()));
            }
            TranscriptItem::ChangedFiles(item) => {
                in_run = false;
                blocks.push(TranscriptBlock::ChangedFiles(item.clone()));
            }
            TranscriptItem::Activity(item) => {
                in_run = false;
                blocks.push(TranscriptBlock::Activity(item.clone()));
            }
        }
    }

    blocks
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChangedFilesTotals {
    pub file_count: usize,
    pub additions: u32,
    pub deletions: u32,
}

pub fn changed_files_totals(files: &[ChangedFile]) -> ChangedFilesTotals {
    ChangedFilesTotals {
        file_count: files.len(),
        additions: files.iter().map(|f| f.additions).sum(),
        deletions: files.iter().map(|f| f.deletions).sum(),
    }
}

// ── Variable-height scroll windowing ──

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TranscriptWindow {
    pub start_index: usize,
    pub end_index: usize,
    /// Pixel offset of `start_index` from the top of the scrolled content.
    pub offset_y: f64,
    pub total_height: f64,
}

/// The window of blocks worth rendering, for variable-height rows.
///
/// `tree_virtual_window` divides by a uniform row height, which a transcript
/// does not have — a one-line message, a forty-row tool run and a file tree
/// differ by an order of magnitude. So this walks measured heights instead,
/// falling back to `estimated_height` for rows not yet measured.
pub fn transcript_window(
    heights: &[f64],
    estimated_height: f64,
    scroll_top: f64,
    viewport_height: f64,
    overscan: usize,
) -> TranscriptWindow {
    let height_at = |index: usize| -> f64 {
        match heights.get(index) {
            Some(h) if *h > 0.0 => *h,
            _ => estimated_height,
        }
    };

    let count = heights.len();
    let top = scroll_top.max(0.0);
    let bottom = top + viewport_height.max(0.0);

    let mut start_index = 0usize;
    let mut offset_y = 0.0f64;
    let mut cursor = 0.0f64;
    let mut end_index = count;
    let mut seen_bottom = false;

    for index in 0..count {
        let row_bottom = cursor + height_at(index);

        if row_bottom <= top {
            start_index = index + 1;
            offset_y = row_bottom;
        } else if cursor >= bottom && !seen_bottom {
            end_index = index;
            seen_bottom = true;
        }

        cursor = row_bottom;
    }

    // Overscan is applied after the scan so it cannot push the offset out of
    // step with the index it describes.
    for _ in 0..overscan {
        if start_index == 0 {
            break;
        }
        start_index -= 1;
        offset_y -= height_at(start_index);
    }

    end_index = (end_index + overscan).min(count);

    TranscriptWindow {
        start_index,
        end_index: end_index.max(start_index),
        offset_y: offset_y.max(0.0),
        total_height: cursor,
    }
}

/// Whether the viewport is close enough to the bottom to keep following output.
///
/// Anchoring is a latch, not a computation: once the reader scrolls up the
/// transcript must stop dragging them back down, and resume only when they
/// return to the bottom themselves. The threshold exists because "at the
/// bottom" is never exact — subpixel scroll positions and a row growing by a
/// line both leave slack that should still count as following.
pub fn is_pinned_to_bottom(
    scroll_top: f64,
    scroll_height: f64,
    client_height: f64,
    threshold_px: f64,
) -> bool {
    scroll_height - (scroll_top + client_height) <= threshold_px
}

// ── Changed-file tree ──

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChangedFileNode {
    /// Full path from the root, used as the tree node's value.
    pub path: String,
    /// What this row displays — a collapsed chain shows `crates/latex`.
    pub label: String,
    pub is_directory: bool,
    pub additions: u32,
    pub deletions: u32,
    pub children: Vec<ChangedFileNode>,
}

/// Fold flat paths into a directory tree with counts rolled up from descendants.
///
/// Single-child directory chains collapse into one row. A path like
/// `cp-api/crates/latex/src/parser.rs` would otherwise cost four rows to say one
/// thing; the chain collapses exactly as long as no node in it has a sibling.
/// That is the difference between a tree you can read and an indentation
/// staircase.
///
/// Mirror of core `buildChangedFileTree`, driven by the shared vectors in
/// `vectors/changed-file-tree.json`.
pub fn build_changed_file_tree(files: &[ChangedFile]) -> Vec<ChangedFileNode> {
    fn insert(nodes: &mut Vec<ChangedFileNode>, segments: &[&str], prefix: &str, file: &ChangedFile) {
        let Some((head, rest)) = segments.split_first() else {
            return;
        };
        let path = if prefix.is_empty() {
            (*head).to_string()
        } else {
            format!("{prefix}/{head}")
        };

        let index = match nodes.iter().position(|n| n.path == path) {
            Some(i) => i,
            None => {
                nodes.push(ChangedFileNode {
                    path: path.clone(),
                    label: (*head).to_string(),
                    is_directory: !rest.is_empty(),
                    ..Default::default()
                });
                nodes.len() - 1
            }
        };

        nodes[index].additions += file.additions;
        nodes[index].deletions += file.deletions;
        insert(&mut nodes[index].children, rest, &path, file);
    }

    fn collapse(nodes: Vec<ChangedFileNode>) -> Vec<ChangedFileNode> {
        nodes
            .into_iter()
            .map(|node| {
                let mut current = node;
                let mut label = current.label.clone();

                // Only directories collapse, and only through single children —
                // a directory with two entries is a real fork and has to render
                // as one.
                while current.is_directory
                    && current.children.len() == 1
                    && current.children[0].is_directory
                {
                    let child = current.children.into_iter().next().expect("single child");
                    label = format!("{label}/{}", child.label);
                    current = child;
                }

                ChangedFileNode {
                    label,
                    children: collapse(current.children),
                    ..current
                }
            })
            .collect()
    }

    let mut roots: Vec<ChangedFileNode> = Vec::new();
    for file in files {
        let segments: Vec<&str> = file.path.split('/').filter(|s| !s.is_empty()).collect();
        insert(&mut roots, &segments, "", file);
    }

    collapse(roots)
}

/// Top-level directories with their file counts, for the collapsed summary.
pub fn changed_file_scopes(files: &[ChangedFile]) -> Vec<(String, usize)> {
    let mut order: Vec<String> = Vec::new();
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for file in files {
        let name = file
            .path
            .split('/')
            .find(|s| !s.is_empty())
            .unwrap_or(file.path.as_str())
            .to_string();

        if !counts.contains_key(&name) {
            order.push(name.clone());
        }
        *counts.entry(name).or_insert(0) += 1;
    }

    // Insertion order, matching the JS `Map` the TS side iterates.
    order
        .into_iter()
        .map(|name| {
            let count = counts[&name];
            (name, count)
        })
        .collect()
}
