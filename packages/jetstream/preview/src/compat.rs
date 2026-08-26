//! Compat shim: the old tier's `js_*` render-only signatures, implemented on
//! `poodle-render` (Spec + Theme -> Node). Specimens keep their call shapes;
//! chrome and slots are `crate::nel::El`; the shell converts once via
//! `jetstream_poodle::to_js_el`.
#![allow(clippy::too_many_arguments)]

use poodle_jetstream::JetstreamThemeProvider;
use poodle_node::Node;
use poodle_render as pr;
use poodle_specs::*;

pub use crate::nel::*;
pub use pr::presentation::{control_height_rem, size_font_rem};

pub fn js_spinner(spec: &SpinnerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::spinner(spec, &pr::RenderContext::new(theme)))
}

pub fn js_accordion(spec: &AccordionSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::accordion(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_action_discovery_panel(
    spec: &ActionDiscoveryPanelSpec,
    theme: &JetstreamThemeProvider,
) -> El {
    El(pr::action_discovery_panel(
        spec,
        &pr::RenderContext::new(theme),
        pr::ActionDiscoveryPanelHandlers::default(),
    ))
}

pub fn js_message_center(spec: &MessageCenterSpec, theme: &JetstreamThemeProvider) -> El {
    let handlers = pr::MessageCenterHandlers {
        on_item_select: Some(std::sync::Arc::new(|_| {})),
        on_read_change: Some(std::sync::Arc::new(|_, _| {})),
        on_remove: Some(std::sync::Arc::new(|_| {})),
        on_mark_all_read: Some(std::sync::Arc::new(|| {})),
        ..Default::default()
    };
    El(pr::message_center(spec, &pr::RenderContext::new(theme), handlers))
}

pub fn js_agent_chat_input(
    spec: &AgentChatInputSpec,
    theme: &JetstreamThemeProvider,
    question_children: Vec<El>,
    plan_children: Vec<El>,
    toolbar_children: Vec<El>,
    footer_children: Vec<El>,
) -> El {
    let question_children: Vec<Node> = question_children.into_iter().map(Node::from).collect();
    let plan_children: Vec<Node> = plan_children.into_iter().map(Node::from).collect();
    let toolbar_children: Vec<Node> = toolbar_children.into_iter().map(Node::from).collect();
    let footer_children: Vec<Node> = footer_children.into_iter().map(Node::from).collect();
    El(pr::agent_chat_input(
        spec,
        &pr::RenderContext::new(theme),
        question_children,
        plan_children,
        toolbar_children,
        footer_children,
        pr::AgentChatInputHandlers::default(),
    ))
}

pub fn js_agent_plan(spec: &AgentPlanSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::agent_plan(
        spec,
        &pr::RenderContext::new(theme),
        pr::AgentPlanHandlers::default(),
    ))
}

pub fn js_agent_question(spec: &AgentQuestionSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::agent_question(
        spec,
        &pr::RenderContext::new(theme),
        pr::AgentQuestionHandlers::default(),
    ))
}

pub fn js_agent_transcript(spec: &AgentTranscriptSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::agent_transcript(
        spec,
        &pr::RenderContext::new(theme),
        pr::AgentTranscriptHandlers::default(),
    ))
}

pub fn js_alert_dialog_working(
    spec: &AlertDialogSpec,
    theme: &JetstreamThemeProvider,
    working: bool,
    working_label: &str,
) -> El {
    El(pr::alert_dialog(
        spec,
        &pr::RenderContext::new(theme),
        working,
        working_label,
        pr::AlertDialogHandlers::default(),
    ))
}

pub fn js_app_header(spec: &AppHeaderSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::app_header(spec, &pr::RenderContext::new(theme), None, None, None, None))
}

pub fn js_app_header_with_slots(
    spec: &AppHeaderSpec,
    theme: &JetstreamThemeProvider,
    identity: Option<El>,
    center: Option<El>,
    actions: Option<El>,
    utility: Option<El>,
) -> El {
    // Compile-only jetstream adaptation: this preview's `El` chrome is built
    // eagerly, so the already-built node is wrapped in the slot builder the
    // shared renderer now requires. The internal scope cannot reach it until
    // this preview defers construction; no parity claim is made.
    let identity = identity.map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>);
    let center = center.map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>);
    let actions = actions.map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>);
    let utility = utility.map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>);
    El(pr::app_header(
        spec,
        &pr::RenderContext::new(theme),
        identity,
        center,
        actions,
        utility,
    ))
}

