# g16.049 — Icon Geometry Format And Registry Foundation

Status: ready
Type: implementation — internal architecture foundation
Opened: 2026-09-01
Depends on: merged `g16.034`, accepted icon-geometry programme in
`../../triage/20260901-230405-icon-geometry-programme.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/icon.md`,
`../../contracts/components/icon-provider.md`,
`../../architecture/012-semantic-motion-policy.md`

## Goal

Deliver IG-01 and IG-02: one renderer-neutral, deterministic 24×24 stroke-
geometry format and one curated generated pair registry with complete
provenance. Keep existing Icon static and add no public IconMorph.

## Fixed Boundary

- Resolve aliases to the current canonical icon manifest. Accept only the
  generated 24×24, uniform-stroke, round-cap/join subset named by the packet.
- Explicitly reject unsupported elements, transforms, paint semantics,
  malformed data, off-grid view boxes, topology invention, and incompatible
  contour/closure signatures.
- Preserve canonical endpoints separately from sampled flight geometry.
  Quantization and sample counts are versioned; endpoint progress resolves to
  exact canonical output.
- Start with 8–12 candidate pairs, at most eight contours, 64 samples per
  contour, 512 samples per endpoint, and 16 KiB generated pair payload.
  Candidate, accepted, and rejected are distinct states.
- TypeScript and Rust consume the same golden vectors and generated lineage.
  Source version/digest, schema/normalizer version, topology, diagnostics,
  quality review, derived digest, and Lucide/Feather notice identity are
  mandatory.
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
  reversed-duplicate, unreviewed-candidate, and payload drift fail closed.
- Exact endpoints, direction reversal, contour ordering, tie-breaks, closure,
  quantization, and unsupported geometry pass the shared vector corpus.
- Numerical cost never auto-accepts a visibly bad pair. Human quality status is
  explicit in the registry.
- No public component, node variant, provider widening, runtime clock, GPUI
  production route, web shell, visual claim, release, or Jetstream work enters
  the diff.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Grid/paint boundary is exact | 23×24 asset or transformed group | explicit rejection, no implicit fit |
| Topology is not invented | menu ↔ x contour mismatch | rejected fixture; no contour duplication |
| Correspondence is deterministic | reverse pair and rotate closed start | one stable bidirectional plan and exact endpoints |
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

Run focused TypeScript/Rust vectors and generation tests, deterministic
regeneration, `effigy audit:icons`, relevant licence/docs checks,
`effigy ci:web`, `effigy ci:rust`, `effigy docs:check`, one final headless
`effigy qa`, and `git diff --check origin/main...HEAD`. No windowed/native-
visual selector is allowed.

## Stop Conditions

Stop if the subset needs arbitrary provider input, raw public paths, topology
invention, fill/multicolour semantics, implicit fitting, unbounded syntax,
incomplete provenance, non-deterministic paired output, or a public/runtime
surface to prove the foundation.

## Continuation

Accepted merge unlocks `g16.050`. It does not admit a component or visual
capability.
