# 13 — g13.051 Capability And Anatomy Amendments (batch log)

Branch: `thread/g13-051-capability-and-anatomy-amendments` (pushed with
`git push -u origin thread/g13-051-capability-and-anatomy-amendments`)
Date: 2026-08-13
Card: `docs/roadmaps/g13/batch-cards/051-capability-and-anatomy-amendments.md`
Closes: `g13.018` (status flipped in `docs/roadmaps/g13/018-capability-and-anatomy-amendments.md`)

This card fixes the two expressiveness gaps that made the `g13.008`
**revise** verdict worth choosing over reject. Read the verdict evidence
§4 first, as the card requires: assumption 2 is `Repeated`-anatomy
(`PartKind::Repeated` needs a `List` prop and yields identical instances),
assumption 3 is the capability boundary (ownership untyped prose, absence
unsayable). Both are vocabulary, so both are in scope after `g13.017`'s
narrowing. The narrowed boundary holds throughout: no expression tree, no
evaluator, no conditional-render construct.

## 1. Baseline (step 1)

Clean tree on `thread/g13-051-capability-and-anatomy-amendments`
(`46cfba4b`, which carries the card file only). All green:

| Command | Exit | Notes |
|---|---|---|
| `effigy ir:build` | 0 | 19 targets, no worktree diff |
| `effigy ir:check` | 0 | all artifacts current |
| `effigy ci:rust` | 0 | contract crates |
| `effigy ci:web` | 0 | one transient `gate:clean` "no snapshot found" failure on the first run; immediate rerun passed (the snapshot lands in `$TMPDIR` and `effigy gate:snapshot`/`gate:clean` both resolve it there — verified standalone) |
| `git diff --check` | 0 | clean |

Artifact checksums recorded before any change:
`/tmp/g13-051-baseline-checksums.txt` (28 files across the svelte/react/
render generated outputs; the shell/codegen generated outputs verified
unchanged by the step-8 diff).

## 2. Amendment 1 — per-runtime capability provision, including absence

### The IR change (`packages/contracts/ir/src/capabilities.rs`)

