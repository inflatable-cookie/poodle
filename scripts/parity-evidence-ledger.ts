import fs from "node:fs";
import path from "node:path";
import { canonicalComponents } from "../packages/svelte/preview/src/generated/catalogue/catalogue";
import { assertPublicBarrelAgrees } from "./component-denominator";
import {
  deriveNucleusReceiptRows,
  NUCLEUS_MANIFEST_PATH,
  NUCLEUS_RECEIPT_SCHEMA,
  type NucleusReceiptRow,
} from "./nucleus-parity-receipts";

export const LEDGER_PATH = "docs/evidence/nucleus/parity-evidence-ledger.md";

const ROOT = path.resolve(import.meta.dir, "..");
const COMPONENT_COLUMNS = [
  "Component",
  "Contract",
  "Svelte surface",
  "React surface",
  "Shared Rust surface",
  "GPUI construction",
  "GPUI mounted behaviour",
  "Web accessibility",
  "GPUI accessibility",
  "Web visual",
  "GPUI visual",
  "Known deltas",
] as const;

const EVIDENCE_STATUSES = [
  "present",
  "focused",
  "mounted",
  "compared",
  "manual",
  "missing",
  "not-applicable",
  "deferred",
] as const;

type EvidenceStatus = (typeof EVIDENCE_STATUSES)[number];
type ComponentColumn = (typeof COMPONENT_COLUMNS)[number];
type ComponentRow = Record<ComponentColumn, string>;
type LiveComponent = {
  name: string;
  slug: string;
  portable: boolean;
};

const RUST_SPEC_OVERRIDES: Record<string, string> = {
  Callout: "CallOutSpec",
  StatusBar: "ShellStatusBarSpec",
  TimeInput: "TimeInputSpec",
};

const RENDER_MODULE_OVERRIDES: Record<string, string> = {
  Box: "bx",
  StatusBar: "shell_status_bar",
  TimeInput: "time_input",
  UiPresentationProvider: "context",
  MotionPolicyProvider: "context",
};

const AUDIO_RENDER_COMPONENTS = new Set([
  "AudioMeter",
  "AudioSwitch",
  "DragNumberField",
  "EnvelopeEditor",
  "Fader",
  "GainReductionMeter",
  "Keyboard",
  "Knob",
  "ModMatrixGrid",
  "ValueReadout",
  "WaveformDisplay",
  "XYPad",
]);

