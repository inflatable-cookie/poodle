# g16.049 — Icon Geometry Format And Registry Foundation

Status: complete — merged in PR #156 after exact-head repair
Type: implementation — internal architecture foundation
Opened: 2026-09-01
Closed: 2026-09-02
Depends on: merged `g16.034` and the funded icon-geometry programme recorded in
`../../handoffs/20260901-234025-post-triage-canonical-runway.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/icon.md`,
`../../contracts/components/icon-provider.md`,
`../../architecture/012-semantic-motion-policy.md`
Architecture: `../../architecture/013-icon-geometry-substrate.md`
Execution log: `../../logs/2026-09/20260902-g16-049-icon-geometry-format-and-registry-foundation.md`
PR: https://github.com/inflatable-cookie/poodle/pull/156
Merge: `47ab6aa3b1a754706e498b1ce4e30776b6bd6efb`

## Goal

Deliver IG-01 and IG-02: one renderer-neutral, deterministic 24×24 stroke-
geometry format and one curated generated pair registry with complete
provenance. Keep existing Icon static and add no public IconMorph.

## Fixed Boundary

- Resolve aliases to the current canonical icon manifest. Accept only the
  generated 24×24, uniform-stroke, round-cap/join subset named by the packet.
- Explicitly reject unsupported elements, transforms, paint semantics,
  malformed data, non-SVG numeric spellings, off-grid view boxes, topology
  invention, and incompatible contour/closure signatures.
- Preserve canonical endpoints separately from sampled flight geometry.
  Quantization and sample counts are versioned; endpoint progress resolves to
  exact canonical output.
- Start with 8–12 candidate pairs, at most eight contours, 64 samples per
  contour, 512 samples per endpoint, and 16 KiB generated pair payload.
  Candidate, accepted, and rejected are distinct states.
- TypeScript and Rust consume the same golden vectors and generated lineage,
  including exact canonical/sample wire digests, paired mappings, and costs.
  Source version/digest, schema/normalizer version, topology, diagnostics,
  quality state and notes, derived digest, and Lucide/Feather notice identity
  are mandatory; no reviewer identity is asserted by this foundation.
- Every structurally plannable pair remains `candidate` until later human
  visual review. g16.049 has zero runtime-eligible pairs; g16.050 may consume
  candidate geometry only as an internal test fixture and cannot promote it.
- Existing Icon, IconProvider, NodeKind::Icon, generic NodeAnimation, default
  registry behavior, packages, and public exports remain unchanged.

## Ordered Work

1. Promote the renderer-neutral ownership and format into a new architecture
   record. Define segments, contours, canonical/sampled frames, quantization,
   failures, topology, and deterministic correspondence.
2. Implement pure TypeScript/Rust normalization against shared positive and
   negative vectors, including reverse direction and closed-loop offsets.
3. Extend the existing generated icon lineage with the curated pair manifest,
   deterministic web/native projections, provenance, notice, drift checks, and
   payload budgets.
4. Prove clean regeneration and explicit rejection. Record one execution log.

## Acceptance

- Separate runs emit byte-identical TypeScript/Rust registry artifacts and
  digests from the same pinned source and schema.
- Alias, source-version, source-byte, topology, schema, orphan, duplicate,
  reversed-duplicate, invalid-quality-state, accepted-before-visual-gate, and
  payload drift fail closed.
- Exact endpoints, direction reversal, contour ordering, tie-breaks,
  closure-preserving assignment, full-string numeric grammar, quantization,
  exact paired wire/cost parity, and unsupported geometry pass the shared
  vector corpus.
- Numerical cost never changes a candidate into an accepted pair. Human visual
  review is a later gate; this foundation records no accepted or runtime-
  eligible pair.
- No public component, node variant, provider widening, runtime clock, GPUI
  production route, web shell, visual claim, release, or Jetstream work enters
  the diff.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Grid/paint boundary is exact | 23×24 asset or transformed group | explicit rejection, no implicit fit |
| Numeric grammar is exact | `x1="0x10"`, `x1="0b10"`, or `width="4px"` | both parsers reject the full string; decimal/exponent forms remain accepted |
| Topology is not invented | menu ↔ x contour mismatch | rejected fixture; no contour duplication |
| Correspondence is deterministic | reverse pair, rotate closed start, and cross-closure cost trap | one stable closure-safe bidirectional plan, exact endpoints, and paired wire/cost oracle |
| Paired numbers are exact | one Rust sample coordinate changes by one unit | shared canonical/sample and plan digests fail |
| Planning state is honest | structurally impossible candidate | generator rejects any non-rejected pair without a plan |
| Provenance is live | mutate one source asset without pair edit | audit fails on source/digest drift |
| Outputs are reproducible | generate twice in clean environments | identical bytes and derived digests |
| Public Icon remains static | change an Icon name under full policy | no geometry clock, prop, or node change |

## Writable Scope

One new icon-geometry architecture record; pure core and Rust geometry modules;
shared vectors; current icon manifest/generator/audit lineage; generated
internal registry projections; focused tests; this card, one log, and new
papercuts. Do not edit public Icon/IconProvider contracts or exports, node
vocabulary, runtime shells/backends, visual evidence, releases, workflows,
consumers, or Jetstream behavior.

## Validation

Validation completed in the execution log: focused TypeScript/Rust vectors and
generation checks, deterministic regeneration, `effigy audit:icons`, licence
and docs checks, `effigy ci:rust`, `effigy docs:check`, and the required range
diff. No windowed/native-visual selector was run.

## Completion Evidence

- The pure normalizers live in `packages/core/src/icons/geometry.ts` and
  `packages/contracts/components/src/icon_geometry.rs`; neither is re-exported
  through the public icon or component surfaces.
- `packages/core/src/icons/geometry-vectors.json` contains 18 shared positive,
  negative, topology, strict-number, closure-assignment, exact-wire, reverse,
  and closed-start vectors consumed independently by both languages.
- `packages/core/src/icons/morph-pairs.json` contains 12 explicit entries:
  six structurally plannable candidates and six rejected pairs; no pair is
  accepted or runtime-eligible in g16.049. Alias
  `home` resolves to canonical `house` and remains an explicit rejection;
  `menu-to-ellipsis` is explicitly rejected for closure mismatch.
- The generator emits the TypeScript and Rust projections with source-node and
  asset digests, schema/normalizer versions, topology, diagnostics, quality
  state, notes, notice identity, and derived registry digest. The largest
  candidate fixture payload is 14,023 bytes against the 16 KiB limit.
- The static `Icon` contract and public icon barrels are unchanged. The
  adversarial plant/restore record, strict numeric counterexample, exact
  parity oracles, and validation results are in the execution log.

## Stop Conditions

Stop if the subset needs arbitrary provider input, raw public paths, topology
invention, fill/multicolour semantics, implicit fitting, unbounded syntax,
incomplete provenance, non-deterministic paired output, or a public/runtime
surface to prove the foundation.

## Continuation

Once the g16.049 foundation is merged, orchestrator review may continue with
`g16.050`. Candidate geometry remains internal test-fixture input only; this
card admits no pair, component, or visual capability.
