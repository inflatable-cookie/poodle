//! RenderComponent implementations for editing, navigation, list interaction,
//! and operational composites.
//!
//! g07.009: AudioPlayer, VideoPlayer, MediaPicker, MarkdownEditor, BlockEditor,
//! EmbedInput, EmbedPreview,
//! Breadcrumbs, CardRadioGroup, ListCard, NavCard,
//! OrderBy, PageHeader, LogList, PageLoading,
//! StateTile, ToastStack, EmptyState

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    AudioPlayerSpec, BlockEditorSpec, CardRadioGroupSpec, EmbedInputSpec, EmbedPreviewSpec,
    EmptyStateSpec, LogListSpec, MarkdownEditorSpec, MediaPickerSpec, PageHeaderSpec,
    PageLoadingSpec, StateTileSpec, ToastStackSpec, VideoPlayerSpec,
};
use poodle_specs::{BreadcrumbsSpec, FilterBuilderSpec, ListCardSpec, NavCardSpec, OrderBySpec, ThemeSelectSpec};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

impl RenderComponent<AudioPlayerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &AudioPlayerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("audio-player", "AudioPlayerSpec")
    }
}

impl RenderComponent<VideoPlayerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &VideoPlayerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("video-player", "VideoPlayerSpec")
    }
}

impl RenderComponent<MediaPickerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &MediaPickerSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("media-picker", "MediaPickerSpec")
    }
}

impl RenderComponent<MarkdownEditorSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &MarkdownEditorSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("markdown-editor", "MarkdownEditorSpec")
    }
}

impl RenderComponent<BlockEditorSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &BlockEditorSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("block-editor", "BlockEditorSpec")
    }
}

impl RenderComponent<EmbedInputSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &EmbedInputSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("embed-input", "EmbedInputSpec")
    }
}

impl RenderComponent<EmbedPreviewSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &EmbedPreviewSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("embed-preview", "EmbedPreviewSpec")
    }
}

impl RenderComponent<BreadcrumbsSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &BreadcrumbsSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("breadcrumbs", "BreadcrumbsSpec")
    }
}

impl RenderComponent<CardRadioGroupSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &CardRadioGroupSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("card-radio-group", "CardRadioGroupSpec")
    }
}

impl RenderComponent<ListCardSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &ListCardSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("list-card", "ListCardSpec")
    }
}

impl RenderComponent<NavCardSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &NavCardSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("nav-card", "NavCardSpec")
    }
}

impl RenderComponent<OrderBySpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &OrderBySpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("order-by", "OrderBySpec")
    }
}

impl RenderComponent<FilterBuilderSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &FilterBuilderSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("filter-builder", "FilterBuilderSpec")
    }
}

impl RenderComponent<ThemeSelectSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &ThemeSelectSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("theme-select", "ThemeSelectSpec")
    }
}

impl RenderComponent<PageHeaderSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &PageHeaderSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("page-header", "PageHeaderSpec")
    }
}

impl RenderComponent<PageLoadingSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &PageLoadingSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("page-loading", "PageLoadingSpec")
    }
}

impl RenderComponent<LogListSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &LogListSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("log-list", "LogListSpec")
    }
}

impl RenderComponent<StateTileSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &StateTileSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("state-tile", "StateTileSpec")
    }
}

impl RenderComponent<ToastStackSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &ToastStackSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("toast-stack", "ToastStackSpec")
    }
}

impl RenderComponent<EmptyStateSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &EmptyStateSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("empty-state", "EmptyStateSpec")
    }
}

#[cfg(test)]
mod tests {
    use crate::{theme::GpuiThemeProvider, GpuiAdapter};
    use poodle_adapter::RenderComponent;
    use poodle_specs::*;
    use poodle_specs::{BreadcrumbsSpec, ChoiceOption, ListCardSpec, NavCardSpec, OrderBySpec};
    use poodle_style::StyleDescriptor;

    fn a() -> GpuiAdapter {
        GpuiAdapter::new(GpuiThemeProvider::default())
    }
    fn s() -> StyleDescriptor {
        StyleDescriptor::new()
    }
    fn t() -> GpuiThemeProvider {
        GpuiThemeProvider::default()
    }

    #[test]
    fn audio_player() {
        assert_eq!(
            a().render(&AudioPlayerSpec::new("test.mp3"), &s(), &t())
                .spec_type,
            "AudioPlayerSpec"
        );
    }
    #[test]
    fn video_player() {
        assert_eq!(
            a().render(&VideoPlayerSpec::new("test.mp4"), &s(), &t())
                .spec_type,
            "VideoPlayerSpec"
        );
    }
    #[test]
    fn media_picker() {
        assert_eq!(
            a().render(&MediaPickerSpec::new("Pick media"), &s(), &t())
                .spec_type,
            "MediaPickerSpec"
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
            a().render(
                &CardRadioGroupSpec::new(vec![ChoiceOption::new("a", "A")]),
                &s(),
                &t()
            )
            .spec_type,
            "CardRadioGroupSpec"
        );
    }
    #[test]
    fn list_card() {
        assert_eq!(
            a().render(&ListCardSpec::new(), &s(), &t()).spec_type,
            "ListCardSpec"
        );
    }
    #[test]
    fn nav_card() {
        assert_eq!(
            a().render(&NavCardSpec::new(), &s(), &t()).spec_type,
            "NavCardSpec"
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
    fn log_list() {
        assert_eq!(
            a().render(&LogListSpec::new(), &s(), &t()).spec_type,
            "LogListSpec"
        );
    }
    #[test]
    fn state_tile() {
        assert_eq!(
            a().render(&StateTileSpec::new("CPU", "42%"), &s(), &t())
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
    fn empty_state() {
        assert_eq!(
            a().render(&EmptyStateSpec::new("Nothing here"), &s(), &t())
                .spec_type,
            "EmptyStateSpec"
        );
    }
}
