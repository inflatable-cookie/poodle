<!-- parity consv=fixed gpui=4 jetstream=4 specimen=gap -->
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

Spec/code-side (not contract↔Svelte; left for Rust pass — docs-only task):
- [ ] (spec) `TimeAgoSpec::text_color_token()` returns `COLOR_TEXT_SECONDARY` (`time_ago.rs:58-60`); contract/Svelte want `text-primary`. Spec is wrong — fix in code.
- [ ] (spec) `TimeAgoSpec` lacks `interval`/`tooltip_format`/`timezone` (`time_ago.rs:6-14`); contract §3 has them. Add fields in code.
- [ ] (spec) `TimeAgoSpec::default()` sets `live: false` (`time_ago.rs:20`); contract/Svelte default `true`. Fix default in code.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded font size `el.text_size(px(rem_to_px(0.875)))` at `time_ago.rs:80` — `0.875` is a raw literal. Resolve from `spec.font_size_token()` via the theme, like Jetstream does. ZERO hardcoded rem values allowed.
- [ ] Renders `text-secondary` (spec returns `COLOR_TEXT_SECONDARY`, applied at `time_ago.rs:70`) — contract/Svelte want `text-primary`. Blocked on the spec `text_color_token()` fix above.
- [ ] Threshold divergence: GPUI inserts a **week** tier (`WEEK`, `"w"/"week"`) between days and months (`time_ago.rs:158,176-184,193`). Svelte/contract go straight from days (<30d) to months — no weeks. Months are also computed as `seconds / MONTH` only in the `< YEAR` branch, so a 10-day diff falls into the week branch and prints e.g. `"1w ago"`, which Svelte never produces. **Fix: drop the week tier; match the 6-row threshold table.**
- [ ] No live re-render: `live` is stored (`time_ago.rs:45-48`) but `into_element` computes once against `SystemTime::now()` (`time_ago.rs:145`) with no timer/frame loop. Contract §10 requires periodic re-render when `live=true`. Preview has no timer wiring either.
- accepted: no `<time>`/`datetime`/`aria-label` (gpui has no semantic-element or accessibility API) — renders a plain `div`.
- accepted: no tooltip / absolute-time `title` (no Tooltip primitive wired here).

## Jetstream gap (vs Svelte + contract)

- [ ] **No relative-time computation at all.** `js_time_ago` labels `spec.timestamp` verbatim (`time_ago.rs:11`). It only "works" because the specimen passes pre-formatted strings like `"2 minutes ago"`. Pass a real timestamp and it prints the raw ISO string. Contract §10 (and GPUI) require replicating the threshold table. **Fix: port the `format_duration` logic (parse timestamp → diff → relative string).**
- [ ] Renders `text-secondary` (spec `text_color_token()` applied at `time_ago.rs:10`) — contract/Svelte want `text-primary`. Blocked on the spec fix above.
- [ ] No live re-render: `spec.live` is never read; no timer in component or preview `main.rs`. Contract §10 requires periodic update when `live=true`.
- [ ] No tooltip / absolute-time affordance — no `title`/Tooltip equivalent.
- accepted: no ARIA / semantic `<time>` element (Jetstream has no accessibility channel).
- note: token resolution itself is clean — `resolve_px(theme, spec.font_size_token())` (`time_ago.rs:16`), no hardcoded literals. The gaps are behavioral, not token violations.

## Specimen parity

- Svelte covers: Recent (2m/3h/2d), Future (+5m), Long format (2m/2d), Static (`live=false`), Inherited typography (inline prose), From ISO string (`TimeAgoSpecimen.svelte`).
- GPUI covers: Recent (2m/3h/2d), Future (+5m), Inherit typography, Long format (2m/2d), From ISO string, Static (no live update) — **all from ISO timestamps**, so it actually exercises the relative-time math (`time_ago_specimen.rs`). Closest to Svelte. — missing: nothing material; static group can't demonstrate the live/no-live distinction since live isn't implemented.
- Jetstream covers: Timestamps (2 entries + "Yesterday"), Inherit typography (`time_ago.rs`). — missing: **Future**, **Long format**, **Static**, **From ISO string** groups. Worse, every value is a **hand-typed relative string** (`"2 minutes ago"`, `"Yesterday"`), not a timestamp — this is a fake specimen per CLAUDE.md "No Mockups": it hides that `js_time_ago` does no formatting. **Fix: feed ISO timestamps once the component computes relative time.**

## Notes

- Biggest cross-target driver: the spec (`packages/contracts/components/src/time_ago.rs`) is the real bottleneck — wrong `text_color_token` (secondary vs primary), wrong `live` default (`false` vs `true`), and missing `interval`/`tooltipFormat`/`timezone`. Fixing the spec unblocks the color-token todo in both Rust targets at once.
- GPUI's epoch math (`datetime_to_epoch`, `time_ago.rs:212-231`) is a hand-rolled proleptic-Gregorian conversion — fine functionally, but the week-tier threshold bug is the visible parity break.
- Contract §6 ARIA (`<time datetime=…>` + combined `aria-label`) is met only by Svelte; both Rust targets are accepted-delta on semantics. The native-`title` vs `Tooltip` wrapper mismatch is a contract-staleness issue, not an implementation bug.
- Live updating is unimplemented in all three Rust paths (component + preview). Treat as a single cross-target follow-up rather than per-target churn.
