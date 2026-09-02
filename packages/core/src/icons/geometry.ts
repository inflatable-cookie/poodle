import type { IconNodeElement } from "./types";

export const ICON_GEOMETRY_SCHEMA_VERSION = 1 as const;
export const ICON_GEOMETRY_NORMALIZER_VERSION = "1.0.0" as const;
export const ICON_GEOMETRY_GRID_SIZE = 24 as const;
export const ICON_GEOMETRY_QUANTIZATION_SCALE = 10_000 as const;
export const ICON_GEOMETRY_SAMPLE_COUNT = 64 as const;
export const ICON_GEOMETRY_MAX_CONTOURS = 8 as const;
export const ICON_GEOMETRY_MAX_SAMPLES = 512 as const;

const CIRCLE_SEGMENTS = 32;
const TAU = Math.PI * 2;
const COST_EPSILON = 1e-12;

export type GeometryPoint = readonly [number, number];

export type GeometrySegment = {
  readonly kind: "line";
  readonly start: GeometryPoint;
  readonly end: GeometryPoint;
  readonly closing: boolean;
};

export type CanonicalContour = {
  readonly closed: boolean;
  readonly segments: readonly GeometrySegment[];
};

export type CanonicalGeometry = {
  readonly viewBox: readonly [0, 0, 24, 24];
  readonly contours: readonly CanonicalContour[];
};

export type SampledContour = {
  readonly closed: boolean;
  readonly points: readonly GeometryPoint[];
};

export type SampledGeometry = {
  readonly sampleCount: 64;
  readonly contours: readonly SampledContour[];
};

export type GeometryTopology = {
  readonly contourCount: number;
  readonly closed: readonly boolean[];
  readonly segmentCounts: readonly number[];
  readonly sampleCount: 64;
};

export type NormalizedIconGeometry = {
  readonly schemaVersion: 1;
  readonly normalizerVersion: "1.0.0";
  readonly canonical: CanonicalGeometry;
  readonly sampled: SampledGeometry;
  readonly topology: GeometryTopology;
  readonly elementTypes: readonly string[];
};

export type IconGeometryInput = {
  readonly viewBox: readonly [number, number, number, number];
  readonly fill: string;
  readonly stroke: string;
  readonly strokeWidth: number;
  readonly strokeLinecap: string;
  readonly strokeLinejoin: string;
  readonly nodes: readonly IconNodeElement[];
};

export type GeometryErrorCode =
  | "invalid-view-box"
  | "invalid-paint"
  | "unsupported-element"
  | "unsupported-attribute"
  | "unsupported-transform"
  | "unsupported-path-command"
  | "malformed-path"
  | "invalid-number"
  | "rounded-rect"
  | "empty-contour"
  | "degenerate-contour"
  | "out-of-bounds"
  | "too-many-contours"
  | "too-many-samples"
  | "pair-contour-count"
  | "pair-closure"
  | "pair-planning";

export class IconGeometryError extends Error {
  readonly code: GeometryErrorCode;

  constructor(code: GeometryErrorCode, message: string) {
    super(`${code}: ${message}`);
    this.name = "IconGeometryError";
    this.code = code;
  }
}

export type NormalizeResult =
  | { readonly ok: true; readonly value: NormalizedIconGeometry }
  | { readonly ok: false; readonly error: IconGeometryError };

export type ContourCorrespondence = {
  readonly leftIndex: number;
  readonly rightIndex: number;
  /** The right sample order is traversed in reverse when true. */
  readonly reversed: boolean;
  /** The right sample index paired with left sample zero. */
  readonly offset: number;
  readonly costMicros: number;
};

export type IconGeometryPairPlan = {
  readonly contourMappings: readonly ContourCorrespondence[];
  readonly costMicros: number;
};

export type PlannedIconGeometryPair = {
  readonly left: NormalizedIconGeometry;
  readonly right: NormalizedIconGeometry;
  readonly plan: IconGeometryPairPlan;
};

export type GeometryFrame = {
  readonly contours: readonly SampledContour[];
};

type FloatPoint = [number, number];
type RawContour = { points: FloatPoint[]; closed: boolean };
type PathToken =
  | { kind: "command"; value: string }
  | { kind: "number"; value: number };

