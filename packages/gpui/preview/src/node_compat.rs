//! Preview component facades backed by the shared render tree.
//!
//! Specimens use a compact `Type::from_spec(spec, theme)` call shape while
//! rendering through `poodle-render` and the GPUI node backend. This module
//! owns preview-only event wiring and slots; public component behavior remains
//! in the shared contracts and renderer.

use std::rc::Rc;
use std::sync::Arc;

use gpui::{
    div, px, AnyElement, App, Hsla, InteractiveElement, IntoElement, KeyDownEvent, ParentElement,
    Rgba, StatefulInteractiveElement, Styled, Window,
};
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_render::{AccordionHandlers, RenderContext, SlotBuilder};
use poodle_specs::{
    AccordionSelectionValue, AccordionSpec, ActionDiscoveryPanelSpec, AgentChatInputSpec, AgentMessageSpec,
    AgentPlanRecordSpec, AgentPlanSpec, AgentQuestionRecordSpec, AgentQuestionSpec,
    AgentSubagentSpec, AgentTranscriptSpec, AlertDialogSpec, AppHeaderSpec, AudioPlayerSpec,
    AvatarSpec, BlockEditorSpec, BoxSpec, BreadcrumbsSpec, BulkActionBarSpec, ButtonSpec,
    CalendarSpec, CallOutSpec, CardRadioGroupSpec, ChangedFilesSpec, CheckboxSpec, CodeInputSpec,
    CodeSpec, CollapseToggleSpec, CollapsibleSpec, ColorPickerSpec, CommandPaletteSpec,
    ConfirmActionSpec, ContextMenuSpec, ControlDensity, ControlSize, DataTableSpec, DatePickerSpec,
    DateRangePickerSpec, DateRangeValue, DateTimePickerSpec, DateTimeRangePickerSpec,
    DateTimeZonePickerSpec, DebugDialogSpec, DetailItemSpec, DetailSectionGroupSpec,
    DetailSectionSpec, DetailShellSpec, DialogSpec, DockRegionSpec, DrawerSpec, DurationInputSpec,
    EditableLabelSpec, EditableListSpec, EmbedInputSpec, EmbedPreviewSpec, EmptyStateSpec,
    ErrorBoundarySpec, EyebrowSpec, FieldSetSpec, FieldSpec, FileUploadSpec, FilterBuilderSpec,
    FilterToolbarSpec, FormActionsSpec, FormDialogSpec, FormLayoutSpec, FormShellSpec, GridSpec,
    HoverCardSpec, IconButtonSpec, IconSpec, InlineListSectionSpec, LicenceActivationSpec,
    LicenceSeatsSpec, LicenceStatusSpec, ListCardCounterSpec, ListCardSpec, ListContainerSpec,
    ListGridSpec, LogListSpec, MarkdownEditorSpec, MediaBrowsePanelSpec, MediaPickerSpec,
    MediaPreviewSpec, MediaThumbnailSpec, MenuSpec, MenubarSpec, MetaBarSpec, MetaItemSpec,
    MeterSpec, MetricTileSpec, ModelPickerSpec, NavCardSpec, NavigationMenuSpec, NumberInputSpec,
    OrderBySpec, OverlayPlacement, PageHeaderSpec, PageLoadingSpec, PaginationSpec,
    PaginationSummarySpec, PasswordRequirementsSpec, PickerShellSpec, PillSpec, PopoverSpec,
    ProgressSpec, RadioGroupSpec, RadioSpec, RangeSliderSpec, RatingSpec, RefSelectSpec,
    RegionSpec, RelationPickerSpec, RemediationBannerSpec, ResizeHandleSpec, ScrollShellSpec,
    SelectSpec, SelectionSummarySpec, SeparatorSpec, ShellStatusBarSpec, SidebarNavSpec,
    SkeletonSpec, SliderSpec, SpacerSpec, SpinnerSpec, SplitOrientation, SplitViewSpec, StackSpec,
    StateTileSpec, StatusIndicatorSpec, StepperSpec, SurfaceSpec, SwitchSpec, TabStripSpec,
    TableSpec, TabsSpec, TextInputSpec, TextLinkSpec, TextSpec, ThemeSelectSpec, TimeAgoSpec,
    TimeFieldSpec, TimeZoneSelectSpec, ToastHostSpec, ToastStackSpec, TokenInputSpec,
    ToolCallGroupSpec, ToolCallSpec, ToolbarSpec, TooltipSpec, TreeSpec, TriStateSwitchSpec,
    TriStateValue, ValidationSummarySpec, VideoPlayerSpec,
};
use poodle_tokens::typed::ColorValue;

use crate::app_state::NodeSpecimenEvent;
use poodle_gpui_node_backend::file_capability::SingleFilePickSpec;

type OpenChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

pub(crate) struct Eyebrow;

impl Eyebrow {
    pub(crate) fn from_spec(spec: EyebrowSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::eyebrow(&spec, &RenderContext::new(theme)))
    }
}

pub(crate) struct Text;

impl Text {
    pub(crate) fn from_spec(spec: TextSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::text(&spec, &RenderContext::new(theme)))
    }

    pub(crate) fn node_from_spec(spec: TextSpec, theme: &GpuiThemeProvider) -> poodle_node::Node {
        poodle_render::text(&spec, &RenderContext::new(theme))
    }
}

pub(crate) struct Skeleton;

impl Skeleton {
    pub(crate) fn from_spec(spec: SkeletonSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::skeleton(&spec, &RenderContext::new(theme)))
    }
}

pub(crate) struct Spinner;

impl Spinner {
    pub(crate) fn from_spec(spec: SpinnerSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::spinner(&spec, &RenderContext::new(theme)))
    }
}

pub(crate) struct Avatar;

impl Avatar {
    pub(crate) fn from_spec(spec: AvatarSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::avatar(&spec, &RenderContext::new(theme)))
    }
}

pub(crate) struct StatusIndicator;

impl StatusIndicator {
    pub(crate) fn from_spec(spec: StatusIndicatorSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::status_indicator(&spec, &RenderContext::new(theme)))
    }

    pub(crate) fn node_from_spec(
        spec: StatusIndicatorSpec,
        theme: &GpuiThemeProvider,
    ) -> poodle_node::Node {
        poodle_render::status_indicator(&spec, &RenderContext::new(theme))
    }
}

pub(crate) struct Meter;

impl Meter {
    pub(crate) fn from_spec(spec: MeterSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::meter(&spec, &RenderContext::new(theme)))
    }
}

pub(crate) struct Table;

impl Table {
    pub(crate) fn from_spec(spec: TableSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::table(&spec, &RenderContext::new(theme)))
    }
}

pub(crate) struct Rating {
    spec: RatingSpec,
    theme: GpuiThemeProvider,
    instance_id: String,
    on_change: Option<Arc<dyn Fn(Option<f64>) + Send + Sync>>,
}

pub(crate) struct CollapseToggle {
    spec: CollapseToggleSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

impl CollapseToggle {
    pub(crate) fn from_spec(spec: CollapseToggleSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_toggle = Some(handler);
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut node = poodle_render::collapse_toggle(&self.spec, &RenderContext::new(&self.theme), self.on_toggle);
        if let Some(id) = self.id_suffix {
            node.id = Some(format!("poodle-collapse-toggle-{id}"));
        }
        node
    }
}

impl IntoElement for CollapseToggle {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl Rating {
    pub(crate) fn from_spec(
        spec: RatingSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            instance_id: instance_id.into(),
            on_change: None,
        }
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(Option<f64>) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }
}

impl IntoElement for Rating {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::rating(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::RatingHandlers {
                instance_id: self.instance_id,
                on_change: self.on_change,
            },
        ))
    }
}

pub(crate) struct MetaBar {
    spec: MetaBarSpec,
    theme: GpuiThemeProvider,
    children: Vec<(poodle_node::Node, bool)>,
}

impl MetaBar {
    pub(crate) fn from_spec(spec: MetaBarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub(crate) fn with_child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push((child.into_compat_node(), true));
        self
    }

    pub(crate) fn with_child_sep(mut self, child: impl IntoCompatNode, separator: bool) -> Self {
        self.children.push((child.into_compat_node(), separator));
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::meta_bar_sep(&self.spec, &RenderContext::new(&self.theme), self.children)
    }
}

impl IntoCompatNode for MetaBar {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for MetaBar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct MetaItem {
    spec: MetaItemSpec,
    theme: GpuiThemeProvider,
    value: Option<poodle_node::Node>,
}

impl MetaItem {
    pub(crate) fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(MetaItemSpec::new(), theme)
    }

    pub(crate) fn from_spec(spec: MetaItemSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            value: None,
        }
    }

    pub(crate) fn with_value(mut self, value: impl IntoCompatNode) -> Self {
        self.value = Some(value.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::meta_item(&self.spec, &RenderContext::new(&self.theme), self.value)
    }
}

impl IntoCompatNode for MetaItem {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for MetaItem {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct NavCard {
    spec: NavCardSpec,
    theme: GpuiThemeProvider,
    icon: Option<poodle_node::Node>,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub(crate) struct Callout {
    spec: CallOutSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::CalloutHandlers,
}

impl Callout {
    pub(crate) fn from_spec(spec: CallOutSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::CalloutHandlers::default(),
        }
    }

    pub(crate) fn on_dismiss(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_dismiss = Some(handler);
        self
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.handlers.instance_id = Some(instance_id.into());
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::callout(&self.spec, &RenderContext::new(&self.theme), self.handlers)
    }
}

impl IntoCompatNode for Callout {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Callout {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct RemediationBanner {
    spec: RemediationBannerSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::RemediationBannerHandlers,
}

impl RemediationBanner {
    pub(crate) fn from_spec(spec: RemediationBannerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::RemediationBannerHandlers::default(),
        }
    }

    pub(crate) fn on_action(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_action = Some(handler);
        self
    }

    pub(crate) fn on_dismiss(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_dismiss = Some(handler);
        self
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.handlers.instance_id = Some(instance_id.into());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::remediation_banner(&self.spec, &RenderContext::new(&self.theme), self.handlers)
    }
}

impl IntoElement for RemediationBanner {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct StatusBar {
    spec: ShellStatusBarSpec,
    theme: GpuiThemeProvider,
    leading: Vec<poodle_node::Node>,
    trailing: Vec<poodle_node::Node>,
}

pub(crate) struct PageHeader {
    spec: PageHeaderSpec,
    theme: GpuiThemeProvider,
    breadcrumbs: Option<SlotBuilder<'static>>,
    actions: Option<SlotBuilder<'static>>,
    meta: Option<SlotBuilder<'static>>,
}

pub(crate) struct AppHeader {
    spec: AppHeaderSpec,
    theme: GpuiThemeProvider,
    identity: Option<SlotBuilder<'static>>,
    center: Option<SlotBuilder<'static>>,
    actions: Option<SlotBuilder<'static>>,
    utility: Option<SlotBuilder<'static>>,
}

impl AppHeader {
    pub(crate) fn from_spec(spec: AppHeaderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            identity: None,
            center: None,
            actions: None,
            utility: None,
        }
    }

    pub(crate) fn with_primary_actions(
        mut self,
        actions: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.actions = Some(std::boxed::Box::new(actions));
        self
    }

    pub(crate) fn with_utility_items(
        mut self,
        utility: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.utility = Some(std::boxed::Box::new(utility));
        self
    }

    pub(crate) fn with_center(
        mut self,
        center: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.center = Some(std::boxed::Box::new(center));
        self
    }

    pub(crate) fn with_identity(
        mut self,
        identity: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.identity = Some(std::boxed::Box::new(identity));
        self
    }

    pub(crate) fn with_leading(
        self,
        leading: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.with_identity(leading)
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::app_header(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.identity,
            self.center,
            self.actions,
            self.utility,
        )
    }
}

impl IntoCompatNode for AppHeader {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for AppHeader {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct FilterToolbar {
    spec: FilterToolbarSpec,
    theme: GpuiThemeProvider,
    children: Vec<SlotBuilder<'static>>,
    actions: Option<SlotBuilder<'static>>,
    secondary: Option<SlotBuilder<'static>>,
    on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

impl FilterToolbar {
    pub(crate) fn from_spec(spec: FilterToolbarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
            actions: None,
            secondary: None,
            on_toggle: None,
        }
    }

    pub(crate) fn with_child(
        mut self,
        child: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.children.push(std::boxed::Box::new(child));
        self
    }

    pub(crate) fn with_actions(
        mut self,
        actions: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.actions = Some(std::boxed::Box::new(actions));
        self
    }