pub fn js_audio_player(spec: &AudioPlayerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::audio_player(spec, &pr::RenderContext::new(theme)))
}

pub fn js_avatar(spec: &AvatarSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::avatar(spec, &pr::RenderContext::new(theme)))
}

pub fn js_badge(spec: &BadgeSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::badge(spec, &pr::RenderContext::new(theme)))
}

pub fn js_banner(spec: &BannerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::banner(spec, &pr::RenderContext::new(theme)))
}

pub fn js_block_editor(spec: &BlockEditorSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::block_editor(spec, &pr::RenderContext::new(theme)))
}

pub fn js_breadcrumbs(spec: &BreadcrumbsSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::breadcrumbs(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_bulk_action_bar(spec: &BulkActionBarSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::bulk_action_bar(
        spec,
        &pr::RenderContext::new(theme),
        pr::BulkActionBarHandlers::default(),
    ))
}

pub fn js_button(spec: &ButtonSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::button(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_calendar(spec: &CalendarSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::calendar(spec, &pr::RenderContext::new(theme), pr::CalendarHandlers::default()))
}

pub fn js_callout(spec: &CallOutSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::callout(spec, &pr::RenderContext::new(theme), pr::CalloutHandlers::default()))
}

pub fn js_card(spec: &CardSpec, theme: &JetstreamThemeProvider, children: Vec<El>) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::card(spec, &pr::RenderContext::new(theme), children))
}