const numberPattern =
  /[-+]?(?:(?:\d+\.\d*)|(?:\.\d+)|(?:\d+))(?:[eE][-+]?\d+)?/y;

function fail(code: GeometryErrorCode, message: string): never {
  throw new IconGeometryError(code, message);
}

function readNumber(
  attrs: Record<string, string>,
  key: string,
  fallback?: number,
): number {
  const raw = attrs[key];
  if (raw === undefined && fallback !== undefined) return fallback;
  if (raw === undefined || raw.trim() === "") {
    fail("invalid-number", `missing ${key}`);
  }
  const value = Number(raw);
  if (!Number.isFinite(value)) fail("invalid-number", `${key} is not finite`);
  return value;
}

function validateAttributes(
  tag: string,
  attrs: Record<string, string>,
  allowed: readonly string[],
): void {
  for (const key of Object.keys(attrs)) {
    if (key === "transform") fail("unsupported-transform", `${tag}.${key}`);
    if (!allowed.includes(key)) {
      fail("unsupported-attribute", `${tag}.${key}`);
    }
  }
}

function tokenizePath(value: string): PathToken[] {
  const tokens: PathToken[] = [];
  let index = 0;
  while (index < value.length) {
    while (index < value.length && /[\s,]/.test(value[index]!)) index += 1;
    if (index >= value.length) break;

    const character = value[index]!;
    if (/[A-Za-z]/.test(character)) {
      if (!"MmLlHhVvZz".includes(character)) {
        fail("unsupported-path-command", character);
      }
      tokens.push({ kind: "command", value: character });
      index += 1;
      continue;
    }

    numberPattern.lastIndex = index;
    const match = numberPattern.exec(value);
    if (!match) fail("malformed-path", `unexpected token at ${index}`);
    const numberText = match[0]!;
    const number = Number(numberText);
    if (!Number.isFinite(number)) fail("invalid-number", numberText);
    tokens.push({ kind: "number", value: number });
    index = numberPattern.lastIndex;
  }
  return tokens;
}

function parsePath(value: string): RawContour[] {
  const tokens = tokenizePath(value);
  const contours: RawContour[] = [];
  let tokenIndex = 0;
  let command: string | null = null;
  let commandNeedsValues = false;
  let current: RawContour | null = null;
  let point: FloatPoint = [0, 0];
  let start: FloatPoint = [0, 0];

  const hasCurrent = (): RawContour => {
    if (!current) fail("malformed-path", "drawing command has no moveto");
    return current;
  };

  const addPoint = (next: FloatPoint): void => {
    const contour = hasCurrent();
    if (contour.closed) {
      fail("malformed-path", "drawing after closepath requires moveto");
    }
    contour.points.push(next);
    point = next;
  };

  const pushCurrent = (): void => {
    if (!current) return;
    if (current.points.length < 2) {
      fail("empty-contour", "a contour needs at least one segment");
    }
    contours.push(current);
    current = null;
  };

  while (tokenIndex < tokens.length) {
    const token = tokens[tokenIndex];
    if (!token) fail("malformed-path", "path ended unexpectedly");
    if (token.kind === "command") {
      command = token.value;
      tokenIndex += 1;
      commandNeedsValues = true;
      if (command === "Z" || command === "z") {
        const contour = hasCurrent();
        if (contour.points.length < 2) {
          fail("empty-contour", "closepath has no segment");
        }
        contour.closed = true;
        point = [...start] as FloatPoint;
        command = null;
        commandNeedsValues = false;
      }
      continue;
    }

    if (!command) fail("malformed-path", "numbers require a command");
    const absolute = command === command.toUpperCase();

    const take = (count: number): number[] => {
      const values: number[] = [];
      for (let offset = 0; offset < count; offset += 1) {
        const next = tokens[tokenIndex + offset];
        if (!next || next.kind !== "number") {
          fail("malformed-path", `command ${command} needs ${count} values`);
        }
        values.push(next.value);
      }
      tokenIndex += count;
      commandNeedsValues = false;
      return values;
    };

    if (command === "M" || command === "m") {
      const values = take(2);
      const x = values[0]!;
      const y = values[1]!;
      pushCurrent();
      point = absolute ? [x, y] : [point[0] + x, point[1] + y];
      start = [...point] as FloatPoint;
      current = { points: [point], closed: false };
      // SVG treats additional moveto pairs as implicit lineto pairs.
      command = absolute ? "L" : "l";
      commandNeedsValues = false;
      continue;
    }

    if (command === "L" || command === "l") {
      const values = take(2);
      const x = values[0]!;
      const y = values[1]!;
      addPoint(absolute ? [x, y] : [point[0] + x, point[1] + y]);
      continue;
    }

    if (command === "H" || command === "h") {
      const x = take(1)[0]!;
      addPoint(absolute ? [x, point[1]] : [point[0] + x, point[1]]);
      continue;
    }

    if (command === "V" || command === "v") {
      const y = take(1)[0]!;
      addPoint(absolute ? [point[0], y] : [point[0], point[1] + y]);
      continue;
    }

    fail("unsupported-path-command", command);
  }

  if (commandNeedsValues) fail("malformed-path", `command ${command} has no values`);
  pushCurrent();
  if (contours.length === 0) fail("empty-contour", "path has no contours");
  return contours;
}

