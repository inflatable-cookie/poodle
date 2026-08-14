/**
 * Model-connection display types and pure helpers.
 *
 * Poodle receives presentation records with opaque ids and safe labels. No type
 * here includes a credential, credential reference, raw probe output, filesystem
 * evidence, executable handle, target, or account identifier.
 *
 * Contracts: model-connection-picker, model-connection-setup,
 * model-connection-card, model-catalogue-editor.
 */

import { applyReorder } from "./tabs";

// ── Shared display vocabulary ────────────────────────────────────────────

export type ModelConnectionBadgeTone =
  | "neutral"
  | "info"
  | "success"
  | "warning"
  | "danger";

export type ModelConnectionBadge = {
  label: string;
  tone?: ModelConnectionBadgeTone;
};

export type ModelConnectionAvailability =
  | "available"
  | "checking"
  | "unavailable"
  | "unsupported";

export type ModelConnectionPickerState =
  | "ready"
  | "loading"
  | "error"
  | "empty"
  | "noResults";

/** PickerShell browse-state spelling for `noResults`. */
export type ModelConnectionPickerShellState =
  | "ready"
  | "loading"
  | "error"
  | "empty"
  | "no-results";

export type ModelConnectionOption = {
  id: string;
  providerLabel: string;
  routeLabel: string | null;
  description: string | null;
  group: string;
  keywords: string[];
  badges: ModelConnectionBadge[];
  availability: ModelConnectionAvailability;
  availabilityLabel: string;
  isDisabled: boolean;
};

export type ModelConnectionSetupStage = "choose" | "configure";

export type ModelConnectionReadiness =
  | "ready"
  | "checking"
  | "attention"
  | "unavailable"
  | "unknown"
  | "error";

export type ModelConnectionSummary = {
  id: string;
  title: string;
  providerLabel: string;
  routeLabel: string | null;
  version: string | null;
  accessSummary: string | null;
  readiness: ModelConnectionReadiness;
  readinessLabel: string;
  enabled: boolean;
};

export type ModelCatalogueState =
  | "ready"
  | "loading"
  | "unavailable"
  | "empty"
  | "error"
  | "sessionNegotiated";

export type ModelCatalogueItem = {
  id: string;
  label: string;
  providerLabel: string | null;
  description: string | null;
  badges: ModelConnectionBadge[];
  visible: boolean;
  isDisabled: boolean;
};

export type ModelCatalogueVisibilityChange = {
  id: string;
  visible: boolean;
};

export type ModelConnectionStatusTone =
  | "neutral"
  | "success"
  | "warning"
  | "danger"
  | "info";

export type ModelConnectionOptionGroup = {
  group: string;
  options: ModelConnectionOption[];
};

// ── Picker filtering ─────────────────────────────────────────────────────

function caseFold(value: string): string {
  return value.toLocaleLowerCase();
}

function optionMatchesQuery(option: ModelConnectionOption, foldedQuery: string): boolean {
  if (!foldedQuery) return true;

  const haystacks = [
    option.providerLabel,
    option.routeLabel ?? "",
    option.description ?? "",
    option.group,
    ...option.keywords,
  ];

  return haystacks.some((part) => caseFold(part).includes(foldedQuery));
}

/** Case-folded filter across provider, route, description, group, and keywords.
 *  Retains host source order. */
export function filterModelConnectionOptions(
  options: readonly ModelConnectionOption[],
  query: string,
): ModelConnectionOption[] {
  const folded = caseFold(query.trim());
  if (!folded) return [...options];
  return options.filter((option) => optionMatchesQuery(option, folded));
}

/** Group filtered options while retaining first-seen group order and source order. */
export function groupModelConnectionOptions(
  options: readonly ModelConnectionOption[],
): ModelConnectionOptionGroup[] {
  const groups: ModelConnectionOptionGroup[] = [];
  const indexByGroup = new Map<string, number>();

  for (const option of options) {
    const existing = indexByGroup.get(option.group);
    if (existing === undefined) {
      indexByGroup.set(option.group, groups.length);
      groups.push({ group: option.group, options: [option] });
      continue;
    }
    groups[existing].options.push(option);
  }

  return groups;
}

export function modelConnectionOptionSelectable(option: ModelConnectionOption): boolean {
  return !option.isDisabled && option.availability === "available";
}

export function resolveModelConnectionPickerShellState(
  state: ModelConnectionPickerState,
  sourceCount: number,
  matchCount: number,
  query: string,
): ModelConnectionPickerShellState {
  if (state === "loading" || state === "error") return state;
  if (state === "empty" || sourceCount === 0) return "empty";
  if (state === "noResults" || (query.trim() !== "" && matchCount === 0)) {
    return "no-results";
  }
  return "ready";
}

