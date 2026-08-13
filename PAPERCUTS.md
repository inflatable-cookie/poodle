# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

<!-- Keep entries short. Append newest entries at the top. Do not include secrets. -->

- 2026-08-12 — RESOLVED at review. `g13-b041` had `Button.svelte`/`Button.tsx`
  import the generated definition across a package boundary
  (`../../preview/src/generated/button`), so the packed `poodle-svelte` /
  `poodle-react` tarballs (`files: src`) did not carry it and
  `test:web-pack-install` failed to resolve the import in its consumer. Green
  on `main`, red on the branch — a real regression, not a pre-existing gap.

  **Cause was the card, not the worker.** `041`'s Writable Paths named
  `packages/{svelte,react}/preview/src/**/generated/**` as the artifact
  location, copied from `b035` where the consumer *was* the preview. A
  component's artifact has to live in the package that ships it. Fixed at
  review: the `button-ts` target now emits into
  `packages/{svelte,react}/components/src/generated/button/`, the four imports
  point there, and `test:web-pack-install` is green again.

  **`test:web-pack-install` is not in `ci:web`.** Every gate the card listed
  passed while the packaged build was broken. A card that moves where code is
  emitted must run it; better, it belongs in `ci:web`.

- 2026-08-12 — g13-038 found a dead focus rule: `.poodle-order-by__item:focus-visible`
  (order-by.css) rings the item row, but the item div is never focusable — no
  `tabindex`, and the only focusable inside is the drag-handle button. The ring
  never renders; the drag-handle got its own ring this batch. The dead rule
  should be deleted or retargeted to the handle.

- 2026-08-12 — RESOLVED 2026-08-12: g13-038 closed the sweep. The 56-sheet
  stacking is gated by `docs:focus-ring-drift` (absent-treatment + stacked-UA
  checks with baselines); hidden inputs and machine-driven controls now set
  `outline: none`, and the detail-item/field info triggers defer to the nested
  Popover's own ring. Original report follows. g13-037 measured the other half
  of the 2026-08-11 focus-ring entry: 56 component stylesheets draw their own
  focus ring without suppressing the UA `outline: auto` on the same element,
  so Chrome stacks a second 1px ring. Live in checkbox/radio/switch/tri-state/
  segmented-control (the hidden native input draws `outline: auto` — invisible
  on the clipped inputs, so harmless there) and visibly on the
  `detail-item`/`field` info-trigger wrappers, whose own ring lands on the
  icon while the UA ring draws on the focusable wrap. The follow-up card owns
  the sweep; the fix pattern is `outline: none` on the focusable element + the
  component's own ring on its visible surface.

- 2026-08-12 — g13-036: the scene now owns the control labels in all four
  shells, but the web shells still hardcode accessibility labels: Svelte and
  React `DisplayControls` pass `ariaLabel="Neutral contrast"` (contrast),
  `ariaLabel="Search components"` (search) and `ariaLabel="Control size"`
  (size) while the visible label comes from the scene (`Contrast`/`Search`/
  `Size`). The natives are aligned (GPUI's contrast aria label is now the
  scene's `Contrast`; Jetstream's was already), so the web pair carries the
  only remaining second copy of label text — R3's "the scene owns the word"
  is not complete until the web aria labels read from the artifact too.
  Out of scope for 036 (web preview sources are not its writable paths).

- 2026-08-12 — RESOLVED 2026-08-12: the include now covers `test/**` and the
  36 errors are cleared (701 files, 0 errors, up from 449). Verified by
  reintroducing the stale `rootContinuationCount` — the gate catches it now.
  The previews are still unchecked and remain open. Original report follows.
  `check:svelte` type-checks `packages/svelte/components/src`
  only. Its tsconfig `include` is `src/**`, so the 36 files in `test/` are not
  checked, and no task type-checks either preview at all. This is not
  hypothetical: the `rootContinuationCount` → `precedingContinuationCount`
  rename (`2a6d3af9`) left four live sites stale — both HistoryCentre specimens
  and both component test suites — and every gate stayed green. Widening the
  include to `test/**` surfaces 36 pre-existing errors across 11 files
  (`SplitView.svelte.test.ts` 12, `AppHeader.svelte.test.ts` 8,
  `contract-prop-drift.ts` 6 via its test, and others); adding `vite/client`
  and `vitest/globals` types does not reduce them, so they are real. Clear that
  backlog, then widen the include, then decide whether the previews get a gate
  of their own.

