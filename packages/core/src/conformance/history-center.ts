/**
 * HistoryCenter portable interface — composite profile (g14.007).
 *
 * Contract: `docs/contracts/components/history-center.md`. This is the first
 * host-coordinated composite in the conformance kernel: the component owns no
 * history, validates no protocol rule, and decides nothing about undo. Data
 * arrives as `pages`; every operation leaves as a named command and comes back
 * as host-supplied records.
 *
 * Three things make that expressible without a second conformance
 * architecture, and all three are generic kernel vocabulary:
 *
 * - **Structured fixture data.** `pages` is a collection whose `entries` field
 *   is itself a collection, so the page → entry shape the authority hands over
 *   survives into the fixture instead of being flattened into something the
 *   component never sees. One nesting level; deeper is an authoring error.
 * - **Host record channels.** `continuations` and `runEntries` are the data a
 *   fixture host answers `loadContinuations` and `loadContinuationRun` from.
 *   They are declared — so their identity is validated and repeated parts key
 *   over them — but they are not portable props: the component receives them
 *   through its callbacks' results, never as inputs, so they generate no Rust
 *   spec field.
 * - **Repeat sources.** A row is a row whether it came from the spine the host
 *   passed in or from a run the host answered with, so `row`, `entry` and
 *   `disclosure` key over both sources into one identity space. A fork run
 *   entry that repeated a spine entry id is rejected at authoring — that
 *   collision is exactly the duplicate-row defect the v3 derivation exists to
 *   prevent.
 *
 * Longhorn is absent and cannot arrive: the dependency runs Longhorn → Poodle,
 * the record shapes here are structural mirrors, and `checkout` is Poodle's own
 * word for what a host maps onto its own prefer operation.
 */

import { defineComponentInterface } from "./define";
import type {
  AxisNamesOf,
  EventNamesOf,
  PartIdsOf,
  PortableEventsOf,
  PortablePropsOf,
  TokenRoleNamesOf,
} from "./define";

/** One history entry, as the authority's `ForkEntryRecord` mirrors it.
 * `position` and the timestamps are host facts; nothing here invents a clock. */
const HISTORY_ENTRY_FIELDS = [
  { name: "id", type: { kind: "string" } },
  { name: "label", type: { kind: "string" } },
  // "past" | "current" | "future". Collection item fields carry no enum
  // vocabulary; the renderer projects the value onto `data-position` and the
  // corpus asserts the projection, so a wrong value is still observable.
  { name: "position", type: { kind: "string" } },
  { name: "checkpoint", type: { kind: "boolean" }, optional: true },
  { name: "groupId", type: { kind: "string" }, optional: true },
  { name: "recordedAtMs", type: { kind: "number" }, optional: true },
  // Every continuation from this entry, the run's own next row included. A
  // fork count is one less (R4); a run's last entry is always 0.
  { name: "continuationCount", type: { kind: "number" } },
] as const;

