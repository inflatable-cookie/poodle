# g15.032 — Screen-clear review: composition navigation and overlays

Date: 2026-08-21
Card: `docs/roadmaps/g15/032-review-composition-navigation-overlays.md`
Handoff: `docs/handoffs/20260821-111649-g15-032-review-composition-navigation-overlays.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
PR: https://github.com/inflatable-cookie/poodle/pull/58
Merge: `8021ce93`

## Outcome

Fifth serial screen-clear review child. All ten owned navigation/overlay pages
received the human teaching review against the carried rubric — live Svelte and
React routes, GPUI specimen source, and the `g15.026` headless
construction/axis evidence. **Nine pages keep unchanged; Popover is a routed
web contract/runtime blocker.** No specimen, contract, public API, component,
shared-CSS, generated catalogue, or infrastructure file changed in this batch.

The ten human-teaching verdicts are recorded in the existing audit rows in
`docs/roadmaps/g15/specimen-catalogue-audit.md`; screening `keep` /
"no named defect" text was replaced, not extended with a second table.
Mechanical totals recounted once for the Popover blocker (Svelte A 89→88 /
C 44→45, React A 102→101 / C 47→48, worst-of-three A 66→65 / C 52→53;
`keep` 56→55, `contract/runtime-blocker` 0→1).

Gesture evidence was gathered by mounting each specimen page and firing the
contract's real events — right-click for ContextMenu, pointer-enter and focus
for HoverCard, Escape/outside-press dismissal for the overlays, hover-switch
for Menubar — in both web runtimes (happy-dom scratch harness, not committed).

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

### Routed blocker (1)

- **`Popover` (Sv/Rc — contract/runtime-blocker, C/C/A)** — both examples
  anchor the popover to a real `Button`. In the default composition the
  trigger wrapper adds its own `role="button"`/`tabindex=0` around the
  interactive child (nested interactives) and Escape restores focus to the
  inert wrapper — the g14.007 defect shape. The `triggerIsInteractive`
  alternative removes the wrapper role but cannot transfer `aria-expanded`/
  `aria-controls` to the child, so the contract's required
  trigger-to-content relationship (`docs/contracts/components/popover.md`
  §5–6) is lost: no supported web composition satisfies the trigger contract
  today. This is a component/contract defect, not specimen scope; the
  orchestrator routes a dedicated Popover API/semantics repair. GPUI grades A
  and stays `keep`-equivalent evidence because native composes its trigger
  directly and its page already adds placement, surface-width, and disabled
  evidence.

## Review round 2 (orchestrator, PR #58)

The first revision of this PR carried a specimen-level repair: both Popover
examples gained `triggerIsInteractive`, plus focused Sv/Rc tests. Orchestrator
review showed that repair exposes the contract gap above — the relationship
attributes disappear with no API to reattach them — so the repair and its
tests were reverted and the row re-graded C/C/A `contract/runtime-blocker`.
After the revert, **no specimen page changed in this batch** and there are no
changed routes awaiting operator live sign-off.

## Changed routes for operator review

None. The Popover repair was reverted in review round 2; the batch is
docs-only (audit rows/totals plus this log).

## Changed files

- `docs/roadmaps/g15/specimen-catalogue-audit.md` — ten human verdict rows,
  recounted totals, status/revision header
- `docs/logs/2026-08/20260821-g15-032-navigation-overlays-review.md` — this log

## Validation

- `effigy catalogue:check` — passed
- `effigy check:svelte` — passed (0 errors; warnings identical to pristine main)
- `effigy react:build` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

Live review used the Svelte preview on `http://127.0.0.1:4175` and the React
preview on `http://127.0.0.1:4181`, both with `--strictPort`. No GPUI specimen
code changed, so `check:gpui` / `regressions:native` were not required. No
`*-windowed`, `test:native-visual`, browser screenshot gate, Jetstream, or
release selector ran.

## Operator checkpoint

No changed routes remain, so no live operator sign-off is owed by this batch.
The lane stays open on the routed Popover blocker: the orchestrator owns the
dedicated API/semantics repair and the return to `g15.032` closeout.

## Addendum (2026-08-21, `g15.041`)

The routed Popover API/semantics repair landed as `g15.041`: core authors a
`PopoverTriggerState` payload (`expanded` / `controls` / `disabled`), the
Svelte and React interactive trigger is a state-aware render that applies it
to the real control, and the paired Popover specimens compose real Poodle
Button triggers again. The Popover audit row returns to A/A/A `keep` with
totals recounted (revision 14); operator live sign-off on the paired Popover
routes was not repeated: the operator explicitly authorised final PR #59 fixes
and merge without a renewed live-route pass, so no fresh visual evidence is
claimed. PR #59 merged as `e19aea4b`; the blocker and this card are closed.
Batch log:
`docs/logs/2026-08/20260821-g15-041-popover-interactive-trigger-semantics.md`.