    pub(crate) fn with_secondary(
        mut self,
        secondary: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.secondary = Some(std::boxed::Box::new(secondary));
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::filter_toolbar(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.children,
            self.actions,
            self.secondary,
            self.on_toggle,
        )
    }
}

impl IntoCompatNode for FilterToolbar {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for FilterToolbar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct FormShell {
    spec: FormShellSpec,
    theme: GpuiThemeProvider,
    section_slots: Vec<Option<poodle_node::Node>>,
    actions: Vec<poodle_node::Node>,
}

pub(crate) struct FormLayout {
    spec: FormLayoutSpec,
    theme: GpuiThemeProvider,
    children: Vec<poodle_node::Node>,
    actions: Option<poodle_node::Node>,
}

pub(crate) struct FieldSet {
    spec: FieldSetSpec,
    theme: GpuiThemeProvider,
    children: Vec<poodle_node::Node>,
}

pub(crate) struct ThemeSelect {
    spec: ThemeSelectSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

pub(crate) struct ModelPicker {
    spec: ModelPickerSpec,
    theme: GpuiThemeProvider,
    instance_id: String,
}

pub(crate) struct PickerShell {
    spec: PickerShellSpec,
    theme: GpuiThemeProvider,
    toolbar: Option<poodle_node::Node>,
    selection: Option<poodle_node::Node>,
    body: Option<poodle_node::Node>,
    state_content: Option<poodle_node::Node>,
    footer: Option<poodle_node::Node>,
}

pub(crate) struct FormDialog {
    spec: FormDialogSpec,
    theme: GpuiThemeProvider,
    children: Vec<poodle_node::Node>,
    actions: Option<poodle_node::Node>,
}

pub(crate) struct ScrollShell {
    spec: ScrollShellSpec,
    theme: GpuiThemeProvider,
    children: Vec<poodle_node::Node>,
}

pub(crate) struct Region {
    spec: RegionSpec,
    theme: GpuiThemeProvider,
}

pub(crate) struct EmbedPreview {
    spec: EmbedPreviewSpec,
    theme: GpuiThemeProvider,
}

pub(crate) struct MediaThumbnail {
    spec: MediaThumbnailSpec,
    theme: GpuiThemeProvider,
}

pub(crate) struct MediaPreview {
    spec: MediaPreviewSpec,
    theme: GpuiThemeProvider,
    media_content: Option<SlotBuilder<'static>>,
}

pub(crate) struct CardRadioGroup {
    spec: CardRadioGroupSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub(crate) struct PageLoading {
    spec: PageLoadingSpec,
    theme: GpuiThemeProvider,
}

pub(crate) struct MediaPicker {
    spec: MediaPickerSpec,
    theme: GpuiThemeProvider,
}

pub(crate) struct DataTable {
    spec: DataTableSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::DataTableHandlers,
}

pub(crate) struct AgentQuestion {
    spec: AgentQuestionSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::AgentQuestionHandlers,
}

pub(crate) struct AgentMessage {
    spec: AgentMessageSpec,
    theme: GpuiThemeProvider,
}

pub(crate) struct AgentPlan {
    spec: AgentPlanSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::AgentPlanHandlers,
}

pub(crate) struct AgentPlanRecord {
    spec: AgentPlanRecordSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::AgentPlanRecordHandlers,
}

pub(crate) struct AgentQuestionRecord {
    spec: AgentQuestionRecordSpec,
    theme: GpuiThemeProvider,
}

pub(crate) struct AgentSubagent {
    spec: AgentSubagentSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::AgentSubagentHandlers,
}

pub(crate) struct ChangedFiles {
    spec: ChangedFilesSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::ChangedFilesHandlers,
}

pub(crate) struct ToolCall {
    spec: ToolCallSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::ToolCallHandlers,
}

pub(crate) struct ToolCallGroup {
    spec: ToolCallGroupSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::ToolCallGroupHandlers,
}

pub(crate) struct AgentTranscript {
    spec: AgentTranscriptSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::AgentTranscriptHandlers,
}

pub(crate) struct MediaBrowsePanel {
    spec: MediaBrowsePanelSpec,
    theme: GpuiThemeProvider,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub(crate) struct SidebarNav {
    spec: SidebarNavSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub(crate) struct Drawer {
    spec: DrawerSpec,
    theme: GpuiThemeProvider,
    content: Option<poodle_node::Node>,
    actions: Option<poodle_node::Node>,
    on_request_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub(crate) struct Dialog {
    spec: DialogSpec,
    theme: GpuiThemeProvider,
    children: Vec<poodle_node::Node>,
    actions: Option<poodle_node::Node>,
    header: Option<poodle_node::Node>,
    footer: Option<poodle_node::Node>,
    on_request_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub(crate) struct ToastStack {
    spec: ToastStackSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::ToastStackHandlers,
}

pub(crate) struct ToastHost {
    spec: ToastHostSpec,
    stack_spec: ToastStackSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::ToastStackHandlers,
}

pub(crate) struct DebugDialog {
    spec: DebugDialogSpec,
    theme: GpuiThemeProvider,
}

pub(crate) struct ActionDiscoveryPanel {
    spec: ActionDiscoveryPanelSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::ActionDiscoveryPanelHandlers,
}

pub(crate) struct BulkActionBar {
    spec: BulkActionBarSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::BulkActionBarHandlers,
}

pub(crate) struct AgentChatInput {
    spec: AgentChatInputSpec,
    theme: GpuiThemeProvider,
    question_children: Vec<poodle_node::Node>,
    plan_children: Vec<poodle_node::Node>,
    toolbar_children: Vec<poodle_node::Node>,
    footer_children: Vec<poodle_node::Node>,
    handlers: poodle_render::AgentChatInputHandlers,
}

pub(crate) struct FilterBuilder {
    spec: FilterBuilderSpec,
    theme: GpuiThemeProvider,
    instance_id: String,
    handlers: poodle_render::FilterBuilderHandlers,
}

pub(crate) struct MarkdownEditor {
    spec: MarkdownEditorSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::MarkdownEditorHandlers,
}

pub(crate) struct EditableList {
    spec: EditableListSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::EditableListHandlers,
}

pub(crate) struct RelationPicker {
    spec: RelationPickerSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::RelationPickerHandlers,
}

pub(crate) struct EditableLabel {
    spec: EditableLabelSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::EditableLabelHandlers,
}

impl EmbedPreview {
    pub(crate) fn from_spec(spec: EmbedPreviewSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::embed_preview(&self.spec, &RenderContext::new(&self.theme))
    }
}

impl IntoCompatNode for EmbedPreview {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for EmbedPreview {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl MediaThumbnail {
    pub(crate) fn from_spec(spec: MediaThumbnailSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::media_thumbnail(&self.spec, &RenderContext::new(&self.theme))
    }
}

impl IntoCompatNode for MediaThumbnail {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for MediaThumbnail {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl MediaPreview {
    pub(crate) fn from_spec(spec: MediaPreviewSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            media_content: None,
        }
    }

    pub(crate) fn with_media_content(
        mut self,
        content: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.media_content = Some(std::boxed::Box::new(content));
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::media_preview_with_content(&self.spec, &RenderContext::new(&self.theme), self.media_content)
    }
}

impl IntoCompatNode for MediaPreview {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for MediaPreview {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl CardRadioGroup {
    pub(crate) fn from_spec(spec: CardRadioGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
        }
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }

    pub(crate) fn with_size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::card_radio_group(&self.spec, &RenderContext::new(&self.theme), self.on_change)
    }
}

impl IntoElement for CardRadioGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl PageLoading {
    pub(crate) fn from_spec(spec: PageLoadingSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::page_loading(&self.spec, &RenderContext::new(&self.theme), None)
    }
}

impl IntoElement for PageLoading {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl MediaPicker {
    pub(crate) fn from_spec(spec: MediaPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    pub(crate) fn with_items(mut self, items: Vec<poodle_specs::MediaPickerItem>) -> Self {
        self.spec.items = items;
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::media_picker(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::MediaPickerHandlers::default(),
        )
    }
}

impl DataTable {
    pub(crate) fn from_spec(spec: DataTableSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::DataTableHandlers::default(),
        }
    }

    pub(crate) fn on_sort(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_sort = Some(handler);
        self
    }

    pub(crate) fn on_row_click(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_row_click = Some(handler);
        self
    }
}

impl AgentQuestion {
    pub(crate) fn from_spec(spec: AgentQuestionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::AgentQuestionHandlers::default(),
        }
    }

    pub(crate) fn on_select(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_select = Some(handler);
        self
    }
}

impl IntoElement for AgentQuestion {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::agent_question(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl AgentMessage {
    pub(crate) fn from_spec(spec: AgentMessageSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for AgentMessage {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::agent_message(&self.spec, &RenderContext::new(&self.theme)))
    }
}

impl AgentPlan {
    pub(crate) fn from_spec(spec: AgentPlanSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::AgentPlanHandlers::default(),
        }
    }

    pub(crate) fn on_accept(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_accept = Some(handler);
        self
    }

    pub(crate) fn on_revise(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_revise = Some(handler);
        self
    }

    pub(crate) fn on_dismiss(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_dismiss = Some(handler);
        self
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.handlers.instance_id = Some(instance_id.into());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::agent_plan(&self.spec, &RenderContext::new(&self.theme), self.handlers)
    }
}

impl IntoCompatNode for AgentPlan {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for AgentPlan {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl AgentPlanRecord {
    pub(crate) fn from_spec(spec: AgentPlanRecordSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::AgentPlanRecordHandlers::default(),
        }
    }

    pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.handlers.on_toggle = Some(handler);
        self
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.handlers.instance_id = Some(instance_id.into());
        self
    }
}

impl IntoElement for AgentPlanRecord {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::agent_plan_record(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl AgentQuestionRecord {
    pub(crate) fn from_spec(spec: AgentQuestionRecordSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for AgentQuestionRecord {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::agent_question_record(
            &self.spec,
            &RenderContext::new(&self.theme),
        ))
    }
}

impl AgentSubagent {
    pub(crate) fn from_spec(spec: AgentSubagentSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::AgentSubagentHandlers::default(),
        }
    }

    pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.handlers.on_toggle = Some(handler);
        self
    }

    pub(crate) fn on_open_child(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_open_child = Some(handler);
        self
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.handlers.instance_id = Some(instance_id.into());
        self
    }
}

impl IntoElement for AgentSubagent {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::agent_subagent(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl ChangedFiles {
    pub(crate) fn from_spec(spec: ChangedFilesSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::ChangedFilesHandlers::default(),
        }
    }

    pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_toggle = Some(handler);
        self
    }

    pub(crate) fn on_file_select(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_file_select = Some(handler);
        self
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.handlers.instance_id = Some(instance_id.into());
        self
    }
}

impl IntoElement for ChangedFiles {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::changed_files(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl ToolCall {
    pub(crate) fn from_spec(spec: ToolCallSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::ToolCallHandlers::default(),
        }
    }

    pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_toggle = Some(handler);
        self
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.handlers.instance_id = Some(instance_id.into());
        self
    }
}

impl IntoElement for ToolCall {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::tool_call(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl ToolCallGroup {
    pub(crate) fn from_spec(spec: ToolCallGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::ToolCallGroupHandlers::default(),
        }
    }

    pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_toggle = Some(handler);
        self
    }

    pub(crate) fn on_call_toggle(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_call_toggle = Some(handler);
        self
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.handlers.instance_id = Some(instance_id.into());
        self
    }
}

impl IntoElement for ToolCallGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::tool_call_group(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl AgentTranscript {
    pub(crate) fn from_spec(spec: AgentTranscriptSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::AgentTranscriptHandlers::default(),
        }
    }

    pub(crate) fn on_tool_run_toggle(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_tool_run_toggle = Some(handler);
        self
    }

    pub(crate) fn on_tool_call_toggle(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_tool_call_toggle = Some(handler);
        self
    }

    pub(crate) fn on_changed_files_toggle(
        mut self,
        handler: Arc<dyn Fn(&str) + Send + Sync>,
    ) -> Self {
        self.handlers.on_changed_files_toggle = Some(handler);
        self
    }

    pub(crate) fn on_file_select(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_file_select = Some(handler);
        self
    }
}

impl IntoElement for AgentTranscript {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::agent_transcript(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl MediaBrowsePanel {
    pub(crate) fn from_spec(spec: MediaBrowsePanelSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_select: None,
        }
    }
}

impl IntoElement for MediaBrowsePanel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::media_browse_panel(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.on_select,
        ))
    }
}

impl SidebarNav {
    pub(crate) fn from_spec(spec: SidebarNavSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
        }
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }
}

impl IntoElement for SidebarNav {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::sidebar_nav(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.on_change,
        ))
    }
}

impl ToastStack {
    pub(crate) fn from_spec(spec: ToastStackSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::ToastStackHandlers::default(),
        }
    }

    pub(crate) fn with_size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for ToastStack {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::toast_stack(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl ToastHost {
    pub(crate) fn from_spec(spec: ToastHostSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            stack_spec: ToastStackSpec::new(),
            theme: theme.clone(),
            handlers: poodle_render::ToastStackHandlers::default(),
        }
    }

    pub(crate) fn toasts(mut self, toasts: Vec<poodle_specs::Toast>) -> Self {
        self.stack_spec.toasts = toasts;
        self
    }
}

impl IntoElement for ToastHost {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::toast_host(
            &self.spec,
            &RenderContext::new(&self.theme),
            &self.stack_spec,
            self.handlers,
        ))
    }
}

impl DebugDialog {
    pub(crate) fn from_spec(spec: DebugDialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for DebugDialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::debug_dialog(&self.spec, &RenderContext::new(&self.theme)))
    }
}

impl ActionDiscoveryPanel {
    pub(crate) fn from_spec(spec: ActionDiscoveryPanelSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::ActionDiscoveryPanelHandlers::default(),
        }
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.handlers.instance_id = Some(instance_id.into());
        self
    }

    pub(crate) fn on_select(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_select = Some(handler);
        self
    }
}

impl IntoElement for ActionDiscoveryPanel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::action_discovery_panel(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl BulkActionBar {
    pub(crate) fn from_spec(spec: BulkActionBarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::BulkActionBarHandlers::default(),
        }
    }
}

impl IntoElement for BulkActionBar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::bulk_action_bar(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl AgentChatInput {
    pub(crate) fn from_spec(spec: AgentChatInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            question_children: Vec::new(),
            plan_children: Vec::new(),
            toolbar_children: Vec::new(),
            footer_children: Vec::new(),
            handlers: poodle_render::AgentChatInputHandlers::default(),
        }
    }

    pub(crate) fn question_child(mut self, child: impl IntoCompatNode) -> Self {
        self.question_children.push(child.into_compat_node());
        self
    }

    pub(crate) fn plan_child(mut self, child: impl IntoCompatNode) -> Self {
        self.plan_children.push(child.into_compat_node());
        self
    }

    pub(crate) fn toolbar_child(mut self, child: impl IntoCompatNode) -> Self {
        self.toolbar_children.push(child.into_compat_node());
        self
    }

    pub(crate) fn footer_child(mut self, child: impl IntoCompatNode) -> Self {
        self.footer_children.push(child.into_compat_node());
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for AgentChatInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::agent_chat_input(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.question_children,
            self.plan_children,
            self.toolbar_children,
            self.footer_children,
            self.handlers,
        ))
    }
}

impl FilterBuilder {
    pub(crate) fn from_spec(
        spec: FilterBuilderSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            instance_id: instance_id.into(),
            handlers: poodle_render::FilterBuilderHandlers::default(),
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for FilterBuilder {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::filter_builder(
            &self.spec,
            &RenderContext::new(&self.theme),
            &self.instance_id,
            &self.handlers,
        ))
    }
}

impl MarkdownEditor {
    pub(crate) fn from_spec(spec: MarkdownEditorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::MarkdownEditorHandlers::default(),
        }
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_change = Some(handler);
        self
    }

    pub(crate) fn on_mode_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_mode_change = Some(handler);
        self
    }
}

impl IntoElement for MarkdownEditor {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::markdown_editor_with_handlers(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl EditableList {
    pub(crate) fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(EditableListSpec::new(), theme)
    }

    pub(crate) fn from_spec(spec: EditableListSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::EditableListHandlers::default(),
        }
    }

    pub(crate) fn aria_label(mut self, label: impl Into<String>) -> Self {
        self.spec.aria_label = label.into();
        self
    }

    pub(crate) fn add_label(mut self, label: impl Into<String>) -> Self {
        self.spec.add_label = label.into();
        self
    }

    pub(crate) fn placeholder(mut self, value: impl Into<String>) -> Self {
        self.spec.placeholder = value.into();
        self
    }

    pub(crate) fn items(mut self, values: Vec<String>) -> Self {
        let items = values
            .into_iter()
            .enumerate()
            .map(|(index, value)| {
                poodle_specs::EditableListItem::new(format!("item-{index}")).with_label(value)
            })
            .collect();
        self.spec = self.spec.with_items(items);
        self
    }

    pub(crate) fn editable(mut self, value: bool) -> Self {
        self.spec.is_editable = value;
        self
    }

    pub(crate) fn removable(mut self, value: bool) -> Self {
        self.spec.is_removable = value;
        self
    }

    pub(crate) fn reorderable(mut self, value: bool) -> Self {
        self.spec.is_reorderable = value;
        self
    }

    pub(crate) fn max_items(mut self, value: usize) -> Self {
        self.spec.max_items = Some(value);
        self
    }

    pub(crate) fn dirty(mut self, value: bool) -> Self {
        self.spec.is_dirty = value;
        self
    }

    pub(crate) fn submitting(mut self, value: bool) -> Self {
        self.spec.is_submitting = value;
        self
    }

    pub(crate) fn error_message(mut self, value: impl Into<String>) -> Self {
        self.spec.error_message = Some(value.into());
        self
    }

    pub(crate) fn info_message(mut self, value: impl Into<String>) -> Self {
        self.spec.info_message = Some(value.into());
        self
    }

    pub(crate) fn disabled(mut self, value: bool) -> Self {
        self.spec.is_disabled = value;
        self
    }