function parsePointList(value: string): FloatPoint[] {
  const values: number[] = [];
  let index = 0;
  while (index < value.length) {
    while (index < value.length && /[\s,]/.test(value[index]!)) index += 1;
    if (index >= value.length) break;
    numberPattern.lastIndex = index;
    const match = numberPattern.exec(value);
    if (!match) fail("malformed-path", `invalid points at ${index}`);
    const numberText = match[0]!;
    const number = Number(numberText);
    if (!Number.isFinite(number)) fail("invalid-number", numberText);
    values.push(number);
    index = numberPattern.lastIndex;
  }
  if (values.length < 4 || values.length % 2 !== 0) {
    fail("malformed-path", "points must contain coordinate pairs");
  }
  const points: FloatPoint[] = [];
  for (let index = 0; index < values.length; index += 2) {
    points.push([values[index]!, values[index + 1]!]);
  }
  return points;
}

function lowerNode([tag, attrs]: IconNodeElement): RawContour[] {
  switch (tag) {
    case "path":
      validateAttributes(tag, attrs, ["d"]);
      if (!attrs.d) fail("malformed-path", "path has no d attribute");
      return parsePath(attrs.d);
    case "line":
      validateAttributes(tag, attrs, ["x1", "x2", "y1", "y2"]);
      return [
        {
          points: [
            [readNumber(attrs, "x1"), readNumber(attrs, "y1")],
            [readNumber(attrs, "x2"), readNumber(attrs, "y2")],
          ],
          closed: false,
        },
      ];
    case "polyline":
      validateAttributes(tag, attrs, ["points"]);
      return [{ points: parsePointList(attrs.points ?? ""), closed: false }];
    case "polygon":
      validateAttributes(tag, attrs, ["points"]);
      return [{ points: parsePointList(attrs.points ?? ""), closed: true }];
    case "rect": {
      validateAttributes(tag, attrs, ["x", "y", "width", "height", "rx", "ry"]);
      const rx = readNumber(attrs, "rx", 0);
      const ry = readNumber(attrs, "ry", 0);
      if (rx !== 0 || ry !== 0) fail("rounded-rect", "rounded rectangles are unsupported");
      const x = readNumber(attrs, "x", 0);
      const y = readNumber(attrs, "y", 0);
      const width = readNumber(attrs, "width");
      const height = readNumber(attrs, "height");
      if (width <= 0 || height <= 0) fail("degenerate-contour", "rect has no area");
      return [
        {
          points: [
            [x, y],
            [x + width, y],
            [x + width, y + height],
            [x, y + height],
          ],
          closed: true,
        },
      ];
    }
    case "circle": {
      validateAttributes(tag, attrs, ["cx", "cy", "r"]);
      const cx = readNumber(attrs, "cx", 0);
      const cy = readNumber(attrs, "cy", 0);
      const radius = readNumber(attrs, "r");
      if (radius <= 0) fail("degenerate-contour", "circle has no radius");
      return [
        {
          points: Array.from({ length: CIRCLE_SEGMENTS }, (_, index) => {
            const angle = (index * TAU) / CIRCLE_SEGMENTS;
            return [cx + radius * Math.cos(angle), cy + radius * Math.sin(angle)];
          }),
          closed: true,
        },
      ];
    }
    case "ellipse": {
      validateAttributes(tag, attrs, ["cx", "cy", "rx", "ry"]);
      const cx = readNumber(attrs, "cx", 0);
      const cy = readNumber(attrs, "cy", 0);
      const rx = readNumber(attrs, "rx");
      const ry = readNumber(attrs, "ry");
      if (rx <= 0 || ry <= 0) fail("degenerate-contour", "ellipse has no area");
      return [
        {
          points: Array.from({ length: CIRCLE_SEGMENTS }, (_, index) => {
            const angle = (index * TAU) / CIRCLE_SEGMENTS;
            return [cx + rx * Math.cos(angle), cy + ry * Math.sin(angle)];
          }),
          closed: true,
        },
      ];
    }
    default:
      fail("unsupported-element", tag);
  }
}