export function modelConnectionPickerResultAnnouncement(
  matchCount: number,
  query: string,
): string {
  const trimmed = query.trim();
  if (!trimmed) {
    return matchCount === 1 ? "1 connection" : `${matchCount} connections`;
  }
  if (matchCount === 0) return `No connections match “${trimmed}”`;
  return matchCount === 1
    ? `1 connection matches “${trimmed}”`
    : `${matchCount} connections match “${trimmed}”`;
}

// ── Setup stage guards ───────────────────────────────────────────────────

export type ModelConnectionSetupContext = {
  stage: ModelConnectionSetupStage;
  value: string | null;
  query: string;
  options: readonly ModelConnectionOption[];
  canSubmit: boolean;
  isPending: boolean;
};

export type ModelConnectionSetupEvent =
  | { type: "SELECT"; id: string }
  | { type: "SET_VALUE"; id: string | null }
  | { type: "SET_QUERY"; query: string }
  | { type: "SET_STAGE"; stage: ModelConnectionSetupStage }
  | { type: "CONTINUE" }
  | { type: "BACK" }
  | { type: "SUBMIT" }
  | { type: "CANCEL" };

export type ModelConnectionSetupEffect =
  | { type: "emitValueChange"; id: string }
  | { type: "emitQueryChange"; query: string }
  | { type: "emitStageChange"; stage: ModelConnectionSetupStage }
  | { type: "emitSubmit"; id: string }
  | { type: "emitCancel" };

export type ModelConnectionSetupResult = {
  context: ModelConnectionSetupContext;
  effects: ModelConnectionSetupEffect[];
};

function selectedOption(
  context: ModelConnectionSetupContext,
): ModelConnectionOption | undefined {
  if (context.value === null) return undefined;
  return context.options.find((option) => option.id === context.value);
}

export function modelConnectionSetupCanContinue(
  context: Pick<ModelConnectionSetupContext, "value" | "options" | "isPending">,
): boolean {
  if (context.isPending || context.value === null) return false;
  const option = context.options.find((candidate) => candidate.id === context.value);
  return option !== undefined && modelConnectionOptionSelectable(option);
}

export function modelConnectionSetupCanSubmit(
  context: Pick<
    ModelConnectionSetupContext,
    "stage" | "value" | "options" | "canSubmit" | "isPending"
  >,
): boolean {
  if (context.isPending || context.stage !== "configure" || !context.canSubmit) {
    return false;
  }
  if (context.value === null) return false;
  return context.options.some((option) => option.id === context.value);
}

export function modelConnectionSetupTransition(
  context: ModelConnectionSetupContext,
  event: ModelConnectionSetupEvent,
): ModelConnectionSetupResult {
  const stay: ModelConnectionSetupResult = { context, effects: [] };

  switch (event.type) {
    case "SET_VALUE":
      return { context: { ...context, value: event.id }, effects: [] };

    case "SET_STAGE":
      return { context: { ...context, stage: event.stage }, effects: [] };

    case "SET_QUERY": {
      if (context.isPending) return stay;
      return {
        context: { ...context, query: event.query },
        effects: [{ type: "emitQueryChange", query: event.query }],
      };
    }

    case "SELECT": {
      if (context.isPending) return stay;
      const option = context.options.find((candidate) => candidate.id === event.id);
      if (!option || !modelConnectionOptionSelectable(option)) return stay;
      if (context.value === event.id) return stay;
      return {
        context: { ...context, value: event.id },
        effects: [{ type: "emitValueChange", id: event.id }],
      };
    }

    case "CONTINUE": {
      if (!modelConnectionSetupCanContinue(context)) return stay;
      return {
        context: { ...context, stage: "configure" },
        effects: [{ type: "emitStageChange", stage: "configure" }],
      };
    }

    case "BACK": {
      if (context.isPending || context.stage !== "configure") return stay;
      return {
        context: { ...context, stage: "choose" },
        effects: [{ type: "emitStageChange", stage: "choose" }],
      };
    }

    case "SUBMIT": {
      if (!modelConnectionSetupCanSubmit(context) || context.value === null) {
        return stay;
      }
      return {
        context,
        effects: [{ type: "emitSubmit", id: context.value }],
      };
    }

    case "CANCEL": {
      if (context.isPending) return stay;
      return { context, effects: [{ type: "emitCancel" }] };
    }
  }
}

export function modelConnectionSelectedSummary(
  option: ModelConnectionOption | undefined,
): Pick<ModelConnectionSummary, "id" | "title" | "providerLabel" | "routeLabel"> | null {
  if (!option) return null;
  return {
    id: option.id,
    title: option.providerLabel,
    providerLabel: option.providerLabel,
    routeLabel: option.routeLabel,
  };
}

