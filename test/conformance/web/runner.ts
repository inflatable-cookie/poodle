/**
 * Shared web conformance machinery (spec 066): interprets the serialized
 * case corpus against a runtime adapter (Svelte or React), observing the
 * real DOM through the interface's part descriptors and state observation
 * rules. No component identifier, class name, part list, or icon name is
 * hardcoded here — the interface data drives everything.
 *
 * Verdicts are strict: every case assertion must be observable by the
 * runtime evaluating it. An expected field the runtime cannot observe is a
 * failure naming runtime, case, step, and field — never a silently passable
 * "vacuous".
 */

import type {
  SerializedCase,
  SerializedComponentInterface,
} from "@inflatable-cookie/poodle-core/conformance";
// Relative import like the hosts' component imports: package subpaths and
// the bare entry are not reliably resolvable from the test tree in a clean
// checkout (bun resolves them against a stale package cache).
import { dismissLayerStackLength } from "../../../packages/core/src/dom/dismiss";
import { channelsOf, geometryFields, geometryOf } from "./observer";

export interface TraceEntry {
  event: string;
  payload?: Record<string, unknown>;
}

export interface PartObservation {
  present: boolean;
  role: string | null;
  name: string | null;
  text: string | null;
  icon: string | null;
  /** Scalar thumb value, controlled pair, or editable text. */
  value: number | [number, number] | string | null;
  /** Caret/selection offsets, in characters. */
  selectionStart: number | null;
  selectionEnd: number | null;
  states: Record<string, boolean | null>;
  tokenRoles: Record<string, string | null>;
  expanded: boolean | null;
  /** The part that hosts this part's layer (nearest part ancestor, else the
   * component root for portalled surfaces). */
  parent: string | null;
  /** Overlay evidence: the part renders above normal content. */
  overlay: boolean | null;
  focusable: boolean | null;
  focused: boolean | null;
  focusVisible: boolean | null;
  tabbable: boolean | null;
  selected: boolean | null;
  orientation: string | null;
  controls: string | null;
  labelledBy: string | null;
  /** Root-only: the focused element's text (focused-part progression). */
  focusedText: string | null;
  /** Root-only: the number of open overlay layers. */
  layerCount: number | null;
  geometry: Record<string, number | null>;
  channels: Record<string, string | null>;
}

export interface RuntimeObservation {
  runtime: string;
  component: string;
  parts: Record<string, PartObservation>;
  trace: TraceEntry[];
}

export interface AssertionResult {
  stepIndex: number;
  part: string | null;
  field: string;
  verdict: "pass" | "fail";
  expected?: unknown;
  actual?: unknown;
  reason?: string;
}

export interface CaseResult {
  caseId: string;
  pass: boolean;
  failures: AssertionResult[];
  /** Every assertion verdict for this case (pass | fail). */
  assertions: AssertionResult[];
  observations: RuntimeObservation[];
}

export interface RuntimeCaseReport {
  runtime: string;
  component: string;
  results: CaseResult[];
}

/** What a runtime adapter must provide: mount, observe, act, capture. */
export interface RuntimeAdapter {
  readonly runtime: string;
  mount(caseFixture: { props: Record<string, unknown>; regions: Record<string, string>; host?: Record<string, unknown> }): void;
  rootElement(): HTMLElement | null;
  press(part: string, input: "pointer" | "keyboard"): Promise<void>;
  focus(part: string): void;
  key(part: string, key: string): Promise<void>;
  scrub(part: string, fraction: number, phase: "press" | "drag" | "release"): Promise<void>;
  /** The harness's real dismissal route (document-level Escape). */
  dismiss(part: string): Promise<void>;
  /** A real pointer interaction with inside/outside intent relative to a part. */
  pointer(part: string, target: "inside" | "outside"): Promise<void>;
  /** Insert text through the runtime's real editing path. */
  insert?(part: string, text: string): Promise<void>;
  /** Set caret/selection through the runtime's real selection path. */
  select?(part: string, start: number, end: number): Promise<void>;
  /** Drive composition/IME start, update, or commit. */
  compose?(part: string, text: string, phase: "start" | "update" | "commit"): Promise<void>;
  /** Flush pending framework state/effects so observation sees final DOM. */
  flush(): Promise<void>;
  trace(): TraceEntry[];
  cleanup(): void;
}

