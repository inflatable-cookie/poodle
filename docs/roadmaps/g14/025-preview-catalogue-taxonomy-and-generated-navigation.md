# g14.025 — Preview Catalogue Taxonomy and Generated Navigation

Status: complete — accepted in PR #16
Depends on: current `main`; no conformance verdict dependency
Governing refs: `../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../specs/025-parity-automation-and-harness-boundary.md`,
`../../contracts/001-working-rules.md`

## Outcome

Make the growing component catalogue easy to navigate without creating a new
source of cross-runtime drift. Replace the overloaded `tag` field and copied
Rust registries with one renderer-neutral manifest. Generate the catalogue
metadata consumed by TypeScript and Rust, then give the active Svelte, React,
and GPUI previews the same section/family navigation and search behavior.

This is preview infrastructure. It does not change component contracts,
public APIs, specimens, routes, implementation packages, or conformance
meaning. Jetstream receives generated catalogue metadata but remains outside
the active preview and validation cohort.

## Goals

- [x] Give every catalogue entry one broad section, one primary family, one
      anatomy kind, and optional secondary collections.
- [x] Keep related suites together: agent/tools, model connections,
      audio/music, and account/lifecycle.
- [x] Generate TypeScript and Rust catalogue data from one neutral manifest.
- [x] Make incomplete, duplicate, unknown, or stale classification fail a
      checked generator instead of falling into a default bucket.
- [x] Preserve every `#components/<slug>` route and runtime-local specimen
      availability difference.
- [x] Reduce sidebar scanning through collapsible families, counts, active
      family disclosure, and useful search results.

## Fixed Taxonomy

The manifest uses four separate dimensions. Do not collapse them back into a
single `tag`:

- `section`: broad navigation tier
- `family`: exactly one primary sidebar home
- `kind`: anatomy/filter vocabulary; not a second sidebar hierarchy
- `collections`: optional secondary discovery groupings; never duplicate a
  component in the primary sidebar

Section and family order is fixed:

1. **Foundations**
   - Actions & selection
   - Text & value entry
   - Date & time
   - Layout
   - Content & identity
   - Status & progress
2. **Composition**
   - Navigation
   - Overlays & disclosure
   - Forms & validation
   - Data & collections
   - Media
3. **Systems**
   - Application shell
   - Agent & tools
   - Model connections
   - Audio & music
   - Account & lifecycle

Use stable kebab-case IDs for sections, families, kinds, and collections.
Kinds are a closed vocabulary derived from anatomy, initially `control`,
`input`, `layout`, `display`, `overlay`, `navigation`, `data`, `media`,
`feedback`, `form`, and `composite`. A component's family answers “where do I
look for it?”; its kind answers “what shape is it?”.

The following placements are fixed because they motivated this card:

- every `Agent*`, `ToolCall*`, and `ChangedFiles` entry → Agent & tools
- `ModelPicker`, `ModelCatalogueEditor`, and every `ModelConnection*` entry →
  Model connections
- audio controls and displays, including `Knob`, `Fader`, `AudioSwitch`,
  `Keyboard`, `EnvelopeEditor`, `XYPad`, `ModMatrixGrid`, `AudioMeter`,
  `GainReductionMeter`, `WaveformDisplay`, and `ValueReadout` → Audio & music
- every `Licence*` and `Update*` entry → Account & lifecycle
- application chrome, workspace/detail/settings shells, `MessageCenter`, and
  `HistoryCenter` → Application shell

Classify every other entry by the smallest reusable family that describes its
primary use. Do not create `General`, `Miscellaneous`, `Other`, or a fallback
family. Record genuinely ambiguous choices in the implementation log, but
still give each entry one explicit family.

## Execution Plan

### Batch A — Neutral manifest and checked generation

- [x] Rebase first. Inventory every entry present on current `main` before
      migration; the planning baseline contains 174, but concurrent component
      work may add more.
