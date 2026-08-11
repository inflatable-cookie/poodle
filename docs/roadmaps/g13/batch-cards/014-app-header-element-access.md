# 014 AppHeader Element Access

Status: ready
Milestone: side-quest (component API, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-014-app-header-element-access`
Governing refs: `docs/contracts/components/app-header.md`,
`docs/contracts/001-working-rules.md` §Runtime Parity Authority

## Problem

`AppHeader` takes `dragRegion?: boolean` and renders `data-drag-region` on its
`<header>`. That marker is correct — Poodle is multi-renderer and must not know
about Tauri — but `Props` is closed with no escape hatch, so a host has **no
supported way to attach behaviour to that element**. The marker is readable by
CSS and nothing else.

This is not hypothetical. `finch` (`~/Dev/projects/finch/app-tauri`) renders
`<AppHeader title="Finch" dragRegion ariaLabel="Finch">` and styles
`[data-drag-region="true"]`, but nothing implements the gesture, so its titlebar
does not drag. Five sibling apps hand-rolled headers and all five drag. finch is
the only one using the shared component and the only one broken.

Longhorn ships the behaviour as a Svelte action, `windowDrag`, from
`@inflatable-cookie/longhorn-poodle-svelte`.

## Maintainer Rulings (already decided — do not re-litigate)

1. **Element access only.** Expose the `<header>` element. Svelte via
   `bind:element`; React by forwarding `ref` to the `<header>`.
2. **No rest-props spread.** Rejected deliberately. It adds no capability over
   element access, and every `{...rest}` is an unbounded surface the IR cannot
   model — `BTN-15` is already carried as a `NEG-02` escape hatch. Do not add
   `{...rest}`, `{...restProps}`, or equivalent.
3. **No `action` prop.** Svelte actions have no React equivalent, so an action
   prop would be a Svelte-only API, which the Runtime Parity Authority rule
   forbids.
4. **No new named props.** Not `id`, not `class`, not `style`. Element access
   only; anything further is a separate decision.
5. **GPUI and Jetstream are not applicable.** Native window dragging is a
   platform capability the shell owns and there is no element to hand out.
   Record it as `AC` (adapter capability) in the contract's parity notes — do
   **not** invent a native escape hatch.
6. **`data-drag-region` keeps its current meaning.** Do not change it.
7. **Poodle implements no dragging and imports no `@tauri-apps/*`.**

## Scope

### In scope

- Svelte `AppHeader`: a bindable `element` prop exposing the `<header>`,
  defaulting to `null`.
- React `AppHeader`: `ref` forwarded to the `<header>`, following the
  `MenuSurface` precedent (`forwardRef`; React is 19.1).
- `app-header.md`: document the escape hatch, its intended use (host attaches
  behaviour such as window dragging), and the explicit non-goal that Poodle
  does not implement dragging.
- `app-header.md`: document the **min-height override** — see below.
- Usage docs for `app-header` in `component-docs.ts`.
- Tests in both web runtimes.

### Out of scope — stop conditions if reached

- Any rest-props spread, action prop, or additional named prop.
- Any Tauri import or drag implementation.
- Any change to GPUI, Jetstream, `poodle-render`, or `poodle-specs`. Element
  access is a web-runtime concern and must not reach the portable spec.
- Consumer repositories, including `finch`. Migration is separate.
- `poodle-ir`.

## The Documentation Ask (same card)

Consumers need per-app header heights (observed: 35.2, 48, 48, 62, 72px against
a 36/40/44/48/52px size ladder). Overriding
`--poodle-app-header-min-height` already works, but is undocumented and has a
**specificity trap**:

- The property is declared on `.poodle-app-header`
  (`packages/core/src/styles/app-header.css:7`).
- The size ladder overrides it at `.poodle-app-header[data-size="xs"]` …
  `[data-size="xl"]` (lines 54–78), specificity `0,2,0`.
- So a plain `.poodle-app-header { --poodle-app-header-min-height: … }` from an
  app **silently loses to the size ladder**.

`finch` worked around this with `.poodle-app-header[data-drag-region="true"]`,
which only wins because finch happens to set `dragRegion` — a technique one app
discovered, not a stated API.

Document the sanctioned route: overriding the custom property is supported, and
the override must match or exceed the size ladder's `0,2,0` specificity. Give a
worked example that does **not** depend on `dragRegion` being set — for example
scoping under an app-owned ancestor class, or using `[data-size]`. State the
default (`--poodle-size-panel-header`) and that the ladder sets it per size.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Svelte and React must expose the **same thing**: the `<header>` DOM element.
  Do not expose a custom handle object in React — `MenuSurface` is the
  precedent for *how* to forward, not for *what* to expose.
- Contracts are authority and must be updated in the same commit.
- Do not refresh any visual baseline. Nothing here should change rendering; a
  visual diff is a stop condition.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- Another worker holds all Tabs files on a parallel branch. Do not touch them.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-014-app-header-element-access`. Do not merge.

## Writable Paths

- `packages/svelte/components/src/AppHeader.svelte`
- `packages/react/components/src/AppHeader.tsx`
- `docs/contracts/components/app-header.md`
- `packages/svelte/preview/src/component-docs.ts` (`app-header` entry only)
- Tests for AppHeader in either web runtime
- `docs/logs/2026-08/<DD>-g13-014-app-header-element-access.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

## Steps

1. Baseline: `bun install`, `effigy test:components`, `effigy test:parity`,
   `effigy docs:lint`, `git diff --check`. Record exit states.
2. Read `AppHeader.svelte`, `AppHeader.tsx`, and `MenuSurface.tsx` (forwarding
   precedent). Note that 75 Svelte components already use `$bindable`, but none
   currently expose an element — follow the established `$bindable` idiom.
3. Svelte: add `element = $bindable<HTMLElement | null>(null)` and
   `bind:this={element}` on the `<header>`.
4. React: forward `ref` to the `<header>` element.
5. Contract: document the escape hatch, the non-goals, the GPUI/Jetstream `AC`
   classification, and the min-height override with its specificity rule.
6. Usage docs: show a host attaching behaviour via the element — a generic
   `$effect` applying an action, **not** a Tauri-specific example.
7. Tests: the Svelte binding yields the header element; the React ref resolves
   to the header element; neither changes rendered output.
8. Validate:
   ```sh
   effigy test:components
   effigy test:parity
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:spec-drift
   effigy docs:check
   git checkout -- packages/tokens/artifacts/rust/
   git diff --check
   git status --porcelain
   ```
   `docs:spec-drift` must stay green — `element`/`ref` must not appear to widen
   the contract past `AppHeaderSpec`. If it goes red, report how the contract
   marks web-only props (see `WEB_ONLY_PROPS` in `contract-spec-drift.ts`).

## Acceptance Criteria

- [ ] Svelte `AppHeader` exposes the `<header>` via a bindable `element` prop
  defaulting to `null`.
- [ ] React `AppHeader` forwards `ref` to the same `<header>` element.
- [ ] Both runtimes expose the DOM element itself, not a handle object.
- [ ] No rest spread, no action prop, no new named props, no Tauri import, no
  dragging implementation.
- [ ] `app-header.md` documents the escape hatch, its non-goals, and the
  GPUI/Jetstream `AC` classification.
- [ ] `app-header.md` documents the min-height override **and** the `0,2,0`
  specificity requirement, with an example not dependent on `dragRegion`.
- [ ] Tests cover both runtimes; rendered output is unchanged.
- [ ] No Rust, adapter, native, or baseline change.
- [ ] All step-8 commands exit 0.
- [ ] Batch log records commands, exit states, and the Svelte/React shapes used.

## Stop Conditions

- Exposing the element forces a rendering change.
- `docs:spec-drift` goes red.
- React cannot forward a ref without changing another prop's behaviour.
- The min-height override cannot be made to work without a `dragRegion`-style
  attribute selector — that would mean the specificity analysis is wrong;
  report it with the selectors and computed specificity.

Stop with exact paths, line numbers, commands, and the smallest unresolved
question.