function resolveWebPart(
  root: HTMLElement,
  part: SerializedComponentInterface["parts"][number],
  partId: string = part.id,
): HTMLElement | null {
  const { kind } = part.resolve.web;
  switch (kind) {
    case "self":
      return root;
    case "class": {
      const key = part.repeat && partId.startsWith(`${part.id}:`)
        ? partId.slice(part.id.length + 1)
        : null;
      const find = (scope: ParentNode): HTMLElement | null => {
        if (key && part.resolve.web.keyAttribute) {
          return Array.from(scope.querySelectorAll<HTMLElement>(part.resolve.web.className))
            .find((candidate) => candidate.getAttribute(part.resolve.web.keyAttribute!) === key) ?? null;
        }
        return scope.querySelector<HTMLElement>(part.resolve.web.className);
      };
      const inRoot = find(root);
      if (inRoot) return inRoot;
      // Portalled overlay surfaces leave the component subtree (the anchored
      // surface mounts at the theme root), so a part resolution misses them.
      // Fall back to the document — the corpus mounts one instance at a time,
      // and first-match order still prefers the component's own subtree.
      return find(root.ownerDocument);
    }
    case "icon": {
      const { position, gatedBy, selector } = part.resolve.web;
      if (!root.hasAttribute(gatedBy)) return null;
      const spans = Array.from(root.querySelectorAll<HTMLElement>(selector));
      return position === "first" ? spans[0] ?? null : spans[spans.length - 1] ?? null;
    }
  }
  return null;
}

function expandedParts(
  iface: SerializedComponentInterface,
  root: HTMLElement,
): Array<{ id: string; decl: SerializedComponentInterface["parts"][number] }> {
  const expanded: Array<{ id: string; decl: SerializedComponentInterface["parts"][number] }> = [];
  for (const decl of iface.parts) {
    if (!decl.repeat || decl.resolve.web.kind !== "class" || !decl.resolve.web.keyAttribute) {
      expanded.push({ id: decl.id, decl });
      continue;
    }
    const keys = new Set(
      Array.from(root.querySelectorAll<HTMLElement>(decl.resolve.web.className))
        .map((candidate) => candidate.getAttribute(decl.resolve.web.keyAttribute!))
        .filter((key): key is string => Boolean(key)),
    );
    for (const key of keys) expanded.push({ id: `${decl.id}:${key}`, decl });
  }
  return expanded;
}

function normalizeRelationship(
  raw: string | null,
  iface: SerializedComponentInterface,
  root: HTMLElement,
): string | null {
  if (!raw) return null;
  for (const { id, decl } of expandedParts(iface, root)) {
    const element = resolveWebPart(root, decl, id);
    if (element?.id === raw) return id;
  }
  const repeatedKeys = new Set(
    expandedParts(iface, root)
      .map(({ id }) => id.includes(":") ? id.slice(id.indexOf(":") + 1) : null)
      .filter((key): key is string => Boolean(key)),
  );
  for (const decl of iface.parts) {
    if (!decl.repeat?.webIdPrefix || !raw.startsWith(decl.repeat.webIdPrefix)) continue;
    for (const key of repeatedKeys) {
      if (raw.endsWith(`-${key}`)) return `${decl.id}:${key}`;
    }
  }
  return raw;
}

/** The icon identity channel: the attribute the descriptor names on the
 * resolved part element. */
function iconIdentity(
  part: SerializedComponentInterface["parts"][number],
  el: HTMLElement | null,
): string | null {
  if (!el) return null;
  const web = part.resolve.web;
  const attribute = ("attribute" in web && web.attribute) || "data-icon";
  return el.getAttribute(attribute);
}

function roleOf(el: HTMLElement): string | null {
  const explicit = el.getAttribute("role");
  if (explicit) return explicit;
  if (el.tagName === "BUTTON") return "button";
  if (el.tagName === "INPUT" && (el as HTMLInputElement).type === "range") return "slider";
  if (el.tagName === "INPUT" || el.tagName === "TEXTAREA") return "textbox";
  return null;
}

function isFocusable(el: HTMLElement): boolean {
  if (el.hasAttribute("disabled") || el.getAttribute("aria-disabled") === "true") return false;
  if (el.tabIndex >= 0) return true;
  const tag = el.tagName;
  return tag === "BUTTON" || tag === "INPUT" || tag === "SELECT" || tag === "TEXTAREA" || tag === "A";
}

