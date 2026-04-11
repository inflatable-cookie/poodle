//! RenderComponent implementations for data, browse, detail, and media composites.
//!
//! g07.008: DataTableSpec, DetailShellSpec,
//! DetailSectionSpec, FilterToolbarSpec, PickerShellSpec, RelationPickerSpec,
//! SelectionSummarySpec, PaginationSummarySpec, MediaThumbnailSpec, MediaPreviewSpec

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    DataTableSpec, DetailSectionSpec, DetailShellSpec, FilterToolbarSpec,
    MediaPreviewSpec, MediaThumbnailSpec, PaginationSummarySpec, PickerShellSpec,
    RelationPickerSpec, SelectionSummarySpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

impl RenderComponent<DataTableSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &DataTableSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("data-table", "DataTableSpec")
    }
}

impl RenderComponent<DetailShellSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &DetailShellSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("detail-shell", "DetailShellSpec")
    }
}

impl RenderComponent<DetailSectionSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &DetailSectionSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("detail-section", "DetailSectionSpec")
    }
}

impl RenderComponent<FilterToolbarSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &FilterToolbarSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("filter-toolbar", "FilterToolbarSpec")
    }
}

impl RenderComponent<PickerShellSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &PickerShellSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("picker-shell", "PickerShellSpec")
    }
}

impl RenderComponent<RelationPickerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &RelationPickerSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("relation-picker", "RelationPickerSpec")
    }
}

impl RenderComponent<SelectionSummarySpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &SelectionSummarySpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("selection-summary", "SelectionSummarySpec")
    }
}

impl RenderComponent<PaginationSummarySpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &PaginationSummarySpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("pagination-summary", "PaginationSummarySpec")
    }
}

impl RenderComponent<MediaThumbnailSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &MediaThumbnailSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("media-thumbnail", "MediaThumbnailSpec")
    }
}

impl RenderComponent<MediaPreviewSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &MediaPreviewSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("media-preview", "MediaPreviewSpec")
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::RenderComponent;
    use poodle_specs::*;
    use poodle_style::StyleDescriptor;
    use crate::{GpuiAdapter, theme::GpuiThemeProvider};

    fn a() -> GpuiAdapter { GpuiAdapter::new(GpuiThemeProvider::default()) }
    fn s() -> StyleDescriptor { StyleDescriptor::new() }
    fn t() -> GpuiThemeProvider { GpuiThemeProvider::default() }

    #[test] fn data_table() { assert_eq!(a().render(&DataTableSpec::new(vec![], vec![]), &s(), &t()).spec_type, "DataTableSpec"); }
    #[test] fn detail_shell() { assert_eq!(a().render(&DetailShellSpec::new(), &s(), &t()).spec_type, "DetailShellSpec"); }
    #[test] fn detail_section() { assert_eq!(a().render(&DetailSectionSpec::new(), &s(), &t()).spec_type, "DetailSectionSpec"); }
    #[test] fn filter_toolbar() { assert_eq!(a().render(&FilterToolbarSpec::new(), &s(), &t()).spec_type, "FilterToolbarSpec"); }
    #[test] fn picker_shell() { assert_eq!(a().render(&PickerShellSpec::new("Pick"), &s(), &t()).spec_type, "PickerShellSpec"); }
    #[test] fn relation_picker() { assert_eq!(a().render(&RelationPickerSpec::new(vec![]), &s(), &t()).spec_type, "RelationPickerSpec"); }
    #[test] fn selection_summary() { assert_eq!(a().render(&SelectionSummarySpec::new(vec![]), &s(), &t()).spec_type, "SelectionSummarySpec"); }
    #[test] fn pagination_summary() { assert_eq!(a().render(&PaginationSummarySpec::new(1, 10, 50), &s(), &t()).spec_type, "PaginationSummarySpec"); }
    #[test] fn media_thumbnail() { assert_eq!(a().render(&MediaThumbnailSpec::new(MediaKind::Image), &s(), &t()).spec_type, "MediaThumbnailSpec"); }
    #[test] fn media_preview() { assert_eq!(a().render(&MediaPreviewSpec::new(MediaKind::Audio, "Test"), &s(), &t()).spec_type, "MediaPreviewSpec"); }
}