function quantize(value: number): number {
  if (!Number.isFinite(value)) fail("invalid-number", `${value}`);
  return Math.round(value * ICON_GEOMETRY_QUANTIZATION_SCALE);
}

function quantizePoint(point: FloatPoint): GeometryPoint {
  const quantized: GeometryPoint = [quantize(point[0]), quantize(point[1])];
  const max = ICON_GEOMETRY_GRID_SIZE * ICON_GEOMETRY_QUANTIZATION_SCALE;
  if (
    quantized[0] < 0 ||
    quantized[0] > max ||
    quantized[1] < 0 ||
    quantized[1] > max
  ) {
    fail("out-of-bounds", `${point[0]},${point[1]}`);
  }
  return quantized;
}

function samePoint(left: GeometryPoint, right: GeometryPoint): boolean {
  return left[0] === right[0] && left[1] === right[1];
}

function canonicalizeContour(raw: RawContour): CanonicalContour {
  const points = raw.points.map(quantizePoint);
  if (raw.closed && samePoint(points[0]!, points[points.length - 1]!)) points.pop();
  if (points.length < 2) fail("empty-contour", "a contour has fewer than two points");

  const segments: GeometrySegment[] = [];
  for (let index = 0; index < points.length - 1; index += 1) {
    if (samePoint(points[index]!, points[index + 1]!)) {
      fail("degenerate-contour", "a contour contains a zero-length segment");
    }
    segments.push({
      kind: "line",
      start: points[index]!,
      end: points[index + 1]!,
      closing: false,
    });
  }
  if (raw.closed) {
    if (samePoint(points[points.length - 1]!, points[0]!)) {
      fail("degenerate-contour", "a closed contour has no closing segment");
    }
    segments.push({
      kind: "line",
      start: points[points.length - 1]!,
      end: points[0]!,
      closing: true,
    });
  }
  return { closed: raw.closed, segments };
}

function pointFromSegmentStart(contour: CanonicalContour): GeometryPoint {
  const first = contour.segments[0];
  if (!first) fail("empty-contour", "contour has no segments");
  return first.start;
}

function canonicalPoints(contour: CanonicalContour): GeometryPoint[] {
  const points = [pointFromSegmentStart(contour)];
  for (const segment of contour.segments) {
    if (!segment.closing) points.push(segment.end);
  }
  return points;
}

function distance(left: GeometryPoint, right: GeometryPoint): number {
  return Math.hypot(left[0] - right[0], left[1] - right[1]);
}

function sampleContour(contour: CanonicalContour): SampledContour {
  const points = canonicalPoints(contour);
  const edges = contour.segments;
  const lengths = edges.map((edge) => distance(edge.start, edge.end));
  const total = lengths.reduce((sum, length) => sum + length, 0);
  if (total <= 0) fail("degenerate-contour", "contour has no length");

  const sampleCount = ICON_GEOMETRY_SAMPLE_COUNT;
  const samples: GeometryPoint[] = [];
  for (let index = 0; index < sampleCount; index += 1) {
    const target = contour.closed
      ? (index / sampleCount) * total
      : (index / (sampleCount - 1)) * total;
    let cursor = 0;
    let edgeIndex = edges.length - 1;
    for (let candidate = 0; candidate < edges.length; candidate += 1) {
      if (target <= cursor + lengths[candidate]! || candidate === edges.length - 1) {
        edgeIndex = candidate;
        break;
      }
      cursor += lengths[candidate]!;
    }
    const edge = edges[edgeIndex]!;
    const length = lengths[edgeIndex]!;
    const ratio = length === 0 ? 0 : (target - cursor) / length;
    samples.push([
      Math.round(edge.start[0] + (edge.end[0] - edge.start[0]) * ratio),
      Math.round(edge.start[1] + (edge.end[1] - edge.start[1]) * ratio),
    ]);
  }
  return { closed: contour.closed, points: samples };
}

