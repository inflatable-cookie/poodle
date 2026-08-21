# g15.032 — Screen-clear review: composition navigation and overlays

Date: 2026-08-21
Card: `docs/roadmaps/g15/032-review-composition-navigation-overlays.md`
Handoff: `docs/handoffs/20260821-111649-g15-032-review-composition-navigation-overlays.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
PR: https://github.com/inflatable-cookie/poodle/pull/58

## Outcome

Fifth serial screen-clear review child. All ten owned navigation/overlay pages
received the human teaching review against the carried rubric — live Svelte and
React routes, GPUI specimen source, and the `g15.026` headless
construction/axis evidence. **Nine pages keep unchanged; Popover needed a
bounded Sv/Rc specimen repair.** No contract, public API, component, shared-CSS,
generated catalogue, or infrastructure file moved outside specimen
presentation.

The ten human-teaching verdicts are recorded in the existing audit rows in
`docs/roadmaps/g15/specimen-catalogue-audit.md`; screening `keep` /
"no named defect" text was replaced, not extended with a second table.
Mechanical totals unchanged (all ten remain `keep` at A/A/A).

Gesture evidence was gathered by mounting each specimen page and firing the
contract's real events — right-click for ContextMenu, pointer-enter and focus
for HoverCard, Escape/outside-press dismissal for the overlays, hover-switch
for Menubar — in both web runtimes (happy-dom scratch harness, not committed;
the committed focused tests below cover the one changed interaction).

## Verdict inventory

### Unchanged (9)

| Page | Verdict |
| --- | --- |
| `Breadcrumbs` | keep — live-navigation basic trail, deep path, collapsed ellipsis; Sv/Rc paired; Gp mirrors statically with both axes |
| `NavigationMenu` | keep — six sections each teach a distinct activeEdge/activeFill treatment; live value readout; disabled item shown; Gp mirrors with a live first example |
| `Pagination` | keep — numbered, simple-with-limit-selector, full, and chrome variants distinct and live; Gp's extra standalone and last-page sections judged useful renderer-owned boundary evidence, not drift |
| `PaginationSummary` | keep — default, single-page, large-dataset derived copy; read-only by design |
| `Collapsible` | keep — closed/open/disabled/highlighted/custom-trigger; live toggle verified; Gp toggles live, omits only the custom-trigger example |
| `ContextMenu` | keep — real right-click opens at the pointer, actions fire, Escape/outside dismiss; left-click no-op is the contract's gesture |
| `DebugDialog` | keep — payload and custom trigger open live; "Hidden when null" caption teaches the absent trigger |
| `HoverCard` | keep — hover and focus both open after the 180ms intent delay, Escape closes; Gp wires real hover-intent delays |
| `Menubar` | keep — live readout, hover-to-switch, item-focused Escape returns focus to the trigger; Gp adds live checkbox/radio state |

### Repaired (1, Sv/Rc only)

- **`Popover`** — both examples anchored the popover to a real `Button` inside
  the default trigger wrapper, so the wrapper added its own `role="button"`
  and `tabindex=0` around the button (nested interactives) and Escape restored
  focus to the inert wrapper instead of the button the operator was on — the
  g14.007 defect shape. The contract (`popover.md` §3) directs
  `triggerIsInteractive` when the trigger is already an interactive control;
  both Sv/Rc examples now set it, the wrapper only observes clicks, and focus
  returns to the inner button. Focused Sv/Rc regression tests cover the
  trigger composition and the open/Escape/focus-return cycle, twice for
  repeatability. Gp unchanged — native composes its trigger directly, and its
  page already adds placement, surface-width, and disabled evidence.

## Changed routes for operator review

Changed Svelte routes: `popover`
Changed React routes: `popover`
Changed GPUI routes: none

Operator live sign-off on the changed Svelte and React `popover` routes is
**pending** — not yet given.

## Changed files

- `packages/svelte/preview/src/specimens/PopoverSpecimen.svelte`
- `packages/react/preview/src/gallery/specimens/PopoverSpecimen.tsx`
- `packages/svelte/preview/test/g15-032-navigation-overlays.test.ts`
- `packages/react/preview/test/g15-032-navigation-overlays.test.tsx`
- `docs/roadmaps/g15/specimen-catalogue-audit.md` — ten human verdict rows,
  status/revision header

## Validation

- `bunx vitest run --project svelte-preview packages/svelte/preview/test/g15-032-navigation-overlays.test.ts` — 2 passed
- `bunx vitest run --project react-preview packages/react/preview/test/g15-032-navigation-overlays.test.tsx` — 2 passed
- `effigy catalogue:check` — passed
- `effigy check:svelte` — passed
- `effigy react:build` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

Live review used the Svelte preview on `http://127.0.0.1:4175` and the React
preview on `http://127.0.0.1:4181`, both with `--strictPort`. No GPUI specimen
code changed, so `check:gpui` / `regressions:native` were not required. No
`*-windowed`, `test:native-visual`, browser screenshot gate, Jetstream, or
release selector ran.

## Operator checkpoint

Pending orchestrator review and live operator sign-off on the changed
`popover` routes.
