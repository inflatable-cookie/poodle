# g16.095 — Svelte↔React Public Prop Drift Gate

Status: complete — awaiting orchestrator review
Date: 2026-09-04
Card: `docs/roadmaps/g16/095-react-prop-drift-gate.md`
Handoff: `docs/handoffs/20260904-132736-g16-095-react-prop-drift-gate.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/architecture/001-poodle-system-shape.md`,
`docs/architecture/006-headless-core-and-machine-model.md`
Branch: `feature/g16-095-react-prop-drift-gate`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-095-react-prop-drift-gate`
Base: `origin/main` at `3fe9c052767e87fe22b53369e541606e3d1e434d`

## Outcome

Delivered the Svelte↔React public prop drift gate script
(`packages/svelte/preview/scripts/react-prop-drift.ts`), biting counterexample
tests in `packages/svelte/preview/test/react-prop-drift.test.ts`, and wired the
new standalone selector `docs:react-prop-drift` into `tasks/effigy.tasks.toml`
and the `docs:check` sequence.

The gate parses Svelte `$props()` destructuring and React `interface
<Name>Props` (or exported prop types), canonicalizes framework idioms and DOM
attribute casing, excludes snippets/children/slots, and compares public prop
surfaces and static literal defaults across 176 catalogue components.

As specified by the card, the gate fails honestly without altering component
APIs or silently baselining missing ports. The baseline register requires
a non-empty reason string for every entry and is seeded only with deltas
requiring an architectural decision (`dock-region.showTabs`, awaiting
DockRegionSpec tab strip modeling in g13.014).

Across the catalogue, 176 components were checked (0 skipped), revealing 29
un-baselined drift issues across 20 components:
- 7 Svelte props missing from React (5 components)
- 22 React-only props (15 components)
- 0 conflicting static literal defaults

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
| Baseline is reasoned | add baseline entry without reason string | `validateBaseline` throws error on load |
| Board integration | run `effigy docs:check` on planted drift | board fails at `docs:react-prop-drift` (exit 1) |

All 8 unit tests in `react-prop-drift.test.ts` pass, proving every oracle
invariant.

## Grouped Findings

### 1. Port to React (Svelte props absent from React)

Props present in Svelte and documented in component contracts that the React
shells have not yet ported:

- **Button (`button`):** `formenctype`, `formmethod`, `style` — HTML button form submission attributes and inline style pass-through.
- **Calendar (`calendar`):** `today` — ISO date string override for highlighting current date.
- **SplitView (`split-view`):** `divider` — boolean flag to control divider visibility.
- **AppHeader (`app-header`):** `element` — Svelte binds `element?: HTMLElement | null`; React lacks DOM element exposure.
- **DockRegion (`dock-region`):** `showCollapseToggle` — control strip toggle visibility. (`showTabs` is baselined awaiting g13.014).

### 2. Candidate for Svelte Inclusion (React-only props)

Props added in React shells that provide uncontrolled defaults or event
notifications not declared in Svelte:

- **Uncontrolled input defaults:**
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
- **Component callbacks:**
  - `Tree` (`tree`): `onEditingChange`
  - `OrderBy` (`order-by`): `onActiveSortChange`

### 3. Framework Idiom

- **Uncontrolled props:** React component convention expects `defaultValue` /
  `default*` for uncontrolled initialization. Svelte 5 runes handle this via
  `$bindable(initialValue)` destructuring bindings. If contracts formalize
  uncontrolled initialization, Svelte components can support them as named
  props.
- **DOM element binding:** Svelte binds DOM elements using `element = $bindable(null)`
  props (e.g. `AppHeader`). React idiomatic access uses `ref` / `forwardRef`.

### 4. Needs Decision (Escalation to Planning / Chatterbox)

- **Uncontrolled defaults strategy:** Should `defaultValue` and related
  `default*` props be promoted into component contracts across all targets,
  or should React remove them to match Svelte's current contracted surface?
- **ToastStack callback signature:** In `ToastStack`, `onDismiss` exists on
  both Svelte and React (so the prop name matches), but Svelte uses
  `(id: string) => void` while React uses `(item: ToastItem) => void`. Needs a
  contract resolution for callback signature consistency.
- **DockRegion spec alignment:** `dock-region`'s `showTabs` remains baselined
  pending g13.014 tab strip modeling in `poodle_specs`.

## Validation

- `effigy docs:react-prop-drift`: fails with exit code 1 as required, reporting
  29 drift issues across 20 components with grouped findings.
- `DRIFT_REPORT=1 effigy docs:react-prop-drift`: passes with exit code 0 and
  prints complete diagnostic report.
- `bunx vitest run packages/svelte/preview/test/react-prop-drift.test.ts`: 8/8
  passed.
- `bunx vitest run --project svelte-preview`: 10 files / 49 tests passed.
- `effigy docs:check`: runs through lint, then fails at `docs:react-prop-drift`
  (exit code 1) as required by the card.
- `effigy ci:web`: 1265 core unit tests passed, 51 distribution tests passed,
  Svelte/React preview builds passed, exit code 0.
- `git diff --check origin/main...HEAD`: clean, no trailing whitespace or
  merge markers.

## Limits

- No component API, default, or contract was changed to artificially silence
  drift.
- No missing port was baselined.
- React publication metadata and workflows were not modified.
- Review dispatch, merge, and coordinator closeout files (`docs/roadmaps/g16/README.md`,
  `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`) are reserved
  for the orchestrator.

## Continuation

The grouped findings are returned to Chatterbox and the Northstar orchestrator
to schedule:
1. A "port to React" tranche for missing ports (`Button`, `Calendar`,
   `SplitView`, `AppHeader`, `DockRegion`).
2. An architectural/contract decision regarding uncontrolled `defaultValue`
   props in React vs Svelte.
