//! RenderComponent implementations for all composites.
//!
//! g08.009–010: Form/validation, data/browse, editing, media, navigation,
//! and operational composites — full parity with GPUI adapter.

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    ActionDiscoveryPanelSpec, AppHeaderSpec, AudioPlayerSpec, BlockEditorSpec, CardRadioGroupSpec,
    CommandPaletteSpec, ConfirmActionSpec, DataTableSpec, DetailSectionSpec, DetailShellSpec,
    DockRegionSpec, EditableListSpec, EmbedInputSpec, EmbedPreviewSpec, EmptyStateSpec,
    FilterToolbarSpec, FormShellSpec, InlineRemediationSpec, ListContainerSpec, LogListSpec,
    MarkdownEditorSpec, MediaBrowsePanelSpec, MediaPickerSpec, MediaPreviewSpec,
    MediaThumbnailSpec, MessageCenterSpec, MetricTileSpec, PageHeaderSpec, PageLoadingSpec,
    PaginationSummarySpec, PickerShellSpec, RelationPickerSpec, RemediationBannerSpec,
    SelectionSummarySpec, ShellStatusBarSpec, SidebarNavSpec, SplitViewSpec, StateTileSpec,
    ToastHostSpec, ToastStackSpec, ValidationSummarySpec, VideoPlayerSpec,
};
use poodle_specs::{
    BreadcrumbsSpec, FilterBuilderSpec, ListCardSpec, NavCardSpec, OrderBySpec, ThemeSelectSpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{JetstreamAdapter, JetstreamNodeHandle, JetstreamTarget, WidgetKind};

// g08.009 — Form and validation composites

impl RenderComponent<FormShellSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &FormShellSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("form-shell", "FormShellSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<ValidationSummarySpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &ValidationSummarySpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "validation-summary",
            "ValidationSummarySpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<RemediationBannerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &RemediationBannerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "remediation-banner",
            "RemediationBannerSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<InlineRemediationSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &InlineRemediationSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "inline-remediation",
            "InlineRemediationSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<ConfirmActionSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &ConfirmActionSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "confirm-action",
            "ConfirmActionSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

// g08.010 — Data and browse composites

impl RenderComponent<DataTableSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &DataTableSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("data-table", "DataTableSpec", WidgetKind::List, mapped)
    }
}

impl RenderComponent<DetailShellSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &DetailShellSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("detail-shell", "DetailShellSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<DetailSectionSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &DetailSectionSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "detail-section",
            "DetailSectionSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<FilterToolbarSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &FilterToolbarSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "filter-toolbar",
            "FilterToolbarSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<PickerShellSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &PickerShellSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("picker-shell", "PickerShellSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<SelectionSummarySpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &SelectionSummarySpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "selection-summary",
            "SelectionSummarySpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<PaginationSummarySpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &PaginationSummarySpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "pagination-summary",
            "PaginationSummarySpec",
            WidgetKind::Label,
            mapped,
        )
    }
}

impl RenderComponent<EmptyStateSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &EmptyStateSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("empty-state", "EmptyStateSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<PageHeaderSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &PageHeaderSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("page-header", "PageHeaderSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<PageLoadingSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &PageLoadingSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("page-loading", "PageLoadingSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<StateTileSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &StateTileSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("state-tile", "StateTileSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<ToastStackSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &ToastStackSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("toast-stack", "ToastStackSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<LogListSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &LogListSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("log-list", "LogListSpec", WidgetKind::List, mapped)
    }
}

impl RenderComponent<NavCardSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &NavCardSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("nav-card", "NavCardSpec", WidgetKind::Button, mapped)
    }
}

impl RenderComponent<ListCardSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &ListCardSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("list-card", "ListCardSpec", WidgetKind::Button, mapped)
    }
}

// Editing, media, navigation, and list composites (full parity)

impl RenderComponent<AudioPlayerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &AudioPlayerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("audio-player", "AudioPlayerSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<VideoPlayerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &VideoPlayerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("video-player", "VideoPlayerSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<MediaPickerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &MediaPickerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("media-picker", "MediaPickerSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<MediaThumbnailSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &MediaThumbnailSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "media-thumbnail",
            "MediaThumbnailSpec",
            WidgetKind::Image,
            mapped,
        )
    }
}

