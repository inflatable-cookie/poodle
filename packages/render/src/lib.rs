//! The single Rust component implementation.
//!
//! Each component here is a pure function `Spec + Theme → poodle_node::Node`.
//! No backend types, no measurement, no window: what a Button *is*, decided
//! once. Per-backend adapters (GPUI's in this repo, Jetstream's in its own)
//! interpret the node tree; the parity evidence for each backend lives with
//! that backend, against fixtures this crate can generate headlessly.
//!
//! The g12.019 migration consolidated the former GPUI and Jetstream component
//! tiers here after proving the node vocabulary against `Select`. Migration
//! history and parity evidence live in
//! `docs/roadmaps/g12/019-gpui-node-backend.md`.

pub mod accordion;
pub mod action_discovery_panel;
pub mod agent_chat_input;
pub mod agent_message;
pub mod agent_plan;
pub mod agent_plan_record;
pub mod agent_question;
pub mod agent_question_record;
pub mod agent_subagent;
pub mod agent_transcript;
pub mod alert_dialog;
pub mod app_header;
pub mod audio;
pub mod audio_handlers;
pub mod audio_player;
pub mod audio_specimens;
pub mod avatar;
pub mod badge;
pub mod banner;
pub mod block_editor;
pub mod breadcrumbs;
pub mod bulk_action_bar;
pub mod button;
pub mod bx;
pub mod calendar;
pub mod callout;
pub mod card;
pub mod card_radio_group;
pub mod card_toggle_group;
pub mod changed_files;
pub mod checkbox;
pub mod code;
pub mod code_input;
pub mod collapse_toggle;
pub mod collapsible;
pub mod color;
pub mod color_picker;
pub mod command_palette;
pub mod confirm_action;
pub mod context;
pub mod context_menu;
pub mod data_table;
pub mod date_picker;
pub mod date_range_picker;
pub mod date_time_picker;
pub mod date_time_range_picker;
pub mod date_time_zone_picker;
pub mod debug_dialog;
pub mod detail_item;
pub mod detail_section;
pub mod detail_section_group;
pub mod detail_shell;
pub mod dialog;
pub mod dock_region;
pub mod drawer;
pub mod duration_input;
pub mod editable_label;
pub mod editable_list;
pub mod embed_input;
pub mod embed_preview;
pub mod empty_state;
pub mod error_boundary;
pub mod eyebrow;
pub mod field;
pub mod field_set;
pub mod file_upload;
pub mod filter_builder;
pub mod filter_toolbar;
pub mod floating_overlay;
pub mod form_actions;
pub mod form_dialog;
pub mod form_layout;
pub mod form_shell;
pub mod grid;
pub mod history_center;
pub mod hover_card;
pub mod icon;
pub mod icon_button;
pub mod icon_provider;
pub mod inline_list_section;
pub mod inline_remediation;
pub mod licence_activation;
pub mod licence_seats;
pub mod licence_status;
pub mod list_card;
pub mod list_card_counter;
pub mod list_container;
pub mod list_grid;
pub mod log_list;
pub mod markdown_editor;
pub mod media_browse_panel;
pub mod media_picker;
pub mod media_preview;
pub mod media_thumbnail;
pub mod menu;
pub mod menubar;
pub mod message_center;
pub mod meta_bar;
pub mod meta_item;
pub mod meter;
pub mod metric_tile;
pub mod model_catalogue_editor;
pub mod model_connection_card;
pub mod model_connection_picker;
pub mod model_connection_setup;
pub mod model_picker;
pub mod nav_card;
pub mod navigation_menu;
pub mod number_input;
pub mod order_by;
pub mod page_header;
pub mod page_loading;
pub mod pagination;
pub mod pagination_summary;
pub mod password_requirements;
pub mod picker_shell;
mod picker_trigger;
pub mod pill;
pub mod popover;
pub mod presentation;
pub mod progress;
pub mod radio;
pub mod radio_group;
pub mod range_slider;
pub mod rating;
pub mod ref_select;
pub mod region;
pub mod relation_picker;
pub mod remediation_banner;
pub mod resize_handle;
pub mod scroll_shell;
pub mod segmented_control;
pub mod select;
pub mod selection_summary;
pub mod separator;
pub mod settings_shell;
pub mod shell_status_bar;
pub mod sidebar_nav;
pub mod skeleton;
pub mod slider;
pub mod spacer;
pub mod spinner;
pub mod split_button;
pub mod split_view;
pub mod stack;
pub mod state_tile;
pub mod status_indicator;
pub mod stepper;
pub mod surface;
pub mod switch;
pub mod tab_strip;
pub mod table;
pub mod tabs;
pub mod text;
pub mod text_input;
pub mod text_link;
pub mod theme_select;
pub mod time_ago;
pub mod time_input;
pub mod time_zone_select;
pub mod toast_host;
pub mod toast_stack;
pub mod toggle_group;
pub mod token_input;
pub mod tool_call;
pub mod tool_call_group;
pub mod toolbar;
pub mod tooltip;
pub mod tree;
pub mod tri_state_switch;
pub mod update_center;
pub mod update_status;
pub mod validation_summary;
pub mod video_player;

