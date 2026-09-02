# 013 Icon Geometry Substrate

Status: active internal foundation
Recorded: 2026-09-02
Owner: Poodle core
Depends on: [Semantic motion policy](012-semantic-motion-policy.md),
[Icon contract](../contracts/components/icon.md),
[IconProvider contract](../contracts/components/icon-provider.md)

## Decision

Poodle's first icon-geometry capability is an internal, renderer-neutral
normalization and registry layer. It is a generated projection of the existing
Poodle-owned Lucide manifest. It does not change `Icon`, `IconProvider`, the
named `NodeKind::Icon`, package exports, or public runtime behavior.

The layer has two parts:

- a pure normalizer that turns a bounded 24×24 stroke icon into canonical line
  segments and fixed sampled contours; and
- a curated pair registry that records normalized endpoints as candidate or
  rejected evidence. The schema retains an accepted state for a later visual
  gate, but g16.049 emits no accepted or runtime-eligible pair.

The registry is evidence and future internal input. A deterministic geometry
plan is not a visual-quality approval and does not admit a public `IconMorph`.
Architecture 012 remains the only motion-policy and lifecycle authority.

## Fixed format

The format version is `1`. The normalizer version is `1.0.0`.

| Rule | Value |
| --- | --- |
| Coordinate space | `0 0 24 24` only |
| Quantization | four decimal places, stored as signed integer units of `1/10,000` |
| Canonical segments | line segments with explicit start, end, and closing flag |
| Contours | ordered; each has an explicit `closed` flag; no duplicated closing point |
| Flight samples | 64 arc-length samples per contour; a closed contour does not repeat its first sample |
| Limits | 8 contours and 512 samples per endpoint |
| Paint | `fill=none`, `stroke=currentColor`, `stroke-width=2`, round cap, round join |

Numeric attributes are trimmed and must match the full SVG number grammar:
`[+-]?(digits with optional fraction, or leading fraction)([eE][+-]?digits)?`.
Hexadecimal, binary, suffixed, non-finite, and partial numeric strings are
rejected in both implementations.

Canonical endpoint geometry remains separate from sampled flight geometry. A
sampled frame is an approximation for interpolation; progress `0` and `1`
resolve to the exact canonical endpoint representation.

The normalizer lowers `path`, `line`, `polyline`, `polygon`, `circle`,
`ellipse`, and non-rounded `rect` elements. Paths are limited to `M/m`,
`L/l`, `H/h`, `V/v`, and `Z/z`. It rejects groups, transforms, masks, clips,
filters, gradients, unsupported elements or commands, malformed numbers,
rounded rectangles, non-canonical view boxes, incompatible paint, degenerate
contours, and limit violations. Rejection is typed and fail-closed; it never
returns an empty substitute geometry.

## Pair planning

Pair identity canonicalizes aliases before identity, source digests, and
generation. Self-pairs and reversed duplicates are invalid. A pair must have
the same contour count and the same number of open and closed contours.
Assignment edges exist only when the two assigned contours have the same
`closed` flag, so endpoint source order may differ but a plan can never change
closure during flight. The first registry never splits, merges, duplicates, or
invents contours.

One deterministic plan serves both directions. Contours are matched using
length, centroid, bounds, and sampled shape cost. Equal-count assignments are
searched exhaustively within the eight-contour limit; stable source/index order
breaks ties. Each matched contour evaluates forward/reverse traversal and,
for closed contours, every cyclic start offset without duplicating the first
sample. Correspondence may reorder, reverse, or offset samples. It never
rewrites endpoint coordinates or applies an unrecorded rotation, scale, or
reflection.

The shared vector corpus stores exact endpoint and plan oracles. Each positive
endpoint oracle is a stable FNV-1a 64-bit digest over the canonical ASCII wire fields,
including canonical segments, sampled points, topology, and element order. A
positive pair oracle also records every contour mapping, traversal, cyclic
offset, per-contour cost, total `costMicros`, and a digest over the paired wire.
TypeScript and Rust compute these values independently from the same JSON
corpus. The reverse-plan oracle swaps endpoints, checks exact canonical
endpoints, and compares every interior sample after only the recorded
traversal/cyclic-start reindexing.

Numeric cost is diagnostic only. A candidate remains a candidate until a later
human visual review explicitly gates it; g16.049 stores no reviewer identity or
acceptance authority. Visible twisting, self-crossing, collapse, unintended
global motion, contour duplication, or a semantic state change remains a
rejection even when the numeric plan is deterministic.

## Registry and provenance

`packages/core/src/icons/morph-pairs.json` is the authored pair surface. It
contains 8–12 entries from the current default manifest, with explicit
`candidate` or `rejected` status and a quality-state record; `accepted` is
reserved for a later visual-review gate. No g16.049 entry is runtime-eligible.
Candidate geometry is generated evidence and may be used by g16.050 only as an
internal test fixture. Rejected entries cannot become silent fallbacks.

The generator extends `scripts/build-default-icons.ts` and emits paired
internal projections:

- `packages/core/src/icons/morph-pairs.generated.ts`; and
- `packages/contracts/components/src/icon_geometry.generated.rs`.

Both projections use the same source, schema, normalizer, canonical endpoint
names, topology, correspondence, and derived digests. Each normalized record
contains:

- the exact `lucide-static` source version;
- canonical endpoint names;
- source-node and generated-asset SHA-256 digests;
- normalizer and pair-schema versions;
- contour, closure, primitive, and sample topology;
- correspondence cost and quality state, without reviewer identity;
- a derived geometry digest; and
- the existing Lucide/Feather notice identity.

Every non-rejected manifest entry must normalize both endpoints and emit a
plan. Rejected entries retain diagnostics and no geometry payload.

The audit fails on version, source-byte, alias, topology, schema, orphan,
duplicate, reversed-duplicate, invalid-quality-state,
accepted-before-visual-gate, stale-output, or payload drift. Regeneration is
byte-stable. Each pair's normalized payload stays at or below 16 KiB,
excluding notice text.

## Ownership and boundary

- `poodle-core` owns the pure TypeScript normalizer and authored manifest.
- `poodle-specs` owns the paired pure Rust normalizer and generated internal
  Rust projection. The module is not re-exported as a public component API.
- `scripts/build-default-icons.ts` owns deterministic source validation and
  paired projection generation.
- `packages/core/src/icons/geometry-vectors.json` is the shared golden corpus
  consumed independently by TypeScript and Rust tests, including strict
  numeric, closure-assignment, exact-wire, cost, and reverse-flight oracles.
- Existing icon assets and their `LICENSE.txt` notice remain the single source
  lineage. No second catalogue or Morphicons source is copied or vendored.
- A future resolved geometry node, GPUI path paint, web shell, lifecycle
  helper, native visual admission, and public contract require later cards.

The geometry layer must not import Svelte, React, DOM, GPUI, a host clock, or
the public Icon registry. It must not put path data in `NodeAnimation` or make
GPUI perform pair lookup.

## Evidence boundary

The vector suite proves format laws, exact endpoints, quantization, supported
commands, full-string numeric rejection, unsupported input rejection, topology
rejection, closure-preserving assignment, exact paired wire/cost parity,
reverse direction, closed-loop offsets, and deterministic correspondence in
both languages.
Generator tests prove manifest state handling, provenance, paired output,
payload limits, and drift failures. These checks establish an internal
geometry foundation only. They do not claim visual quality, browser behavior,
native pixels, frame pacing, accessibility, or public component parity.