// Retained as an expected-test map for planning traceability. It is never
// evidence by itself; GPUI mounted cells below are driven by validated M1
// receipts only.
export const EXPECTED_MOUNTED_BEHAVIOUR_TESTS: Record<string, string | string[]> = {
  Avatar: "first_mounted_parity_avatar",
  Box: "first_mounted_parity_box",
  Grid: "first_mounted_parity_grid",
  ListGrid: "first_mounted_parity_list_grid",
  Stack: "first_mounted_parity_stack",
  Spacer: "first_mounted_parity_spacer",
  Skeleton: "first_mounted_parity_skeleton",
  TimeAgo: "first_mounted_parity_time_ago",
  MetaBar: "first_mounted_parity_meta_bar",
  MetaItem: "first_mounted_parity_meta_item",
  AudioPlayer: "first_mounted_parity_audio_player",
  Table: "first_mounted_parity_table",
  DataTable: "first_mounted_parity_data_table",
  FilterBuilder: "first_mounted_parity_filter_builder",
  FilterToolbar: "first_mounted_parity_filter_toolbar",
  Toolbar: "first_mounted_parity_toolbar",
  TimeZoneSelect: "first_mounted_parity_time_zone_select",
  DatePicker: "first_mounted_parity_date_picker",
  DateRangePicker: "first_mounted_parity_date_range_picker",
  DateTimePicker: "first_mounted_parity_date_time_picker",
  DateTimeRangePicker: "first_mounted_parity_date_time_range_picker",
  DateTimeZonePicker: "first_mounted_parity_date_time_zone_picker",
  MediaBrowsePanel: "media_browse_panel_selection_and_media_thumbnail_content_reach_mounted_gpui",
  MediaPicker: "media_picker_dialog_search_selection_and_dismissal_rebuild_the_host",
  MediaThumbnail: "media_thumbnail_states_name_and_size_reach_mounted_gpui",
  VideoPlayer: "video_player_canvas_and_play_button_rebuild_the_mounted_host",
  Button: "a_mounted_button_carries_its_controls_target",
  Checkbox: "checkbox_toggle_readonly_and_disabled_rebuild_the_host_spec",
  Switch: "switch_toggle_readonly_and_disabled_rebuild_the_host_spec",
  SegmentedControl: "segmented_control_exclusive_focus_identity_and_disabled_paths",
  RadioGroup: "radio_group_exclusive_focus_identity_and_disabled_paths",
  ToggleGroup: "toggle_group_result_focus_identity_and_disabled_paths",
  CardRadioGroup: "first_mounted_parity_card_radio_group",
  CardToggleGroup: "first_mounted_parity_card_toggle_group",
  ListCard: "first_mounted_parity_list_card",
  RangeSlider: "a_scrub_reports_change_while_dragging_and_commits_once_at_release",
  Slider: "slider_axis_keyboard_and_disabled_rebuild_the_host_spec",
  Fader: "fader_mounted_parity_through_production_dispatch",
  Knob: "knob_mounted_parity_through_production_dispatch",
  XYPad: "xy_pad_mounted_parity_through_production_dispatch",
  Tabs: "tabs_drag_keyboard_and_identity_rebuild_the_host_spec",
  Tree: "tree_selection_expand_and_substrate_reorder_rebuild_the_host_spec",
  SidebarNav: [
    "sidebar_nav_end_labels_render_muted_metadata_and_describe_the_item",
    "sidebar_nav_foo_and_foo_end_label_values_keep_distinct_ids",
    "sidebar_nav_vertical_tab_and_form_feed_values_encode_in_ids",
    "sidebar_nav_instance_scopes_keep_same_value_rows_distinct",
    "sidebar_nav_wrapped_label_keeps_the_end_label_on_its_first_line",
    "sidebar_nav_end_label_counts_paint_tabular",
    "sidebar_nav_item_context_menu_opens_by_pointer_and_keyboard_and_restores_focus",
  ],
  EditableList: "editable_list_substrate_reorder_rebuilds_the_host_spec",
  OrderBy: "order_by_substrate_reorder_and_alt_arrow_rebuild_the_host_spec",
  BlockEditor: "block_editor_grip_drag_and_move_controls_rebuild_the_host_spec",
  MarkdownEditor: "markdown_editor_mode_toolbar_and_edit_rebuild_the_host_spec",
  TextInput: "text_input_controlled_editing_and_identity_rebuild_the_host_spec",
  TokenInput: "token_input_entry_removal_and_keyboard_rebuild_the_host_spec",
  DragNumberField:
    "drag_number_field_scrub_keyboard_bounds_and_entry_rebuild_the_host_spec",
  DurationInput: "duration_input_segments_edit_and_rebuild_the_host_spec",
  TimeInput: "time_input_segmented_editor_commits_drafts_and_bounds",
  NumberInput: [
    "number_input_mounted_valid_direct_editing_rebuilds_host_draft_and_value",
    "number_input_mounted_accessibility_projects_spin_button_surface",
  ],
  EditableLabel: [
    "editable_label_commits_on_enter_and_once_through_the_blur_tab_causes",
    "editable_label_live_draft_stays_off_the_committed_value",
  ],
  Breadcrumbs: "breadcrumbs_callback_navigation_through_mounted_pointer_and_keyboard",
  IconButton: "icon_button_activation_toggle_and_tooltip_through_mounted_pointer_and_keyboard",
  Collapsible: "collapsible_disclosure_and_identity_through_mounted_pointer_and_keyboard",
  CollapseToggle: "collapse_toggle_disclosure_focus_and_disabled_through_mounted_pointer_and_keyboard",
  Pagination: "pagination_navigation_limit_and_loading_through_mounted_pointer_and_keyboard",
  PaginationSummary: "gpui_mounted_pagination_summary_range_live_name_and_geometry",
  Pill: "gpui_mounted_pill_dismiss_actions_are_instance_scoped_and_retain_focus_after_rebuild",
  SelectionSummary: "gpui_mounted_selection_summary_split_actions_and_accessible_names",
  NavCard: "gpui_mounted_nav_card_link_button_actions_and_accessibility",
  Rating: "rating_nullable_fractional_and_whole_step_through_mounted_pointer_and_keyboard",
  Accordion: "accordion_result_disclosure_focus_identity_and_disabled_paths",
  TriStateSwitch: "tri_state_switch_value_focus_identity_and_disabled_paths",
  Popover: "a_nested_popover_paints_without_nesting_deferred_draws",
  ColorPicker: "first_mounted_parity_color_picker",
  HoverCard: "first_mounted_parity_hover_card",
  Tooltip:
    "tooltip_hover_focus_escape_and_bubble_reach_mounted_gpui",
  Field: "first_mounted_parity_field",
  FieldSet: "first_mounted_parity_field_set",
  Calendar: "first_mounted_parity_calendar",
  FormActions: "first_mounted_parity_form_actions",
  FormLayout: "first_mounted_parity_form_layout",
  ValidationSummary: "first_mounted_parity_validation_summary",
  CodeInput: "a_grouped_code_input_types_and_completes_through_the_real_tree",
  FileUpload: "a_dropzone_browse_flows_fixture_bytes_through_the_generic_seam",
  LicenceActivation: [
    "licence_activation_key_entry_types_and_emits_through_the_real_tree",
    "licence_activation_machine_name_enter_and_escape_restore_display_focus",
  ],
  LicenceSeats: [
    "licence_seats_release_flows_through_confirm_in_a_mounted_window",
    "licence_seats_seat_row_enter_and_escape_restore_display_focus",
  ],
  LicenceStatus: "licence_status_renders_state_and_authority_reads_in_a_mounted_window",
  ModelConnectionPicker: "model_connection_picker_roving_focus_moves_real_backend_focus",
  ModelConnectionSetup: "model_connection_setup_direct_add_submits_from_choose_in_a_mounted_window",
  ModelConnectionCard: "model_connection_card_closes_and_returns_real_focus_to_the_disclosure",
  ModelCatalogueEditor: "model_catalogue_editor_grabs_moves_and_cancels_in_a_mounted_window",
  Radio: "radio_selects_on_activate_and_does_not_uncheck_itself",
  Select: "select_two_instances_search_pointer_and_dismiss_through_mounted_rebuilds",
  UpdateStatus: "update_status_confirm_then_install_through_the_real_tree",
  UpdateCenter: "update_center_hidden_presence_mounts_nothing_and_open_shows_status",
  SettingsShell: "settings_shell_navigates_and_refused_close_stays_open",
  ResizeHandle: [
    "a_focused_resize_handle_steps_the_pane_and_its_declared_value",
    "a_disabled_resize_handle_takes_no_focus_and_answers_no_key",
  ],
  SplitView: "two_composed_split_views_do_not_share_a_divider_focus_handle",
  Callout: "callout_dismiss_rebuilds_the_host_spec_through_mounted_input",
  RemediationBanner: "remediation_banner_action_and_dismiss_rebuild_the_host_spec",
  ActionDiscoveryPanel: "action_discovery_selection_rebuilds_the_host_spec_through_mounted_input",
  DockRegion: "dock_region_tab_and_collapse_rebuild_the_host_spec_through_mounted_input",
  DetailShell: "first_mounted_parity_detail_shell",
  DetailSectionGroup: "first_mounted_parity_detail_section_group",
  PageHeader: "first_mounted_parity_page_header",
  Region: "first_mounted_parity_region",
  AgentPlan: "agent_plan_decisions_rebuild_the_host_spec_through_mounted_input",
  AgentPlanRecord: "agent_plan_record_disclosure_rebuilds_the_host_spec_through_mounted_input",
  AgentSubagent: "agent_subagent_disclosure_rebuilds_the_host_spec_through_mounted_input",
  ChangedFiles: "changed_files_disclosure_and_selection_rebuild_the_host_spec",
  ToolCall: "tool_call_disclosure_rebuilds_the_host_spec_through_mounted_input",
  ToolCallGroup: "tool_call_group_disclosure_rebuilds_the_host_spec_through_mounted_input",
  AgentTranscript: "agent_transcript_detaches_jumps_and_resumes_following_on_a_real_viewport",
  Stepper: [
    "stepper_selection_and_rerun_reach_separate_mounted_controls",
    "stepper_collapse_stays_independent_in_a_mounted_window",
    "stepper_keyboard_entry_focuses_and_activates_without_a_pointer_press",
    "stepper_summary_takes_keyboard_entry_and_paints_the_inset_ring",
  ],
  BulkActionBar: "gpui_mounted_bulk_action_bar_actions_clear_and_select_all",
  ListCardCounter: "first_mounted_parity_list_card_counter",
  ListContainer: "gpui_mounted_list_container_state_pagination_and_accessible_name",
  LogList: "gpui_mounted_log_list_stream_audit_and_clear_filters",
  ContextMenu:
    "context_menu_open_panel_semantics_activation_and_dismissal_through_mounted_backend",
  Menubar: "menubar_trigger_open_select_and_dismissal_through_mounted_backend",
  NavigationMenu:
    "navigation_menu_disclosure_viewport_roving_and_dismissal_through_mounted_backend",
  SplitButton:
    "split_button_halves_menu_keyboard_and_dismissal_through_mounted_backend",
  AlertDialog: "first_mounted_parity_alert_dialog",
  FormDialog: "first_mounted_parity_form_dialog",
  DebugDialog: "first_mounted_parity_debug_dialog",
  Drawer: "first_mounted_parity_drawer",
  PickerShell: "first_mounted_parity_picker_shell",
  RefSelect: "first_mounted_parity_ref_select",
  RelationPicker: "first_mounted_parity_relation_picker",
  ThemeSelect: "first_mounted_parity_theme_select",
  HistoryCenter: "first_mounted_parity_history_center",
  PageLoading: "first_mounted_parity_page_loading",
  Spinner: "first_mounted_parity_spinner",
  Progress: "first_mounted_parity_progress",
  Meter: "first_mounted_parity_meter",
  AudioMeter: "first_mounted_parity_audio_meter",
  AudioSwitch: "first_mounted_parity_audio_switch",
  GainReductionMeter: "first_mounted_parity_gain_reduction_meter",
  ValueReadout: "first_mounted_parity_value_readout",
  Code: [
    "code_copy_feedback_through_production_app_state_scheduling",
    "first_mounted_parity_code",
  ],
  Eyebrow: "first_mounted_parity_eyebrow",
  TextLink: "first_mounted_parity_text_link",
  IconProvider: "first_mounted_parity_icon_provider",
  MetricTile: "first_mounted_parity_metric_tile",
  StateTile: "first_mounted_parity_state_tile",
  MediaPreview: "first_mounted_parity_media_preview",
  StatusBar: "first_mounted_parity_status_bar",
  ToastStack: [
    "first_mounted_parity_toast_stack",
    "toast_stack_renderer_owned_presence_phases_and_inert_remnant",
    "toast_stack_action_removal_hands_focus_on_or_leaves_it_alone",
  ],
  Separator: "first_mounted_parity_separator",
  Card: "first_mounted_parity_card",
  EmptyState: "first_mounted_parity_empty_state",
  ErrorBoundary: "first_mounted_parity_error_boundary",
  EmbedPreview: "first_mounted_parity_embed_preview",
  InlineListSection: "first_mounted_parity_inline_list_section",
  PasswordRequirements: "first_mounted_parity_password_requirements",
  ScrollShell: "first_mounted_parity_scroll_shell",
  DetailSection: "first_mounted_parity_detail_section",
  UiPresentationProvider: "first_mounted_parity_ui_presentation_provider",
  MotionPolicyProvider: "first_mounted_parity_motion_policy_provider",
  AgentMessage: "first_mounted_parity_agent_message",
  EmbedInput: "embed_input_url_entry_validation_and_preview_rebuild_the_host_spec",
  AgentQuestionRecord: "agent_question_record_answer_display_rebuilds_the_host_spec",
  Keyboard: "keyboard_pointer_computer_key_and_held_notes_rebuild_the_host_spec",
  EnvelopeEditor: "first_mounted_parity_envelope_editor",
  ModMatrixGrid: "first_mounted_parity_mod_matrix_grid",
  WaveformDisplay: "first_mounted_parity_waveform_display",
};