function partValue(el: HTMLElement | null): number | string | null {
  if (!el) return null;
  if (el instanceof HTMLInputElement && el.type === "range") {
    const parsed = Number(el.value);
    return Number.isFinite(parsed) ? parsed : null;
  }
  if (el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement) {
    return el.value;
  }
  const now = el.getAttribute("aria-valuenow");
  if (now == null) return null;
  const parsed = Number(now);
  return Number.isFinite(parsed) ? parsed : null;
}

function partSelection(el: HTMLElement | null): { start: number | null; end: number | null } {
  if (el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement) {
    return { start: el.selectionStart, end: el.selectionEnd };
  }
  return { start: null, end: null };
}

function observeRootStates(
  iface: SerializedComponentInterface,
  root: HTMLElement,
): Record<string, boolean | null> {
  const doc = root.ownerDocument;
  const states: Record<string, boolean | null> = {};
  for (const state of iface.states) {
    switch (state.web) {
      case "disabled-attr":
        states[state.name] = root.hasAttribute("disabled");
        break;
      case "data-attr":
        states[state.name] = root.getAttribute(state.attr ?? "") === "true";
        break;
      case "aria-pressed":
        states[state.name] = root.getAttribute("aria-pressed") === "true";
        break;
      case "active-element":
        states[state.name] = doc.activeElement === root;
        break;
      case "focus-visible-pseudo":
        states[state.name] = doc.activeElement === root && root.matches(":focus-visible");
        break;
      case "part-disabled-attr": {
        const decl = iface.parts.find((part) => part.id === state.part);
        const el = decl ? resolveWebPart(root, decl) : null;
        states[state.name] = el
          ? el.hasAttribute("disabled") || el.getAttribute("aria-disabled") === "true"
          : null;
        break;
      }
      case "part-active-element": {
        const decl = iface.parts.find((part) => part.id === state.part);
        const el = decl ? resolveWebPart(root, decl) : null;
        states[state.name] = el ? doc.activeElement === el : null;
        break;
      }
      case "part-present": {
        const decl = iface.parts.find((part) => part.id === state.part);
        const el = decl ? resolveWebPart(root, decl) : null;
        states[state.name] = el !== null;
        break;
      }
    }
  }
  return states;
}

function observeRootTokenRoles(
  iface: SerializedComponentInterface,
  root: HTMLElement,
): Record<string, string | null> {
  const roles: Record<string, string | null> = {};
  for (const role of iface.tokenRoles) {
    // Data attributes are kebab-cased (`data-surface-width`), the prop is
    // camelCase (`surfaceWidth`).
    const attribute = `data-${role.prop.replace(/([A-Z])/g, (letter) => `-${letter.toLowerCase()}`)}`;
    roles[role.name] = root.getAttribute(attribute) ?? role.default ?? null;
  }
  return roles;
}

/** The part that hosts this part's layer: the nearest ancestor element that
 * resolves to an interface part, else the component root (portalled overlay
 * surfaces render outside the subtree but still belong to its layer). */
function parentPartOf(
  el: HTMLElement,
  root: HTMLElement,
  partId: string,
  iface: SerializedComponentInterface,
): string | null {
  if (el === root) return null;
  let node = el.parentElement;
  while (node && node !== root && root.contains(node)) {
    for (const candidate of iface.parts) {
      if (candidate.id === partId) continue;
      const resolved = resolveWebPart(root, candidate, candidate.id);
      if (resolved && resolved === node) return candidate.id;
    }
    node = node.parentElement;
  }
  if (node === root) return "root";
  // Portalled: no part ancestor inside the component subtree.
  return "root";
}

/** Relative logical bounds of a part against its `relativeTo` anchor, from
 * the two elements' viewport boxes: facing-edge gaps, start/end alignment
 * deltas, and the width difference. */