pub fn js_card_radio_group(spec: &CardRadioGroupSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::card_radio_group(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_card_toggle_group(spec: &CardToggleGroupSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::card_toggle_group(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_checkbox(spec: &CheckboxSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::checkbox(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_code(spec: &CodeSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::code(spec, &pr::RenderContext::new(theme)))
}

pub fn js_code_input(spec: &CodeInputSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::code_input(spec, &pr::RenderContext::new(theme)))
}

pub fn js_collapse_toggle(spec: &CollapseToggleSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::collapse_toggle(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_collapsible(
    spec: &CollapsibleSpec,
    theme: &JetstreamThemeProvider,
    content: Option<El>,
) -> El {
    let content: Option<Node> = content.map(Node::from);
    El(pr::collapsible(spec, &pr::RenderContext::new(theme), content, None))
}

pub fn js_color_picker(
    spec: &ColorPickerSpec,
    theme: &JetstreamThemeProvider,
    instance_id: &str,
) -> El {
    El(pr::color_picker(
        spec,
        &pr::RenderContext::new(theme),
        instance_id,
        pr::ColorPickerHandlers::default(),
    ))
}

pub fn js_command_palette(spec: &CommandPaletteSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::command_palette(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_confirm_action(spec: &ConfirmActionSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::confirm_action(
        spec,
        &pr::RenderContext::new(theme),
        pr::ConfirmActionHandlers::default(),
    ))
}

pub fn js_context_menu(spec: &ContextMenuSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::context_menu(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_data_table(spec: &DataTableSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::data_table(
        spec,
        &pr::RenderContext::new(theme),
        pr::DataTableHandlers::default(),
    ))
}

pub fn js_data_table_loading(
    spec: &DataTableSpec,
    theme: &JetstreamThemeProvider,
    row_count: usize,
) -> El {
    El(pr::data_table_loading(spec, &pr::RenderContext::new(theme), row_count))
}

pub fn js_date_picker(spec: &DatePickerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::date_picker(
        spec,
        &pr::RenderContext::new(theme),
        pr::DatePickerHandlers::default(),
    ))
}

pub fn js_date_range_picker(spec: &DateRangePickerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::date_range_picker(
        spec,
        &pr::RenderContext::new(theme),
        pr::DatePickerHandlers::default(),
    ))
}

pub fn js_date_time_picker(spec: &DateTimePickerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::date_time_picker(
        spec,
        &pr::RenderContext::new(theme),
        pr::DatePickerHandlers::default(),
    ))
}

pub fn js_date_time_range_picker(
    spec: &DateTimeRangePickerSpec,
    theme: &JetstreamThemeProvider,
) -> El {
    El(pr::date_time_range_picker(
        spec,
        &pr::RenderContext::new(theme),
        pr::DatePickerHandlers::default(),
    ))
}

pub fn js_date_time_zone_picker(
    spec: &DateTimeZonePickerSpec,
    theme: &JetstreamThemeProvider,
) -> El {
    El(pr::date_time_zone_picker(
        spec,
        &pr::RenderContext::new(theme),
        pr::DateTimeZonePickerHandlers::default(),
    ))
}

pub fn js_debug_dialog(spec: &DebugDialogSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::debug_dialog(spec, &pr::RenderContext::new(theme)))
}

pub fn js_detail_item(spec: &DetailItemSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::detail_item(spec, &pr::RenderContext::new(theme)))
}

pub fn js_detail_item_with_slots(
    spec: &DetailItemSpec,
    theme: &JetstreamThemeProvider,
    value_content: Option<El>,
    action: Option<El>,
) -> El {
    let value_content: Option<Node> = value_content.map(Node::from);
    let action: Option<Node> = action.map(Node::from);
    El(pr::detail_item_with_slots(
        spec,
        &pr::RenderContext::new(theme),
        value_content,
        action,
    ))
}

pub fn js_detail_section(
    spec: &DetailSectionSpec,
    theme: &JetstreamThemeProvider,
    content: Vec<El>,
    actions: Option<El>,
) -> El {
    let content: Vec<Node> = content.into_iter().map(Node::from).collect();
    let actions: Option<Node> = actions.map(Node::from);
    El(pr::detail_section(spec, &pr::RenderContext::new(theme), content, actions))
}

pub fn js_detail_section_group(
    spec: &DetailSectionGroupSpec,
    _theme: &JetstreamThemeProvider,
    children: Vec<El>,
) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::detail_section_group(spec, _theme, children))
}

pub fn js_detail_shell(
    spec: &DetailShellSpec,
    theme: &JetstreamThemeProvider,
    header: Option<El>,
    content: Option<El>,
    state_content: Option<El>,
) -> El {
    let header: Option<Node> = header.map(Node::from);
    let content: Option<Node> = content.map(Node::from);
    let state_content: Option<Node> = state_content.map(Node::from);
    El(pr::detail_shell(
        spec,
        &pr::RenderContext::new(theme),
        header,
        content,
        state_content,
    ))
}

pub fn js_dialog(
    spec: &DialogSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<El>,
    actions: Option<El>,
) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    let actions: Option<Node> = actions.map(Node::from);
    El(pr::dialog(spec, &pr::RenderContext::new(theme), children, actions, None))
}

pub fn js_dock_region(
    spec: &DockRegionSpec,
    theme: &JetstreamThemeProvider,
    content: Option<El>,
) -> El {
    let content: Option<Node> = content.map(Node::from);
    El(pr::dock_region(
        spec,
        &pr::RenderContext::new(theme),
        content,
        pr::DockRegionHandlers::default(),
    ))
}

pub fn js_drawer(spec: &DrawerSpec, theme: &JetstreamThemeProvider, content: Option<El>) -> El {
    let content: Option<Node> = content.map(Node::from);
    El(pr::drawer(spec, &pr::RenderContext::new(theme), content, None, None))
}

pub fn js_drawer_with_actions(
    spec: &DrawerSpec,
    theme: &JetstreamThemeProvider,
    content: Option<El>,
    actions: Option<El>,
) -> El {
    let content: Option<Node> = content.map(Node::from);
    let actions: Option<Node> = actions.map(Node::from);
    El(pr::drawer(spec, &pr::RenderContext::new(theme), content, actions, None))
}

pub fn js_duration_input(spec: &DurationInputSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::duration_input(spec, &pr::RenderContext::new(theme)))
}

pub fn js_editable_label(spec: &EditableLabelSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::editable_label(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_editable_list(spec: &EditableListSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::editable_list(
        spec,
        &pr::RenderContext::new(theme),
        pr::EditableListHandlers::default(),
    ))
}

pub fn js_embed_input(spec: &EmbedInputSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::embed_input(spec, &pr::RenderContext::new(theme)))
}