/** The map's size when g16.062 stopped promoting from it. Fixed: the live
 * counts below keep growing and must not rewrite this baseline. */
const HISTORICAL_PROMOTED_COMPONENT_COUNT = 142;
const HISTORICAL_PROMOTED_TEST_COUNT = 158;
const EXPECTED_MOUNTED_COMPONENT_COUNT = Object.keys(EXPECTED_MOUNTED_BEHAVIOUR_TESTS).length;
const EXPECTED_MOUNTED_TEST_COUNT = Object.values(EXPECTED_MOUNTED_BEHAVIOUR_TESTS).reduce(
  (count, tests) => count + (Array.isArray(tests) ? tests.length : 1),
  0,
);

const NUCLEUS_COLUMNS = [
  "Component",
  "Scenario",
  "Direct dependencies",
  "Expected mounted run",
  "M1 execution",
  "A1",
  "V1",
] as const;

function read(root: string, relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function exists(root: string, relativePath: string): boolean {
  return fs.existsSync(path.join(root, relativePath));
}

function isFile(root: string, relativePath: string): boolean {
  try {
    return fs.statSync(path.join(root, relativePath)).isFile();
  } catch {
    return false;
  }
}

function walkFiles(root: string, relativeDirectory: string): string[] {
  const directory = path.join(root, relativeDirectory);
  if (!fs.existsSync(directory)) return [];

  return fs
    .readdirSync(directory, { withFileTypes: true })
    .flatMap((entry) => {
      const relativePath = path.join(relativeDirectory, entry.name);
      return entry.isDirectory() ? walkFiles(root, relativePath) : [relativePath];
    })
    .sort();
}

function toPosix(relativePath: string): string {
  return relativePath.split(path.sep).join("/");
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function toSlug(name: string): string {
  return name
    .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
    .replace(/([A-Z])([A-Z][a-z])/g, "$1-$2")
    .toLowerCase();
}

function toSnake(name: string): string {
  return toSlug(name).replaceAll("-", "_");
}

function pathRef(relativePath: string, fragment?: string): string {
  return `\`${relativePath}${fragment === undefined ? "" : `#${fragment}`}\``;
}

function cell(status: EvidenceStatus, evidence: string): string {
  return `${status} — ${evidence}`;
}

function parseSvelteExports(source: string): Map<string, string> {
  const exports = new Map<string, string>();
  const pattern = /export\s*\{\s*default\s+as\s+(\w+)\s*\}\s+from\s+"\.\/([^"]+\.svelte)"/g;

  for (const match of source.matchAll(pattern)) {
    exports.set(match[1], match[2]);
  }

  return exports;
}

function parseReactExports(source: string): Map<string, string> {
  const exports = new Map<string, string>();
  const pattern = /export\s*\{([\s\S]*?)\}\s*from\s+"\.\/([^"]+)"/g;

  for (const match of source.matchAll(pattern)) {
    const names = new Set(
      [...match[1].matchAll(/\b([A-Z][A-Za-z0-9]*)\b/g)].map((entry) => entry[1]),
    );
    for (const name of names) exports.set(name, match[2]);
  }

  return exports;
}

function parseSveltePublicExports(root: string): Map<string, string> {
  const exports = parseSvelteExports(read(root, "packages/svelte/components/src/index.ts"));
  for (const [name, source] of parseSvelteExports(read(root, "packages/svelte/components/src/markdown.ts"))) {
    exports.set(name, source);
  }
  return exports;
}

function parseReactPublicExports(root: string): Map<string, string> {
  const exports = parseReactExports(read(root, "packages/react/components/src/index.ts"));
  for (const [name, source] of parseReactExports(read(root, "packages/react/components/src/markdown.ts"))) {
    exports.set(name, source);
  }
  return exports;
}

function svelteExportBarrel(root: string, name: string): string {
  const markdown = parseSvelteExports(read(root, "packages/svelte/components/src/markdown.ts"));
  return markdown.has(name)
    ? "packages/svelte/components/src/markdown.ts"
    : "packages/svelte/components/src/index.ts";
}

function reactExportBarrel(root: string, name: string): string {
  const markdown = parseReactExports(read(root, "packages/react/components/src/markdown.ts"));
  return markdown.has(name)
    ? "packages/react/components/src/markdown.ts"
    : "packages/react/components/src/index.ts";
}

export function resolveSourceFile(root: string, directory: string, sourcePath: string): string | undefined {
  const candidates = [
    sourcePath,
    `${sourcePath}.tsx`,
    `${sourcePath}.ts`,
    `${sourcePath}.jsx`,
    `${sourcePath}.js`,
  ];
  return candidates.find((candidate) => isFile(root, `${directory}/${candidate}`));
}