    pub(crate) fn with_size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for EditableList {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::editable_list(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl RelationPicker {
    pub(crate) fn from_spec(
        spec: RelationPickerSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::RelationPickerHandlers::new(instance_id),
        }
    }

    pub(crate) fn on_drill_enter(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_drill_enter = Some(handler);
        self
    }

    pub(crate) fn on_breadcrumb_click(mut self, handler: Arc<dyn Fn(usize) + Send + Sync>) -> Self {
        self.handlers.on_breadcrumb_click = Some(handler);
        self
    }
}

impl IntoElement for RelationPicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::relation_picker(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl EditableLabel {
    pub(crate) fn from_spec(spec: EditableLabelSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::EditableLabelHandlers::default(),
        }
    }

    pub(crate) fn with_id(self, _id: impl Into<String>) -> Self {
        self
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_change = Some(handler);
        self
    }

    pub(crate) fn on_commit(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_commit = Some(handler);
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for EditableLabel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::editable_label_with_handlers(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl Dialog {
    pub(crate) fn from_spec(spec: DialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
            actions: None,
            header: None,
            footer: None,
            on_request_close: None,
        }
    }

    pub(crate) fn on_open_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_request_close = Some(Arc::new(move || handler(false)));
        self
    }

    pub(crate) fn with_content(mut self, content: impl IntoCompatNode) -> Self {
        self.children.push(content.into_compat_node());
        self
    }

    pub(crate) fn with_actions(mut self, actions: impl IntoCompatNode) -> Self {
        self.actions = Some(actions.into_compat_node());
        self
    }

    pub(crate) fn with_header(mut self, header: impl IntoCompatNode) -> Self {
        self.header = Some(header.into_compat_node());
        self
    }

    pub(crate) fn with_footer(mut self, footer: impl IntoCompatNode) -> Self {
        self.footer = Some(footer.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::dialog_with_slots(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.children,
            self.actions,
            self.header,
            self.footer,
            self.on_request_close,
        )
    }
}

impl IntoCompatNode for Dialog {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Dialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        native_dialog_element(self.into_node())
    }
}

impl Drawer {
    pub(crate) fn from_spec(spec: DrawerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            actions: None,
            on_request_close: None,
        }
    }

    pub(crate) fn with_content(mut self, content: impl IntoCompatNode) -> Self {
        self.content = Some(content.into_compat_node());
        self
    }

    pub(crate) fn with_actions(mut self, actions: impl IntoCompatNode) -> Self {
        self.actions = Some(actions.into_compat_node());
        self
    }

    pub(crate) fn on_open_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_request_close = Some(Arc::new(move || handler(false)));
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::drawer(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.content,
            self.actions,
            self.on_request_close,
        )
    }
}

impl IntoCompatNode for Drawer {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Drawer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl IntoElement for DataTable {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::data_table(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

impl IntoElement for MediaPicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl Region {
    pub(crate) fn from_spec(spec: RegionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::region(&self.spec, &RenderContext::new(&self.theme))
    }
}

impl IntoCompatNode for Region {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Region {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl ScrollShell {
    pub(crate) fn from_spec(spec: ScrollShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub(crate) fn with_child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::scroll_shell(&self.spec, &RenderContext::new(&self.theme), self.children)
    }
}

impl IntoCompatNode for ScrollShell {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for ScrollShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl FormDialog {
    pub(crate) fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(FormDialogSpec::default(), theme)
    }

    pub(crate) fn from_spec(spec: FormDialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
            actions: None,
        }
    }

    pub(crate) fn title(mut self, value: impl Into<String>) -> Self {
        self.spec.title = value.into();
        self
    }

    pub(crate) fn description(mut self, value: impl Into<String>) -> Self {
        self.spec.description = Some(value.into());
        self
    }

    pub(crate) fn subtitle(mut self, value: impl Into<String>) -> Self {
        self.spec.subtitle = Some(value.into());
        self
    }

    pub(crate) fn submit_label(mut self, value: impl Into<String>) -> Self {
        self.spec.submit_label = value.into();
        self
    }

    pub(crate) fn cancel_label(mut self, value: impl Into<String>) -> Self {
        self.spec.cancel_label = value.into();
        self
    }

    pub(crate) fn submitting(mut self, value: bool) -> Self {
        self.spec.is_submitting = value;
        self
    }

    pub(crate) fn error_message(mut self, value: impl Into<String>) -> Self {
        self.spec.error = Some(value.into());
        self
    }

    pub(crate) fn bare(mut self, value: bool) -> Self {
        self.spec.is_bare = value;
        self
    }

    pub(crate) fn show_default_actions(mut self, value: bool) -> Self {
        self.spec.show_default_actions = value;
        self
    }

    pub(crate) fn with_child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }

    pub(crate) fn with_actions(mut self, actions: impl IntoCompatNode) -> Self {
        self.actions = Some(actions.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::form_dialog(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.children,
            self.actions,
            poodle_render::FormDialogHandlers::default(),
        )
    }
}

impl IntoCompatNode for FormDialog {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for FormDialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl PickerShell {
    pub(crate) fn from_spec(spec: PickerShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            toolbar: None,
            selection: None,
            body: None,
            state_content: None,
            footer: None,
        }
    }

    pub(crate) fn with_toolbar(mut self, toolbar: impl IntoCompatNode) -> Self {
        self.toolbar = Some(toolbar.into_compat_node());
        self
    }

    pub(crate) fn with_search(self, search: impl IntoCompatNode) -> Self {
        self.with_toolbar(search)
    }

    pub(crate) fn with_body(mut self, body: impl IntoCompatNode) -> Self {
        self.body = Some(body.into_compat_node());
        self
    }

    pub(crate) fn with_results(self, results: impl IntoCompatNode) -> Self {
        self.with_body(results)
    }

    pub(crate) fn with_footer(mut self, footer: impl IntoCompatNode) -> Self {
        self.footer = Some(footer.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::picker_shell(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.toolbar,
            self.selection,
            self.body,
            self.state_content,
            self.footer,
        )
    }
}

impl IntoCompatNode for PickerShell {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for PickerShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl ModelPicker {
    pub(crate) fn from_spec(
        spec: ModelPickerSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            instance_id: instance_id.into(),
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::model_picker(&self.spec, &RenderContext::new(&self.theme), &self.instance_id, None)
    }
}

impl IntoCompatNode for ModelPicker {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for ModelPicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl ThemeSelect {
    pub(crate) fn from_spec(spec: ThemeSelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
            on_open_change: None,
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }

    /// Fires with the open state the trigger is moving to; `is_open` is
    /// controlled, so the host flips the spec.
    pub(crate) fn on_open_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_open_change = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::theme_select_with_handlers(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::ThemeSelectHandlers {
                on_change: self.on_change,
                on_open_change: self.on_open_change,
            },
        )
    }
}

impl IntoCompatNode for ThemeSelect {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for ThemeSelect {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl FieldSet {
    pub(crate) fn from_spec(spec: FieldSetSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub(crate) fn with_child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::field_set(&self.spec, &RenderContext::new(&self.theme), self.children)
    }
}

impl IntoCompatNode for FieldSet {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for FieldSet {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl FormLayout {
    pub(crate) fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(FormLayoutSpec::new(), theme)
    }

    pub(crate) fn from_spec(spec: FormLayoutSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
            actions: None,
        }
    }

    pub(crate) fn description(mut self, value: impl Into<String>) -> Self {
        self.spec.description = Some(value.into());
        self
    }

    pub(crate) fn error(mut self, value: impl Into<String>) -> Self {
        self.spec.error = Some(value.into());
        self
    }

    pub(crate) fn success(mut self, value: impl Into<String>) -> Self {
        self.spec.success = Some(value.into());
        self
    }

    pub(crate) fn columns(mut self, value: usize) -> Self {
        self.spec.columns = value as u32;
        self
    }

    pub(crate) fn with_field_error(
        mut self,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        self.spec.field_errors.push((field.into(), message.into()));
        self
    }

    pub(crate) fn with_child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }

    pub(crate) fn with_actions(mut self, actions: impl IntoCompatNode) -> Self {
        self.actions = Some(actions.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::form_layout(&self.spec, &RenderContext::new(&self.theme), self.children, self.actions)
    }
}

impl IntoCompatNode for FormLayout {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for FormLayout {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl FormShell {
    pub(crate) fn from_spec(spec: FormShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            section_slots: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub(crate) fn with_section_slot(mut self, slot: impl IntoCompatNode) -> Self {
        self.section_slots.push(Some(slot.into_compat_node()));
        self
    }

    pub(crate) fn with_action(mut self, action: impl IntoCompatNode) -> Self {
        self.actions.push(action.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let actions = if self.actions.is_empty() {
            None
        } else {
            let mut row = poodle_node::Node::container();
            row.style.descriptor.layout.direction = poodle_node::LayoutDirection::Row;
            row.style.descriptor.layout.alignment.cross = poodle_node::CrossAxisAlignment::Center;
            row.style.descriptor.layout.spacing.gap = self.theme.resolve_space("space.inline.sm");
            row.style.descriptor.layout.spacing.padding.top =
                self.theme.resolve_space("space.stack.sm");
            Some(self.actions.into_iter().fold(row, poodle_node::Node::child))
        };
        poodle_render::form_shell(&self.spec, &RenderContext::new(&self.theme), self.section_slots, actions)
    }
}

impl IntoCompatNode for FormShell {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for FormShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl PageHeader {
    pub(crate) fn from_spec(spec: PageHeaderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            breadcrumbs: None,
            actions: None,
            meta: None,
        }
    }

    pub(crate) fn with_breadcrumbs(
        mut self,
        breadcrumbs: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.breadcrumbs = Some(std::boxed::Box::new(breadcrumbs));
        self
    }

    pub(crate) fn with_actions(
        mut self,
        actions: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.actions = Some(std::boxed::Box::new(actions));
        self
    }

    pub(crate) fn with_meta(
        mut self,
        meta: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.meta = Some(std::boxed::Box::new(meta));
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::page_header(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.breadcrumbs,
            self.actions,
            self.meta,
        )
    }
}

impl IntoCompatNode for PageHeader {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for PageHeader {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct Toolbar {
    spec: ToolbarSpec,
    theme: GpuiThemeProvider,
    children: Vec<poodle_node::Node>,
}

impl Toolbar {
    pub(crate) fn from_spec(spec: ToolbarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub(crate) fn child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::toolbar(&self.spec, &RenderContext::new(&self.theme), self.children)
    }
}

impl IntoCompatNode for Toolbar {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Toolbar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct OrderBy {
    spec: OrderBySpec,
    theme: GpuiThemeProvider,
    instance_id: String,
    on_direction_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl OrderBy {
    pub(crate) fn from_spec(
        spec: OrderBySpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            instance_id: instance_id.into(),
            on_direction_toggle: None,
            on_remove: None,
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::order_by(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::OrderByHandlers {
                instance_id: self.instance_id,
                on_direction_toggle: self.on_direction_toggle,
                on_remove: self.on_remove,
            },
        )
    }
}

impl IntoCompatNode for OrderBy {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for OrderBy {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct RefSelect {
    spec: RefSelectSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub(crate) struct FormActions {
    spec: FormActionsSpec,
    theme: GpuiThemeProvider,
    danger: Vec<poodle_node::Node>,
    actions: Vec<poodle_node::Node>,
}

impl FormActions {
    pub(crate) fn from_spec(spec: FormActionsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            danger: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub(crate) fn with_action(mut self, action: impl IntoCompatNode) -> Self {
        self.actions.push(action.into_compat_node());
        self
    }

    pub(crate) fn with_danger_action(mut self, action: impl IntoCompatNode) -> Self {
        self.danger.push(action.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::form_actions_full(&self.spec, &RenderContext::new(&self.theme), self.danger, self.actions)
    }
}

impl IntoCompatNode for FormActions {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for FormActions {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl RefSelect {
    pub(crate) fn from_spec(spec: RefSelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::ref_select(&self.spec, &RenderContext::new(&self.theme), self.on_change)
    }
}

impl IntoCompatNode for RefSelect {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for RefSelect {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl StatusBar {
    pub(crate) fn from_spec(spec: ShellStatusBarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            leading: Vec::new(),
            trailing: Vec::new(),
        }
    }

    pub(crate) fn chrome(mut self, value: bool) -> Self {
        self.spec.chrome = value;
        self
    }

    pub(crate) fn with_size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn with_leading_items(mut self, items: impl IntoCompatNode) -> Self {
        self.leading = vec![items.into_compat_node()];
        self
    }

    pub(crate) fn with_trailing_items(mut self, items: impl IntoCompatNode) -> Self {
        self.trailing = vec![items.into_compat_node()];
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::shell_status_bar(&self.spec, &RenderContext::new(&self.theme), self.leading, self.trailing)
    }
}

impl IntoCompatNode for StatusBar {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for StatusBar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl NavCard {
    pub(crate) fn from_spec(spec: NavCardSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            icon: None,
            on_click: None,
        }
    }

    pub(crate) fn with_icon(mut self, icon: impl IntoCompatNode) -> Self {
        self.icon = Some(icon.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::nav_card_with_icon(&self.spec, &RenderContext::new(&self.theme), self.on_click, self.icon)
    }
}

impl IntoCompatNode for NavCard {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for NavCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct PaginationSummary {
    spec: PaginationSummarySpec,
    theme: GpuiThemeProvider,
}

impl PaginationSummary {
    pub(crate) fn from_spec(spec: PaginationSummarySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PaginationSummary {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::pagination_summary(
            &self.spec,
            &RenderContext::new(&self.theme),
        ))
    }
}

pub(crate) struct ValidationSummary {
    spec: ValidationSummarySpec,
    theme: GpuiThemeProvider,
}

impl ValidationSummary {
    pub(crate) fn from_spec(spec: ValidationSummarySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for ValidationSummary {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::validation_summary(
            &self.spec,
            &RenderContext::new(&self.theme),
        ))
    }
}

pub(crate) struct Progress {
    spec: ProgressSpec,
    theme: GpuiThemeProvider,
}

impl Progress {
    pub(crate) fn from_spec(spec: ProgressSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }
}

impl IntoElement for Progress {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::progress(&self.spec, &RenderContext::new(&self.theme)))
    }
}

pub(crate) struct EmptyState {
    spec: EmptyStateSpec,
    theme: GpuiThemeProvider,
}

pub(crate) struct ErrorBoundary {
    spec: ErrorBoundarySpec,
    theme: GpuiThemeProvider,
    child: Option<poodle_node::Node>,
}

impl ErrorBoundary {
    pub(crate) fn from_spec(spec: ErrorBoundarySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            child: None,
        }
    }

    pub(crate) fn with_child(mut self, child: impl IntoCompatNode) -> Self {
        self.child = Some(child.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::error_boundary(&self.spec, &RenderContext::new(&self.theme), self.child)
    }
}

impl IntoCompatNode for ErrorBoundary {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for ErrorBoundary {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct PasswordRequirements {
    spec: PasswordRequirementsSpec,
    theme: GpuiThemeProvider,
}

impl PasswordRequirements {
    pub(crate) fn from_spec(spec: PasswordRequirementsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PasswordRequirements {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::password_requirements(
            &self.spec,
            &RenderContext::new(&self.theme),
        ))
    }
}

impl EmptyState {
    pub(crate) fn from_spec(spec: EmptyStateSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for EmptyState {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::empty_state(&self.spec, &RenderContext::new(&self.theme)))
    }
}

pub(crate) struct ResizeHandle {
    spec: ResizeHandleSpec,
    theme: GpuiThemeProvider,
    on_resize: Option<Arc<dyn Fn(poodle_render::ResizePhase, f32) + Send + Sync>>,
}

impl ResizeHandle {
    pub(crate) fn from_spec(spec: ResizeHandleSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_resize: None,
        }
    }

    pub(crate) fn on_resize(
        mut self,
        handler: Arc<dyn Fn(poodle_render::ResizePhase, f32) + Send + Sync>,
    ) -> Self {
        self.on_resize = Some(handler);
        self
    }
}

impl IntoElement for ResizeHandle {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::resize_handle(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.on_resize,
        ))
    }
}

pub(crate) trait IntoCompatNode {
    fn into_compat_node(self) -> poodle_node::Node;
}

impl IntoCompatNode for poodle_node::Node {
    fn into_compat_node(self) -> poodle_node::Node {
        self
    }
}

impl IntoCompatNode for String {
    fn into_compat_node(self) -> poodle_node::Node {
        poodle_node::Node::text(self)
    }
}

impl IntoCompatNode for &str {
    fn into_compat_node(self) -> poodle_node::Node {
        poodle_node::Node::text(self)
    }
}

pub(crate) struct CompatRow {
    children: Vec<poodle_node::Node>,
    gap: f32,
    justify_end: bool,
}

impl CompatRow {
    pub(crate) fn new() -> Self {
        Self {
            children: Vec::new(),
            gap: 0.0,
            justify_end: false,
        }
    }

    pub(crate) fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub(crate) fn justify_end(mut self) -> Self {
        self.justify_end = true;
        self
    }

    pub(crate) fn child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }
}

impl IntoCompatNode for CompatRow {
    fn into_compat_node(self) -> poodle_node::Node {
        let mut row = poodle_node::Node::container();
        row.style.descriptor.layout.direction = poodle_node::LayoutDirection::Row;
        row.style.descriptor.layout.alignment.cross = poodle_node::CrossAxisAlignment::Center;
        if self.justify_end {
            row.style.descriptor.layout.alignment.main = poodle_node::MainAxisAlignment::End;
        }
        row.style.descriptor.layout.spacing.gap = self.gap;
        row.children = self.children;
        row
    }
}

pub(crate) struct Box {
    spec: BoxSpec,
    theme: GpuiThemeProvider,
    children: Vec<poodle_node::Node>,
}

impl Box {
    pub(crate) fn from_spec(spec: BoxSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub(crate) fn with_child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::bx(&self.spec, &RenderContext::new(&self.theme), self.children)
    }
}

impl IntoCompatNode for Box {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Box {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct Grid {
    spec: GridSpec,
    theme: GpuiThemeProvider,
    children: Vec<poodle_node::Node>,
}

impl Grid {
    pub(crate) fn from_spec(spec: GridSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub(crate) fn with_child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::grid(&self.spec, &RenderContext::new(&self.theme), self.children)
    }
}

impl IntoCompatNode for Grid {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Grid {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct Stack {
    spec: StackSpec,
    theme: GpuiThemeProvider,
    children: Vec<poodle_node::Node>,
}

impl Stack {
    pub(crate) fn from_spec(spec: StackSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub(crate) fn with_child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::stack(&self.spec, &RenderContext::new(&self.theme), self.children)
    }
}

impl IntoCompatNode for Stack {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Stack {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct Spacer {
    theme: GpuiThemeProvider,
}

impl Spacer {
    pub(crate) fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
        }
    }
}

impl IntoCompatNode for Spacer {
    fn into_compat_node(self) -> poodle_node::Node {
        poodle_render::spacer(&SpacerSpec::new(), &RenderContext::new(&self.theme))
    }
}

impl IntoElement for Spacer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_compat_node())
    }
}

pub(crate) struct Separator {
    spec: SeparatorSpec,
    theme: GpuiThemeProvider,
}

impl Separator {
    pub(crate) fn from_spec(spec: SeparatorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoCompatNode for Separator {
    fn into_compat_node(self) -> poodle_node::Node {
        poodle_render::separator(&self.spec, &RenderContext::new(&self.theme))
    }
}

impl IntoElement for Separator {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_compat_node())
    }
}

pub(crate) struct Pill {
    spec: PillSpec,
    theme: GpuiThemeProvider,
    on_remove: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl Pill {
    pub(crate) fn from_spec(spec: PillSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_remove: None,
        }
    }

    pub(crate) fn on_remove(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_remove = Some(Arc::new(handler));
        self
    }

    pub(crate) fn into_node_with(self, ctx: &RenderContext<'_>) -> poodle_node::Node {
        let mut node = poodle_render::pill_with_remove(&self.spec, ctx, self.on_remove);
        // The old GPUI Pill made its root focusable even though the shared
        // contract treats Pill as display metadata. Keep that preview-local.
        node.interaction.focusable = true;
        node
    }

    fn into_node(self) -> poodle_node::Node {
        let theme = self.theme.clone();
        self.into_node_with(&RenderContext::new(&theme))
    }
}

impl IntoCompatNode for Pill {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Pill {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

/// Icon keeps the one preview-only override used by composite specimens.
pub(crate) struct Icon {
    spec: IconSpec,
    theme: GpuiThemeProvider,
    color: Option<Hsla>,
}

impl Icon {
    pub(crate) fn from_spec(spec: IconSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            color: None,
        }
    }

    pub(crate) fn with_color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut node = poodle_render::icon(&self.spec, &RenderContext::new(&self.theme));
        if let Some(color) = self.color {
            let rgba: Rgba = color.into();
            node.style.descriptor.text_color = Some(ColorValue(rgba.r, rgba.g, rgba.b, rgba.a));
        }
        node
    }
}

impl IntoCompatNode for Icon {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Icon {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct ListCardCounter {
    spec: ListCardCounterSpec,
    theme: GpuiThemeProvider,
    on_link_click: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl ListCardCounter {
    pub(crate) fn from_spec(spec: ListCardCounterSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_link_click: None,
        }
    }

    pub(crate) fn on_link_click(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_link_click = Some(Arc::new(handler));
        self
    }
}

impl IntoCompatNode for ListCardCounter {
    fn into_compat_node(self) -> poodle_node::Node {
        poodle_render::list_card_counter(&self.spec, &RenderContext::new(&self.theme), self.on_link_click)
    }
}

impl IntoElement for ListCardCounter {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_compat_node())
    }
}

pub(crate) struct MetricTile {
    spec: MetricTileSpec,
    theme: GpuiThemeProvider,
}

impl MetricTile {
    pub(crate) fn from_spec(spec: MetricTileSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for MetricTile {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::metric_tile(&self.spec, &RenderContext::new(&self.theme)))
    }
}

pub(crate) struct StateTile {
    spec: StateTileSpec,
    theme: GpuiThemeProvider,
}

impl StateTile {
    pub(crate) fn from_spec(spec: StateTileSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for StateTile {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::state_tile(&self.spec, &RenderContext::new(&self.theme)))
    }
}

pub(crate) struct Code {
    spec: CodeSpec,
    theme: GpuiThemeProvider,
}

impl Code {
    pub(crate) fn from_spec(spec: CodeSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::code(&self.spec, &RenderContext::new(&self.theme))
    }
}

impl IntoCompatNode for Code {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Code {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct Surface {
    spec: SurfaceSpec,
    theme: GpuiThemeProvider,
    content: Option<poodle_node::Node>,
}

impl Surface {
    pub(crate) fn from_spec(spec: SurfaceSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
        }
    }

    pub(crate) fn with_content(mut self, content: impl IntoCompatNode) -> Self {
        self.content = Some(content.into_compat_node());
        self
    }

    pub(crate) fn into_node_with(self, ctx: &RenderContext<'_>) -> poodle_node::Node {
        poodle_render::surface(&self.spec, ctx, self.content.into_iter().collect())
    }

    fn into_node(self) -> poodle_node::Node {
        let theme = self.theme.clone();
        self.into_node_with(&RenderContext::new(&theme))
    }
}

impl IntoCompatNode for Surface {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Surface {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct ListGrid {
    spec: ListGridSpec,
    theme: GpuiThemeProvider,
    header: Option<poodle_node::Node>,
    children: Vec<poodle_node::Node>,
}

impl ListGrid {
    pub(crate) fn from_spec(spec: ListGridSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            header: None,
            children: Vec::new(),
        }
    }

    pub(crate) fn with_header(mut self, header: impl IntoCompatNode) -> Self {
        self.header = Some(header.into_compat_node());
        self
    }

    pub(crate) fn with_child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::list_grid(&self.spec, &RenderContext::new(&self.theme), self.header, self.children)
    }
}

impl IntoCompatNode for ListGrid {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for ListGrid {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct ListContainer {
    spec: ListContainerSpec,
    theme: GpuiThemeProvider,
    content: Option<poodle_node::Node>,
}

impl ListContainer {
    pub(crate) fn from_spec(spec: ListContainerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
        }
    }

    pub(crate) fn with_content(mut self, content: impl IntoCompatNode) -> Self {
        self.content = Some(content.into_compat_node());
        self
    }
}

impl IntoElement for ListContainer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let slots = poodle_render::ListContainerSlots {
            content: self.content,
            ..Default::default()
        };
        poodle_gpui_node_backend::to_gpui(&poodle_render::list_container(
            &self.spec,
            &RenderContext::new(&self.theme),
            slots,
            None,
        ))
    }
}

pub(crate) struct InlineListSection {
    spec: InlineListSectionSpec,
    theme: GpuiThemeProvider,
    items: Vec<poodle_node::Node>,
    action: Option<poodle_node::Node>,
}

impl InlineListSection {
    pub(crate) fn from_spec(spec: InlineListSectionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            items: Vec::new(),
            action: None,
        }
    }

    pub(crate) fn with_action(mut self, action: impl IntoCompatNode) -> Self {
        self.action = Some(action.into_compat_node());
        self
    }

    pub(crate) fn item(mut self, item: impl IntoCompatNode) -> Self {
        self.items.push(item.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::inline_list_section(&self.spec, &RenderContext::new(&self.theme), self.items, self.action)
    }
}

impl IntoCompatNode for InlineListSection {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for InlineListSection {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct ListCard {
    spec: ListCardSpec,
    theme: GpuiThemeProvider,
    leading: Option<poodle_node::Node>,
    corner: Option<poodle_node::Node>,
    footer: Option<poodle_node::Node>,
    trailing: Option<poodle_node::Node>,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl ListCard {
    pub(crate) fn from_spec(spec: ListCardSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            leading: None,
            corner: None,
            footer: None,
            trailing: None,
            on_click: None,
        }
    }

    pub(crate) fn with_leading(mut self, element: impl IntoCompatNode) -> Self {
        self.leading = Some(element.into_compat_node());
        self
    }

    pub(crate) fn with_corner(mut self, element: impl IntoCompatNode) -> Self {
        self.corner = Some(element.into_compat_node());
        self
    }

    pub(crate) fn with_footer(mut self, element: impl IntoCompatNode) -> Self {
        self.footer = Some(element.into_compat_node());
        self
    }

    pub(crate) fn with_trailing(mut self, element: impl IntoCompatNode) -> Self {
        self.trailing = Some(element.into_compat_node());
        self
    }

    pub(crate) fn on_click(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_click = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let selection_precedes_leading = self.spec.is_selectable
            && self.spec.selection_indicator == poodle_specs::SelectionIndicator::Checkbox;
        let active = self.spec.is_active;
        let title = self.spec.title.clone();
        let mut node = poodle_render::list_card(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::ListCardSlots {
                // The old GPUI tier always painted an empty leading shell when
                // no slot was supplied; the shared renderer otherwise derives
                // a first-letter glyph.
                leading: Some(self.leading.unwrap_or_else(poodle_node::Node::container)),
                corner: self.corner,
                footer: self.footer,
                trailing: self.trailing,
                ..Default::default()
            },
            self.on_click,
        );
        node.id = Some(format!("poodle-list-card-{title}"));

        // GPUI 0.2.2 cannot paint the shared inset active bar. Preserve the
        // old tier's child approximation in this preview adapter.
        if active {
            let accent = node
                .style
                .shadow_layers
                .last()
                .expect("active ListCard shadow carries its accent")
                .color;
            let radius = node.style.descriptor.corner_radii.top_left;
            let mut bar = poodle_node::Node::container();
            bar.style.descriptor.layout.direction = poodle_node::LayoutDirection::Row;
            bar.style.descriptor.layout.width = poodle_node::LayoutSizing::Fixed(
                poodle_render::presentation::rem_to_px(self.spec.active_bar_width_rem()),
            );
            bar.style.fill_height = true;
            bar.style.flex_shrink_zero = true;
            bar.style.descriptor.background = Some(accent);
            bar.style.descriptor.corner_radii.top_left = radius;
            bar.style.descriptor.corner_radii.bottom_left = radius;
            node.children
                .insert(usize::from(selection_precedes_leading), bar);
        }
        node
    }
}

impl IntoCompatNode for ListCard {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for ListCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

/// Preview bridge for the first form-control slice. It keeps the old
/// constructor shape while the stored value and interaction intent now flow
/// through `poodle-render` and the node backend.
pub(crate) struct TextInput {
    spec: TextInputSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<poodle_node::TextChangeHandler>,
    on_selection_change: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
    on_focus_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

impl TextInput {
    pub(crate) fn from_spec(spec: TextInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
            on_selection_change: None,
            on_focus_change: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_change(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_change = Some(Arc::new(handler));
        self
    }

    pub(crate) fn on_focus_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_focus_change = Some(handler);
        self
    }

    pub(crate) fn on_selection_change(
        mut self,
        handler: Arc<dyn Fn(usize, usize) + Send + Sync>,
    ) -> Self {
        self.on_selection_change = Some(handler);
        self
    }

    pub(crate) fn into_node_with(self, ctx: &RenderContext<'_>) -> poodle_node::Node {
        let mut node = poodle_render::text_input_with_handlers(
            &self.spec,
            ctx,
            poodle_render::TextInputHandlers {
                on_change: self.on_change,
                on_selection_change: self.on_selection_change,
                on_focus_change: self.on_focus_change,
                ..poodle_render::TextInputHandlers::default()
            },
        );
        if let Some(id) = self.id_suffix {
            node.id = Some(format!("poodle-input-{id}"));
        }
        node
    }

    fn into_node(self) -> poodle_node::Node {
        let theme = self.theme.clone();
        self.into_node_with(&RenderContext::new(&theme))
    }

    /// Deferred construction for a scoped slot (architecture 010): the
    /// boundary invokes the builder with its internal presentation scope.
    pub(crate) fn into_slot(self) -> SlotBuilder<'static> {
        std::boxed::Box::new(move |ctx| self.into_node_with(ctx))
    }
}

impl IntoElement for TextInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl IntoCompatNode for TextInput {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

pub(crate) struct Select {
    spec: SelectSpec,
    theme: GpuiThemeProvider,
    instance_scope: String,
    id_suffix: Option<String>,
    on_transition: Option<Arc<dyn Fn(poodle_render::SelectTransitionResult) + Send + Sync>>,
}

impl Select {
    pub(crate) fn from_spec(
        spec: SelectSpec,
        theme: &GpuiThemeProvider,
        instance_scope: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            instance_scope: instance_scope.into(),
            id_suffix: None,
            on_transition: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn into_node_with(self, ctx: &RenderContext<'_>) -> poodle_node::Node {
        let mut handlers = poodle_render::SelectHandlers::new(self.instance_scope);
        if let Some(on_transition) = self.on_transition {
            handlers = handlers.on_transition(on_transition);
        }
        let mut node = poodle_render::select(&self.spec, ctx, &handlers);
        if let Some(id) = self.id_suffix {
            node.id = Some(format!("poodle-select-{id}"));
        }
        node
    }

    fn into_node(self) -> poodle_node::Node {
        let theme = self.theme.clone();
        self.into_node_with(&RenderContext::new(&theme))
    }

    /// Deferred construction for a scoped slot (architecture 010): the
    /// boundary invokes the builder with its internal presentation scope.
    pub(crate) fn into_slot(self) -> SlotBuilder<'static> {
        std::boxed::Box::new(move |ctx| self.into_node_with(ctx))
    }
}

impl IntoCompatNode for Select {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Select {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct ColorPicker {
    spec: ColorPickerSpec,
    theme: GpuiThemeProvider,
    instance_id: String,
    id_suffix: Option<String>,
    on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl ColorPicker {
    pub(crate) fn from_spec(
        spec: ColorPickerSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            instance_id: instance_id.into(),
            id_suffix: None,
            on_toggle: None,
            on_change: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_toggle = Some(handler);
        self
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut node = poodle_render::color_picker(
            &self.spec,
            &RenderContext::new(&self.theme),
            &self.instance_id,
            poodle_render::ColorPickerHandlers {
                on_toggle: self.on_toggle,
                on_change: self.on_change,
            },
        );
        if let Some(id) = self.id_suffix {
            node.id = Some(format!("poodle-color-picker-{id}"));
        }
        node
    }
}

impl IntoElement for ColorPicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct CodeInput {
    spec: CodeInputSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_complete: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_selection_change: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
}

impl CodeInput {
    pub(crate) fn from_spec(spec: CodeInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
            on_complete: None,
            on_selection_change: None,
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn density(self, density: ControlDensity) -> Self {
        self.with_density(density)
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }

    pub(crate) fn on_complete(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_complete = Some(handler);
        self
    }

    pub(crate) fn on_selection_change(
        mut self,
        handler: Arc<dyn Fn(usize, usize) + Send + Sync>,
    ) -> Self {
        self.on_selection_change = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut node = poodle_render::code_input_with_handlers(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::CodeInputHandlers {
                on_value_change: self.on_change,
                on_complete: self.on_complete,
                on_selection_change: self.on_selection_change,
            },
        );
        if let Some(id) = self.id_suffix {
            node.id = Some(format!("poodle-code-input-{id}"));
        }
        node
    }
}

impl IntoElement for CodeInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct TokenInput {
    spec: TokenInputSpec,
    theme: GpuiThemeProvider,
    on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl TokenInput {
    pub(crate) fn from_spec(spec: TokenInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_remove: None,
        }
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::token_input(&self.spec, &RenderContext::new(&self.theme), self.on_remove)
    }
}

impl IntoElement for TokenInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct FileUpload {
    spec: FileUploadSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_browse: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl FileUpload {
    pub(crate) fn from_spec(spec: FileUploadSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_remove: None,
            on_browse: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn on_remove(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_remove = Some(handler);
        self
    }

    /// Wire the generic browse seam: pressing the dropzone requests one file
    /// through the shared capability (g15.007). The request is queued as an
    /// event; the preview host starts the OS prompt on its next frame.
    pub(crate) fn on_browse(
        mut self,
        queue: Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
        key: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Self {
        let key = key.into();
        let spec = SingleFilePickSpec {
            prompt: prompt.into(),
            accept: self.spec.accept.clone(),
            max_size: self.spec.max_size,
        };
        self.on_browse = Some(Arc::new(move || {
            queue.lock().unwrap().push(NodeSpecimenEvent::FileBrowse {
                key: key.clone(),
                spec: spec.clone(),
                failed_message: None,
            });
        }));
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut node = poodle_render::file_upload_with_handlers(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::FileUploadHandlers {
                on_browse: self.on_browse,
                on_remove: self.on_remove,
            },
        );
        if let Some(id) = self.id_suffix {
            node.id = Some(format!("poodle-file-upload-{id}"));
        }
        node
    }
}

impl IntoElement for FileUpload {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct LicenceStatus {
    spec: LicenceStatusSpec,
    theme: GpuiThemeProvider,
}

impl LicenceStatus {
    pub(crate) fn from_spec(spec: LicenceStatusSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    pub(crate) fn with_size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for LicenceStatus {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::licence_status(&self.spec, &RenderContext::new(&self.theme)))
    }
}

pub(crate) struct LicenceSeats {
    spec: LicenceSeatsSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::LicenceSeatsHandlers,
}

impl LicenceSeats {
    pub(crate) fn from_spec(spec: LicenceSeatsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::LicenceSeatsHandlers::default(),
        }
    }

    pub(crate) fn on_rename(
        mut self,
        handler: Arc<dyn Fn(&str, Option<&str>) + Send + Sync>,
    ) -> Self {
        self.handlers.on_rename = Some(handler);
        self
    }

    pub(crate) fn on_rename_edit(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_rename_edit = Some(handler);
        self
    }

    pub(crate) fn on_release(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_release = Some(handler);
        self
    }

    pub(crate) fn on_release_trigger(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_release_trigger = Some(handler);
        self
    }

    pub(crate) fn on_release_cancel(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_release_cancel = Some(handler);
        self
    }
}

impl IntoElement for LicenceSeats {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::licence_seats(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

pub(crate) struct LicenceActivation {
    spec: LicenceActivationSpec,
    theme: GpuiThemeProvider,
    account_content: Option<poodle_node::Node>,
    handlers: poodle_render::LicenceActivationHandlers,
}

impl LicenceActivation {
    pub(crate) fn from_spec(spec: LicenceActivationSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            account_content: None,
            handlers: poodle_render::LicenceActivationHandlers::default(),
        }
    }

    pub(crate) fn with_account_content(mut self, content: poodle_node::Node) -> Self {
        self.account_content = Some(content);
        self
    }

    pub(crate) fn on_key_change(mut self, handler: poodle_node::TextChangeHandler) -> Self {
        self.handlers.on_key_change = Some(handler);
        self
    }

    pub(crate) fn on_key_selection_change(
        mut self,
        handler: Arc<dyn Fn(usize, usize) + Send + Sync>,
    ) -> Self {
        self.handlers.on_key_selection_change = Some(handler);
        self
    }

    pub(crate) fn on_machine_label_change(
        mut self,
        handler: Arc<dyn Fn(&str) + Send + Sync>,
    ) -> Self {
        self.handlers.on_machine_label_change = Some(handler);
        self
    }

    pub(crate) fn on_machine_label_commit(
        mut self,
        handler: Arc<dyn Fn(&str) + Send + Sync>,
    ) -> Self {
        self.handlers.on_machine_label_commit = Some(handler);
        self
    }

    pub(crate) fn on_machine_label_cancel(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_machine_label_cancel = Some(handler);
        self
    }

    pub(crate) fn on_machine_label_edit(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_machine_label_edit = Some(handler);
        self
    }

    pub(crate) fn on_view_change(
        mut self,
        handler: Arc<dyn Fn(poodle_headless::licence::LicenceActivationRoute) + Send + Sync>,
    ) -> Self {
        self.handlers.on_view_change = Some(handler);
        self
    }

    pub(crate) fn on_submit(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_submit = Some(handler);
        self
    }

    pub(crate) fn on_file_browse(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_file_browse = Some(handler);
        self
    }

    pub(crate) fn on_file_remove(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_file_remove = Some(handler);
        self
    }

    pub(crate) fn on_key_check(
        mut self,
        handler: Arc<dyn Fn(&str) -> poodle_headless::licence::LicenceKeyResult + Send + Sync>,
    ) -> Self {
        self.handlers.on_key_check = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::licence_activation_with_slots(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.account_content,
            self.handlers,
        )
    }
}

impl IntoElement for LicenceActivation {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

// ── Model-connection family ────────────────────────────────────────────────
//
// Four bridges over the shared `poodle-render` compositions. Host content is
// keyed by opaque id at this seam, never inside a spec, and every focus
// request the components name is performed here by the backend.

pub(crate) struct ModelConnectionPicker {
    spec: poodle_specs::ModelConnectionPickerSpec,
    theme: GpuiThemeProvider,
    slots: poodle_render::ModelConnectionPickerSlots,
    handlers: poodle_render::ModelConnectionPickerHandlers,
}

impl ModelConnectionPicker {
    pub(crate) fn from_spec(
        spec: poodle_specs::ModelConnectionPickerSpec,
        theme: &GpuiThemeProvider,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            slots: poodle_render::ModelConnectionPickerSlots::default(),
            handlers: poodle_render::ModelConnectionPickerHandlers::default(),
        }
    }

    /// Stable backend-state scope. Two pickers over the same routes would
    /// otherwise share one focus handle per option id.
    pub(crate) fn with_instance_id(mut self, instance_id: &str) -> Self {
        self.handlers.instance_id = Some(instance_id.to_string());
        self
    }

    pub(crate) fn with_leading(mut self, option_id: &str, mark: poodle_node::Node) -> Self {
        self.slots.leading.insert(option_id.to_string(), mark);
        self
    }

    pub(crate) fn with_footer(mut self, footer: poodle_node::Node) -> Self {
        self.slots.footer = Some(footer);
        self
    }

    pub(crate) fn on_value_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_value_change = Some(handler);
        self
    }

    pub(crate) fn on_query_change(mut self, handler: poodle_node::TextChangeHandler) -> Self {
        self.handlers.on_query_change = Some(handler);
        self
    }
}

impl IntoElement for ModelConnectionPicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::model_connection_picker_with_slots(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.slots,
            self.handlers,
        ))
    }
}

pub(crate) struct ModelConnectionSetup {
    spec: poodle_specs::ModelConnectionSetupSpec,
    theme: GpuiThemeProvider,
    slots: poodle_render::ModelConnectionSetupSlots,
    handlers: poodle_render::ModelConnectionSetupHandlers,
}

impl ModelConnectionSetup {
    pub(crate) fn from_spec(
        spec: poodle_specs::ModelConnectionSetupSpec,
        theme: &GpuiThemeProvider,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            slots: poodle_render::ModelConnectionSetupSlots::default(),
            handlers: poodle_render::ModelConnectionSetupHandlers::default(),
        }
    }

    /// Stable backend-state scope, forwarded to the composed picker.
    pub(crate) fn with_instance_id(mut self, instance_id: &str) -> Self {
        self.handlers.instance_id = Some(instance_id.to_string());
        self
    }

    pub(crate) fn with_leading(mut self, option_id: &str, mark: poodle_node::Node) -> Self {
        self.slots
            .picker
            .leading
            .insert(option_id.to_string(), mark);
        self
    }

    /// The host's configuration body. Poodle never reads its values.
    pub(crate) fn with_configuration(mut self, content: poodle_node::Node) -> Self {
        self.slots.configuration = Some(content);
        self
    }

    pub(crate) fn with_configure_aside(mut self, content: poodle_node::Node) -> Self {
        self.slots.configure_aside = Some(content);
        self
    }

    pub(crate) fn on_stage_change(
        mut self,
        handler: Arc<
            dyn Fn(poodle_headless::model_connection::ModelConnectionSetupStage) + Send + Sync,
        >,
    ) -> Self {
        self.handlers.on_stage_change = Some(handler);
        self
    }

    pub(crate) fn on_value_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_value_change = Some(handler);
        self
    }

    pub(crate) fn on_query_change(mut self, handler: poodle_node::TextChangeHandler) -> Self {
        self.handlers.on_query_change = Some(handler);
        self
    }

    pub(crate) fn on_submit(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_submit = Some(handler);
        self
    }

    pub(crate) fn on_cancel(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.handlers.on_cancel = Some(handler);
        self
    }

    pub(crate) fn on_focus_request(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_focus_request = Some(handler);
        self
    }
}

impl IntoElement for ModelConnectionSetup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::model_connection_setup_with_slots(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.slots,
            self.handlers,
        ))
    }
}

pub(crate) struct ModelConnectionCard {
    spec: poodle_specs::ModelConnectionCardSpec,
    theme: GpuiThemeProvider,
    slots: poodle_render::ModelConnectionCardSlots,
    handlers: poodle_render::ModelConnectionCardHandlers,
}

impl ModelConnectionCard {
    pub(crate) fn from_spec(
        spec: poodle_specs::ModelConnectionCardSpec,
        theme: &GpuiThemeProvider,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            slots: poodle_render::ModelConnectionCardSlots::default(),
            handlers: poodle_render::ModelConnectionCardHandlers::default(),
        }
    }

    /// Stable backend-state scope, for two surfaces showing one connection.
    pub(crate) fn with_instance_id(mut self, instance_id: &str) -> Self {
        self.handlers.instance_id = Some(instance_id.to_string());
        self
    }

    pub(crate) fn with_leading(mut self, mark: poodle_node::Node) -> Self {
        self.slots.leading = Some(mark);
        self
    }

    pub(crate) fn with_badges(mut self, badges: poodle_node::Node) -> Self {
        self.slots.badges = Some(badges);
        self
    }

    pub(crate) fn with_closed_accessory(mut self, accessory: poodle_node::Node) -> Self {
        self.slots.closed_accessory = Some(accessory);
        self
    }

    pub(crate) fn with_actions(mut self, actions: poodle_node::Node) -> Self {
        self.slots.actions = Some(actions);
        self
    }

    pub(crate) fn with_details(mut self, details: poodle_node::Node) -> Self {
        self.slots.details = Some(details);
        self
    }

    pub(crate) fn on_open_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.handlers.on_open_change = Some(handler);
        self
    }

    pub(crate) fn on_enabled_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.handlers.on_enabled_change = Some(handler);
        self
    }

    pub(crate) fn on_focus_request(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_focus_request = Some(handler);
        self
    }
}

impl IntoElement for ModelConnectionCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::model_connection_card_with_slots(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.slots,
            self.handlers,
        ))
    }
}

pub(crate) struct ModelCatalogueEditor {
    spec: poodle_specs::ModelCatalogueEditorSpec,
    theme: GpuiThemeProvider,
    slots: poodle_render::ModelCatalogueEditorSlots,
    handlers: poodle_render::ModelCatalogueEditorHandlers,
}

impl ModelCatalogueEditor {
    pub(crate) fn from_spec(
        spec: poodle_specs::ModelCatalogueEditorSpec,
        theme: &GpuiThemeProvider,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            slots: poodle_render::ModelCatalogueEditorSlots::default(),
            handlers: poodle_render::ModelCatalogueEditorHandlers::default(),
        }
    }

    /// Stable backend-state scope. Two editors over the same catalogue would
    /// otherwise share one focus handle per item id.
    pub(crate) fn with_instance_id(mut self, instance_id: &str) -> Self {
        self.handlers.instance_id = Some(instance_id.to_string());
        self
    }

    pub(crate) fn with_leading(mut self, item_id: &str, mark: poodle_node::Node) -> Self {
        self.slots.leading.insert(item_id.to_string(), mark);
        self
    }

    pub(crate) fn with_row_meta(mut self, item_id: &str, meta: poodle_node::Node) -> Self {
        self.slots.row_meta.insert(item_id.to_string(), meta);
        self
    }

    pub(crate) fn with_custom_action(mut self, action: poodle_node::Node) -> Self {
        self.slots.custom_action = Some(action);
        self
    }

    pub(crate) fn on_order_change(mut self, handler: Arc<dyn Fn(&[String]) + Send + Sync>) -> Self {
        self.handlers.on_order_change = Some(handler);
        self
    }

    pub(crate) fn on_visibility_change(
        mut self,
        handler: Arc<
            dyn Fn(&poodle_headless::model_connection::ModelCatalogueVisibilityChange)
                + Send
                + Sync,
        >,
    ) -> Self {
        self.handlers.on_visibility_change = Some(handler);
        self
    }

    pub(crate) fn on_info(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_info = Some(handler);
        self
    }

    pub(crate) fn on_grab_change(
        mut self,
        handler: Arc<dyn Fn(Option<&str>) + Send + Sync>,
    ) -> Self {
        self.handlers.on_grab_change = Some(handler);
        self
    }

    pub(crate) fn on_drop_target_change(
        mut self,
        handler: Arc<dyn Fn(Option<&str>) + Send + Sync>,
    ) -> Self {
        self.handlers.on_drop_target_change = Some(handler);
        self
    }

    pub(crate) fn on_hidden_open_change(
        mut self,
        handler: Arc<dyn Fn(bool) + Send + Sync>,
    ) -> Self {
        self.handlers.on_hidden_open_change = Some(handler);
        self
    }

    pub(crate) fn on_announce(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_announce = Some(handler);
        self
    }

    pub(crate) fn on_focus_request(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_focus_request = Some(handler);
        self
    }
}

impl IntoElement for ModelCatalogueEditor {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::model_catalogue_editor_with_slots(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.slots,
            self.handlers,
        ))
    }
}

pub(crate) struct Breadcrumbs {
    spec: BreadcrumbsSpec,
    theme: GpuiThemeProvider,
    on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl Breadcrumbs {
    pub(crate) fn from_spec(spec: BreadcrumbsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_navigate: None,
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_navigate(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_navigate = Some(handler);
        self
    }

    pub(crate) fn into_node_with(self, ctx: &RenderContext<'_>) -> poodle_node::Node {
        poodle_render::breadcrumbs(&self.spec, ctx, self.on_navigate)
    }

    fn into_node(self) -> poodle_node::Node {
        let theme = self.theme.clone();
        self.into_node_with(&RenderContext::new(&theme))
    }

    /// Deferred construction for a scoped slot (architecture 010): the
    /// boundary invokes the builder with its internal presentation scope.
    pub(crate) fn into_slot(self) -> SlotBuilder<'static> {
        std::boxed::Box::new(move |ctx| self.into_node_with(ctx))
    }
}

impl IntoElement for Breadcrumbs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

impl IntoCompatNode for Breadcrumbs {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

pub(crate) struct TextLink {
    spec: TextLinkSpec,
    theme: GpuiThemeProvider,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl TextLink {
    pub(crate) fn from_spec(spec: TextLinkSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_click: None,
        }
    }
}

pub(crate) struct SelectionSummary {
    spec: SelectionSummarySpec,
    theme: GpuiThemeProvider,
    on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_clear: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl SelectionSummary {
    pub(crate) fn from_spec(spec: SelectionSummarySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_remove: None,
            on_clear: None,
        }
    }

    pub(crate) fn with_size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_remove(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_remove = Some(handler);
        self
    }

    pub(crate) fn on_clear(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_clear = Some(handler);
        self
    }
}

impl IntoElement for SelectionSummary {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::selection_summary(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::SelectionSummaryHandlers {
                on_remove: self.on_remove,
                on_clear: self.on_clear,
            },
        ))
    }
}

impl IntoElement for TextLink {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::text_link(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.on_click,
        ))
    }
}

pub(crate) struct Tabs {
    spec: TabsSpec,
    theme: GpuiThemeProvider,
    id: Option<String>,
    content: Vec<(String, gpui::AnyElement)>,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_close: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_reorder: Option<Arc<dyn Fn(Vec<String>) + Send + Sync>>,
    on_drag_start: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_drag_end: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_drop_target_change: Option<Arc<dyn Fn(Option<&str>) + Send + Sync>>,
    focused_value: Option<String>,
}

impl Tabs {
    pub(crate) fn from_spec(spec: TabsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id: None,
            content: Vec::new(),
            on_change: None,
            on_close: None,
            on_reorder: None,
            on_drag_start: None,
            on_drag_end: None,
            on_drop_target_change: None,
            focused_value: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }

    pub(crate) fn on_close(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_close = Some(handler);
        self
    }

    pub(crate) fn on_reorder(mut self, handler: Arc<dyn Fn(Vec<String>) + Send + Sync>) -> Self {
        self.on_reorder = Some(handler);
        self
    }

    pub(crate) fn on_drag_start(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_drag_start = Some(handler);
        self
    }

    pub(crate) fn on_drag_end(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_drag_end = Some(handler);
        self
    }

    pub(crate) fn on_drop_target_change(
        mut self,
        handler: Arc<dyn Fn(Option<&str>) + Send + Sync>,
    ) -> Self {
        self.on_drop_target_change = Some(handler);
        self
    }

    pub(crate) fn with_focused_value(mut self, value: impl Into<String>) -> Self {
        self.focused_value = Some(value.into());
        self
    }

    pub(crate) fn with_content(
        mut self,
        value: impl Into<String>,
        content: impl IntoElement,
    ) -> Self {
        self.content
            .push((value.into(), content.into_any_element()));
        self
    }

    fn into_node(
        self,
    ) -> (
        poodle_node::Node,
        TabsSpec,
        GpuiThemeProvider,
        Vec<(String, gpui::AnyElement)>,
    ) {
        let focused_value = self.focused_value.or_else(|| {
            self.id.as_ref().and_then(|scope| {
                self.spec
                    .tabs
                    .iter()
                    .find(|tab| {
                        poodle_gpui_node_backend::focus_state_for(&format!(
                            "tabs:{scope}:tab:{}",
                            tab.value
                        )) == Some(true)
                    })
                    .map(|tab| tab.value.clone())
            })
        });
        let node = poodle_render::tabs_with_handlers(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::TabsHandlers {
                on_change: self.on_change,
                on_close: self.on_close,
                on_reorder: self.on_reorder,
                on_drag_start: self.on_drag_start,
                on_drag_end: self.on_drag_end,
                on_drop_target_change: self.on_drop_target_change,
                focused_value,
                instance_id: self.id,
                ..poodle_render::TabsHandlers::default()
            },
        );
        (node, self.spec, self.theme, self.content)
    }
}

impl IntoElement for Tabs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let (node, spec, theme, content) = self.into_node();
        let current = spec.current_value().map(str::to_owned);
        let mut wrapper = div()
            .flex()
            .flex_col()
            .child(poodle_gpui_node_backend::to_gpui(&node));
        for (value, content) in content {
            if current.as_deref() == Some(value.as_str()) {
                wrapper = wrapper.child(
                    div()
                        .p(px(theme.resolve_space("space.panel.y")))
                        .child(content),
                );
                break;
            }
        }
        wrapper.into_any_element()
    }
}

pub(crate) struct TabStrip {
    spec: TabStripSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_close: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl TabStrip {
    pub(crate) fn from_spec(spec: TabStripSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
            on_close: None,
        }
    }

    pub(crate) fn with_id(self, _id: impl Into<String>) -> Self {
        self
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }

    pub(crate) fn on_close(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_close = Some(handler);
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for TabStrip {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::tab_strip(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::TabStripHandlers {
                on_select: self.on_change,
                on_close: self.on_close,
            },
        ))
    }
}

pub(crate) struct DurationInput {
    spec: DurationInputSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Arc<dyn Fn(u32, u32, u32, u64) + Send + Sync>>,
}

impl DurationInput {
    pub(crate) fn from_spec(spec: DurationInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_change(
        mut self,
        handler: Arc<dyn Fn(u32, u32, u32, u64) + Send + Sync>,
    ) -> Self {
        self.on_change = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut node = poodle_render::duration_input_with_handlers(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::DurationInputHandlers {
                on_change: self.on_change,
            },
        );
        if let Some(id) = self.id_suffix {
            node.id = Some(format!("poodle-duration-input-{id}"));
        }
        node
    }
}

impl IntoElement for DurationInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct NumberInput {
    spec: NumberInputSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_increment: Option<Arc<dyn Fn() + Send + Sync>>,
    on_decrement: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl NumberInput {
    pub(crate) fn from_spec(spec: NumberInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_increment: None,
            on_decrement: None,
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_increment(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_increment = Some(handler);
        self
    }

    pub(crate) fn on_decrement(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_decrement = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut node = poodle_render::number_input(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::NumberInputHandlers {
                on_increment: self.on_increment,
                on_decrement: self.on_decrement,
            },
        );
        if let Some(id) = self.id_suffix {
            node.id = Some(format!("poodle-number-input-{id}"));
        }
        node
    }
}

impl IntoElement for NumberInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

/// Preview bridge for the date-picker family. Open state and values remain
/// specimen-owned; the render node only reports interaction intent.
macro_rules! define_date_picker_compat {
    ($name:ident, $spec:ty, $render:path, $id_prefix:literal) => {
        pub(crate) struct $name {
            spec: $spec,
            theme: GpuiThemeProvider,
            id_suffix: Option<String>,
            on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
            on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
        }

        impl $name {
            pub(crate) fn from_spec(spec: $spec, theme: &GpuiThemeProvider) -> Self {
                Self {
                    spec,
                    theme: theme.clone(),
                    id_suffix: None,
                    on_toggle: None,
                    on_select: None,
                }
            }

            pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
                self.id_suffix = Some(id.into());
                self
            }

            pub(crate) fn size(mut self, size: ControlSize) -> Self {
                self.spec.size = Some(size);
                self
            }

            pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
                self.spec.density = Some(density);
                self
            }

            pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
                self.on_toggle = Some(handler);
                self
            }

            fn into_node(self) -> poodle_node::Node {
                let handlers = poodle_render::DatePickerHandlers {
                    on_toggle: self.on_toggle,
                    on_select: self.on_select,
                    on_navigate: None,
                };
                let mut node = $render(&self.spec, &RenderContext::new(&self.theme), handlers);
                if let Some(id) = self.id_suffix {
                    node.id = Some(format!(concat!($id_prefix, "{}"), id));
                }
                node
            }
        }

        impl IntoElement for $name {
            type Element = AnyElement;

            fn into_element(self) -> Self::Element {
                poodle_gpui_node_backend::to_gpui(&self.into_node())
            }
        }
    };
}

define_date_picker_compat!(
    DatePicker,
    DatePickerSpec,
    poodle_render::date_picker,
    "poodle-date-picker-"
);

impl DatePicker {
    pub(crate) fn on_select(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_select = Some(handler);
        self
    }
}

define_date_picker_compat!(
    DateRangePicker,
    DateRangePickerSpec,
    poodle_render::date_range_picker,
    "poodle-date-range-picker-"
);
define_date_picker_compat!(
    DateTimePicker,
    DateTimePickerSpec,
    poodle_render::date_time_picker,
    "poodle-datetime-picker-"
);
define_date_picker_compat!(
    DateTimeRangePicker,
    DateTimeRangePickerSpec,
    poodle_render::date_time_range_picker,
    "poodle-datetime-range-picker-"
);

pub(crate) struct Calendar {
    spec: CalendarSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_range_select: Option<Arc<dyn Fn(&DateRangeValue) + Send + Sync>>,
}

impl Calendar {
    pub(crate) fn from_spec(spec: CalendarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_select: None,
            on_navigate: None,
            on_range_select: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_select(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_select = Some(handler);
        self
    }

    pub(crate) fn on_navigate(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_navigate = Some(handler);
        self
    }

    pub(crate) fn on_range_select(
        mut self,
        handler: Arc<dyn Fn(&DateRangeValue) + Send + Sync>,
    ) -> Self {
        self.on_range_select = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut node = poodle_render::calendar(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::CalendarHandlers {
                on_select: self.on_select,
                on_range_select: self.on_range_select,
                on_navigate: self.on_navigate,
            },
        );
        if let Some(id) = self.id_suffix {
            node.id = Some(format!("poodle-calendar-{id}"));
        }
        node
    }
}

impl IntoElement for Calendar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct DateTimeZonePicker {
    spec: DateTimeZonePickerSpec,
    theme: GpuiThemeProvider,
    instance_id: String,
    on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl DateTimeZonePicker {
    pub(crate) fn from_spec(
        spec: DateTimeZonePickerSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            instance_id: instance_id.into(),
            on_toggle: None,
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_toggle = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::date_time_zone_picker(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::DateTimeZonePickerHandlers {
                instance_id: self.instance_id,
                on_toggle: self.on_toggle,
                on_select: None,
                on_navigate: None,
                on_zone_toggle: None,
                on_zone_change: None,
            },
        )
    }
}

impl IntoElement for DateTimeZonePicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct TimeField {
    spec: TimeFieldSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl TimeField {
    pub(crate) fn from_spec(spec: TimeFieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut node =
            poodle_render::time_field_with_change(&self.spec, &RenderContext::new(&self.theme), self.on_change);
        if let Some(id) = self.id_suffix {
            node.id = Some(format!("poodle-time-field-{id}"));
        }
        node
    }
}

impl IntoElement for TimeField {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct TimeZoneSelect {
    spec: TimeZoneSelectSpec,
    theme: GpuiThemeProvider,
    instance_id: String,
    on_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl TimeZoneSelect {
    pub(crate) fn from_spec(
        spec: TimeZoneSelectSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            instance_id: instance_id.into(),
            on_toggle: None,
        }
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_toggle = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::time_zone_select(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::TimeZoneSelectHandlers {
                instance_id: self.instance_id,
                on_toggle: self.on_toggle,
                on_change: None,
            },
        )
    }
}

impl IntoElement for TimeZoneSelect {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct EmbedInput {
    spec: EmbedInputSpec,
    theme: GpuiThemeProvider,
}

impl EmbedInput {
    pub(crate) fn from_spec(spec: EmbedInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    pub(crate) fn into_node_with(self, ctx: &RenderContext<'_>) -> poodle_node::Node {
        poodle_render::embed_input(&self.spec, ctx)
    }

    fn into_node(self) -> poodle_node::Node {
        let theme = self.theme.clone();
        self.into_node_with(&RenderContext::new(&theme))
    }

    /// Deferred construction for a scoped slot (architecture 010): the
    /// boundary invokes the builder with its internal presentation scope.
    pub(crate) fn into_slot(self) -> SlotBuilder<'static> {
        std::boxed::Box::new(move |ctx| self.into_node_with(ctx))
    }
}

impl IntoElement for EmbedInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct Field {
    spec: FieldSpec,
    theme: GpuiThemeProvider,
    control: Option<SlotBuilder<'static>>,
}

impl Field {
    pub(crate) fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        theme: &GpuiThemeProvider,
    ) -> Self {
        Self::from_spec(FieldSpec::new(id, label), theme)
    }

    pub(crate) fn from_spec(spec: FieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            control: None,
        }
    }

    pub(crate) fn with_control(
        mut self,
        control: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.control = Some(std::boxed::Box::new(control));
        self
    }

    pub(crate) fn with_embed_control(
        self,
        control: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.with_control(control)
    }

    pub(crate) fn validation_state(mut self, state: poodle_specs::ValidationState) -> Self {
        self.spec.validation_state = state;
        self
    }

    pub(crate) fn error(mut self, error: impl Into<String>) -> Self {
        self.spec.error = Some(error.into());
        self
    }
}

impl IntoElement for Field {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::field(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.control,
        ))
    }
}

impl IntoCompatNode for Field {
    fn into_compat_node(self) -> poodle_node::Node {
        poodle_render::field(&self.spec, &RenderContext::new(&self.theme), self.control)
    }
}

pub(crate) struct Button {
    spec: ButtonSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl Button {
    pub(crate) fn from_spec(spec: ButtonSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_click: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn on_click(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_click = Some(handler);
        self
    }

    pub(crate) fn into_node_with(self, ctx: &RenderContext<'_>) -> poodle_node::Node {
        let id = self
            .id_suffix
            .unwrap_or_else(|| self.spec.label.clone().unwrap_or_default());
        let mut node = poodle_render::button(&self.spec, ctx, self.on_click);
        node.id = Some(format!("poodle-btn-{id}"));
        node
    }

    fn into_node(self) -> poodle_node::Node {
        let theme = self.theme.clone();
        self.into_node_with(&RenderContext::new(&theme))
    }

    /// Deferred construction for a scoped slot (architecture 010): the
    /// boundary invokes the builder with its internal presentation scope.
    pub(crate) fn into_slot(self) -> SlotBuilder<'static> {
        std::boxed::Box::new(move |ctx| self.into_node_with(ctx))
    }
}

impl IntoCompatNode for Button {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Button {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct Accordion {
    spec: AccordionSpec,
    theme: GpuiThemeProvider,
    content: Vec<(String, poodle_node::Node)>,
    handlers: AccordionHandlers,
}

impl Accordion {
    pub(crate) fn from_spec(
        spec: AccordionSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: Vec::new(),
            handlers: AccordionHandlers::new(instance_id),
        }
    }

    pub(crate) fn on_value_change(
        mut self,
        handler: Arc<dyn Fn(AccordionSelectionValue) + Send + Sync>,
    ) -> Self {
        self.handlers.on_value_change = Some(handler);
        self
    }

    pub(crate) fn with_content(
        mut self,
        value: impl Into<String>,
        content: impl IntoCompatNode,
    ) -> Self {
        self.content
            .push((value.into(), content.into_compat_node()));
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for Accordion {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::accordion_with_content(
            &self.spec,
            &RenderContext::new(&self.theme),
            &self.content,
            self.handlers,
        ))
    }
}

pub(crate) struct Collapsible {
    spec: CollapsibleSpec,
    theme: GpuiThemeProvider,
    content: Option<poodle_node::Node>,
    on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    instance_id: Option<String>,
}

impl Collapsible {
    pub(crate) fn from_spec(spec: CollapsibleSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            on_toggle: None,
            instance_id: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.instance_id = Some(id.into());
        self
    }

    pub(crate) fn on_toggle(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_toggle = Some(handler);
        self
    }

    pub(crate) fn with_content(mut self, content: impl IntoCompatNode) -> Self {
        self.content = Some(content.into_compat_node());
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for Collapsible {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::collapsible_with_handlers(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.content,
            poodle_render::CollapsibleHandlers {
                on_open_change: self.on_toggle,
                instance_id: self.instance_id,
            },
        ))
    }
}

pub(crate) struct Menubar {
    spec: MenubarSpec,
    theme: GpuiThemeProvider,
    on_trigger: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl Menubar {
    pub(crate) fn from_spec(spec: MenubarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_trigger: None,
            on_select: None,
        }
    }

    pub(crate) fn with_id(self, _id: impl Into<String>) -> Self {
        self
    }

    pub(crate) fn on_trigger(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_trigger = Some(handler);
        self
    }

    pub(crate) fn on_select(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_select = Some(handler);
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for Menubar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::menubar(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.on_trigger,
            self.on_select,
        ))
    }
}

pub(crate) struct NavigationMenu {
    spec: NavigationMenuSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl NavigationMenu {
    pub(crate) fn from_spec(spec: NavigationMenuSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
        }
    }

    pub(crate) fn with_id(self, _id: impl Into<String>) -> Self {
        self
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for NavigationMenu {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::navigation_menu(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.on_change,
        ))
    }
}

pub(crate) struct AlertDialog {
    spec: AlertDialogSpec,
    theme: GpuiThemeProvider,
    working: bool,
    working_label: String,
}

impl AlertDialog {
    pub(crate) fn from_spec(spec: AlertDialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            working: false,
            working_label: poodle_render::alert_dialog::DEFAULT_WORKING_LABEL.to_string(),
        }
    }

    pub(crate) fn open(mut self, open: bool) -> Self {
        self.spec.open = Some(open);
        self
    }

    pub(crate) fn working(mut self, working: bool) -> Self {
        self.working = working;
        self
    }

    pub(crate) fn working_label(mut self, label: impl Into<String>) -> Self {
        self.working_label = label.into();
        self
    }

    pub(crate) fn item_detail(
        mut self,
        label: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.spec.item_label = Some(label.into());
        self.spec.item_value = Some(value.into());
        self
    }
}

impl IntoElement for AlertDialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut node = poodle_render::alert_dialog(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.working,
            &self.working_label,
            poodle_render::AlertDialogHandlers::default(),
        );
        native_alert_dialog_spacing(
            &mut node,
            self.spec.item_label.is_some(),
            self.theme.resolve_space("typography.body.size"),
        );
        native_dialog_element(node)
    }
}

pub(crate) struct ConfirmAction {
    spec: ConfirmActionSpec,
    theme: GpuiThemeProvider,
    trigger: Option<poodle_node::Node>,
    content: Option<poodle_node::Node>,
    confirm: Option<Arc<dyn Fn() + Send + Sync>>,
    cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl ConfirmAction {
    pub(crate) fn from_spec(spec: ConfirmActionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            content: None,
            confirm: None,
            cancel: None,
        }
    }

    pub(crate) fn with_trigger(mut self, trigger: impl IntoCompatNode) -> Self {
        self.trigger = Some(trigger.into_compat_node());
        self
    }

    pub(crate) fn with_content(mut self, content: impl IntoCompatNode) -> Self {
        self.content = Some(content.into_compat_node());
        self
    }

    pub(crate) fn on_confirm(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.confirm = Some(handler);
        self
    }

    pub(crate) fn on_cancel(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.cancel = Some(handler);
        self
    }
}

impl IntoElement for ConfirmAction {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut node = poodle_render::confirm_action_with_slots(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.trigger,
            self.content,
            poodle_render::ConfirmActionHandlers {
                on_trigger: None,
                on_confirm: self.confirm,
                on_cancel: self.cancel,
            },
        );
        if self.spec.is_open {
            native_alert_dialog_spacing(
                &mut node,
                false,
                self.theme.resolve_space("typography.body.size"),
            );
        }
        native_dialog_element(node)
    }
}

fn native_dialog_element(mut node: poodle_node::Node) -> AnyElement {
    if matches!(node.position, poodle_node::NodePosition::Absolute { .. }) {
        return native_dialog_backdrop(node);
    }

    let Some(backdrop_index) = node
        .children
        .iter()
        .rposition(|child| matches!(child.position, poodle_node::NodePosition::Absolute { .. }))
    else {
        return poodle_gpui_node_backend::to_gpui(&node);
    };
    let backdrop = node.children.remove(backdrop_index);
    let mut host = div();
    for child in &node.children {
        host = host.child(poodle_gpui_node_backend::to_gpui(child));
    }
    host.child(native_dialog_backdrop(backdrop))
        .into_any_element()
}

fn native_dialog_backdrop(mut node: poodle_node::Node) -> AnyElement {
    let Some(panel) = node.children.pop() else {
        return poodle_gpui_node_backend::to_gpui(&node);
    };
    let fill = node
        .style
        .descriptor
        .background
        .map(poodle_gpui_node_backend::color)
        .unwrap_or_else(gpui::transparent_black);
    let dismiss = node.interaction.on_activate;
    let mut backdrop = div()
        .id("poodle-dialog-backdrop")
        .absolute()
        .inset_0()
        .bg(fill)
        .flex()
        .items_center()
        .justify_center()
        .occlude()
        .child(poodle_gpui_node_backend::to_gpui(&panel));
    if let Some(dismiss) = dismiss {
        let click = dismiss.clone();
        backdrop = backdrop
            .on_click(move |_event, _window, cx| {
                click();
                cx.refresh_windows();
            })
            .on_key_down(move |event: &KeyDownEvent, _window, cx| {
                if event.keystroke.key == "escape" {
                    dismiss();
                    cx.refresh_windows();
                }
            });
    }
    backdrop.into_any_element()
}

fn native_alert_dialog_spacing(
    root: &mut poodle_node::Node,
    stack_item_detail: bool,
    body_size: f32,
) {
    // ConfirmAction preserves its custom trigger beside the open dialog. Find
    // the backdrop before applying the preview-only legacy geometry shim.
    let dialog_root = if matches!(root.position, poodle_node::NodePosition::Absolute { .. }) {
        root
    } else {
        let Some(dialog_root) = root.children.last_mut() else {
            return;
        };
        dialog_root
    };
    let Some(panel) = dialog_root.children.first_mut() else {
        return;
    };
    // The outgoing native Dialog used a flat panel stack: header, description,
    // body, actions. Re-form the shared nested header into that paint tree so
    // overlapping preview dialogs retain the same GPUI draw order.
    let description = panel.children.first_mut().and_then(|header| {
        let header_copy = header.children.first_mut()?;
        let description = if header_copy.children.len() > 1 {
            Some(header_copy.children.remove(1))
        } else {
            None
        };
        if let Some(mut title) = header_copy.children.pop() {
            title.style.descriptor.layout.width = poodle_node::LayoutSizing::Grow;
            header.children[0] = title;
        }
        header.style.descriptor.layout.spacing.margin.bottom = 0.0;
        description
    });
    if let Some(description) = description {
        panel.children.insert(1, description);
    }
    panel.style.descriptor.layout.spacing.gap = 6.0;

    if let Some(actions) = panel.children.last_mut() {
        if let Some(button_group) = actions.children.first_mut() {
            button_group.style.descriptor.layout.spacing.gap = 0.0;
        }
    }
    if stack_item_detail {
        if let Some(body) = panel.children.iter_mut().find(|child| {
            child.children.len() == 1
                && matches!(child.children[0].kind, poodle_node::NodeKind::Container)
        }) {
            if let Some(detail) = body.children.first_mut() {
                detail.style.descriptor.layout.direction = poodle_node::LayoutDirection::Column;
                detail.style.descriptor.layout.alignment.cross =
                    poodle_node::CrossAxisAlignment::Start;
                detail.style.descriptor.layout.spacing.gap = 0.0;
                for text in &mut detail.children {
                    text.style.text_size = Some(body_size);
                }
            }
        }
    }
}

pub(crate) struct Menu {
    spec: MenuSpec,
    theme: GpuiThemeProvider,
    trigger: Option<poodle_node::Node>,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl Menu {
    pub(crate) fn from_spec(spec: MenuSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            on_select: None,
        }
    }

    pub(crate) fn with_id(self, _id: impl Into<String>) -> Self {
        self
    }

    pub(crate) fn with_trigger(mut self, trigger: impl IntoCompatNode) -> Self {
        self.trigger = Some(trigger.into_compat_node());
        self
    }

    pub(crate) fn on_select(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_select = Some(handler);
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut wrapper = poodle_node::Node::container();
        wrapper.style.descriptor.layout.direction = poodle_node::LayoutDirection::Column;
        wrapper.style.descriptor.layout.spacing.gap = self.theme.resolve_space("space.inline.xs");
        if let Some(trigger) = self.trigger {
            wrapper = wrapper.child(trigger);
        }
        if self.spec.current_open() {
            wrapper = wrapper.child(poodle_render::menu(&self.spec, &RenderContext::new(&self.theme), self.on_select));
        }
        wrapper
    }
}

impl IntoElement for Menu {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct CommandPalette {
    spec: CommandPaletteSpec,
    theme: GpuiThemeProvider,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_query_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl CommandPalette {
    pub(crate) fn from_spec(spec: CommandPaletteSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_select: None,
            on_query_change: None,
            on_close: None,
        }
    }

    pub(crate) fn with_id(self, _id: impl Into<String>) -> Self {
        self
    }

    pub(crate) fn on_select(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_select = Some(handler);
        self
    }

    pub(crate) fn on_query_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_query_change = Some(handler);
        self
    }

    pub(crate) fn on_close(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_close = Some(handler);
        self
    }
}

impl IntoElement for CommandPalette {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let close = self.on_close.clone();
        let mut node = poodle_render::command_palette_with_handlers(
            &self.spec,
            &RenderContext::new(&self.theme),
            poodle_render::CommandPaletteHandlers {
                select: self.on_select,
                query_change: self.on_query_change,
                close: self.on_close,
            },
        );
        // The outgoing native tier rendered shortcut copy in the inherited
        // sans family. Keep that baseline-local delta out of the shared recipe.
        clear_mono_family(&mut node);
        let Some(modal) = node.children.pop() else {
            return poodle_gpui_node_backend::to_gpui(&node);
        };
        let scrim = node
            .style
            .descriptor
            .background
            .map(poodle_gpui_node_backend::color)
            .unwrap_or_else(gpui::transparent_black);
        let mut backdrop = div()
            .id("poodle-cmd-palette-overlay")
            .absolute()
            .inset_0()
            .bg(scrim)
            .flex()
            .items_center()
            .justify_center()
            .occlude()
            .child(poodle_gpui_node_backend::to_gpui(&modal));
        if let Some(close) = close {
            let click = close.clone();
            backdrop = backdrop
                .on_click(move |_event, _window, cx| {
                    click();
                    cx.refresh_windows();
                })
                .on_key_down(move |event: &KeyDownEvent, _window, cx| {
                    if event.keystroke.key == "escape" {
                        close();
                        cx.refresh_windows();
                    }
                });
        }
        backdrop.into_any_element()
    }
}

fn clear_mono_family(node: &mut poodle_node::Node) {
    if node.style.font_family == Some(poodle_node::FontFamily::Mono) {
        node.style.font_family = None;
    }
    for child in &mut node.children {
        clear_mono_family(child);
    }
}

pub(crate) struct ContextMenu {
    spec: ContextMenuSpec,
    theme: GpuiThemeProvider,
    trigger: Option<AnyElement>,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl ContextMenu {
    pub(crate) fn from_spec(spec: ContextMenuSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            on_select: None,
        }
    }

    pub(crate) fn with_id(self, _id: impl Into<String>) -> Self {
        self
    }

    pub(crate) fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub(crate) fn anchor_point(mut self, point: (i32, i32)) -> Self {
        self.spec.anchor_point = Some(point);
        self
    }

    pub(crate) fn on_select(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_select = Some(handler);
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for ContextMenu {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let menu = self.spec.current_open().then(|| {
            let node = poodle_render::context_menu(&self.spec, &RenderContext::new(&self.theme), self.on_select);
            let menu = poodle_gpui_node_backend::to_gpui(&node);
            if let Some((x, y)) = self.spec.anchor_point {
                div()
                    .absolute()
                    .left(px(x as f32))
                    .top(px(y as f32))
                    .child(menu)
                    .into_any_element()
            } else {
                menu
            }
        });
        if let Some(trigger) = self.trigger {
            let mut host = div().relative().child(trigger);
            if let Some(menu) = menu {
                host = host.child(menu);
            }
            host.into_any_element()
        } else {
            menu.unwrap_or_else(|| div().into_any_element())
        }
    }
}

pub(crate) struct HoverCard {
    spec: HoverCardSpec,
    theme: GpuiThemeProvider,
    trigger: Option<AnyElement>,
    content: Option<poodle_node::Node>,
    on_open_change: Option<OpenChangeHandler>,
}

impl HoverCard {
    pub(crate) fn from_spec(spec: HoverCardSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            content: None,
            on_open_change: None,
        }
    }

    pub(crate) fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub(crate) fn with_content(mut self, content: impl IntoCompatNode) -> Self {
        self.content = Some(content.into_compat_node());
        self
    }

    pub(crate) fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for HoverCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut trigger = div()
            .id((
                "poodle-hover-card-trigger",
                placement_index(self.spec.placement),
            ))
            .child(self.trigger.unwrap_or_else(|| div().into_any_element()));
        if let Some(handler) = self.on_open_change {
            trigger = trigger.on_hover(move |hovered, window, cx| {
                handler(*hovered, window, cx);
            });
        }
        let surface = self.spec.current_open().then(|| {
            poodle_gpui_node_backend::to_gpui(&poodle_render::hover_card(
                &self.spec,
                &RenderContext::new(&self.theme),
                self.content,
            ))
        });
        floating_overlay(trigger.into_any_element(), surface, self.spec.placement)
    }
}

pub(crate) struct Tooltip {
    spec: TooltipSpec,
    theme: GpuiThemeProvider,
    trigger: Option<AnyElement>,
    on_open_change: Option<OpenChangeHandler>,
}

impl Tooltip {
    pub(crate) fn from_spec(spec: TooltipSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            on_open_change: None,
        }
    }

    pub(crate) fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub(crate) fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for Tooltip {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut trigger = div()
            .id((
                "poodle-tooltip-trigger",
                placement_index(self.spec.placement),
            ))
            .child(self.trigger.unwrap_or_else(|| div().into_any_element()));
        if let Some(handler) = self.on_open_change {
            trigger = trigger.on_hover(move |hovered, window, cx| {
                handler(*hovered, window, cx);
            });
        }
        let bubble = (self.spec.current_open() && self.spec.has_content()).then(|| {
            poodle_gpui_node_backend::to_gpui(&poodle_render::tooltip(&self.spec, &RenderContext::new(&self.theme)))
        });
        floating_overlay(trigger.into_any_element(), bubble, self.spec.placement)
    }
}

pub(crate) struct Popover {
    spec: PopoverSpec,
    theme: GpuiThemeProvider,
    trigger: Option<poodle_node::Node>,
    content: Option<poodle_node::Node>,
    /// Context-free toggle handler: the shared composition's activation is a
    /// node handler without a window, so the preview delivers through its
    /// node-event queue instead.
    on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// Stable per-instance scope for backend state (focus, bounds, layers).
    /// Distinct instances on one page must not collide.
    instance_id: Option<String>,
}

impl Popover {
    pub(crate) fn from_spec(spec: PopoverSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            content: None,
            on_open_change: None,
            instance_id: None,
        }
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.instance_id = Some(instance_id.into());
        self
    }

    pub(crate) fn with_trigger(mut self, trigger: impl IntoCompatNode) -> Self {
        self.trigger = Some(trigger.into_compat_node());
        self
    }

    pub(crate) fn with_content(mut self, content: impl IntoCompatNode) -> Self {
        self.content = Some(content.into_compat_node());
        self
    }

    pub(crate) fn on_open_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_open_change = Some(handler);
        self
    }
}

impl IntoElement for Popover {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let instance_id = self.instance_id.clone();
        let spec = self.spec.clone();
        let trigger = self.trigger.clone();
        let content = self.content.clone();
        let on_open_change = self.on_open_change.clone();
        let theme = self.theme.clone();
        let surface_id = format!(
            "{}:popover-surface",
            instance_id.as_deref().unwrap_or_default()
        );
        let trigger_id = format!(
            "{}:popover-trigger",
            instance_id.as_deref().unwrap_or_default()
        );

        // The production host runs the same machine the web shells and the
        // headless adapter run: the trigger and the dismiss routes emit
        // `openChange` through the preview's node-event queue, and the focus
        // effects (entry and restore) are queued for the overlay host's
        // paint-time focus application.
        let context = poodle_headless::popover::PopoverContext {
            disabled: spec.disabled,
            dismiss_on_outside_interact: spec.dismiss_on_outside_interact,
            initial_focus: match spec.initial_focus {
                poodle_specs::PopoverInitialFocus::Content => {
                    poodle_headless::popover::PopoverInitialFocus::Content
                }
                poodle_specs::PopoverInitialFocus::None => {
                    poodle_headless::popover::PopoverInitialFocus::None
                }
                poodle_specs::PopoverInitialFocus::FirstFocusable => {
                    poodle_headless::popover::PopoverInitialFocus::FirstFocusable
                }
            },
        };
        let run_machine = {
            use poodle_headless::popover::{popover_transition, PopoverEffect, PopoverState};
            let context = context;
            let handler = on_open_change.clone();
            let content = content.clone();
            let surface_id = surface_id.clone();
            let trigger_id = trigger_id.clone();
            let current_open = spec.current_open();
            move |event: poodle_headless::popover::PopoverEvent| {
                let (_, effects) = popover_transition(
                    if current_open {
                        PopoverState::Open
                    } else {
                        PopoverState::Closed
                    },
                    context,
                    event,
                );
                for effect in effects {
                    match effect {
                        PopoverEffect::EmitOpenChange { open } => {
                            if let Some(handler) = &handler {
                                handler(open);
                            }
                        }
                        PopoverEffect::FocusOnOpen { strategy } => match strategy {
                            poodle_headless::popover::PopoverInitialFocus::Content => {
                                poodle_gpui_node_backend::request_focus(&surface_id);
                            }
                            poodle_headless::popover::PopoverInitialFocus::FirstFocusable => {
                                let target = content
                                    .as_ref()
                                    .and_then(|content| content.find(&|n| n.interaction.focusable));
                                if let Some(target) = target {
                                    let id = target
                                        .runtime_id
                                        .clone()
                                        .or_else(|| target.id.clone())
                                        .unwrap_or_default();
                                    if !id.is_empty() {
                                        poodle_gpui_node_backend::request_focus(&id);
                                    }
                                }
                            }
                            poodle_headless::popover::PopoverInitialFocus::None => {}
                        },
                        PopoverEffect::RestoreTriggerFocus => {
                            poodle_gpui_node_backend::request_focus(&trigger_id);
                        }
                    }
                }
            }
        };

        // The shared poodle-render composition owns trigger, surface,
        // placement, accessibility metadata, and the layer/dismiss intent;
        // the specimen host supplies the trigger/content nodes, the toggle
        // handler, and the dismissal route (Escape/outside through the
        // overlay host's layer registry).
        let on_activate = on_open_change.as_ref().map(|_| {
            let run = run_machine.clone();
            Arc::new(move || run(poodle_headless::popover::PopoverEvent::Toggle))
                as Arc<dyn Fn() + Send + Sync>
        });
        let on_dismiss = on_open_change.as_ref().map(|_| {
            let run = run_machine.clone();
            Arc::new(move |reason| {
                run(match reason {
                    poodle_node::DismissReason::Escape => {
                        poodle_headless::popover::PopoverEvent::Escape
                    }
                    poodle_node::DismissReason::Outside => {
                        poodle_headless::popover::PopoverEvent::OutsideInteract
                    }
                })
            }) as poodle_node::DismissHandler
        });
        let node = poodle_render::popover(
            &spec,
            &RenderContext::new(&theme),
            &poodle_render::PopoverHandlers {
                on_activate,
                on_dismiss,
                instance_id,
            },
            trigger,
            content,
        );
        poodle_gpui_node_backend::to_gpui(&node)
    }
}

fn floating_overlay(
    trigger: AnyElement,
    surface: Option<AnyElement>,
    placement: OverlayPlacement,
) -> AnyElement {
    let anchor = px(36.0);
    let gap = px(4.0);
    let mut wrapper = div().relative().flex_shrink_0().child(trigger);
    if let Some(surface) = surface {
        let positioned = match placement {
            OverlayPlacement::Bottom | OverlayPlacement::BottomStart => div()
                .absolute()
                .top(anchor + gap)
                .left(px(0.0))
                .child(surface),
            OverlayPlacement::BottomEnd => div()
                .absolute()
                .top(anchor + gap)
                .right(px(0.0))
                .child(surface),
            OverlayPlacement::Top | OverlayPlacement::TopStart => div()
                .absolute()
                .bottom(anchor + gap)
                .left(px(0.0))
                .child(surface),
            OverlayPlacement::TopEnd => div()
                .absolute()
                .bottom(anchor + gap)
                .right(px(0.0))
                .child(surface),
            OverlayPlacement::Right | OverlayPlacement::RightStart => div()
                .absolute()
                .left(anchor + gap)
                .top(px(0.0))
                .child(surface),
            OverlayPlacement::RightEnd => div()
                .absolute()
                .left(anchor + gap)
                .bottom(px(0.0))
                .child(surface),
            OverlayPlacement::Left | OverlayPlacement::LeftStart => div()
                .absolute()
                .right(anchor + gap)
                .top(px(0.0))
                .child(surface),
            OverlayPlacement::LeftEnd => div()
                .absolute()
                .right(anchor + gap)
                .bottom(px(0.0))
                .child(surface),
        };
        wrapper = wrapper.child(positioned);
    }
    wrapper.into_any_element()
}

fn placement_index(placement: OverlayPlacement) -> usize {
    match placement {
        OverlayPlacement::Top => 0,
        OverlayPlacement::TopStart => 1,
        OverlayPlacement::TopEnd => 2,
        OverlayPlacement::Right => 3,
        OverlayPlacement::RightStart => 4,
        OverlayPlacement::RightEnd => 5,
        OverlayPlacement::Bottom => 6,
        OverlayPlacement::BottomStart => 7,
        OverlayPlacement::BottomEnd => 8,
        OverlayPlacement::Left => 9,
        OverlayPlacement::LeftStart => 10,
        OverlayPlacement::LeftEnd => 11,
    }
}

pub(crate) struct DetailItem {
    spec: DetailItemSpec,
    theme: GpuiThemeProvider,
    value_content: Option<poodle_node::Node>,
    action: Option<poodle_node::Node>,
}

impl DetailItem {
    pub(crate) fn from_spec(spec: DetailItemSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            value_content: None,
            action: None,
        }
    }

    pub(crate) fn with_value_content(mut self, content: impl IntoCompatNode) -> Self {
        self.value_content = Some(content.into_compat_node());
        self
    }

    pub(crate) fn with_action(mut self, action: impl IntoCompatNode) -> Self {
        self.action = Some(action.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::detail_item_with_slots(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.value_content,
            self.action,
        )
    }
}

impl IntoCompatNode for DetailItem {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for DetailItem {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct DetailSection {
    spec: DetailSectionSpec,
    theme: GpuiThemeProvider,
    body: Option<poodle_node::Node>,
    actions: Option<poodle_node::Node>,
}

impl DetailSection {
    pub(crate) fn from_spec(spec: DetailSectionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            body: None,
            actions: None,
        }
    }

    pub(crate) fn with_body(mut self, body: impl IntoCompatNode) -> Self {
        self.body = Some(body.into_compat_node());
        self
    }

    pub(crate) fn with_actions(mut self, actions: impl IntoCompatNode) -> Self {
        self.actions = Some(actions.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::detail_section(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.body.into_iter().collect(),
            self.actions,
        )
    }
}

impl IntoCompatNode for DetailSection {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for DetailSection {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct DetailSectionGroup {
    spec: DetailSectionGroupSpec,
    theme: GpuiThemeProvider,
    children: Vec<poodle_node::Node>,
}

impl DetailSectionGroup {
    pub(crate) fn from_spec(spec: DetailSectionGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub(crate) fn child(mut self, child: impl IntoCompatNode) -> Self {
        self.children.push(child.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::detail_section_group(&self.spec, &RenderContext::new(&self.theme), self.children)
    }
}

impl IntoElement for DetailSectionGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct DetailShell {
    spec: DetailShellSpec,
    theme: GpuiThemeProvider,
    header: Option<poodle_node::Node>,
    content: Option<poodle_node::Node>,
    state_content: Option<poodle_node::Node>,
}

impl DetailShell {
    pub(crate) fn from_spec(spec: DetailShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            header: None,
            content: None,
            state_content: None,
        }
    }

    pub(crate) fn with_header(mut self, header: impl IntoCompatNode) -> Self {
        self.header = Some(header.into_compat_node());
        self
    }

    pub(crate) fn with_content(mut self, content: impl IntoCompatNode) -> Self {
        self.content = Some(content.into_compat_node());
        self
    }

    #[expect(
        dead_code,
        reason = "retained for parity with the state-content compatibility contract"
    )]
    pub(crate) fn with_state_content(mut self, content: impl IntoCompatNode) -> Self {
        self.state_content = Some(content.into_compat_node());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let id = self
            .spec
            .title
            .clone()
            .unwrap_or_else(|| format!("{:?}", self.spec.state));
        let mut node = poodle_render::detail_shell(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.header,
            self.content,
            self.state_content,
        );
        // Repeated stateful ids alias GPUI's element cache when several shell
        // specimens share one page. Keep the old prefix but disambiguate the
        // preview instances.
        node.id = Some(format!("poodle-detail-shell-{id}"));
        node
    }
}

impl IntoElement for DetailShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct Checkbox {
    spec: CheckboxSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

impl Checkbox {
    pub(crate) fn from_spec(spec: CheckboxSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let id = self.id_suffix.unwrap_or_else(|| {
            self.spec
                .label
                .clone()
                .unwrap_or_else(|| "anon".to_string())
        });
        let mut node = poodle_render::checkbox(&self.spec, &RenderContext::new(&self.theme), self.on_change);
        node.id = Some(format!("poodle-checkbox-{id}"));
        node
    }
}

impl IntoCompatNode for Checkbox {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Checkbox {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

pub(crate) struct Switch {
    spec: SwitchSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

impl Switch {
    pub(crate) fn from_spec(spec: SwitchSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }
}

impl IntoElement for Switch {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let id = self.id_suffix.unwrap_or_else(|| {
            self.spec
                .label
                .clone()
                .unwrap_or_else(|| "anon".to_string())
        });
        let mut node = poodle_render::switch(&self.spec, &RenderContext::new(&self.theme), self.on_change);
        node.id = Some(format!("poodle-switch-{id}"));
        poodle_gpui_node_backend::to_gpui(&node)
    }
}

pub(crate) struct TriStateSwitch {
    spec: TriStateSwitchSpec,
    theme: GpuiThemeProvider,
    id: Option<String>,
    handlers: poodle_render::TriStateSwitchHandlers,
}

impl TriStateSwitch {
    pub(crate) fn from_spec(
        spec: TriStateSwitchSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        let instance_id = instance_id.into();
        Self {
            spec,
            theme: theme.clone(),
            id: Some(instance_id.clone()),
            handlers: poodle_render::TriStateSwitchHandlers::new(instance_id),
        }
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(TriStateValue) + Send + Sync>) -> Self {
        self.handlers.on_value_change = Some(handler);
        self
    }
}

impl IntoElement for TriStateSwitch {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut node = poodle_render::tri_state_switch(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        );
        if let Some(id) = self.id {
            node.id = Some(id);
        }
        poodle_gpui_node_backend::to_gpui(&node)
    }
}

pub(crate) struct Slider {
    spec: SliderSpec,
    theme: GpuiThemeProvider,
    id: Option<String>,
    on_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
}

impl Slider {
    pub(crate) fn from_spec(spec: SliderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id: None,
            on_change: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(f64) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }

    pub(crate) fn aria_label(mut self, label: impl Into<String>) -> Self {
        self.spec.aria_label = Some(label.into());
        self
    }
}

impl IntoElement for Slider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let handlers = poodle_render::SliderHandlers {
            on_change: self.on_change,
            on_value_commit: None,
        };
        let mut node = poodle_render::slider(&self.spec, &RenderContext::new(&self.theme), &handlers);
        if let Some(id) = self.id {
            fn stamp(node: &mut poodle_node::Node, id: &str) {
                if node.a11y.role == Some(poodle_node::NodeRole::Slider) {
                    node.id = Some(id.to_owned());
                    return;
                }
                for child in &mut node.children {
                    stamp(child, id);
                }
            }
            stamp(&mut node, &id);
        }
        poodle_gpui_node_backend::to_gpui(&node)
    }
}

pub(crate) struct RangeSlider {
    spec: RangeSliderSpec,
    theme: GpuiThemeProvider,
    id: Option<String>,
    on_change: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>,
}

impl RangeSlider {
    pub(crate) fn from_spec(spec: RangeSliderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id: None,
            on_change: None,
        }
    }

    pub(crate) fn on_change(
        mut self,
        interaction_key: impl Into<String>,
        handler: Arc<dyn Fn(f64, f64) + Send + Sync>,
    ) -> Self {
        self.id = Some(format!("poodle-range-slider-{}", interaction_key.into()));
        self.on_change = Some(handler);
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for RangeSlider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let handlers = poodle_render::RangeSliderHandlers {
            on_change: self.on_change,
            on_value_commit: None,
        };
        let mut node = poodle_render::range_slider(&self.spec, &RenderContext::new(&self.theme), handlers);
        if let Some(id) = self.id {
            node.id = Some(id);
        }
        poodle_gpui_node_backend::to_gpui(&node)
    }
}

pub(crate) struct RadioGroup {
    spec: RadioGroupSpec,
    theme: GpuiThemeProvider,
    id: Option<String>,
    handlers: poodle_render::RadioGroupHandlers,
}

impl RadioGroup {
    pub(crate) fn from_spec(
        spec: RadioGroupSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        let instance_id = instance_id.into();
        Self {
            spec,
            theme: theme.clone(),
            id: Some(instance_id.clone()),
            handlers: poodle_render::RadioGroupHandlers::new(instance_id),
        }
    }

    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_change = Some(handler);
        self
    }
}

impl IntoElement for RadioGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut node = poodle_render::radio_group(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        );
        if let Some(id) = self.id {
            node.id = Some(id);
        }
        poodle_gpui_node_backend::to_gpui(&node)
    }
}

pub(crate) struct Radio {
    spec: RadioSpec,
    theme: GpuiThemeProvider,
    id: Option<String>,
    on_checked_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

impl Radio {
    pub(crate) fn from_spec(spec: RadioSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id: None,
            on_checked_change: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub(crate) fn on_checked_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_checked_change = Some(handler);
        self
    }
}

impl IntoElement for Radio {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut node = poodle_render::radio(&self.spec, &RenderContext::new(&self.theme), self.on_checked_change);
        if let Some(id) = self.id {
            node.id = Some(id);
        }
        poodle_gpui_node_backend::to_gpui(&node)
    }
}

pub(crate) struct Pagination {
    spec: PaginationSpec,
    theme: GpuiThemeProvider,
    instance_id: String,
    on_page_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    limit_open: bool,
    on_limit_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    on_page_size_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
}

impl Pagination {
    pub(crate) fn from_spec(
        spec: PaginationSpec,
        theme: &GpuiThemeProvider,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            instance_id: instance_id.into(),
            on_page_change: None,
            limit_open: false,
            on_limit_open_change: None,
            on_page_size_change: None,
        }
    }

    pub(crate) fn on_page_change(mut self, handler: Arc<dyn Fn(usize) + Send + Sync>) -> Self {
        self.on_page_change = Some(handler);
        self
    }

    pub(crate) fn limit_selector_open(mut self, open: bool) -> Self {
        self.limit_open = open;
        self
    }

    pub(crate) fn on_limit_open_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_limit_open_change = Some(handler);
        self
    }

    pub(crate) fn on_page_size_change(mut self, handler: Arc<dyn Fn(usize) + Send + Sync>) -> Self {
        self.on_page_size_change = Some(handler);
        self
    }

    pub(crate) fn size(mut self, size: ControlSize) -> Self {
        self.spec.size = Some(size);
        self
    }

    pub(crate) fn with_density(mut self, density: ControlDensity) -> Self {
        self.spec.density = Some(density);
        self
    }
}

impl IntoElement for Pagination {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let handlers = poodle_render::PaginationHandlers {
            instance_id: self.instance_id,
            page_change: self.on_page_change,
            limit_open: self.limit_open,
            limit_open_change: self.on_limit_open_change,
            page_size_change: self.on_page_size_change,
        };
        poodle_gpui_node_backend::to_gpui(&poodle_render::pagination_with_handlers(
            &self.spec,
            &RenderContext::new(&self.theme),
            &handlers,
        ))
    }
}

pub(crate) struct Stepper {
    spec: StepperSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::StepperHandlers,
}

impl Stepper {
    pub(crate) fn from_spec(spec: StepperSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::StepperHandlers::default(),
        }
    }

