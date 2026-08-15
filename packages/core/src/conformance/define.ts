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
 *
 * Capability names are closed over the primitive roster (g14.002).
 */

import { assertKnownCapabilities, type PrimitiveCapabilityId } from "./primitives";

// ── Interface declarations ─────────────────────────────────────────────────

export type ItemScalarTypeDef =
  | { kind: "boolean" }
  | { kind: "string" }
  | { kind: "icon" }
  | { kind: "number" }
  /** A nested collection field (g14.007): structured fixture data whose items
   * are themselves records. One level of nesting is what a composite's page
   * → entry shape needs; the Rust item type is hand-written in `crate::types`
   * exactly like a top-level collection's. */
  | { kind: "collection"; fields: readonly ItemFieldDecl[]; rustType: string };

export interface ItemFieldDecl {
  name: string;
  type: ItemScalarTypeDef;
  optional?: boolean;
}

export type ScalarTypeDef =
  | ItemScalarTypeDef
  | { kind: "dimension" }
  | { kind: "remDimension" }
  | { kind: "numberPair" }
  | { kind: "collection"; fields: readonly ItemFieldDecl[]; rustType: string }
  | { kind: "enum"; values: readonly string[] };

export interface PropDecl {
  /** Portable prop name (web casing). The canonical name. */
  name: string;
  type: ScalarTypeDef;
  /** Literal default, serializable to JSON. */
  default: boolean | string | number | readonly [number, number] | readonly object[] | null;
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
  /** Framework carrier for the web handler (e.g. "mouse-event" for press).
   * The public web callback keeps its framework shape; the carrier marks
   * how the semantic event surfaces on web. */
  webCarrier?: "mouse-event";
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
  | {
      kind: "class";
      className: string;
      attribute?: string;
      keyAttribute?: string;
      /** Ancestor that carries the key when the keyed element belongs to a
       * composed component. A composite owns the wrapper it puts the anchor
       * on; it does not own the trigger a `Select` renders inside it, so the
       * part resolves as "the target inside the ancestor carrying this key". */
      keyScope?: string;
    }
  | { kind: "icon"; position: "first" | "last"; gatedBy: string; selector: string; attribute: string };

/** Native part resolution: how the node observer finds the part in the tree. */
export type NativeResolution =
  | { kind: "self" }
  | { kind: "id"; id: string }
  | { kind: "id-template"; template: string }
  | { kind: "root-label" }
  | { kind: "first-text" }
  | { kind: "icon-side"; side: "leading" | "trailing"; except: readonly string[] }
  | { kind: "icon-named"; name: string };

/** Where a repeated part's keys come from: a portable collection prop, or a
 * declared host record channel. `path` walks into one nested collection field
 * first, so a composite's page → entry records key like a flat roster does. */
export type RepeatSource =
  | { prop: string; path?: string }
  | { hostRecord: string; path?: string };

export interface RepeatDecl {
  /** One or more record sources. A composite's rows come from the spine the
   * host passed in and from the runs it answered a command with; both are
   * fixture data and one part renders both, so identity is one key space. */
  sources: readonly RepeatSource[];
  key: string;
  webIdPrefix?: string;
}

/** A host fixture record channel (g14.007): the data a per-component fixture
 * host answers a named host command from. Declared — not opaque — so its
 * identity is validated and repeated parts can key over it. Host records are
 * never portable props: the component receives them through its callbacks'
 * results, so they generate no Rust spec field. */
export interface HostRecordDecl {
  name: string;
  /** The portable event these records answer. */
  answers: string;
  fields: readonly ItemFieldDecl[];
}

export interface PartDecl {
  id: string;
  /** Semantic role (exact, e.g. "button"). */
  role?: string;
  /** What the part carries, for observation. */
  contains?: "label" | "text" | "icon";
  /** Repeated part keyed by one field of its record sources. Cases address it
   * as `<id>:<semantic-key>`; runtime indices never enter the corpus. */
  repeat?: RepeatDecl;
  /** The part this part is positioned relative to (an anchored surface's
   * trigger). Observers record relative logical bounds gaps against it. */
  relativeTo?: string;
  resolve: { web: WebResolution; native: NativeResolution };
}

/** Per-runtime state observation rules. */
export interface StateDecl {
  name: string;
  /** Declarative prose; never executed. */
  condition?: string;
  /** How the web adapter observes the state. */
  web:
    | "disabled-attr"
    | "data-attr"
    | "aria-pressed"
    | "active-element"
    | "focus-visible-pseudo"
    | "part-disabled-attr"
    | "part-active-element"
    | "part-present";
  /** The data attribute name for `data-attr`. */
  attr?: string;
  /** How the native observer records the state. */
  native:
    | "interaction-disabled"
    | "part-present"
    | "a11y-toggled"
    | "backend-focus"
    | "focus-with-focus-style"
    | "part-interaction-disabled"
    | "part-backend-focus";
  /** The part id for `part-present` / part-scoped state observation. */
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
  name: PrimitiveCapabilityId;
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
  /** Declared host fixture record channels. Absent for components whose
   * fixtures need no host-answered data. */
  hostRecords?: readonly HostRecordDecl[];
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

/** Collection item fields: names are unique and nesting stops at one level.
 * A record inside a record inside a record is a tree, and a tree in fixture
 * data is a second component model — the thing spec 066 forbids. */
function assertItemFields(
  owner: string,
  fields: readonly ItemFieldDecl[],
  depthBudget: number,
): void {
  const names = new Set<string>();
  for (const field of fields) {
    if (names.has(field.name)) {
      throw new Error(`collection '${owner}' has duplicate field '${field.name}'`);
    }
    names.add(field.name);
    if (field.type.kind !== "collection") continue;
    if (depthBudget <= 0) {
      throw new Error(
        `collection '${owner}' field '${field.name}' nests deeper than one level`,
      );
    }
    assertItemFields(`${owner}.${field.name}`, field.type.fields, depthBudget - 1);
  }
}

function sourceLabel(source: RepeatSource): string {
  const base = "prop" in source ? `prop '${source.prop}'` : `host record '${source.hostRecord}'`;
  return source.path ? `${base}.${source.path}` : base;
}

/** The record fields one repeat source yields, walking an optional nested
 * collection field first. */
function resolveSourceFields(
  config: InterfaceConfig,
  source: RepeatSource,
  owner: string,
): readonly ItemFieldDecl[] {
  let fields: readonly ItemFieldDecl[];
  if ("prop" in source) {
    const prop = config.props.find((candidate) => candidate.name === source.prop);
    if (!prop || prop.type.kind !== "collection") {
      throw new Error(`${owner} repeats unknown collection prop '${source.prop}'`);
    }
    fields = prop.type.fields;
  } else {
    const record = config.hostRecords?.find((candidate) => candidate.name === source.hostRecord);
    if (!record) {
      throw new Error(`${owner} repeats unknown host record '${source.hostRecord}'`);
    }
    fields = record.fields;
  }
  if (!source.path) return fields;
  const nested = fields.find((field) => field.name === source.path);
  if (!nested || nested.type.kind !== "collection") {
    throw new Error(`${owner} repeat path '${source.path}' is not a nested collection of ${sourceLabel({ ...source, path: undefined } as RepeatSource)}`);
  }
  return nested.type.fields;
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
    if (prop.type.kind === "collection") {
      assertItemFields(prop.name, prop.type.fields, 1);
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
    if (part.repeat) {
      const { sources, key } = part.repeat;
      if (sources.length === 0) {
        throw new Error(`repeated part '${part.id}' needs at least one record source`);
      }
      for (const source of sources) {
        const fields = resolveSourceFields(config, source, `part '${part.id}'`);
        const keyField = fields.find((field) => field.name === key);
        if (!keyField) {
          throw new Error(`part '${part.id}' repeats unknown key '${key}' in ${sourceLabel(source)}`);
        }
        if (keyField.type.kind !== "string") {
          throw new Error(`part '${part.id}' repeated key '${key}' must be a string field`);
        }
      }
      if (part.resolve.web.kind !== "class" || !part.resolve.web.keyAttribute) {
        throw new Error(`repeated part '${part.id}' needs a web class keyAttribute`);
      }
      if (part.resolve.native.kind !== "id-template" || !part.resolve.native.template.includes("{key}")) {
        throw new Error(`repeated part '${part.id}' needs a native id-template containing '{key}'`);
      }
    }
    if (!part.role && !part.contains && part.resolve.native.kind !== "self") {
      throw new Error(`part '${part.id}' needs a role or a payload`);
    }
  }
  if (!partIds.has("root")) throw new Error("interface needs a 'root' part");
  for (const part of config.parts) {
    if (part.relativeTo && !partIds.has(part.relativeTo)) {
      throw new Error(`part '${part.id}' relativeTo names unknown part '${part.relativeTo}'`);
    }
  }
  for (const state of config.states) {
    if (names.has(`state:${state.name}`)) throw new Error(`duplicate state '${state.name}'`);
    names.add(`state:${state.name}`);
    if (
      (state.native === "part-present" || state.web === "part-present") &&
      !state.part
    ) {
      throw new Error(`state '${state.name}' needs a part for part-present observation`);
    }
    if (
      (state.web === "part-disabled-attr" ||
        state.web === "part-active-element" ||
        state.native === "part-interaction-disabled" ||
        state.native === "part-backend-focus") &&
      !state.part
    ) {
      throw new Error(`state '${state.name}' needs a part for part-scoped observation`);
    }
    if (state.part && !partIds.has(state.part)) {
      throw new Error(`state '${state.name}' names unknown part '${state.part}'`);
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
  const hostRecordNames = new Set<string>();
  for (const record of config.hostRecords ?? []) {
    if (hostRecordNames.has(record.name)) {
      throw new Error(`duplicate host record '${record.name}'`);
    }
    hostRecordNames.add(record.name);
    if (!eventNames.has(record.answers)) {
      throw new Error(`host record '${record.name}' answers unknown event '${record.answers}'`);
    }
    if (propNames.has(record.name)) {
      throw new Error(`host record '${record.name}' collides with a portable prop`);
    }
    assertItemFields(`host record '${record.name}'`, record.fields, 1);
  }
  const capabilityNames = new Set<string>();
  for (const capability of config.capabilities) {
    if (capabilityNames.has(capability.name)) {
      throw new Error(`duplicate capability '${capability.name}'`);
    }
    capabilityNames.add(capability.name);
  }
  assertKnownCapabilities(
    config.capabilities.map((capability) => capability.name),
    `interface '${config.id}'`,
  );
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
    : S extends { kind: "number" }
      ? Nullable extends true ? number | null : number
      : S extends { kind: "numberPair" }
        ? Nullable extends true ? [number, number] | null : [number, number]
        : S extends { kind: "collection"; fields: readonly ItemFieldDecl[] }
          ? Nullable extends true ? CollectionItemToTs<S["fields"]>[] | null : CollectionItemToTs<S["fields"]>[]
        : S extends { kind: "enum"; values: readonly string[] }
          ? Nullable extends true ? S["values"][number] | null : S["values"][number]
          : S extends { kind: "string" | "icon" | "dimension" | "remDimension" }
            ? Nullable extends true ? string | null : string
            : never;

type ItemScalarToTs<S extends ItemScalarTypeDef> = S extends { kind: "boolean" }
  ? boolean
  : S extends { kind: "number" }
    ? number
    : S extends { kind: "collection"; fields: readonly ItemFieldDecl[] }
      ? CollectionItemToTs<S["fields"]>[]
      : string;

type RequiredCollectionFields<F extends readonly ItemFieldDecl[]> = {
  [P in F[number] as P extends { optional: true } ? never : P["name"]]: ItemScalarToTs<P["type"]>;
};

type OptionalCollectionFields<F extends readonly ItemFieldDecl[]> = {
  [P in F[number] as P extends { optional: true } ? P["name"] : never]?: ItemScalarToTs<P["type"]>;
};

export type CollectionItemToTs<F extends readonly ItemFieldDecl[]> =
  RequiredCollectionFields<F> & OptionalCollectionFields<F>;

export type PayloadToTs<P> = {
  [K in keyof P]: P[K] extends "boolean"
    ? boolean
    : P[K] extends "number"
      ? number
      : P[K] extends "numberPair"
        ? [number, number]
        : string;
};

type IsUnion<T, Whole = T> = T extends Whole
  ? [Whole] extends [T]
    ? false
    : true
  : never;

/** Handler arguments for an event payload: no payload → no args; a
 * single-field payload → the field value; more fields → the payload
 * object (one argument, not a union of field values). */
export type PayloadArgs<P> = keyof P extends never
  ? []
  : true extends IsUnion<keyof P>
    ? [PayloadToTs<P>]
    : [PayloadToTs<P>[keyof P]];

type AssertPayloadProjection<T extends true> = T;
type Exact<A, B> =
  (<T>() => T extends A ? 1 : 2) extends (<T>() => T extends B ? 1 : 2)
    ? true
    : false;
type _MultiFieldPayloadStaysOneObjectArgument = AssertPayloadProjection<
  Exact<PayloadArgs<{ value: "string"; committed: "boolean" }>, [{ value: string; committed: boolean }]>
>;

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
  | {
      kind: "action";
      name: string;
      part: string;
      input?: string;
      key?: string;
      fraction?: number;
      phase?: string;
      target?: string;
      text?: string;
      start?: number;
      end?: number;
    }
  | { kind: "expectPart"; part: string; expect: Record<string, unknown> }
  | {
      kind: "expectEvents";
      events: (string | { name: string; payload: Record<string, string | number | boolean | null> })[];
    };

/** A record in a fixture collection: scalar fields plus, at one level of
 * nesting, records of its own. */
export type SerializedCollectionItem = {
  readonly [field: string]:
    | string
    | boolean
    | number
    | readonly SerializedCollectionItem[]
    | undefined;
};

/** Fixture prop values in the neutral JSON (includes structured numbers). */
export type SerializedFixtureProp =
  | string
  | boolean
  | number
  | readonly [number, number]
  | readonly SerializedCollectionItem[]
  | null;

/** The loose, runtime-shaped case in the neutral JSON. */
export interface SerializedCase {
  id: string;
  fixture: {
    props: Record<string, SerializedFixtureProp>;
    regions: Record<string, string>;
    host?: Record<string, unknown>;
  };
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
  /** Opaque host composition data: interpreted only by per-component fixture
   * hosts (e.g. which additional dismissable layer to compose inside the
   * content). Generic runners and observers never read it. */
  host?: Record<string, unknown>;
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

/** The portable dimension subset: a rem length (`14rem`, `20.5rem`). Any
 * other shape is rejected at case authoring and by the Rust codegen, so an
 * unsupported value never silently becomes a default. */
const REM_DIMENSION = /^[0-9]+(\.[0-9]+)?rem$/;

function isPortableRemDimension(value: string): boolean {
  return REM_DIMENSION.test(value)
    && Number.isFinite(Math.fround(Number(value.slice(0, -3))));
}

export type GeometryField =
  | "height"
  | "minWidth"
  | "paddingLeft"
  | "paddingRight"
  | "radius"
  | "borderWidth"
  // Relative logical bounds against the part's `relativeTo` anchor: the gap
  // between the part's edge and the anchor's facing edge (positive when the
  // part is past the anchor in that direction), the start/end alignment
  // deltas, and the width difference. Axis-agnostic names; the case authors
  // the family that applies to its placement.
  | "topGap"
  | "bottomGap"
  | "leftGap"
  | "rightGap"
  | "hStart"
  | "hEnd"
  | "vStart"
  | "widthGap"
  // A bounded scroll region's own height cap (g14.007). Paired with
  // `scrollable`, this is what makes "the list scrolls instead of growing"
  // an observation rather than a screenshot.
  | "maxHeight";

export type GeometryExpectation = Partial<Record<GeometryField, number>> & {
  /** Explicit assertion-local bound. Blanket runtime tolerances are forbidden. */
  tolerance: number;
};

export interface PartExpectation<I extends InterfaceConfig> {
  role?: string;
  name?: string;
  text?: string;
  /** The icon name the part carries (observed on all runtimes). */
  icon?: string;
  /** Observed value: a scalar thumb, a controlled pair, or editable text. */
  value?: number | readonly [number, number] | string;
  /** Caret/selection start, in UTF-16 code units. Runtime adapters translate
   * when their editor stores another index unit. */
  selectionStart?: number;
  /** Caret/selection end, in UTF-16 code units. Equal to start is a caret. */
  selectionEnd?: number;
  present?: boolean;
  focusable?: boolean;
  focused?: boolean;
  focusVisible?: boolean;
  tabbable?: boolean;
  selected?: boolean;
  /** ARIA expanded projection (overlay triggers). */
  expanded?: boolean;
  orientation?: "horizontal" | "vertical";
  controls?: string;
  labelledBy?: string;
  /** The part that hosts this part's layer: the nearest part ancestor in the
   * runtime tree, or the component root that hosts the layer when the part
   * renders outside the tree (portalled / overlay surfaces). */
  parent?: string;
  /** Overlay evidence: the part renders above normal content. */
  overlay?: boolean;
  /** Root-only: the text of the currently focused element/node. */
  focusedText?: string;
  /** Root-only: the number of open overlay layers at/under this component. */
  layerCount?: number;
  /** Hierarchy level (1-based). Depth an assistive technology can hear, not
   * indentation only a sighted reader can infer. */
  level?: number;
  /** The part is a bounded scroll region rather than an unbounded column. */
  scrollable?: boolean;
  states?: Partial<Record<StateNamesOf<I>, boolean>>;
  tokenRoles?: Partial<Record<TokenRoleNamesOf<I>, string>>;
  geometry?: GeometryExpectation;
}

export type CasePartId<I extends InterfaceConfig> = I["parts"][number] extends infer P
  ? P extends { id: infer Id extends string; repeat: object }
    ? `${Id}:${string}`
    : P extends { id: infer Id extends string }
      ? Id
      : never
  : never;

export type CaseStep<I extends InterfaceConfig> =
  | { kind: "action"; name: "press" | "focus"; part: CasePartId<I>; input?: "pointer" | "keyboard" }
  | { kind: "action"; name: "key"; part: CasePartId<I>; key: string }
  | {
      kind: "action";
      name: "scrub";
      part: CasePartId<I>;
      fraction: number;
      phase: "press" | "drag" | "release";
    }
  | { kind: "action"; name: "dismiss"; part: CasePartId<I> }
  | { kind: "action"; name: "pointer"; part: CasePartId<I>; target: "inside" | "outside" }
  | { kind: "action"; name: "insert"; part: CasePartId<I>; text: string }
  | { kind: "action"; name: "select"; part: CasePartId<I>; start: number; end: number }
  | {
      kind: "action";
      name: "compose";
      part: CasePartId<I>;
      text: string;
      phase: "start" | "update" | "commit";
    }
  | { kind: "expectPart"; part: CasePartId<I>; expect: PartExpectation<I> }
  | { kind: "expectEvents"; events: EventExpectation<I>[] };

/** An expected command in the trace: the event name alone, or the name plus
 * the payload fields that must have travelled with it (g14.007). A composite's
 * commands are only meaningful with their payload — "navigate fired" is not
 * the claim; "navigate fired for this branch and this entry" is. */
export type EventExpectation<I extends InterfaceConfig> =
  | EventNamesOf<I>
  | { name: EventNamesOf<I>; payload: Record<string, string | number | boolean | null> };

export function eventExpectationName<I extends InterfaceConfig>(
  expectation: EventExpectation<I>,
): string {
  return typeof expectation === "string" ? expectation : expectation.name;
}

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

/** Fixture collection values, one record at a time and one nesting level
 * deep. Unknown fields are errors here exactly as they are at the top level:
 * a typo in a nested record would otherwise render as an absent row. */
function validateCollectionValue(
  at: string,
  fields: readonly ItemFieldDecl[],
  value: unknown,
): void {
  if (!Array.isArray(value)) {
    throw new Error(`${at} must be a collection`);
  }
  for (const [index, item] of value.entries()) {
    if (typeof item !== "object" || item === null || Array.isArray(item)) {
      throw new Error(`${at} item ${index} must be an object`);
    }
    const itemRecord = item as Record<string, unknown>;
    for (const field of fields) {
      const fieldValue = itemRecord[field.name];
      if (fieldValue === undefined && field.optional) continue;
      if (field.type.kind === "collection") {
        validateCollectionValue(
          `${at} item ${index} field '${field.name}'`,
          field.type.fields,
          fieldValue,
        );
        continue;
      }
      const expectedType =
        field.type.kind === "number"
          ? "number"
          : field.type.kind === "boolean"
            ? "boolean"
            : "string";
      if (typeof fieldValue !== expectedType) {
        throw new Error(`${at} item ${index} field '${field.name}' must be ${expectedType}`);
      }
    }
    for (const fieldName of Object.keys(itemRecord)) {
      if (!fields.some((field) => field.name === fieldName)) {
        throw new Error(`${at} item ${index} uses unknown field '${fieldName}'`);
      }
    }
  }
}

/** The records a repeated part keys over: the collection prop's items, or the
 * items of its named nested collection field. */
function repeatedRecords(
  collection: unknown,
  path: string | undefined,
): Record<string, unknown>[] {
  if (!Array.isArray(collection)) return [];
  const items = collection.filter(
    (item): item is Record<string, unknown> =>
      typeof item === "object" && item !== null && !Array.isArray(item),
  );
  if (!path) return items;
  return items.flatMap((item) => {
    const nested = item[path];
    return Array.isArray(nested)
      ? nested.filter(
          (entry): entry is Record<string, unknown> =>
            typeof entry === "object" && entry !== null && !Array.isArray(entry),
        )
      : [];
  });
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
    if (prop.type.kind === "number" && value !== null && typeof value !== "number") {
      throw new Error(`case '${config.id}' prop '${key}' must be a number`);
    }
    if (prop.type.kind === "remDimension" && value !== null) {
      if (typeof value !== "string" || !isPortableRemDimension(value)) {
        throw new Error(
          `case '${config.id}' prop '${key}' value '${String(value)}' is not a portable rem length (the §12 subset)`,
        );
      }
    }
    if (prop.type.kind === "numberPair" && value !== null) {
      if (!Array.isArray(value) || value.length !== 2 || value.some((entry) => typeof entry !== "number")) {
        throw new Error(`case '${config.id}' prop '${key}' must be a [number, number] pair`);
      }
    }
    if (prop.type.kind === "collection" && value !== null) {
      validateCollectionValue(`case '${config.id}' prop '${key}'`, prop.type.fields, value);
    }
  }
  void propNames;
  const regionNames = new Set(iface.regions.map((r) => r.name));
  for (const key of Object.keys(config.fixture.regions)) {
    if (!regionNames.has(key)) {
      throw new Error(`case '${config.id}' uses unknown region '${key}'`);
    }
  }
  for (const record of iface.hostRecords ?? []) {
    const supplied = (config.fixture.host ?? {})[record.name];
    if (supplied === undefined) continue;
    validateCollectionValue(
      `case '${config.id}' host record '${record.name}'`,
      record.fields,
      supplied,
    );
  }
  const partIds = new Set(iface.parts.filter((p) => !p.repeat).map((p) => p.id));
  const repeatedPartKeys = new Map<string, Set<string>>();
  for (const part of iface.parts) {
    if (!part.repeat) continue;
    const keys = new Set<string>();
    for (const source of part.repeat.sources) {
      const collection =
        "prop" in source
          ? (config.fixture.props as Record<string, unknown>)[source.prop]
          : (config.fixture.host ?? {})[source.hostRecord];
      for (const [index, item] of repeatedRecords(collection, source.path).entries()) {
        const key = item[part.repeat.key];
        if (typeof key !== "string" || key.length === 0) {
          throw new Error(
            `case '${config.id}' ${sourceLabel(source)} item ${index} key '${part.repeat.key}' must be a non-empty string`,
          );
        }
        if (keys.has(key)) {
          // One key space across the sources: a fork run entry that repeated a
          // spine entry id would render the same entry twice.
          throw new Error(
            `case '${config.id}' part '${part.id}' has duplicate key '${key}'`,
          );
        }
        keys.add(key);
      }
    }
    repeatedPartKeys.set(part.id, keys);
  }
  const partIsKnown = (part: string): boolean => {
    if (partIds.has(part)) return true;
    const separator = part.indexOf(":");
    if (separator <= 0 || separator >= part.length - 1) return false;
    return repeatedPartKeys.get(part.slice(0, separator))?.has(part.slice(separator + 1)) ?? false;
  };
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
        if (!partIsKnown(step.part)) {
          throw new Error(`case '${config.id}' action targets unknown part '${step.part}'`);
        }
        if (step.name === "key" && (!("key" in step) || !step.key)) {
          throw new Error(`case '${config.id}' key action needs a key`);
        }
        if (step.name === "scrub") {
          if (!("fraction" in step) || !Number.isFinite(step.fraction) || step.fraction < 0 || step.fraction > 1) {
            throw new Error(`case '${config.id}' scrub action needs a fraction in [0, 1]`);
          }
          if (!("phase" in step) || !["press", "drag", "release"].includes(step.phase)) {
            throw new Error(`case '${config.id}' scrub action needs phase press|drag|release`);
          }
        }
        if (step.name === "pointer") {
          if (!("target" in step) || !["inside", "outside"].includes(step.target)) {
            throw new Error(`case '${config.id}' pointer action needs target inside|outside`);
          }
        }
        break;
      }
      case "expectPart": {
        if (!partIsKnown(step.part)) {
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
        if (step.expect.geometry) {
          const { tolerance, ...fields } = step.expect.geometry;
          if (!Number.isFinite(tolerance) || tolerance < 0) {
            throw new Error(`case '${config.id}' geometry tolerance must be a finite non-negative number`);
          }
          if (Object.keys(fields).length === 0) {
            throw new Error(`case '${config.id}' geometry expectation needs at least one field`);
          }
          for (const [field, value] of Object.entries(fields)) {
            if (!GEOMETRY_FIELDS.has(field as GeometryField)) {
              throw new Error(`case '${config.id}' expects unknown geometry field '${field}'`);
            }
            if (!Number.isFinite(value)) {
              throw new Error(`case '${config.id}' geometry '${field}' must be a finite number`);
            }
          }
        }
        break;
      }
      case "expectEvents": {
        for (const expectation of step.events) {
          const event = eventExpectationName(expectation);
          if (!eventNames.has(event)) {
            throw new Error(`case '${config.id}' expects unknown event '${event}'`);
          }
          if (typeof expectation === "string") continue;
          const decl = iface.events.find((candidate) => candidate.name === event);
          const payloadFields = new Set(Object.keys((decl?.payload ?? {}) as object));
          for (const field of Object.keys(expectation.payload)) {
            if (!payloadFields.has(field)) {
              throw new Error(
                `case '${config.id}' expects unknown payload field '${event}.${field}'`,
              );
            }
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

const GEOMETRY_FIELDS = new Set<GeometryField>([
  "height",
  "minWidth",
  "paddingLeft",
  "paddingRight",
  "radius",
  "borderWidth",
  "topGap",
  "bottomGap",
  "leftGap",
  "rightGap",
  "hStart",
  "hEnd",
  "vStart",
  "widthGap",
  "maxHeight",
]);

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
    // Absent for interfaces that declare none, so existing serialized
    // fixtures stay byte-identical.
    ...(iface.hostRecords ? { hostRecords: [...iface.hostRecords] } : {}),
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
        props: { ...c.fixture.props } as Record<string, SerializedFixtureProp>,
        regions: { ...c.fixture.regions } as Record<string, string>,
        ...(c.fixture.host ? { host: c.fixture.host } : {}),
      },
      specimen: {
        group: c.specimen.group,
        caption: c.specimen.caption,
        axes: [...c.specimen.axes] as string[],
        captureId: c.specimen.captureId,
      },
      steps: c.steps.map((step) => {
        if (step.kind === "action") {
          if (step.name === "key") {
            return { kind: "action", name: "key", part: step.part, key: step.key };
          }
          if (step.name === "scrub") {
            return {
              kind: "action",
              name: "scrub",
              part: step.part,
              fraction: step.fraction,
              phase: step.phase,
            };
          }
          if (step.name === "pointer") {
            return {
              kind: "action",
              name: "pointer",
              part: step.part,
              target: step.target,
            };
          }
          if (step.name === "dismiss") {
            return { kind: "action", name: "dismiss", part: step.part };
          }
          if (step.name === "insert") {
            return { kind: "action", name: "insert", part: step.part, text: step.text };
          }
          if (step.name === "select") {
            return {
              kind: "action",
              name: "select",
              part: step.part,
              start: step.start,
              end: step.end,
            };
          }
          if (step.name === "compose") {
            return {
              kind: "action",
              name: "compose",
              part: step.part,
              text: step.text,
              phase: step.phase,
            };
          }
          return {
            kind: "action",
            name: step.name,
            part: step.part,
            ...(step.input ? { input: step.input } : {}),
          };
        }
        if (step.kind === "expectPart") {
          return { kind: "expectPart", part: step.part, expect: { ...step.expect } as Record<string, unknown> };
        }
        return {
          kind: "expectEvents",
          events: step.events.map((expectation) =>
            typeof expectation === "string"
              ? expectation
              : { name: expectation.name, payload: { ...expectation.payload } },
          ),
        };
      }),
    })),
  };
}

export function actionPress<I extends InterfaceConfig>(
  part: CasePartId<I>,
  input: "pointer" | "keyboard" = "pointer",
): CaseStep<I> {
  return { kind: "action", name: "press", part, input };
}

export function actionFocus<I extends InterfaceConfig>(part: CasePartId<I>): CaseStep<I> {
  return { kind: "action", name: "focus", part };
}

export function actionKey<I extends InterfaceConfig>(part: CasePartId<I>, key: string): CaseStep<I> {
  return { kind: "action", name: "key", part, key };
}

export function actionScrub<I extends InterfaceConfig>(
  part: CasePartId<I>,
  fraction: number,
  phase: "press" | "drag" | "release" = "press",
): CaseStep<I> {
  return { kind: "action", name: "scrub", part, fraction, phase };
}

/** The harness's real dismissal route (document-level Escape on web, the
 * window dispatch tree on native). Never a direct callback invocation. */
export function actionDismiss<I extends InterfaceConfig>(part: CasePartId<I>): CaseStep<I> {
  return { kind: "action", name: "dismiss", part };
}

/** A real pointer interaction with inside/outside intent relative to a part:
 * inside = a press on the resolved part (its own element or subtree), outside
 * = a press outside the mounted component. */
export function actionPointer<I extends InterfaceConfig>(
  part: CasePartId<I>,
  target: "inside" | "outside",
): CaseStep<I> {
  return { kind: "action", name: "pointer", part, target };
}

/** Insert text through the runtime's real editing path (DOM input or native
 * key/IME dispatch). Never a direct host state write. */
export function actionInsert<I extends InterfaceConfig>(
  part: CasePartId<I>,
  text: string,
): CaseStep<I> {
  return { kind: "action", name: "insert", part, text };
}

/** Set the caret or selection through the runtime's real selection path.
 * Offsets use UTF-16 code units, matching DOM and platform text APIs. */
export function actionSelect<I extends InterfaceConfig>(
  part: CasePartId<I>,
  start: number,
  end: number,
): CaseStep<I> {
  return { kind: "action", name: "select", part, start, end };
}

/** Drive composition/IME: start and update must not commit; commit is the
 * single valueChange. Mechanism is runtime-owned; the committed value is not. */
export function actionCompose<I extends InterfaceConfig>(
  part: CasePartId<I>,
  text: string,
  phase: "start" | "update" | "commit",
): CaseStep<I> {
  return { kind: "action", name: "compose", part, text, phase };
}

export function expectPart<I extends InterfaceConfig>(
  part: CasePartId<I>,
  expect: PartExpectation<I>,
): CaseStep<I> {
  return { kind: "expectPart", part, expect };
}

export function expectEvents<I extends InterfaceConfig>(
  events: EventExpectation<I>[],
): CaseStep<I> {
  return { kind: "expectEvents", events };
}
