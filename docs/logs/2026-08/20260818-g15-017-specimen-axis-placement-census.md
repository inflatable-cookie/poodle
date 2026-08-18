# g15.017 web specimen axis placement — batch log (3/3: census)

Date: 2026-08-18
Branch: `t3code/web-specimen-axis-placement`
Card: `docs/roadmaps/g15/017-specimen-axis-placement.md`
Commits: `ce5ef599` (helpers + SceneSpecimen + Avatar codegen) · `a69c0b82` (24 paired routes + removals) · this batch (census)

## Scope delivered (this batch)

The acceptance evidence for the card: a 175-route census proving every catalogue
page shows `Sizes`/`Densities` exactly when the component takes the prop, in
both web runtimes, with every visible axis tab populated.

- `test/parity/specimen-axis-census.test.tsx` — the census.
- `packages/svelte/preview/test/AxisHelper{NoRenderers,SizesOnly,DensitiesOnly,HiddenRenderers}.svelte` — SpecimenLayout hardening fixtures.
- `packages/svelte/preview/test/AxisSceneFixture.svelte` — SceneSpecimen slug wiring fixture.

## What the census asserts

1. **Helper hardening** — SpecimenLayout shows only Examples with no axis
   renderer; exactly the matching tab for a supplied sizes/densities renderer;
   `showSizes={false}`/`showDensities={false}` hides a supplied renderer. Both
   runtimes, including pane-content evidence.
2. **Authored scene projection** — Avatar's generated scene is size-only
   (`tabs: ["examples","sizes"]`, `densityAxis: []`); SceneSpecimen renders only
   the declared tabs; the callout scene keeps all three axes.
3. **Per-route decision evidence** — the exact 24 corrected routes show
   Examples + eligible axes, paired and populated, in both runtimes.
4. **PR #38 validation** — the twelve audio routes keep paired, populated axis
   tabs outside Examples.
5. **Aggregate sweep** — all 175 routes: tab set equals eligibility-derived
   contract, Svelte/React agree, axis panes render content. Eligibility is read
   from each component's own source props, never from habit.

## Findings

- **9 pre-existing demo pages** (`time-ago`, `box`, `grid`, `list-grid`,
  `region`, `resize-handle`, `scroll-shell`, `separator`, `spacer`) render no
  SpecimenLayout at all — no tabs, consistently across both runtimes. The
  census treats a layout-less page as a legitimate no-tab state (the audit's
  "all 175 pages have tabs" is oversimplified); runtime agreement is still
  asserted. Out of g15.017 scope; no changes made.
- **`toolbar` and `error-boundary` specimens log by design** (icons absent from
  the fixture set; a demonstrated caught throw). Covered by a tolerant test
  that detaches the suite's `console.error` guard for those two routes only —
  their tab contract is still asserted. Pre-existing; out of scope.
- **`xy-pad` component source is `XYPad.svelte`** — eligibility lookup is
  case-insensitive so the census works on case-sensitive filesystems (macOS's
  default case-insensitive APFS masked this during the audit's static pass).
- **React tabs machine flushes its transition in a microtask** — census clicks
  wrap in async `act` so the pane re-renders inside act, keeping the suite's
  `console.error` guard clean.

## Validation run

- `effigy test:parity` — 4 files, 247 passed.
- `effigy ci:web` — 341 files, 2698 passed (+ 5 files, 11 passed).
- `effigy check:svelte-preview` — 0 errors, 13 warnings (main baseline).
- `effigy react:build` — pass.
- `effigy ir:check` — generated artifacts current.
- `effigy catalogue:check` — pass.
- `effigy docs:check` — pass.
- `git diff --check` — clean.

## Handoff notes for the orchestrator

Census covers the full 175-route denominator (174 catalogue entries +
web-only `MeterSurface`). The 24 corrected routes are exercised individually;
everything else is swept in the aggregate loop. All route changes from commits
1–2 are untouched by this batch.

## Review response (2026-08-18) — PR #39

### Blocker 1 — overlay axes mounted every modal at once
The Dialog/Drawer/AlertDialog/FormDialog axis renderers returned already-open
overlays, stacking five/three modal portals and leaving body scroll-locked
(each instance saved/restored the same `body.style.overflow`). Reworked all
four routes in both runtimes to one-open-at-a-time triggers: each step renders
a `Open {size} dialog` / `Open {size} drawer` button plus a controlled overlay
(`axisOpen` state map), so the pane mounts triggers only and a step click opens
exactly one overlay. FormDialog axis field ids are now per-step
(`form-dialog-axis-name-{size|density}`), removing the duplicate-id mount.
ConfirmAction already rendered a trigger and was left as-is.

### Blocker 2 — SceneSpecimen retained a tab the next scene dropped
`SpecimenLayout` kept `activeTab` when the tab set shrank (Callout → Densities
→ Avatar left the pane blank). Both runtimes now normalize an invalid active
tab back to `examples` when the available tab set changes (React `useEffect`
keyed on the tab-value set; Svelte `$effect` over a `$derived` tab-value list).
Verified with a paired regression that reuses one SceneSpecimen instance
(`AxisSceneSwitch` fixture for Svelte, `rerender` for React).

### Census hardening
- Overlay lifecycle regressions: for dialog/alert-dialog/form-dialog/drawer in
  both runtimes — pane mounts with zero open overlays, one step click opens
  exactly one overlay with content, backdrop closes it, and body scroll is
  restored on tab exit and on unmount.
- The sweep now derives expectations from component props independently of page
  shape: a layout-less page whose component takes `size`/`density` fails until
  it exposes the axis. The nine layout-less demo pages (`time-ago`, `box`,
  `grid`, `list-grid`, `region`, `resize-handle`, `scroll-shell`, `separator`,
  `spacer`) take no axis props, so they still pass with no tabs.

### Test-infrastructure notes
- Svelte 5 runes components no longer expose `$set` (`component_api_changed`);
  the scene-navigation regression uses a switch fixture that patches the same
  SceneSpecimen instance via its own `$state`.
- Svelte fixtures must use `onclick` (lowercase): camelCase `onClick` handlers
  do not fire under `fireEvent.click` in this Svelte/happy-dom setup.
- Drawer's Web Animations API usage (`element.animate`) is polyfilled in the
  census file, mirroring the existing Drawer component test.

### Validation after review response
- `effigy test:parity` — 4 files, 252 passed (was 247; +5 new regressions).
- `effigy ci:web` — 341 files, 2703 passed (+ 5 files, 11 passed).
- `effigy check:svelte-preview` — 0 errors, 13 warnings (baseline).
- `effigy react:build`, `effigy ir:check`, `effigy catalogue:check` — pass.
- `git diff --check` — clean.