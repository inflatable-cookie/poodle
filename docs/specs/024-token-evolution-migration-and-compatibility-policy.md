# 024 Token Evolution, Migration, And Compatibility Policy

Status: active
Updated: 2026-03-12
Depends on: `001-token-source-and-artifact-contract.md`, `008-parity-evidence-documented-delta-and-downstream-extension-rules.md`, `021-public-package-api-stability-and-parity-debt-baseline.md`

## Purpose

Freeze how canonical Poodle tokens may change over time so downstream consumers,
bridges, and parity work do not have to infer compatibility from naming
coincidence or changelog tone.

## Scope Rule

This policy applies to canonical token meaning across:

- primitive token definitions
- semantic token definitions
- mode and theme overlays
- emitted token artifacts for CSS, TypeScript, and Rust
- alias and deprecation metadata under `packages/tokens/schema/metadata/`

It does not make internal generator structure or schema folder layout a public
compatibility surface.

## Canonical Meaning Rule

Token compatibility is determined by meaning, not by path stability alone.

A token is compatibility-relevant if any of these change:

- the semantic role it is meant to represent
- the relationship between a semantic token and its primitive source
- the mode or theme behavior expected from that token
- the emitted artifact shape visible to downstream runtime packages

An unchanged token path does not imply compatibility if its meaning or emitted
behavior changed.

## Evolution Classes

Every token change must be classified as one of:

- `additive`: new token paths or new overlays with no required downstream move
- `alias`: compatibility-preserving indirection from one path to another
- `deprecation`: a path remains available but is marked for future removal
- `behavioral`: path remains available but its effective meaning or output
  changes in a way downstreams may need to review
- `breaking`: rename, removal, split, merge, narrowing, broadening, or emitted
  shape change that requires downstream migration

If a change plausibly forces downstream review, it must not be described as
purely additive.

## Additive Rule

Additive token work is preferred whenever new capability can be introduced
without changing existing meaning.

Additive changes may include:

- new primitive values
- new semantic roles
- new theme overlays
- new optional artifact metadata

Additive changes should avoid silently reinterpreting existing downstream usage.

## Alias Rule

Aliases are the first compatibility lever for token path evolution.

Canonical alias records live in `packages/tokens/schema/metadata/aliases.json`.
Each alias entry should describe:

- `from`
- `to`
- a note that explains why the alias exists

Aliases must only be used where the source and target meanings are compatible
enough for temporary coexistence. Aliases are not a substitute for documenting
behavioral or breaking changes.

## Deprecation Rule

Deprecations are the second compatibility lever for managed change.

Canonical deprecation records live in
`packages/tokens/schema/metadata/deprecations.json`. Each deprecation entry
must carry:

- `path`
- `status`
- `replacement` when one exists
- a note describing the migration posture

`status: active` means the path is still emitted but must be treated as on the
way out. A deprecated token must not be removed until at least one documented
migration tranche records the downstream impact and the intended replacement or
removal posture.

## Behavioral Change Rule

Behavioral changes require explicit review even when token paths stay intact.

Examples include:

- a semantic token pointing at a meaningfully different primitive ramp
- a light or dark theme role changing from subtle to prominent usage
- a density or control-size token altering interaction or layout expectations

Behavioral changes must be logged as compatibility-relevant and must state which
surfaces should be re-reviewed in Svelte, GPUI, preview/docs, and downstream
bridges where applicable.

## Breaking Change Rule

The following are breaking until documented otherwise:

- token renames without an alias bridge
- token removals
- token splits or merges
- narrowing or broadening a semantic role so prior usage becomes misleading
- emitted artifact key or shape changes that alter runtime package imports
- mode or theme changes that invalidate expected role behavior downstream

Pre-`1.0` status allows breaking changes to happen, but it does not allow them
to happen silently.

## Compatibility Surface Rule

Downstream consumers may rely on:

- documented runtime token packages and their declared exports
- emitted artifact meaning in CSS, TypeScript, and Rust outputs
- alias and deprecation metadata as the canonical migration record

Downstream consumers must not rely on:

- raw schema folder layout
- generator implementation details
- undocumented deep imports into source-of-truth packages

## Migration Record Rule

Any non-trivial token change must leave a traceable migration record in the
tranche that introduced it. The record must state:

- affected token paths
- evolution class
- impacted emitted artifacts
- expected downstream review areas
- alias or deprecation metadata added or updated
- migration guidance and timing posture

If no migration is required, that should be stated explicitly.

## Removal Rule

A token may only be removed after all of the following are true:

- the path was previously deprecated or explicitly classified as breaking
- the removal is called out in roadmap and log material
- a replacement or deletion rationale is documented
- downstream review scope is named explicitly

Poodle should prefer alias-plus-deprecation sequences over abrupt removal wherever
that does not preserve misleading meaning.

## Evidence Rule

Compatibility claims for token evolution should be backed by repo-visible
evidence such as:

- alias metadata updates
- deprecation metadata updates
- emitted artifact diffs
- preview/docs review notes where visual or interaction semantics changed
- downstream bridge notes where mapping posture changed

## Current Baseline

The current repo baseline already includes the metadata hooks this policy relies
on:

- `packages/tokens/schema/metadata/aliases.json`
- `packages/tokens/schema/metadata/deprecations.json`

These files are now normative migration instruments, not bootstrap placeholders.

## Next Task

Use this policy while executing `g03.002`, so parity automation and regression
evidence can classify token changes and downstream review scope consistently.