- 2026-08-12 — g13-032's picker pencil is an `IconButton`, which forwards no
  data attributes (explicit props only, no rest spread): the pencil's
  `data-part="picker-rename"`/`data-branch` markers had to live on a wrapper
  span, and the rename focus-restore targets the wrapper's inner button. Any
  composite that needs a `data-part`/`data-branch` on an `IconButton` hits
  the same wall; a rest-prop pass on `IconButton` (forward `data-*`/`aria-*`
  to the inner button) would remove the wrapper pattern.

- 2026-08-12 — g13-030's picker puts Poodle's `Select` inside the
  `HistoryCenter` popover, and Select portals its listbox to the document
  body — so Tab from an open listbox is outside `Popover`'s focus trap
  (`trapFocusKeydown` only intercepts keys whose target is inside the
  surface; the portalled menu's Tab bubbles to body instead). The Select's
  own arrows/Enter/Escape work and roving traversal is unaffected; the
  escape is a Select-in-Popover composite concern that predates the
  component-level fix. Either trap on a `contains()` that includes the
  Select's portalled layer, or have Select stop Tab from its open listbox.
  Affects any composite that puts a Select inside a trap (HistoryCenter
  first instance).

- 2026-08-12 — g13-029 hit this: b028's `packages/core/src/history-center.ts`
  does not type-check as committed, and `effigy check:svelte` (a card
  acceptance gate) cannot exit 0 until it is fixed. `HistoryCenterOpenFork.inner`
  is declared `HistoryCenterOpenFork | null` (line 198) but every use site
  treats it as `ReadonlyMap<string, HistoryCenterOpenFork> | null`
  (`withAddedLevel`, `new Map(level.inner)`, `findLevel`, `walkLevels`), plus
  strict-null gaps in the machine (`pick` possibly null in `confirm`, `anchor`/
  `entry` possibly undefined). Type-only — `effigy test:core` passes 482/0 —
  but the declaration contradicts the code. Core has no type gate in effigy
  (`test:core` is vitest, which strips types), so the error ships silently.
  Fix: one declaration line + the strict-null guards; a `tsc --noEmit` gate on
  core would have caught it at b028.

- 2026-08-12 — RESOLVED 2026-08-12 by Longhorn `777de887`. The mechanism in
  this report was wrong: `record_applied` always installs the new entry as its
  parent's preferred continuation, so a run's terminal entry has **no**
  children and its `continuationCount` is 0, not 1. Poodle's implementation
  saturates at 0 and was never affected. The report did surface a real hole
  the other way round — `ForkHistory::from_state` accepted a parent with
  children and no preference, and two Longhorn fixtures were already in that
  state, including an anchor whose 64 alternates were unreachable. Longhorn
  added a two-half guard: one child needs no preference, two or more without
  one is `MissingPreferredChild`. Original report follows.
  R4's `forkCount = continuationCount - 1` makes a **single fork
  off a run's last entry invisible**: the authority defines
  `continuation_count` as all children ("this page's own next entry included…
  a run's last entry is always zero", `ForkEntryRecord` doc), so a last entry
  with one divergent child carries 1 → `forkCount 0` → no disclosure
  affordance, no way to open the fork. Verified in the Longhorn prototype
  (`projection/project.rs`: `continuation_count = child_ids(entry).len()`);
  the v3 disclosure model therefore cannot reach such a fork at all, while the
  authority's own doc line claims the case never occurs. Either the authority
  forbids recording a fork at the preferred chain's leaf (making the doc line
  a true invariant), or v3 needs a head-fork affordance. Affects g13.029
  rendering and the Longhorn thread.

- 2026-08-12 — RESOLVED 2026-08-12 (b027 Part 1, `1331b5e5`: the twelve specs
  now carry `dismiss_on_outside_interact` — default `true`, matching the web —
  and `OPEN_GAPS` is `{}` again; each renderer resolves the field). Original
  report follows. A contract cannot document a prop the matching `*Spec` struct
  deliberately lacks without `effigy docs:lint` failing: the contract ↔
  poodle-specs drift gate (`packages/svelte/preview/scripts/contract-spec-drift.ts`)
  requires every documented Public Prop to exist on the Spec. g13-026 needed
  `dismissOnOutsideInteract` documented on twelve specs (`select`, `menu`,
  `context-menu`, `menubar`, `navigation-menu`, `split-button`, `theme-select`,
  `ref-select`, `model-picker`, `order-by`, `list-card`, `filter-builder`) that
  deliberately model no dismissal (default matches every native platform, so a
  field would be invented data) — carried via `OPEN_GAPS` entries instead,
  which the card's writable paths exclude. Same decision-shaped hole as the
  g13.009 `initialFocus` papercut: either the specs gain the fields or the gate
  needs a sanctioned carve-out for "default-is-platform-standard" props. Decide
  once, then burn down both gaps (delete the `OPEN_GAPS` entries).

- 2026-08-12 — RESOLVED 2026-08-12 (b027 Part 2, `c04a9cdc`: the parser no
  longer reads a comma inside a string literal as a prop boundary — regression
  test on the exact `placeholder = "Select date, time, and zone"` line —
  Snippet-typed props are separated from props, and the gate now enforces the
  reverse direction, exiting non-zero on undocumented props). Original report
  follows. `contract-prop-drift` only checks that documented props are
  implemented, never that implemented props are documented, so an undocumented
  public prop can never fail the gate. The reverse direction exists behind
  `DRIFT_REPORT=1` but never exits non-zero and mixes snippets (`children`,
  `footer`) in with real props. It also has a depth bug: it reports `and` and
  `time` as props of `date-time-zone-picker`, both lifted from inside
  `placeholder = "Select date, time, and zone"` and `defaultValue = { date:
  null, time: null, timeZone: null }`, though the comment at
  `contract-prop-drift.ts:51` says default values and object literals are
  skipped. Fix the parser, separate snippets from props, then enforce.

- 2026-08-11 — RESOLVED 2026-08-12 (`5854634c` regenerated the artifacts,
  `761f81d8` added the `gate:snapshot`/`gate:clean` guard that now fails any
  gate which rewrites a committed artifact). Original report follows.
  `effigy docs:check` fails at HEAD once the parity reports
  regenerate: `packages/svelte/preview/artifacts/parity-report.json` reports
  201 exports / 164 components (`HistoryCenter`, from g13-b020/b021) while
  the committed report and `packages/shared-demo-app-audit.json` say
  200/163, so `docs:lint`'s audit-json consistency check fails. The report
  generators have no `--check` mode (b015 failure mode 8), so the committed
  report drifted silently and the first regeneration after any component
  lands breaks the gate. Either regenerate+commit the report artifacts
  together with component landings, or give the report generators a check
  mode wired into `docs:check`. Affects every gate run after a component
  lands.

- 2026-08-11 — (the `isForkPoint` instance was fixed in b023 review; the
  structural point stands and is why it recurs.)
  `packages/core/src/index.ts` hard-lists every per-module
  export (`export { historyCenterTransition, …, isForkPoint } from
  "./history-center"`), so a card that deletes a core export (g13-023's D1
  dropped `isForkPoint`) cannot remove it: `index.ts` is outside the card's
  writable set. Plain bun/Node ESM validates re-export bindings at link time
  (`export 'missing' not found`, probe-verified), so any consumer importing
  the package index crashes until the follow-up card edits `index.ts`;
  vitest is lenient, which is why the component gate showed only the
  HistoryCenter suites red. Same wall recurs for any future export deletion.
  Either make `index.ts` writable on export-deleting cards, or switch these
  blocks to wildcard re-exports (`export * from "./history-center"`) so
  deletions stop breaking the package index.

- 2026-08-11 — RESOLVED 2026-08-12 (`761f81d8`: added the tsconfig and the
  `*.css` ambient declaration, wired `check:svelte-components` into
  `check:svelte`; coverage went from 1 file to 449, and the two real errors it
  exposed were fixed in AudioSwitch). Original report follows.
  `packages/svelte/components` has no `tsconfig.json` and is never
  type-checked. `effigy check:svelte` runs `svelte-check` against
  `packages/svelte/install-smoke` — one file, 0 errors — so the 164 components
  are unchecked. A `class` prop passed to `Icon`, which accepts no such prop,
  shipped through every gate: it was reported from a consumer repo whose own
  type-check caught it. The prop was silently dropped, so the checkpoint pin
  also rendered unstyled. Pairs with the 2026-08-10 React entry (`react:build`
  transpiles without `tsc`): neither web runtime has a type gate. Standing one
  up will surface a backlog, so it wants its own card.

- 2026-08-11 — `box-sizing: border-box` plus `width: 100%` plus a horizontal
  margin overflows by twice the margin: border-box covers padding and border but
  never margin. Hit on `HistoryCenter`'s inline rename input, which spilled past
  the popover edge. `width: auto` is not the escape for form controls — an
  `<input>` is replaced, so auto resolves to its intrinsic size rather than
  filling. Worth a lint rule or a shared field-inset helper, since the
  combination reads as correct.

- 2026-08-11 — Components that render inside a `Popover` can set a width that
  fights the surface's. `HistoryCenter` set `width: clamp(20rem, 34vw, 26rem)`
  while passing the surface `surfaceMaxWidth: min(26rem, ...)`, so content could
  reach the surface's own maximum and then the surface's horizontal padding
  pushed it past the rounded edge. `MessageCenter` has the same shape
  (`clamp(22rem, 36vw, 28rem)` inside a 30rem surface) and only escapes because
  its numbers happen to leave room for the padding — a token change to
  `space-panel-x` would break it. Consider a rule that a popover-hosted root
  never sets its own width, or a check for it.

- 2026-08-11 — The 85-icon default icon set has no undo, redo, history, or
  pin glyphs (no `corner-up-left`, `rotate-ccw`, `pin`, …), so the new
  `HistoryCenter` titlebar cluster falls back to `arrow-left` / `list` /
  `arrow-right` and checkpoint pins to `git-commit-horizontal`. Tooltips and
  labels carry the semantics, but canonical undo/redo/pin affordances need
  the icon set extended (an icon-crate change, not a component change).

- 2026-08-11 — Consumers are compensating for Poodle API gaps with `:global`
  CSS overrides, and those fail silently when Poodle refactors: a scan found
  `.poodle-tabs*` overrides in 9 repos, including three still targeting the
  pre-rename `[data-variant="text"]` selector. Overrides on `__list`, `__tab`
  and `__panel` (figmatic, soundcheck-library, nucleus, underlay,
  loophole-legacy) are each an unmet prop. Treat the override inventory as an
  API-gap backlog, and note that consumer migrations must audit CSS selectors
  as well as props — a selector break produces no type error and no build
  error, just wrong layout.

- 2026-08-11 — happy-dom evaluates `@media` rules only at stylesheet parse
  time: changing the window width afterwards updates `matchMedia` but not the
  cascade, so `getComputedStyle` keeps the pre-resize rules (verified with a
  standalone happy-dom probe). A responsive-layout test must set the width
  *before* injecting the stylesheet, which is fragile when the CSS arrives via
  a component import (vitest stubs CSS anyway, so tests re-inject it).
  Affects any future media-query behavior test in the component suites.

- 2026-08-11 — `cargo build --manifest-path packages/jetstream/preview/`
  cannot resolve its poodle deps from a g13 worktree: the sibling
  `jetstream-poodle` crate (`~/Dev/projects/poodle-wt/jetstream/crates/
  jetstream-poodle/Cargo.toml`) hard-points `../../../poodle/packages/…` at the
  main checkout, which does not exist alongside the worktrees — so the
  jetstream preview never compiles here (g13-013 hit the same wall and logged
  it). The workaround for a one-off check is temporarily repointing those
  paths at the worktree; it should not need to exist. Consider canonicalizing
  the sibling path or documenting the required checkout layout.

- 2026-08-11 — **Resolved 2026-08-12.** The docs preview's global
  `button:focus-visible, input:focus-visible` outline
  (`packages/svelte/preview/src/app.css`) outranked every component that draws
  its own focus treatment: a bare element selector plus pseudo-class is (0,1,1)
  and beats a component class at (0,1,0). 33 component stylesheets set
  `outline: none` for exactly this reason, so all of them were being overridden
  — TextInput visibly rendered its rounded focus border and the preview's
  square outline at the same time. The first fix narrowed it with
  `:not([class*="poodle-"])`, which made it match nothing: every focusable
  element in the shell is a Poodle component or carries a `poodle-` class
  (measured 0 matches across four pages). Dead, but still dangerous — a shell
  copied into a host app takes the rule with it and it starts matching that
  host's chrome. Now deleted. Chrome that needs a ring gives itself one, keyed
  to its own class and radius.

- 2026-08-11 — Removing an explicit `border` declaration from a `<button>`-based
  component style leaks the UA default `2px outset buttonborder`, and the two
  preview shells resolve that default differently (Svelte preview: black,
  React preview: white — a `color-scheme` difference), silently breaking the
  Svelte↔React visual gate. g13-016 hit exactly this: deleting the
  NavigationMenu trigger's unconditional border without adding `border: 0`
  produced a 0.564% pixel diff on the navigation-menu slug only. Button styles
  that go borderless must declare `border: 0` explicitly (the `.poodle-tabs__tab`
  and Menubar trigger pattern). Affects any future border-removal on a button.

- 2026-08-11 — The visual gate cannot catch a bug in a brand-new specimen:
  with no prior baseline to differ from, a new specimen reports no diff. `b013`
  added `activeOutline` specimens and the sweep showed zero tabs diffs, while
  the outline was in fact drawing a square border around a rounded tab. Only a
  human looking at it found it. Consider failing, or at least reporting, when a
  gate encounters a slug with no baseline rather than silently passing.
  Affects visual-gate trust for new components and new specimens.

- 2026-08-11 — Both the Svelte↔React parity gate
  (`test/parity/component-parity.test.tsx`) and the React smoke sweep
  (`packages/react/components/test/smoke.test.tsx`) enumerate React exports
  with `typeof comp === "function"`, and React `forwardRef` returns an element
  type object, not a function — so every ref-forwarding component silently
  drops out of both gates. MenuSurface and (since b010) TextInput already sit
  outside coverage this way; g13-014's AppHeader joined them. Counts drift
  down with no signal. Accept element-type objects in the filter (a
  `$$typeof` check) so forwardRef components stay gated. Affects Svelte↔React
  parity and smoke coverage.

- 2026-08-11 — `packages/tokens/scripts/build-tokens.ts --check` cannot see a
  stale committed artifact: `writeFile` only writes/compares files the
  generator still emits and never deletes (`build-tokens.ts:233-251`), so a
  token file removed from the generator stays committed forever and the gate
  passes. Only the svelte mirror deletes on regenerate
  (`syncSvelteTokenArtifacts`, `:270-274`); `scripts/build-default-icons.ts`
  has the right pattern (stale set, deleted in write mode / failed in check
  mode, `build-default-icons.ts:176-197`). Port the icons stale-detection to
  `audit:tokens`. Affects token drift gating (found by card 015 research).

- 2026-08-11 — The web visual gate's fixed ports (4174/4180) are squattable by
  any other worktree's dev server: a stale `vite --port 4180 --strictPort`
  left running by a previous session makes the gate's strict-port spawn die
  instantly while `waitForPort` polls the squatter's 404s until the 60s
  timeout, with the failing port hidden because the spawned preview's output
  is ignored. `test/visual/run.ts` should pre-flight `isUp` + a content probe
  (200 + expected root) before spawning, or the gate should pick ephemeral
  ports. Affects every card that needs visual enumeration.

- 2026-08-11 — g13-013's batch-card Known State claimed `render_underline`
  renders "no accessories at all — no icon, no count, no close", but
  `build_tab_label` already renders icon+count in every renderer (only close
  was missing). Card Known State is not always current against the working
  tree; workers must verify rather than trust "verified — build on this".
  Affects card-authoring accuracy and worker trust.

- 2026-08-11 — `effigy docs:lint` requires every Cargo.toml under
  `packages/` to be registered in `packages/release-manifest.json` (reverse
  check in `packages/svelte/preview/scripts/lint-docs.ts`
  `validateReleaseOperations`), so a new contract crate cannot pass the docs
  gate until the orchestrator adds its entry — but card writable-path rules
  exclude the release manifest. g13-011 shipped `poodle-ir` with `docs:lint`
  failing on exactly this error and recorded the conflict in its batch log
  (entry shape already fixed by ruling R1: kind `contract-crate`, channel
  `preview`). Either extend the lint to accept a "registration pending" state
  for new crates, or make the release-manifest writable on crate-creation
  cards. Affects new-crate onboarding.

- 2026-08-11 — `effigy visual:report` (sweep tier) reports 53 failing
  Svelte↔React pairs at HEAD (308 compared; 46 size / 4 capture / 3 pixels,
  spread across both themes — e.g. `tree`, `xy-pad`, `tool-call`, `fader`,
  `agent-plan`) with no allowlist or debt inventory covering them, so report
  mode cannot distinguish pre-existing parity debt from a card's real
  regressions without a manual before/after comparison (g13.008 had to run
  the gate twice and diff the summaries). Track the failing-pair set (committed
  allowlist with reasons, or a parity-debt inventory) so cards can diff
  against it. Affects visual-gate triage.

- 2026-08-11 — A new public web prop cannot be documented in a contract
  without `effigy docs:lint` failing: the contract ↔ poodle-specs drift gate
  (`packages/svelte/preview/scripts/contract-spec-drift.ts`) requires every
  documented Public Prop to exist on the matching `*Spec` struct, so g13.009's
  `initialFocus` (a focus-behaviour semantic, not web plumbing) cannot ship
  without `DialogSpec`/`FormDialogSpec` gaining `initial_focus` — stopped the
  card; see `docs/logs/2026-08/11-g13-009-dialog-initial-focus.md` §5. Either
  the Specs need the field (native renderers own focus per dialog.md §10) or
  the gate needs a sanctioned web-only carve-out; a decision is required
  before the next prop of this kind lands.

- 2026-08-11 — Batch card 006 (button tone parity) lists `split-button.css`
  as "implements danger", but the contract amendment it depends on
  (`282ce489`) added a `--poodle-split-shadow` column to split-button.md §8
  "Tone: danger" that the stylesheet never implemented — primary danger
  resolves `none` against a mandated elevation shadow (stopped the card;
  see `docs/logs/2026-08/11-g13-006-button-tone-parity.md` §5). Card gap
  tables should be re-verified against amended contracts at dispatch time.

- 2026-08-11 — `docs/guides/svelte-developer-guide.md` §Types still defines
  `type ButtonTone = "default" | "danger"` (2 members) while
  `docs/contracts/004-shared-control-types.md` canonically defines all four
  (`default | danger | success | warning`); the same block restates
  `ControlSize`/`ControlDensity`/`SemanticControlSizeRole`/`ButtonVariant`/
  `ValidationState`/`StatusTone` with no link to 004. The guide contradicts the
  canonical contract and misleads docs-based type resolution. Refresh the block
  or replace it with a 004 reference. Affects docs-authority consistency.

- 2026-08-11 — The g13.002 generated-LOC measure globs
  `packages/core/src/tokens/generated/*` and `packages/tokens/artifacts/**`
  return 0 lines without bash `globstar`: the first matches only the
  `css/`/`ts/` subdirectories and `**` degenerates to `*`. Actual counts are
  1,813 lines / 25 files and 2,407 lines / 31 files. Document the recursion or
  use an explicit recursive glob. Affects reproducible measurement.

- 2026-08-11 — `pilot-expressiveness-corpus.md` §8 counts `EXT` as
  Button 5 / RangeSlider 3 / TextInput 2 (10 total), but the row-level
  classification marks only 3/2/1 (6 total). The manifest records both; the
  count table drifted from the rows. Recompute the §8 table from the rows.
  Affects corpus arithmetic.

- 2026-08-11 — Jetstream RangeSlider densities specimen
  (`packages/jetstream/preview/src/specimens/range_slider.rs`) renders the
  standard variant, while range-slider.md §13 / `RNG-25` specify density
  specimens use embedded bipolar (Svelte/React do). Align the Jetstream
  densities group. Affects cross-runtime specimen parity.

- 2026-08-11 — `docs/parity/text-input.md` status line reads `gpui=2
  jetstream=2` but its own gap sections enumerate 8 GPUI / 9 Jetstream open
  `[ ]` bullets. The pass-level count drifted from the bullets (distinct from
  the stale-path lag of `OBS-04`). Refresh the status line. Affects parity-doc
  readability.

- 2026-08-11 — `effigy docs:check` rewrites the committed
  `packages/tokens/artifacts/rust/*` via `report:parity` → `tokens:build`,
  leaving a dirty worktree, and `effigy audit:tokens` fails at HEAD: commit
  `45caae82` rustfmt-formatted the generated Rust artifacts without updating
  `packages/tokens/scripts/build-tokens.ts`, whose emitter writes 4-space
  override arrays. Align the generator with the committed formatting or
  regenerate the artifacts from it. Affects the docs gate and token drift
  audit.

- 2026-08-11 — React ports of Button and TextInput omit contract-listed
  web-native props: `Button.tsx` has no `formenctype`/`formmethod` passthrough
  and `TextInput.tsx` has no `autocorrect`, all three documented in the
  contracts and implemented in Svelte. Add the props to the React components.
  Affects Svelte↔React surface parity.

- 2026-08-11 — A playbook-style detached `nohup omp -p` launch exited
  immediately with an empty log and untouched worktree under the Codex exec
  harness. Document a harness-safe detach or persistent-session recipe.
  Affects worker orchestration from agent threads.

- 2026-08-10 — `effigy doctor` runs a failing health check that reformats
  generated Rust token artifacts and unrelated Rust tests, leaving a dirty
  worktree. Make doctor diagnostics read-only or isolate formatter output.
  Affects routine repo-health checks.

- 2026-08-10 — `effigy react:build` runs Vite transpilation without a TypeScript
  check; direct `tsc` finds pre-existing `AgentChatStatus` errors the build
  reports as green. Add a React consumer type-check selector and CI lane.
  Affects React parity work.

- 2026-08-10 — A combined `git log -G` secret-pattern history scan ran for
  more than 90 seconds without progress or a result. Add a bounded, cached
  tracked-tree and history secret scanner. Affects public-release audits.

- 2026-08-10 — `effigy tasks` advertises a `distribution` built-in, but
  `effigy distribution --help` resolves it as an undefined catalog task and
  the general help omits it. Align discovery and command routing. Affects
  package-publication audits.

- 2026-08-10 — `effigy graph explore` can hang after a successful index refresh
  without progress or timeout output. Add a bounded query timeout and a clear
  fallback diagnostic. Affects agent code-navigation workflows.

- 2026-08-10 — `cargo fmt --manifest-path` follows Poodle's linked Rust
  workspaces and reformats unrelated crates, creating broad release-sweep
  churn. Add an Effigy formatter that accepts an explicit changed-file set.

- 2026-08-09 — Vite JSON named exports omit kebab-case `lucide-static`
  keys, so bundle-safe imports cannot express most icon names. This blocks a
  plain `createIconSet({ ... })` helper over `icon-nodes.json`; add a build-time
  extractor or a per-icon node package surface. Affects web icon adoption.

- 2026-08-06 — `effigy doctor` reports the repo's `isolation` manifest key as
  unsupported, so routine health checks cannot go green on the checked-in
  manifest. Align the manifest schema or update Effigy's accepted config keys.


- 2026-08-12 — RESOLVED 2026-08-13 (g13-044): `hasMounted` is now `$state`,
  so the persistence `$effect` re-runs on mount and on every preview-mode
  change; the Svelte preview writes `?theme=…&density=…&controlSize=…` back
  to the URL exactly like React's `useEffect` (browser-verified, incoming
  params preserved). Original report follows.
  Svelte preview's SHELL-08 URL persistence never fires: in
  `packages/svelte/preview/src/App.svelte`, the `$effect` that writes
  `theme`/`density`/`controlSize` back into the URL is gated on
  `hasMounted`, a plain `let` in a runes-mode component — not reactive, so
  the effect never observes the state it is supposed to re-serialize.
  Measured live (g13-035): clicking density/size updates the top-bar pills
  and `data-theme` but `location.href` keeps the pre-change query string,
  with no console error; the React preview's equivalent `useEffect` does
  persist (`?theme=forest&density=default&controlSize=lg` after clicks), so
  the two web shells drift on SHELL-08. Either make `hasMounted` `$state`
  (or drop the guard and run the write on mount) or route the effect through
  `$effect` with an explicit reactive flag. Affects the shared-preview-shell
  parity story once 036 lands.