function findFocusedTest(root: string, runtime: "svelte" | "react", name: string): string {
  const directory = runtime === "svelte" ? "packages/svelte/components/test" : "packages/react/components/test";
  const files = walkFiles(root, directory).filter((file) => /\.(ts|tsx|svelte)$/.test(file));
  const escapedName = escapeRegExp(name);
  const directPattern =
    runtime === "svelte"
      ? new RegExp(`from\\s+["']\\.\\./src/${escapedName}\\.svelte["']`)
      : new RegExp(`from\\s+["']\\.\\./src/${escapedName}["']`);
  const barrelPattern = new RegExp(`from\\s+["']\\.\\./src(?:/index)?["']`);
  const componentPattern = new RegExp(`\\b${escapedName}\\b`);
  const matches = files.filter((file) => {
    const source = read(root, file);
    return directPattern.test(source) || (runtime === "react" && barrelPattern.test(source) && componentPattern.test(source));
  });

  if (matches.length === 0) {
    throw new Error(`No focused ${runtime} test import found for ${name}.`);
  }

  matches.sort((left, right) => {
    const leftName = path.basename(left);
    const rightName = path.basename(right);
    const leftDirect = leftName.startsWith(name) ? 0 : 1;
    const rightDirect = rightName.startsWith(name) ? 0 : 1;
    return leftDirect - rightDirect || left.localeCompare(right);
  });
  return toPosix(matches[0]);
}

function findRustSpec(root: string, name: string): string {
  const specName = RUST_SPEC_OVERRIDES[name] ?? `${name}Spec`;
  const files = walkFiles(root, "packages/contracts/components/src").filter((file) => file.endsWith(".rs"));
  const pattern = new RegExp(`\\bpub\\s+struct\\s+${escapeRegExp(specName)}\\b`);
  const match = files.find((file) => pattern.test(read(root, file)));

  if (match === undefined) throw new Error(`No Rust spec found for ${name} (${specName}).`);
  return toPosix(match);
}

function findRenderModule(root: string, name: string): string {
  const module =
    RENDER_MODULE_OVERRIDES[name] ?? (AUDIO_RENDER_COMPONENTS.has(name) ? "audio" : toSnake(name));
  const relativePath = `packages/render/src/${module}.rs`;

  if (!exists(root, relativePath)) throw new Error(`No poodle-render module found for ${name} (${relativePath}).`);
  return relativePath;
}

