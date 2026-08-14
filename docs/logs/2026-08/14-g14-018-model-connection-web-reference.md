# 14 — g14.018 Model Connection Web Reference (batch log)

Branch: `t3code/model-connection-components`
Date: 2026-08-14
Base SHA: `d93f26e6`
Card: `docs/roadmaps/g14/018-model-connection-web-reference.md`
Contracts: `model-connection-picker.md`, `model-connection-setup.md`,
`model-connection-card.md`, `model-catalogue-editor.md`

Svelte and React reference implementations for choosing, configuring,
inspecting, enabling, and curating model connections. No Rust, no native, no
conformance — that stays g14.020 after the conformance pilot.

## Files

### Batch A — shared core

| File | What |
|---|---|
| `packages/core/src/model-connection.ts` | new — display types, picker filter/group, setup transition, catalogue order/visibility helpers, fixtures |
| `packages/core/src/styles/model-connection.css` | new — one stylesheet, four blocks |
| `packages/core/src/index.ts` | exports added |
| `packages/core/test/model-connection.test.ts` | new — 19 tests |

### Batch B — web components

| File | What |
|---|---|
| `packages/svelte/components/src/ModelConnection{Picker,Setup,Card}.svelte` | new |
| `packages/svelte/components/src/ModelCatalogueEditor.svelte` | new |
| `packages/react/components/src/ModelConnection{Picker,Setup,Card}.tsx` | new |
| `packages/react/components/src/ModelCatalogueEditor.tsx` | new |
| both `index.ts` files | 4 exports each |

### Batch C — specimens, tests, registration

| File | What |
|---|---|
| specimen pages ×4 per runtime | new |
| component tests (Svelte + React) | 33 focused tests |
| registries / docs / parity / audit | 207 → 211 |
| `test/fixtures/component-props.ts` | 4 fixtures (inlined; no package import) |
| pack-install fixture | imports + mounts all four in both runtimes |
| `PAPERCUTS.md` | two append-only entries |

## Shared core API

| Helper | Role |
|---|---|
| `filterModelConnectionOptions` / `groupModelConnectionOptions` | case-folded filter; host order retained |
| `modelConnectionOptionSelectable` | available ∧ ¬disabled |
| `resolveModelConnectionPickerShellState` | ready/loading/error/empty/no-results → PickerShell |
| `modelConnectionSetupTransition` (+ canContinue / canSubmit) | choose→configure guards; exact-id submit |
| `shownModelCatalogueItems` / `hiddenModelCatalogueItems` | partition; hidden order meaningless |
| `requestModelCatalogueOrder` | complete shown-id order via `applyReorder` |
| `requestModelCatalogueVisibility` | `{ id, visible }` only |
| `modelCatalogueFocusAfterHide` / announcements / tones | a11y + status mapping |
| `MODEL_CONNECTION_*_FIXTURES` / `MODEL_CATALOGUE_FIXTURES` | inert specimen data |

## Public component API (Svelte ≡ React)

`ModelConnectionPicker` — `options`, controlled/uncontrolled `value` + `query`,
`state`, shell copy, `isDisabled`, `variant`, `leading` / `footer`,
`onValueChange` / `onQueryChange`.

`ModelConnectionSetup` — controlled/uncontrolled `stage` + `value` + `query`,
picker forwards, `canSubmit` / `isPending` / feedback copy, action labels,
`leading` / `configuration` / `configureAside`, stage/value/query/submit/cancel
callbacks.

`ModelConnectionCard` — required `id` / `title` / `providerLabel`, optional
route/version/access/readiness, controlled/uncontrolled `open`, host-owned
`isEnabled`, independent disclosure + Switch, `leading` / `badges` /
`closedAccessory` / `actions` / `details`.

`ModelCatalogueEditor` — controlled `items` + `state`, reorder (pointer,
keyboard grab, explicit move), hide/restore, optional `onInfo` /
`customAction` / `leading` / `rowMeta`.

## Orchestrator acceptance fixes

- Added explicit loading, error, empty, and no-results picker copy.
- Kept a selectable picker route in the tab order when the controlled value is
  filtered out or unavailable; added the required checked indicator.
- Made setup-stage focus follow controlled as well as local transitions.
- Made card disclosure IDs instance-safe and restored trigger focus after a
  controlled external close.
- Tracked catalogue keyboard grabs by stable item ID rather than array index.
- Restricted pointer drag initiation to the reorder handle and exposed its
  pressed state.
- Rendered catalogue failures with the required danger posture.
- Added regressions for each correction in both web runtimes where applicable.

## Acceptance → test map

| Criterion | Where |
|---|---|
| deterministic filter + source order | core filter tests; picker suites |
| exact available id selection | picker suites (legacy/unsupported inert) |
| loading/error/empty/no-results postures distinct | core + picker suites |
| continue/submit guards + pending lock | setup transition core + setup suites |
| disclosure ⊥ enable | card suites |
| disable leaves readiness/access copy | card svelte suite |
| complete shown-id order | catalogue suites + core |
| visibility `{ id, visible }` only | catalogue suites + core |
| catalogue postures distinct | catalogue suites |
| Svelte/React agreement | `test:parity` (174), shared CSS, mirrored props |
| packed public imports | `test:web-pack-install` mounts all four ×2 |
| g14.020 remainder still open | this log; contracts Known Deltas; no roadmap edit |

## Validation

| Selector | Result |
|---|---|
| `packages/core/test/model-connection.test.ts` | pass (19) |
| focused component vitest | pass (33) |
| `effigy ci:web` | pass (98 files, 1,299 tests; packed consumer 7/7) |
| `effigy docs:check` | pass |
| `effigy test:parity` | pass (174) |
| `effigy test:a11y` | pass (176) |
| `effigy svelte:surface-audit` | pass (174/174) |
| `effigy docs:callback-drift` | pass |
| `effigy docs:react-specimen-drift` | pass |
| `effigy test:web-pack-install` | pass |
| `effigy docs:contract-drift` | pre-existing: Button `children/leading/trailing` |
| `effigy check:svelte` | pass (0 errors; 4 existing warnings) |
| `effigy test:core` | pass (722) |
| `git diff --check` | clean |

## Screenshots

Specimen routes (both previews):

- `#components/model-connection-picker`
- `#components/model-connection-setup`
- `#components/model-connection-card`
- `#components/model-catalogue-editor`

Operator capture for the PR handoff; not committed in this batch.

## g14.020 remainder

- shared executable conformance cases replacing temporary framework specimen wrappers
- Rust declarations + renderer/node support
- GPUI execution
- Jetstream still deferred under current g14 backend policy
- do not touch `packages/core/src/conformance/**` from this lane
