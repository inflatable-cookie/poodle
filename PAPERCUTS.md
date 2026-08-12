# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

<!-- Keep entries short. Append newest entries at the top. Do not include secrets. -->

- 2026-08-12 — `contract-prop-drift` only checks that documented props are
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

- 2026-08-11 — The docs preview's global `button:focus-visible,
  input:focus-visible` outline (`packages/svelte/preview/src/app.css`) outranked
  every component that draws its own focus treatment: a bare element selector
  plus pseudo-class is (0,1,1) and beats a component class at (0,1,0). 33
  component stylesheets set `outline: none` for exactly this reason, so all of
  them were being overridden — TextInput visibly rendered its rounded focus
  border and the preview's square outline at the same time. Preview chrome
  styling should not be able to reach into components; consider scoping all
  docs-site element selectors, not just this one.

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

