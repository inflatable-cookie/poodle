# g15.039 — DateTimeZonePicker Nested-Layer Pointer Commit

Status: **complete** — PR #54 accepted and merged as `cb3d7ede`; `g15.030`
unblocked
Parent: `027-screen-clear-human-review.md`
Found by: `029-review-foundation-date-time.md`, PR #53
Governing refs: `../../contracts/components/date-time-zone-picker.md`,
`../../contracts/components/time-zone-select.md`,
`../../contracts/001-working-rules.md`

## Problem

DateTimeZonePicker composes a portalled TimeZoneSelect inside its own
portalled surface. Both web implementations treat only the picker root and
outer surface as inside the composite. A pointer press on a timezone option is
therefore seen by the outer document-level dismiss handler as outside: the
picker closes before the option commits. Keyboard selection works, but the
primary pointer workflow is dead in Svelte and React.

## Goal

Make nested timezone-option pointer selection count as interaction inside the
DateTimeZonePicker composite, without weakening genuine outside-click
dismissal or introducing a new public API.

## Scope

- Svelte and React DateTimeZonePicker nested-layer ownership/dismiss logic
- TimeZoneSelect or shared internal layer plumbing only where needed to expose
  the nested portal to its owning composite
- paired focused DateTimeZonePicker tests
- the DateTimeZonePicker contract only if nested-layer wording needs to become
  explicit
- the audit row, totals, and one August batch log

Do not change picker value shape, public props, timezone data, date/time math,
specimen composition, shared Rust, GPUI, Jetstream, or unrelated overlays.
Do not special-case a global CSS selector or accept a two-click outside
dismissal as the fix. Prefer explicit layer ownership over DOM-shape coupling.

## Acceptance

- A pointer click on a portalled timezone option commits its identifier through
  `onValueChange` in Svelte and React.
- The nested option click does not dismiss the outer picker before commit.
- A genuine outside pointer press still dismisses the whole composite in one
  gesture, including while the timezone list is open.
- Existing Escape, controlled/uncontrolled open state, calendar selection,
  time entry, focus, and disabled behavior do not regress.
- Paired focused tests exercise the real portalled option path and the
  outside-click path; a synthetic direct callback is not sufficient evidence.
- The audit moves DateTimeZonePicker from D/D to the grade supported by the
  repaired live behavior, with totals mechanically recounted.
- No public package API or native-runtime surface changes.

## Validation

- focused Svelte and React DateTimeZonePicker tests
- `effigy check:svelte`
- `effigy react:build`
- `effigy catalogue:check`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Do not run a `*-windowed`, native-visual, Jetstream, or release
selector.

## Stop Conditions

- A general overlay-stack redesign becomes necessary.
- The fix requires a new public prop or breaking component contract.
- Svelte and React need observably different dismissal semantics.
- Validation exposes the same defect in another composite whose repair would
  expand this card.

## Closeout

PR #54 moved DateTimeZonePicker and its nested TimeZoneSelect onto the shared
host-aware dismiss stack in both web runtimes. Real option `mousedown` then
`click` evidence proves the portalled option commits without dismissing the
picker; a genuine outside press still closes the whole composite in one
gesture. Paired tests also lock the shared innermost-first Escape policy.

The full 3,018-test component board, Svelte check, React build, catalogue and
docs gates, and range diff check passed in review. The operator authorised the
merge after the paired live routes were opened. No public API, specimen, native,
windowed, Jetstream, or release surface changed.