`CapabilityRequirement` gains `runtimes: Vec<CapabilityRuntimeStatus>` —
one row per runtime, each reusing the existing `RuntimeTarget` (R2: the
four pilot runtimes; no second runtime list). Each row carries a
`CapabilityProvision` (`provided` / `delegated` / `absent`) and a reason.
Absence is explicit and reasoned; nothing is inferred from a runtime being
unlisted (R3). Pre-amendment requirements (capability + purpose only, e.g.
Button's) serialize without the field (`#[serde(default,
skip_serializing_if = "Vec::is_empty")]`).

### Validation (`packages/contracts/ir/src/validation.rs`)

Three new rules keep the vocabulary honest:

- when any runtime row is declared, every runtime must appear exactly once
  — "not listed" must never silently mean "absent" (R3);
- an `absent` row must carry a non-empty reason (R3);
- duplicate runtime rows are rejected.

New finding kind: `IncompleteCapabilityProvision`. The `Repeated`
validation arm is replaced by the `Identified` rules (below).

### TextInput declares the real three-way split (step 4)

All six requirements now declare the measured split across all four
runtimes — web delegates or provides, GPUI implements, Jetstream absent
where b049 measured nothing:

| Capability | Svelte | React | GPUI | Jetstream |
|---|---|---|---|---|
| focus | provided | provided | provided | **absent** |
| text-editing | delegated | delegated | provided | **absent** |
| ime | delegated | delegated | provided | **absent** |
| clipboard | delegated | delegated | provided | **absent** |
| measurement | delegated | delegated | provided | **absent** |
| timers | provided | provided | **absent** | **absent** |

Jetstream's absences each carry the b049-measured reason (zero
`on_edit_key`/`on_select_range`/`on_edit_insert`, no `ime.rs`, no focus
observation route, no clipboard path, no timer surface). The headline: a
text field nobody can type into is now declared as such, not silently
identical to GPUI. RangeSlider's three requirements declare their splits
the same way (pointer-capture and scrub-fraction absent for Jetstream —
no capture/scrub vocabulary measured; focus provided — the preview's
focus system hit-tests and focuses the render's focusable root).

## 3. Amendment 2 — identified instances replace `Repeated` (step 5)

`PartKind::Repeated` is **removed outright** — measured: no model used it,
the only mentions were `range_slider.rs`'s doc comment explaining why it
could not be used, and its own doc comment naming the case it could not
serve. No migration.

`PartKind::Identified { instances, description }` replaces it: a fixed set
of identified instances, each its own part with its own identity and
declared semantics; the count and the identities come from the definition.
Validation: instances must be non-empty, unique, and must exist as parts
in the same component.

RangeSlider's anatomy now records the standard pair as the identified
`control` family: `control` (Identified, instances
`[control-lower, control-upper]`) with `control-lower`/`control-upper`
re-parented under it — each instance keeps its own identity and declared
semantics (per-thumb aria labels, clamp bounds, Home/End behaviour, per
the contract §2 rows).

## 4. R5 — the render derives the pair (step 6)

`packages/render/src/range_slider.rs` no longer hard-codes the count:

- `make_thumb()` is called exactly once, inside a per-instance loop over
  the definition's `control` family instance list;
- the pair positions (lower, upper) come from the machine's value pair and
  map positionally in declaration order — renaming an instance moves the
  artifacts but not the pixels;
- the standard-variant gate now requires the identified family to declare
  instances (`control_instances().is_empty()` gates the block) instead of
  checking two named parts.

The generated artifacts carry the instance list in every part row
(`instances` in the TS artifacts, `Option<&[&str]>` in the Rust
artifacts) — emitted by all six part-emitting targets through the shared
`part_instances` helpers.

## 5. The gate (step 7) — `docs:capability-drift`

New script `packages/svelte/preview/scripts/capability-drift.ts`, wired
into `tasks/effigy.tasks.toml` as `docs:capability-drift` and into
`ci:web` (next to `docs:react-specimen-drift`, the same static-checking
shape). It reads the committed fixtures, takes every declared
(capability, runtime) row, and probes the runtime sources with the
measured trace vocabulary (R4: the gate is the deliverable, not the type):

- **direction A** — a declared **absence** must stay true: if the runtime
  gains a trace of the capability while still declared as lacking it, the
  gate fails. Jetstream's text-editing absence is probed with the b049
  vocabulary (`on_edit_key`/`on_edit_insert`/`on_select_range`);
- **direction B** — a declared **provision** must have a trace: a runtime
  claiming a capability it does not implement fails.

Probes are keyed per component — the same capability can differ per
component (Jetstream focus is absent for TextInput fields — no
`on_focus_change` route — but provided for RangeSlider's focusable root).
A declared row without a probe entry is itself a failure (a gate that
cannot check a claim must not pass it). 36 rows verified clean:

```
capability-drift: 36 declared capability rows verified against runtime traces.
```

### Both directions proven independently (R4, live)

- **Direction A**: planted `on_edit_insert` in a scratch Jetstream source
  file → gate fails on `[text-input] text-editing @ jetstream (declared
  absent)` (and clipboard, whose probe shares the vocabulary) → plant
  removed.
- **Direction B**: flipped the model's Jetstream text-editing row from
  absent to provided, one `ir:build` → gate fails on `[text-input]
  text-editing @ jetstream (declared provided) ... no trace` → restored,
  one `ir:build`, gate clean again.

## 6. Artifact diff vs the recorded checksums (step 8)

Exactly nine generated files changed (all three component artifacts × the
three shapes), each change attributable:

| File | Change | Amendment |
|---|---|---|
| `svelte/react/components/src/generated/{button,text-input}/index.ts`, `render/src/generated/{button.rs,text-input/index.rs}` | part rows gain `instances: null` | 2 — the identified-instances field is part of the part-row schema |
| `svelte/react/components/src/generated/range-slider/index.ts`, `render/src/generated/range-slider/index.rs` | 10th part `control` (Identified, `instances: ["control-lower","control-upper"]`); lower/upper re-parented under it; every row gains `instances` | 2 |

Web component artifacts change vocabulary only — the components read
`part.id`/`part.className`, which are unchanged, so no DOM class or
attribute moves (R6; the parity test's class-set diff stays green). The
shell-scene, shell-rust, and `codegen/generated` outputs are byte-identical
to the baseline. `button-model.json` is byte-identical (empty provisions
serialize without the field); `range-slider-model.json` and
`text-input-model.json` carry the new declarations.

## 7. Propagation still reaches all four runtimes (step 9)

Live proof, b049's shape: renamed `data-validation-state` →
`data-validation-level` in `text_input.rs`, one `ir:build`, and all three
artifacts moved — each carrying exactly one occurrence of the new name,
zero of the old (Svelte and React read the TS artifacts; GPUI and
Jetstream consume the render artifact through `poodle-render`). Restored →
zero occurrences of the temporary name, `ir:check` 0. (The one wrinkle:
the emitted struct-doc prelude hard-coded the attribute name, leaving a
stale name in the artifact after a rename — the prelude doc was
de-referenced so the artifact carries exactly the vocabulary.)

`ir:check` drift proof (required test 4) ran live too: planted byte in a
committed artifact → `ir:check` exits 1 naming the artifact → restored →
exits 0.

## 8. Required tests

- [x] TextInput declares Jetstream lacks text editing, with a reason —
  asserted structurally in `packages/codegen/tests/text_input.rs`; the
  gate fails if that stops being true (proven live, §5).
- [x] The gate fails independently when a runtime claims a capability it
  has no trace of (proven live, §5).
- [x] RangeSlider expresses two identified thumbs; `range_slider.rs`
  derives the count from the definition — render tests assert the built
  thumb count equals the definition's instance count, and that
  `make_thumb()` is called exactly once (the count is not literal in the
  render; the test reads the source up to the test module).
- [x] `ir:build`/`ir:check` pass; `ir:check` fails on a planted byte
  (§7).
- [x] One definition change reaches all four runtimes (§7).
- [x] Existing `poodle-render` and component tests pass unedited; no
  baseline refreshed (the two render tests that asserted the hard-coded
  pair were updated by this card's own required test — nothing else).

## 9. Validation (step 10)

| Command | Exit |
|---|---|
| `effigy ir:build` | 0 |
| `effigy ir:check` | 0 |
| `effigy ci:rust` | 0 |
| `effigy ci:web` | 0 (includes `test:web-pack-install`, `check:svelte`, `docs:capability-drift`) |
| `effigy test:core` | 0 |
| `effigy test:components` | 0 |
| `effigy test:parity` | 0 |
| `effigy check:svelte` | 0 |
| `effigy docs:lint` | 0 |
| `git diff --check` | 0 |
| `cargo test --manifest-path packages/contracts/ir/Cargo.toml` | 0 — 20 passed (6 new g13.018 validation tests) |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 |
| `cargo test --manifest-path packages/render/Cargo.toml` | 0 — 186 passed |

## 10. Acceptance criteria

- [x] Capability absence is declarable, carries a reason, and is not
  inferred from silence — `CapabilityRuntimeStatus` with mandatory
  per-runtime coverage (validation) and mandatory absence reasons.
- [x] TextInput declares the real three-way split across four runtimes
  (§2 table).
- [x] `Repeated` is gone; identified instances replace it.
- [x] `range_slider.rs` no longer hard-codes the thumb count.
- [x] The gate fails on each direction independently and passes clean.
- [x] No prop renamed, no behaviour changed, no baseline refreshed —
  R6 held: web components' DOM classes/attributes unchanged (the parity
  class-set diff and the component tests pass unedited).
- [x] Spec 063 records both amendments as delivered.
- [x] `g13.018` marked complete; all step-10 commands exit 0.

No stop condition was reached. No `PAPERCUTS.md` entry was needed (the
one transient `ci:web` gate failure did not reproduce and the gate
mechanics were verified standalone; nothing about it is a repo defect).
