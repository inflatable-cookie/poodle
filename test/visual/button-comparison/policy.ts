/**
 * g15.047 — the fixed comparison policy. These numbers are the card's
 * authority (`docs/roadmaps/g15/047-primitive-visual-comparison.md`), copied
 * here verbatim; the worker may not widen them. If real antialiasing cannot
 * fit the pixel cap while geometry and roles are correct, the run stops for
 * an operator decision instead of editing this table.
 *
 * Svelte ↔ React is exact: same pinned Chromium, same stylesheet, so every
 * channel must be identical. Svelte ↔ GPUI is renderer-aware: the bounds and
 * tolerances below separate renderer antialiasing from structural drift.
 */

/** Web ↔ GPUI geometry limits, logical pixels. */
export const GEOMETRY = {
  /** Root landmark, each of the four edges. */
  rootEdge: 0.5,
  /** Icon/spinner centre, each axis. */
  contentCentre: 1,
  /** Icon/spinner size, each axis. */
  contentSize: 1,
  /** Content width/height. */
  contentExtent: 2,
} as const;

/** Web ↔ GPUI role limits. */
export const ROLES = {
  /** fill/border/text/focus-ring colour, per 8-bit sRGB channel. */
  colorChannel8Bit: 1,
  /** border/focus-ring width, logical pixels. */
  lineWidth: 0.5,
  /** shadow offset/blur/spread, logical pixels; layer count and inset are exact. */
  shadowGeometry: 0.5,
} as const;

/** Web ↔ GPUI pixel limit. */
export const PIXELS = {
  /** pixelmatch per-pixel threshold. */
  threshold: 0.1,
  /** Antialiasing pixels are identified, not counted as differences. */
  includeAA: false,
  /** At most this fraction of the full viewport may differ. */
  maxDiffRatio: 0.03,
} as const;

export type Channel = "dimensions" | "geometry" | "roles" | "pixels";

export type Finding = {
  channel: Channel;
  /** The landmark or role the finding is about, e.g. "root", "shadow". */
  subject: string;
  detail: string;
};

export type ChannelVerdict = {
  status: "pass" | "fail";
  findings: Finding[];
  /** Always-on channel measurements (e.g. pixel diff counts), pass or fail. */
  metrics?: Record<string, number>;
};

export type PairKind = "svelte-react" | "svelte-gpui";

export type PairVerdict = {
  fixture: string;
  pair: PairKind;
  channels: Record<Channel, ChannelVerdict>;
  /** True only when every channel passed. A pixel pass never lifts geometry or roles. */
  ok: boolean;
};

/**
 * The closed registry of renderer deltas the current Button contract already
 * decides (`docs/contracts/components/button.md` §12 Known Deltas). Shadow
 * and letter-spacing absences apply to every fixture identically. The
 * subpixel-edge snap is the g16.106 exception: it names two leading-slot
 * fixtures only. Every occurrence is reported, never hidden. A finding that
 * does not match one of these exactly stays a failure.
 */
export const KNOWN_RENDERER_DELTAS = [
  {
    id: "gpui-omits-box-shadow",
    citation: "button.md §12: box-shadow omitted in GPUI — GPUI lacks CSS box-shadow support (allowed)",
  },
  {
    id: "gpui-omits-letter-spacing",
    citation: "button.md §12: letter-spacing omitted in GPUI — no letter-spacing API (allowed)",
  },
  {
    id: "gpui-snaps-subpixel-edge",
    citation:
      "button.md §12: GPUI snaps the leading subpixel edge to a whole logical pixel on content-leading-icon and state-loading — poodle-render emits the CSS inset exactly (pad_left 10px, inset 2px at md/default); the 1.0 vs 0.5 logical-px edge is rasterisation (allowed)",
  },
] as const;

/** Fixtures whose 1.0 vs 0.5 logical-px leading edge is the contracted GPUI snap. */
export const SUBPIXEL_EDGE_FIXTURES = [
  "button/content-leading-icon",
  "button/state-loading",
] as const;

export type KnownDeltaId = (typeof KNOWN_RENDERER_DELTAS)[number]["id"];

/**
 * Classify a finding against the known-delta registry. Returns the delta id
 * only when the finding is exactly a contract-approved absence; anything
 * else is `null`.
 *
 * Classification is ANNOTATION ONLY: it attaches the contract citation to the
 * finding in every output. It never excuses the finding — a classified
 * finding still fails its channel and still blocks the run, because the fixed
 * policy says shadow layer count/inset are exact and root-edge tolerance is
 * 0.5 logical px. Changing those exit semantics requires an orchestrator
 * card change, not a runner edit.
 *
 * The shadow delta is recognized structurally: the web receipt carries one or
 * more shadow layers and the GPUI receipt carries none, which is precisely
 * "GPUI paints no shadow". Numeric shadow-geometry differences between two
 * non-empty layer sets are NOT covered and stay failures.
 *
 * The subpixel-edge snap is recognized by fixture id plus a root geometry
 * finding: only `content-leading-icon` and `state-loading`. The same 1.0
 * logical-px root delta on any other fixture stays unclassified.
 */
export function classifyKnownDelta(
  finding: Finding,
  context?: {
    webShadowLayers: number;
    gpuiShadowLayers: number;
    fixture?: string;
  },
): KnownDeltaId | null {
  if (
    finding.channel === "roles" &&
    finding.subject === "shadow" &&
    context !== undefined &&
    context.webShadowLayers > 0 &&
    context.gpuiShadowLayers === 0
  ) {
    return "gpui-omits-box-shadow";
  }
  if (
    finding.channel === "geometry" &&
    finding.subject === "root" &&
    context?.fixture !== undefined &&
    (SUBPIXEL_EDGE_FIXTURES as readonly string[]).includes(context.fixture)
  ) {
    return "gpui-snaps-subpixel-edge";
  }
  return null;
}
