# g14.007 — HistoryCenter composite proof

Status: complete — all three active runtimes executing
Date: 2026-08-15
Card: `docs/roadmaps/g14/007-history-center-composite-proof.md`

## What landed

The first host-coordinated composite in the conformance kernel. HistoryCenter
owns no history, validates no protocol rule, and decides nothing about undo:
data arrives as `pages`, every operation leaves as a named command, and the
answer comes back as host-supplied records. The corpus exercises that boundary
rather than pretending across it.

- **Portable interface** — `packages/core/src/conformance/history-center.ts`.
  20 parts, 8 commands, 2 host record channels, composite profile.
- **Case corpus** — 23 cases covering empty, linear, page-seam, current entry,
  open/dismiss, undo/redo, busy, loading/failed/rejected, roving focus,
  multiple-fork, fork-run navigation, selection, checkout, rename with
  Escape-cancel, single fork, nested fork, uncapped depth, narrow layout.
- **Rust authority** — record types and the generated `HistoryCenterSpec`.
- **Behaviour core** — `poodle-headless::history_center`: the flat visible-row
  derivation and the machine, 20 tests.
- **Native composition** — `poodle-render::history_center`, 10 tests.
- **Execution** — Svelte, React and GPUI each run all 23 cases and pass. All
  three suites gate: the web ones through `conformance:test-web`, the native
  one through the headless completion board.
- **Planted failures** — flattened hierarchy, wrong command payload,
  unfocusable row, unbounded list, and an inert disclosure each fail the
  corpus's own machinery.
- **Specimen** — the native composition is reachable from the GPUI preview
  catalogue, driven by the preview's toggle state through the same handlers
  the conformance host uses.

## Reusable vocabulary delta

Five kernel extensions, all generic. No component identifier entered a runner,
an observer, or the codegen.

| Extension | What it buys | Why a composite needed it |
| --- | --- | --- |
| Nested collection fields | A collection prop's field may itself be a collection, one level deep | The page → entry shape the authority hands over survives into the fixture instead of being flattened into something the component never sees |
| Repeat sources | `repeat` keys over several record sources into one identity space | A row is a row whether it came from the spine the host passed in or a run the host answered with; a fork entry colliding with a spine id is now an authoring error |
| Host record channels | Declared fixture data a host answers a named command from | Validated identity and keyable parts, without becoming portable props — the component receives them through its callbacks' results, so they generate no Rust spec field |
| Command payloads | `expectEvents` entries may pin the payload a command carried | "Navigate fired" is not the claim; "navigate fired for this branch and this entry" is |
| Hierarchy and scroll observation | `level`, `scrollable`, and a `maxHeight` geometry field | Depth becomes something assistive technology can hear rather than indentation only a sighted reader infers |

Nesting stops at one level and the validators say so: a record inside a record
inside a record is a tree, and a tree in fixture data is the second component
model spec 066 forbids.

**No new primitive capability was needed.** The composite is built from
capabilities the substrate already certifies, which is a stronger result than
adding rows would have been.

## Defects the corpus caught

Not waived — fixed where the defect was.

- **The web runner dropped relative-bounds geometry.** It asserted against a
  fixed six-field computed-style list, so `topGap`/`hStart` and the rest were
  checked on GPUI and silently skipped on Svelte and React. The overlay
  profiles had been authoring them since g14.005.
- **`icon_button` carries no focus style**, and the GPUI backend creates a
  focus handle only for a focusable node that has one. Undo, redo and the
  picker's actions trigger were unreachable by keyboard and unfocusable by the
  backend. The composition stamps its own focus ring; the IconButton gap is
  real beyond this card and needs its own lane.
- **`assert_part` could not assert absence.** A repeated part that expanded to
  no keys reported "not observed by gpui" rather than absent, so "the row is
  gone" was not a claim a case could make. Affects every profile with
  conditional repeated parts.
- **A `contains: "text"` part read only a Text node natively** while the web
  reads `textContent`, making container-wrapped copy invisible. It now reads
  the part's own text children — a decorative spinner beside a status line is
  not the status line's text.