    /// Selection. The trigger navigates and nothing else — see `stepper.md`
    /// §2 for why this is not the same control as re-run.
    pub(crate) fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_change = Some(handler);
        self
    }

    /// Re-run of one completed step. Separate from `on_change` because a
    /// re-run spends whatever the step costs and must never also select.
    pub(crate) fn on_rerun(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_rerun = Some(handler);
        self
    }

    pub(crate) fn on_collapsed_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.handlers.on_collapsed_change = Some(handler);
        self
    }
}

impl IntoElement for Stepper {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&poodle_render::stepper(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.handlers,
        ))
    }
}

pub(crate) struct IconButton {
    spec: IconButtonSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    on_pressed_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

impl IconButton {
    pub(crate) fn from_spec(spec: IconButtonSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_click: None,
            on_pressed_change: None,
        }
    }

    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id_suffix = Some(id.into());
        self
    }

    pub(crate) fn on_click(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_click = Some(handler);
        self
    }

    pub(crate) fn on_pressed_change(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_pressed_change = Some(handler);
        self
    }

    pub(crate) fn into_node_with(self, ctx: &RenderContext<'_>) -> poodle_node::Node {
        let id = self
            .id_suffix
            .unwrap_or_else(|| self.spec.icon.clone().unwrap_or_default());
        let mut node = poodle_render::icon_button_with_handlers(
            &self.spec,
            ctx,
            poodle_render::IconButtonHandlers {
                on_click: self.on_click,
                on_pressed_change: self.on_pressed_change,
            },
        );
        node.id = Some(format!("poodle-icon-btn-{id}"));
        node
    }

