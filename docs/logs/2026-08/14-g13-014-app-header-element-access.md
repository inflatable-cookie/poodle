---
title: g13 batch 014 — AppHeader element access
status: complete
milestone: side-quest (component API, outside the g13 IR lane)
owner: Poodle core
updated: 2026-08-11
tags: [log, g13, AppHeader, element access, web-only, side-quest]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/014-app-header-element-access.md` on branch
`thread/g13-014-app-header-element-access`: gave `AppHeader` a supported
element escape hatch so a host can attach behaviour (e.g. window dragging) to
the rendered `<header>`, exactly as ruled — element access only, no rest
spread, no action prop, no new named props, no Tauri, no dragging
implementation. Also documented the `--poodle-app-header-min-height` override
and its `0,2,0` specificity trap in the contract.

The card exists because `AppHeader`'s `Props` was closed with no escape hatch:
finch renders `<AppHeader title="Finch" dragRegion ariaLabel="Finch">`, styles
`[data-drag-region="true"]`, but nothing implements the gesture, so its
titlebar does not drag. Longhorn ships the behaviour as a Svelte action
(`windowDrag` from `@inflatable-cookie/longhorn-poodle-svelte`); hosts can now
attach that action to the exposed element.

## Deliverables (only the scoped writes)

- `packages/svelte/components/src/AppHeader.svelte` — bindable `element` prop.
- `packages/react/components/src/AppHeader.tsx` — `ref` forwarded to the
  `<header>` via `forwardRef`.
- `docs/contracts/components/app-header.md` — `### Public Props` heading (was
  `## 3. Props` with no sub-heading, so the drift gates skipped the component;
  it is now checked), the `element` row marked web-only, a new `### Element
  Access` section (intended use, non-goals, GPUI/Jetstream `AC` parity note),
  and a new `### Overriding Header Height` section with the specificity trap
  and a worked example.
- `packages/svelte/preview/scripts/contract-spec-drift.ts` — `element` added to
  `WEB_ONLY_PROPS` (the sanctioned mechanism for web-only props; see below).
- `packages/svelte/preview/src/component-docs.ts` — `app-header` entry:
  `element` prop row and a usage example showing a host attaching behaviour via
  a generic `$effect` (not Tauri-specific).
- `packages/svelte/components/test/AppHeader.svelte.test.ts` +
  `AppHeaderElementHarness.svelte`, `packages/react/components/test/AppHeader.test.tsx`
  — the binding/ref yields the header element; rendered output unchanged.
- `docs/logs/2026-08/14-g13-014-app-header-element-access.md` — this log.
- `PAPERCUTS.md` — one new, non-duplicate friction (see Findings).

## The Svelte / React shapes used

Svelte follows the established `$bindable` idiom (75 components use it; none
previously exposed an element):

```svelte
interface Props {
  /* ... */
  /** Bindable escape hatch: the rendered `<header>` DOM element, so a host
   * can attach behaviour (for example window dragging) to the root. */
  element?: HTMLElement | null;
}

let {
  /* ... */
  element = $bindable<HTMLElement | null>(null),
  /* ... */
}: Props = $props();

<header bind:this={element} class="poodle-app-header" ...>
```

React forwards `ref` to the same `<header>`, following the `MenuSurface`
precedent (`forwardRef`; React 19.1). The card's ruling: `MenuSurface` is the
precedent for **how** to forward, not for **what** to expose — both runtimes
expose the raw DOM element, never a handle object, so there is no
`useImperativeHandle`:

```tsx
export const AppHeader = forwardRef<HTMLElement, AppHeaderProps>(function AppHeader(
  { title = null, /* ... */ },
  ref,
) {
  /* ... */
  return (
    <UiPresentationProvider ...>
      <header ref={ref} className="poodle-app-header" ...>
```

`AppHeaderProps` itself is unchanged: `ref` is React's own mechanism (the
forwardRef second argument), not a member of the props type — the same
treatment text-input.md gives its React ref handle.

## Web-only classification (the drift gates)

`docs:spec-drift` must stay green — `element`/`ref` must not widen the
contract past `AppHeaderSpec`. The mechanism the contract uses for web-only
props is `WEB_ONLY_PROPS` in `packages/svelte/preview/scripts/contract-spec-drift.ts`
(comment: "web-platform plumbing, not component semantics"). Precedent: b009's
`initialFocus` and b010's `autofocus` both landed there, and b010 explicitly
noted the entry sits "outside its writable paths".

- `element` **is** a member of the contract's `### Public Props` table
  (it is a real Svelte prop) and is exempted from the `AppHeaderSpec` check by
  a new `WEB_ONLY_PROPS` entry, with a comment recording the g13-b014
  classification (GPUI/Jetstream own window dragging as an adapter capability
  and have no element to hand out).
- `ref` is documented in prose in the `### Element Access` section, not in the
  Public Props table — it is React's own mechanism, not a member of
  `AppHeaderProps`. Documenting it in prose matches the text-input.md
  precedent for React ref handles (`focus()` / `TextInputHandle`), so neither
  drift gate sees it. No `WEB_ONLY_PROPS` entry for `ref` (dead config; the
  set filters table rows only).

