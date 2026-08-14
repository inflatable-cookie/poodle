/**
 * Conformance authority schema (spec 066, architecture 009): constrained
 * TypeScript data builders for portable component interfaces and component
 * cases. Declares data, not behaviour — no transitions, guards, derivations,
 * loops, or arbitrary callback bodies.
 *
 * The interface value is the single authority. `defineComponentInterface`
 * takes a `const` generic so literal types survive, and mapped types derive
 * the portable prop/event types, part ids, state names, event names, token
 * roles, and axes from it — there is no second hand-written type mirror.
 * `componentCase(iface, ...)` binds every fixture field, part, state, event,
 * axis, and enum value to the interface at authoring time and again at
 * serialization; unknown names are errors, never ignored.
 */

// ── Interface declarations ─────────────────────────────────────────────────

export type ScalarTypeDef =
  | { kind: "boolean" }
  | { kind: "string" }
  | { kind: "icon" }
  | { kind: "dimension" }
  | { kind: "enum"; values: readonly string[] };

export interface PropDecl {
  /** Portable prop name (web casing). The canonical name. */
  name: string;
  type: ScalarTypeDef;
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

export interface EventDecl<P = object> {
  name: string;
  /** Payload fields as scalar type names. The generic preserves literal
   * payload types under `const` inference (a `Record` constraint widens
   * them, which is how the handler projections lost their value types). */
  payload: P;
}

export interface RegionDecl {
  name: string;
  /** What the region carries. */
  payload: "text" | "icon" | "node";
  multiple?: boolean;
}

/** Web part resolution: how the web adapter finds the part in the DOM. */
export type WebResolution =
  | { kind: "self" }
  | { kind: "class"; className: string }
  | { kind: "icon"; position: "first" | "last"; gatedBy: string };

/** Native part resolution: how the node observer finds the part in the tree. */
export type NativeResolution =
  | { kind: "self" }
  | { kind: "root-label" }
  | { kind: "first-text" }
  | { kind: "icon-side"; side: "leading" | "trailing"; except: readonly string[] }
  | { kind: "icon-named"; name: string };

export interface PartDecl {
  id: string;
  /** Semantic role (exact, e.g. "button"). */
  role?: string;
  /** What the part carries, for observation. */
  contains?: "label" | "text" | "icon";
  resolve: { web: WebResolution; native: NativeResolution };
}

/** Per-runtime state observation rules. */
export interface StateDecl {
  name: string;
  /** Declarative prose; never executed. */
  condition?: string;
  /** How the web adapter observes the state. */
  web: "disabled-attr" | "data-attr" | "aria-pressed" | "active-element" | "focus-visible-pseudo";
  /** The data attribute name for `data-attr`. */
  attr?: string;
  /** How the native observer records the state. */
  native:
    | "interaction-disabled"
    | "part-present"
    | "a11y-toggled"
    | "backend-focus"
    | "focus-with-focus-style";
  /** The part id for `part-present`. */
  part?: string;
}

export interface TokenRoleDecl {
  name: string;
  /** The prop the role projects. */
  prop: string;
  /** The role value when the projection is absent (web omits it). */
  default?: string;
}

export interface CapabilityDecl {
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

export interface InterfaceConfig {
  id: string;
  profile: ComponentProfile;
  props: readonly PropDecl[];
  events: readonly EventDecl[];
  regions: readonly RegionDecl[];
  parts: readonly PartDecl[];
  states: readonly StateDecl[];
  tokenRoles: readonly TokenRoleDecl[];
  /** Specimen axes: enum props plus any global axes ("theme"). */
  axes: readonly string[];
  capabilities: readonly CapabilityDecl[];
}

export interface InterfaceValue extends InterfaceConfig {
  /** Brand so only `defineComponentInterface` produces interface values. */
  readonly __interface: unique symbol;
}

/** Validates and brands an interface. Literal types are preserved by the
 * `const` generic; derived types below project from this value alone. */
export function defineComponentInterface<const C extends InterfaceConfig>(config: C): C & InterfaceValue {
  validateInterface(config);
  return config as C & InterfaceValue;
}

export function validateInterface(config: InterfaceConfig): void {
  const names = new Set<string>();
  const propNames = new Set<string>();
  for (const prop of config.props) {
    if (propNames.has(prop.name)) throw new Error(`duplicate prop '${prop.name}'`);
    propNames.add(prop.name);
    if (prop.type.kind === "enum" && prop.type.values.length === 0) {
      throw new Error(`enum prop '${prop.name}' needs values`);
    }
  }
  const eventNames = new Set<string>();
  for (const event of config.events) {
    if (eventNames.has(event.name)) throw new Error(`duplicate event '${event.name}'`);
    eventNames.add(event.name);
  }
  for (const region of config.regions) {
    if (names.has(`region:${region.name}`)) throw new Error(`duplicate region '${region.name}'`);
    names.add(`region:${region.name}`);
  }
  const partIds = new Set<string>();
  for (const part of config.parts) {
    if (partIds.has(part.id)) throw new Error(`duplicate part '${part.id}'`);
    partIds.add(part.id);
    if (!part.role && !part.contains && part.resolve.native.kind !== "self") {
      throw new Error(`part '${part.id}' needs a role or a payload`);
    }
  }
  if (!partIds.has("root")) throw new Error("interface needs a 'root' part");
  for (const state of config.states) {
    if (names.has(`state:${state.name}`)) throw new Error(`duplicate state '${state.name}'`);
    names.add(`state:${state.name}`);
    if (state.native === "part-present" && !state.part) {
      throw new Error(`state '${state.name}' needs a part for part-present observation`);
    }
  }
  const tokenNames = new Set<string>();
  for (const role of config.tokenRoles) {
    if (tokenNames.has(role.name)) throw new Error(`duplicate token role '${role.name}'`);
    tokenNames.add(role.name);
    if (!propNames.has(role.prop)) {
      throw new Error(`token role '${role.name}' names unknown prop '${role.prop}'`);
    }
  }
  const axisNames = new Set<string>();
  for (const axis of config.axes) {
    if (axisNames.has(axis)) throw new Error(`duplicate axis '${axis}'`);
    axisNames.add(axis);
    if (axis !== "theme" && !propNames.has(axis)) {
      throw new Error(`axis '${axis}' names unknown prop`);
    }
  }
  for (const prop of config.props) {
    if (prop.controlledBy && !eventNames.has(prop.controlledBy)) {
      throw new Error(`prop '${prop.name}' controls unknown event '${prop.controlledBy}'`);
    }
  }
}

// ── Type-level projections ─────────────────────────────────────────────────

export type PortablePropsOf<I extends InterfaceConfig> = {
  [P in I["props"][number] as P extends { extension: string }
    ? never
    : P["name"]]: ScalarToTs<P["type"], P extends { nullable: true } ? true : false>;
};

export type ScalarToTs<S extends ScalarTypeDef, Nullable extends boolean> =
  S extends { kind: "boolean" }
    ? Nullable extends true ? boolean | null : boolean
    : S extends { kind: "enum"; values: readonly string[] }
      ? Nullable extends true ? S["values"][number] | null : S["values"][number]
      : S extends { kind: "string" | "icon" | "dimension" }
        ? Nullable extends true ? string | null : string
        : never;

export type PayloadToTs<P> = {
  [K in keyof P]: P[K] extends "boolean" ? boolean : P[K] extends "number" ? number : string;
};

/** Handler arguments for an event payload: no payload → no args; a
 * single-field payload → the field value; more fields → the object. */
export type PayloadArgs<P> = keyof P extends never
  ? []
  : PayloadToTs<P> extends infer O
    ? [O[keyof O]]
    : never;

export type PortableEventsOf<I extends InterfaceConfig> = {
  [E in I["events"][number] as E["name"]]: (...args: PayloadArgs<E["payload"]>) => void;
};

export type PartIdsOf<I extends InterfaceConfig> = I["parts"][number]["id"];
export type StateNamesOf<I extends InterfaceConfig> = I["states"][number]["name"];
export type EventNamesOf<I extends InterfaceConfig> = I["events"][number]["name"];
export type TokenRoleNamesOf<I extends InterfaceConfig> = I["tokenRoles"][number]["name"];
export type AxisNamesOf<I extends InterfaceConfig> = I["axes"][number];
export type RegionNamesOf<I extends InterfaceConfig> = I["regions"][number]["name"];
export type EnumValuesOf<P extends PropDecl> = P["type"] extends {
  kind: "enum";
  values: readonly string[];
}
  ? P["type"]["values"][number]
  : never;

/** The fixture value type for one prop. */
export type FixtureValueOf<P extends PropDecl> =
  P extends { extension: string }
    ? never
    : ScalarToTs<P["type"], P extends { nullable: true } ? true : false>;

// ── Serialized forms ───────────────────────────────────────────────────────

export interface SerializedComponentInterface extends InterfaceConfig {
  schemaVersion: 1;
}

/** The loose, runtime-shaped case step in the neutral JSON. */
export type SerializedCaseStep =
  | { kind: "action"; name: string; part: string; input?: string }
  | { kind: "expectPart"; part: string; expect: Record<string, unknown> }
  | { kind: "expectEvents"; events: string[] };

/** The loose, runtime-shaped case in the neutral JSON. */
export interface SerializedCase {
  id: string;
  fixture: { props: Record<string, string | boolean | number | null>; regions: Record<string, string> };
  specimen: { group: string; caption: string; axes: string[]; captureId: string };
  steps: SerializedCaseStep[];
}

export interface SerializedComponentCases {
  schemaVersion: 1;
  component: string;
  cases: SerializedCase[];
}

// ── Component cases ────────────────────────────────────────────────────────

export type CaseFixture<I extends InterfaceConfig> = {
  props: Partial<{ [K in PortablePropNamesOf<I>]: FixtureValueOf<PropByName<I, K>> }>;
  regions: Partial<Record<RegionNamesOf<I>, string>>;
};

type PortablePropNamesOf<I extends InterfaceConfig> = Extract<
  I["props"][number],
  { extension?: never }
>["name"];
type PropByName<I extends InterfaceConfig, N> = Extract<I["props"][number], { name: N }>;

export interface CaseSpecimen<I extends InterfaceConfig> {
  group: string;
  caption: string;
  /** Axis names from the interface (theme is always a valid axis). */
  axes: readonly AxisNamesOf<I>[];
  captureId: string;
}

export interface PartExpectation<I extends InterfaceConfig> {
  role?: string;
  name?: string;
  text?: string;
  present?: boolean;
  focusable?: boolean;
  states?: Partial<Record<StateNamesOf<I>, boolean>>;
  tokenRoles?: Partial<Record<TokenRoleNamesOf<I>, string>>;
  geometry?: Partial<
    Record<"height" | "minWidth" | "paddingLeft" | "paddingRight" | "radius" | "borderWidth", number>
  > & { tolerance?: number };
}

export type CaseStep<I extends InterfaceConfig> =
  | { kind: "action"; name: "press" | "focus"; part: PartIdsOf<I>; input?: "pointer" | "keyboard" }
  | { kind: "expectPart"; part: PartIdsOf<I>; expect: PartExpectation<I> }
  | { kind: "expectEvents"; events: EventNamesOf<I>[] };

export interface ComponentCase<I extends InterfaceConfig = InterfaceConfig> {
  id: string;
  fixture: CaseFixture<I>;
  specimen: CaseSpecimen<I>;
  steps: readonly CaseStep<I>[];
}

export function componentCase<I extends InterfaceConfig>(
  iface: I,
  config: ComponentCase<I>,
): ComponentCase<I> {
  validateCase(iface, config);
  return config;
}

export function validateCase<I extends InterfaceConfig>(
  iface: I,
  config: ComponentCase<I>,
): void {
  if (!config.id || config.id.split("/").length !== 2) {
    throw new Error(`case id must be '<component>/<name>', got '${config.id}'`);
  }
  if (!config.specimen.group || !config.specimen.caption) {
    throw new Error(`case '${config.id}' needs specimen group and caption`);
  }
  const propNames = new Set(iface.props.map((p) => p.name));
  for (const [key, value] of Object.entries(config.fixture.props)) {
    const prop = iface.props.find((p) => p.name === key);
    if (!prop) {
      throw new Error(`case '${config.id}' uses unknown prop '${key}'`);
    }
    if (prop.type.kind === "enum" && value !== null) {
      if (!prop.type.values.includes(String(value))) {
        throw new Error(
          `case '${config.id}' prop '${key}' value '${value}' is not one of ${prop.type.values.join(", ")}`,
        );
      }
    }
  }
  void propNames;
  const regionNames = new Set(iface.regions.map((r) => r.name));
  for (const key of Object.keys(config.fixture.regions)) {
    if (!regionNames.has(key)) {
      throw new Error(`case '${config.id}' uses unknown region '${key}'`);
    }
  }
  const partIds = new Set(iface.parts.map((p) => p.id));
  const stateNames = new Set(iface.states.map((s) => s.name));
  const eventNames = new Set(iface.events.map((e) => e.name));
  const tokenNames = new Set(iface.tokenRoles.map((t) => t.name));
  const axisNames = new Set(iface.axes);
  for (const axis of config.specimen.axes) {
    if (!axisNames.has(axis)) {
      throw new Error(`case '${config.id}' uses unknown axis '${axis}'`);
    }
  }
  let sawAction = false;
  for (const step of config.steps) {
    switch (step.kind) {
      case "action": {
        sawAction = true;
        if (!partIds.has(step.part)) {
          throw new Error(`case '${config.id}' action targets unknown part '${step.part}'`);
        }
        break;
      }
      case "expectPart": {
        if (!partIds.has(step.part)) {
          throw new Error(`case '${config.id}' expects unknown part '${step.part}'`);
        }
        for (const state of Object.keys(step.expect.states ?? {})) {
          if (!stateNames.has(state)) {
            throw new Error(`case '${config.id}' expects unknown state '${state}'`);
          }
        }
        for (const token of Object.keys(step.expect.tokenRoles ?? {})) {
          if (!tokenNames.has(token)) {
            throw new Error(`case '${config.id}' expects unknown token role '${token}'`);
          }
        }
        break;
      }
      case "expectEvents": {
        for (const event of step.events) {
          if (!eventNames.has(event)) {
            throw new Error(`case '${config.id}' expects unknown event '${event}'`);
          }
        }
        break;
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

/** JSON-stable serialization: key order fixed by construction, no undefined. */
export function serializeInterface<I extends InterfaceConfig>(iface: I): SerializedComponentInterface {
  const serialized: SerializedComponentInterface = {
    schemaVersion: 1,
    id: iface.id,
    profile: iface.profile,
    props: [...iface.props],
    events: [...iface.events],
    regions: [...iface.regions],
    parts: [...iface.parts],
    states: [...iface.states],
    tokenRoles: [...iface.tokenRoles],
    axes: [...iface.axes],
    capabilities: [...iface.capabilities],
  };
  validateInterface(serialized);
  return serialized;
}

/** JSON-stable serialization. */
export function serializeCases<I extends InterfaceConfig>(
  component: string,
  cases: readonly ComponentCase<I>[],
): SerializedComponentCases {
  return {
    schemaVersion: 1,
    component,
    cases: cases.map((c) => ({
      id: c.id,
      fixture: {
        props: { ...c.fixture.props } as Record<string, string | boolean | number | null>,
        regions: { ...c.fixture.regions } as Record<string, string>,
      },
      specimen: {
        group: c.specimen.group,
        caption: c.specimen.caption,
        axes: [...c.specimen.axes] as string[],
        captureId: c.specimen.captureId,
      },
      steps: c.steps.map((step) =>
        step.kind === "action"
          ? { kind: "action", name: step.name, part: step.part, ...(step.input ? { input: step.input } : {}) }
          : step.kind === "expectPart"
            ? { kind: "expectPart", part: step.part, expect: { ...step.expect } as Record<string, unknown> }
            : { kind: "expectEvents", events: [...step.events] },
      ),
    })),
  };
}

export function actionPress<I extends InterfaceConfig>(
  part: PartIdsOf<I>,
  input: "pointer" | "keyboard" = "pointer",
): CaseStep<I> {
  return { kind: "action", name: "press", part, input };
}

export function actionFocus<I extends InterfaceConfig>(part: PartIdsOf<I>): CaseStep<I> {
  return { kind: "action", name: "focus", part };
}

export function expectPart<I extends InterfaceConfig>(
  part: PartIdsOf<I>,
  expect: PartExpectation<I>,
): CaseStep<I> {
  return { kind: "expectPart", part, expect };
}

export function expectEvents<I extends InterfaceConfig>(events: EventNamesOf<I>[]): CaseStep<I> {
  return { kind: "expectEvents", events };
}
