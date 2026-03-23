mod action_discovery_panel;
mod app_header;
mod audio_player;
mod autonomous_list;
mod block_editor;
mod card_radio_group;
mod command_palette;
mod confirm_action;
mod data_table;
mod detail_section;
mod detail_shell;
mod dock_region;
mod embed_input;
mod embed_preview;
mod empty_state;
mod filter_toolbar;
mod form_shell;
mod inline_editable_field;
mod inline_remediation;
mod log_list;
mod markdown_editor;
mod media_picker;
mod media_preview;
mod media_thumbnail;
mod metric_tile;
mod page_header;
mod page_loading;
mod pagination_summary;
mod picker_shell;
mod relation_picker;
mod remediation_banner;
mod reorderable_list;
mod selection_summary;
mod shell_status_bar;
mod slug_field;
mod split_view;
mod state_tile;
mod toast_stack;
mod types;
mod validation_summary;
mod video_player;

pub use action_discovery_panel::ActionDiscoveryPanelSpec;
pub use app_header::AppHeaderSpec;
pub use audio_player::AudioPlayerSpec;
pub use autonomous_list::AutonomousListSpec;
pub use block_editor::BlockEditorSpec;
pub use card_radio_group::CardRadioGroupSpec;
pub use command_palette::CommandPaletteSpec;
pub use confirm_action::ConfirmActionSpec;
pub use data_table::DataTableSpec;
pub use detail_section::DetailSectionSpec;
pub use detail_shell::{DetailShellSpec, DetailState};
pub use dock_region::{DockRegionSpec, DockTabsPlacement};
pub use embed_input::EmbedInputSpec;
pub use embed_preview::EmbedPreviewSpec;
pub use empty_state::EmptyStateSpec;
pub use filter_toolbar::FilterToolbarSpec;
pub use form_shell::FormShellSpec;
pub use inline_editable_field::InlineEditableFieldSpec;
pub use inline_remediation::InlineRemediationSpec;
pub use log_list::LogListSpec;
pub use markdown_editor::MarkdownEditorSpec;
pub use media_picker::MediaPickerSpec;
pub use media_preview::MediaPreviewSpec;
pub use media_thumbnail::MediaThumbnailSpec;
pub use metric_tile::MetricTileSpec;
pub use page_header::{PageHeaderAlign, PageHeaderSpec};
pub use page_loading::PageLoadingSpec;
pub use pagination_summary::PaginationSummarySpec;
pub use picker_shell::PickerShellSpec;
pub use relation_picker::RelationPickerSpec;
pub use remediation_banner::RemediationBannerSpec;
pub use reorderable_list::ReorderableListSpec;
pub use selection_summary::SelectionSummarySpec;
pub use shell_status_bar::ShellStatusBarSpec;
pub use slug_field::SlugFieldSpec;
pub use split_view::SplitViewSpec;
pub use state_tile::StateTileSpec;
pub use toast_stack::{Toast, ToastPosition, ToastStackSpec, ToastTone};
pub use types::{
    ActionDiscoverySection, AnnouncementMode, AspectRatio, BrowseState,
    CommandActionItem, DiscoveryState, DockEdge, EmptyStateVariant, FormActionLayout,
    FormFieldState, FormSectionSpec, FormStatusSummary, MediaKind, MediaState, MinColumnWidth,
    PanelTabItem, PickerItemSpec, PickerVariant, RemediationAction, ScrollOwner, SelectionMode,
    SelectionSummaryItem, SplitOrientation, TableColumnSpec, TableRowSpec, TableSortDirection,
    ValidationSummaryEntry,
};
pub use validation_summary::ValidationSummarySpec;
pub use video_player::VideoPlayerSpec;

pub const CURRENT_GENERATION: &str = "g06.002";
pub const FORM_VALIDATION_REMEDIATION_EXPORTS: &[&str] = &[
    "FormShellSpec",
    "ValidationSummarySpec",
    "RemediationBannerSpec",
    "InlineRemediationSpec",
];
pub const DATA_BROWSE_DETAIL_PICKER_MEDIA_EXPORTS: &[&str] = &[
    "DataTableSpec",
    "DetailShellSpec",
    "FilterToolbarSpec",
    "PaginationSummarySpec",
    "EmptyStateSpec",
    "PickerShellSpec",
    "RelationPickerSpec",
    "SelectionSummarySpec",
    "MediaThumbnailSpec",
    "MediaPreviewSpec",
];

