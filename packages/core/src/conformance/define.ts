/**
 * Conformance authority schema (spec 066, architecture 009): constrained
 * TypeScript data builders for portable component interfaces and component
 * cases. Declares data, not behaviour — no transitions, guards, derivations,
 * loops, or arbitrary callback bodies. The serialized forms are consumed by
 * the Rust pipeline (`poodle-codegen --conformance` and the native runners).
 *
 * This module is the pilot-scaled subset of the schema: exactly what Button
 * needs, with the extension points spec 066 names (profiles, capabilities,
 * parts, geometry tolerances).
 */

export interface PortableScalarType {
  kind: "boolean" | "string" | "icon" | "dimension" | "enum";
  values?: readonly string[];
}

export interface PortableProp {
  /** Portable prop name (web casing). The canonical name. */
  name: string;
  type: PortableScalarType;
  /** Literal default, serializable to JSON. */
  default: boolean | string | number | null;
  /** Accepts null (absent value) in addition to the default. */
  nullable?: boolean;
  /** Rust field name when not the mechanical camelCase → snake_case form. */
  rustName?: string;
  /** Rust type in `crate::types` for enum props (e.g. "ButtonVariant"). */
  rustType?: string;
  /** Generated Rust enum name when the values are not a poodle type. */
  rustEnumName?: string;
  /** Controlled-state pair: the event carrying the new value. */
  controlledBy?: string;
  /** Platform extension marker. Extensions are not portable. */
  extension?: string;
}

export interface PortableEvent {
  name: string;
  /** Payload fields as scalar type names. */
  payload: Record<string, "boolean" | "string" | "number">;
}

export interface PortableRegion {
  name: string;
  /** What the region carries. */
  payload: "text" | "icon" | "node";
  multiple?: boolean;
}

export interface PortablePart {
  id: string;
  /** Semantic role (exact, e.g. "button"). */
  role?: string;
  /** What the part carries, for observation. */
  contains?: "label" | "text" | "icon";
}

export interface PortableState {
  name: string;
  /** Declarative prose; never executed. */
  condition?: string;
}

export interface PortableCapability {
  name: string;
  required: boolean;
}

export type ComponentProfile =
  | "display"
  | "control"
  | "collection"
  | "overlay"
  | "input"
  | "composite";

export interface ComponentInterface {
  id: string;
  profile: ComponentProfile;
  props: readonly PortableProp[];
  events: readonly PortableEvent[];
  regions: readonly PortableRegion[];
  parts: readonly PortablePart[];
  states: readonly PortableState[];
  capabilities: readonly PortableCapability[];
}

/** Serialized form of a component interface (consumed by Rust). */
export interface SerializedComponentInterface {
  schemaVersion: 1;
  id: string;
  profile: ComponentProfile;
  props: PortableProp[];
  events: PortableEvent[];
  regions: PortableRegion[];
  parts: PortablePart[];
  states: PortableState[];
  capabilities: PortableCapability[];
}

export function defineComponentInterface(config: ComponentInterface) {
  validateInterface(config);
  return config;
}

export function validateInterface(config: ComponentInterface): void {
  const ids = new Set<string>();
  for (const prop of config.props) {
    if (ids.has(prop.name)) throw new Error(`duplicate prop '${prop.name}'`);
    ids.add(prop.name);
    if (prop.type.kind === "enum" && !prop.type.values?.length) {
      throw new Error(`enum prop '${prop.name}' needs values`);
    }
  }
  for (const event of config.events) {
    if (ids.has(`event:${event.name}`)) {
      throw new Error(`duplicate event '${event.name}'`);
    }
    ids.add(`event:${event.name}`);
  }
  for (const region of config.regions) {
    if (ids.has(`region:${region.name}`)) {
      throw new Error(`duplicate region '${region.name}'`);
    }
    ids.add(`region:${region.name}`);
  }
  const parts = new Set<string>();
  for (const part of config.parts) {
    if (parts.has(part.id)) throw new Error(`duplicate part '${part.id}'`);
    parts.add(part.id);
    if (!part.role && !part.contains) {
      throw new Error(`part '${part.id}' needs a role or a payload`);
    }
  }
  if (!parts.has("root")) throw new Error("interface needs a 'root' part");
  const states = new Set<string>();
  for (const state of config.states) {
    if (states.has(state.name)) throw new Error(`duplicate state '${state.name}'`);
    states.add(state.name);
  }
  for (const prop of config.props) {
    if (prop.controlledBy && !config.events.some((e) => e.name === prop.controlledBy)) {
      throw new Error(`prop '${prop.name}' controls unknown event '${prop.controlledBy}'`);
    }
  }
}

