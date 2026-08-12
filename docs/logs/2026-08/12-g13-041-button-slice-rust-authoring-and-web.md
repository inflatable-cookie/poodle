---
title: g13 batch 041 — Button vertical slice, Rust authoring and the two web runtimes
status: complete
milestone: g13.005 (part 1 of 2 — does not close the milestone)
owner: Poodle core
updated: 2026-08-12
tags: [log, g13, IR, button, component, authoring, svelte, react, spec-063, g13.005]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/041-button-slice-rust-authoring-and-web.md` on
branch `thread/g13-041-button-slice-rust-authoring-and-web`: authored the
Button definition in Rust (R1), serialized it to its own fixture, emitted it
to both web packages' `generated/` directories through a new select-only
`button-ts` target, and rewired `Button.svelte` and `Button.tsx` to read the
**rendered vocabulary** from the artifact instead of hard-coding the eleven
`data-*` attribute names and the anatomy classes inline (R2). The R2 proof —
the ruling that decides whether the card proves anything — ran live: renamed
`data-tone` in `button.rs`, one `ir:build`, and both web previews emitted
`data-tone-level` in their DOM with no hand edit; restored, both emitted
`data-tone` again.

Per the card's worker rules: no sub-agents; sources read directly
(`button.md`, both `Button` implementations, the full `ComponentDefinition`
vocabulary in `packages/contracts/ir/src/*` and its validation rules before
authoring); no planning/status authority exercised beyond the card's own
writable status line. No stop condition was reached: every capability Button
needs has a `poodle-ir` field (R2 held — no `poodle-ir` change), the rendered
vocabulary reached both runtimes with no schema change and no new preview
dependency, all 34 web props and all 11 attributes stayed byte-identical
(`svelte:surface-audit`, `docs:contract-drift`, and every existing Button
test prove it), and no pixel moved (`button.css` untouched, class and
attribute values identical before/after, parity test green).

## Measured before-state — the surface this card preserves (step 2)

**The 34-web-prop surface** (the card's R3 line): 27 data props
(`variant`, `tone`, `size`, `sizeRole`, `density`, `type`, `form`,
`formaction`, `formenctype`, `formmethod`, `formnovalidate`, `formtarget`,
`disabled`, `loading`, `leadingIcon`, `trailingIcon`, `chevron`, `truncate`,
`fit`, `maxWidth`, `pressed`, `defaultPressed`, `ariaLabel`, `ariaExpanded`,
`describedBy`, `className`, `style`) + 4 callbacks (`onClick`, `onFocus`,
`onBlur`, `onPressedChange`) + 3 snippet slots (`children`, `leading`,
`trailing`).

**The eleven data attributes and their value domains** (B §9, BTN-18):

| Attribute | Form | Emission | Value domain |
|---|---|---|---|
| `data-variant` | valued | always | primary, secondary, ghost |
| `data-tone` | valued | omit-when-default | danger, success, warning |
| `data-size` | valued | always | xs, sm, md, lg, xl |
| `data-density` | valued | always | compact, default, comfortable |
| `data-icon-only` | presence-only | when icon-only | — |
| `data-has-leading` | presence-only | when leading content or loading | — |
| `data-has-trailing` | presence-only | when trailing content or chevron | — |
| `data-truncate` | presence-only | when truncate | — |
| `data-fit` | valued | omit-when-default | content |
| `data-loading` | valued | always | true, false |
| `data-pressed` | valued | toggle mode only | true, false |

## Deliverables (only the card's writable paths)

- `packages/codegen/src/models/button.rs` (new) — the Rust-authored Button
  definition (`button_model()` / `button_definition()`): 9 shared types
  (`button-variant`, `button-tone`, `control-size`, `control-density`,
  `control-size-role`, `button-fit`, `button-type`, `button-form-enc-type`,
  `button-form-method`), 30 props + 4 events (the 34-web-prop surface), 6
  anatomy parts, 11 state attributes with emission conditions (the
  expression vocabulary), the size/density axes with the contract's fixed
  rem ladder (BTN-23), 22 semantic token refs (BTN-22), the 76 distinct
  recipe hooks with their override chains, accessibility intent (BTN-21),
  keyboard table (BTN-20), the 11-field VisualState projection (BTN-19), and
  the B §12 known deltas as extensions (BTN-26/27/29). Module header records
  the pilot-scoped placement and the no-macros rule, and names the two
  vocabulary notes below.
- `packages/codegen/src/models/mod.rs` — `pub mod button;`.
- `packages/codegen/src/targets/button.rs` (new) — the `button-ts` target
  (output root `generated/button`, one `index.ts` per model): the artifact
  carries the **rendered vocabulary** — `parts` (id, name, DOM class,
  parent), `attributes` (id, name, form, emission, **value domains**), and
  `recipeHooks` (hook + override chain). The part→class projection is
  authored once here (`poodle-button__<part>`, with the two documented
  exceptions: root is `poodle-button`, and the Leading/Trailing Icon parts
  share `poodle-button__icon` — B §2); the value domains are the emitter's
  projection of each attribute's source type (shared-type members, or
  `true`/`false` for booleans), minus the default member when the emission
  policy is omit-when-default.
- `packages/codegen/src/targets/mod.rs` — `button-ts` registered in the
  `selectable()` list, **not** in `all()`: a plain `ir:build` over the
  synthetic fixture must never write into a web package.
- `packages/codegen/src/write.rs` and `src/check.rs` — the orphan sweep and
  the drift scan are scoped to each target's **top level**. Two targets now
  own files in the same physical `generated/` directory inside the web
  packages (`shell-scene` at its top level, `button-ts` in its
  `generated/button/` subroot), and a recursive sweep would delete the
  sibling target's artifact — observed live during this card (the first
  `ir:build` deleted `preview-shell.ts`; `ir:check` then flagged `button.ts`
  as a stale orphan). Nested directories are other targets' roots; both
  modes agree on what "stale" means.
- `packages/codegen/src/bin/poodle-codegen.rs` — `--author-button <OUT>
  [--check]`, the mirror of `--author-shell`: serialize the authored Button
  model to the fixture after a validate round trip; the check branch remains
  structurally incapable of writing.
- `packages/codegen/fixtures/button-model.json` (new, 90.5 KB) — the
  serialized Button model, generated by `--author-button`.
  `synthetic-model.json` and `shell-model.json` untouched.
- `packages/{svelte,react}/preview/src/generated/button/index.ts` (new) —
  the committed artifact both web components consume. Byte-identical in
  both packages (the parity test proves it).
- `packages/svelte/components/src/Button.svelte` and
  `packages/react/components/src/Button.tsx` — the components read the
  artifact: a `<script module>`-level `parts`/`attributes` map (Svelte) and
  module-level maps (React) supply the anatomy class names and the eleven
  `data-*` attribute names; the markup emits them via an attribute spread.
  The value derivation stays in the components (the runtime's projection,
  CROSS-14). No prop, default, class string, or emitted value changed.
- `packages/{svelte,react}/components/test/Button.generated.test.ts(x)`
  (new) — the definition→DOM tests: the DOM attribute names equal the
  artifact's entries, `data-loading` always emits even as `false`, tone/fit
  omit at default, and the anatomy renders under the artifact's classes.
- `tasks/effigy.tasks.toml` — `ir:build` / `ir:check` now run
  `--author-button` first (write / byte-compare), then the button fixture
  through `--target button-ts` into each web package. `ir:check` remains the
  only gate-shaped selector.
- `packages/codegen/tests/button.rs` (new, 7 tests) — see Tests.
- `docs/roadmaps/g13/005-button-component-vertical-slice.md` — status line
  only: `planned` → `in progress` (part 1 of 2 landed; 042 closes the
  milestone).
- `PAPERCUTS.md` — one appended entry (the pack-install consequence, below).
- This log.

Nothing else in the repo changed. No `poodle-ir` change (R2); no
`poodle-render`, adapter, or native preview touched (R5); `synthetic-model.json`
and `targets/shell.rs` untouched; no visual baseline refreshed; `button.css`
untouched (R3).

## Design

- **The authoring form.** `button.rs` is ordinary Rust types and constructor
  helpers (spec 063 "Authoring Form"), no macros. The expression conditions
  use the bounded vocabulary (`ne` against a shared-type member for
  omit-when-default, visual-field references for derived gates). Prop order
  is the contract's §3 table order; shared types carry `canonical_ref` to
  `button.md` / `004-shared-control-types.md`.
- **The R2 reading — the artifact carries the rendered vocabulary.** The
  shell scene precedent (card 035) was labels-as-projection; the component
  card's equivalent is the DOM vocabulary itself. `parts` gives the anatomy
  and the classes the markup renders; `attributes` gives the eleven names,
  their forms, emission policies, and value domains; `recipeHooks` gives the
  override chains `button.css` already consumes. The components read the
  names and classes; renaming an attribute in `button.rs` moves the DOM with
  no hand edit (proven, step 7). The value domains are projections of the
  source prop types minus the default member under omit-when-default — no
  IR field was needed (R2 held).
- **The cross-package artifact location.** The card's writable paths put the
  artifact in `packages/{svelte,react}/preview/src/generated/**`, so
  `Button.svelte`/`Button.tsx` import `../../preview/src/generated/button`
  across the package boundary. That is a pilot consequence, recorded below.
- **The shared-root sweep fix.** `shell-scene` and `button-ts` both write
  into the web packages' `generated/` directories. The orphan sweep was
  recursive and would delete the sibling target's artifact (observed). The
  fix: each target owns its output root's top level; `button-ts` owns the
  nested `generated/button/` root; both write and check scan top-level files
  only. The b015 stale-orphan protection is unchanged for every existing
  target (all emit flat files; the drift tests plant top-level orphans).
- **What stayed hand-written and why (R4).** The R4 inventory is below, per
  runtime. The web runtimes read the names, forms, and domains; they keep
  their own condition evaluation (the IR expressions are type-checked data;
  evaluating them in the web targets is a g13.008 question — this pilot's
  proof is the rename test, which the artifact satisfies directly).

## The R2 proof (step 7, live)

Renamed the `tone` attribute in `packages/codegen/src/models/button.rs`
(`data-tone` → `data-tone-level`), ran `effigy ir:build`, and drove both
previews with a browser:

| Step | Svelte preview (worktree, :4174) | React preview (worktree, :4181) |
|---|---|---|
| after rename + rebuild | 23 buttons; **6 carry `data-tone-level`**, 0 carry `data-tone`; danger specimen emits `data-tone-level="danger"` | 23 buttons; **6 carry `data-tone-level`**, 0 carry `data-tone`; danger specimen emits `data-tone-level="danger"` |
| after restore + rebuild | 23 buttons; **6 carry `data-tone`**, 0 carry `data-tone-level` | 23 buttons; **6 carry `data-tone`**, 0 carry `data-tone-level` |

One definition change moved both web previews' DOM in one `ir:build` with no
hand edit. The only file touched during the proof was `button.rs` (renamed,
then restored). The restored artifact contains zero occurrences of
`data-tone-level`.

Note for the record: an unrelated vite server was already listening on
:4173/:4180 from the main repo (`~/Dev/projects/poodle`), so the proof ran
against worktree servers on :4174/:4181 and the cwd of each listener was
verified.

## The R4 hand-written exception inventory

Spec 063's acceptance: *"hand-written exceptions are zero or explicitly
justified in the pilot log."* The inventory per runtime — what came from the
definition, what stayed hand-written, and why.

### Svelte (`Button.svelte`)

**From the definition (via the artifact):** the eleven `data-*` attribute
names; the root and part class names; the attribute value domains (carried in
the artifact).

**Hand-written:**

| Exception | Reason |
|---|---|
| the `<button>` element and its native attributes (`type`, form family, `disabled`, `aria-*`) | DOM element and native-attribute projection are adapter-owned (`NEG-02`, `IR-05`); the definition declares the mappings (`accessibility.native`/`aria`), the runtime projects them (`CROSS-03` marks the web-only props) |
| event wiring (`onclick`/`onfocus`/`onblur`, the toggle state machine, `onPressedChange`-before-`onClick` ordering) | events are declared intent (`CROSS-05`); delivery is framework lifecycle (`IR-05`). The ordering constraint is recorded in the definition |
| the derived values (`$derived`: `isToggle`, `currentPressed`, `isUnavailable`, `iconOnly`, `hasLeading`, `hasTrailing`, `leadingContent`, `trailingContent`, `resolvedSize`, `resolvedDensity`, `resolvedStyle`) | the VisualState projection's runtime computation (`CROSS-14`, BTN-19): the definition declares the fields; the runtime computes them — the spec's own model (drawing consumes serializable state, `IR-06`) |
| snippet interpretation (`children`/`leading`/`trailing`, `@render`) | opaque framework payloads cannot cross the serializable boundary (`IR-04`; BTN-16) |
| `Spinner`/`Icon` sub-component composition | shared-primitive reuse (B §2), not Button's definition |
| `restProps` passthrough | the documented native-attribute escape hatch (B §6) |
| `resolvedStyle` composition (`maxWidth` → inline `style`) | BTN-13 composition; inline-style mechanics are web-owned |
| per-attribute value derivation (`tone !== "default" ? tone : undefined`, …) | the emission-policy logic (`CROSS-13`) implemented in the runtime; the names, forms, and domains come from the definition. The web targets do not yet evaluate IR expressions — a g13.008 question |

### React (`Button.tsx`)

Mirrors Svelte exactly, plus:

| Exception | Reason |
|---|---|
| `useState` for `uncontrolledPressed` | the controlled/uncontrolled toggle machine (`CROSS-04`) in React's lifecycle idiom (`IR-05`) |
| camelCase web-only prop names (`formAction`, `formNoValidate`, `formTarget`) | React's prop-naming convention for web-only props (`CROSS-03`) |
| JSX children/leading/trailing interpretation | framework content slots (BTN-16) |

### Both runtimes, shared

| Exception | Reason |
|---|---|
| recipe hooks are carried in the artifact but consumed by `button.css`, not by component markup | the CSS is the styling seam (R3 — untouched); the definition is now the single record of the hooks and their chains |
| the artifact's `values` domains are carried, not re-derived in markup | the emitted values are the props' values; re-reading them in markup would double-store |

## Vocabulary notes recorded for g13.008

- **`Value::Null` is not a valid default for `Shared`-typed props.** The
  nullable shared props (`size`, `density`, `formenctype`, `formmethod`)
  record `default: None` and name the null default in their description.
  `poodle-ir`'s `value_matches_type` excludes `Null` for `Shared`; widening
  it is schema work, not this card's.
- **Button's toggle pair is controlled-wins, not do-not-mix.** The IR's only
  `ControlRule` is `DoNotMix`, and Button allows both `pressed` and
  `defaultPressed` bound simultaneously (controlled wins). The pair is
  therefore recorded through the props and the VisualState projection
  (`isToggle`, `currentPressed`) instead of `controlled_state`.
- **The web artifact location is a packaging consequence.** The card's
  writable paths put the artifact in the preview packages; the component
  packages import across the boundary. `test:web-pack-install` (outside this
  card's gate list) now fails: packed `poodle-svelte`/`poodle-react`
  tarballs carry `src` only, so the consumer cannot resolve
  `../../preview/src/generated/button`. The artifact location is a g13.008
  production-placement question; recorded in `PAPERCUTS.md`.
- **Shared-root orphan sweeps.** Two select-only targets now own files in
  one physical `generated/` directory; the sweep/scan scope is each target's
  top level (`write.rs`/`check.rs`).

## Tests (53 total in `poodle-codegen`, all passing)

`tests/button.rs` (7 new):

- `button_model_validates_and_round_trips_as_json` — in-memory validate
  clean; serialization round-trips; the committed fixture equals the
  authored model (the fixture cannot drift from the Rust source without
  `ir:check` failing).
- `button_definition_authors_the_full_contract_surface` — 30 props + 4
  events (the 34-web-prop surface), 11 attributes, 6 parts, 76 recipe
  hooks, 9 shared types, the 5-rung size ladder with the contract's fixed
  rem metrics, and key defaults (R3).
- `both_web_components_carry_the_same_button_derived_artifact` — the card's
  required parity test: the expectation is the target's render of the
  authored definition (derived, not hand-listed), and **both** committed web
  artifacts must equal it byte-exact. A component drifting on attribute
  names, part classes, or recipe hooks fails the comparison.
- `artifact_renders_parts_attributes_and_recipe_hooks` — the R2 vocabulary:
  every part id and class (including the shared icon-span class), every
  attribute name with its form and value domain (variant domain, tone domain
  minus the default, size domain, fit domain minus the default, boolean
  domains), and every recipe hook with its chain kinds.
- `artifact_header_names_the_source_definition_and_generator_version` — the
  Generated Artifact Contract.
- `one_definition_change_moves_both_web_artifacts` — the R2 proof encoded:
  renaming `data-tone` → `data-tone-level` in a cloned model moves **both**
  committed artifacts in one build.
- `button_artifacts_fail_check_on_drift_and_check_never_writes` — the CLI
  `--target button-ts --check` fails on planted drift + stale orphan and
  leaves the tree byte-identical; `--author-button` gates the fixture the
  same way.

Plus new unit tests in `targets/button.rs` (part-class projection, 1) and
the two definition→DOM component tests per runtime (Svelte 4, React 4).

## Validation (all step-9 commands exit 0)

| Command | Exit state |
|---|---|
| `effigy ir:build` | 0 — authored button + shell models, synthetic fixture, all web/native targets |
| `effigy ir:check` | 0 — all current (button fixture + both `button-ts` artifacts gated) |
| `effigy ci:rust` | 0 |
| `effigy test:core` | 0 |
| `effigy test:components` | 0 |
| `effigy test:parity` | 0 |
| `effigy check:svelte` | 0 — install-smoke + 706 component files, 0 errors |
| `effigy docs:lint` | 0 |
| `effigy docs:contract-drift` | 0 — the public prop surface is unchanged (R3) |
| `effigy docs:callback-drift` | 0 |
| `effigy docs:focus-ring-drift` | 0 |
| `effigy drift:recipes` | 0 |
| `effigy svelte:surface-audit` | 0 |
| `effigy ci:web` | 0 |
| `git diff --check` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 — 53 passed |
| `cargo clippy --manifest-path packages/codegen/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check` | 0 |

**Drift proof (required test).** Planted one line into
`packages/svelte/preview/src/generated/button/index.ts` → `effigy ir:check`
exits 1 naming the artifact → restored → exits 0. `--author-button --check`
gates the fixture the same way.

**R3 proof — the surface is byte-identical.** The components' prop
interfaces were not touched; `docs:contract-drift` and `svelte:surface-audit`
pass, the parity test's class-set diff passes, every existing Button test
(`Button.test.ts`/`.tsx`) passes unchanged, and the browser samples before
and after the proof show the same attribute set and values (`data-variant`,
`data-tone`, `data-size`, `data-density`, `data-loading="false"` on the same
specimen buttons).

## Acceptance criteria

- [x] Button's definition is Rust-authored; `poodle-ir` gained no field and
  no `[[bin]]`.
- [x] A definition change moves the DOM in both web previews (R2), shown
  live and encoded as a test.
- [x] All 34 props unchanged; all 11 attributes unchanged; no pixel moved
  (`button.css` untouched, class/attribute values identical, parity green).
- [x] The hand-written exception inventory exists, per runtime, with
  reasons.
- [x] `synthetic-model.json` and `targets/shell.rs` untouched.
- [x] All step-9 commands exit 0; no baseline refreshed.

## Not done

Per batch card and worker rules: no merge (branch pushed only), no
`poodle-render`, adapter, or native preview work (042), no other component,
no IR schema change, no visual-baseline refresh, no hand edit of generated
files. `test:web-pack-install` is not part of the card's gate list and was
not run as a gate; its breakage from the cross-package artifact import is
recorded in `PAPERCUTS.md` and above as a g13.008 production-placement
question.