- **Shift was dropped in the rename path**, committing "Wide mix v2" as
  "wide mix v2"; and a key *name* was appended as content, so space typed the
  word and produced "Widespacemixspacev2".
- **Roving focus moved the tab stop without moving backend focus**, and the
  machine's focus effect was dropped on the floor.
- **Escape ownership was inverted.** The rename input and the window's dismiss
  route see the same keystroke; the rename has to claim it first, or cancelling
  a rename closes the whole popover.

## Corrections to the corpus itself

Four, all mine, all caught by running it:

- Fork-run rows were asserted one level too deep. A disclosed run renders at
  its picker's level, one below the anchor.
- The multi-fork fixture made an offered fork `preferred`. The preferred
  continuation is the child already on the list, so an offered fork never is.
- An intermediate `expectEvents` claimed a moment between the request and the
  answer. The host answers in the same flush; no such moment exists.
- `states` were asserted on a non-root part, and listbox options addressed
  while the listbox was closed. Neither could ever resolve.

## Known deltas

- **`continuationsResult` / `runResult` are web-shell props.** The portable
  claim is that the host answers `loadContinuations` / `loadContinuationRun`
  and the answer reaches the picker. How the answer arrives is shell mechanism:
  the web shells take a reference-diffed prop, a native host holds the fork
  tree in its own state. Same boundary as TextInput's DOM vs GPUI editing
  paths. Recorded in the contract's Known Deltas.
- **The native picker composes select and menu semantics from the shared node
  vocabulary** rather than calling `select()` / `menu()`, because those own
  their element ids and cannot carry per-anchor part identity. The web has the
  same constraint: `Select` options are identified by list index today. Both
  close the same way — an instance-id parameter on `Select`.

## Cost

`effigy conformance:cost`: HistoryCenter pilot increment **5,101 LOC** — 1,127
authored authority, 141 generated source, 3,833 harness and runtime deltas —
plus the generated fixture JSON. The generic kernel grew by 55 LOC across the
five vocabulary extensions, which is the number that matters for the card's
"extensions, not a second architecture" test: the composite cost sits almost
entirely in its own authority, renderer and behaviour core, not in shared
machinery.

The increment is the largest of the pilots by a wide margin, and honestly so:
HistoryCenter had no native implementation at all before this card. Roughly
1,850 of the 3,833 delta LOC are the behaviour core and renderer that any
native HistoryCenter would have needed with or without conformance.

## Defects the web lane caught

Running the corpus against the shells found three more, all shipped:

- **The failed-status row only rendered when the list was empty**, in both web
  shells. A history that loaded entries and then failed to load more showed
  nothing at all — no message, no spinner, a list that quietly stopped growing.
- **`Popover` restored focus to its trigger wrapper**, which with
  `triggerIsInteractive` is not a button. After dismissing, focus sat on
  something the operator could not activate: press Enter, nothing happens.
  Present in Svelte and React alike.
- **`Select` options and `Menu` items carried no value attribute.** An option
  identified only by `${listboxId}-option-${index}` cannot be addressed stably
  by anything — a conformance corpus, a consumer's tests, or an automation
  script. `Tabs` already exposed `data-value`; these now match it.

Three of the session's defects existed in one runtime of a pair and not the
other (the web observer's absence gap, the failed-status row, the Popover focus
target). That is the pattern worth carrying forward: a fix to one web shell or
one observer is not done until its pair is checked.

## Cross-runtime observability limit

The bounded-list case asserts that the list scrolls, not the height it caps at.
The web expresses the cap as `min(28rem, 60vh)`, which is not a resolvable
computed length outside a real viewport. A runtime that reports a number and
one that cannot are not disagreeing about the component — they are disagreeing
about what a stylesheet means without layout, and a case that asserted it would
fail for a reason that is not about HistoryCenter.

## Baseline

`cargo test --bin poodle-preview` has three pre-existing
`contract_usage_docs` failures. They fail identically with this card's changes
stashed out and are untouched by it.

## Open

- React specimen parity for the native composition's catalogue entry.
