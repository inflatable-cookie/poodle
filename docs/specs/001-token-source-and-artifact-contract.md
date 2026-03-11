# 001 Token Source And Artifact Contract

Status: draft
Updated: 2026-03-11
Depends on: `../architecture/002-token-system-and-package-layout.md`

## Purpose

Define the first normative contract for where tokens come from and how they are
emitted, including how one semantic theme definition translates across runtime
targets, while keeping future implementation targets open.

## Research Inputs

This spec should stay aligned with:

- `docs/research/translation-memos/tm-token-system.md`
- `docs/research/source-hubs/hub-gpui.md`
- `docs/research/value-tracks/tk-design-token-systems.md`

## Canonical Format

Canonical token source files should use W3C DTCG format.

The schema layer is therefore expected to use DTCG concepts such as:

- `$type`
- `$value`
- `$description`
- alias/reference paths

No runtime package may invent a second canonical source format for the same
token meaning.

## Token Layer Rule

The canonical token model should be structured as:

- `primitives/` for raw scales and foundations
- `semantic/` for role-based aliases consumed by components
- `modes/` for theme, density, and control-size overlays
- `metadata/` for aliases, deprecations, and supporting manifest data

Components should reference semantic roles by default. Raw primitive references
should be exceptional and documented.

## Source Of Truth

Canonical token values must live only in:

- `packages/tokens/schema/primitives/`
- `packages/tokens/schema/semantic/`
- `packages/tokens/schema/modes/`
- `packages/tokens/schema/metadata/`

No other package may introduce canonical token values directly.

Canonical theme definitions must also originate in the schema layer and be
translated outward. No runtime package may become the canonical home of a theme
definition.

## Emission Baseline

The initial emission baseline is:

- Style Dictionary 4.x for DTCG-aware emission
- CSS artifacts for browser consumers
- TypeScript artifacts for browser-side code and tooling
- Rust artifacts for GPUI consumers through a Rust-facing custom format or
  transform

Equivalent tooling is allowed later only if it preserves the same schema
contract and artifact expectations.

## Required Artifact Families

The token system must emit three artifact families:

- CSS artifacts for browser/Svelte consumers
- TypeScript artifacts for typed runtime consumption
- Rust artifacts for GPUI consumption

These artifact families must be capable of expressing the same semantic theme
definition across targets.

## Required Initial Artifact Outputs

### CSS

- `packages/tokens/artifacts/css/pug-tokens.css`
- `packages/tokens/artifacts/css/pug-theme-light.css`
- `packages/tokens/artifacts/css/pug-theme-dark.css`
- `packages/tokens/artifacts/css/pug-theme-loophole-studio.css`
- `packages/tokens/artifacts/css/pug-density-compact.css`
- `packages/tokens/artifacts/css/pug-density-comfortable.css`

### TypeScript

- `packages/tokens/artifacts/ts/index.ts`
- `packages/tokens/artifacts/ts/themes.ts`
- `packages/tokens/artifacts/ts/metadata.ts`

### Rust

- `packages/tokens/artifacts/rust/mod.rs`
- `packages/tokens/artifacts/rust/themes.rs`
- `packages/tokens/artifacts/rust/density.rs`
- `packages/tokens/artifacts/rust/metadata.rs`

## Theme Translation Rule

One semantic theme definition must be able to translate into:

- CSS/browser-facing artifacts for Svelte consumers
- TypeScript-facing metadata/runtime helpers where needed
- Rust/GPUI-facing artifacts for native consumers

Format adaptation is expected. Semantic divergence is not.

The same named theme, such as a Loophole theme, should be representable from
one canonical source without requiring separate hand-authored framework-local
theme definitions.

Theme names and semantic role names should survive emission across browser,
TypeScript, Rust, and future runtimes without changing meaning.

## Required Token Families

The first schema slice must include:

- color
- typography
- spacing
- sizing
- radius
- border
- elevation
- motion
- density
- icon
- overlay
- state

## Required Modes

The first mode slice must include:

- light theme
- dark theme
- compact density
- comfortable density
- small control size
- medium control size
- large control size

## Rust Realization Rule

GPUI-facing consumers should receive generated Rust modules and theme helpers,
not hand-maintained duplicate values.

The initial Rust realization should support:

- semantic token constants or structs
- named theme modules or constructors
- density/control-size helpers where applicable
- metadata sufficient to trace generated values back to schema origin

## Naming Rule

Semantic token naming must survive emission to CSS, TypeScript, and Rust
without changing semantic meaning. Format adaptation is allowed; semantic
renaming is not.

The same rule applies to theme names and semantic theme roles.

## Consumer Rule

- Svelte packages may consume emitted artifacts and runtime helpers.
- GPUI packages may consume emitted Rust artifacts and Rust-side helpers.
- Underlay bridges may map emitted CSS and TypeScript artifacts into
  Underlay-owned naming and runtime systems.
- future implementation packages may consume emitted artifacts that fit their
  runtime, provided they do not fork token or theme meaning

Consumers may adapt theme application to their runtime, but they must not fork
theme meaning or replace the canonical theme source with runtime-local truth.

## Forbidden Patterns

- hand-maintained duplicate token constants in Svelte packages
- hand-maintained duplicate token constants in GPUI packages
- hand-maintained duplicate canonical theme definitions in CSS and Rust
- defining canonical token or theme meaning in a future implementation package
- Underlay bridge code becoming the source of truth for token meaning
- component packages reading raw schema files directly when emitted artifacts
  exist for their target runtime
- framework-local token taxonomies that rename the canonical semantic layer
- separately authored canonical theme definitions for future runtimes such as
  React or another desktop UI kit

## First Implementation Exit

The first token implementation tranche is complete when:

- the schema layer exists
- one build pipeline exists
- CSS, TypeScript, and Rust artifacts are emitted from the same schema source
- at least one named semantic theme is emitted across CSS and Rust targets from
  the same source
- Svelte and GPUI token consumer stubs can read emitted artifacts

## Next Task

Update `g01.002` and `g01.003` so their execution checklists and deliverables
point directly to this spec and the concrete package layout.
