import fs from "node:fs";
import path from "node:path";
import { canonicalComponents } from "../packages/svelte/preview/src/generated/catalogue/catalogue";

export const LEDGER_PATH = "docs/roadmaps/g16/parity-evidence-ledger.md";

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
  TimeInput: "TimeFieldSpec",
};

const RENDER_MODULE_OVERRIDES: Record<string, string> = {
  Box: "bx",
  StatusBar: "shell_status_bar",
  TimeInput: "time_field",
  UiPresentationProvider: "context",
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

const MOUNTED_BEHAVIOUR_TESTS: Record<string, string | string[]> = {
  Button: "a_mounted_button_carries_its_controls_target",
  Checkbox: "checkbox_toggle_readonly_and_disabled_rebuild_the_host_spec",
  Switch: "switch_toggle_readonly_and_disabled_rebuild_the_host_spec",
  SegmentedControl: "segmented_control_exclusive_focus_identity_and_disabled_paths",
  RadioGroup: "radio_group_exclusive_focus_identity_and_disabled_paths",
  ToggleGroup: "toggle_group_result_focus_identity_and_disabled_paths",
  RangeSlider: "a_scrub_reports_change_while_dragging_and_commits_once_at_release",
  Slider: "slider_axis_keyboard_and_disabled_rebuild_the_host_spec",
  Tabs: "tabs_drag_keyboard_and_identity_rebuild_the_host_spec",
  TextInput: "text_input_controlled_editing_and_identity_rebuild_the_host_spec",
  DurationInput: "duration_input_segments_edit_and_rebuild_the_host_spec",
  Breadcrumbs: "breadcrumbs_callback_navigation_through_mounted_pointer_and_keyboard",
  IconButton: "icon_button_activation_toggle_and_tooltip_through_mounted_pointer_and_keyboard",
  Collapsible: "collapsible_disclosure_and_identity_through_mounted_pointer_and_keyboard",
  Accordion: "accordion_result_disclosure_focus_identity_and_disabled_paths",
  TriStateSwitch: "tri_state_switch_value_focus_identity_and_disabled_paths",
  Popover: "a_nested_popover_paints_without_nesting_deferred_draws",
  CodeInput: "a_grouped_code_input_types_and_completes_through_the_real_tree",
  FileUpload: "a_dropzone_browse_flows_fixture_bytes_through_the_generic_seam",
  LicenceActivation: "licence_activation_key_entry_types_and_emits_through_the_real_tree",
  LicenceSeats: "licence_seats_release_flows_through_confirm_in_a_mounted_window",
  LicenceStatus: "licence_status_renders_state_and_authority_reads_in_a_mounted_window",
  ModelConnectionPicker: "model_connection_picker_roving_focus_moves_real_backend_focus",
  ModelConnectionSetup: "model_connection_setup_direct_add_submits_from_choose_in_a_mounted_window",
  ModelConnectionCard: "model_connection_card_closes_and_returns_real_focus_to_the_disclosure",
  ModelCatalogueEditor: "model_catalogue_editor_grabs_moves_and_cancels_in_a_mounted_window",
  Radio: "radio_selects_on_activate_and_does_not_uncheck_itself",
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
  AgentPlan: "agent_plan_decisions_rebuild_the_host_spec_through_mounted_input",
  AgentPlanRecord: "agent_plan_record_disclosure_rebuilds_the_host_spec_through_mounted_input",
  AgentSubagent: "agent_subagent_disclosure_rebuilds_the_host_spec_through_mounted_input",
  ChangedFiles: "changed_files_disclosure_and_selection_rebuild_the_host_spec",
  ToolCall: "tool_call_disclosure_rebuilds_the_host_spec_through_mounted_input",
  ToolCallGroup: "tool_call_group_disclosure_rebuilds_the_host_spec_through_mounted_input",
  Stepper: [
    "stepper_selection_and_rerun_reach_separate_mounted_controls",
    "stepper_collapse_stays_independent_in_a_mounted_window",
    "stepper_keyboard_entry_focuses_and_activates_without_a_pointer_press",
    "stepper_summary_takes_keyboard_entry_and_paints_the_inset_ring",
  ],
};

function read(root: string, relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function exists(root: string, relativePath: string): boolean {
  return fs.existsSync(path.join(root, relativePath));
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

function resolveSourceFile(root: string, directory: string, sourcePath: string): string | undefined {
  const candidates = [
    sourcePath,
    `${sourcePath}.tsx`,
    `${sourcePath}.ts`,
    `${sourcePath}.jsx`,
    `${sourcePath}.js`,
  ];
  return candidates.find((candidate) => exists(root, `${directory}/${candidate}`));
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
  const svelteExports = parseSvelteExports(read(root, "packages/svelte/components/src/index.ts"));
  const canonicalByName = new Map(canonicalComponents.map((component) => [component.displayName, component]));
  const entries = [...svelteExports.keys()].map((name) => ({
    name,
    slug: canonicalByName.get(name)?.slug ?? toSlug(name),
    portable: canonicalByName.has(name),
  }));
  const names = new Set(entries.map((entry) => entry.name));
  const nativeExclusions = entries.filter((entry) => !entry.portable);

  if (entries.length !== 175) {
    throw new Error(`Expected 175 public Svelte components, found ${entries.length}.`);
  }
  if (canonicalComponents.length !== 174) {
    throw new Error(`Expected 174 portable catalogue components, found ${canonicalComponents.length}.`);
  }
  if (nativeExclusions.length !== 1 || nativeExclusions[0].name !== "MeterSurface") {
    throw new Error(
      `Expected MeterSurface as the sole native exclusion, found ${nativeExclusions.map((entry) => entry.name).join(", ")}.`,
    );
  }
  if (names.size !== entries.length) throw new Error("The public Svelte component export list contains duplicates.");

  const canonicalNames = new Set(canonicalComponents.map((component) => component.displayName));
  const missingCanonical = [...canonicalNames].filter((name) => !names.has(name));
  if (missingCanonical.length > 0) {
    throw new Error(`Portable catalogue components missing from the public Svelte index: ${missingCanonical.join(", ")}.`);
  }

  return entries;
}

function expectedComponentRow(root: string, component: LiveComponent, visualSkipped: Set<string>): ComponentRow {
  const { name, slug, portable } = component;
  const contractPath = `docs/contracts/components/${slug}.md`;
  const svelteIndexPath = "packages/svelte/components/src/index.ts";
  const svelteSourcePath = `packages/svelte/components/src/${name}.svelte`;
  const svelteRegistryPath = "packages/svelte/preview/src/specimens/registry.ts";
  const reactIndexPath = "packages/react/components/src/index.ts";
  const reactRegistryPath = "packages/react/preview/src/gallery/specimen-map.ts";
  const gpuiRegistryPath = "packages/gpui/preview/src/specimens/mod.rs";
  const nativeProofPath = "packages/gpui/native-accessibility-proof.json";
  const visualInventoryPath = "test/visual/fixtures/button-visual-inventory.json";
  const visualSummaryPath = "docs/logs/2026-08/assets/g15-047/summary.json";
  const visualRunPath = "test/visual/run.ts";

  if (!exists(root, contractPath)) throw new Error(`Missing contract for ${name}: ${contractPath}.`);
  if (!exists(root, svelteSourcePath)) throw new Error(`Missing Svelte implementation for ${name}.`);

  const svelteTestPath = findFocusedTest(root, "svelte", name);
  const reactExports = parseReactExports(read(root, reactIndexPath));
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
    base["GPUI construction"] = cell("not-applicable", `MeterSurface is excluded from the 174-route native probe`);
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
      `route ${pathRef(gpuiRegistryPath, slug)}; ${pathRef("packages/gpui/preview/src/specimen_probe.rs")} via effigy probe:gpui-specimens (174/174 routes)`,
    );

    const mountedTests = MOUNTED_BEHAVIOUR_TESTS[name];
    base["GPUI mounted behaviour"] =
      mountedTests === undefined
        ? cell("missing", `no named mounted regression in ${pathRef("packages/gpui/preview/tests/headless_regressions.rs")}`)
        : cell(
            "mounted",
            (Array.isArray(mountedTests) ? mountedTests : [mountedTests])
              .map((testName) => pathRef("packages/gpui/preview/tests/headless_regressions.rs", testName))
              .join("; "),
          );
    base["GPUI accessibility"] = cell(
      "manual",
      `${pathRef(nativeProofPath, "currentPosture")}; spec and bounded mounted evidence are not broad native assistive-technology proof`,
    );
    base["GPUI visual"] =
      name === "Button"
        ? cell(
            "compared",
            `${pathRef(visualInventoryPath)} and ${pathRef(visualSummaryPath)}; 18 Button fixtures across Svelte, React, and GPUI; GPUI capture is operator-approved, non-activating, and windowed`,
          )
        : cell(
            "missing",
            `Button-only comparison boundary; no GPUI comparison fixture for ${name} in ${pathRef(visualInventoryPath)}`,
          );
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

export function deriveRows(root = ROOT): ComponentRow[] {
  const roster = deriveLiveRoster(root);
  const svelteExports = parseSvelteExports(read(root, "packages/svelte/components/src/index.ts"));
  const svelteRegistry = read(root, "packages/svelte/preview/src/specimens/registry.ts");
  const reactRegistry = read(root, "packages/react/preview/src/gallery/specimen-map.ts");
  const visualSkipped = parseVisualSkipped(root);

  for (const component of roster) {
    if (!svelteExports.has(component.name)) throw new Error(`Svelte export disappeared for ${component.name}.`);
    const slugPattern = new RegExp(`["']?${escapeRegExp(component.slug)}["']?\\s*:`);
    if (!slugPattern.test(svelteRegistry)) throw new Error(`Svelte specimen route missing for ${component.name}.`);
    if (!slugPattern.test(reactRegistry)) throw new Error(`React specimen route missing for ${component.name}.`);
  }

  return roster.map((component) => expectedComponentRow(root, component, visualSkipped));
}

function rowMarkdown(row: ComponentRow): string {
  return `| ${COMPONENT_COLUMNS.map((column) => row[column]).join(" | ")} |`;
}

export function generateLedgerMarkdown(root = ROOT): string {
  const rows = deriveRows(root);
  const componentRows = rows.map(rowMarkdown).join("\n");
  return `# g16.001 — Active-Cohort Parity Evidence Ledger

Status: current evidence snapshot
Updated: 2026-08-26
Source: live public Svelte exports, generated portable catalogue, runtime registries, focused tests, and retained g15 evidence

## Purpose

This ledger records what Poodle proves today for the active cohort: Svelte,
React, shared Rust composition, and GPUI. It separates implementation presence,
focused tests, mounted behaviour, accessibility, and visual comparison. A
specimen route proves construction only; a focused test proves only its named
claim; one runtime's evidence never transfers to another runtime.

## Denominator

- Public Svelte components: **175**, derived from
  \`packages/svelte/components/src/index.ts\`.
- Portable native components: **174**, derived from the generated catalogue.
- Native \`not-applicable\`: **MeterSurface** only, by the fixed web-only
  boundary. It remains in the 175-component public denominator.
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
| Shared Rust | 174 renderer-neutral surfaces present; MeterSurface is not-applicable | \`packages/contracts/components/src/\`; \`packages/render/src/\` |
| GPUI | 174/174 portable specimen routes construct headlessly; mounted behaviour is bounded | \`packages/gpui/preview/src/specimen_probe.rs\`; \`packages/gpui/preview/tests/headless_regressions.rs\` |
| Jetstream | deferred at program level | \`packages/jetstream/cross-runtime-parity-report.json\` |

## Component evidence ledger

| ${COMPONENT_COLUMNS.join(" | ")} |
| ${COMPONENT_COLUMNS.map(() => "---").join(" | ")} |
${componentRows}

## Limitations and measured next gaps

- GPUI mounted behaviour is the named regression set, not a 174-component
  behaviour pass.
- GPUI accessibility remains manual: shared specs and bounded mounted tests do
  not prove broad native semantics, focus, keyboard, announcement, or
  assistive-technology parity.
- Web accessibility is asymmetric: the Svelte axe sweep covers the live Svelte
  surface; no React axe sweep currently exists.
- The accepted three-runtime visual comparison is Button-only: 18 named
  fixtures across Svelte, React, and GPUI. GPUI pixels require the
  operator-approved, non-activating windowed diagnostic and are absent from
  default QA/CI.
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
as a pass.
`;
}

export function validateLedgerText(markdown: string, root = ROOT): void {
  const errors: string[] = [];
  let expectedRows: ComponentRow[];
  try {
    expectedRows = deriveRows(root);
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

  const requiredPhrases = [
    "174/174 portable specimen routes construct headlessly",
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

  validateLedgerText(fs.readFileSync(ledgerPath, "utf8"), ROOT);
  console.log(`Validated ${deriveLiveRoster(ROOT).length} component evidence rows in ${LEDGER_PATH}.`);
}

if (import.meta.main) main();