- [x] Add a versioned neutral manifest under
      `packages/codegen/fixtures/preview-catalogue.json`. It owns section and
      family definitions, their order and labels, the kind vocabulary,
      optional collection definitions, and canonical component identity:
      slug, display name, description, section, family, kind, collections.
- [x] Keep package names and specimen availability out of canonical identity.
      Those are runtime overlays derived or declared beside the runtime's
      actual specimen registry.
- [x] Extend the existing codegen package with a focused catalogue target.
      Generate one TypeScript module for both web previews and one Rust module
      shape consumed by both native previews. Generated files carry source and
      do-not-edit headers.
- [x] Replace the hand-authored Svelte registry data and GPUI/Jetstream static
      catalogue copies. React may keep a small explicit runtime overlay, but it
      must consume the generated canonical module rather than mirror labels or
      ordering.
- [x] Remove GPUI's unknown-slug → `Workstation` fallback. Unknown slugs,
      duplicate slugs, invalid relationships, empty families, missing fields,
      or stale generated output fail generation/check mode.
- [x] Add `effigy catalogue:build` and `effigy catalogue:check` selectors.
      `build` writes deterministic output; `check` proves the tree matches the
      manifest without rewriting it.

### Batch B — Complete classification and audit

- [x] Classify every current component. Preserve names, descriptions, slugs,
      package routing, specimen routing, and route identity.
- [x] Add a machine-readable audit test proving every canonical entry appears
      once, every family belongs to its declared section, every kind and
      collection is declared, and every generated runtime entry resolves to
      the same canonical section/family/kind.
- [x] Prove the fixed agent, model, audio, licence/update, and application-shell
      assignments with assertions, not review memory.
- [x] Compare canonical entries with Svelte/React specimen maps and GPUI's
      implemented specimen roots. Missing specimens remain visible runtime
      availability facts; they do not remove or silently reclassify entries.
- [x] Update scripts that import `allComponents` so contract, capability, and
      surface audits keep working from the generated web registry.
- [x] Keep `packages/svelte/preview/src/catalog.ts` documentation-suite and
      adoption-layer metadata separate. This card does not merge its
      `docsSections`/`docsFamilies` vocabulary into component navigation.

### Batch C — Active-preview navigation

- [x] Update Svelte and React together. Share catalogue data and CSS; keep only
      framework lifecycle/state glue separate.
- [x] Render the same three sections and ordered families in Svelte, React,
      and GPUI. Family headings show component counts and can be collapsed.
- [x] On a direct component route, automatically disclose its family and show
      the active item. Navigating between components must not lose active
      context.
- [x] When no search is active, families start collapsed except the active
      family. On the catalogue landing route, no arbitrary family is forced
      open.
- [x] When search is active, replace the hierarchy with one flat result list.
      Match display name, description, family label, kind label, and collection
      labels. Each result shows a compact `Family · Kind` breadcrumb.
- [x] Keep catalogue search in the existing shell search surface. Do not add a
      second input inside the component sidebar.
- [x] Rebuild the landing page around the same section/family authority. It may
      show all family cards expanded because the content region has room; it
      must not revert to the old anatomy tags.
- [x] Compose existing preview-shell and Poodle primitives where they fit.
      Shell-local disclosure state is allowed. Do not widen `SidebarNav`,
      `Collapsible`, or another public component API for preview convenience.
- [x] Preserve keyboard reachability, visible focus, link semantics, scroll
      behavior, theme behavior, and narrow sidebar layout.

### Batch D — Deferred Jetstream metadata and evidence

- [x] Generate Jetstream's catalogue module from the same manifest so entries,
      descriptions, sections, families, and order cannot drift.
- [x] Preserve Jetstream-local `has_specimen` truth. Do not require a sibling
      Jetstream checkout or import Jetstream sources into Poodle.
- [x] Do not rebuild Jetstream's interactive shell in this card. Record that
      navigation consumption is deferred with runtime admission.
- [x] Add focused web and GPUI tests for grouping, collapse/default state,
      direct-route disclosure, search matching/breadcrumbs, counts, and stable
      hrefs.