function relativeBoundsOf(
  partId: string,
  iface: SerializedComponentInterface,
  root: HTMLElement,
): Record<string, number> | null {
  const part = iface.parts.find((candidate) => candidate.id === partId);
  if (!part?.relativeTo) return null;
  const anchor = iface.parts.find((candidate) => candidate.id === part.relativeTo);
  if (!anchor) return null;
  const el = resolveWebPart(root, part, partId);
  const anchorEl = resolveWebPart(root, anchor, anchor.id);
  if (!el || !anchorEl) return null;
  const r = el.getBoundingClientRect();
  const a = anchorEl.getBoundingClientRect();
  return {
    topGap: r.top - a.bottom,
    bottomGap: a.top - r.bottom,
    leftGap: r.left - a.right,
    rightGap: a.left - r.right,
    hStart: r.left - a.left,
    hEnd: a.right - r.right,
    vStart: r.top - a.top,
    widthGap: r.width - a.width,
  };
}

/** Observes a mounted component root through the interface's descriptors. */
export function observeDom(
  runtime: string,
  component: string,
  iface: SerializedComponentInterface,
  root: HTMLElement,
): RuntimeObservation {
  const states = observeRootStates(iface, root);
  const tokenRoles = observeRootTokenRoles(iface, root);
  const parts: Record<string, PartObservation> = {};
  for (const { id: partId, decl: part } of expandedParts(iface, root)) {
    const el = resolveWebPart(root, part, partId);
    const isRoot = partId === "root";
    const identity =
      isRoot ||
      Boolean(part.role) ||
      part.resolve?.native?.kind === "id" ||
      part.resolve?.native?.kind === "self";
    const identityName = el
      ? el.getAttribute("aria-label") ||
        (part.role && !el.hasAttribute("aria-labelledby") ? el.textContent?.trim() || null : null)
      : null;
    const geometry: Record<string, number | null> = el ? { ...geometryOf(el) } : {};
    const relative = relativeBoundsOf(partId, iface, root);
    if (relative) {
      for (const [field, value] of Object.entries(relative)) {
        geometry[field] = value;
      }
    }
    parts[partId] = {
      present: Boolean(el),
      role: el ? roleOf(el) : null,
      name: identityName,
      text: !isRoot && part.contains === "text" ? (el?.textContent?.trim() || null) : null,
      icon: iconIdentity(part, el),
      value: partValue(el),
      selectionStart: partSelection(el).start,
      selectionEnd: partSelection(el).end,
      states: {},
      tokenRoles: {},
      expanded: el?.getAttribute("aria-expanded") === "true" ? true : el?.hasAttribute("aria-expanded") ? false : null,
      parent: el ? parentPartOf(el, root, partId, iface) : null,
      overlay: el ? el.hasAttribute("data-poodle-anchored") : null,
      focusable: el && identity ? isFocusable(el) : null,
      focused:
        el && identity && !el.hasAttribute("aria-disabled")
          ? root.ownerDocument.activeElement === el
          : el && identity
            ? false
            : null,
      focusVisible:
        el && identity && !el.hasAttribute("aria-disabled")
          ? root.ownerDocument.activeElement === el && el.matches(":focus-visible")
          : el && identity
            ? false
            : null,
      tabbable: el && identity ? isFocusable(el) && el.tabIndex === 0 : null,
      selected: el && identity && el.hasAttribute("aria-selected")
        ? el.getAttribute("aria-selected") === "true"
        : null,
      orientation: el?.getAttribute("aria-orientation") ?? null,
      controls: normalizeRelationship(el?.getAttribute("aria-controls") ?? null, iface, root),
      labelledBy: normalizeRelationship(el?.getAttribute("aria-labelledby") ?? null, iface, root),
      geometry,
      channels: el ? channelsOf(el) : {},
    };
  }
  // Root accessible name: aria-label, else the first text-carrying part, else
  // the root's own text — all resolved through the interface, no class names.
  let accessibleName = root.getAttribute("aria-label");
  if (!accessibleName) {
    for (const part of iface.parts) {
      if (part.contains === "text" && parts[part.id]?.text) {
        accessibleName = parts[part.id].text;
        break;
      }
    }
  }
  if (!accessibleName) {
    const trimmed = root.textContent?.trim() ?? "";
    accessibleName = trimmed.length > 0 ? trimmed : null;
  }
  const rootPart = parts.root;
  const rootDecl = iface.parts.find((part) => part.id === "root");
  if (rootDecl?.role && rootPart.name == null) rootPart.name = accessibleName;
  rootPart.states = states;
  rootPart.tokenRoles = tokenRoles;
  rootPart.focusable = isFocusable(root);
  rootPart.focused = states.focused ?? rootPart.focused;
  rootPart.focusVisible = states.focusVisible ?? rootPart.focusVisible;
  rootPart.geometry = geometryOf(root);
  rootPart.channels = channelsOf(root);
  // Root-only identity channels: the focused element's text (focused-part
  // progression) and the open overlay layer count (layer order / overlay
  // state). Both are generic — the component never appears here.
  const active = root.ownerDocument.activeElement;
  rootPart.focusedText =
    active && active !== root.ownerDocument.body && !active.hasAttribute("aria-disabled")
      ? active.textContent?.trim() || null
      : null;
  rootPart.layerCount = dismissLayerStackLength();
  const lower = parts.lower?.value;
  const upper = parts.upper?.value;
  if (typeof lower === "number" && typeof upper === "number") {
    rootPart.value = [lower, upper];
  }
  return { runtime, component, parts, trace: [] };
}

