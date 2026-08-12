//! RefSelectSpec — version-control ref chooser, Rust mirror of the
//! `@inflatable-cookie/poodle-svelte` RefSelect type model and `ref-select-model.ts` logic.
//!
//! Contract: `docs/contracts/components/ref-select.md`.
//!
//! Poodle knows the shape of a ref, never git itself: no fetching, no parsing,
//! no ahead/behind maths. The host owns the list and what selecting one means.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

pub use crate::model_picker::{
    ModelPickerEmphasis as RefSelectEmphasis, ModelPickerVariant as RefSelectVariant,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RefKind {
    #[default]
    Branch,
    Tag,
    Commit,
}

impl RefKind {
    pub fn as_str(self) -> &'static str {
        match self {
            RefKind::Branch => "branch",
            RefKind::Tag => "tag",
            RefKind::Commit => "commit",
        }
    }

    /// Default glyph per kind (contract §2).
    pub fn icon(self) -> &'static str {
        match self {
            RefKind::Branch => "git-branch",
            RefKind::Tag => "tag",
            RefKind::Commit => "git-commit-horizontal",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RefOption {
    pub value: String,
    pub label: String,
    pub kind: RefKind,
    /// Secondary line — a short sha, an ahead/behind summary, a commit subject.
    pub description: Option<String>,
    /// Overrides the kind glyph.
    pub icon: Option<String>,
    pub group: Option<String>,
    pub is_disabled: bool,
}

impl RefOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            kind: RefKind::Branch,
            description: None,
            icon: None,
            group: None,
            is_disabled: false,
        }
    }

    pub fn with_kind(mut self, kind: RefKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    /// The glyph this row renders: its own override, else the kind glyph.
    pub fn resolved_icon(&self) -> &str {
        self.icon.as_deref().unwrap_or_else(|| self.kind.icon())
    }

    /// Case-insensitive substring match across label, then value, then
    /// description (contract §4 Filtering).
    pub fn matches(&self, needle: &str) -> bool {
        let needle = needle.trim().to_lowercase();
        if needle.is_empty() {
            return true;
        }
        [
            self.label.as_str(),
            self.value.as_str(),
            self.description.as_deref().unwrap_or(""),
        ]
        .iter()
        .any(|text| text.to_lowercase().contains(&needle))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RefSelectSpec {
    pub refs: Vec<RefOption>,
    pub value: String,
    /// The checked-out ref, marked in the list. Often equals `value`, but a host
    /// browsing another ref keeps the marker where it belongs.
    pub current_ref: Option<String>,
    pub current_label: String,
    pub placeholder: String,
    pub is_searchable: bool,
    /// Controlled query. `Some` means the host is filtering and the passed list
    /// is already the answer.
    pub search_value: Option<String>,
    pub search_placeholder: String,
    pub search_label: String,
    pub is_loading: bool,
    pub loading_label: String,
    pub empty_label: String,
    pub aria_label: String,
    pub is_disabled: bool,
    pub variant: RefSelectVariant,
    pub emphasis: RefSelectEmphasis,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    pub is_open: bool,
    /// Refuses outside-interact dismissal when false. Matches Svelte
    /// `dismissOnOutsideInteract` (default `true`).
    pub dismiss_on_outside_interact: bool,
}

impl Default for RefSelectSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl RefSelectSpec {
    pub fn new() -> Self {
        Self {
            refs: Vec::new(),
            value: String::new(),
            current_ref: None,
            current_label: "current".to_string(),
            placeholder: "Select ref".to_string(),
            is_searchable: true,
            search_value: None,
            search_placeholder: "Search refs…".to_string(),
            search_label: "Search refs".to_string(),
            is_loading: false,
            loading_label: "Loading more refs…".to_string(),
            empty_label: "No refs found".to_string(),
            aria_label: "Ref".to_string(),
            is_disabled: false,
            variant: RefSelectVariant::Bare,
            emphasis: RefSelectEmphasis::Default,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            is_open: false,
            dismiss_on_outside_interact: true,
        }
    }

    pub fn with_refs(mut self, refs: Vec<RefOption>) -> Self {
        self.refs = refs;
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_current_ref(mut self, value: impl Into<String>) -> Self {
        self.current_ref = Some(value.into());
        self
    }

    pub fn with_current_label(mut self, label: impl Into<String>) -> Self {
        self.current_label = label.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_searchable(mut self, searchable: bool) -> Self {
        self.is_searchable = searchable;
        self
    }

    pub fn with_search_value(mut self, query: impl Into<String>) -> Self {
        self.search_value = Some(query.into());
        self
    }

    pub fn with_loading(mut self, loading: bool) -> Self {
        self.is_loading = loading;
        self
    }

    pub fn with_loading_label(mut self, label: impl Into<String>) -> Self {
        self.loading_label = label.into();
        self
    }

    pub fn with_empty_label(mut self, label: impl Into<String>) -> Self {
        self.empty_label = label.into();
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = label.into();
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_variant(mut self, variant: RefSelectVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_emphasis(mut self, emphasis: RefSelectEmphasis) -> Self {
        self.emphasis = emphasis;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_dismiss_on_outside_interact(mut self, dismiss_on_outside_interact: bool) -> Self {
        self.dismiss_on_outside_interact = dismiss_on_outside_interact;
        self
    }

    /// The refs the panel shows. A host-supplied `search_value` means the list is
    /// already filtered, so the spec passes it through untouched.
    pub fn visible_refs(&self) -> Vec<&RefOption> {
        match &self.search_value {
            Some(_) => self.refs.iter().collect(),
            None => self.refs.iter().collect(),
        }
    }

    /// The refs the panel shows for a locally-typed query (native previews hold
    /// the query in `search_value`, so this is the shared filter entry point).
    pub fn filtered_refs(&self, query: &str) -> Vec<&RefOption> {
        self.refs
            .iter()
            .filter(|option| option.matches(query))
            .collect()
    }

    /// Rows to render: the host's list when it drives search, else the local
    /// filter over the current query.
    pub fn rows(&self) -> Vec<&RefOption> {
        match self.search_value.as_deref() {
            Some(_) => self.visible_refs(),
            None => self.filtered_refs(""),
        }
    }

    pub fn selected(&self) -> Option<&RefOption> {
        self.refs.iter().find(|option| option.value == self.value)
    }

    pub fn has_selection(&self) -> bool {
        !self.value.is_empty()
    }

    /// Trigger label: the selected ref's label, the raw value when the host holds
    /// a ref outside the list, else the placeholder.
    pub fn trigger_label(&self) -> String {
        if !self.has_selection() {
            return self.placeholder.clone();
        }
        match self.selected() {
            Some(option) => option.label.clone(),
            None => self.value.clone(),
        }
    }

    pub fn trigger_icon(&self) -> String {
        match self.selected() {
            Some(option) => option.resolved_icon().to_string(),
            None => RefKind::Branch.icon().to_string(),
        }
    }

    pub fn trigger_aria_label(&self) -> String {
        format!("{}: {}", self.aria_label, self.trigger_label())
    }

    pub fn is_current(&self, option: &RefOption) -> bool {
        self.current_ref.as_deref() == Some(option.value.as_str())
    }

    /// Group heading to emit before this row, if it opens a new run. Runs are
    /// computed over the rendered rows, so a heading never survives its last
    /// matching row.
    pub fn group_heading_for<'a>(&self, rows: &[&'a RefOption], index: usize) -> Option<&'a str> {
        let group = rows.get(index)?.group.as_deref()?;
        let previous = index
            .checked_sub(1)
            .and_then(|prev| rows.get(prev))
            .and_then(|option| option.group.as_deref());
        if previous == Some(group) {
            None
        } else {
            Some(group)
        }
    }

    /// The empty message shows only when nothing matched *and* nothing is still
    /// arriving (contract §4).
    pub fn show_empty(&self) -> bool {
        self.rows().is_empty() && !self.is_loading
    }

    // ── Token accessors (shared by GPUI + Jetstream) ──────────────────────

    pub fn label_color_token(&self) -> &'static str {
        if self.emphasis.is_subdued() {
            semantic::COLOR_TEXT_SECONDARY
        } else {
            semantic::COLOR_TEXT_PRIMARY
        }
    }

    pub fn secondary_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn muted_color_token(&self) -> &'static str {
        "color.text.placeholder"
    }

    pub fn subdued_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_MUTED
    }

    pub fn trigger_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn trigger_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn surface_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn surface_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn item_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> RefSelectSpec {
        RefSelectSpec::new()
            .with_refs(vec![
                RefOption::new("main", "main")
                    .with_group("Branches")
                    .with_description("a1b2c3d"),
                RefOption::new("tree-component", "tree-component").with_group("Branches"),
                RefOption::new("v1.4.0", "v1.4.0")
                    .with_kind(RefKind::Tag)
                    .with_group("Tags"),
            ])
            .with_value("tree-component")
            .with_current_ref("main")
    }

    #[test]
    fn filtering_spans_label_value_and_description() {
        let spec = sample();
        assert_eq!(spec.filtered_refs("").len(), 3);
        assert_eq!(spec.filtered_refs("TREE").len(), 1);
        // A sha typed from a commit line finds its row.
        assert_eq!(spec.filtered_refs("a1b2").len(), 1);
        assert_eq!(spec.filtered_refs("nothing").len(), 0);
    }

    #[test]
    fn a_host_supplied_query_disables_local_filtering() {
        // The host already filtered: every passed ref renders, whatever the query.
        let spec = sample().with_search_value("zzz");
        assert_eq!(spec.rows().len(), 3);
    }

    #[test]
    fn current_marker_is_independent_of_selection() {
        let spec = sample();
        assert_eq!(spec.trigger_label(), "tree-component");
        let rows = spec.rows();
        assert!(spec.is_current(rows[0]), "main is checked out");
        assert!(
            !spec.is_current(rows[1]),
            "the selected ref is not the current one"
        );
    }

    #[test]
    fn kind_drives_the_glyph_unless_overridden() {
        assert_eq!(RefOption::new("main", "main").resolved_icon(), "git-branch");
        assert_eq!(
            RefOption::new("v1", "v1")
                .with_kind(RefKind::Tag)
                .resolved_icon(),
            "tag"
        );
        assert_eq!(
            RefOption::new("v1", "v1").with_icon("star").resolved_icon(),
            "star"
        );
        // An unselected trigger still shows a ref glyph rather than nothing.
        assert_eq!(RefSelectSpec::new().trigger_icon(), "git-branch");
    }

    #[test]
    fn group_headings_emit_once_per_run() {
        let spec = sample();
        let rows = spec.rows();
        assert_eq!(spec.group_heading_for(&rows, 0), Some("Branches"));
        assert_eq!(spec.group_heading_for(&rows, 1), None);
        assert_eq!(spec.group_heading_for(&rows, 2), Some("Tags"));
    }

    #[test]
    fn loading_suppresses_the_empty_message() {
        let empty = RefSelectSpec::new();
        assert!(empty.show_empty());
        // Results may still be arriving — do not claim there are none.
        assert!(!empty.with_loading(true).show_empty());
    }

    #[test]
    fn placeholder_and_unknown_ref_labels() {
        assert_eq!(RefSelectSpec::new().trigger_label(), "Select ref");
        assert_eq!(sample().with_value("ghost").trigger_label(), "ghost");
    }
}