pub fn js_embed_preview(spec: &EmbedPreviewSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::embed_preview(spec, &pr::RenderContext::new(theme)))
}

pub fn js_empty_state(spec: &EmptyStateSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::empty_state(spec, &pr::RenderContext::new(theme)))
}

pub fn js_error_boundary(
    spec: &ErrorBoundarySpec,
    theme: &JetstreamThemeProvider,
    child: Option<El>,
) -> El {
    let child: Option<Node> = child.map(Node::from);
    El(pr::error_boundary(spec, &pr::RenderContext::new(theme), child))
}

pub fn js_eyebrow(spec: &EyebrowSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::eyebrow(spec, &pr::RenderContext::new(theme)))
}

pub fn js_field(spec: &FieldSpec, theme: &JetstreamThemeProvider, control: Option<El>) -> El {
    // Compile-only jetstream adaptation: this preview's `El` chrome is built
    // eagerly, so the already-built node is wrapped in the slot builder the
    // shared renderer now requires. The internal scope cannot reach it until
    // this preview defers construction; no parity claim is made.
    let control = control.map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>);
    El(pr::field(spec, &pr::RenderContext::new(theme), control))
}

pub fn js_field_set(spec: &FieldSetSpec, theme: &JetstreamThemeProvider, children: Vec<El>) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::field_set(spec, &pr::RenderContext::new(theme), children))
}

pub fn js_file_upload(spec: &FileUploadSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::file_upload(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_filter_builder(
    spec: &FilterBuilderSpec,
    theme: &JetstreamThemeProvider,
    instance_id: &str,
) -> El {
    El(pr::filter_builder(
        spec,
        &pr::RenderContext::new(theme),
        instance_id,
        &pr::FilterBuilderHandlers::default(),
    ))
}

pub fn js_filter_toolbar(
    spec: &FilterToolbarSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<El>,
    actions: Option<El>,
    secondary: Option<El>,
) -> El {
    // Compile-only jetstream adaptation: this preview's `El` chrome is built
    // eagerly, so the already-built node is wrapped in the slot builder the
    // shared renderer now requires. The internal scope cannot reach it until
    // this preview defers construction; no parity claim is made.
    let children: Vec<pr::SlotBuilder<'static>> = children
        .into_iter()
        .map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>)
        .collect();
    let actions = actions.map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>);
    let secondary = secondary.map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>);
    El(pr::filter_toolbar(
        spec,
        &pr::RenderContext::new(theme),
        children,
        actions,
        secondary,
        None,
    ))
}

pub fn js_floating_overlay(
    anchor: El,
    surface: Option<El>,
    placement: OverlayPlacement,
    anchor_h: f32,
    anchor_w: f32,
) -> El {
    let surface: Option<Node> = surface.map(Node::from);
    El(pr::floating_overlay(
        anchor.into(),
        surface,
        placement,
        anchor_h,
        anchor_w,
    ))
}

pub fn js_form_actions(
    spec: &FormActionsSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<El>,
) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::form_actions(spec, &pr::RenderContext::new(theme), children))
}

pub fn js_form_actions_full(
    spec: &FormActionsSpec,
    theme: &JetstreamThemeProvider,
    danger: Vec<El>,
    children: Vec<El>,
) -> El {
    let danger: Vec<Node> = danger.into_iter().map(Node::from).collect();
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::form_actions_full(spec, &pr::RenderContext::new(theme), danger, children))
}

pub fn js_form_dialog(
    spec: &FormDialogSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<El>,
    custom_actions: Option<El>,
) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    let custom_actions: Option<Node> = custom_actions.map(Node::from);
    El(pr::form_dialog(
        spec,
        &pr::RenderContext::new(theme),
        children,
        custom_actions,
        pr::FormDialogHandlers::default(),
    ))
}

pub fn js_form_layout(
    spec: &FormLayoutSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<El>,
    actions: Option<El>,
) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    let actions: Option<Node> = actions.map(Node::from);
    El(pr::form_layout(spec, &pr::RenderContext::new(theme), children, actions))
}

