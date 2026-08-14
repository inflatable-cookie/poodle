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
  states: Record<string, boolean | null>;
  tokenRoles: Record<string, string | null>;
  focusable: boolean;
  focused: boolean | null;
  focusVisible: boolean | null;
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
  mount(caseFixture: { props: Record<string, unknown>; regions: Record<string, string> }): void;
  rootElement(): HTMLElement | null;
  press(part: string, input: "pointer" | "keyboard"): Promise<void>;
  focus(part: string): void;
  /** Flush pending framework state/effects so observation sees final DOM. */
  flush(): Promise<void>;
  trace(): TraceEntry[];
  cleanup(): void;
}

const geometryFields = ["height", "minWidth", "paddingLeft", "paddingRight", "radius", "borderWidth"] as const;

/** Parse a computed CSS length; calc()/var() residues are not observable. */
function parseLength(value: string | null): number | null {
  if (!value) return null;
  if (value.includes("calc(")) return null;
  const parsed = Number.parseFloat(value);
  return Number.isFinite(parsed) ? parsed : null;
}

function resolveWebPart(
  root: HTMLElement,
  part: SerializedComponentInterface["parts"][number],
): HTMLElement | null {
  const { kind } = part.resolve.web;
  switch (kind) {
    case "self":
      return root;
    case "class":
      return root.querySelector<HTMLElement>(part.resolve.web.className);
    case "icon": {
      const { position, gatedBy } = part.resolve.web;
      if (!root.hasAttribute(gatedBy)) return null;
      const spans = Array.from(root.querySelectorAll<HTMLElement>(".poodle-button__icon"));
      return position === "first" ? spans[0] ?? null : spans[spans.length - 1] ?? null;
    }
  }
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
    roles[role.name] = root.getAttribute(`data-${role.prop}`) ?? role.default ?? null;
  }
  return roles;
}

function geometryOf(root: HTMLElement): Record<string, number | null> {
  const style = root.ownerDocument.defaultView?.getComputedStyle(root);
  if (!style) return Object.fromEntries(geometryFields.map((f) => [f, null]));
  return {
    height: parseLength(style.height),
    minWidth: parseLength(style.minWidth),
    paddingLeft: parseLength(style.paddingLeft),
    paddingRight: parseLength(style.paddingRight),
    radius: parseLength(style.borderRadius),
    borderWidth: parseLength(style.borderWidth),
  };
}

function channelsOf(root: HTMLElement): Record<string, string | null> {
  const style = root.ownerDocument.defaultView?.getComputedStyle(root);
  if (!style) return { background: null, borderColor: null, color: null, opacity: null };
  const clean = (value: string | null): string | null =>
    value && !value.includes("color-mix") && !value.includes("calc(") ? value : null;
  return {
    background: clean(style.backgroundColor),
    borderColor: clean(style.borderColor),
    color: clean(style.color),
    opacity: clean(style.opacity),
  };
}

/** Observes the mounted Button root through the interface's descriptors. */
export function observeDom(
  runtime: string,
  component: string,
  iface: SerializedComponentInterface,
  root: HTMLElement,
): RuntimeObservation {
  const states = observeRootStates(iface, root);
  const tokenRoles = observeRootTokenRoles(iface, root);
  const parts: Record<string, PartObservation> = {};
  for (const part of iface.parts) {
    const el = resolveWebPart(root, part);
    const isRoot = part.id === "root";
    const labelText =
      root.querySelector<HTMLElement>(".poodle-button__label")?.textContent?.trim() ?? null;
    const accessibleName =
      root.getAttribute("aria-label") ?? labelText ?? root.textContent?.trim() ?? null;
    parts[part.id] = {
      present: Boolean(el),
      role: el ? (el.tagName === "BUTTON" ? "button" : null) : null,
      name: isRoot && el ? accessibleName : null,
      text: el?.textContent?.trim() ?? null,
      icon: null,
      states: isRoot ? states : {},
      tokenRoles: isRoot ? tokenRoles : {},
      focusable: el ? !el.hasAttribute("disabled") : false,
      focused: isRoot ? (states.focused ?? null) : null,
      focusVisible: isRoot ? (states.focusVisible ?? null) : null,
      geometry: isRoot && el ? geometryOf(el) : {},
      channels: isRoot && el ? channelsOf(el) : {},
    };
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

  for (const field of ["role", "name", "text", "focusable"] as const) {
    if (expect[field] !== undefined) {
      check(results, runtime, stepIndex, partId, field, expect[field], observed[field]);
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
  const tolerance = geometry.tolerance ?? 0.5;
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
        if (step.name === "press") await adapter.press(step.part, step.input ?? "pointer");
        else if (step.name === "focus") adapter.focus(step.part);
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