- [x] Capture matching Svelte/React screenshots at catalogue landing, an
      agent component, a model component, and active search. Use headless
      structural/render tests for GPUI; no GPUI screenshot is required and the
      native preview must not be launched.
- [x] Add one August implementation log containing before/after category
      counts, the full ambiguity register, generated outputs, active-runtime
      evidence, deferred Jetstream status, and residual risk.

## Acceptance Criteria

- [x] One neutral manifest owns every canonical component description and
      classification; no runtime owns a second handwritten taxonomy.
- [x] All entries present after rebase are classified exactly once with no
      fallback, orphaned family, or uncategorized component.
- [x] Agent, model, audio, and account/lifecycle suites each occupy one obvious
      family even though their members have different anatomy kinds.
- [x] Svelte and React render the same hierarchy, counts, active disclosure,
      search order, breadcrumbs, and routes from shared data and CSS.
- [x] GPUI exposes the same hierarchy, counts, active disclosure, search
      fields/order, and routes from generated Rust data.
- [x] Jetstream's generated metadata matches the manifest while its shell and
      runtime validation remain explicitly deferred.
- [x] A planted missing classification, duplicate slug, invalid family, stale
      TS output, or stale Rust output fails `effigy catalogue:check`.
- [x] Adding a new component requires one manifest entry plus a runtime
      availability/specimen implementation; it never requires editing four
      category match tables.
- [x] Existing component routes, specimens, contract audits, and package names
      are unchanged.

## Stop Conditions

- The taxonomy needs public component behavior or API changes to render.
- A generator must infer family from a component name, directory, old tag, or
  runtime implementation instead of reading an explicit manifest assignment.
- Svelte and React require separate taxonomy data, ordering, or CSS.
- GPUI needs component-specific navigation branches rather than generic
  generated metadata.
- Preserving routes or runtime-local availability requires canonical entries
  to encode framework/package-specific truth.
- The work expands into specimen redesign, component recategorization in
  public docs/contracts, documentation-suite taxonomy, conformance, visual
  baseline replacement, Jetstream shell work, or release automation.

Stop with the smallest conflicting case and options. Do not silently widen the
card.

## Writable Scope

- `packages/codegen/fixtures/preview-catalogue.json`
- focused catalogue codegen target, tests, and generated TS/Rust outputs
- Svelte/React preview registry adapters, component sections, catalogue
  landing pages, shared preview CSS, and focused tests
- GPUI preview catalogue registry, sidebar/landing integration, and focused
  tests
- generated Jetstream catalogue registry only; no Jetstream shell/specimen
  implementation
- existing registry-consuming preview audit scripts only as required by the
  generated module migration
- `effigy.toml` or `tasks/effigy.tasks.toml` for the two catalogue selectors
- one August implementation log and append-only `PAPERCUTS.md`

Do not edit public component implementations, CSS recipes, component
contracts, component specs, conformance fixtures/runners, native backends,
Jetstream shell/specimens, release workflows, unrelated roadmap status, or
stable route syntax.

## Validation

Use Effigy. All automated validation is headless:

- focused catalogue schema, generation, overlay, and navigation tests
- `effigy catalogue:check`
- `effigy check:svelte`
- `effigy react:build`
- `effigy check:gpui`
- `effigy test:components`
- `effigy test:parity`
- `effigy docs:check`
- `git diff --check`

Do not run `effigy qa`, `effigy qa:jetstream`, `ci:conformance-windowed`,
`test:native-visual`, `gpui:run`, `jetstream:run`, or any selector that opens
or activates an operating-system window. Do not treat a foreground manual
preview as automated evidence.

## Completion Protocol

Open one PR from the dedicated worktree. Do not mark this roadmap complete or
edit dispatch state. The orchestrator reviews the classification audit,
generated outputs, active-preview behavior, screenshots, headless validation,
and deferred Jetstream note; records corrections; merges; then decides whether
curated secondary collections need a later UX card.