function validateInput(input: IconGeometryInput): void {
  const [x, y, width, height] = input.viewBox;
  if (x !== 0 || y !== 0 || width !== 24 || height !== 24) {
    fail("invalid-view-box", "expected 0 0 24 24");
  }
  if (
    input.fill !== "none" ||
    input.stroke !== "currentColor" ||
    input.strokeWidth !== 2 ||
    input.strokeLinecap !== "round" ||
    input.strokeLinejoin !== "round"
  ) {
    fail("invalid-paint", "expected the canonical Lucide stroke paint");
  }
}

export function normalizeIconGeometry(input: IconGeometryInput): NormalizedIconGeometry {
  validateInput(input);
  const rawContours: RawContour[] = [];
  const elementTypes: string[] = [];
  for (const node of input.nodes) {
    elementTypes.push(node[0]);
    rawContours.push(...lowerNode(node));
  }
  if (rawContours.length === 0) fail("empty-contour", "icon has no geometry");
  if (rawContours.length > ICON_GEOMETRY_MAX_CONTOURS) {
    fail("too-many-contours", `${rawContours.length} contours`);
  }

  const contours = rawContours.map(canonicalizeContour);
  const sampledContours = contours.map(sampleContour);
  const sampleTotal = sampledContours.reduce(
    (sum, contour) => sum + contour.points.length,
    0,
  );
  if (sampleTotal > ICON_GEOMETRY_MAX_SAMPLES) {
    fail("too-many-samples", `${sampleTotal} samples`);
  }

  return {
    schemaVersion: ICON_GEOMETRY_SCHEMA_VERSION,
    normalizerVersion: ICON_GEOMETRY_NORMALIZER_VERSION,
    canonical: {
      viewBox: [0, 0, 24, 24],
      contours,
    },
    sampled: {
      sampleCount: ICON_GEOMETRY_SAMPLE_COUNT,
      contours: sampledContours,
    },
    topology: {
      contourCount: contours.length,
      closed: contours.map((contour) => contour.closed),
      segmentCounts: contours.map((contour) => contour.segments.length),
      sampleCount: ICON_GEOMETRY_SAMPLE_COUNT,
    },
    elementTypes,
  };
}

export function tryNormalizeIconGeometry(input: IconGeometryInput): NormalizeResult {
  try {
    return { ok: true, value: normalizeIconGeometry(input) };
  } catch (error) {
    if (error instanceof IconGeometryError) return { ok: false, error };
    throw error;
  }
}

type ContourMetrics = {
  length: number;
  centroid: GeometryPoint;
  bounds: readonly [number, number, number, number];
};

function contourMetrics(contour: SampledContour): ContourMetrics {
  const points = contour.points;
  let length = 0;
  let x = 0;
  let y = 0;
  let minX = points[0]![0];
  let minY = points[0]![1];
  let maxX = points[0]![0];
  let maxY = points[0]![1];
  for (let index = 0; index < points.length; index += 1) {
    const point = points[index]!;
    x += point[0];
    y += point[1];
    minX = Math.min(minX, point[0]);
    minY = Math.min(minY, point[1]);
    maxX = Math.max(maxX, point[0]);
    maxY = Math.max(maxY, point[1]);
    const next = points[(index + 1) % points.length]!;
    if (index < points.length - 1 || contour.closed) length += distance(point, next);
  }
  return {
    length,
    centroid: [x / points.length, y / points.length],
    bounds: [minX, minY, maxX, maxY],
  };
}

function mod(value: number, divisor: number): number {
  return ((value % divisor) + divisor) % divisor;
}

function orientedIndex(index: number, count: number, reversed: boolean, offset: number): number {
  return reversed ? mod(offset - index, count) : mod(offset + index, count);
}

function orientedPoints(
  contour: SampledContour,
  reversed: boolean,
  offset: number,
): GeometryPoint[] {
  return contour.points.map((_, index) =>
    contour.points[orientedIndex(index, contour.points.length, reversed, offset)]!,
  );
}