function parseVisualSkipped(root: string): Set<string> {
  const source = read(root, "test/visual/config.ts");
  const start = source.indexOf("export const SKIPPED");
  const end = source.indexOf("};", start);
  if (start < 0 || end < 0) throw new Error("Could not locate the visual skip list.");
  return new Set([...source.slice(start, end).matchAll(/^\s*"([^"]+)":/gm)].map((match) => match[1]));
}

export function deriveLiveRoster(root = ROOT): LiveComponent[] {
  const svelteExports = parseSveltePublicExports(root);
  const canonicalByName = new Map(canonicalComponents.map((component) => [component.displayName, component]));
  const entries = [...svelteExports.keys()].map((name) => ({
    name,
    slug: canonicalByName.get(name)?.slug ?? toSlug(name),
    portable: canonicalByName.has(name),
  }));

  // The expected public and portable counts come from the generated catalogue;
  // this is the agreement check that the barrel still carries exactly those
  // names plus the declared web-only supplement.
  assertPublicBarrelAgrees(entries.map((entry) => entry.name));

  return entries;
}

function expectedComponentRow(
  root: string,
  component: LiveComponent,
  visualSkipped: Set<string>,
  portableCount: number,
  nucleusRows: Map<string, NucleusReceiptRow>,
): ComponentRow {
  const { name, slug, portable } = component;
  const contractPath = `docs/contracts/components/${slug}.md`;
  const svelteIndexPath = svelteExportBarrel(root, name);
  const svelteSourcePath = `packages/svelte/components/src/${name}.svelte`;
  const svelteRegistryPath = "packages/svelte/preview/src/specimens/registry.ts";
  const reactIndexPath = reactExportBarrel(root, name);
  const reactRegistryPath = "packages/react/preview/src/gallery/specimen-map.ts";
  const gpuiRegistryPath = "packages/gpui/preview/src/specimens/mod.rs";
  const nativeProofPath = "packages/gpui/native-accessibility-proof.json";
  const visualInventoryPath = "test/visual/fixtures/button-visual-inventory.json";
  const visualSummaryPath = "docs/evidence/visual/g15-047-button-comparison/summary.json";
  const visualRunPath = "test/visual/run.ts";

  if (!exists(root, contractPath)) throw new Error(`Missing contract for ${name}: ${contractPath}.`);
  if (!exists(root, svelteSourcePath)) throw new Error(`Missing Svelte implementation for ${name}.`);

  const svelteTestPath = findFocusedTest(root, "svelte", name);
  const reactExports = parseReactPublicExports(root);
  const reactSource = reactExports.get(name);
  if (reactSource === undefined) throw new Error(`React export missing for ${name}.`);
  const reactSourcePath = resolveSourceFile(root, "packages/react/components/src", reactSource);
  if (reactSourcePath === undefined) throw new Error(`React implementation missing for ${name}: ${reactSource}.`);
  const reactTestPath = findFocusedTest(root, "react", name);

  const base: ComponentRow = {
    Component: name,
    Contract: cell("present", `${pathRef(contractPath)}`),
    "Svelte surface": cell(
      "focused",
      `implementation ${pathRef(svelteSourcePath)}; export ${pathRef(svelteIndexPath, name)}; specimen ${pathRef(svelteRegistryPath, slug)}; focused test ${pathRef(svelteTestPath)}`,
    ),
    "React surface": cell(
      "focused",
      `implementation ${pathRef(`packages/react/components/src/${reactSourcePath}`)}; export ${pathRef(reactIndexPath, name)}; specimen ${pathRef(reactRegistryPath, slug)}; focused test ${pathRef(reactTestPath)}`,
    ),
    "Shared Rust surface": "",
    "GPUI construction": "",
    "GPUI mounted behaviour": "",
    "Web accessibility": cell(
      "focused",
      `Svelte axe case ${pathRef("test/a11y/component-a11y.test.ts", `${name} has no axe violations`)}`,
    ),
    "GPUI accessibility": "",
    "Web visual": "",
    "GPUI visual": "",
    "Known deltas": "",
  };

  const contractSource = read(root, contractPath);
  const deltaHeading = contractSource.match(
    /^#{2,4}\s+(?:\d+(?:[a-z])?\.\s+)?(Known Deltas|Known Differences)\s*$/m,
  );
  base["Known deltas"] =
    deltaHeading === null
      ? cell("not-applicable", `no runtime delta section in ${pathRef(contractPath)}`)
      : cell("present", `${pathRef(contractPath, deltaHeading[1])}; status and runtime reason are contract-owned`);

  if (!portable) {
    base["Shared Rust surface"] = cell("not-applicable", `${pathRef(contractPath, "MeterSurface")}; web-only by the fixed native boundary`);
    base["GPUI construction"] = cell("not-applicable", `MeterSurface is excluded from the ${portableCount}-route native probe`);
    base["GPUI mounted behaviour"] = cell("not-applicable", `MeterSurface is web-only and has no GPUI mounted target`);
    base["GPUI accessibility"] = cell("not-applicable", `MeterSurface is web-only and has no GPUI accessibility target`);
    base["GPUI visual"] = cell("not-applicable", `MeterSurface is web-only and has no GPUI pixel target`);
  } else {
    const specPath = findRustSpec(root, name);
    const renderPath = findRenderModule(root, name);
    const gpuiSource = read(root, gpuiRegistryPath);
    const routePattern = new RegExp(`"${escapeRegExp(slug)}"\\s*=>`);
    if (!routePattern.test(gpuiSource)) throw new Error(`GPUI specimen route missing for ${name}: ${slug}.`);

    base["Shared Rust surface"] = cell(
      "present",
      `spec ${pathRef(specPath, RUST_SPEC_OVERRIDES[name] ?? `${name}Spec`)}; renderer ${pathRef(renderPath)}`,
    );
    base["GPUI construction"] = cell(
      "focused",
      `route ${pathRef(gpuiRegistryPath, slug)}; ${pathRef("packages/gpui/preview/src/specimen_probe.rs")} via effigy probe:gpui-specimens (${portableCount}/${portableCount} routes)`,
    );

    const nucleus = nucleusRows.get(name);
    if (nucleus?.receipt !== undefined && nucleus.receiptPath !== undefined) {
      base["GPUI mounted behaviour"] = cell(
        "mounted",
        `validated ${pathRef(nucleus.receiptPath, "proof_level")}; ${pathRef(nucleus.receiptPath, "production_path_observation")}; receipt schema ${NUCLEUS_RECEIPT_SCHEMA}`,
      );
    } else if (nucleus !== undefined) {
      const expected = nucleus.entry.expected_test === null ? "no named test yet" : nucleus.entry.expected_test;
      base["GPUI mounted behaviour"] = cell(
        "missing",
        `expected ${pathRef(NUCLEUS_MANIFEST_PATH, nucleus.entry.scenario_id)} via ${nucleus.entry.expected_selector} (${expected}); no validated M1 receipt`,
      );
    } else {
      const mountedTests = EXPECTED_MOUNTED_BEHAVIOUR_TESTS[name];
      base["GPUI mounted behaviour"] =
        mountedTests === undefined
          ? cell("missing", `no validated M1 receipt; expected map has no entry for ${name}`)
          : cell(
              "missing",
              `expected-only ${
                (Array.isArray(mountedTests) ? mountedTests : [mountedTests])
                  .map((testName) => pathRef("packages/gpui/preview/tests/headless_regressions.rs", testName))
                  .join("; ")
              }; no validated execution receipt`,
            );
    }
    base["GPUI accessibility"] =
      nucleus?.a1Receipt !== undefined && nucleus.a1ReceiptPath !== undefined
        ? cell(
            "mounted",
            `validated ${pathRef(nucleus.a1ReceiptPath, "proof_level")}; ${pathRef(nucleus.a1ReceiptPath, "accessibility")}; paired GPUI node-tree and Svelte DOM snapshots ${pathRef(nucleus.a1Receipt.accessibility?.gpui_snapshot_path ?? "", "nodes")} and ${pathRef(nucleus.a1Receipt.accessibility?.svelte_snapshot_path ?? "", "nodes")} for ${pathRef(nucleus.a1Receipt.accessibility?.scenario_path ?? "", "actions")}; not broad native assistive-technology proof (A2)`,
          )
        : cell(
            "manual",
            `${pathRef(nativeProofPath, "currentPosture")}; spec and bounded mounted evidence are not broad native assistive-technology proof`,
          );
    if (nucleus?.v1Receipt !== undefined && nucleus.v1ReceiptPath !== undefined) {
      const visual = nucleus.v1Receipt;
      const okPairs = visual.pairs.filter((pair) => pair.ok).length;
      const retainedButton =
        name === "Button"
          ? `; retained ${pathRef(visualInventoryPath)} and ${pathRef(visualSummaryPath)} (18 Button fixtures; GPUI capture operator-approved, non-activating, and windowed)`
          : "";
      base["GPUI visual"] = cell(
        "compared",
        `validated ${pathRef(nucleus.v1ReceiptPath, "proof_level")}; Lab run ${visual.lab_bundle.run_id} ${visual.pairs.length} pair verdicts (${okPairs} ok) in ${pathRef(`${visual.lab_bundle.dir}/summary.json`, "runId")}; ${visual.finding_count} findings retained as open evidence${retainedButton}`,
      );
    } else if (name === "Button") {
      base["GPUI visual"] = cell(
        "compared",
        `${pathRef(visualInventoryPath)} and ${pathRef(visualSummaryPath)}; 18 Button fixtures across Svelte, React, and GPUI; GPUI capture is operator-approved, non-activating, and windowed`,
      );
    } else {
      base["GPUI visual"] = cell(
        "missing",
        `Button-only comparison boundary; no GPUI comparison fixture for ${name} in ${pathRef(visualInventoryPath)}`,
      );
    }
  }

  base["Web visual"] =
    name === "Button"
      ? cell(
          "compared",
          `${pathRef(visualInventoryPath)} and ${pathRef(visualSummaryPath)}; Svelte↔React exact comparison for the accepted 18-case Button inventory`,
        )
      : visualSkipped.has(slug)
        ? cell("manual", `${pathRef("test/visual/config.ts")}; SKIPPED includes ${slug}; no deterministic Svelte↔React sweep claim`)
        : cell("focused", `effigy test:visual-sweep via ${pathRef(visualRunPath)}; Svelte↔React route sweep for ${slug}; final visual acceptance remains manual`);

  base["Web accessibility"] =
    name === "MeterSurface"
      ? cell("focused", `Svelte axe case ${pathRef("test/a11y/component-a11y.test.ts", `${name} has no axe violations`)}`)
      : base["Web accessibility"];

  return base;
}

function statusOf(value: string): EvidenceStatus {
  const match = value.match(/^([^\s]+)\s+—\s+/);
  if (match === null || !EVIDENCE_STATUSES.includes(match[1] as EvidenceStatus)) {
    throw new Error(`Unknown evidence status in cell: ${value}`);
  }
  return match[1] as EvidenceStatus;
}

function parseTable(markdown: string, heading: string): string[][] {
  const headingIndex = markdown.indexOf(heading);
  if (headingIndex < 0) throw new Error(`Missing ledger section ${heading}.`);
  const lines = markdown.slice(headingIndex).split(/\r?\n/);
  const headerIndex = lines.findIndex((line) => line.startsWith("| "));
  if (headerIndex < 0) throw new Error(`Missing table under ${heading}.`);

  const rows: string[][] = [];
  for (const line of lines.slice(headerIndex)) {
    if (!line.startsWith("| ")) {
      if (rows.length > 0) break;
      continue;
    }
    if (/^\|\s*-+/.test(line)) continue;
    const cells = line
      .split("|")
      .slice(1, -1)
      .map((value) => value.trim());
    if (cells.length > 0) rows.push(cells);
  }
  return rows;
}

function parseComponentRows(markdown: string): ComponentRow[] {
  const rows = parseTable(markdown, "## Component evidence ledger");
  if (rows.length === 0) throw new Error("The component evidence table is empty.");
  const header = rows[0];
  if (header.join("|") !== COMPONENT_COLUMNS.join("|")) {
    throw new Error(`Unexpected component ledger columns: ${header.join(" | ")}.`);
  }

  return rows.slice(1).map((values) => {
    if (values.length !== COMPONENT_COLUMNS.length) {
      throw new Error(`Component ledger row has ${values.length} cells; expected ${COMPONENT_COLUMNS.length}.`);
    }
    return Object.fromEntries(COMPONENT_COLUMNS.map((column, index) => [column, values[index]])) as ComponentRow;
  });
}

function parseSummary(markdown: string): Map<string, Record<string, number>> {
  const rows = parseTable(markdown, "## Summary");
  if (rows.length === 0) throw new Error("The evidence summary is empty.");
  const header = rows[0];
  const statuses = header.slice(1) as EvidenceStatus[];
  const summary = new Map<string, Record<string, number>>();

  for (const values of rows.slice(1)) {
    if (values.length !== header.length) throw new Error(`Summary row has ${values.length} cells; expected ${header.length}.`);
    const counts: Record<string, number> = {};
    statuses.forEach((status, index) => {
      const value = Number(values[index + 1]);
      if (!Number.isInteger(value) || value < 0) throw new Error(`Invalid summary count for ${values[0]} / ${status}.`);
      counts[status] = value;
    });
    summary.set(values[0], counts);
  }

  return summary;
}

type EvidenceReference = { path: string; fragment?: string };

function referencedPaths(value: string): EvidenceReference[] {
  return [...value.matchAll(/`((?:docs|packages|scripts|test|tasks)\/[^`#;]+)(?:#([^`]*))?`/g)].map((match) => ({
    path: match[1],
    ...(match[2] === undefined ? {} : { fragment: match[2] }),
  }));
}

