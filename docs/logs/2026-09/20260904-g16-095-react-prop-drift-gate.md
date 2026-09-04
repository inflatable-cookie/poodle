# g16.095 — Svelte↔React Public Prop Drift Gate

Status: complete — revision after Review Oracle review of PR #202
Date: 2026-09-04
Card: `docs/roadmaps/g16/095-react-prop-drift-gate.md`
Handoff: `docs/handoffs/20260904-132736-g16-095-react-prop-drift-gate.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/architecture/001-poodle-system-shape.md`,
`docs/architecture/006-headless-core-and-machine-model.md`
Branch: `feature/g16-095-react-prop-drift-gate`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-095-react-prop-drift-gate`
Base: `origin/main` at `cecc42b2d71b48b1bfb9aa0f3e6a64db9f9ca545`

## Outcome

Delivered the revised Svelte↔React public prop drift gate script
(`packages/svelte/preview/scripts/react-prop-drift.ts`), biting counterexample
and ratchet tests in `packages/svelte/preview/test/react-prop-drift.test.ts`,
and kept the standalone selector `docs:react-prop-drift` in `tasks/effigy.tasks.toml`
and the `docs:check` sequence.

Per operator decision 2026-09-04 following Review Oracle inspection:
1. All 29 findings (+ `dock-region.showTabs`) across 20 components are seeded
   into a kind-tagged `BASELINE` register:
   - `pending-port` (5 entries: Button, Calendar, SplitView, AppHeader, DockRegion.showCollapseToggle)
     citing `g16.099`.
   - `framework-idiom` (13 entries) capturing React uncontrolled `default*`
     initializers and change callbacks mirroring Svelte `$bindable` state.
   - `needs-decision` (3 entries: Tree.onEditingChange, OrderBy.onActiveSortChange,
     DockRegion.showTabs) citing owning cards or Chatterbox contract decisions.
2. The baseline is an active ratchet:
   - `validateBaseline` enforces valid kind and non-empty reason; `pending-port`
     entries must cite a card (`g16.099`).
   - Liveness checking detects stale baseline entries: if any baselined prop
     ceases to drift (e.g., when ported in `g16.099`), the gate fails and
     requires deleting the baseline entry.
   - Missing React shells are failed immediately (`skipped > 0`), ensuring no
     unimplemented component escapes drift gating.
3. Reviewer non-blocking notes addressed:
   - Parser unit test coverage added in `react-prop-drift.test.ts` for both Svelte
     runes and React TypeScript interfaces/components.
   - `AudioPresentationProps` is parsed dynamically from `audio/useAudioPresentation.ts`
     rather than hardcoded.
   - Missing React shells are failed rather than silently skipped.
   - Boundary deviation documented: `parseSvelteProps` includes `on*` callbacks
     because Svelte↔React public prop parity requires name-level callback
     comparison (normalization rule 5), whereas `contract-prop-drift.ts`'s
     `svelteProps` deliberately omits callbacks.

## Normalization Rules

Documented in the script header and verified by unit tests:
1. **DOM attribute casing:** React camelCase DOM attributes (`autoComplete`,
   `spellCheck`, `autoCapitalize`, `autoCorrect`, `formAction`,
   `formNoValidate`, `formTarget`, `tabIndex`, `readOnly`, `colSpan`,
   `rowSpan`) map to Svelte lowercase attributes (`autocomplete`,
   `spellcheck`, etc.).
2. **Class attribute:** Svelte `class` and React `className` map to `className`.
3. **For attribute:** Svelte `for` and React `htmlFor` map to `htmlFor`.
4. **Slots and snippets:** Svelte `Snippet`-typed props and React `children` /
   `render` props are excluded as slot plumbing.
5. **Callback signatures:** `on*` callbacks are compared by prop name only;
   callback parameter signatures and arities are framework-idiomatic.
6. **Rest props and index signatures:** `...restProps` and `[key: string]`
   index signatures are excluded.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| React-only prop fails | plant `defaultValue?: number` on React shell whose contract lacks it | `compareComponentProps` returns finding naming component and prop; standalone selector exits 1 |
| Missing React port fails | remove documented Svelte prop (`formenctype`) from React shell | `compareComponentProps` returns finding naming component and prop; standalone selector exits 1 |
| Attribute casing is not drift | `autocomplete` (Svelte) vs `autoComplete` (React) | `canonicalizePropName` normalizes both; no finding |
| Snippets and children are not drift | Svelte `children: Snippet` vs React `children: ReactNode` | both excluded; no finding |
| Baseline is reasoned | add baseline entry without reason string or kind | `validateBaseline` throws error on load |
| Ratchet holds | add a `pending-port` entry whose reason names no card | `validateBaseline` throws error naming card requirement |
| Ratchet shrinks | a baselined prop no longer drifts in source | gate exits 1 reporting stale baseline prop to delete |
| Main is green | run `effigy docs:check` on the PR head against current main | passes cleanly (exit 0) |
| Board integration | run `effigy docs:check` on planted drift | board fails at `docs:react-prop-drift` (exit 1) |

All 14 unit tests in `react-prop-drift.test.ts` pass, proving every oracle
invariant.

## Grouped Findings (Seeded in BASELINE)

### 1. Port to React (`kind: "pending-port"`, cleared by `g16.099`)

Props present in Svelte and documented in component contracts that the React
shells have not yet ported:

- **Button (`button`):** `formenctype`, `formmethod`, `style`
- **Calendar (`calendar`):** `today`
- **SplitView (`split-view`):** `divider`
- **AppHeader (`app-header`):** `element` (forwarded ref or elementRef callback)
- **DockRegion (`dock-region`):** `showCollapseToggle`

### 2. Framework Idiom (`kind: "framework-idiom"`)

React uncontrolled `default*` initializers and change callbacks mirroring Svelte
`$bindable` state initial values:

- `TriStateSwitch` (`tri-state-switch`): `defaultValue`
- `EmbedInput` (`embed-input`): `defaultValue`
- `FileUpload` (`file-upload`): `defaultFiles`
- `RangeSlider` (`range-slider`): `defaultValue`
- `Slider` (`slider`): `defaultValue`
- `TokenInput` (`token-input`): `defaultValues`
- `DurationInput` (`duration-input`): `defaultHours`, `defaultMinutes`, `defaultSeconds`
- `SidebarNav` (`sidebar-nav`): `defaultValue`
- `FilterToolbar` (`filter-toolbar`): `defaultCollapsed`, `onCollapsedChange`
- `LogList` (`log-list`): `defaultFilterLevel`, `defaultFilterText`, `onFilterLevelChange`, `onFilterTextChange`
- `RelationPicker` (`relation-picker`): `defaultQuery`, `defaultSelectedIds`
- `ActionDiscoveryPanel` (`action-discovery-panel`): `defaultActiveId`
- `SettingsShell` (`settings-shell`): `defaultSearchQuery`

### 3. Needs Decision (`kind: "needs-decision"`)

Divergences requiring an architectural/contract decision by Chatterbox:

- `Tree` (`tree`): `onEditingChange` (candidate for Svelte inclusion)
- `OrderBy` (`order-by`): `onActiveSortChange` (candidate for Svelte inclusion)
- `DockRegion` (`dock-region`): `showTabs` (spec-surface-pending in g13.014)

## Validation

- `effigy docs:react-prop-drift`: passes with exit code 0 (`176 checked, 20 components with accepted deltas`).
- `bunx vitest run packages/svelte/preview/test/react-prop-drift.test.ts`: 14/14 passed.
- `bunx vitest run --project svelte-preview`: 10 files / 55 tests passed.
- `effigy docs:check`: passes cleanly with exit code 0.
- `effigy ci:web`: 1265 core unit tests passed, 51 distribution tests passed, 381 preview tests passed, Svelte/React preview builds passed, pack-install passed, exit code 0.
- `git diff --check origin/main...HEAD`: clean, no trailing whitespace or merge markers.

## Limits

- No component API, default, or contract was changed.
- `g16.099` was not started.
- React publication metadata and workflows were not modified.
- Reserved shared closeout surfaces are untouched.
