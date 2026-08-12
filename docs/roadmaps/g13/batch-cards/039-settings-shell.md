# 039 SettingsShell — A New Component, Designed Not Ported

Status: ready
Milestone: side-quest (new component, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-039-settings-shell`
Depends on: none
Governing refs: `docs/contracts/components/history-center.md` (lines 20–24 —
the no-Longhorn rule and the structural-types precedent),
`docs/contracts/components/detail-shell.md`,
`docs/contracts/components/picker-shell.md`,
`docs/contracts/001-working-rules.md`

## Goal

Longhorn's settings dialog is the last substantial piece of design living in an
authority package. Build the Poodle component that replaces it.

**Do not port it.** Read the existing 435-line implementation **once**, to learn
the data it is fed, then design the component you would have designed if it had
never existed. Its concept is right — search on top, navigation left, page
right, all inside a dialog. Its execution is wrong in almost every way, and the
list below is the maintainer's, from using it.

## Fixed By Ruling (do not re-decide)

### R1 — The seven defects the design must not reproduce.

1. **The navigation floats.** No surface, no border, no container: it reads as
   text on the dialog background with no boundary against the page. It is a
   **region**, and wants a surface, its own scroll, and a clear edge.
2. **Group labels are composed and too long.** The current one builds
   `` `${module.label} · ${section.label}` `` whenever more than one module is
   registered, so a Storage module holding a Storage & Backups section renders
   `STORAGE · STORAGE & BACKUPS`, which wraps to two lines. Longhorn will stop
   composing, but **a component that assumes short labels breaks on the next
   host**: group labels render on **one line**, truncated with a tooltip. Never
   wrapped.
3. **Scrolling is unowned.** Page content runs past the dialog. Navigation and
   page scroll **independently**; the search field and the page header stay
   put.
4. **Two close buttons.** The dialog draws its own `×` and the shell adds a
   ghost "Close" into every page header. **One close: the dialog's.** Nothing
   in the page header.
5. **The proportions are off.** Dialog too wide, search spanning the full width
   above both columns, page title enormous against everything else,
   inconsistent rhythm between header, description and content.
6. **Search overlays.** It must **replace**: while a query is active the page
   region shows a flat result list; clearing it returns to the page. No
   dropdown, no overlay.
7. **Empty and refused states are undesigned.** No groups, no results, and a
   refused close each need a real state. **A refused close is not an error** —
   it is the host saying there are unsaved changes here. Use `Callout`, not an
   error treatment.

### R2 — A composition. No core machine.

`DetailShell`, `PickerShell`, `FormShell` and `ScrollShell` all ship CSS and no
`packages/core` logic. SettingsShell follows them. State is the host's, through
props; `searchQuery` and `open` are bindable.

If you find yourself wanting a machine in `packages/core`, stop and say why —
that is a design change, not an implementation detail.

### R3 — Structural types. No Longhorn.

Declare the shapes locally, exactly as `history-center.md` lines 20–24 require.
Poodle never imports Longhorn, never fetches, and never learns what a storage
profile or a keymap is. The page body is **always** a snippet.

Starting shapes from the handoff — treat as what the current host needs, not as
a specification. Take the shape you want and say why if you change it:

```ts
interface SettingsNavGroup {
  id: string;
  label: string;
  items: { value: string; label: string }[];
}

interface SettingsSearchResult {
  pageId: string;
  pageLabel: string;
  anchorId?: string;
  anchorLabel?: string;
}
```

Props, roughly: `groups`, `activePageId`, `pageTitle`, `pageDescription?`,
`searchQuery` (bindable), `searchResults` (`null` means not searching),
`open` (bindable), `page` (Snippet), `onNavigate(pageId, anchorId?)`,
`onRequestClose()`, `closeRefusedReason?`.

`onRequestClose` is **commands out**: the host decides, and refuses by
supplying `closeRefusedReason`. The shell never closes itself against a
refusal.

### R4 — Compose the primitives that exist.

`Dialog`, `SidebarNav`, `PageHeader`, `Surface`, `ScrollShell`, `TextInput`,
`Callout` and `EmptyState` all exist in both web runtimes with contracts —
verified. Use them. A hand-rolled equivalent of any of them is a failed card.

If a primitive is genuinely missing something this needs, that is a finding:
say which primitive and what it lacks, rather than working around it locally.

### R5 — Web only, and record the native gap.

Svelte first, React mirrors exactly. **No Rust spec, no native
implementation** — `HistoryCenter` is the precedent for a web-only component,
and native parity is deferred to `g13.014`
(`docs/roadmaps/g13/native-registration-gap.md`).

Add `SettingsShell` to that inventory's table in the same commit, so the gap
stays counted rather than silently growing to sixteen.

### R6 — Register it everywhere a component must be registered.

Mirror `DetailShell`'s registration set exactly:

- `packages/{svelte,react}/components/src/index.ts`
- `packages/core/src/styles/settings-shell.css`
- `packages/svelte/preview/src/component-registry.ts`
- `packages/svelte/preview/src/specimens/registry.ts`
- `packages/svelte/preview/src/parity.ts`
- `packages/svelte/preview/src/component-docs.ts`
- `packages/react/preview/src/gallery/specimen-map.ts`
- both specimen files
- `test/fixtures/component-props.ts` — the smoke and parity suites are
  glob-driven over every component; a new one without a fixture is a silent
  hole, not a pass.

## Scope

### In scope

- The component in both web runtimes, its CSS, its contract, its specimens,
  its tests, and every registration point in R6.

### Out of scope — stop conditions if reached

- Longhorn's settings **content**. Storage, backup, restore and keymap pages
  render Longhorn domains and stay there. You are building the frame.
- Any Longhorn or `longhorn-poodle-svelte` file, including the `<style>`-block
  check that package will gain. That is Longhorn's side of the move.
- Native adapters, Rust specs, `poodle-ir` (R5).
- Refreshing visual baselines.
- `HistoryCenter`, and anything card `037` is touching in
  `packages/core/src/styles/*.css` — if you need a focus-ring change in an
  existing sheet, say so rather than editing it.

## Required Tests

Both runtimes:

- Group labels never wrap: a label long enough to overflow truncates and
  carries a tooltip.
- Exactly one close affordance in the whole shell.
- A non-null `searchResults` replaces the page region; the page snippet does
  not render. Clearing the query restores it.
- Navigation and page are separately scrollable regions; the search field and
  page header are outside both.
- `onNavigate` fires with the page id, and with the anchor id when a result
  carries one.
- `onRequestClose` fires on a close attempt and the shell stays open.
- `closeRefusedReason` renders as a `Callout`, not an error, and is announced.
- Empty groups and empty results each render their designed state.
- Both runtimes render the same anatomy.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read the Longhorn implementation **once**, for the data shape. Do not open it
  again while designing. If your layout mirrors its structure, you have ported
  it.
- **Run `effigy check:svelte`.** Not optional.
- Run `effigy docs:callback-drift` — new callbacks must be documented.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with `git push -u origin thread/g13-039-settings-shell`. Do
  not merge.

## Writable Paths

- `packages/{svelte,react}/components/src/SettingsShell.{svelte,tsx}`
- `packages/{svelte,react}/components/src/index.ts`
- `packages/{svelte,react}/components/test/SettingsShell.test.*`
- `packages/core/src/styles/settings-shell.css`
- `packages/svelte/preview/src/component-registry.ts`
- `packages/svelte/preview/src/specimens/registry.ts`
- `packages/svelte/preview/src/specimens/SettingsShellSpecimen.svelte`
- `packages/svelte/preview/src/parity.ts`
- `packages/svelte/preview/src/component-docs.ts`
- `packages/react/preview/src/gallery/specimen-map.ts`
- `packages/react/preview/src/gallery/specimens/SettingsShellSpecimen.tsx`
- `test/fixtures/component-props.ts`
- `docs/contracts/components/settings-shell.md`
- `docs/roadmaps/g13/native-registration-gap.md`
- `docs/logs/2026-08/<DD>-g13-039-settings-shell.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:components`, `check:svelte`, `docs:lint`,
   `git diff --check`. All green.
2. Read the Longhorn implementation once. Write down only the data it is fed.
3. Design: write the contract **first** — anatomy, props, callbacks, states,
   keyboard, accessibility. The seven defects in R1 are acceptance criteria for
   the design, not afterthoughts.
4. Build the Svelte component from the contract, composing R4's primitives.
5. CSS: the navigation surface, the two scroll regions, the proportions.
6. Mirror React exactly.
7. Specimens in both previews, covering: normal, searching, no groups, no
   results, and a refused close.
8. Register everything in R6, including the test fixture.
9. Validate:
   ```sh
   effigy test:core
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:callback-drift
   effigy svelte:surface-audit
   effigy drift:recipes
   effigy ci:web
   git diff --check
   ```

## Acceptance Criteria

- [ ] All seven R1 defects absent, each demonstrable in a specimen.
- [ ] Composed from existing primitives; nothing hand-rolled that exists.
- [ ] No `packages/core` machine, no Longhorn import, no fetch.
- [ ] Page body is a snippet in both runtimes.
- [ ] Registered at every point in R6, fixture included.
- [ ] Added to the native registration gap inventory.
- [ ] All step-9 commands exit 0; no baseline refreshed.

## Stop Conditions

- A named primitive cannot do what the design needs (say which, and what it
  lacks).
- The design needs state Poodle cannot own without knowing page content.
- Truncation with a tooltip cannot be made accessible in a nav region.

Stop with exact paths, commands, and the smallest unresolved question.
