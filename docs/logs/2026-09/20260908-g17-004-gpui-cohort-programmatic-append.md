# g17.004 — GPUI cohort programmatic append replay

Status: complete — merged in PR #231 at `8bd95d3a2cdf8c86edacb450cc33a0a4d02b9983` on 2026-09-08
Date: 2026-09-08
Card: `docs/roadmaps/g17/004-gpui-cohort-programmatic-append.md`
Base: `origin/main` at `fdb9cd5acaba299a22bf0f819da872b99e5676be`
Implementation commit: `2cf135d1820068f331b5ec908013dc7ba2ad0f61`
Repin commit: `585ee5cfa0dd56f71c2be6b862fbff982cb54648` (docs-only;
Nucleus cohort evidence payloads differ only in `source_commit`)
Branch: `ns-32f0bd22-428b-4fdf-93a6-cf257d0768df` (queue-owned)
Review: accepted independent exact-head `ready_to_merge` verdict in [review
comment #5585250834](https://github.com/inflatable-cookie/poodle/pull/231#issuecomment-5585250834)
at head `acfb0bdef1e17c5a21cd4fdc705c6f123ec8b955`; merge commit:
`8bd95d3a2cdf8c86edacb450cc33a0a4d02b9983`

## Outcome

`poodle-window-capture --cohort` now consumes the complete pinned Nucleus A1
action vocabulary. The GPUI after-actions replay for AgentTranscript appends
the declared transcript item once and remounts before capture, matching the
Svelte and React hosts. All changes are confined to
`packages/gpui/preview/src/bin/window_capture/cohort_capture.rs` plus this
log and the Lab adoption request.

- `Action` gains the closed `ProgrammaticAppend { item: Value }` variant.
  Unknown action types, extra action fields, and a missing `item` stay
  rejected by the same `deny_unknown_fields` deserialisation.
- The cohort `HostState` now carries explicit `transcript_items`, projected
  from the scenario props at `initial_state`. The AgentTranscript renderer
  builds from that state, so the initial capture is unchanged (same items,
  same order, same spec) and an append cannot leak into it. A declared
  AgentTranscript row still fails closed when its props carry no items.
- One shared closed mapping, `transcript_item`, turns a scenario value into
  the renderer's `TranscriptItem` (message with id/known role/markdown, or
  activity with id/label). The renderer and the replay validate through the
  same function — no parallel permissive shape was added. Malformed items
  are refused with a typed error before any state is touched.
- On after-actions replay, `ReplayController` accepts the action only when
  the cohort component is `AgentTranscript`, appends the declared item
  exactly once, remounts, and then waits the same two settle frames every
  other replay action uses. Use on any other component, an unknown item
  kind, or an item missing a required field fails the run before capture or
  receipt publication.
- Pointer/key replay, scenario files, component APIs, receipts, capture
  ordering, scale, and the foreground proof are unchanged. No windowed
  selector was run.

## Oracle evidence (unit tests, no window opened)

| Invariant | Counterexample | Result |
| --- | --- | --- |
| Contract parses | canonical AgentTranscript scenario rejected | `agent_transcript_scenario_parses_with_the_declared_programmatic_append`; `every_scenario_file_loads_from_the_closed_registry` (previously red) |
| Initial is stable | append leaks into initial host state | `initial_transcript_projection_is_scenario_props_only` |
| Append is real | parser accepts but projection unchanged | `one_programmatic_append_appends_the_declared_item_exactly_once` (state order `hello, done, appended`; rendered text appears exactly once) |
| Scope is closed | append attached to another component | `programmatic_append_on_another_component_fails_closed` |
| Item shape is closed | unknown kind or missing required field | `malformed_declared_items_are_refused_before_append`; `unknown_action_types_and_extra_fields_stay_rejected` |
| Existing replay holds | pointer or key action | full capture-target suite green; `every_registered_scenario_builds_through_the_production_renderer` (previously red) |
| Lab can repin | adoption request omits exact contract boundary | `docs/handoffs/20260908-g17-004-lab-adoption-request.md` |

## Validation

- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --bin
  poodle-window-capture --features window-capture` — 71 passed, 0 failed
  (the 12 `cohort_capture::` tests include the two previously red
  load/build regressions and the five new binding tests).
- `effigy regressions:native` — 233 passed, including the headless A1
  `agent_transcript` programmatic-append model.
- `effigy docs:check` — passed after the Nucleus cohort evidence repin
  (manifest plus every committed M1/A1 receipt advanced `source_commit`
  from `93266dad9` to `2cf135d18`; no payload beyond that field changed).
- `git diff --check` — clean.
- One windowless capture-binary build (`cargo build --bin
  poodle-window-capture --features window-capture`) — passed. The
  pre-existing unused-import warning in `transport.rs` is present at the
  base commit too and was left alone.

No `*-windowed` selector was run.

The separately authorized Lab acceptance is deferred: this Poodle closeout
does not claim a fresh full `g01.006` cohort capture. The existing Lab task
must repin to the merge commit, publish all 174 rows, and verify the
AgentTranscript after-actions item exactly once. The reviewer also noted only
non-blocking follow-up: the inner replay `unreachable!()` is intentionally
exhaustiveness-only, and the windowless build retained the pre-existing unused
`transport.rs` import warning. Neither changes the merged acceptance.

## Pre-existing reds (now repaired)

`test/nucleus-a11y/scenarios/agent-transcript.json` declares action type
`programmatic_append`; `cohort_capture.rs`'s scenario action enum only knew
`pointer_activate` and `key`, so the two capture-target load/build tests
failed at the base and the Lab cohort lane rejected the row after 96/174
captures. Both reds were recorded as out of scope by g17.003 and are the
subject of this card.

## Notes

The first worker on this task stopped on a provider quota error with no
callback; its thread made no commits and opened no PR. This run picked the
task up from the committed handoff on the queue-owned branch at the exact
planning base. Reserved for coordinator closeout: `docs/roadmaps/g17/README.md`,
`docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.
