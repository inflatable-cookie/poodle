# g12.010 Agent Composer Family

Status: complete (2026-07-25)
Owner: Poodle core
Depends on: `g12.001` (shared stylesheet), `g12.008` (React parity), the
`FilterBuilder` precedent for host-declared vocabulary

## Why

Consuming apps (Loophole, Underlay, future agent surfaces) all need the same
thing at the bottom of a conversation view: a message composer with a model
control, a context-budget indicator and one button that submits or stops. Every
app was going to hand-build it. Poodle owns that shape.

The reference composer splits model and reasoning across two separate pickers;
the requirement here was one cohesive widget covering model **plus** its
capability axes.

## Scope

Three deliverables, all four targets (Svelte, React, GPUI, Jetstream):

1. **`Meter` `shape="ring"`** — the context indicator. Context usage is a
   bounded measurement in a known range with `low`/`high` hints, so it extends
   `Meter` rather than `Progress` (task completion) or a new component. Adds
   `shape`, `tone`, `showValue`, `valueText`, a derived `level`, and the
   threshold-driven warning fill the contract had listed as a future follow-up.
2. **`ModelPicker`** — one popover choosing a model *and* its capability axes.
   Axes are host-declared (`select` level sets or boolean `toggle`s) and may be
   scoped to a subset of models; the trigger shows a combined summary
   (`High · Fast · 1M`). No vendor vocabulary: "reasoning", "fast mode" and
   "context window" are host data, exactly as filter fields are for
   `FilterBuilder`.
3. **`AgentChatInput`** — the composer: auto-growing editor, attachment chips,
   a host-composed toolbar region (canonically holding a `ModelPicker`), the
   context ring, and the submit ↔ stop action button.

Non-goals: transcript rendering, transport, slash-command / mention
autocomplete, prompt history, attachment previews.

## Decisions

- **Ring lives on `Meter`, not `Progress`.** Meter already carries
  `low`/`high`/`optimum` and "bounded measurement" semantics. Ring geometry is
  pure CSS (`conic-gradient` + `mask`), no SVG.
- **Axes are data, not props.** A fixed `reasoningLevel` + `fastMode` API would
  have been simpler and useless to the second consumer.
- **Axis config is keyed; models reference keys.** The first cut scoped axes with
  a `models: string[]` list on the axis, which forces one axis declaration per
  (provider × axis) combination. Cross-harness apps list several providers at
  once, so scoping moved onto the model: `model.axes` names the keys it exposes,
  and a binding can override the level set for that model while keeping the
  shared key. Hosts still read `selection.axes.effort` across providers.
- **The action button is a bespoke `<button>`**, not `IconButton` — the
  circular accent treatment plus dual-state semantics are composer-specific.
  Everything else in the composer reuses a primitive.
- **Submitting never clears the editor.** A failed send must not lose text.
- **Keyboard submit is dropped while busy.** Stopping is deliberate; a stray
  Enter must not abort a run.
- **The picker's panel is two columns** — models left, capability axes in a
  right-hand rail — whenever the selected model has applicable axes. Stacking
  them pushed the axes below the fold of a scrolling model list; side-by-side
  keeps the whole configuration on one screen (panel height dropped ~40%).
- **The picker's surface opens upward** (its home is a bottom-pinned toolbar)
  and flips down when there is no room above.

## Accepted Deltas

- GPUI and Jetstream render the ring as a circular stroked track plus the value
  readout: neither runtime exposes conic gradients or arc geometry. Measurement
  semantics are unchanged (`meter.md` §12).
- Native editor auto-grow is line-count based (`visible_rows()`), not text
  measurement; text editing itself stays host-event-loop work, matching every
  other native component.

## Cross-Target Audit (2026-07-25)

Diffed the three non-reference targets against Svelte:

- **React: 1:1.** Same prop surface (ModelPicker 14, AgentChatInput 28), same
  emitted `poodle-*` classes (26 / 13), same data attributes, same test names
  (13 / 7). The glob-driven parity gate covers the collapsed anatomy; the
  per-component suites cover the open panel and both attachment variants.
- **GPUI and Jetstream: identical to each other** — both read the same 37
  composer spec surfaces and the same 38 picker surfaces, including `emphasis`,
  `variant`, per-model axis resolution, the segmented/list control split,
  thumbnails and images.
- **One real gap found and fixed:** neither native target dimmed a disabled
  attachment (the web gets it free from the remove `IconButton`). Both now apply
  the disabled opacity to the chip or tile.

Everything else unreferenced natively is ARIA (`aria_label`, `submit_label`,
`stop_label`) or interaction-only (`submit_on_enter`, `allow_empty_submit`,
`max_length`, `is_read_only`) — the standing render-only posture, already in
§Accepted Deltas.

## Verification

- `poodle-specs`: 18 new unit tests (submit gating, gesture table, axis scoping,
  selection normalisation, level derivation, ring ladder)
- `test:components`: 572 green, including new `AgentChatInput` / `ModelPicker`
  suites in both frameworks; the glob-driven smoke, parity and axe gates picked
  the components up automatically
- `check:gpui`, `check:jetstream`, both native previews: build-verified;
  Jetstream component probes assert placeholder/value, chips, dividers and the
  ring's conditional render
- Headless Playwright over both web previews: specimen pages render, the picker
  opens and flips, React and Svelte match

## Next Task

None — the family is complete. `g12.009` (visual regression gate) remains the
generation's open seam, and will pick these slugs up automatically.
