# g16.119 — A1 focus and state semantics

Status: implementation complete — pending independent exact-head review
Date: 2026-09-05
Card: `docs/roadmaps/g16/119-a1-focus-and-state-semantics.md`
Base: `origin/main` at `ef483d029` (dispatch manifest revision 19)
Branch: `worker/g16-119-a1-focus-state-semantics`
Repin: `e2630da998d13466a5de8ff266f0f1e9dc371e13`, lock digest
`c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`

## Outcome

Five rows executed through the mounted A1 runner. One reaches an empty diff
and emits a receipt; four keep a recorded divergence with the ruling that
holds them there.

| Row | Repaired | Left recorded |
| --- | --- | --- |
| AgentQuestion | option group `labelled_by` the prompt node; post-action focus lands on the answered radio | nothing — empty diff, receipt emitted |
| Menu | opening moves focus to the first enabled item through production dispatch | node 5 `focus_order`: Svelte makes every enabled item a tab stop |
| RadioGroup | stopped projecting `orientation` | nodes 1–2 `focus_order`: extractor counts both native radios |
| SegmentedControl | stopped projecting `selected` | nodes 1–2 `focus_order`: same extractor cause |
| AgentTranscript | nothing to repair | node 0 `focused`: happy-dom focuses a clicked `role="log"` |

Rulings, with both snapshots and the exact attributes, are in
`docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/README.md`. In
short:

- RadioGroup and SegmentedControl: a real browser gives a native radio group
  one roving tab stop, which is what GPUI projects.
  `test/nucleus-a11y/extract.ts` does not model that, so it lists every
  enabled radio. Teaching the extractor native radio-group semantics is
  outside this card.
- Menu: Svelte `MenuSurface` renders each item as a plain `<button>` with no
  `tabindex`, so every enabled item is a sequential stop. `menu.md` §6 names
  focus entry and highlighted-item movement but states no tab-stop rule, so
  the contract does not decide it. Question returned rather than repaired —
  this is the card's stop condition.
- AgentTranscript: `agent-transcript.md` §"Focus And Announcement" states the
  transcript never takes focus. GPUI is correct; the Svelte value is the
  extractor focusing a clicked container. Making the log a tab stop to match
  would contradict the contract.

## Backend and render changes

- `packages/gpui/node-backend/src/interaction.rs`: a menu whose first
  sequential item is a tab stop requests focus once, while the window holds
  no attributed focus. Production dispatch, not a test shim; re-queuing after
  another item is focused would steal arrow-key movement every frame.
- `packages/render/src/agent_question.rs`: the prompt gets a semantic id
  (`agent_question_prompt_id`) and the option group is `labelled_by` it,
  matching Svelte's unresolved `aria-labelledby` target.
- `packages/render/src/radio_group.rs`: `a11y.orientation` removed per
  `radio-group.md` §6; §10 of that contract still claimed the opposite and is
  corrected to match §6 and Svelte.
- `packages/render/src/segmented_control.rs`: `a11y.selected` removed;
  segments are native radios and carry checked semantics.

## Out-of-card work, flagged

Three A1 rows — `callout`, `editable-label`, `text-input` — carry committed
A1 receipts and committed GPUI snapshots, but no probe for them exists in any
commit (`git log -S` over all history finds none); the tranche merges dropped
them. Without a probe the cohort cannot be re-emitted at a new head, which
this card's repin requires, and the only alternative was hand-editing three
receipts that cannot be re-run. The probes were restored instead. Each one is
checked against its committed `<row>.gpui.json` and reproduces it
byte-for-byte, so the receipts stay executed evidence. Recorded in
`PAPERCUTS.md` with the validator gap that let it happen.

## Validation

- `effigy regressions:native` — 224 passed, 1 ignored
- `POODLE_NUCLEUS_RECEIPT_DIR=... cargo test ... select_a1 -- --ignored` —
  1 passed (the Select A1 probe is still `#[ignore]`d on a stale `g16.111`
  reason; without this second run its receipt keeps the old pin — papercut
  filed)
- `effigy test:nucleus-a11y` — 30 passed
- `effigy test:a11y` — 179 passed
- `effigy test:nucleus-parity-receipts` — 11 passed
- `effigy test:parity-evidence-ledger` — 6 passed
- `effigy check:parity-evidence-ledger` — 176 rows validated
- `effigy docs:check` — passed
- `effigy docs:react-prop-drift` — 176 checked
- `effigy ci:web` — 386 files, 3740 tests passed (one 5s parity timeout on the
  first run under concurrent load; green on re-run and in isolation)
- `git diff --check` — clean

Cohort re-emitted at the repin: 29 M1 receipts and 13 A1 receipts, plus the
four divergence stores. The ledger moves exactly one cell — AgentQuestion
GPUI accessibility, `missing` to `mounted` (12 to 13). No windowed selector
was run.

Known red, not from this lane: `cargo test --manifest-path
packages/render/Cargo.toml` fails two cases on `main` at `ef483d029`
(`context::tests::the_provider_adds_no_wrapper_node_layout_or_accessibility_entry`,
`segmented_control::tests::icon_only_without_an_icon_keeps_the_visible_label`).
Neither touches a line this card changed. Filed in `PAPERCUTS.md`.

## Review state

One focused PR is pushed and the worker stops. Merge and independent
exact-head review remain orchestrator-owned.