function evidenceReferenceExists(root: string, reference: EvidenceReference): "path" | "fragment" | undefined {
  if (!exists(root, reference.path)) return "path";
  if (reference.fragment === undefined) return undefined;

  const source = read(root, reference.path);
  if (source.includes(reference.fragment)) return undefined;
  if (reference.fragment.endsWith(" has no axe violations") && source.includes("has no axe violations")) return undefined;
  return "fragment";
}

function expectedSummary(rows: ComponentRow[]): Map<string, Record<string, number>> {
  const summary = new Map<string, Record<string, number>>();
  for (const column of COMPONENT_COLUMNS.slice(1)) {
    const counts: Record<string, number> = Object.fromEntries(EVIDENCE_STATUSES.map((status) => [status, 0]));
    for (const row of rows) counts[statusOf(row[column])] += 1;
    summary.set(column, counts);
  }
  return summary;
}

function summaryMarkdown(rows: ComponentRow[]): string {
  const summary = expectedSummary(rows);
  const headers = ["Claim", ...EVIDENCE_STATUSES];
  const lines = [`| ${headers.join(" | ")} |`, `| ${headers.map(() => "---").join(" | ")} |`];
  for (const [claim, counts] of summary) {
    lines.push(`| ${claim} | ${EVIDENCE_STATUSES.map((status) => counts[status]).join(" | ")} |`);
  }
  return lines.join("\n");
}

export function deriveRows(root = ROOT, nucleusRows = deriveNucleusReceiptRows(root)): ComponentRow[] {
  const roster = deriveLiveRoster(root);
  const svelteExports = parseSveltePublicExports(root);
  const svelteRegistry = read(root, "packages/svelte/preview/src/specimens/registry.ts");
  const reactRegistry = read(root, "packages/react/preview/src/gallery/specimen-map.ts");
  const visualSkipped = parseVisualSkipped(root);

  for (const component of roster) {
    if (!svelteExports.has(component.name)) throw new Error(`Svelte export disappeared for ${component.name}.`);
    const slugPattern = new RegExp(`["']?${escapeRegExp(component.slug)}["']?\\s*:`);
    if (!slugPattern.test(svelteRegistry)) throw new Error(`Svelte specimen route missing for ${component.name}.`);
    if (!slugPattern.test(reactRegistry)) throw new Error(`React specimen route missing for ${component.name}.`);
  }

  const portableCount = roster.filter((component) => component.portable).length;
  const nucleusByName = new Map(nucleusRows.map((row) => [row.entry.name, row]));
  return roster.map((component) => expectedComponentRow(root, component, visualSkipped, portableCount, nucleusByName));
}

function rowMarkdown(row: ComponentRow): string {
  return `| ${COMPONENT_COLUMNS.map((column) => row[column]).join(" | ")} |`;
}

function nucleusRow(nucleus: NucleusReceiptRow): string[] {
  const { entry, receipt, receiptPath, a1Receipt, a1ReceiptPath, v1Receipt, v1ReceiptPath } = nucleus;
  return [
    entry.name,
    pathRef(NUCLEUS_MANIFEST_PATH, entry.scenario_id),
    entry.direct_dependencies.length === 0 ? "none" : entry.direct_dependencies.join(", "),
    entry.expected_test === null
      ? `${entry.expected_selector}; no named test yet`
      : `${entry.expected_selector}; ${pathRef("packages/gpui/preview/tests/headless_regressions.rs", entry.expected_test)}`,
    receipt === undefined || receiptPath === undefined
      ? cell("missing", "no validated execution receipt")
      : cell("mounted", `validated ${pathRef(receiptPath, "proof_level")}; ${pathRef(receiptPath, "outcome")}`),
    a1Receipt === undefined || a1ReceiptPath === undefined
      ? cell("missing", "M1 does not infer executable accessibility semantics; no validated A1 receipt")
      : cell("mounted", `validated ${pathRef(a1ReceiptPath, "proof_level")}; ${pathRef(a1ReceiptPath, "accessibility")}`),
    v1Receipt === undefined || v1ReceiptPath === undefined
      ? cell("missing", "M1 does not infer visual comparison; no validated V1 receipt")
      : cell(
          "compared",
          `validated ${pathRef(v1ReceiptPath, "proof_level")}; Lab run ${v1Receipt.lab_bundle.run_id} ${v1Receipt.pairs.length} pair verdicts in ${pathRef(`${v1Receipt.lab_bundle.dir}/summary.json`, "runId")}; ${v1Receipt.finding_count} findings retained as open evidence`,
        ),
  ];
}

function nucleusTable(rows: NucleusReceiptRow[]): string {
  const header = `| ${NUCLEUS_COLUMNS.join(" | ")} |`;
  const separator = `| ${NUCLEUS_COLUMNS.map(() => "---").join(" | ")} |`;
  return [header, separator, ...rows.map((row) => `| ${nucleusRow(row).join(" | ")} |`)].join("\n");
}

