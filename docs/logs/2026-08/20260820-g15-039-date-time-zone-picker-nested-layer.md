# g15.039 — DateTimeZonePicker nested-layer pointer commit

Date: 2026-08-20
Card: `docs/roadmaps/g15/039-date-time-zone-picker-nested-layer.md`
Handoff: `docs/handoffs/20260820-210052-g15-039-date-time-zone-picker-nested-layer.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
Found by: `g15.029`, PR #53

## Outcome

Paired-web repair for the DateTimeZonePicker nested-layer pointer blocker.
A timezone option in the portalled TimeZoneSelect list now commits through
`onValueChange` without closing the picker. A genuine outside press still
dismisses the whole composite in one gesture, including while that list is
open. No public API, specimen, timezone data, or native surface changed.

## Defect

Both web implementations installed a private document `mousedown` dismiss
handler whose `contains` covered only the picker root and outer surface.
TimeZoneSelect portals its options through Select, so a real option press
reached that handler as an outside interaction and closed the picker before
the option click committed. Keyboard Enter still worked.

Open-state tracking alone would have been the wrong fix: ignoring outside
presses while the nested list is open would leave the composite open or
require a second click.

## Implementation

DateTimeZonePicker now registers on the shared dismiss-layer stack while
open. `contains` still covers only the trigger root and the picker's own
portalled surface. Nested Select already registered its listbox; it now
also passes `hostElement` so parenthood is containment-based when child
effects register first. Stack ancestry then spares the picker on an option
press and still dismisses picker plus list on a true outside press, in one
gesture.

Escape follows the shared dismiss-stack policy: innermost layer first. That
is an intentional contract decision, not a side-effect of switching
handlers. Closing the outer picker on the first Escape while the timezone
list is open would fight `002-anchored-overlays.md`. The picker-only Escape
path is unchanged.

This follows `docs/contracts/002-anchored-overlays.md`: a host does not
widen `contains` into a child's portal.

## Contract

`docs/contracts/components/date-time-zone-picker.md` now states the nested
ownership guarantee: a portalled timezone option is inside the composite,
and a genuine outside press dismisses the whole picker in one gesture.
Escape is an explicit stack decision: first keypress closes the innermost
layer only (timezone list, then picker). The previous picker-owned
document handler closed the outer overlay on the first Escape even when
the list was open; that behavior is not preserved, because it conflicts
with the shared stack.

## Evidence

Paired focused tests dispatch `mousedown` on a real portalled timezone
option, assert the picker is still open and `onValueChange` has not run,
then `click` and assert the timezone commit. A separate case mousedowns
`document.body` while that list is open. A third case proves Escape closes
the list first and the picker on the next keypress. The option is asserted
to sit outside the picker surface before the pointer sequence.

## Audit

`specimen-catalogue-audit.md` revision 9. DateTimeZonePicker moves from
D/D/A `contract/runtime-blocker` to A/A/A `keep`.

| | Before | After |
| --- | --- | --- |
| Svelte | 88 / 33 / 44 / 10 | 89 / 33 / 44 / 9 |
| React | 101 / 26 / 47 / 1 | 102 / 26 / 47 / 0 |
| GPUI | 103 / 65 / 6 / 0 + 1 n/a | unchanged |
| Worst of three | 65 / 48 / 52 / 10 | 66 / 48 / 52 / 9 |
| `keep` | 55 | 56 |
| `contract/runtime-blocker` | 1 | 0 |

Totals recounted from the 175 inventory rows.

## Validation

Headless only. No windowed, native-visual, Jetstream, or release selector ran.

- focused Svelte and React DateTimeZonePicker tests — 20 passed
- paired Select tests after the `hostElement` plumbing — 8 passed
- `effigy check:svelte` — 0 errors
- `effigy react:build` — pass
- `effigy catalogue:check` — pass
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD`

## Review round 1 (PR #54)

Two evidence/contract blockers; both addressed on this branch.

1. The timezone commit case now dispatches `mousedown` on the real portalled
   option, asserts the picker is still open and `onValueChange` has not run,
   then `click` and asserts the commit. `fireEvent.click` alone does not
   emit `mousedown`, so it could not catch the original defect.
2. Nested Escape is an explicit contract decision: the shared dismiss stack
   unwinds innermost-first. Paired tests cover list-then-picker. Restoring
   "first Escape closes the outer picker" would fight
   `002-anchored-overlays.md` and is not done here.

## Unresolved

None inside this card. `g15.030` resumes the serial screen-clear review lane.

## Closeout

The orchestrator independently reran the full component board (350 files,
3,018 tests), `effigy check:svelte`, `effigy react:build`,
`effigy catalogue:check`, `effigy docs:check`, and
`git diff --check origin/main...HEAD`. All passed. PR #54 was accepted at
`b2d45a80` and merged as `cb3d7ede` after the paired live routes were opened
and the operator authorised merge.
