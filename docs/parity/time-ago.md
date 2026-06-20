<!-- parity consv=gap gpui=4 jetstream=4 specimen=gap -->
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

Prop/anatomy/state divergences. For each: what differs, which side is right
(Svelte unless it's missing contract-specified functionality), and the action.

- **Color token mismatch.** Contract §8 + Svelte `.poodle-time-ago` (`TimeAgo.svelte:154`) use `--poodle-color-text-primary`. `TimeAgoSpec::text_color_token()` returns `COLOR_TEXT_SECONDARY` (`time_ago.rs:58-60`), so BOTH Rust targets render secondary. **Fix: spec must return `COLOR_TEXT_PRIMARY`.** (Spec is wrong, not Svelte/contract.)
- **Anatomy: Svelte is wrapped in `Tooltip`, not a native `title`.** Svelte renders `<Tooltip content={absoluteText}>` around the `<time>` (`TimeAgo.svelte:141-150`); there is no `title` attribute. Contract §2/§6/§8 specify a native `title` attribute. Svelte is authoritative. **Fix: update contract anatomy + §6/§8 to describe the `Tooltip` wrapper instead of a native `title`; note GPUI/Jetstream tooltip is a deferred delta.**
- **`cursor` divergence.** Contract §7/§8 say `cursor: default`. Svelte sets `cursor: help` (`TimeAgo.svelte:164`). Svelte authoritative. **Fix: contract → `cursor: help`.**
- **Dotted-underline affordance not in contract.** Svelte adds `text-decoration: dotted underline` with `--poodle-time-ago-underline{,-hover}` color-mix vars + a hover/focus-visible color transition (`TimeAgo.svelte:155-178`). Contract §8 root table omits all of it. Svelte authoritative. **Fix: add underline + hover treatment to contract §8.**
- **`tooltipFormat` value semantics drift.** Contract §8 says `title` is always the `datetime` (`toLocaleString` y/mon-short/d/h/m). Svelte's `formatAbsolute` branches on `tooltipFormat`: `date` → `toLocaleDateString`, `full` → adds `second` + `timeZoneName: "short"`, `datetime` → long month + h/m (`TimeAgo.svelte:81-123`). Contract table is stale. **Fix: document the three-branch `formatAbsolute` in §8.**
- **`short` default contradiction inside the contract.** Contract §3 props table says `short` default `true`; §4 "just-now" row and the formatting table both list `"now"`/`"just now"` as if both are reachable. Svelte: `short=true` → `"now"`, `short=false` → `"just now"` (`TimeAgo.svelte:43`). Consistent with Svelte; **contract §4 just-now row is fine but should clarify short=`"now"` / long=`"just now"`.**
- **`yesterday` long-form special case missing from contract.** Svelte returns `"yesterday"` when `!short && !isFuture && days === 1` (`TimeAgo.svelte:64`); contract §4 mentions `"yesterday"` in passing (line 58) but the §8 formatting table has no row for it. Svelte authoritative. **Fix: add the long-form `days===1 → "yesterday"` rule to §8.**
- **Spec missing contract props.** Contract §3 lists `interval` (30000), `tooltipFormat` (`"datetime"`), `timezone` (null). `TimeAgoSpec` (`time_ago.rs:6-14`) has none of them. **Fix: add `interval`, `tooltip_format`, `timezone` to the spec** (needed before Rust targets can reach parity).
- **Spec `live` default wrong.** Contract §3 + Svelte (`TimeAgo.svelte:6`) default `live=true`. `TimeAgoSpec::default()` sets `live: false` (`time_ago.rs:20`). **Fix: spec default → `true`.**

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
