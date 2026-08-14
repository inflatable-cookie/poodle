/**
 * Shared web conformance runner (spec 066): interprets the serialized case
 * corpus against a runtime adapter (Svelte or React), observing the real
 * DOM, and emits per-runtime results with per-assertion verdicts
 * (`pass` | `fail` | `vacuous`). A vacuous verdict means the runtime cannot
 * observe that field (e.g. icon identity from an inline SVG) — recorded, not
 * masked; the orchestrator fails any assertion no runtime can exercise.
 */

import type {
  CaseStep,
  ComponentCase,
  PartExpectation,
  SerializedComponentCases,
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
  states: Record<string, boolean>;
  tokenRoles: Record<string, string>;
  focusable: boolean;
  focused: boolean;
  focusVisible: boolean;
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
  verdict: "pass" | "fail" | "vacuous";
  expected?: unknown;
  actual?: unknown;
}

export interface CaseResult {
  caseId: string;
  pass: boolean;
  failures: AssertionResult[];
  /** Every assertion verdict for this case (pass | fail | vacuous). */
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
  observe(): RuntimeObservation;
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

export function observeRootGeometry(root: HTMLElement): Record<string, number | null> {
  const style = root.ownerDocument.defaultView?.getComputedStyle(root);
  if (!style) return Object.fromEntries(geometryFields.map((f) => [f, null]));
  const out: Record<string, number | null> = {
    height: parseLength(style.height),
    minWidth: parseLength(style.minWidth),
    paddingLeft: parseLength(style.paddingLeft),
    paddingRight: parseLength(style.paddingRight),
    radius: parseLength(style.borderRadius),
    borderWidth: parseLength(style.borderWidth),
  };
  return out;
}

export function observeRootChannels(root: HTMLElement): Record<string, string | null> {
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

/** Per-field compare with a local named tolerance for geometry. */
function fieldMatches(
  field: string,
  expected: unknown,
  actual: unknown,
  tolerance?: number,
): boolean {
  if (field === "icon" && actual === null) return false;
  if (typeof expected === "number" && typeof actual === "number") {
    const tol = tolerance ?? 0.5;
    return Math.abs(expected - actual) <= tol;
  }
  return expected === actual;
}

export function assertPartObservation(
  partId: string,
  observed: PartObservation | undefined,
  expect: PartExpectation,
  stepIndex: number,
): AssertionResult[] {
  const results: AssertionResult[] = [];
  if (!observed) {
    return [
      {
        stepIndex,
        part: partId,
        field: "present",
        verdict: "fail",
        expected: true,
        actual: "part not observed",
      },
    ];
  }

  const check = (
    field: string,
    expected: unknown,
    actual: unknown,
    options: { tolerance?: number; vacuousWhen?: (actual: unknown) => boolean } = {},
  ): void => {
    if (expected === undefined) return;
    const vacuous = options.vacuousWhen ? options.vacuousWhen(actual) : actual === null || actual === undefined;
    if (vacuous) {
      results.push({ stepIndex, part: partId, field, verdict: "vacuous", expected, actual });
      return;
    }
    const match = fieldMatches(field, expected, actual, options.tolerance);
    results.push({
      stepIndex,
      part: partId,
      field,
      verdict: match ? "pass" : "fail",
      expected,
      actual,
    });
  };

  check("present", expect.present ?? true, observed.present);
  check("role", expect.role, observed.role);
  check("name", expect.name, observed.name);
  check("text", expect.text, observed.text);
  check("icon", expect.icon, observed.icon);
  check("focusable", expect.focusable, observed.focusable);
  for (const [state, value] of Object.entries(expect.states ?? {})) {
    const actual = observed.states[state];
    const vacuous = actual === undefined;
    const match = !vacuous && actual === value;
    results.push({
      stepIndex,
      part: partId,
      field: `state.${state}`,
      verdict: vacuous ? "vacuous" : match ? "pass" : "fail",
      expected: value,
      actual: vacuous ? undefined : actual,
    });
  }
  for (const [token, value] of Object.entries(expect.tokenRoles ?? {})) {
    const actual = observed.tokenRoles[token] ?? "";
    const vacuous = actual === "";
    const match = !vacuous && actual === value;
    results.push({
      stepIndex,
      part: partId,
      field: `tokenRole.${token}`,
      verdict: vacuous ? "vacuous" : match ? "pass" : "fail",
      expected: value,
      actual: vacuous ? undefined : actual,
    });
  }
  const tolerance = expect.geometry?.tolerance;
  for (const field of geometryFields) {
    const expected = expect.geometry?.[field];
    if (expected === undefined) continue;
    const actual = observed.geometry[field] ?? null;
    const vacuous = actual === null;
    const match = !vacuous && fieldMatches(field, expected, actual, tolerance);
    results.push({
      stepIndex,
      part: partId,
      field: `geometry.${field}`,
      verdict: vacuous ? "vacuous" : match ? "pass" : "fail",
      expected,
      actual: vacuous ? undefined : actual,
    });
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
  caseData: ComponentCase,
): Promise<{ results: AssertionResult[]; observations: RuntimeObservation[] }> {
  const results: AssertionResult[] = [];
  const observations: RuntimeObservation[] = [];

  adapter.mount(caseData.fixture);
  await adapter.flush();
  observations.push(adapter.observe());

  for (let index = 0; index < caseData.steps.length; index += 1) {
    const step = caseData.steps[index];
    switch (step.kind) {
      case "action": {
        if (step.name === "press") await adapter.press(step.part, step.input ?? "pointer");
        else if (step.name === "focus") adapter.focus(step.part);
        await adapter.flush();
        observations.push(adapter.observe());
        break;
      }
      case "expectPart": {
        const observed = adapter.observe().parts[step.part];
        results.push(...assertPartObservation(step.part, observed, step.expect, index));
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