function contourCost(left: SampledContour, right: SampledContour, rightPoints: readonly GeometryPoint[]): number {
  const leftMetrics = contourMetrics(left);
  const rightMetrics = contourMetrics({ closed: right.closed, points: rightPoints });
  const grid = ICON_GEOMETRY_GRID_SIZE * ICON_GEOMETRY_QUANTIZATION_SCALE;
  const lengthCost = Math.abs(leftMetrics.length - rightMetrics.length) / grid;
  const centroidCost = distance(leftMetrics.centroid, rightMetrics.centroid) / grid;
  const boundsCost =
    (Math.abs(leftMetrics.bounds[0] - rightMetrics.bounds[0]) +
      Math.abs(leftMetrics.bounds[1] - rightMetrics.bounds[1]) +
      Math.abs(leftMetrics.bounds[2] - rightMetrics.bounds[2]) +
      Math.abs(leftMetrics.bounds[3] - rightMetrics.bounds[3])) /
    (grid * 4);
  let shapeCost = 0;
  for (let index = 0; index < left.points.length; index += 1) {
    shapeCost += distance(left.points[index]!, rightPoints[index]!) ** 2;
  }
  shapeCost /= left.points.length * grid * grid;
  return lengthCost + centroidCost + boundsCost + shapeCost;
}

function betterOption(
  candidate: ContourCorrespondence,
  candidateCost: number,
  current: { correspondence: ContourCorrespondence; cost: number } | null,
): boolean {
  if (!current) return true;
  if (candidateCost < current.cost - COST_EPSILON) return true;
  if (Math.abs(candidateCost - current.cost) > COST_EPSILON) return false;
  if (candidate.reversed !== current.correspondence.reversed) {
    return !candidate.reversed;
  }
  return candidate.offset < current.correspondence.offset;
}

function bestCorrespondence(
  left: SampledContour,
  right: SampledContour,
  leftIndex: number,
  rightIndex: number,
): { correspondence: ContourCorrespondence; cost: number } {
  let best: { correspondence: ContourCorrespondence; cost: number } | null = null;
  const offsets = right.closed
    ? Array.from({ length: right.points.length }, (_, index) => index)
    : [0];
  for (const reversed of [false, true]) {
    for (const offset of offsets) {
      const actualOffset = reversed && !right.closed ? right.points.length - 1 : offset;
      const points = orientedPoints(right, reversed, actualOffset);
      const cost = contourCost(left, right, points);
      const correspondence: ContourCorrespondence = {
        leftIndex,
        rightIndex,
        reversed,
        offset: actualOffset,
        costMicros: Math.round(cost * 1_000_000),
      };
      if (betterOption(correspondence, cost, best)) best = { correspondence, cost };
    }
  }
  if (!best) fail("pair-planning", "no contour correspondence");
  return best;
}

function lexicographicallyBefore(left: readonly number[], right: readonly number[]): boolean {
  for (let index = 0; index < left.length; index += 1) {
    if (left[index] !== right[index]) return left[index]! < right[index]!;
  }
  return false;
}

export function planIconGeometryPair(
  left: NormalizedIconGeometry,
  right: NormalizedIconGeometry,
): IconGeometryPairPlan {
  if (left.sampled.contours.length !== right.sampled.contours.length) {
    fail("pair-contour-count", "endpoints have different contour counts");
  }
  if (left.sampled.contours.some((contour, index) => contour.closed !== right.sampled.contours[index]!.closed)) {
    fail("pair-closure", "endpoints have different closure signatures");
  }

  const count = left.sampled.contours.length;
  const options = left.sampled.contours.map((leftContour, leftIndex) =>
    right.sampled.contours.map((rightContour, rightIndex) =>
      bestCorrespondence(leftContour, rightContour, leftIndex, rightIndex),
    ),
  );
  let best: { assignment: number[]; cost: number; mappings: ContourCorrespondence[] } | null = null;

  const visit = (assignment: number[], used: Set<number>, cost: number): void => {
    const leftIndex = assignment.length;
    if (leftIndex === count) {
      const mappings = assignment.map(
        (rightIndex, index) => options[index]![rightIndex]!.correspondence,
      );
      if (
        !best ||
        cost < best.cost - COST_EPSILON ||
        (Math.abs(cost - best.cost) <= COST_EPSILON &&
          lexicographicallyBefore(assignment, best.assignment))
      ) {
        best = { assignment: [...assignment], cost, mappings };
      }
      return;
    }
    for (let rightIndex = 0; rightIndex < count; rightIndex += 1) {
      if (used.has(rightIndex)) continue;
      used.add(rightIndex);
      assignment.push(rightIndex);
      visit(assignment, used, cost + options[leftIndex]![rightIndex]!.cost);
      assignment.pop();
      used.delete(rightIndex);
    }
  };
  visit([], new Set(), 0);
  if (!best) fail("pair-planning", "no contour assignment");
  const selected = best as {
    assignment: number[];
    cost: number;
    mappings: ContourCorrespondence[];
  };
  return {
    contourMappings: selected.mappings,
    costMicros: Math.round(selected.cost * 1_000_000),
  };
}