function expectedMapTable(): string {
  const rows = Object.entries(EXPECTED_MOUNTED_BEHAVIOUR_TESTS).map(([component, tests]) => {
    const names = Array.isArray(tests) ? tests : [tests];
    return `| ${component} | ${names.map((testName) => pathRef("packages/gpui/preview/tests/headless_regressions.rs", testName)).join("; ")} | expected only |`;
  });
  return [
    "| Component | Expected test name(s) | Execution status |",
    "| --- | --- | --- |",
    ...rows,
  ].join("\n");
}

function evidenceUpdatedDate(nucleusRows: NucleusReceiptRow[]): string {
  const days = nucleusRows
    .map((row) => row.v1Receipt?.lab_bundle.run_id.slice(0, 10))
    .filter((day): day is string => Boolean(day) && /^\d{4}-\d{2}-\d{2}$/.test(day));
  days.sort();
  return days.at(-1) ?? "1970-01-01";
}

/** Drop the evidence-derived Updated line so reproduction fails on cells, not the header date. */
export function ledgerComparableText(markdown: string): string {
  return markdown.replace(/^Updated: \d{4}-\d{2}-\d{2}\n/m, "");
}

export function generateLedgerMarkdown(root = ROOT): string {
  const nucleusRows = deriveNucleusReceiptRows(root);
  const rows = deriveRows(root, nucleusRows);
  const componentRows = rows.map(rowMarkdown).join("\n");
  const roster = deriveLiveRoster(root);
  const publicCount = roster.length;
  const portableCount = roster.filter((component) => component.portable).length;
  return `# g16.001 — Active-Cohort Parity Evidence Ledger

Status: current evidence snapshot
Updated: ${evidenceUpdatedDate(nucleusRows)}
Source: live public Svelte exports, generated portable catalogue, runtime registries, focused tests, validated execution receipts, and retained g15 evidence

## Purpose

This ledger records what Poodle proves today for the active cohort: Svelte,
React, shared Rust composition, and GPUI. It separates implementation presence,
focused tests, mounted behaviour, accessibility, and visual comparison. A
specimen route proves construction only; a focused test or expected test name
proves no mounted execution; one runtime's evidence never transfers to another
runtime.

## Denominator

- Public Svelte components: **${publicCount}**, derived from
  \`packages/svelte/components/src/index.ts\`.
- Portable native components: **${portableCount}**, derived from the generated catalogue.
- Native \`not-applicable\`: **MeterSurface** only, by the fixed web-only
  boundary. It remains in the ${publicCount}-component public denominator.
- Jetstream: one program-level \`deferred\` target. Shared Rust composition and
  the in-repo adapter do not make the sibling backend pass.

## Evidence vocabulary

| Status | Meaning |
| --- | --- |
| \`present\` | The named implementation, export, contract, or structural surface exists. |
| \`focused\` | A named owner-local test or bounded probe proves one scoped claim. |
| \`mounted\` | A named test drives the real runtime tree. |
| \`compared\` | A named cross-runtime comparison has a fixed inventory and evidence. |
| \`manual\` | Human review remains required; no automated pass is claimed. |
| \`missing\` | Required active-cohort evidence is absent. |
| \`not-applicable\` | The contract-approved runtime exclusion applies. |
| \`deferred\` | The target is outside the active cohort by program decision. |

## Summary

${summaryMarkdown(rows)}

## Runtime posture

| Runtime | Posture | Evidence |
| --- | --- | --- |
| Svelte | reference implementation; focused component tests and Svelte axe sweep are present | \`test/a11y/component-a11y.test.ts\` |
| React | implementation and focused tests are present; React axe sweep is missing | no React axe equivalent; Svelte axe evidence does not transfer |
| Shared Rust | ${portableCount} renderer-neutral surfaces present; MeterSurface is not-applicable | \`packages/contracts/components/src/\`; \`packages/render/src/\` |
| GPUI | ${portableCount}/${portableCount} portable specimen routes construct headlessly; mounted behaviour is bounded | \`packages/gpui/preview/src/specimen_probe.rs\`; \`packages/gpui/preview/tests/headless_regressions.rs\` |
| Jetstream | deferred at program level | \`packages/jetstream/cross-runtime-parity-report.json\` |

## Component evidence ledger

| ${COMPONENT_COLUMNS.join(" | ")} |
| ${COMPONENT_COLUMNS.map(() => "---").join(" | ")} |
${componentRows}

## Nucleus fixed cohort execution ledger

The Nucleus denominator is **29 rendered components**. \`IconProvider\` is one
separate construction prerequisite and is not row 30. A row is \`mounted\` only
when its validated receipt was emitted after the real GPUI render, node backend,
or \`V1\`. An \`A1\` row is \`mounted\` only when its validated paired receipt
(g16.111) records an empty diff between the mounted GPUI node-tree projection
and the mounted Svelte DOM for the same shared scenario file. A \`V1\` row is
\`compared\` only when its validated receipt (g17.001) cites the pinned Lab
bundle by run id, validator version, and directory hash and retains every
reported finding as open evidence; findings adjudicate nothing.

Manifest: ${pathRef(NUCLEUS_MANIFEST_PATH)}; receipt schema: \`${NUCLEUS_RECEIPT_SCHEMA}\`.
Poodle resolution: \`${nucleusRows[0]?.receipt?.package ?? "poodle-gpui-preview"}@${nucleusRows[0]?.receipt?.package_version ?? "0.3.0"}\`;
source commit and Cargo.lock resolution are pinned in the manifest. A run that
uses a published package must produce a separate resolution; this workspace
receipt does not claim publication.

${nucleusTable(nucleusRows)}

## Historical mounted expectation map

Before g16.062, the generator promoted **${HISTORICAL_PROMOTED_COMPONENT_COUNT}
component entries across ${HISTORICAL_PROMOTED_TEST_COUNT} named tests** to
\`mounted\`. The map remains planning input and is shown for traceability only;
the current component ledger consumes validated receipts, so expected-without-
receipt remains \`missing\`.
The map now lists ${EXPECTED_MOUNTED_COMPONENT_COUNT} component entries across
${EXPECTED_MOUNTED_TEST_COUNT} named tests.

${expectedMapTable()}

## Limitations and measured next gaps

- GPUI mounted behaviour is the named regression set, not a ${portableCount}-component
  behaviour pass. Expected test names are not execution evidence; the fixed
  Nucleus cohort advances only from validated receipts.
- GPUI accessibility remains manual except where a validated \`A1\` receipt
  pairs the mounted node-tree projection with the Svelte DOM for one scenario;
  shared specs, bounded mounted tests, and \`A1\` do not prove broad native
  semantics, announcement, or assistive-technology parity (\`A2\`).
- Web accessibility is asymmetric: the Svelte axe sweep covers the live Svelte
  surface; no React axe sweep currently exists.
- Outside the Nucleus V1 cohort, the accepted three-runtime visual comparison
  is Button-only: 18 named fixtures across Svelte, React, and GPUI. GPUI pixels
  require the operator-approved, non-activating windowed diagnostic and are
  absent from default QA/CI. The 29-row Nucleus V1 cohort adds 58 fixtures × 3
  runtimes from Lab run 2026-09-08T14-06-48 (174 captures, 116 comparisons);
  its 160 findings remain open evidence and adjudicate nothing.
- The next evidence decision should be chosen from the measured missing cells:
  semantic/interface, mounted behaviour, accessibility, web visual, or GPUI
  visual. \`g16.002\` closed three selection-control mounted rows. \`g16.003\`
  closed RadioGroup's GPUI mounted-behaviour cell after host-owned native
  identity landed. \`g16.004\` closed ToggleGroup after resulting-selection
  payloads, single-mode roving focus, and instance-scoped native identity
  landed. \`g16.005\` closed Slider axis, keyboard, and mounted parity.
  \`g16.006\` closed Tabs drag, keyboard, and mounted parity. This ledger
  does not compile another card or choose a visual-fixture lane.

## Jetstream posture

| Target | Status | Boundary |
| --- | --- | --- |
| Jetstream backend admission | \`deferred\` | The sibling converter, input, accessibility, preview, and visual programme remains outside the active cohort. |

## Checker

Run \`effigy check:parity-evidence-ledger\` to derive the roster, validate every
row and evidence reference, and verify that summary counts match the rows. The
checker intentionally fails on missing, extra, duplicate, or unresolved
component evidence rather than treating a specimen or another runtime's proof
as a pass. The Nucleus receipt directory is the only committed execution
input for this foundation; unmanifested receipts are rejected.
`;
}