pub use accordion::{
    accordion, accordion_panel_focus_id, accordion_trigger_focus_id, accordion_with_content,
    AccordionHandlers,
};
pub use action_discovery_panel::{
    action_discovery_panel, action_discovery_row_focus_id, ActionDiscoveryPanelHandlers,
};
pub use agent_chat_input::{agent_chat_input, AgentChatInputHandlers};
pub use agent_message::agent_message;
pub use agent_plan::{agent_plan, agent_plan_action_focus_id, AgentPlanHandlers};
pub use agent_plan_record::{
    agent_plan_record, agent_plan_record_toggle_focus_id, AgentPlanRecordHandlers,
    AGENT_PLAN_RECORD_TOGGLE_ID,
};
pub use agent_question::{agent_question, AgentQuestionHandlers};
pub use agent_question_record::agent_question_record;
pub use agent_subagent::{agent_subagent, agent_subagent_action_focus_id, AgentSubagentHandlers};
pub use agent_transcript::{agent_transcript, AgentTranscriptHandlers};
pub use alert_dialog::{
    alert_dialog, alert_dialog_with_content, AlertDialogHandlers, DEFAULT_WORKING_LABEL,
};
pub use app_header::app_header;
pub use audio::{
    audio_meter, audio_switch, drag_number_field, envelope_editor, fader, fader_with_handlers,
    gain_reduction_meter, keyboard, knob, knob_with_handlers, mod_matrix_grid, value_readout,
    waveform_display, xy_pad, xy_pad_with_handlers,
};
pub use audio_handlers::{
    audio_entry_id, audio_root_id, fader_context_from_spec, fader_spec_from_context,
    knob_context_from_spec, knob_spec_from_context, xy_pad_context_from_spec,
    xy_pad_spec_from_context, xy_pad_x_id, xy_pad_y_id, FaderHandlers, FaderLive, KnobHandlers,
    KnobLive, XYPadHandlers, XYPadLive,
};
pub use audio_player::audio_player;
pub use avatar::avatar;
pub use badge::badge;
pub use banner::banner;
pub use block_editor::{block_editor, block_editor_with_children};
pub use breadcrumbs::breadcrumbs;
pub use bulk_action_bar::{bulk_action_bar, BulkActionBarHandlers};
pub use button::button;
pub use bx::bx;
pub use calendar::{calendar, CalendarHandlers};
pub use callout::{callout, callout_dismiss_focus_id, CalloutHandlers, CALLOUT_DISMISS_ID};
pub use card::card;
pub use card_radio_group::card_radio_group;
pub use card_toggle_group::card_toggle_group;
pub use changed_files::{changed_files, ChangedFilesHandlers};
pub use checkbox::checkbox;
pub use code::code;
pub use code_input::{code_input, code_input_with_handlers, CodeInputHandlers};
pub use collapse_toggle::collapse_toggle;
pub use collapsible::{
    collapsible, collapsible_content_focus_id, collapsible_trigger_focus_id,
    collapsible_with_handlers, CollapsibleHandlers,
};
pub use color_picker::{color_picker, ColorPickerHandlers};
pub use command_palette::{command_palette, command_palette_with_handlers, CommandPaletteHandlers};
pub use confirm_action::{confirm_action, confirm_action_with_slots, ConfirmActionHandlers};
// The fixed public path for the construction-time presentation API
// (architecture 010): `poodle_render::RenderContext`.
pub use context::{ui_presentation_provider, RenderContext, SlotBuilder};
pub use context_menu::context_menu;
pub use data_table::{data_table, data_table_loading, DataTableHandlers};
pub use date_picker::{date_picker, DatePickerHandlers};
pub use date_range_picker::date_range_picker;
pub use date_time_picker::date_time_picker;
pub use date_time_range_picker::date_time_range_picker;
pub use date_time_zone_picker::{date_time_zone_picker, DateTimeZonePickerHandlers};
pub use debug_dialog::debug_dialog;
pub use detail_item::{detail_item, detail_item_with_slots};
pub use detail_section::detail_section;
pub use detail_section_group::detail_section_group;
pub use detail_shell::detail_shell;
pub use dialog::{dialog, dialog_with_slots};
pub use dock_region::{dock_collapse_focus_id, dock_region, dock_tab_focus_id, DockRegionHandlers};
pub use drawer::drawer;
pub use duration_input::{duration_input, duration_input_with_handlers, DurationInputHandlers};
pub use editable_label::{editable_label, editable_label_with_handlers, EditableLabelHandlers};
pub use editable_list::{editable_list, EditableListHandlers};
pub use embed_input::embed_input;
pub use embed_preview::embed_preview;
pub use empty_state::empty_state;
pub use error_boundary::error_boundary;
pub use eyebrow::eyebrow;
pub use field::field;
pub use field_set::field_set;
pub use file_upload::{file_upload, file_upload_with_handlers, FileUploadHandlers};
pub use filter_builder::{filter_builder, FilterBuilderHandlers};
pub use filter_toolbar::filter_toolbar;
pub use floating_overlay::floating_overlay;
pub use form_actions::{form_actions, form_actions_full};
pub use form_dialog::{form_dialog, FormDialogHandlers};
pub use form_layout::form_layout;
pub use form_shell::form_shell;
pub use grid::grid;
pub use history_center::{
    history_center, HistoryCenterDelete, HistoryCenterHandlers, HistoryCenterRename,
    HistoryCenterView,
};
pub use hover_card::hover_card;
pub use icon::icon;
pub use icon_button::{icon_button, icon_button_with_handlers, IconButtonHandlers};
pub use icon_provider::icon_provider;
pub use inline_list_section::inline_list_section;
pub use inline_remediation::inline_remediation;
pub use licence_activation::{
    licence_activation, licence_activation_with_slots, LicenceActivationHandlers,
};
pub use licence_seats::{licence_seats, LicenceSeatsHandlers};
pub use licence_status::licence_status;
pub use list_card::{list_card, ListCardSlots};
pub use list_card_counter::list_card_counter;
pub use list_container::{list_container, ListContainerSlots};
pub use list_grid::list_grid;
pub use log_list::log_list;
pub use markdown_editor::{markdown_editor, markdown_editor_with_handlers, MarkdownEditorHandlers};
pub use media_browse_panel::media_browse_panel;
pub use media_picker::{media_picker, MediaPickerHandlers};
pub use media_preview::{media_preview, media_preview_with_content};
pub use media_thumbnail::{media_thumbnail, media_thumbnail_with_content};
pub use menu::menu;
pub use menubar::menubar;
pub use message_center::{message_center, MessageCenterHandlers};
pub use meta_bar::{meta_bar, meta_bar_sep};
pub use meta_item::meta_item;
pub use meter::meter;
pub use metric_tile::metric_tile;
pub use model_catalogue_editor::{
    model_catalogue_editor, model_catalogue_editor_with_slots, model_catalogue_handle_focus_id,
    model_catalogue_hidden_focus_id, ModelCatalogueEditorHandlers, ModelCatalogueEditorSlots,
};
pub use model_connection_card::{
    model_connection_card, model_connection_card_with_slots, ModelConnectionCardHandlers,
    ModelConnectionCardSlots,
};
pub use model_connection_picker::{
    model_connection_option_focus_id, model_connection_option_id, model_connection_picker,
    model_connection_picker_search_id, model_connection_picker_with_slots,
    ModelConnectionPickerHandlers, ModelConnectionPickerSlots,
};
pub use model_connection_setup::{
    model_connection_setup, model_connection_setup_action_id,
    model_connection_setup_title_focus_id, model_connection_setup_with_slots,
    ModelConnectionSetupHandlers, ModelConnectionSetupSlots, MODEL_CONNECTION_SETUP_TITLE_ID,
};
pub use model_picker::model_picker;
pub use nav_card::{nav_card, nav_card_with_icon};
pub use navigation_menu::navigation_menu;
pub use number_input::{number_input, NumberInputHandlers};
pub use order_by::{order_by, OrderByHandlers};
pub use page_header::page_header;
pub use page_loading::page_loading;
pub use pagination::{pagination, pagination_with_handlers, PaginationHandlers};
pub use pagination_summary::pagination_summary;
pub use password_requirements::password_requirements;
pub use picker_shell::picker_shell;
pub use pill::{pill, pill_with_remove};
pub use poodle_headless::select::{SelectContext, SelectEffect, SelectEvent};
pub use popover::{
    popover, popover_surface, PopoverHandlers, POPOVER_ANCHOR_HEIGHT_PX, POPOVER_ANCHOR_WIDTH_PX,
};
pub use progress::progress;
pub use radio::radio;
pub use radio_group::{radio_group, RadioGroupHandlers};
pub use range_slider::{range_slider, RangeSliderHandlers};
pub use rating::{rating, RatingHandlers};
pub use ref_select::ref_select;
pub use region::region;
pub use relation_picker::{relation_picker, RelationPickerHandlers};
pub use remediation_banner::{
    remediation_banner, remediation_banner_action_focus_id, remediation_banner_dismiss_focus_id,
    RemediationBannerHandlers,
};
pub use resize_handle::{resize_handle, resize_handle_focus_id, ResizePhase};
pub use scroll_shell::scroll_shell;
pub use segmented_control::segmented_control;
pub use select::{
    select, select_option_id, select_search_focus_id, select_trigger_focus_id, SelectHandlers,
    SelectTransitionResult,
};
pub use selection_summary::{selection_summary, SelectionSummaryHandlers};
pub use separator::separator;
pub use settings_shell::{settings_shell, SettingsShellHandlers};
pub use shell_status_bar::shell_status_bar;
pub use sidebar_nav::sidebar_nav;
pub use skeleton::skeleton;
pub use slider::{slider, SliderHandlers};
pub use spacer::spacer;
pub use spinner::spinner;
pub use split_button::{split_button, SplitButtonHandlers};
pub use split_view::{split_view, SplitViewHandlers};
pub use stack::stack;
pub use state_tile::state_tile;
pub use status_indicator::status_indicator;
pub use stepper::{stepper, StepperHandlers};
pub use surface::surface;
pub use switch::switch;
pub use tab_strip::{tab_strip, TabStripHandlers};
pub use table::table;
pub use tabs::{
    tabs, tabs_with_handlers, tabs_with_panel, TabDropTargetHandler, TabHandler, TabOrderHandler,
    TabsHandlers,
};
pub use text::text;
pub use text_input::{
    text_input, text_input_with_change, text_input_with_handlers, TextInputHandlers,
};
pub use text_link::text_link;
pub use theme_select::{theme_select, theme_select_with_handlers, ThemeSelectHandlers};
pub use time_ago::time_ago;
pub use time_input::{
    context_from_spec, time_input, time_input_with_change, time_input_with_handlers,
    time_input_with_persistent_context, TimeInputHandlers,
};
pub use time_zone_select::{time_zone_select, TimeZoneSelectHandlers};
pub use toast_host::toast_host;
pub use toast_stack::{toast_stack, ToastStackHandlers};
pub use toggle_group::{toggle_group, ToggleGroupHandlers};
pub use token_input::token_input;
pub use tool_call::{tool_call, tool_call_focus_id, ToolCallHandlers};
pub use tool_call_group::{tool_call_group, ToolCallGroupHandlers};
pub use toolbar::toolbar;
pub use tooltip::tooltip;
pub use tree::{tree, TreeDropHandler, TreeHandlers, TreeKeyHandler};
pub use tri_state_switch::{tri_state_switch, TriStateSwitchHandlers};
pub use update_center::{update_center, UpdateCenterHandlers};
pub use update_status::{update_status, UpdateStatusHandlers};
pub use validation_summary::validation_summary;
pub use video_player::video_player;
