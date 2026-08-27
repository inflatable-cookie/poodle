# g16.012 — Collapsible Disclosure And Mounted Parity

Date: 2026-08-27
Status: complete — PR #86 (review fixes pushed)
Branch: `t3code/g16-012-collapsible-mounted-parity`
Card: `docs/roadmaps/g16/012-collapsible-disclosure-and-mounted-parity.md`
Source triage: `docs/triage/20260827-141543-post-g16-011-native-lane-decision.md`

## Outcome

Shared Rust Collapsible painted `default_open` correctly while announcing
`spec.open.unwrap_or(false)`, placed disclosure semantics on the outer shell,
left disabled triggers focusable, and ignored the GPUI wrapper's `with_id`.
The renderer now uses `spec.current_open()` for paint and trigger semantics,
moves button/region ownership onto the trigger/content pair, suppresses disabled
focus and activation, and exposes a handler-bundle entry point with
host-supplied instance scope while keeping the simple `collapsible(...,
on_open_change)` helper for composites.

The generated ledger moves only Collapsible's GPUI mounted-behaviour cell:
`missing` → `mounted` (40 → 41 mounted, 134 → 133 missing). Known-delta
totals stay 115 present / 60 not-applicable. GPUI accessibility stays
`manual`. GPUI visual stays `missing`. Jetstream stays deferred.

## Repair

- Effective open state is single-source through `spec.current_open()` for paint,
  content presence, trigger `expanded`, and next callback payload.
- Controlled `open` wins over `default_open`. The renderer stays stateless; the
  host rebuilds after `on_open_change`.
- The trigger owns button role, accessible name, expanded/controls state,
  sequential focus, structured focus ring, and activation. Open content owns
  region role and `labelled_by` the trigger. The outer shell is layout only.
- Disabled triggers emit nothing, carry no activation handler, use not-allowed
  cursor, and are skipped by sequential focus. Root opacity remains the visual
  disabled treatment.
- `CollapsibleHandlers.instance_id` and `collapsible_with_handlers` preserve
  lifetime-stable trigger/content identity across rebuilds. The GPUI wrapper's
  existing `with_id` now supplies that scope.
- The "Default open" specimen uses `default_open=true` with host rebuild after
  the first activation rather than a controlled `open` seed.

## Mounted evidence

`packages/gpui/preview/tests/headless_regressions.rs#collapsible_disclosure_and_identity_through_mounted_pointer_and_keyboard`
proves, through production hit testing, focus, and key dispatch:

- controlled closed → open and open → closed pointer activation
- Enter and Space report the next value and rebuild host content/expanded state
- `default_open=true` begins open, announces open, and first activation reports
  false
- title and aria-label naming, button/region ownership, controls/labelled-by,
  tab position, focus ring, and stable scoped identity
- two same-titled instances keep independent backend focus handles
- disabled targets never emit and are skipped by sequential focus

Direct handler calls are used only in focused renderer tests.

## Explicit non-claims

- no content height animation or exact web transition timing
- no trigger snippets in native Rust
- no Svelte/React public behavior or prop change
- no Accordion, TriStateSwitch, NumberInput, EditableLabel, Select, Drawer, or
  other disclosure consumer work
- no Jetstream admission, visual comparison, or other ledger row
- no broad native assistive-technology coverage

## Validation

Focused `poodle-render` Collapsible tests (8), Svelte and React Collapsible
tests (unchanged), named mounted Collapsible regression.
`effigy regressions:native`, `effigy probe:gpui-specimens`, drift selectors,
`effigy test:parity-evidence-ledger`, `effigy check:parity-evidence-ledger`,
`effigy ci:native`, `effigy ci:web`, `effigy docs:check`, `effigy qa`, and
`git diff --check origin/main...HEAD`.

`effigy doctor` is already red on the planning base (generated-in-src,
god-files, stale-suppressions). That baseline is unchanged and was not
absorbed.

## Remaining gaps

- native accessibility, visual comparison, and Jetstream admission are
  unchanged and unclaimed
- the next evidence lane is an orchestrator checkpoint against 41 mounted /
  133 missing