#[cfg(test)]
mod tests {
    use flint_primitives::{
        ButtonVariant, CheckState, FormActionAlign, StatusTone, ValidationState,
    };
    use flint_tokens::semantic;

    use super::{
        AnnouncementMode, AspectRatio, BrowseState, DataTableSpec, DetailShellSpec, DetailState,
        EmptyStateSpec, EmptyStateVariant, FilterToolbarSpec, FormFieldState, FormSectionSpec,
        FormShellSpec, InlineRemediationSpec, MediaKind,
        MediaPreviewSpec, MediaState, MediaThumbnailSpec, PaginationSummarySpec,
        PickerItemSpec, PickerShellSpec, PickerVariant, RelationPickerSpec, RemediationAction,
        RemediationBannerSpec, SelectionMode, SelectionSummaryItem,
        SelectionSummarySpec, TableColumnSpec, TableRowSpec, TableSortDirection,
        ValidationSummaryEntry, ValidationSummarySpec,
    };

    #[test]
    fn form_shell_counts_invalid_and_pending_fields_and_blocks_submit() {
        let spec = FormShellSpec::new("mix-review")
            .with_sections(vec![FormSectionSpec::new(
                "main",
                "Main",
                vec![String::from("title"), String::from("search")],
            )])
            .with_fields(vec![
                FormFieldState::new("title", "Title")
                    .with_validation_state(ValidationState::Invalid)
                    .with_message("Title is required."),
                FormFieldState::new("search", "Search")
                    .with_validation_state(ValidationState::Pending)
                    .with_message("Checking references."),
            ])
            .with_actions(FormActionAlign::End, 2);

        assert_eq!(spec.invalid_field_count(), 1);
        assert_eq!(spec.pending_field_count(), 1);
        assert!(spec.blocks_submission());
        assert_eq!(spec.resolved_status_tone(), StatusTone::Danger);
        assert_eq!(spec.section_gap_token(), semantic::SPACE_STACK_MD);
    }

    #[test]
    fn validation_summary_filters_to_active_entries_and_exposes_alert_role() {
        let spec = ValidationSummarySpec::new(vec![
            ValidationSummaryEntry::new(
                "title",
                "Project title",
                "Title is required.",
                ValidationState::Invalid,
            ),
            ValidationSummaryEntry::new(
                "search",
                "Asset search",
                "Checking references.",
                ValidationState::Pending,
            ),
        ])
        .with_title("Fix the following fields")
        .with_announce_mode(AnnouncementMode::Assertive)
        .with_include_pending(true);

        assert_eq!(spec.active_entries().len(), 2);
        assert_eq!(spec.blocking_entry_count(), 1);
        assert_eq!(spec.accessibility_role(), Some("alert"));
        assert_eq!(spec.border_token(), semantic::COLOR_STATUS_DANGER);
    }

    #[test]
    fn remediation_banner_reports_actions_and_urgency() {
        let spec = RemediationBannerSpec::new(
            "Review attention needed",
            "Resolve the blocking validation before publishing.",
        )
        .with_tone(StatusTone::Warning)
        .with_announce_mode(AnnouncementMode::Polite)
        .with_primary_action(
            RemediationAction::new("resolve", "Resolve").with_variant(ButtonVariant::Primary),
        )
        .with_secondary_action(RemediationAction::new("inspect", "Inspect"))
        .with_dismissible(true);

        assert_eq!(spec.action_count(), 2);
        assert_eq!(spec.accessibility_role(), Some("status"));
        assert_eq!(spec.border_token(), semantic::COLOR_STATUS_WARNING);
        assert_eq!(spec.background_token(), semantic::COLOR_BACKGROUND_PANEL);
    }

    #[test]
    fn inline_remediation_tracks_field_references_and_actionability() {
        let spec = InlineRemediationSpec::new("Validation is failing.")
            .with_tone(StatusTone::Danger)
            .with_title("Invalid form state")
            .with_referenced_field_ids(vec![String::from("title"), String::from("search")])
            .with_action(RemediationAction::new("jump", "Jump to field"));

        assert_eq!(spec.reference_count(), 2);
        assert!(spec.is_actionable());
        assert_eq!(spec.border_token(), semantic::COLOR_STATUS_DANGER);
        assert_eq!(spec.gap_token(), semantic::SPACE_STACK_SM);
    }

