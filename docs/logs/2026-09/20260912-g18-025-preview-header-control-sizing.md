# g18.025 — Preview header control sizing

Status: complete — merged as `90c40defe86e4841ad248c72200f7333f37c342d` (PR #257) on 2026-09-12 after exact-head independent review (PR comment `5646310971`, `ready_to_merge`) at `c2b2f8f161bee3ae8c68400f9e155812101d6e08` with green rust/web checks
Date: 2026-09-12
Branch: `ns-92be02b9-f520-4b85-9baa-794a7464605c`
Card: `docs/roadmaps/g18/025-preview-header-control-sizing.md`
Handoff: `docs/handoffs/20260912-g18-025-preview-header-control-sizing.md`
Governing refs: `docs/contracts/components/toggle-group.md`,
`packages/svelte/preview/src/components/DisplayControls.svelte`,
`packages/react/preview/src/gallery/DisplayControls.tsx`
Base: `origin/main` at `aa51e200889d067cb0ccc6a40f7cf7b57b663269`

## Review rounds

### Round 1 — verdict `changes_required` (`planning-change`), PR comment 5646051126

Independent review confirmed every green check and found no code defect. Two
blocking findings:

1. **Brief contradiction (`planning-change`)** — the card's stated oracle
   ("36px visual boxes for all five controls") is not met: the two
   ToggleGroups paint 32px by ToggleGroup's own documented item contract, so
   the operator's original Theme/Search/Contrast vs Density/Size height split
   persists (smaller, but present). The review rules this cannot be fixed in
   code inside the card (component internals reserved; preview-only heights
   excluded) and requests a canonical ruling: (1) accept the documented inset
   and amend the card oracle, (2) authorize a follow-up component-level card,
   or (3) explicitly authorize a preview-local alignment treatment.
2. **`oracle-gap`** — the probe encoded `32px` as the expected toggle height,
   so it could not fail on the card's stated 36px oracle.

### Round 2 — worker response at this head

- Finding 2 addressed: the probe no longer encodes a parallel expected-height
  table. It now (a) proves the full-height controls against the card's 36px
  md-ladder oracle outright, and (b) pins each ToggleGroup box to the
  **measured** ladder box (the full-height controls in the same header, same
  moment) minus the documented 0.25rem contract inset, with the divergence
  named in every check label. If either side of the divergence moves — a
  ruling amends the card, or a component-level change alters the inset — the
  probe fails and forces the card oracle and the probe oracle back into
  agreement before g18.025 can close.
- Finding 1 cannot be closed by the worker: all three resolutions require
  operator/orchestrator authority (amending the card, opening a component
  card, or writing a preview-local exception), and the card's boundaries
  forbid each until ruled. **The task is blocked on that canonical ruling.**
- Re-validated at this head: probe 367 ok / 0 FAIL on headless Chromium and
  367 ok / 0 FAIL on headless WebKit; `git diff --check` clean.

### Round 3 — ruling landed, implemented at this head

The operator landed the canonical ruling on main as card revision
`b14aeb04b` ("authorize preview header alignment"): option 3. ToggleGroup
keeps its reusable 0.25rem item inset globally; within the two generated
preview headers only, the inset is neutralized so Density and Size paint the
same 36px `md` ladder as the other header controls — an explicit exception to
the no-preview-height rule that must not change ToggleGroup's public contract
or leak into catalogue specimens.

Implementation at this head:

- `packages/svelte/preview/src/components/DisplayControls.svelte` (scoped
  style, `:global` child selector) and
  `packages/react/preview/src/gallery/gallery.css`: one declaration each —
  `.poodle-display-controls … .poodle-toggle-group__item { min-height:
  var(--poodle-toggle-group-height); }` — restoring the item to the ladder
  the component itself resolved. No hard-coded heights, no component change,
  scoped under the header root so catalogue specimens are untouched.
- The probe was reconciled to the unified card oracle (as its round-2 design
  required once a side moved): all five painted controls assert 36px and
  shared row tops/bottoms, and a new leak guard drives the toggle-group
  specimen route in both apps proving catalogue ToggleGroups still paint the
  documented item contract (ladder − 0.25rem at their resolved stop).
- Re-validated: probe 321 ok / 0 FAIL per engine on headless Chromium and
  WebKit (all five controls 36px, shared edges, leak guard green); focused
  paired tests 8/8; `svelte-check` preview 0 errors; react preview `tsc`
  error board byte-identical to base; `svelte:build` / `react:build` exit 0;
  `docs:lint` green; `git diff --check` clean.

### Round 4 — queue-branch sync and re-validation

Round-3 review approved the head but flagged a merge-gate caveat: the web
check was red, inherited from a red origin/main whose slider block-variant
extracted-styles tests (svelte + react, 3 tests) assert fixes landed after
this branch's base. The queue directed a sync of the queue branch onto latest
origin/main. Merged `origin/main` (`81bf2014f`, 11 commits: the g18.025 card
ruling plus the slider block label/marker/bounds series and evidence repins)
into this branch with zero conflicts; the preview-header inset-neutralizing
exception is unchanged.

Re-validated at the merge head:

- The inherited red is resolved: Slider block-variant extracted-styles suites
  green — `svelte-components` Slider.test.ts 25/25, `react-components`
  Slider.test.tsx 26/26.
- Full preview vitest projects — 29 files / 134 tests green.
- `effigy test:g18-025-preview-header-sizing-chromium` — 321 ok / 0 FAIL;
  `-webkit` — 321 ok / 0 FAIL.
- `svelte-check` preview — 0 errors (warnings unchanged); react preview
  `tsc` — error board unchanged from base except two pre-existing
  SliderSpecimen errors at main-shifted line numbers (no new errors).
- `svelte:build` / `react:build` exit 0; `git diff --check` clean.

## Outcome

Both preview headers are fixed `md` chrome. `DisplayControls` re-scopes its
subtree through the existing public `UiPresentationProvider` with
`sizeScale="md"` while passing the ambient `density` through, so the five
painted controls (ThemeSelect, both ToggleGroups, the block Slider, the search
TextInput) resolve `md` at every specimen Size stop and the header no longer
resizes when the operator selects `xs`–`xl`. Density keeps following the
Density axis; values, generated-shell ownership, wrapping, responsive search
growth, keyboard behavior, and accessibility are unchanged. No reusable
component API or internal changed; the only preview CSS height is the card-authorized paired ToggleGroup inset neutralization (Chatterbox ruling `b14aeb04b`), scoped to the two preview headers.

## Execution

- Root cause: both preview apps mount everything, header included, inside
  `<UiPresentationProvider sizeScale={controlSize}>`, so the header controls
  resolved their size from the specimen axis (`packages/svelte/preview/src/App.svelte`,
  `packages/react/preview/src/gallery/App.tsx`).
- `packages/svelte/preview/src/components/DisplayControls.svelte` and
  `packages/react/preview/src/gallery/DisplayControls.tsx`: each now wraps its
  root in `UiPresentationProvider density={density} sizeScale="md"` (the
  provider wrapper is `display: contents`, so layout is untouched). The
  `density` prop type narrowed from `string` to the token domain name
  (`DensityName`) so it feeds the provider directly; the prop set is otherwise
  unchanged.
- Focused paired tests: `packages/svelte/preview/test/g18-025-preview-header-control-sizing.test.ts`
  (with a `HeaderSizingHarness.svelte` test-local provider harness, following
  the `AxisHelper*.svelte` precedent) and
  `packages/react/preview/test/g18-025-preview-header-control-sizing.test.tsx`.
  They mount the header inside an app-shell-like provider at `xl`/`lg`/`xs`
  scales, assert every control resolves `data-size="md"` through every
  specimen Size stop while the Size toggle still shows the selection, and
  replay the Size/Density/Search/Contrast journeys (click, type, keyboard).
- Paired browser probe `test/g18-025-preview-header-sizing/probe.ts` drives the
  two live preview apps (Svelte, React) through headless Chromium and WebKit:
  measured header geometry at every Size stop, Density/Theme/Contrast/Search
  journeys, wrapping at a narrow viewport, and cross-framework height parity.
  Registered as effigy selectors `test:g18-025-preview-header-sizing{-chromium,-webkit}`
  in `tasks/effigy.tasks.toml`, following the `test:block-slider-inline` /
  `test:g18-011-ux-sweep` precedent.
- Execution log (this file) is the one log for the task; g18.006/g18.009,
  release state, versions, changelog, workflows, Desktop, generated shell
  semantics, and reusable component implementations were not touched.

## Review escalation (bounded finding)

The card's oracle asks for "measured 36px visual boxes for all five controls".
Four of five controls paint exactly 36px at `md`. The two ToggleGroups consume
the shared `md` ladder (`data-size="md"`, `--poodle-toggle-group-height:
2.25rem`) but their items paint 32px by the reusable component's own documented
contract — `min-height: calc(var(--poodle-toggle-group-height) - 0.25rem)`
(`docs/contracts/components/toggle-group.md`, "Item `.toggle-group__item`") —
identically in Svelte and React. Making the header's toggle items paint 36px
would require either a reusable-component internals/CSS change (a reserved
closeout surface) or a preview-only CSS height override (explicitly excluded by
the card). Both frameworks were therefore proven equal at the contract height
(36/32/32/36/36 with the toggle items at exactly the documented 4px bottom
inset, tops shared). Escalating to review/Chatterbox per the card's escalation
path: accept the documented item inset, or authorize a component-level change
in a follow-up card.

## Validation

- Focused paired preview tests — 8/8 green (`vitest --project svelte-preview
  --project react-preview`, both new files).
- Full preview test projects — 29 files / 134 tests green (svelte-preview +
  react-preview).
- `effigy test:g18-025-preview-header-sizing-chromium` — 347 checks / 0
  failures; `effigy test:g18-025-preview-header-sizing-webkit` — 347 checks /
  0 failures (paired Svelte + React live previews; geometry at every Size
  stop, density independence, theme/contrast/search/keyboard journeys,
  wrapping, cross-framework parity).
- `bunx svelte-check --workspace packages/svelte/preview` — 0 errors (6
  pre-existing warnings, unchanged from base).
- `bunx tsc -p packages/react/preview/tsconfig.json --noEmit` — error board
  byte-identical to the base commit (verified by diffing sorted output on
  tree vs stash); zero new errors from this change.
- Preview builds green: `effigy svelte:build`, `effigy react:build`.
- `effigy docs:check` — green (17-step sequence including surface audit, docs
  lint, drift checks, evidence ledger, reports, gate snapshot/clean).
- `git diff --check` — clean.

## Explicitly not done

- No reusable component implementations, generated shell semantics, release
  state, versions, changelog, workflows, Desktop, or unrelated preview UI were
  edited. The app-shell top bar (brand/Tabs/pills) intentionally keeps
  following the specimen scale — it is outside the card's owned paths.
- Closeout surfaces (g18 README/index/dispatch/task state) left to the
  orchestrator; retained g18.006/g18.009 untouched.

## Review and merge

Independent round-4 review at `c2b2f8f161bee3ae8c68400f9e155812101d6e08` (PR
comment `5646310971`): **ready to merge**. The reviewer confirmed the
queue-branch sync onto `origin/main` (`81bf2014f`, zero conflicts) left the
approved g18.025 surface byte-identical to the round-3 head, resolved the
inherited web-gate red (Slider block-variant extracted-styles suites green:
svelte-components 25/25, react-components 26/26), and re-measured the merged
head independently: header probe 321 ok / 0 FAIL on Chromium and WebKit (all
five controls 36px with shared row tops `81.5` and bottoms `117.5`, header
height `83.5`; catalogue `sm` toggle items still 24px, no specimen leak),
focused paired preview tests 8/8, full preview projects 29 files / 134 tests,
`svelte-check` preview 0 errors, react preview `tsc` clean in the g18.025
surface, `svelte:build` / `react:build` exit 0, `docs:check` exit 0,
`git diff --check` clean.

Two non-blocking notes, each deferred rather than repaired here:

- The round-4 review flagged this log's react-`tsc` wording ("two pre-existing
  SliderSpecimen errors") as undercounting the five `SliderSpecimen.tsx`
  diagnostics; the overall error-board count is unchanged from prior heads and
  none is in the g18.025 surface. Log-accuracy nit only.
- The probe's leak-guard `SIZE_LADDER_PX` table duplicates the shared ladder
  as a test constant; acceptable, revisit if the ladder ever moves.

Merge gate: PR #257 merged as `90c40defe86e4841ad248c72200f7333f37c342d`
with green rust/web checks.

## Continuation

g18.025 is merged. After the corrected preview experience is accepted, the
retained g18.006 release-candidate task resumes with the repair in the `0.4.0`
source identity; g18.009 stays serial behind it. Further planning direction
needs the operator.