impl RenderComponent<MediaPreviewSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &MediaPreviewSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "media-preview",
            "MediaPreviewSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<MarkdownEditorSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &MarkdownEditorSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "markdown-editor",
            "MarkdownEditorSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<BlockEditorSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &BlockEditorSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("block-editor", "BlockEditorSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<EmbedInputSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &EmbedInputSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "embed-input",
            "EmbedInputSpec",
            WidgetKind::TextInput,
            mapped,
        )
    }
}

impl RenderComponent<EmbedPreviewSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &EmbedPreviewSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "embed-preview",
            "EmbedPreviewSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<BreadcrumbsSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &BreadcrumbsSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("breadcrumbs", "BreadcrumbsSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<CardRadioGroupSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &CardRadioGroupSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "card-radio-group",
            "CardRadioGroupSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<OrderBySpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &OrderBySpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("order-by", "OrderBySpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<FilterBuilderSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &FilterBuilderSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "filter-builder",
            "FilterBuilderSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<ThemeSelectSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &ThemeSelectSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("theme-select", "ThemeSelectSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<RelationPickerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &RelationPickerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "relation-picker",
            "RelationPickerSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

// New composites — full contract coverage

impl RenderComponent<ActionDiscoveryPanelSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &ActionDiscoveryPanelSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "action-discovery-panel",
            "ActionDiscoveryPanelSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<MessageCenterSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &MessageCenterSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "message-center",
            "MessageCenterSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<AppHeaderSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &AppHeaderSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("app-header", "AppHeaderSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<CommandPaletteSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &CommandPaletteSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "command-palette",
            "CommandPaletteSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<DockRegionSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &DockRegionSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("dock-region", "DockRegionSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<EditableListSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &EditableListSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "editable-list",
            "EditableListSpec",
            WidgetKind::List,
            mapped,
        )
    }
}

impl RenderComponent<ListContainerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &ListContainerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "list-container",
            "ListContainerSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<MediaBrowsePanelSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &MediaBrowsePanelSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "media-browse-panel",
            "MediaBrowsePanelSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<MetricTileSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &MetricTileSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("metric-tile", "MetricTileSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<ShellStatusBarSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &ShellStatusBarSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new(
            "shell-status-bar",
            "ShellStatusBarSpec",
            WidgetKind::Panel,
            mapped,
        )
    }
}

impl RenderComponent<SplitViewSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &SplitViewSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("split-view", "SplitViewSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<ToastHostSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &ToastHostSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("toast-host", "ToastHostSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<SidebarNavSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(
        &self,
        _spec: &SidebarNavSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("sidebar-nav", "SidebarNavSpec", WidgetKind::List, mapped)
    }
}

#[cfg(test)]
mod tests {
    use crate::{theme::JetstreamThemeProvider, JetstreamAdapter, WidgetKind};
    use poodle_adapter::RenderComponent;
    use poodle_specs::*;
    use poodle_specs::{BreadcrumbsSpec, ListCardSpec, NavCardSpec, OrderBySpec};
    use poodle_style::StyleDescriptor;

    fn a() -> JetstreamAdapter {
        JetstreamAdapter::new(JetstreamThemeProvider::default())
    }
    fn s() -> StyleDescriptor {
        StyleDescriptor::new()
    }
    fn t() -> JetstreamThemeProvider {
        JetstreamThemeProvider::default()
    }

    // Form composites
    #[test]
    fn form_shell() {
        assert_eq!(
            a().render(&FormShellSpec::new("f"), &s(), &t()).spec_type,
            "FormShellSpec"
        );
    }
    #[test]
    fn validation_summary() {
        assert_eq!(
            a().render(&ValidationSummarySpec::new(vec![]), &s(), &t())
                .spec_type,
            "ValidationSummarySpec"
        );
    }
    #[test]
    fn remediation_banner() {
        assert_eq!(
            a().render(&RemediationBannerSpec::new("t", "m"), &s(), &t())
                .spec_type,
            "RemediationBannerSpec"
        );
    }
    #[test]
    fn inline_remediation() {
        assert_eq!(
            a().render(&InlineRemediationSpec::new("m"), &s(), &t())
                .spec_type,
            "InlineRemediationSpec"
        );
    }
    #[test]
    fn confirm_action() {
        assert_eq!(
            a().render(&ConfirmActionSpec::new("t", "m", "y", "n"), &s(), &t())
                .spec_type,
            "ConfirmActionSpec"
        );
    }