pub fn js_form_shell(
    spec: &FormShellSpec,
    theme: &JetstreamThemeProvider,
    section_slots: Vec<Option<El>>,
    actions_slot: Option<El>,
) -> El {
    let actions_slot: Option<Node> = actions_slot.map(Node::from);
    let section_slots: Vec<Option<Node>> = section_slots
        .into_iter()
        .map(|s| s.map(Node::from))
        .collect();
    El(pr::form_shell(spec, &pr::RenderContext::new(theme), section_slots, actions_slot))
}

pub fn js_grid(spec: &GridSpec, theme: &JetstreamThemeProvider, children: Vec<El>) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::grid(spec, &pr::RenderContext::new(theme), children))
}

pub fn js_hover_card(
    spec: &HoverCardSpec,
    theme: &JetstreamThemeProvider,
    content: Option<El>,
) -> El {
    let content: Option<Node> = content.map(Node::from);
    El(pr::hover_card(spec, &pr::RenderContext::new(theme), content))
}

pub fn js_icon(spec: &IconSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::icon(spec, &pr::RenderContext::new(theme)))
}

pub fn js_icon_button(spec: &IconButtonSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::icon_button(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_inline_list_section(
    spec: &InlineListSectionSpec,
    theme: &JetstreamThemeProvider,
    items: Vec<El>,
    action: Option<El>,
) -> El {
    let items: Vec<Node> = items.into_iter().map(Node::from).collect();
    let action: Option<Node> = action.map(Node::from);
    El(pr::inline_list_section(spec, &pr::RenderContext::new(theme), items, action))
}

pub fn js_inline_remediation(spec: &InlineRemediationSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::inline_remediation(spec, &pr::RenderContext::new(theme)))
}

pub fn js_list_card_counter(spec: &ListCardCounterSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::list_card_counter(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_list_grid(
    spec: &ListGridSpec,
    theme: &JetstreamThemeProvider,
    header: Option<El>,
    children: Vec<El>,
) -> El {
    let header: Option<Node> = header.map(Node::from);
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::list_grid(spec, &pr::RenderContext::new(theme), header, children))
}

pub fn js_log_list(spec: &LogListSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::log_list(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_markdown_editor(spec: &MarkdownEditorSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::markdown_editor(spec, &pr::RenderContext::new(theme)))
}

pub fn js_media_browse_panel(spec: &MediaBrowsePanelSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::media_browse_panel(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_media_picker(spec: &MediaPickerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::media_picker(
        spec,
        &pr::RenderContext::new(theme),
        pr::MediaPickerHandlers::default(),
    ))
}

pub fn js_media_preview(spec: &MediaPreviewSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::media_preview(spec, &pr::RenderContext::new(theme)))
}

pub fn js_media_thumbnail(spec: &MediaThumbnailSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::media_thumbnail(spec, &pr::RenderContext::new(theme)))
}

pub fn js_menu(spec: &MenuSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::menu(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_menubar(spec: &MenubarSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::menubar(spec, &pr::RenderContext::new(theme), None, None))
}

pub fn js_meta_bar(spec: &MetaBarSpec, theme: &JetstreamThemeProvider, children: Vec<El>) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::meta_bar(spec, &pr::RenderContext::new(theme), children))
}

pub fn js_meta_bar_sep(
    spec: &MetaBarSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<(El, bool)>,
) -> El {
    El(pr::meta_bar_sep(
        spec,
        &pr::RenderContext::new(theme),
        children
            .into_iter()
            .map(|(c, s)| (Node::from(c), s))
            .collect(),
    ))
}

pub fn js_meta_item(spec: &MetaItemSpec, theme: &JetstreamThemeProvider, value: Option<El>) -> El {
    let value: Option<Node> = value.map(Node::from);
    El(pr::meta_item(spec, &pr::RenderContext::new(theme), value))
}

pub fn js_meter(spec: &MeterSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::meter(spec, &pr::RenderContext::new(theme)))
}

pub fn js_metric_tile(spec: &MetricTileSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::metric_tile(spec, &pr::RenderContext::new(theme)))
}

pub fn js_model_picker(
    spec: &ModelPickerSpec,
    theme: &JetstreamThemeProvider,
    instance_id: &str,
) -> El {
    El(pr::model_picker(spec, &pr::RenderContext::new(theme), instance_id, None))
}

