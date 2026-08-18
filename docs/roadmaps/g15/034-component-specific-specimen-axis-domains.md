# g15.034 — Component-Specific Specimen Axis Domains

Status: **complete** — PR #41 merged as `0bc8aa85` on 2026-08-19
Depends on: `g15.019`
Blocks: `g15.020`, `g15.026`, `g15.012`, `g15.013`
Governing refs: `../../contracts/001-working-rules.md` (Catalogue Specimens,
Contract-First Changes, Runtime Parity Authority),
`../../contracts/components/empty-state.md`,
`../../contracts/components/icon.md`, `specimen-plan-outline.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`

## Outcome

Specimen axes describe the component's real public value domain. They no
longer infer five control sizes from any prop named `size`, and they cannot
pass while rendering only a subset of their advertised values.

This card resolves the two gaps returned by `g15.019` and the substrate defect
that hid them:

- EmptyState has one size input, `default | compact`, in every runtime. The
  duplicate native `compact` boolean is removed.
- Icon has five real sizes in every active runtime. Its dead `density` input is
  removed from Svelte, React, Rust, contracts, callers, and specimens.
- Svelte, React, and GPUI specimen shells accept explicit ordered axis values.
  Generated scenes pass their authored domains through instead of receiving a
  global five-size assumption.
- Evidence fails when any declared value has no rendered representative.

The operator approved this as a clean breaking migration. Do not retain
aliases, parallel fields, deprecated twins, or silent endpoint collapsing.

## Measured Starting Point

### EmptyState

- The public web contract and both web components use `size: "default" |
  "compact"`; shared CSS gives both values distinct geometry.
- `EmptyStateSpec` carries both `compact: bool` and `size: EmptyStateSize`.
  `poodle_render::empty_state` reads only `compact`, so `size` is dead.
- Poodle has direct Rust callers of `with_compact(true)` in shared renderers,
  GPUI specimens, generated-scene adapters, and deferred Jetstream compile
  consumers. They are migration sites, not reasons for a compatibility twin.

### Icon

- The contract and web packages expose `xs | sm | md | lg | xl`; shared tokens
  already contain `size.icon.xs` and `size.icon.xl` for Rust and web.
- Native `IconSize` has only `Sm | Md | Lg`. `From<ControlSize>` silently maps
  `Xs` to `Sm` and `Xl` to `Lg`.
- Icon's `density` prop/field is behaviourally dead. Web emits an attribute
  with no Icon density styles; Rust stores the field and the renderer ignores
  it. Icon is a fixed-size leaf, so spacing belongs to its parent.
- A read-only search of other projects under `~/Dev/projects` found no direct
  use of Icon density or the two Rust migration methods. Re-run that search at
  the worker base and record the result; downstream absence does not waive the
  package change classification.

### Specimen substrate

- Svelte and React `SpecimenLayout` default axis renderers to global
  `xs/sm/md/lg/xl` and `compact/default/comfortable` arrays.
- GPUI `SpecimenAxes` is typed around `ControlSize`; filtering unsupported rows
  can still advertise an incomplete domain.
- Generated specimen scenes already carry named `sizeAxis` and `densityAxis`
  values, but the adapters do not consistently make those values authoritative.
- The parity census infers eligibility from prop names and proves only that a
  pane contains some evidence. It does not prove every declared value.
- Known bounded non-standard size domains are EmptyState (`default`, `compact`)
  and Text/Eyebrow (`xs`, `sm`, `md`). Icon becomes the full five-value domain.

## Delivery

### 1. Make the component contracts true

- Remove `EmptyStateSpec.compact` and `with_compact`. Add/use
  `with_size(EmptyStateSize)` and make `poodle_render::empty_state` derive all
  compact geometry from `spec.size`.
- Migrate every direct Poodle caller once, including only the mechanical
  deferred-Jetstream compile consumers required by the public Rust break. Add
  no Jetstream parity work and run no Jetstream selector.
- Expand `IconSize` to `Xs | Sm | Md | Lg | Xl`, resolve every value through
  the existing semantic icon tokens, and shift/clamp semantic roles across all
  five stops. Delete the endpoint-collapsing conversion behaviour.
- Remove Icon `density` from Svelte and React props, Rust `IconSpec`, styles or
  attributes, specimens, focused tests, and direct callers such as
  HistoryCenter. Do not alter IconButton or any parent component's density.
- Add focused component/render tests for both EmptyState sizes, all five Icon
  sizes, endpoint conversions, semantic-role clamps, and the absence of Icon's
  dead density surface.

### 2. Make specimen domains explicit

- Give Svelte and React `SpecimenLayout` explicit ordered size/density value
  inputs. Preserve standard control-axis defaults for ordinary hand-written
  callers; a provided domain is authoritative and is rendered in order.
- Give GPUI `SpecimenAxes` the equivalent named-value path. Keep the ordinary
  `ControlSize` convenience for standard five-step components, but do not use
  filtering as the representation of a smaller public domain.