- 2026-08-12 — RESOLVED 2026-08-13 (g13-044): both `CatalogueLanding`
  surfaces now derive their grid groups from the `components` prop, so the
  grid card count equals the header count under any query
  (browser-verified: 6/6 with query `date`). Original report follows.
  Both catalogue-landing grids ignore their filtered component
  list: `packages/svelte/preview/src/pages/CatalogueLanding.svelte` and the
  React mirror render `componentsByTag()` (every component) and use the
  `components` prop only for the count line. With a search query active, the
  sidebar filters correctly and the header reads "2 components" while the
  grid still renders ~164 cards (measured live in both previews, g13-035).
  The search axis itself works — the data flows — but the landing grid
  defeats the filter's visible result. Fix: derive `groups` from the
  `components` prop (filter `componentsByTag()` items by the passed set)
  in both files.

- 2026-08-12 — Two findings from the SettingsShell specimen.

  **Poodle's Svelte `Button` silently drops `onclick`.** Its prop is `onClick`;
  `onclick` falls into `restProps`, which is spread at `Button.svelte:146` —
  *before* the component sets its own `onclick={handleClick}` at `:172`. The
  component's handler wins and the consumer's is discarded with no error. The
  SettingsShell specimen shipped with five dead buttons for exactly this reason
  and looked, from outside, like a broken component. React is unaffected —
  `onClick` is its own convention there. Any `on*` prop the component also binds
  natively has the same shape.

  **`DetailItem`'s responsive rule keys off the wrong box.** It declares
  `container-type: inline-size` on `.poodle-detail-item` and then writes
  `@container (max-width: 26rem)` rules targeting `.poodle-detail-item` itself
  — but an element cannot be matched by its own container query, so the rules
  resolve against the nearest ancestor container instead. Measured inside
  SettingsShell: the item is 240px wide, its ancestor container is a page-wide
  `.poodle-detail-section`, so the 26rem query never fires; the label column
  takes its `11.25rem` max and the value column resolves to **20px**, at which
  `word-break: break-word` breaks values character by character ("Da rk",
  "Co m pa ct"). Any narrow grid cell hits this, not just SettingsShell. The fix
  is a layout decision on a shared component with three layouts and two
  presentations, so it wants its own card rather than an improvised column
  tweak.

- 2026-08-13 — `detail-section.css`'s `[data-separated="true"]::before` (a
  0.125rem separator inset) sits inside the section's own `@container
  (max-width: 28rem)`, so it resolves against the enclosing
  `DetailSectionGroup` rather than the section. Plausibly the box you want for
  a separator inset shared across sibling sections — but **it was not
  measured**, and it is baselined in `container-query-drift.ts` on that
  unverified reasoning. Measure it and either delete the baseline entry or
  restructure; do not leave it indefinitely on a guess.