pub fn js_nav_card(spec: &NavCardSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::nav_card(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_navigation_menu(spec: &NavigationMenuSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::navigation_menu(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_number_input(spec: &NumberInputSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::number_input(
        spec,
        &pr::RenderContext::new(theme),
        pr::NumberInputHandlers::default(),
    ))
}

pub fn js_order_by(spec: &OrderBySpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::order_by(spec, &pr::RenderContext::new(theme), pr::OrderByHandlers::default()))
}

pub fn js_page_header(spec: &PageHeaderSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::page_header(spec, &pr::RenderContext::new(theme), None, None, None))
}

pub fn js_page_header_with_slots(
    spec: &PageHeaderSpec,
    theme: &JetstreamThemeProvider,
    breadcrumbs: Option<El>,
    actions: Option<El>,
    meta: Option<El>,
) -> El {
    // Compile-only jetstream adaptation: this preview's `El` chrome is built
    // eagerly, so the already-built node is wrapped in the slot builder the
    // shared renderer now requires. The internal scope cannot reach it until
    // this preview defers construction; no parity claim is made.
    let breadcrumbs = breadcrumbs.map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>);
    let actions = actions.map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>);
    let meta = meta.map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>);
    El(pr::page_header(
        spec,
        &pr::RenderContext::new(theme),
        breadcrumbs,
        actions,
        meta,
    ))
}

pub fn js_page_loading(spec: &PageLoadingSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::page_loading(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_pagination(spec: &PaginationSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::pagination(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_pagination_summary(spec: &PaginationSummarySpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::pagination_summary(spec, &pr::RenderContext::new(theme)))
}

pub fn js_password_requirements(
    spec: &PasswordRequirementsSpec,
    theme: &JetstreamThemeProvider,
) -> El {
    El(pr::password_requirements(spec, &pr::RenderContext::new(theme)))
}

pub fn js_picker_shell(
    spec: &PickerShellSpec,
    theme: &JetstreamThemeProvider,
    toolbar: Option<El>,
    selection: Option<El>,
    body: Option<El>,
    state_content: Option<El>,
    footer: Option<El>,
) -> El {
    let toolbar: Option<Node> = toolbar.map(Node::from);
    let selection: Option<Node> = selection.map(Node::from);
    let body: Option<Node> = body.map(Node::from);
    let state_content: Option<Node> = state_content.map(Node::from);
    let footer: Option<Node> = footer.map(Node::from);
    El(pr::picker_shell(
        spec,
        &pr::RenderContext::new(theme),
        toolbar,
        selection,
        body,
        state_content,
        footer,
    ))
}

pub fn js_pill(spec: &PillSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::pill(spec, &pr::RenderContext::new(theme)))
}

pub fn js_popover(spec: &PopoverSpec, theme: &JetstreamThemeProvider, content: Option<El>) -> El {
    let content: Option<Node> = content.map(Node::from);
    El(pr::popover(spec, &pr::RenderContext::new(theme), content))
}

pub fn js_progress(spec: &ProgressSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::progress(spec, &pr::RenderContext::new(theme)))
}

pub fn js_radio_group(
    spec: &RadioGroupSpec,
    theme: &JetstreamThemeProvider,
    instance_id: impl Into<String>,
) -> El {
    El(pr::radio_group(
        spec,
        &pr::RenderContext::new(theme),
        pr::RadioGroupHandlers::new(instance_id),
    ))
}

pub fn js_range_slider(spec: &RangeSliderSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::range_slider(
        spec,
        &pr::RenderContext::new(theme),
        pr::RangeSliderHandlers::default(),
    ))
}

pub fn js_rating(spec: &RatingSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::rating(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_ref_select(spec: &RefSelectSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::ref_select(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_region(spec: &RegionSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::region(spec, &pr::RenderContext::new(theme)))
}

pub fn js_relation_picker(spec: &RelationPickerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::relation_picker(
        spec,
        &pr::RenderContext::new(theme),
        pr::RelationPickerHandlers::default(),
    ))
}

pub fn js_remediation_banner(spec: &RemediationBannerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::remediation_banner(
        spec,
        &pr::RenderContext::new(theme),
        pr::RemediationBannerHandlers::default(),
    ))
}

pub fn js_resize_handle(spec: &ResizeHandleSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::resize_handle(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_scroll_shell(
    spec: &ScrollShellSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<El>,
) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::scroll_shell(spec, &pr::RenderContext::new(theme), children))
}

pub fn js_segmented_control(spec: &SegmentedControlSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::segmented_control(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_select(spec: &SelectSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::select(spec, &pr::RenderContext::new(theme), &pr::SelectHandlers::default()))
}

pub fn js_selection_summary(spec: &SelectionSummarySpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::selection_summary(
        spec,
        &pr::RenderContext::new(theme),
        pr::SelectionSummaryHandlers::default(),
    ))
}

pub fn js_separator(spec: &SeparatorSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::separator(spec, &pr::RenderContext::new(theme)))
}