// ── Strict assertion evaluation ────────────────────────────────────────────

function numbersMatch(expected: unknown, actual: unknown, tolerance: number): boolean {
  return (
    typeof expected === "number" &&
    typeof actual === "number" &&
    Math.abs(expected - actual) <= tolerance
  );
}

function check(
  results: AssertionResult[],
  runtime: string,
  stepIndex: number,
  part: string,
  field: string,
  expected: unknown,
  actual: unknown,
  tolerance?: number,
): void {
  if (actual === null || actual === undefined) {
    results.push({
      stepIndex,
      part,
      field,
      verdict: "fail",
      expected,
      reason: `not observed by ${runtime}`,
    });
    return;
  }
  const matches =
    tolerance !== undefined ? numbersMatch(expected, actual, tolerance) : expected === actual;
  results.push({
    stepIndex,
    part,
    field,
    verdict: matches ? "pass" : "fail",
    expected,
    actual,
  });
}

export function assertPartObservation(
  runtime: string,
  partId: string,
  observed: PartObservation | undefined,
  expect: Record<string, unknown>,
  stepIndex: number,
): AssertionResult[] {
  const results: AssertionResult[] = [];
  if (!observed) {
    results.push({
      stepIndex,
      part: partId,
      field: "present",
      verdict: "fail",
      expected: true,
      reason: `not observed by ${runtime}`,
    });
    return results;
  }

  check(
    results,
    runtime,
    stepIndex,
    partId,
    "present",
    expect.present ?? true,
    observed.present,
  );
  if (!observed.present) return results;

  for (const field of [
    "role",
    "name",
    "text",
    "icon",
    "focusable",
    "focused",
    "focusVisible",
    "tabbable",
    "selected",
    "expanded",
    "parent",
    "overlay",
    "focusedText",
    "layerCount",
    "orientation",
    "controls",
    "labelledBy",
    "selectionStart",
    "selectionEnd",
  ] as const) {
    if (expect[field] !== undefined) {
      check(results, runtime, stepIndex, partId, field, expect[field], observed[field]);
    }
  }
  if (expect.value !== undefined) {
    const expected = expect.value;
    const actual = observed.value;
    const matches =
      typeof expected === "string"
        ? expected === actual
        : Array.isArray(expected)
          ? Array.isArray(actual) &&
            expected.length === actual.length &&
            expected.every((entry, index) => numbersMatch(entry, actual[index], 1e-9))
          : numbersMatch(expected, actual, 1e-9);
    if (actual === null || actual === undefined) {
      results.push({
        stepIndex,
        part: partId,
        field: "value",
        verdict: "fail",
        expected,
        reason: `not observed by ${runtime}`,
      });
    } else {
      results.push({
        stepIndex,
        part: partId,
        field: "value",
        verdict: matches ? "pass" : "fail",
        expected,
        actual,
      });
    }
  }
  const states = (expect.states ?? {}) as Record<string, boolean>;
  for (const [state, value] of Object.entries(states)) {
    check(
      results,
      runtime,
      stepIndex,
      partId,
      `state.${state}`,
      value,
      observed.states[state],
    );
  }
  const tokenRoles = (expect.tokenRoles ?? {}) as Record<string, string>;
  for (const [token, value] of Object.entries(tokenRoles)) {
    check(
      results,
      runtime,
      stepIndex,
      partId,
      `tokenRole.${token}`,
      value,
      observed.tokenRoles[token],
    );
  }
  const geometry = (expect.geometry ?? {}) as Record<string, number | undefined>;
  const tolerance = geometry.tolerance;
  if (expect.geometry !== undefined && tolerance === undefined) {
    throw new Error(`${runtime} step ${stepIndex} ${partId}.geometry: missing authored tolerance`);
  }
  for (const field of geometryFields) {
    const expected = geometry[field];
    if (expected === undefined) continue;
    check(
      results,
      runtime,
      stepIndex,
      partId,
      `geometry.${field}`,
      expected,
      observed.geometry[field] ?? null,
      tolerance,
    );
  }
  return results;
}