    fn into_node(self) -> poodle_node::Node {
        let theme = self.theme.clone();
        self.into_node_with(&RenderContext::new(&theme))
    }

    /// Deferred construction for a scoped slot (architecture 010): the
    /// boundary invokes the builder with its internal presentation scope.
    pub(crate) fn into_slot(self) -> SlotBuilder<'static> {
        std::boxed::Box::new(move |ctx| self.into_node_with(ctx))
    }
}

impl IntoCompatNode for IconButton {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for IconButton {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

// ── Media and relative-time leaves (g12.019 Wave 41) ────────────────────────
//
// All three are plain `Spec + Theme -> Node` recipes with no host events. The
// native visual gate skips their slugs as non-deterministic (media playhead,
// wall-clock copy), so their proof is the shared render recipe plus the
// preview build, not a pixel capture.

pub(crate) struct AudioPlayer;

impl AudioPlayer {
    pub(crate) fn from_spec(spec: AudioPlayerSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::audio_player(&spec, &RenderContext::new(theme)))
    }
}

pub(crate) struct VideoPlayer;

impl VideoPlayer {
    pub(crate) fn from_spec(spec: VideoPlayerSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::video_player(&spec, &RenderContext::new(theme)))
    }
}

pub(crate) struct TimeAgo;

impl TimeAgo {
    pub(crate) fn from_spec(spec: TimeAgoSpec, theme: &GpuiThemeProvider) -> AnyElement {
        poodle_gpui_node_backend::to_gpui(&poodle_render::time_ago(&spec, &RenderContext::new(theme)))
    }
}