export const historyCenterInterface = defineComponentInterface({
  id: "history-center",
  profile: "composite",
  props: [
    {
      // Root path pages in fetch order, newest page first. `null` disables the
      // list: no rows render and every row event is inert.
      name: "pages",
      type: {
        kind: "collection",
        rustType: "HistoryPathPage",
        fields: [
          { name: "offset", type: { kind: "number" } },
          { name: "precedingContinuationCount", type: { kind: "number" } },
          { name: "truncatedBefore", type: { kind: "boolean" } },
          { name: "truncatedAfter", type: { kind: "boolean" } },
          {
            name: "entries",
            type: {
              kind: "collection",
              rustType: "HistoryEntry",
              fields: HISTORY_ENTRY_FIELDS,
            },
          },
        ],
      },
      default: null,
      nullable: true,
    },
    { name: "canUndo", type: { kind: "boolean" }, default: false, rustName: "can_undo" },
    { name: "canRedo", type: { kind: "boolean" }, default: false, rustName: "can_redo" },
    { name: "busy", type: { kind: "boolean" }, default: false, rustName: "is_busy" },
    {
      name: "status",
      type: { kind: "enum", values: ["idle", "loading", "failed"] },
      default: "idle",
      rustEnumName: "HistoryCenterStatus",
    },
    { name: "statusMessage", type: { kind: "string" }, default: null, nullable: true },
    {
      // A rejection code the host's bridge mapped from its protocol. The
      // component owns the display copy; the protocol never reaches Poodle.
      name: "rejection",
      type: { kind: "enum", values: ["AlreadyAtTarget", "UnknownEntry"] },
      default: null,
      nullable: true,
      rustEnumName: "HistoryCenterRejection",
    },
    {
      name: "open",
      type: { kind: "boolean" },
      default: null,
      nullable: true,
      controlledBy: "openChange",
    },
    { name: "defaultOpen", type: { kind: "boolean" }, default: false },
    {
      name: "placement",
      type: {
        kind: "enum",
        values: [
          "top", "top-start", "top-end",
          "bottom", "bottom-start", "bottom-end",
          "left", "left-start", "left-end",
          "right", "right-start", "right-end",
        ],
      },
      default: "bottom-end",
      rustType: "OverlayPlacement",
    },
    { name: "undoLabel", type: { kind: "string" }, default: "Undo" },
    { name: "redoLabel", type: { kind: "string" }, default: "Redo" },
    { name: "listLabel", type: { kind: "string" }, default: "History" },
    { name: "title", type: { kind: "string" }, default: "History" },
    { name: "emptyMessage", type: { kind: "string" }, default: "No history entries yet." },
    { name: "ariaLabel", type: { kind: "string" }, default: null, nullable: true },
    {
      // Client-side affordance only: it caps the rename input's length and
      // enforces no protocol rule.
      name: "maxBranchNameBytes",
      type: { kind: "number" },
      default: 256,
      rustType: "usize",
    },
    {
      name: "size",
      type: { kind: "enum", values: ["xs", "sm", "md", "lg", "xl"] },
      default: null,
      nullable: true,
      rustType: "ControlSize",
    },
    {
      name: "sizeRole",
      type: { kind: "enum", values: ["chrome", "control", "prominent"] },
      default: "chrome",
      rustType: "SemanticControlSizeRole",
    },
    {
      name: "density",
      type: { kind: "enum", values: ["compact", "default", "comfortable"] },
      default: null,
      nullable: true,
      rustType: "ControlDensity",
    },
  ],
  events: [
    { name: "undo", payload: {} },
    { name: "redo", payload: {} },
    { name: "openChange", payload: { open: "boolean" } },
    // Always the entry actually activated, on the branch that owns its run.
    // `branchId` is empty on the spine — the host knows its own branch.
    { name: "navigateEntry", payload: { branchId: "string", entryId: "string" } },
    { name: "renameBranch", payload: { branchId: "string", name: "string" } },
    { name: "loadContinuations", payload: { entryId: "string" } },
    { name: "loadContinuationRun", payload: { fromEntryId: "string" } },
    // Poodle's own word. A host maps it onto its own prefer operation.
    { name: "checkoutContinuation", payload: { entryId: "string" } },
  ],
  regions: [],
  hostRecords: [
    {
      // The forks at an anchor. The host returns every child of the anchor,
      // the one already on the list included; the derivation filters that one
      // out by id and never assumes its position.
      name: "continuations",
      answers: "loadContinuations",
      fields: [
        { name: "anchorEntryId", type: { kind: "string" } },
        { name: "entryId", type: { kind: "string" } },
        { name: "label", type: { kind: "string" } },
        { name: "preferred", type: { kind: "boolean" } },
        { name: "entryCount", type: { kind: "number" } },
        { name: "branchId", type: { kind: "string" } },
        { name: "branchName", type: { kind: "string" }, optional: true },
      ],
    },
    {
      // A continuation run's entries, newest first, tagged with the fork they
      // belong to. Flat rather than paged: the page-join reversal is proved on
      // the spine, and a run's own paging adds no new claim.
      name: "runEntries",
      answers: "loadContinuationRun",
      fields: [
        { name: "fromEntryId", type: { kind: "string" } },
        ...HISTORY_ENTRY_FIELDS,
      ],
    },
  ],
  parts: [
    { id: "root", resolve: { web: { kind: "self" }, native: { kind: "self" } } },
    {
      id: "undo",
      role: "button",
      resolve: {
        web: { kind: "class", className: ".poodle-history-center__undo .poodle-icon-button" },
        native: { kind: "id", id: "history-center:undo" },
      },
    },
    {
      id: "redo",
      role: "button",
      resolve: {
        web: { kind: "class", className: ".poodle-history-center__redo .poodle-icon-button" },
        native: { kind: "id", id: "history-center:redo" },
      },
    },
    {
      id: "list-trigger",
      role: "button",
      resolve: {
        web: { kind: "class", className: ".poodle-history-center__list-trigger" },
        native: { kind: "id", id: "history-center:list-trigger" },
      },
    },
    {
      id: "surface",
      role: "dialog",
      relativeTo: "list-trigger",
      resolve: {
        web: { kind: "class", className: ".poodle-popover__surface" },
        native: { kind: "id", id: "history-center:surface" },
      },
    },
    {
      // The bounded scroll region. Rows scroll inside it; the surface never
      // grows to the height of the history.
      id: "list",
      role: "list",
      resolve: {
        web: { kind: "class", className: ".poodle-history-center__list" },
        native: { kind: "id", id: "history-center:list" },
      },
    },
    {
      // Every visible entry row, spine or fork run. One part, one key space:
      // the row carries the hierarchy level, the fork identity travels on the
      // command payload, and nothing depends on indentation.
      id: "row",
      role: "listitem",
      repeat: {
        sources: [{ prop: "pages", path: "entries" }, { hostRecord: "runEntries" }],
        key: "id",
      },
      resolve: {
        web: {
          kind: "class",
          className: '.poodle-history-center__row[data-row-kind="entry"]',
          keyAttribute: "data-row-entry",
        },
        native: { kind: "id-template", template: "history-center:row:{key}" },
      },
    },
    {
      id: "entry",
      role: "button",
      repeat: {
        sources: [{ prop: "pages", path: "entries" }, { hostRecord: "runEntries" }],
        key: "id",
      },
      resolve: {
        web: {
          kind: "class",
          className: ".poodle-history-center__entry-content",
          keyAttribute: "data-entry",
        },
        native: { kind: "id-template", template: "history-center:entry:{key}" },
      },
    },
    {
      // The fork disclosure: a sibling of the entry button, never nested
      // inside it — no interactive element inside another interactive element.
      id: "disclosure",
      role: "button",
      repeat: {
        sources: [{ prop: "pages", path: "entries" }, { hostRecord: "runEntries" }],
        key: "id",
      },
      resolve: {
        web: {
          kind: "class",
          className: ".poodle-history-center__fork",
          keyAttribute: "data-entry",
        },
        native: { kind: "id-template", template: "history-center:disclosure:{key}" },
      },
    },
    {
      // The picker row, keyed by the anchor whose forks it offers. Anchors are
      // entries, so it keys over the same sources.
      id: "picker",
      role: "listitem",
      repeat: {
        sources: [{ prop: "pages", path: "entries" }, { hostRecord: "runEntries" }],
        key: "id",
      },
      resolve: {
        web: {
          kind: "class",
          className: '.poodle-history-center__row[data-row-kind="picker"]',
          keyAttribute: "data-row-entry",
        },
        native: { kind: "id-template", template: "history-center:picker:{key}" },
      },
    },
    {
      id: "picker-select",
      role: "combobox",
      repeat: {
        sources: [{ prop: "pages", path: "entries" }, { hostRecord: "runEntries" }],
        key: "id",
      },
      resolve: {
        web: {
          kind: "class",
          className: ".poodle-select__trigger",
          keyAttribute: "data-anchor",
          keyScope: ".poodle-history-center__picker-controls",
        },
        native: { kind: "id-template", template: "history-center:picker-select:{key}" },
      },
    },
    {
      // One option per offered fork, keyed by the fork's first entry — the
      // continuation's stable identity, never a list index.
      id: "picker-option",
      role: "option",
      repeat: {
        sources: [{ hostRecord: "continuations" }],
        key: "entryId",
      },
      resolve: {
        web: {
          kind: "class",
          className: ".poodle-select__option",
          keyAttribute: "data-value",
        },
        native: { kind: "id-template", template: "history-center:picker-option:{key}" },
      },
    },
    {
      id: "picker-actions",
      role: "button",
      repeat: {
        sources: [{ prop: "pages", path: "entries" }, { hostRecord: "runEntries" }],
        key: "id",
      },
      resolve: {
        web: {
          kind: "class",
          className: ".poodle-menu__trigger",
          keyAttribute: "data-anchor",
          keyScope: ".poodle-history-center__picker-actions",
        },
        native: { kind: "id-template", template: "history-center:picker-actions:{key}" },
      },
    },
    {
      // A singleton, not a repeated part: one actions menu is open at a
      // time, and `Menu` portals its surface out of the wrapper carrying the
      // anchor, so no ancestor rule could reach the items anyway.
      id: "action-rename",
      role: "menuitem",
      resolve: {
        web: {
          kind: "class",
          className: '.poodle-menu-surface__item[data-value="rename"]',
        },
        native: { kind: "id", id: "history-center:action-rename" },
      },
    },
    {
      // A singleton, not a repeated part: one actions menu is open at a
      // time, and `Menu` portals its surface out of the wrapper carrying the
      // anchor, so no ancestor rule could reach the items anyway.
      id: "action-checkout",
      role: "menuitem",
      resolve: {
        web: {
          kind: "class",
          className: '.poodle-menu-surface__item[data-value="checkout"]',
        },
        native: { kind: "id", id: "history-center:action-checkout" },
      },
    },
    {
      // The inline input that takes the Select's place while a rename is open.
      id: "rename-input",
      role: "textbox",
      repeat: {
        sources: [{ prop: "pages", path: "entries" }, { hostRecord: "runEntries" }],
        key: "id",
      },
      resolve: {
        web: {
          kind: "class",
          className: ".poodle-history-center__rename-input",
          keyAttribute: "data-anchor",
        },
        native: { kind: "id-template", template: "history-center:rename-input:{key}" },
      },
    },
    {
      // An open fork whose run has not arrived. Never an empty gap, never a
      // dropped entry.
      id: "not-yet-loaded",
      role: "listitem",
      repeat: {
        sources: [{ prop: "pages", path: "entries" }, { hostRecord: "runEntries" }],
        key: "id",
      },
      resolve: {
        web: {
          kind: "class",
          className: '.poodle-history-center__row[data-row-kind="not-yet-loaded"]',
          keyAttribute: "data-row-entry",
        },
        native: { kind: "id-template", template: "history-center:not-yet-loaded:{key}" },
      },
    },
    {
      id: "empty",
      contains: "text",
      resolve: {
        web: { kind: "class", className: ".poodle-history-center__empty" },
        native: { kind: "id", id: "history-center:empty" },
      },
    },
    {
      id: "status",
      role: "status",
      contains: "text",
      resolve: {
        web: { kind: "class", className: ".poodle-history-center__loading" },
        native: { kind: "id", id: "history-center:status" },
      },
    },
    {
      id: "rejection",
      role: "status",
      contains: "text",
      resolve: {
        web: { kind: "class", className: ".poodle-history-center__rejection" },
        native: { kind: "id", id: "history-center:rejection" },
      },
    },
  ],
  states: [
    {
      name: "open",
      condition: "the surface is mounted and registered on the dismissable layer",
      web: "part-present",
      native: "part-present",
      part: "surface",
    },
    {
      name: "undoDisabled",
      condition: "undo is unavailable or an authority operation is running",
      web: "part-disabled-attr",
      native: "part-interaction-disabled",
      part: "undo",
    },
    {
      name: "redoDisabled",
      condition: "redo is unavailable or an authority operation is running",
      web: "part-disabled-attr",
      native: "part-interaction-disabled",
      part: "redo",
    },
    {
      name: "empty",
      condition: "no visible rows: pages absent or the derivation produced none",
      web: "part-present",
      native: "part-present",
      part: "empty",
    },
    {
      name: "rejected",
      condition: "a mapped rejection notice is displayed",
      web: "part-present",
      native: "part-present",
      part: "rejection",
    },
  ],
  tokenRoles: [
    { name: "placement", prop: "placement", default: "bottom-end" },
    { name: "status", prop: "status", default: "idle" },
    // HistoryCenter defaults to the chrome semantic role; at the default
    // presentation scale that resolves to sm in every active runtime.
    { name: "size", prop: "size", default: "sm" },
    { name: "density", prop: "density", default: "default" },
  ],
  axes: ["placement", "size", "density", "theme"],
  capabilities: [
    { name: "structure.identity", required: true },
    { name: "structure.part-resolution", required: true },
    { name: "overlay.intent", required: true },
    { name: "overlay.dismiss", required: true },
    { name: "overlay.layer", required: true },
    { name: "semantic.expanded", required: true },
    { name: "semantic.selected", required: true },
    { name: "semantic.disabled", required: true },
    { name: "semantic.token-roles", required: true },
    { name: "input.value", required: true },
    { name: "input.editing", required: true },
    { name: "focus", required: true },
    { name: "activate", required: true },
    { name: "interaction.key", required: true },
    { name: "layout.geometry", required: true },
    { name: "layout.position", required: true },
    { name: "accessibility.projection", required: true },
  ],
});