    // Data composites
    #[test]
    fn data_table() {
        assert_eq!(
            a().render(&DataTableSpec::new(vec![], vec![]), &s(), &t())
                .widget_kind,
            WidgetKind::List
        );
    }
    #[test]
    fn detail_shell() {
        assert_eq!(
            a().render(&DetailShellSpec::new(), &s(), &t()).spec_type,
            "DetailShellSpec"
        );
    }
    #[test]
    fn detail_section() {
        assert_eq!(
            a().render(&DetailSectionSpec::new(), &s(), &t()).spec_type,
            "DetailSectionSpec"
        );
    }
    #[test]
    fn filter_toolbar() {
        assert_eq!(
            a().render(&FilterToolbarSpec::new(), &s(), &t()).spec_type,
            "FilterToolbarSpec"
        );
    }
    #[test]
    fn picker_shell() {
        assert_eq!(
            a().render(&PickerShellSpec::new("P"), &s(), &t()).spec_type,
            "PickerShellSpec"
        );
    }
    #[test]
    fn selection_summary() {
        assert_eq!(
            a().render(&SelectionSummarySpec::new(vec![]), &s(), &t())
                .spec_type,
            "SelectionSummarySpec"
        );
    }
    #[test]
    fn pagination_summary() {
        assert_eq!(
            a().render(&PaginationSummarySpec::new(1, 10, 50), &s(), &t())
                .widget_kind,
            WidgetKind::Label
        );
    }
    #[test]
    fn empty_state() {
        assert_eq!(
            a().render(&EmptyStateSpec::new("Empty"), &s(), &t())
                .spec_type,
            "EmptyStateSpec"
        );
    }
    #[test]
    fn page_header() {
        assert_eq!(
            a().render(&PageHeaderSpec::new("Title"), &s(), &t())
                .spec_type,
            "PageHeaderSpec"
        );
    }
    #[test]
    fn page_loading() {
        assert_eq!(
            a().render(&PageLoadingSpec::new(), &s(), &t()).spec_type,
            "PageLoadingSpec"
        );
    }
    #[test]
    fn state_tile() {
        assert_eq!(
            a().render(&StateTileSpec::new("L", "V"), &s(), &t())
                .spec_type,
            "StateTileSpec"
        );
    }
    #[test]
    fn toast_stack() {
        assert_eq!(
            a().render(&ToastStackSpec::new(), &s(), &t()).spec_type,
            "ToastStackSpec"
        );
    }
    #[test]
    fn log_list() {
        assert_eq!(
            a().render(&LogListSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::List
        );
    }
    #[test]
    fn nav_card() {
        assert_eq!(
            a().render(&NavCardSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::Button
        );
    }
    #[test]
    fn list_card() {
        assert_eq!(
            a().render(&ListCardSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::Button
        );
    }

    // Editing, media, navigation, and list composites
    #[test]
    fn audio_player() {
        assert_eq!(
            a().render(&AudioPlayerSpec::new("audio.mp3"), &s(), &t())
                .spec_type,
            "AudioPlayerSpec"
        );
    }
    #[test]
    fn video_player() {
        assert_eq!(
            a().render(&VideoPlayerSpec::new("video.mp4"), &s(), &t())
                .spec_type,
            "VideoPlayerSpec"
        );
    }
    #[test]
    fn media_picker() {
        assert_eq!(
            a().render(&MediaPickerSpec::new("Pick"), &s(), &t())
                .spec_type,
            "MediaPickerSpec"
        );
    }
    #[test]
    fn media_thumbnail() {
        assert_eq!(
            a().render(
                &MediaThumbnailSpec::new(poodle_specs::MediaKind::Image),
                &s(),
                &t()
            )
            .widget_kind,
            WidgetKind::Image
        );
    }
    #[test]
    fn media_preview() {
        assert_eq!(
            a().render(
                &MediaPreviewSpec::new(poodle_specs::MediaKind::Image, "Preview"),
                &s(),
                &t()
            )
            .spec_type,
            "MediaPreviewSpec"
        );
    }
    #[test]
    fn markdown_editor() {
        assert_eq!(
            a().render(&MarkdownEditorSpec::new(), &s(), &t()).spec_type,
            "MarkdownEditorSpec"
        );
    }
    #[test]
    fn block_editor() {
        assert_eq!(
            a().render(&BlockEditorSpec::new(), &s(), &t()).spec_type,
            "BlockEditorSpec"
        );
    }
    #[test]
    fn embed_input() {
        assert_eq!(
            a().render(&EmbedInputSpec::new(), &s(), &t()).spec_type,
            "EmbedInputSpec"
        );
    }
    #[test]
    fn embed_preview() {
        assert_eq!(
            a().render(&EmbedPreviewSpec::new(), &s(), &t()).spec_type,
            "EmbedPreviewSpec"
        );
    }
    #[test]
    fn editable_list() {
        assert_eq!(
            a().render(&EditableListSpec::new(), &s(), &t()).widget_kind,
            WidgetKind::List
        );
    }
    #[test]
    fn breadcrumbs() {
        assert_eq!(
            a().render(&BreadcrumbsSpec::new(vec![]), &s(), &t())
                .spec_type,
            "BreadcrumbsSpec"
        );
    }
    #[test]
    fn card_radio_group() {
        assert_eq!(
            a().render(&CardRadioGroupSpec::new(vec![]), &s(), &t())
                .spec_type,
            "CardRadioGroupSpec"
        );
    }
    #[test]
    fn order_by() {
        assert_eq!(
            a().render(&OrderBySpec::new(), &s(), &t()).spec_type,
            "OrderBySpec"
        );
    }
    #[test]
    fn relation_picker() {
        assert_eq!(
            a().render(&RelationPickerSpec::new(vec![]), &s(), &t())
                .spec_type,
            "RelationPickerSpec"
        );
    }

    // New composites
    #[test]
    fn action_discovery_panel() {
        assert_eq!(
            a().render(&ActionDiscoveryPanelSpec::default(), &s(), &t())
                .spec_type,
            "ActionDiscoveryPanelSpec"
        );
    }
    #[test]
    fn message_center() {
        assert_eq!(
            a().render(&MessageCenterSpec::default(), &s(), &t())
                .spec_type,
            "MessageCenterSpec"
        );
    }
    #[test]
    fn app_header() {
        assert_eq!(
            a().render(&AppHeaderSpec::default(), &s(), &t()).spec_type,
            "AppHeaderSpec"
        );
    }
    #[test]
    fn command_palette() {
        assert_eq!(
            a().render(&CommandPaletteSpec::default(), &s(), &t())
                .spec_type,
            "CommandPaletteSpec"
        );
    }
    #[test]
    fn dock_region() {
        assert_eq!(
            a().render(
                &DockRegionSpec::new(poodle_specs::DockEdge::Left, vec![]),
                &s(),
                &t()
            )
            .spec_type,
            "DockRegionSpec"
        );
    }
    #[test]
    fn editable_list_spec_type() {
        assert_eq!(
            a().render(&EditableListSpec::new(), &s(), &t()).spec_type,
            "EditableListSpec"
        );
    }
    #[test]
    fn list_container() {
        assert_eq!(
            a().render(&ListContainerSpec::default(), &s(), &t())
                .spec_type,
            "ListContainerSpec"
        );
    }
    #[test]
    fn media_browse_panel() {
        assert_eq!(
            a().render(&MediaBrowsePanelSpec::new(), &s(), &t())
                .spec_type,
            "MediaBrowsePanelSpec"
        );
    }
    #[test]
    fn metric_tile() {
        assert_eq!(
            a().render(&MetricTileSpec::new("L", "V"), &s(), &t())
                .spec_type,
            "MetricTileSpec"
        );
    }
    #[test]
    fn shell_status_bar() {
        assert_eq!(
            a().render(&ShellStatusBarSpec::default(), &s(), &t())
                .spec_type,
            "ShellStatusBarSpec"
        );
    }
    #[test]
    fn split_view() {
        assert_eq!(
            a().render(
                &SplitViewSpec::new(
                    "split-view:compile-check",
                    poodle_specs::SplitOrientation::Horizontal
                ),
                &s(),
                &t()
            )
            .spec_type,
            "SplitViewSpec"
        );
    }
    #[test]
    fn toast_host() {
        assert_eq!(
            a().render(&ToastHostSpec::new(), &s(), &t()).spec_type,
            "ToastHostSpec"
        );
    }
}