// ── SplitView (g12.019 Wave 42) ────────────────────────────────────────────

pub(crate) struct SplitView {
    spec: SplitViewSpec,
    theme: GpuiThemeProvider,
    primary: Option<poodle_node::Node>,
    secondary: Option<poodle_node::Node>,
    extent_px: f32,
    on_ratio_change: Option<Arc<dyn Fn(f32) + Send + Sync>>,
}

impl SplitView {
    /// Fallback split extent when a caller does not declare one, matching the
    /// preview's usual content-column width.
    const DEFAULT_EXTENT_PX: f32 = 640.0;

    pub(crate) fn from_spec(spec: SplitViewSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            primary: None,
            secondary: None,
            extent_px: Self::DEFAULT_EXTENT_PX,
            on_ratio_change: None,
        }
    }

    pub(crate) fn with_primary(mut self, child: impl IntoCompatNode) -> Self {
        self.primary = Some(child.into_compat_node());
        self
    }

    pub(crate) fn with_secondary(mut self, child: impl IntoCompatNode) -> Self {
        self.secondary = Some(child.into_compat_node());
        self
    }

    /// Declare the split's axis extent in logical px.
    ///
    /// The node vocabulary carries per-frame drag deltas only — absolute
    /// positions depend on layout, which a component never sees — so turning
    /// the divider's gesture back into a ratio needs the axis length from the
    /// caller. This is the same trade `poodle_render::slider` makes with its
    /// fixed track basis; the old tier read it from the drag event's container
    /// bounds instead.
    pub(crate) fn with_extent_px(mut self, extent: f32) -> Self {
        self.extent_px = extent.max(1.0);
        self
    }

    pub(crate) fn on_ratio_change(mut self, handler: Arc<dyn Fn(f32) + Send + Sync>) -> Self {
        self.on_ratio_change = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let mut handlers = poodle_render::SplitViewHandlers::default();

        if let Some(on_ratio) = self.on_ratio_change {
            if !self.spec.is_disabled {
                let is_horizontal = self.spec.orientation == SplitOrientation::Horizontal;
                let extent = self.extent_px;
                // The gesture streams deltas, so the running ratio lives across
                // frames next to the handler rather than in specimen state.
                let live = Arc::new(std::sync::atomic::AtomicU32::new(
                    self.spec.current_ratio().to_bits(),
                ));
                let seed = self.spec.current_ratio();
                handlers.on_resize = Some(Arc::new(move |phase, delta| {
                    use std::sync::atomic::Ordering;
                    match phase {
                        poodle_render::ResizePhase::Start => {
                            live.store(seed.to_bits(), Ordering::SeqCst);
                        }
                        poodle_render::ResizePhase::Move => {
                            // `delta` is already the axis delta the orientation
                            // selects, so both orientations divide by the same
                            // declared extent.
                            let _ = is_horizontal;
                            let current = f32::from_bits(live.load(Ordering::SeqCst));
                            let next = (current + delta / extent).clamp(0.05, 0.95);
                            live.store(next.to_bits(), Ordering::SeqCst);
                            on_ratio(next);
                        }
                        poodle_render::ResizePhase::End => {}
                    }
                }));
            }
        }

        poodle_render::split_view(
            &self.spec,
            &RenderContext::new(&self.theme),
            self.primary,
            self.secondary,
            handlers,
        )
    }
}

