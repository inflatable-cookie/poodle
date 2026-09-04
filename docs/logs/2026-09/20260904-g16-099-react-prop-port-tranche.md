# g16.099 — React Prop Port Tranche

Status: complete
Date: 2026-09-04
Card: `docs/roadmaps/g16/099-react-prop-port-tranche.md`
Handoff: `docs/handoffs/20260904-160000-g16-099-react-prop-port-tranche.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/roadmaps/g16/095-react-prop-drift-gate.md`
Branch: `feature/g16-099-react-prop-port-tranche`
Base: `origin/main` at `7022534e7cb8aa32a0c4767222da89f21f1d1f04`

## Outcome

Ported every pending-port entry from the Svelte↔React public prop drift baseline
to the corresponding React shells, matched static defaults, updated the
contract runtime note for `AppHeader` element access, added focused unit tests,
and cleared the corresponding `pending-port` entries from `BASELINE` in
`packages/svelte/preview/scripts/react-prop-drift.ts`.

### Ports Completed

1. **Button (`button`):**
   - Ported `formEncType` (`"application/x-www-form-urlencoded" | "multipart/form-data" | "text/plain" | null`), default `null`.
   - Ported `formMethod` (`"get" | "post" | "dialog" | null`), default `null`.
   - Ported `style` (`CSSProperties | null`), default `null`. Composed with `maxWidth`.
   - Added unit tests in `packages/react/components/test/Button.test.tsx` verifying DOM attributes and style composition.

2. **Calendar (`calendar`):**
   - Ported `today` (`string | null`), default `null`.
   - Forwarded `today ?? todayIsoDate()` to `buildCalendarWeeks`.
   - Added unit test in `packages/react/components/test/Calendar.test.tsx` verifying deterministic today pinning via `data-today`.

3. **SplitView (`split-view`):**
   - Ported `divider` (`boolean`), default `false`.
   - Emits `data-divider={divider ? "line" : undefined}` on the root element.
   - Added unit tests in `packages/react/components/test/SplitView.test.tsx` verifying `data-divider` output.

4. **AppHeader (`app-header`):**
   - Ported `element` (`((element: HTMLElement | null) => void) | Ref<HTMLElement> | null`), default `null`.
   - Dispatches the raw `<header>` DOM element to both the forwarded `ref` and the `element` callback/ref prop.
   - Updated runtime notes in `docs/contracts/components/app-header.md`.
   - Added unit tests in `packages/react/components/test/AppHeader.test.tsx` verifying both callback ref and ref object pass-through.

5. **DockRegion (`dock-region`):**
   - Ported `showCollapseToggle` (`boolean`), default `true`.
   - Guarded all 4 collapse-toggle render call sites with `collapsible && showCollapseToggle`.
   - Added unit tests in `packages/react/components/test/DockRegionTabPassThroughs.test.tsx` verifying toggle suppression across expanded, icon-strip, and hidden modes.

6. **Baseline Shrinkage (`react-prop-drift.ts`):**
   - Removed `button`, `calendar`, `split-view`, `app-header`, and the `pending-port` entry of `dock-region` from `BASELINE`.
   - Kept all 13 `framework-idiom` entries and 3 `needs-decision` entries intact.
   - Baseline shrank from 20 accepted-delta components to 16.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Port is real | revert one ported prop (`Calendar.today`) | `effigy docs:react-prop-drift` exits 1 naming `missing from React (Svelte-only): today` |
| Baseline shrank | keep cleared `pending-port` entries in `BASELINE` after porting | `effigy docs:react-prop-drift` exits 1 reporting `baselined Svelte prop(s) no longer drift (delete from BASELINE)` across all 5 components |
| Defaults match | plant default drift (`DockRegion.showCollapseToggle = false`) | `effigy docs:react-prop-drift` exits 1 reporting `default drift on "showCollapseToggle": Svelte="true" vs React="false"` |
| Svelte untouched | diff under `packages/svelte/` | `git diff packages/svelte/` shows only the 5 deleted `pending-port` entries in `react-prop-drift.ts` |

## Validation

- **Focused React Tests:**
  `bun run vitest run packages/react/components/test/{Button,Calendar,SplitView,AppHeader,DockRegionTabPassThroughs}.test.tsx`
  Result: 5 test files passed, 38 tests passed.
- **Drift Gate:**
  `effigy docs:react-prop-drift`
  Result: checked 176, skipped 0, OK (16 components with accepted deltas).
- **Docs Check:**
  `effigy docs:check`
  Result: all linting, audits, contract drift, react prop drift, and doc builds passed cleanly (exit code 0).
- **Web CI:**
  `effigy ci:web`
  Result: full web CI sequence completed cleanly (1265 unit tests, 51 distribution tests, Svelte/React preview builds, package installs; exit code 0).
- **Clean Diff:**
  `git diff --check origin/main...HEAD`
  Result: clean, zero whitespace errors or merge markers.

## Limits

- No Svelte component was edited.
- Svelte-inclusion candidates (`Tree.onEditingChange`, `OrderBy.onActiveSortChange`) and spec-pending `DockRegion.showTabs` were left untouched in `needs-decision`.
- Framework-idiom entries were untouched.
- No workflows, release surfaces, or publication metadata modified.
- Reserved coordinator paths (`docs/roadmaps/g16/README.md`, `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`) were untouched.