Adding the `### Public Props` heading also moved `app-header` from the drift
gates' "skipped" bucket into "checked" (`docs:contract-drift` now reports
129 checked, `docs:spec-drift` 113 checked). All seven pre-existing props were
already covered by `AppHeaderSpec` (title, subtitle, dragRegion →
`is_drag_region`, ariaLabel, size, sizeRole, density), so `element` was the
only new drift, and `WEB_ONLY_PROPS` absorbs it.

## The min-height override (documentation ask)

`--poodle-app-header-min-height` defaults to `--poodle-size-panel-header` on
`.poodle-app-header` (`packages/core/src/styles/app-header.css:7`). The size
ladder overrides it at `.poodle-app-header[data-size="xs"]` … `[data-size="xl"]`
(lines 54–78), specificity `0,2,0` (class + attribute). A plain
`.poodle-app-header { --poodle-app-header-min-height: … }` override is
`0,1,0` and **silently loses** — the observed finch workaround
(`.poodle-app-header[data-drag-region="true"]`) only wins because finch
happens to set `dragRegion`.

Sanctioned route documented in the contract: override the custom property with
a selector matching or exceeding `0,2,0`, with a worked example that does not
depend on `dragRegion`:

```css
.app-shell .poodle-app-header[data-size] {
  --poodle-app-header-min-height: 3.75rem; /* 60px */
}
```

`.app-shell .poodle-app-header[data-size]` = `0,3,0` (two classes + one
attribute) — beats every ladder step. `[data-size]` is always present on the
header (unlike `data-drag-region`), so the example tracks the ladder at every
size. The contract also notes the matching-`0,2,0` alternative
(`.app-shell .poodle-app-header`) is a source-order fight and to prefer the
`0,3,0` form.

## Findings

**ForwardRef components silently drop out of the parity and smoke gates**
(stopped-condition adjacent, recorded not worked around — the fix is outside
this card's writable paths). `test/parity/component-parity.test.tsx:35` and
`packages/react/components/test/smoke.test.tsx` enumerate React exports with
`typeof comp === "function"`; React `forwardRef` returns an element-type
object, so `AppHeader` moved from "covered" to "not enumerated" in both gates:
`test:parity` went 157 → 156 tests and the shared-surface count 156 → 155.
Pre-existing blind spot, not new to this card: `MenuSurface` (the card's own
forwarding precedent) and `TextInput` (since b010) were already dropped the
same way. Rendering parity itself is unaffected and is asserted by this card's
new per-runtime tests (identical anatomy assertions in both suites). Recorded
in `PAPERCUTS.md` with the filter fix (`$$typeof` check); the orchestrator can
land it as a maintenance change.

No other stop conditions were reached: no rendering change (tests assert the
pre-existing `data-drag-region="false"` string rendering, which both runtimes
emit identically — confirmed by dumping `container.innerHTML` before fixing
the test assumptions), `docs:spec-drift` green via `WEB_ONLY_PROPS`, `ref`
forwarding changed no other prop's behaviour, and the min-height override
works with the `0,3,0` selector — no `dragRegion`-style attribute needed.

## Validation

| Command | Exit state |
|---------|-----------|
| `bun install` | 0 |
| `effigy test:components` | 0 — 44 files / 838 tests (baseline 42 / 834; +2 files / +6 tests, see count note) |
| `effigy test:parity` | 0 — 156 (baseline 157; −1 = AppHeader's parity test dropped by the forwardRef enumeration blind spot, see Findings) |
| `effigy docs:lint` | 0 |
| `effigy docs:contract-drift` | 0 — checked 129 (app-header newly checked) |
| `effigy docs:spec-drift` | 0 — checked 113, `element` exempted via `WEB_ONLY_PROPS` |
| `effigy docs:check` | 0 — regenerated `packages/tokens/artifacts/rust/*` and `packages/react/preview/artifacts/component-docs.json`; both reverted (generated churn outside writable paths; the JSON artifact has been stale since g13.001 — it also carried b009/b010's unexported `initialFocus`/`autofocus` entries) |
| `git diff --check` | 0 |
| `git status --porcelain` | only the writable paths plus the sanctioned `contract-spec-drift.ts` edit and this log |

*Note on test counts:* `test:components` went 42 files / 834 tests → 44 / 838.
The +4 (not +6) is exact: +6 tests from this card's two new suites, −1 from
`packages/react/components/test/smoke.test.tsx` (154 → 153) and −1 from
`test/parity/component-parity.test.tsx` (155 → 154 shared components), both
drops caused by AppHeader leaving the `typeof comp === "function"` enumeration
(see Findings). `test:parity` separately went 157 → 156 for the same reason.
Per-file JSON listing confirms every other count is unchanged.

## Not done

Per batch card and worker rules: no rest spread, no action prop, no `id` /
`class` / `style` props, no Tauri import, no drag implementation, no change to
`data-drag-region`'s meaning, no GPUI / Jetstream / `poodle-render` /
`poodle-specs` / `poodle-ir` changes, no visual baseline refresh (nothing here
changes rendering), no consumer migration (finch stays on its CSS selector),
no Tabs files, no roadmap/status/dispatch edits, no merge, no `git add -A`.
The parity/smoke filter fix is deliberately left to the orchestrator (outside
writable paths); the parallel Tabs branch is untouched.