impl IntoCompatNode for SplitView {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for SplitView {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

// ── DockRegion (g12.019 Wave 42) ───────────────────────────────────────────

pub(crate) struct DockRegion {
    spec: DockRegionSpec,
    theme: GpuiThemeProvider,
    content: Option<poodle_node::Node>,
    on_tab_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    on_collapse_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    instance_id: Option<String>,
}

impl DockRegion {
    pub(crate) fn from_spec(spec: DockRegionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            on_tab_change: None,
            on_collapse_toggle: None,
            instance_id: None,
        }
    }

    pub(crate) fn with_content(mut self, content: impl IntoCompatNode) -> Self {
        self.content = Some(content.into_compat_node());
        self
    }

    pub(crate) fn on_tab_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_tab_change = Some(handler);
        self
    }

    pub(crate) fn on_collapse_toggle(mut self, handler: Arc<dyn Fn(bool) + Send + Sync>) -> Self {
        self.on_collapse_toggle = Some(handler);
        self
    }

    pub(crate) fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.instance_id = Some(instance_id.into());
        self
    }

    fn into_node(self) -> poodle_node::Node {
        let handlers = poodle_render::DockRegionHandlers {
            on_tab_change: self.on_tab_change,
            on_collapse_toggle: self.on_collapse_toggle,
            instance_id: self.instance_id,
        };
        poodle_render::dock_region(&self.spec, &RenderContext::new(&self.theme), self.content, handlers)
    }
}

