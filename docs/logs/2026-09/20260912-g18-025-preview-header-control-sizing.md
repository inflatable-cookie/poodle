# g18.025 — Preview header control sizing

Status: in review — opened from `ns-92be02b9-f520-4b85-9baa-794a7464605c` for exact-head independent review
Date: 2026-09-12
Branch: `ns-92be02b9-f520-4b85-9baa-794a7464605c`
Card: `docs/roadmaps/g18/025-preview-header-control-sizing.md`
Handoff: `docs/handoffs/20260912-g18-025-preview-header-control-sizing.md`
Governing refs: `docs/contracts/components/toggle-group.md`,
`packages/svelte/preview/src/components/DisplayControls.svelte`,
`packages/react/preview/src/gallery/DisplayControls.tsx`
Base: `origin/main` at `aa51e200889d067cb0ccc6a40f7cf7b57b663269`

## Outcome

Both preview headers are fixed `md` chrome. `DisplayControls` re-scopes its
subtree through the existing public `UiPresentationProvider` with
`sizeScale="md"` while passing the ambient `density` through, so the five
painted controls (ThemeSelect, both ToggleGroups, the block Slider, the search
TextInput) resolve `md` at every specimen Size stop and the header no longer
resizes when the operator selects `xs`–`xl`. Density keeps following the
Density axis; values, generated-shell ownership, wrapping, responsive search
growth, keyboard behavior, and accessibility are unchanged. No reusable
component API or internal changed, and no preview-only CSS heights were added.

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