export function planIconGeometryPairWithEndpoints(
  left: NormalizedIconGeometry,
  right: NormalizedIconGeometry,
): PlannedIconGeometryPair {
  return { left, right, plan: planIconGeometryPair(left, right) };
}

function canonicalFrame(geometry: NormalizedIconGeometry): GeometryFrame {
  return {
    contours: geometry.canonical.contours.map((contour) => ({
      closed: contour.closed,
      points: canonicalPoints(contour),
    })),
  };
}

export function frameAt(
  pair: PlannedIconGeometryPair,
  progress: number,
): GeometryFrame {
  if (!Number.isFinite(progress)) fail("pair-planning", "progress is not finite");
  if (progress <= 0) return canonicalFrame(pair.left);
  if (progress >= 1) return canonicalFrame(pair.right);

  const rightByLeft = new Map(
    pair.plan.contourMappings.map((mapping) => [mapping.leftIndex, mapping]),
  );
  return {
    contours: pair.left.sampled.contours.map((leftContour, leftIndex) => {
      const mapping = rightByLeft.get(leftIndex);
      if (!mapping) fail("pair-planning", `missing mapping for contour ${leftIndex}`);
      const rightContour = pair.right.sampled.contours[mapping.rightIndex]!;
      const rightPoints = orientedPoints(rightContour, mapping.reversed, mapping.offset);
      return {
        closed: leftContour.closed,
        points: leftContour.points.map((leftPoint, index) => [
          Math.round(leftPoint[0] + (rightPoints[index]![0] - leftPoint[0]) * progress),
          Math.round(leftPoint[1] + (rightPoints[index]![1] - leftPoint[1]) * progress),
        ]),
      };
    }),
  };
}

export function reversePairPlan(plan: IconGeometryPairPlan): IconGeometryPairPlan {
  return {
    contourMappings: [...plan.contourMappings]
      .sort((left, right) => left.rightIndex - right.rightIndex)
      .map((mapping) => ({
        leftIndex: mapping.rightIndex,
        rightIndex: mapping.leftIndex,
        reversed: mapping.reversed,
        offset: mapping.reversed
          ? mapping.offset
          : mod(ICON_GEOMETRY_SAMPLE_COUNT - mapping.offset, ICON_GEOMETRY_SAMPLE_COUNT),
        costMicros: mapping.costMicros,
      })),
    costMicros: plan.costMicros,
  };
}

export function geometryToWire(geometry: NormalizedIconGeometry): object {
  return {
    schemaVersion: geometry.schemaVersion,
    normalizerVersion: geometry.normalizerVersion,
    canonical: {
      contours: geometry.canonical.contours.map((contour) => ({
        closed: contour.closed,
        segments: contour.segments.map((segment) => [
          segment.start[0],
          segment.start[1],
          segment.end[0],
          segment.end[1],
          segment.closing,
        ]),
      })),
    },
    sampled: {
      contours: geometry.sampled.contours.map((contour) => ({
        closed: contour.closed,
        points: contour.points,
      })),
    },
    topology: geometry.topology,
    elementTypes: geometry.elementTypes,
  };
}

export function pairPlanToWire(plan: IconGeometryPairPlan): object {
  return {
    contourMappings: plan.contourMappings,
    costMicros: plan.costMicros,
  };
}