impl IntoCompatNode for DockRegion {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for DockRegion {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

// ── BlockEditor (g12.019 Wave 44) ──────────────────────────────────────────

pub(crate) struct BlockEditor {
    spec: BlockEditorSpec,
    theme: GpuiThemeProvider,
    children: Vec<SlotBuilder<'static>>,
}

impl BlockEditor {
    pub(crate) fn from_spec(spec: BlockEditorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    /// Caller-owned block bodies, for consumers that own the block vocabulary
    /// instead of driving it through `spec.blocks`.
    pub(crate) fn with_child(
        mut self,
        child: impl FnOnce(&RenderContext<'_>) -> poodle_node::Node + 'static,
    ) -> Self {
        self.children.push(std::boxed::Box::new(child));
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::block_editor_with_children(&self.spec, &RenderContext::new(&self.theme), self.children)
    }
}

impl IntoCompatNode for BlockEditor {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for BlockEditor {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

// ── LogList (g12.019 Wave 45) ──────────────────────────────────────────────

pub(crate) struct LogList {
    spec: LogListSpec,
    theme: GpuiThemeProvider,
    on_clear_filters: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl LogList {
    pub(crate) fn from_spec(spec: LogListSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_clear_filters: None,
        }
    }

    #[expect(
        dead_code,
        reason = "retained for hosts that wire the optional clear-filters event"
    )]
    pub(crate) fn on_clear_filters(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_clear_filters = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::log_list(&self.spec, &RenderContext::new(&self.theme), self.on_clear_filters)
    }
}

impl IntoCompatNode for LogList {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for LogList {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}

// ── Tree (g12.019 Wave 46) ─────────────────────────────────────────────────
//
// Tree is the only specimen that needed the interaction vocabulary widened.
// The four channels it added — modifier-aware activation, secondary
// activation with an anchor, navigation keys, and payload/zone drag — now
// live on `poodle_node::Interaction`, so this bridge only names handlers.

pub(crate) struct Tree {
    spec: TreeSpec,
    theme: GpuiThemeProvider,
    handlers: poodle_render::TreeHandlers,
}

impl Tree {
    pub(crate) fn from_spec(spec: TreeSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            handlers: poodle_render::TreeHandlers::default(),
        }
    }

    pub(crate) fn focused_value(mut self, value: impl Into<String>) -> Self {
        self.spec.focused_value = Some(value.into());
        self
    }

    pub(crate) fn on_toggle_expand(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_toggle_expand = Some(handler);
        self
    }

    pub(crate) fn on_check(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.handlers.on_check = Some(handler);
        self
    }

    pub(crate) fn on_select_modified(
        mut self,
        handler: Arc<dyn Fn(&str, poodle_node::NodeModifiers) + Send + Sync>,
    ) -> Self {
        self.handlers.on_select_modified = Some(handler);
        self
    }

    pub(crate) fn on_context_menu(
        mut self,
        handler: Arc<dyn Fn(&str, poodle_node::NodePoint) + Send + Sync>,
    ) -> Self {
        self.handlers.on_context_menu = Some(handler);
        self
    }

    pub(crate) fn on_key(
        mut self,
        handler: Arc<dyn Fn(&str, poodle_node::NodeKey, poodle_node::NodeModifiers) + Send + Sync>,
    ) -> Self {
        self.handlers.on_key = Some(handler);
        self
    }

    pub(crate) fn on_drag_over(
        mut self,
        handler: Arc<dyn Fn(&str, &str, poodle_node::DropEdge) + Send + Sync>,
    ) -> Self {
        self.handlers.on_drag_over = Some(handler);
        self
    }

    pub(crate) fn on_reorder(
        mut self,
        handler: Arc<dyn Fn(&str, &str, poodle_node::DropEdge) + Send + Sync>,
    ) -> Self {
        self.handlers.on_reorder = Some(handler);
        self
    }

    fn into_node(self) -> poodle_node::Node {
        poodle_render::tree(&self.spec, &RenderContext::new(&self.theme), self.handlers)
    }
}

impl IntoCompatNode for Tree {
    fn into_compat_node(self) -> poodle_node::Node {
        self.into_node()
    }
}

impl IntoElement for Tree {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        poodle_gpui_node_backend::to_gpui(&self.into_node())
    }
}
