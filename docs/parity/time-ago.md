<!-- parity consv=fixed gpui=2 jetstream=2 specimen=ok -->
<!-- pass: spec fixed (text-primary, live=true default, +interval/tooltip_format/timezone, shared format_relative w/ no week tier); GPUI font-size token-resolved + week tier dropped; Jetstream now computes relative time (parse→diff→shared formatter) + probe tests; Jetstream specimen rebuilt on real ISO timestamps (+Future/Long/Static groups). Remaining gpui/jet=2 each = live re-render (preview-loop) + tooltip (no Tooltip primitive wired) — accepted cross-target follow-ups. -->
# Parity: TimeAgo

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/time-ago.md`
- Svelte (authoritative): `packages/svelte/components/src/TimeAgo.svelte`
- GPUI: `packages/gpui/components/src/primitives/time_ago.rs`
- Jetstream: `packages/jetstream/components/src/time_ago.rs`
- Spec: `packages/contracts/components/src/time_ago.rs` (`poodle_specs::TimeAgoSpec`)
- Specimens: svelte `packages/svelte/preview/src/specimens/TimeAgoSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/time_ago_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/time_ago.rs`

## Contract ↔ Svelte

Prop/anatomy/state divergences. Contract-side mismatches are fixed below. The remaining items are spec/code defaults (not contract↔Svelte) and are left for the Rust-impl pass since this task edits docs only.

- [x] FIXED **Anatomy: `Tooltip`, not native `title`.** Svelte wraps the `<time>` in `<Tooltip content={absoluteText}>` (`TimeAgo.svelte:141-150`); no `title` attribute. Updated contract §2 anatomy, §6 semantics, §7, §8 HTML-attributes, §9, and Tier-1 checklist to describe the `Tooltip` wrapper; GPUI/Jetstream tooltip noted as a deferred delta.
- [x] FIXED **`cursor` divergence.** Svelte sets `cursor: help` (`TimeAgo.svelte:164`). Contract §7 + §8 root table → `cursor: help`.
- [x] FIXED **Dotted-underline affordance.** Svelte adds dotted underline with `--poodle-time-ago-underline{,-hover}` color-mix vars + hover/focus-visible transition (`TimeAgo.svelte:155-178`). Added to §8 root table + new hover/focus-visible table; Tier-2 checklist updated.
- [x] FIXED **`tooltipFormat` semantics.** Svelte `formatAbsolute` branches `date`/`datetime`/`full` (`TimeAgo.svelte:81-123`). Replaced the stale single-`title` row with a three-branch format table in §8; GPUI notes updated.
- [x] FIXED **`short` just-now clarification.** §4 just-now row + §8 table now explicitly label short=`"now"` / long=`"just now"`.
- [x] FIXED **`yesterday` long-form special case.** Svelte returns `"yesterday"` for `!short && !isFuture && days === 1` (`TimeAgo.svelte:64`). Added the special-case note under the §8 formatting table.

Spec/code-side (fixed in this pass):
- [x] FIXED (spec) `TimeAgoSpec::text_color_token()` now returns `COLOR_TEXT_PRIMARY` — contract/Svelte align. Unblocks the color todo on both Rust targets.
- [x] FIXED (spec) Added `interval: u32` (default 30000), `tooltip_format: TimeAgoTooltipFormat` (default `Datetime`), `timezone: Option<String>` (default `None`) + builders, matching contract §3.
- [x] FIXED (spec) `TimeAgoSpec::default()` now sets `live: true`.
- [x] ADDED (spec) `TimeAgoSpec::format_relative(diff_seconds)` + free `format_relative()` — single-source threshold table (no week tier, long-form "yesterday") matching Svelte. Both Rust targets delegate to it. Unit-tested in `poodle_specs`.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED Font size now resolves from `spec.font_size_token()` via `resolve_px(theme, …)` (was hardcoded `rem_to_px(0.875)`).
- [x] FIXED Renders `text-primary` now (via the spec `text_color_token()` fix).
- [x] FIXED Week tier dropped. The component no longer owns its own `format_duration`; it parses + diffs and delegates to `spec.format_relative()` (no week tier; 10d → "10d ago").
- [ ] No live re-render: `live` is stored but `into_element` computes once against `SystemTime::now()` with no timer/frame loop. Contract §10 requires periodic re-render when `live=true`. Preview has no timer wiring either. **Accepted cross-target follow-up (preview-loop concern, not a token/format bug).**
- accepted: no `<time>`/`datetime`/`aria-label` (gpui has no semantic-element or accessibility API) — renders a plain `div`.
- accepted: no tooltip / absolute-time `title` (no Tooltip primitive wired here).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED **Relative-time computation added.** `js_time_ago` now parses the timestamp (ISO 8601 with/without time, `Z` suffix), diffs vs `SystemTime::now()`, and delegates to `spec.format_relative()` (the shared threshold table — same source as GPUI). Falls back to the raw string only on parse failure. Probe-tested (2m/3h/2d, 10d no-week, long-form words).
- [x] FIXED Renders `text-primary` now (via the spec `text_color_token()` fix). Probe-tested.
- [ ] No live re-render: `spec.live` is read by the spec but no timer in component or preview. **Accepted cross-target follow-up (preview-loop concern).**
- [ ] No tooltip / absolute-time affordance. **Accepted delta — no Tooltip primitive wired into this component (GPUI is also tooltip-less here).**
- accepted: no ARIA / semantic `<time>` element (Jetstream has no accessibility channel).
- note: token resolution is clean — font-size via `resolve_px(theme, spec.font_size_token())`, color via the spec token. No hardcoded literals.

## Specimen parity

- Svelte covers: Recent (2m/3h/2d), Future (+5m), Long format (2m/2d), Static (`live=false`), Inherited typography (inline prose), From ISO string (`TimeAgoSpecimen.svelte`).
- GPUI covers: Recent (2m/3h/2d), Future (+5m), Inherit typography, Long format (2m/2d), From ISO string, Static (no live update) — **all from ISO timestamps**, so it actually exercises the relative-time math (`time_ago_specimen.rs`). Closest to Svelte. — missing: nothing material; static group can't demonstrate the live/no-live distinction since live isn't implemented.
- Jetstream covers (rebuilt this pass): Recent (2m/3h/2d), Future, Long format (2m/2d), Static (`live=false`), Inherit typography — **all from real ISO timestamps** now, so the specimen exercises the parse→diff→format path instead of hand-typed strings. No longer a fake specimen. Live-tick demonstration is still a preview-loop follow-up.

## Notes

- Biggest cross-target driver: the spec (`packages/contracts/components/src/time_ago.rs`) is the real bottleneck — wrong `text_color_token` (secondary vs primary), wrong `live` default (`false` vs `true`), and missing `interval`/`tooltipFormat`/`timezone`. Fixing the spec unblocks the color-token todo in both Rust targets at once.
- GPUI's epoch math (`datetime_to_epoch`, `time_ago.rs:212-231`) is a hand-rolled proleptic-Gregorian conversion — fine functionally, but the week-tier threshold bug is the visible parity break.
- Contract §6 ARIA (`<time datetime=…>` + combined `aria-label`) is met only by Svelte; both Rust targets are accepted-delta on semantics. The native-`title` vs `Tooltip` wrapper mismatch is a contract-staleness issue, not an implementation bug.
- Live updating is unimplemented in all three Rust paths (component + preview). Treat as a single cross-target follow-up rather than per-target churn.