export function modelConnectionSetupSelectedOption(
  context: Pick<ModelConnectionSetupContext, "value" | "options">,
): ModelConnectionOption | undefined {
  return selectedOption(context as ModelConnectionSetupContext);
}

// ── Catalogue order and visibility ───────────────────────────────────────

export function shownModelCatalogueItems(
  items: readonly ModelCatalogueItem[],
): ModelCatalogueItem[] {
  return items.filter((item) => item.visible);
}

export function hiddenModelCatalogueItems(
  items: readonly ModelCatalogueItem[],
): ModelCatalogueItem[] {
  return items.filter((item) => !item.visible);
}

/** Complete shown-id order after moving one shown index to another. */
export function requestModelCatalogueOrder(
  shownIds: readonly string[],
  fromIndex: number,
  toIndex: number,
): string[] | null {
  if (
    fromIndex < 0 ||
    toIndex < 0 ||
    fromIndex >= shownIds.length ||
    toIndex >= shownIds.length ||
    fromIndex === toIndex
  ) {
    return null;
  }

  return applyReorder([...shownIds], fromIndex, toIndex).items;
}

export function requestModelCatalogueVisibility(
  id: string,
  visible: boolean,
): ModelCatalogueVisibilityChange {
  return { id, visible };
}

export function modelCatalogueFocusAfterHide(
  shownIds: readonly string[],
  hiddenId: string,
): { kind: "shown"; id: string } | { kind: "hidden-section" } {
  const index = shownIds.indexOf(hiddenId);
  if (index < 0) return { kind: "hidden-section" };
  const next = shownIds[index + 1] ?? shownIds[index - 1];
  if (next === undefined) return { kind: "hidden-section" };
  return { kind: "shown", id: next };
}

export function modelCatalogueReorderAnnouncement(
  label: string,
  position: number,
  total: number,
): string {
  return `Moved ${label} to position ${position} of ${total}.`;
}

export function modelCatalogueVisibilityAnnouncement(
  label: string,
  visible: boolean,
): string {
  return visible ? `Restored ${label}.` : `Hid ${label}.`;
}

// ── Status tone mapping ──────────────────────────────────────────────────

export function modelConnectionAvailabilityTone(
  availability: ModelConnectionAvailability,
): ModelConnectionStatusTone {
  switch (availability) {
    case "available":
      return "success";
    case "checking":
      return "info";
    case "unavailable":
      return "warning";
    case "unsupported":
      return "neutral";
  }
}

export function modelConnectionReadinessTone(
  readiness: ModelConnectionReadiness,
): ModelConnectionStatusTone {
  switch (readiness) {
    case "ready":
      return "success";
    case "checking":
      return "info";
    case "attention":
      return "warning";
    case "unavailable":
      return "warning";
    case "unknown":
      return "neutral";
    case "error":
      return "danger";
  }
}

export function modelCatalogueStateCopy(state: ModelCatalogueState): {
  title: string;
  message: string;
} {
  switch (state) {
    case "loading":
      return {
        title: "Loading models",
        message: "Waiting for the connection catalogue.",
      };
    case "unavailable":
      return {
        title: "Models unavailable",
        message: "This connection does not expose a model catalogue.",
      };
    case "empty":
      return {
        title: "No models",
        message: "The catalogue returned successfully with no entries.",
      };
    case "error":
      return {
        title: "Could not load models",
        message: "The catalogue request failed. Try again from the host.",
      };
    case "sessionNegotiated":
      return {
        title: "Models after session",
        message: "Models for this connection are negotiated after a session starts.",
      };
    case "ready":
      return { title: "Models", message: "" };
  }
}

// ── Specimen fixtures (inert placeholders only) ──────────────────────────

