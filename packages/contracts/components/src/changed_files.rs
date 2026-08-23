//! ChangedFilesSpec — what a turn touched.
//!
//! Contract: `docs/contracts/components/changed-files.md`.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::agent_transcript::{
    build_changed_file_tree, changed_file_scopes, changed_files_totals, ChangedFile,
    ChangedFileNode, ChangedFilesTotals,
};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct ChangedFilesSpec {
    pub id: String,
    pub files: Vec<ChangedFile>,
    pub is_expanded: bool,
    pub expanded_paths: Vec<String>,
    pub chip_limit: usize,
    /// The web prop is a formatter, `(count) => string`. A Rust spec holds data
    /// rather than closures, so the native surface is an optional resolved
    /// override; `None` uses the default phrasing. See the contract's deltas.
    pub count_label: Option<String>,
    pub show_open_diff: bool,
    pub open_diff_label: String,
    pub show_files_label: String,
    pub hide_files_label: String,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for ChangedFilesSpec {
    fn default() -> Self {
        Self::new("", Vec::new())
    }
}

impl ChangedFilesSpec {
    pub fn new(id: impl Into<String>, files: Vec<ChangedFile>) -> Self {
        Self {
            id: id.into(),
            files,
            is_expanded: false,
            expanded_paths: Vec::new(),
            chip_limit: 3,
            count_label: None,
            show_open_diff: true,
            open_diff_label: "Open diff".to_string(),
            show_files_label: "Show files".to_string(),
            hide_files_label: "Hide files".to_string(),
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }

    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }
    pub fn with_expanded_paths(mut self, paths: Vec<String>) -> Self {
        self.expanded_paths = paths;
        self
    }
    pub fn with_chip_limit(mut self, limit: usize) -> Self {
        self.chip_limit = limit;
        self
    }
    pub fn with_count_label(mut self, label: impl Into<String>) -> Self {
        self.count_label = Some(label.into());
        self
    }
    pub fn with_show_open_diff(mut self, show: bool) -> Self {
        self.show_open_diff = show;
        self
    }
    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }
    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    /// An empty card renders nothing rather than an empty state. A turn that
    /// changed no files should not have a box saying so.
    pub fn renders(&self) -> bool {
        !self.files.is_empty()
    }

    pub fn totals(&self) -> ChangedFilesTotals {
        changed_files_totals(&self.files)
    }

    pub fn tree(&self) -> Vec<ChangedFileNode> {
        build_changed_file_tree(&self.files)
    }

    pub fn scopes(&self) -> Vec<(String, usize)> {
        changed_file_scopes(&self.files)
    }

    pub fn visible_chips(&self) -> &[ChangedFile] {
        let end = self.chip_limit.min(self.files.len());
        &self.files[..end]
    }

    pub fn resolved_count_label(&self) -> String {
        self.count_label
            .clone()
            .unwrap_or_else(|| format!("{} changed files", self.totals().file_count))
    }

    /// Counts are colour-coded, and colour alone is not a signal.
    pub fn accessible_name(&self) -> String {
        let totals = self.totals();
        format!(
            "{}, {} added, {} removed",
            self.resolved_count_label(),
            totals.additions,
            totals.deletions
        )
    }

    // ── Tokens ────────────────────────────────────────────────
    pub fn surface_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }
    pub fn count_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }
    pub fn additions_token(&self) -> &'static str {
        semantic::COLOR_STATUS_SUCCESS
    }
    pub fn deletions_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }
    pub fn scope_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
    pub fn chip_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
    pub fn chip_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    // ── Size ─────────────────────────────────────────────────
    pub fn font_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }
    pub fn icon_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.75,
            ControlSize::Sm => 0.8125,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 1.0,
            ControlSize::Xl => 1.125,
        }
    }

    // ── Density ──────────────────────────────────────────────
    pub fn padding_inset_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.0,
        }
    }
    pub fn gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(path: &str, additions: u32, deletions: u32) -> ChangedFile {
        ChangedFile {
            path: path.to_string(),
            additions,
            deletions,
            status: None,
        }
    }

    #[test]
    fn an_empty_card_does_not_render() {
        assert!(!ChangedFilesSpec::new("c", Vec::new()).renders());
    }

    #[test]
    fn totals_sum_across_files() {
        let spec = ChangedFilesSpec::new("c", vec![file("a.rs", 361, 11), file("b.md", 15, 5)]);
        let totals = spec.totals();

        assert_eq!(totals.file_count, 2);
        assert_eq!(totals.additions, 376);
        assert_eq!(totals.deletions, 16);
        assert_eq!(
            spec.accessible_name(),
            "2 changed files, 376 added, 16 removed"
        );
    }

    #[test]
    fn chips_are_capped_without_panicking_on_short_lists() {
        let spec = ChangedFilesSpec::new("c", vec![file("a.rs", 1, 0)]);
        assert_eq!(spec.visible_chips().len(), 1);
        assert_eq!(spec.with_chip_limit(0).visible_chips().len(), 0);
    }

    #[test]
    fn a_chain_with_no_forks_collapses_to_one_row() {
        let spec = ChangedFilesSpec::new("c", vec![file("app/src/lib/editor/machine.ts", 12, 3)]);
        let tree = spec.tree();

        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].label, "app/src/lib/editor");
        assert_eq!(tree[0].additions, 12);
    }
}