/** JSON-stable serialization: key order fixed by construction, no undefined. */
export function serializeInterface(config: ComponentInterface): SerializedComponentInterface {
  return {
    schemaVersion: 1,
    id: config.id,
    profile: config.profile,
    props: [...config.props],
    events: [...config.events],
    regions: [...config.regions],
    parts: [...config.parts],
    states: [...config.states],
    capabilities: [...config.capabilities],
  };
}

// ── Component cases ────────────────────────────────────────────────────────

export interface CaseFixture {
  props: Record<string, boolean | string | number | null>;
  regions: Record<string, string>;
}

export interface CaseSpecimen {
  group: string;
  caption: string;
  /** Axis names from the interface (theme is always a valid axis). */
  axes: readonly string[];
  captureId: string;
}

export interface PartExpectation {
  role?: string;
  name?: string;
  text?: string;
  icon?: string;
  focusable?: boolean;
  present?: boolean;
  states?: Record<string, boolean>;
  tokenRoles?: Record<string, string>;
  geometry?: Partial<Record<"height" | "minWidth" | "paddingLeft" | "paddingRight" | "radius" | "borderWidth", number>> &
    { tolerance?: number };
}

export type CaseStep =
  | { kind: "action"; name: "press" | "focus"; part: string; input?: "pointer" | "keyboard" }
  | { kind: "expectPart"; part: string; expect: PartExpectation }
  | { kind: "expectEvents"; events: string[] };

export interface ComponentCase {
  id: string;
  fixture: CaseFixture;
  specimen: CaseSpecimen;
  steps: readonly CaseStep[];
}

export interface SerializedComponentCases {
  schemaVersion: 1;
  component: string;
  cases: ComponentCase[];
}

export function componentCase(config: ComponentCase) {
  validateCase(config);
  return config;
}

export function validateCase(config: ComponentCase): void {
  if (!config.id || config.id.split("/").length !== 2) {
    throw new Error(`case id must be '<component>/<name>', got '${config.id}'`);
  }
  if (!config.specimen.group || !config.specimen.caption) {
    throw new Error(`case '${config.id}' needs specimen group and caption`);
  }
  let sawAction = false;
  for (const step of config.steps) {
    if (step.kind === "action") {
      sawAction = true;
      if (!["press", "focus"].includes(step.name)) {
        throw new Error(`case '${config.id}' uses unknown action '${step.name}'`);
      }
    }
  }
  if (sawAction) {
    const firstAction = config.steps.findIndex((step) => step.kind === "action");
    const eventsAfterAction = config.steps
      .slice(firstAction + 1)
      .some((step) => step.kind === "expectEvents");
    if (!eventsAfterAction) {
      throw new Error(`case '${config.id}' needs an expectEvents step after its first action`);
    }
  }
}

/** JSON-stable serialization. */
export function serializeCases(
  component: string,
  cases: readonly ComponentCase[],
): SerializedComponentCases {
  return { schemaVersion: 1, component, cases: [...cases] };
}

export function actionPress(part = "root", input: "pointer" | "keyboard" = "pointer"): CaseStep {
  return { kind: "action", name: "press", part, input };
}

export function actionFocus(part = "root"): CaseStep {
  return { kind: "action", name: "focus", part };
}

export function expectPart(part: string, expect: PartExpectation): CaseStep {
  return { kind: "expectPart", part, expect };
}

export function expectEvents(events: string[]): CaseStep {
  return { kind: "expectEvents", events };
}