pub fn js_shell_status_bar(
    spec: &ShellStatusBarSpec,
    theme: &JetstreamThemeProvider,
    leading: Vec<El>,
    trailing: Vec<El>,
) -> El {
    let leading: Vec<Node> = leading.into_iter().map(Node::from).collect();
    let trailing: Vec<Node> = trailing.into_iter().map(Node::from).collect();
    El(pr::shell_status_bar(spec, &pr::RenderContext::new(theme), leading, trailing))
}

pub fn js_sidebar_nav(spec: &SidebarNavSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::sidebar_nav(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_skeleton(spec: &SkeletonSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::skeleton(spec, &pr::RenderContext::new(theme)))
}

pub fn js_slider(spec: &SliderSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::slider(spec, &pr::RenderContext::new(theme), &pr::SliderHandlers::default()))
}

pub fn js_spacer(spec: &SpacerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::spacer(spec, &pr::RenderContext::new(theme)))
}

pub fn js_split_button(spec: &SplitButtonSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::split_button(
        spec,
        &pr::RenderContext::new(theme),
        pr::SplitButtonHandlers::default(),
    ))
}

pub fn js_split_view(
    spec: &SplitViewSpec,
    theme: &JetstreamThemeProvider,
    primary: Option<El>,
    secondary: Option<El>,
) -> El {
    let primary: Option<Node> = primary.map(Node::from);
    let secondary: Option<Node> = secondary.map(Node::from);
    El(pr::split_view(
        spec,
        &pr::RenderContext::new(theme),
        primary,
        secondary,
        pr::SplitViewHandlers::default(),
    ))
}

pub fn js_stack(spec: &StackSpec, theme: &JetstreamThemeProvider, children: Vec<El>) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::stack(spec, &pr::RenderContext::new(theme), children))
}

pub fn js_state_tile(spec: &StateTileSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::state_tile(spec, &pr::RenderContext::new(theme)))
}

pub fn js_status_indicator(spec: &StatusIndicatorSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::status_indicator(spec, &pr::RenderContext::new(theme)))
}

pub fn js_stepper(spec: &StepperSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::stepper(spec, &pr::RenderContext::new(theme), pr::StepperHandlers::default()))
}

pub fn js_surface(spec: &SurfaceSpec, theme: &JetstreamThemeProvider, children: Vec<El>) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::surface(spec, &pr::RenderContext::new(theme), children))
}

pub fn js_switch(spec: &SwitchSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::switch(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_tab_strip(spec: &TabStripSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::tab_strip(spec, &pr::RenderContext::new(theme), pr::TabStripHandlers::default()))
}

pub fn js_table(spec: &TableSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::table(spec, &pr::RenderContext::new(theme)))
}

pub fn js_tabs(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::tabs(spec, &pr::RenderContext::new(theme), None, None))
}

pub fn js_text(spec: &TextSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::text(spec, &pr::RenderContext::new(theme)))
}