export type HistoryCenterInterface = typeof historyCenterInterface;
export type HistoryCenterPortableProps = PortablePropsOf<HistoryCenterInterface>;
export type HistoryCenterPortablePage =
  NonNullable<HistoryCenterPortableProps["pages"]>[number];
export type HistoryCenterPortableEntry = HistoryCenterPortablePage["entries"][number];
export type HistoryCenterPortableEvents = PortableEventsOf<HistoryCenterInterface>;
export type HistoryCenterPartId = PartIdsOf<HistoryCenterInterface>;
export type HistoryCenterEventName = EventNamesOf<HistoryCenterInterface>;
export type HistoryCenterTokenRole = TokenRoleNamesOf<HistoryCenterInterface>;
export type HistoryCenterAxis = AxisNamesOf<HistoryCenterInterface>;

export const HISTORY_CENTER_DEFAULT_PROPS: HistoryCenterPortableProps = {
  pages: null,
  canUndo: false,
  canRedo: false,
  busy: false,
  status: "idle",
  statusMessage: null,
  rejection: null,
  open: null,
  defaultOpen: false,
  placement: "bottom-end",
  undoLabel: "Undo",
  redoLabel: "Redo",
  listLabel: "History",
  title: "History",
  emptyMessage: "No history entries yet.",
  ariaLabel: null,
  maxBranchNameBytes: 256,
  size: null,
  sizeRole: "chrome",
  density: null,
};