export function assertEvents(
  trace: TraceEntry[],
  expected: string[],
  stepIndex: number,
): AssertionResult[] {
  const actual = trace.map((entry) => entry.event);
  if (JSON.stringify(actual) === JSON.stringify(expected)) {
    return [{ stepIndex, part: null, field: "events", verdict: "pass", expected, actual }];
  }
  return [{ stepIndex, part: null, field: "events", verdict: "fail", expected, actual }];
}

/** Runs one case against an adapter; returns the per-assertion results. */
export async function runCase(
  adapter: RuntimeAdapter,
  iface: SerializedComponentInterface,
  component: string,
  caseData: SerializedCase,
): Promise<{ results: AssertionResult[]; observations: RuntimeObservation[] }> {
  const results: AssertionResult[] = [];
  const observations: RuntimeObservation[] = [];

  adapter.mount(caseData.fixture);
  await adapter.flush();
  observations.push(capture(adapter, iface, component));

  for (let index = 0; index < caseData.steps.length; index += 1) {
    const step = caseData.steps[index];
    switch (step.kind) {
      case "action": {
        if (step.name === "press") await adapter.press(step.part, (step.input as "pointer" | "keyboard") ?? "pointer");
        else if (step.name === "focus") adapter.focus(step.part);
        else if (step.name === "key") await adapter.key(step.part, step.key ?? "");
        else if (step.name === "dismiss") await adapter.dismiss(step.part);
        else if (step.name === "pointer") {
          await adapter.pointer(step.part, (step.target as "inside" | "outside") ?? "inside");
        }
        else if (step.name === "scrub") {
          await adapter.scrub(
            step.part,
            typeof step.fraction === "number" ? step.fraction : 0,
            (step.phase as "press" | "drag" | "release") ?? "press",
          );
        } else if (step.name === "insert") {
          await adapter.insert?.(step.part, step.text ?? "");
        } else if (step.name === "select") {
          await adapter.select?.(
            step.part,
            typeof step.start === "number" ? step.start : 0,
            typeof step.end === "number" ? step.end : 0,
          );
        } else if (step.name === "compose") {
          await adapter.compose?.(
            step.part,
            step.text ?? "",
            (step.phase as "start" | "update" | "commit") ?? "commit",
          );
        }
        await adapter.flush();
        observations.push(capture(adapter, iface, component));
        break;
      }
      case "expectPart": {
        const observed = capture(adapter, iface, component).parts[step.part];
        results.push(
          ...assertPartObservation(
            adapter.runtime,
            step.part,
            observed,
            step.expect,
            index,
          ),
        );
        break;
      }
      case "expectEvents": {
        results.push(...assertEvents(adapter.trace(), step.events, index));
        break;
      }
    }
  }

  adapter.cleanup();
  return { results, observations };
}

function capture(
  adapter: RuntimeAdapter,
  iface: SerializedComponentInterface,
  component: string,
): RuntimeObservation {
  const root = adapter.rootElement();
  if (!root) {
    return { runtime: adapter.runtime, component, parts: {}, trace: [...adapter.trace()] };
  }
  const observation = observeDom(adapter.runtime, component, iface, root);
  observation.trace = [...adapter.trace()];
  return observation;
}

export function summarize(
  runtime: string,
  component: string,
  caseResults: { caseId: string; results: AssertionResult[]; observations: RuntimeObservation[] }[],
): RuntimeCaseReport {
  const results: CaseResult[] = caseResults.map(({ caseId, results: perCase, observations }) => {
    const failures = perCase.filter((r) => r.verdict === "fail");
    return {
      caseId,
      pass: failures.length === 0,
      failures,
      assertions: perCase,
      observations,
    };
  });
  return { runtime, component, results };
}