export const MODEL_CONNECTION_PICKER_FIXTURES: ModelConnectionOption[] = [
  {
    id: "openai-responses",
    providerLabel: "OpenAI",
    routeLabel: "Responses API",
    description: "Hosted Responses route for chat and tools.",
    group: "Hosted",
    keywords: ["openai", "responses", "api"],
    badges: [{ label: "Hosted", tone: "info" }],
    availability: "available",
    availabilityLabel: "Available",
    isDisabled: false,
  },
  {
    id: "openai-completions",
    providerLabel: "OpenAI",
    routeLabel: "Chat Completions",
    description: "Legacy chat-completions route.",
    group: "Hosted",
    keywords: ["openai", "completions"],
    badges: [{ label: "Hosted", tone: "info" }],
    availability: "available",
    availabilityLabel: "Available",
    isDisabled: false,
  },
  {
    id: "anthropic-messages",
    providerLabel: "Anthropic",
    routeLabel: "Messages API",
    description: "Hosted Messages route.",
    group: "Hosted",
    keywords: ["anthropic", "messages"],
    badges: [{ label: "Hosted", tone: "info" }],
    availability: "available",
    availabilityLabel: "Available",
    isDisabled: false,
  },
  {
    id: "codex-app",
    providerLabel: "Codex",
    routeLabel: "App install",
    description: "Installed local harness.",
    group: "Installed",
    keywords: ["codex", "local", "harness"],
    badges: [{ label: "Installed", tone: "success" }],
    availability: "checking",
    availabilityLabel: "Checking install",
    isDisabled: true,
  },
  {
    id: "ollama-local",
    providerLabel: "Ollama",
    routeLabel: "Local runtime",
    description: "Local OpenAI-compatible endpoint.",
    group: "Local runtime",
    keywords: ["ollama", "local", "endpoint"],
    badges: [{ label: "Local", tone: "neutral" }],
    availability: "available",
    availabilityLabel: "Available",
    isDisabled: false,
  },
  {
    id: "lmstudio-local",
    providerLabel: "LM Studio",
    routeLabel: "Local endpoint",
    description: "Local OpenAI-compatible server.",
    group: "Local runtime",
    keywords: ["lmstudio", "local"],
    badges: [{ label: "Local", tone: "neutral" }],
    availability: "unavailable",
    availabilityLabel: "Runtime not detected",
    isDisabled: true,
  },
  {
    id: "vendor-legacy",
    providerLabel: "Legacy Vendor",
    routeLabel: "SDK v1",
    description: "Unsupported on this machine.",
    group: "Hosted",
    keywords: ["legacy"],
    badges: [{ label: "Unsupported", tone: "warning" }],
    availability: "unsupported",
    availabilityLabel: "Unsupported on this platform",
    isDisabled: true,
  },
];

export const MODEL_CONNECTION_CARD_FIXTURES: ModelConnectionSummary[] = [
  {
    id: "conn-openai-work",
    title: "OpenAI · Work",
    providerLabel: "OpenAI",
    routeLabel: "Responses API",
    version: "2026-08",
    accessSummary: "API key on file",
    readiness: "ready",
    readinessLabel: "Ready",
    enabled: true,
  },
  {
    id: "conn-openai-personal",
    title: "OpenAI · Personal",
    providerLabel: "OpenAI",
    routeLabel: "Responses API",
    version: "2026-08",
    accessSummary: "API key on file",
    readiness: "ready",
    readinessLabel: "Ready",
    enabled: false,
  },
  {
    id: "conn-codex-checking",
    title: "Codex",
    providerLabel: "Codex",
    routeLabel: "App install",
    version: null,
    accessSummary: "Signed in",
    readiness: "checking",
    readinessLabel: "Checking install",
    enabled: true,
  },
  {
    id: "conn-anthropic-attention",
    title: "Anthropic",
    providerLabel: "Anthropic",
    routeLabel: "Messages API",
    version: "2026-07",
    accessSummary: "Needs re-authorisation",
    readiness: "attention",
    readinessLabel: "Needs attention",
    enabled: true,
  },
  {
    id: "conn-ollama-unavailable",
    title: "Ollama",
    providerLabel: "Ollama",
    routeLabel: "Local runtime",
    version: null,
    accessSummary: "No credentials required",
    readiness: "unavailable",
    readinessLabel: "Runtime not reachable",
    enabled: true,
  },
];

export const MODEL_CATALOGUE_FIXTURES: ModelCatalogueItem[] = [
  {
    id: "model-alpha",
    label: "Frontier Alpha",
    providerLabel: "OpenAI",
    description: "General reasoning.",
    badges: [{ label: "Default candidate", tone: "info" }],
    visible: true,
    isDisabled: false,
  },
  {
    id: "model-beta",
    label: "Frontier Beta",
    providerLabel: "OpenAI",
    description: "Faster variant.",
    badges: [],
    visible: true,
    isDisabled: false,
  },
  {
    id: "model-gamma",
    label: "Gateway Gamma",
    providerLabel: "Anthropic",
    description: "Mixed gateway entry.",
    badges: [{ label: "Gateway", tone: "neutral" }],
    visible: true,
    isDisabled: false,
  },
  {
    id: "model-dup-a",
    label: "Shared Label",
    providerLabel: "OpenAI",
    description: "First opaque id.",
    badges: [],
    visible: true,
    isDisabled: false,
  },
  {
    id: "model-dup-b",
    label: "Shared Label",
    providerLabel: "Anthropic",
    description: "Second opaque id.",
    badges: [],
    visible: false,
    isDisabled: false,
  },
  {
    id: "model-hidden",
    label: "Archive Delta",
    providerLabel: null,
    description: "Recoverable hidden model.",
    badges: [],
    visible: false,
    isDisabled: false,
  },
];