pub fn js_text_input(spec: &TextInputSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::text_input(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_text_link(spec: &TextLinkSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::text_link(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_theme_select(spec: &ThemeSelectSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::theme_select(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_time_ago(spec: &TimeAgoSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::time_ago(spec, &pr::RenderContext::new(theme)))
}

pub fn js_time_field(spec: &TimeFieldSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::time_field(spec, &pr::RenderContext::new(theme)))
}

pub fn js_time_zone_select(spec: &TimeZoneSelectSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::time_zone_select(
        spec,
        &pr::RenderContext::new(theme),
        pr::TimeZoneSelectHandlers::default(),
    ))
}

pub fn js_toast_host(
    spec: &ToastHostSpec,
    theme: &JetstreamThemeProvider,
    stack_spec: &ToastStackSpec,
) -> El {
    El(pr::toast_host(
        spec,
        &pr::RenderContext::new(theme),
        stack_spec,
        pr::ToastStackHandlers::default(),
    ))
}

pub fn js_toast_stack(spec: &ToastStackSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::toast_stack(
        spec,
        &pr::RenderContext::new(theme),
        pr::ToastStackHandlers::default(),
    ))
}

pub fn js_toggle_group(spec: &ToggleGroupSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::toggle_group(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_token_input(spec: &TokenInputSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::token_input(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_toolbar(spec: &ToolbarSpec, theme: &JetstreamThemeProvider, children: Vec<El>) -> El {
    let children: Vec<Node> = children.into_iter().map(Node::from).collect();
    El(pr::toolbar(spec, &pr::RenderContext::new(theme), children))
}

pub fn js_tooltip(spec: &TooltipSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::tooltip(spec, &pr::RenderContext::new(theme)))
}

pub fn js_tree(spec: &TreeSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::tree(spec, &pr::RenderContext::new(theme), pr::TreeHandlers::default()))
}

pub fn js_tri_state_switch(spec: &TriStateSwitchSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::tri_state_switch(spec, &pr::RenderContext::new(theme), None))
}

pub fn js_validation_summary(spec: &ValidationSummarySpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::validation_summary(spec, &pr::RenderContext::new(theme)))
}

pub fn js_video_player(spec: &VideoPlayerSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::video_player(spec, &pr::RenderContext::new(theme)))
}

pub fn js_alert_dialog(spec: &AlertDialogSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::alert_dialog(
        spec,
        &pr::RenderContext::new(theme),
        false,
        pr::alert_dialog::DEFAULT_WORKING_LABEL,
        pr::AlertDialogHandlers::default(),
    ))
}

pub fn js_list_card(spec: &ListCardSpec, theme: &JetstreamThemeProvider) -> El {
    El(pr::list_card(
        spec,
        &pr::RenderContext::new(theme),
        pr::ListCardSlots::default(),
        None,
    ))
}

pub fn js_list_card_with_slots(
    spec: &ListCardSpec,
    theme: &JetstreamThemeProvider,
    leading: Option<El>,
    badges: Vec<El>,
    footer: Option<El>,
    actions: Option<El>,
    trailing: Option<El>,
    corner: Option<El>,
) -> El {
    El(pr::list_card(
        spec,
        &pr::RenderContext::new(theme),
        pr::ListCardSlots {
            leading: leading.map(Node::from),
            badges: badges.into_iter().map(Node::from).collect(),
            footer: footer.map(Node::from),
            actions: actions.map(Node::from),
            trailing: trailing.map(Node::from),
            corner: corner.map(Node::from),
        },
        None,
    ))
}

pub fn js_list_container(
    spec: &ListContainerSpec,
    theme: &JetstreamThemeProvider,
    content: Option<El>,
    filters: Option<El>,
    batch: Option<El>,
) -> El {
    js_list_container_with_slots(spec, theme, content, filters, batch, None, None)
}

pub fn js_list_container_with_slots(
    spec: &ListContainerSpec,
    theme: &JetstreamThemeProvider,
    content: Option<El>,
    filters: Option<El>,
    batch: Option<El>,
    breadcrumbs: Option<El>,
    actions: Option<El>,
) -> El {
    El(pr::list_container(
        spec,
        &pr::RenderContext::new(theme),
        pr::ListContainerSlots {
            content: content.map(Node::from),
            filters: filters.map(Node::from),
            batch: batch.map(Node::from),
            // Eager-`El` compile-only wrap, same caveat as above.
            breadcrumbs: breadcrumbs
                .map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>),
            actions: actions
                .map(|el| Box::new(move |_| Node::from(el)) as pr::SlotBuilder<'static>),
        },
        None,
    ))
}

pub fn js_box(spec: &BoxSpec, theme: &JetstreamThemeProvider, children: Vec<El>) -> El {
    El(pr::bx(
        spec,
        &pr::RenderContext::new(theme),
        children.into_iter().map(Node::from).collect(),
    ))
}