    #[test]
    fn data_table_reports_visible_scope_selection_and_sortability() {
        let spec = DataTableSpec::new(
            vec![
                TableColumnSpec::new("name", "Name").with_sortable(true),
                TableColumnSpec::new("owner", "Owner"),
            ],
            vec![
                TableRowSpec::new(
                    "a",
                    vec![(String::from("name"), String::from("Approval still"))],
                ),
                TableRowSpec::new(
                    "b",
                    vec![(String::from("name"), String::from("Stem waveform"))],
                ),
            ],
        )
        .with_selected_row_ids(vec![String::from("a")])
        .with_sort("name", TableSortDirection::Asc);

        assert_eq!(spec.visible_row_count(), 2);
        assert_eq!(spec.selected_visible_row_count(), 1);
        assert_eq!(spec.select_all_state(), CheckState::Mixed);
        assert_eq!(spec.sortable_column_count(), 1);
        assert_eq!(spec.header_fill_token(), semantic::COLOR_BACKGROUND_SURFACE);
    }

    #[test]
    fn detail_and_empty_state_surface_state_posture() {
        let detail = DetailShellSpec::new()
            .with_title("Delivery brief")
            .with_state(DetailState::Error);
        let empty = EmptyStateSpec::new("No assets yet")
            .with_variant(EmptyStateVariant::FirstRun)
            .with_actions(vec![RemediationAction::new("add", "Add first asset")]);

        assert!(!detail.has_ready_content());
        assert_eq!(detail.body_fill_token(), semantic::COLOR_BACKGROUND_PANEL);
        assert_eq!(empty.action_count(), 1);
        assert_eq!(empty.layout_gap_token(), semantic::SPACE_STACK_MD);
    }

    #[test]
    fn filter_toolbar_and_pagination_summary_report_result_state() {
        let toolbar = FilterToolbarSpec::new()
            .with_query("review")
            .with_active_filter_count(2)
            .with_result_count(8)
            .with_show_clear_action(true);
        let summary = PaginationSummarySpec::new(2, 25, 67);

        assert!(toolbar.has_active_filters());
        assert_eq!(toolbar.gap_token(), semantic::SPACE_INLINE_MD);
        assert_eq!(summary.start_index(), 26);
        assert_eq!(summary.end_index(), 50);
    }

    #[test]
    fn picker_and_relation_picker_preserve_selection_and_variant() {
        let picker = PickerShellSpec::new("Attach asset")
            .with_variant(PickerVariant::Modal)
            .with_selection_mode(SelectionMode::Multiple)
            .with_state(BrowseState::Ready)
            .with_query("stem")
            .with_result_count(3)
            .with_selected_count(2);
        let relation = RelationPickerSpec::new(vec![
            PickerItemSpec::new("a", "Approval still"),
            PickerItemSpec::new("b", "Stem waveform"),
        ])
        .with_selected_ids(vec![String::from("b")])
        .with_variant(PickerVariant::Popover)
        .with_selection_mode(SelectionMode::Single);

        assert!(picker.is_modal_like());
        assert_eq!(picker.summary_tone(), StatusTone::Neutral);
        assert_eq!(relation.selected_item_count(), 1);
        assert_eq!(relation.as_picker_shell("Attach").selected_count, 1);
    }

    #[test]
    fn selection_summary_and_media_specs_preserve_fallback_posture() {
        let summary = SelectionSummarySpec::new(vec![
            SelectionSummaryItem::new("a", "Approval still"),
            SelectionSummaryItem::new("b", "Stem waveform"),
        ])
        .with_clear_action(RemediationAction::new("clear", "Clear"));
        let thumbnail = MediaThumbnailSpec::new(MediaKind::Image)
            .with_state(MediaState::Error)
            .with_aspect_ratio(AspectRatio::Video)
            .with_title("Approval still");
        let preview = MediaPreviewSpec::new(MediaKind::Audio, "Stem waveform")
            .with_state(MediaState::Empty)
            .with_metadata(vec![String::from("01:42"), String::from("WAV")])
            .with_footer_actions(vec![RemediationAction::new("open", "Open external")]);

        assert_eq!(summary.selected_count(), 2);
        assert!(summary.has_clear_action());
        assert!(thumbnail.shows_fallback_copy());
        assert!(thumbnail.caption_visible());
        assert_eq!(
            thumbnail.frame_fill_token(),
            semantic::COLOR_BACKGROUND_SURFACE
        );
        assert_eq!(preview.metadata_count(), 2);
        assert!(preview.has_footer_actions());
        assert!(preview.shows_fallback_copy());
    }
}