export function validateLedgerText(markdown: string, root = ROOT): void {
  const errors: string[] = [];
  let expectedRows: ComponentRow[];
  let expectedNucleusRows: NucleusReceiptRow[];
  try {
    expectedRows = deriveRows(root);
    expectedNucleusRows = deriveNucleusReceiptRows(root);
  } catch (error) {
    throw new Error(error instanceof Error ? error.message : String(error));
  }

  let actualRows: ComponentRow[];
  try {
    actualRows = parseComponentRows(markdown);
  } catch (error) {
    throw new Error(error instanceof Error ? error.message : String(error));
  }

  const expectedNames = expectedRows.map((row) => row.Component);
  const actualNames = actualRows.map((row) => row.Component);
  const expectedSet = new Set(expectedNames);
  const actualSet = new Set(actualNames);
  const duplicates = actualNames.filter((name, index) => actualNames.indexOf(name) !== index);
  const missing = expectedNames.filter((name) => !actualSet.has(name));
  const extra = actualNames.filter((name) => !expectedSet.has(name));
  if (duplicates.length > 0) errors.push(`duplicate component rows: ${[...new Set(duplicates)].join(", ")}`);
  if (missing.length > 0) errors.push(`missing component rows: ${missing.join(", ")}`);
  if (extra.length > 0) errors.push(`extra component rows: ${extra.join(", ")}`);

  const expectedByName = new Map(expectedRows.map((row) => [row.Component, row]));
  for (const row of actualRows) {
    for (const column of COMPONENT_COLUMNS.slice(1)) {
      try {
        statusOf(row[column]);
      } catch (error) {
        errors.push(`${row.Component}/${column}: ${error instanceof Error ? error.message : String(error)}`);
      }
      for (const reference of referencedPaths(row[column])) {
        const failure = evidenceReferenceExists(root, reference);
        if (failure === "path") errors.push(`${row.Component}/${column}: unresolved evidence path ${reference.path}`);
        if (failure === "fragment") {
          errors.push(`${row.Component}/${column}: unresolved evidence reference ${reference.path}#${reference.fragment}`);
        }
      }
    }

    const expected = expectedByName.get(row.Component);
    if (expected !== undefined) {
      for (const column of COMPONENT_COLUMNS.slice(1)) {
        if (row[column] !== expected[column]) {
          errors.push(`${row.Component}/${column}: ledger cell differs from live evidence or contains an unresolved claim`);
        }
      }
    }
  }

  try {
    const actualNucleusRows = parseTable(markdown, "## Nucleus fixed cohort execution ledger");
    const expectedHeader = NUCLEUS_COLUMNS.join("|");
    if (actualNucleusRows.length === 0 || actualNucleusRows[0].join("|") !== expectedHeader) {
      errors.push("Nucleus receipt ledger has unexpected columns");
    } else {
      const actualValues = actualNucleusRows.slice(1);
      const expectedValues = expectedNucleusRows.map((row) => nucleusRow(row));
      if (actualValues.length !== expectedValues.length) {
        errors.push(`Nucleus receipt ledger has ${actualValues.length} rows; expected ${expectedValues.length}`);
      }
      for (let index = 0; index < Math.max(actualValues.length, expectedValues.length); index += 1) {
        if (actualValues[index]?.join("|") !== expectedValues[index]?.join("|")) {
          errors.push(`Nucleus receipt ledger row ${index + 1} differs from the manifest or validated receipts`);
        }
      }
    }
  } catch (error) {
    errors.push(error instanceof Error ? error.message : String(error));
  }

  try {
    const actualSummary = parseSummary(markdown);
    const expectedSummaryValues = expectedSummary(expectedRows);
    for (const [claim, expected] of expectedSummaryValues) {
      const actual = actualSummary.get(claim);
      if (actual === undefined) {
        errors.push(`missing summary row: ${claim}`);
        continue;
      }
      for (const status of EVIDENCE_STATUSES) {
        if ((actual[status] ?? 0) !== expected[status]) {
          errors.push(`summary drift for ${claim}/${status}: expected ${expected[status]}, found ${actual[status] ?? 0}`);
        }
      }
    }
  } catch (error) {
    errors.push(error instanceof Error ? error.message : String(error));
  }

  const portableCount = deriveLiveRoster(root).filter((component) => component.portable).length;
  const requiredPhrases = [
    `${portableCount}/${portableCount} portable specimen routes construct headlessly`,
    "29 rendered components",
    "expected-without-",
    "Button-only",
    "non-activating windowed",
    "Svelte axe evidence does not transfer",
  ];
  for (const phrase of requiredPhrases) {
    if (!markdown.includes(phrase)) errors.push(`missing limitations statement: ${phrase}`);
  }
  if (!/\| Jetstream backend admission \| `deferred` \|/.test(markdown)) {
    errors.push("Jetstream posture is not program-level deferred.");
  }

  if (errors.length > 0) throw new Error(errors.join("\n"));
}

function main(): void {
  const ledgerPath = path.join(ROOT, LEDGER_PATH);
  if (process.argv.includes("--write")) {
    fs.writeFileSync(ledgerPath, generateLedgerMarkdown(ROOT));
    console.log(`Wrote ${LEDGER_PATH}.`);
    return;
  }

  const committed = fs.readFileSync(ledgerPath, "utf8");
  validateLedgerText(committed, ROOT);
  const generated = generateLedgerMarkdown(ROOT);
  if (ledgerComparableText(generated) !== ledgerComparableText(committed)) {
    throw new Error(
      `${LEDGER_PATH} differs from live evidence. Reproduction ignores the evidence-derived Updated header; a cell change must match generateLedgerMarkdown.`,
    );
  }
  console.log(`Validated ${deriveLiveRoster(ROOT).length} component evidence rows in ${LEDGER_PATH}.`);
}

if (import.meta.main) main();
