# g15.029 — Screen-clear review: foundation date and time

Date: 2026-08-20
Card: `docs/roadmaps/g15/029-review-foundation-date-time.md`
Handoff: `docs/handoffs/20260820-193238-g15-029-review-foundation-date-time.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
PR: #53

## Outcome

Second of the six serial screen-clear review children. All seven owned pages
received the human teaching review against the carried rubric — live Svelte
and React source and routes, GPUI specimen source, and the `g15.026` headless
construction/axis evidence. **Four pages keep unchanged, two pages needed
bounded specimen repairs, and one page records a contract/runtime blocker.**
No contract, public API, component, shared-CSS, date math, generated
catalogue, or infrastructure file moved.

The seven human-teaching verdicts are recorded in the existing audit rows in
`docs/roadmaps/g15/specimen-catalogue-audit.md`; the screening `keep` /
"no named defect" text was replaced, not extended with a second table.

## Verdict inventory

### Unchanged (4)

| Page | Verdict |
| --- | --- |
| `DatePicker` | keep — live default with Selected readout, pre-filled Mar 14, disabled; Sv/Rc paired; Gp mirrors with live open/select |
| `DateRangePicker` | keep — live range gesture, pre-filled Mar 1–14, disabled; Gp adds a static Open (range calendar) so the composed surface is visible without interaction |
| `DateTimePicker` | keep — default, pre-filled Mar 14 2:30 PM, disabled; trigger shows the committed date/time; Gp adds Open (calendar + time) |
| `DateTimeRangePicker` | keep — default, pre-filled Mar 10 9:00–Mar 14 5:00, disabled; start/end time fields visible when open; Gp adds Open (range calendar + start/end time) |

### Repaired (2)

- **`Calendar`** — GPUI only. Section order put Range before Disabled; the
  live range example seeded `2026-03-10`–`2026-03-20`, so it taught the same
  filled-range story as "Range with pre-selected range". Reordered to the
  web set (Default, pre-selected, Disabled, Range, Range pre-selected, Range
  disabled), started the live range empty, and show readouts only after a
  pick. Svelte/React unchanged.
- **`DurationInput`** — web and GPUI. The web pages grew a fourth
  "Last change" group after the first interaction; Total already teaches live
  binding, so that group is gone. GPUI captioned the default "Full (H:M:S)",
  hardcoded `Value: 01:30:00 (5400 seconds)` while the input was live, used
  different disabled/hours-and-minutes fixtures, and added Empty/zero and
  Invalid sections the web page does not need. Native page now matches the
  three web sections, fixtures, and a Total readout driven from stored state.

### Contract/runtime blocker (1)

- **`DateTimeZonePicker`** — Svelte and React. The nested `TimeZoneSelect`
  popover is portalled outside the picker's surface. The picker's outside-click
  handler uses `layerContains(root, surface)`
  (`packages/svelte/components/src/DateTimeZonePicker.svelte` and the React
  equivalent), so a pointer choice on a timezone option dismisses the picker
  without committing. Keyboard Enter on a highlighted option does commit.
  This is a component dismiss-layer defect, not dead specimen wiring; not
  implemented here. The audit row is `contract/runtime-blocker` with Sv/Rc
  grade D (dead primary pointer workflow). GPUI default was empty-aligned
  with web as a bounded specimen repair; the Open section still shows
  calendar + time + zone.

No other page needed a contract, public API, or component-semantic change.
Picker, calendar, range, and duration gestures on the kept and repaired pages
worked in the live web previews.

## Changed routes for review

- Svelte: `#components/duration-input`
- React: `#components/duration-input`
- GPUI (headless): `calendar`, `duration-input`, `date-time-zone-picker`

Operator sign-off is pending on the changed Svelte and React DurationInput
routes. This card is not claimed complete. The DateTimeZonePicker blocker is
not implemented here; the orchestrator routes the follow-up.

## Review round 1 (orchestrator, PR #53)

Two documentation blockers; both addressed in this revision. No specimen
source moved.

1. **Audit headline and totals were stale.** The header still said teaching
   judgment applied only to the three pilots, and the published totals did
   not match the 175 rows (they still used the four original dispositions
   and pre-curation grade counts). Header, grading explanation, disposition
   vocabulary, and totals now recount the rows: Svelte 88/33/44/10, React
   101/26/47/1, GPUI 103/65/6/0 + 1 n/a, worst 65/48/52/10; dispositions
   55 keep / 3 pilot-fix / 108 curation-tranche / 6 curation-complete /
   2 verified-no-op / 1 blocker. Recounted after the grade correction below.
2. **DateTimeZonePicker was undergraded.** Pointer selection in the nested
   timezone list dismisses the composite without committing, so the page
   cannot complete its central pointer workflow. Sv/Rc grades move from B
   to D. Disposition stays `contract/runtime-blocker`.

## Validation

- `effigy check:svelte-preview` — 0 errors
- `effigy react:build` — pass
- `effigy check:gpui` — pass
- `effigy probe:gpui-specimens` — 7/7 (174/174 routes construct, advertised axis panes open)
- `effigy catalogue:check` — pass (TS and Rust catalogue targets verified)
- `effigy docs:check` — pass
- `git diff --check origin/main...HEAD` — clean