- Pass generated scene `sizeAxis` and `densityAxis` values through every
  Svelte, React, and GPUI scene adapter. Do not grow the retired scene model
  into behaviour, callbacks, or a render tree.
- Author EmptyState's generated scene axis as `default`, `compact` and render
  both real values in all three active previews.
- Make Text and Eyebrow render only `xs`, `sm`, `md` in all three previews.
  Remove blank, filtered, or invented `lg`/`xl` evidence.
- Keep Icon's size pane at all five values and remove its density pane.

### 3. Close the evidence hole

- Harden the paired-web specimen-axis census so every advertised ordered value
  must produce visible, value-identifiable evidence in Svelte and React.
- Add focused exact-domain assertions for EmptyState, Icon, Text, and Eyebrow.
- Add the narrowest headless GPUI structural evidence for the same four domains.
  Do not build the live page-construction probe owned by `g15.026`.
- Regenerate checked-in scene artifacts through the repository generator; do
  not hand-edit generated output.

### 4. Record the breaking release surface

The batch log must name:

- packages changed: `@inflatable-cookie/poodle-svelte`,
  `@inflatable-cookie/poodle-react`, `poodle-specs`, and direct
  `poodle-render`/preview consumers
- public-entry-point impact: Icon density removal; EmptyState Rust shape and
  builder migration; IconSize enum expansion and IconSpec density removal
- change class: `breaking` on the pre-1.0 preview channel
- migration: remove Icon density at call sites; replace native
  `.with_compact(true)` with `.with_size(EmptyStateSize::Compact)`; handle the
  two added IconSize variants in exhaustive matches
- downstream re-check: repeat the targeted `~/Dev/projects` search and run the
  packed web root-import/mount proof plus Rust compile/test gates

Do not add a deprecation alias or compatibility bridge. The operator chose a
clean pre-v1.0 break under the repository hard rule.

## Acceptance

- [x] EmptyState has exactly one two-value size source in each active runtime;
      changing it produces the contracted geometry.
- [x] Icon exposes and renders five distinct token-backed sizes in each active
      runtime; neither endpoint collapses.
- [x] Icon has no density public input, stored field, emitted attribute,
      specimen pane, or direct caller forwarding it.
- [x] EmptyState panes contain exactly `default`, `compact`; Text and Eyebrow
      exactly `xs`, `sm`, `md`; Icon exactly `xs`, `sm`, `md`, `lg`, `xl`.
- [x] Generated scene adapters consume authored ordered domains; ordinary
      standard-axis specimens remain unchanged.
- [x] Evidence fails if one declared axis value is omitted, blank, collapsed
      into another value, or silently filtered.
- [x] The operator reviews the EmptyState, Icon, Text, and Eyebrow routes in
      the paired Svelte/React previews before merge and accepts their exact
      axis rows.
- [x] No compatibility twin, no new shared render tree, no new conformance
      architecture, and no Jetstream parity claim.
- [x] One batch log records exact commands, counts, package change class,
      migration notes, downstream search, and unresolved findings.

## Writable Scope

- `docs/contracts/components/{empty-state,icon}.md`; update the observable
  contract first in the worker batch, then make implementation follow it
- `packages/contracts/components/src/{empty_state,icon}.rs`
- `packages/render/src/empty_state.rs` and direct Poodle Rust callers of the
  removed APIs, including mechanical Jetstream compile consumers only
- `packages/svelte/components/src/Icon.svelte`,
  `packages/react/components/src/Icon.tsx`, their focused tests, shared Icon
  styles if dead density selectors exist, and direct Icon-density callers
- Svelte, React, and GPUI specimen layout/scene adapters and the four named
  component specimen pages
- `packages/codegen` specimen model/source and regenerated specimen artifacts
- `test/parity/specimen-axis-census.test.tsx` and focused headless GPUI
  structural evidence
- one August batch log
- append-only `PAPERCUTS.md` for new execution friction only

Do not edit unrelated components, the generation runway, dispatch ledger,
release automation, `.github/workflows/`, or worker handoff files.

## Validation

Run one coherent headless round after the migration:

- focused component, Rust contract/render, parity-census, and GPUI structural
  tests added by this card
- `effigy ir:check`
- `effigy test:components`
- `effigy check:svelte`
- `effigy react:build`
- `effigy test:parity`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy test:web-pack-install`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Never run `*-windowed`, `test:native-visual`, `qa:jetstream`, or
any release mutation.

## Stop Conditions

- Another component with a non-standard public axis domain appears outside the
  measured EmptyState/Text/Eyebrow set and changes the migration denominator.
- A real downstream project uses one of the removed public APIs.
- A component cannot identify every rendered axis value without coupling the
  specimen shell to component internals.
- The generated scene path needs behaviour, callbacks, or conditional logic.
- The change requires a compatibility shim or a broader public API decision.
- A validation failure changes the plan rather than exposing an implementation
  defect inside this card.

## Continuation

After merge and closeout, readiness-review `g15.020`. Do not dispatch
`g15.026`, `g15.012`, or `g15.013` from this worker.
