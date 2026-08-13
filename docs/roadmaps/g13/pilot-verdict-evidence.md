# g13.008 Pilot Verdict — Evidence

Status: evidence complete, **verdict not yet recorded**
Milestone: `g13.008`
Owner: Poodle maintainer (the verdict is the maintainer's to record — see
`docs/roadmaps/g13/008-pilot-verdict-and-architecture-promotion.md`)
Compiled: 2026-08-13
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
(pass conditions and stop conditions), `docs/roadmaps/g13/authority-inventory.md`,
`docs/logs/2026-08/11-g13-002-pilot-fixture-and-metrics-freeze.md`
(the frozen baseline), and the six slice logs `b041`/`b042`, `b045`/`b046`,
`b048`/`b049`

Broad migration stays locked until **adopt** is recorded here. Nothing in this
document unlocks `009`–`016`.

## 1. What the pilot proved

Four of spec 063's seven pass conditions hold, and they are not trivial.

| # | Pass condition | Verdict | Evidence |
|---|---|---|---|
| 1 | One Rust definition deterministically updates all four previews | **pass** | Proven live three times, each with a rename plant/restore: `b042` (Button), `b046` (`data-variant` → `data-variant-level`; GPUI 9355-sample diff, Jetstream 9580), `b049` (`data-validation-state`; GPUI 6722-sample diff over exactly three validation fields, Jetstream 32148) |
| 2 | Svelte and React keep idiomatic semantics and public APIs | **pass** | Prop surfaces byte-identical across every slice; verified independently at each merge (TextInput: Svelte 49, React 48, no renames) |
| 3 | GPUI and Jetstream consume the shared Rust path without forks | **pass** | Self-contained generated Rust with zero Poodle-crate imports; `poodle-render` never gained a `poodle-ir`/`poodle-codegen` dependency, asserted by test in each slice (`b003 R2` held throughout) |
| 4 | Interaction, a11y, recipe, size, density and visual evidence pass | **pass, with two honest gaps** | No visual baseline was ever refreshed across six cards. Gaps, both recorded rather than papered over: GPUI caret/selection interactive capture was environment-blocked after six attempts (substituted with render + backend unit evidence), and Jetstream cannot draw a caret at all — which is finding 5 below, not a testing failure |

The drift machinery is genuinely good. `ir:check` fails on a planted byte in a
generated artifact and was proven to do so in every slice. Four-runtime
propagation is real and repeatable.

## 2. What the pilot disproved

### Pass condition 5 fails outright

> *"generated code is smaller and easier to inspect than the duplicated source
> it replaces"*

**Nothing was replaced.** Every consumer grew. Measured against the fixed
pre-pilot baseline `0dd58b80` (the commit before the first slice):

| File | Before | After | Δ |
|---|---|---|---|
| `svelte/components/src/Button.svelte` | 220 | 292 | +72 |
| `react/components/src/Button.tsx` | 164 | 192 | +28 |
| `svelte/components/src/RangeSlider.svelte` | 176 | 251 | +75 |
| `react/components/src/RangeSlider.tsx` | 185 | 251 | +66 |
| `svelte/components/src/TextInput.svelte` | 631 | 720 | +89 |
| `react/components/src/TextInput.tsx` | 553 | 643 | +90 |
| `render/src/button.rs` | 621 | 711 | +90 |
| `render/src/range_slider.rs` | 497 | 665 | +168 |
| `render/src/text_input.rs` | 625 | 912 | +287 |
| **Total** | **3,672** | **4,637** | **+965** |

Not one file shrank. The components the IR targeted are collectively **26%
larger** than before it existed, because consuming a definition is additive:
the component keeps all its behaviour and gains the wiring that reads the
artifact.

### The cost of the machinery

| Component | LOC |
|---|---|
| `poodle-ir` crate (src) | 4,443 |
| `poodle-codegen` authored models | 5,305 |
| `poodle-codegen` emitters (targets) | 4,356 |
| `poodle-codegen` remaining src | 1,135 |
| `poodle-codegen` tests | 5,184 |
| Fixtures (JSON) | 7,398 |
| Generated artifacts, all four runtimes | 2,914 |
| Generated-artifact tests (web) | 670 |
| **Total new** | **≈31,400** |

So: **≈31,400 lines of machinery, plus 965 lines added to the consumers, to
remove zero lines of duplication.**

The ratio inside the machinery is as telling. `models/button.rs` is 1,407
authored lines and emits a 1,444-line Rust artifact — a near 1:1 restatement —
plus a 107-line TypeScript artifact. The web artifacts across all three
components total roughly 400 lines.

### Pass condition 6 is unverified

> *"diagnostics point to the authored definition, not only generated output"*

`ir:check` reports the **artifact** path on drift (`preview-shell.rs`,
`range-slider.rs`). No slice demonstrated a diagnostic naming the authored
definition and the line within it. Not a failure — untested. Recorded as
untested rather than claimed.

## 3. The trend across the three slices

The pilot was designed in increasing difficulty, and the result moved in one
direction the whole way.

| | Button | RangeSlider | TextInput |
|---|---|---|---|
| Props | 34 | 18 | 49 |
| Documented data attributes | 11 | 4 | 3 |
| Props per attribute | ~3:1 | ~5:1 | ~16:1 |
| What the definition carried | vocabulary: part names, `data-*` names, value domains | same, plus geometry hooks | same |
| What it executed | nothing | nothing | **nothing — every prop declared, zero executed** |
| Negative finding | — | `PartKind::Repeated` cannot express the two thumbs it names as its own motivating example (`parts.rs:72`) | the IR cannot express that a runtime **lacks** a declared capability |

The IR absorbs **vocabulary**. It does not absorb **behaviour**. The DOM
element, the event wiring, every derived value and all per-attribute derivation
stayed hand-written in all three slices, in all four runtimes. `expr.rs` is 331
lines with no evaluator; one emitter target mentions `Expr` twice.

## 4. Failed assumptions, named

Spec 063's acceptance says the verdict must name tradeoffs and failed
assumptions, not only green checks. Four:

1. **That declaring a component's surface would let generation replace
   duplicated source.** It did not. Declaration is additive to the
   implementation, not a substitute for it.
2. **That `Repeated` could express repeated anatomy.** It requires a `List`
   prop and yields identical instances. Its own doc comment names the two
   RangeSlider thumbs as the example it cannot serve. Both the web renderer and
   `poodle-render` hard-code "two" — so the limitation is structural, not
   web-specific.
3. **That a capability boundary could be typed.** Half of it can.
   `CapabilityRequirement` has exactly two fields — `capability` and a prose
   `purpose` — so *which runtime owns a capability* is untyped prose, and
   *whether a runtime has it at all* cannot be said. Jetstream renders a text
   field nobody can type into, declared identically to GPUI, which implements
   the whole editing model. This is the closest the pilot came to a stop
   condition ("a runtime needs an untyped side channel"): ownership is exactly
   that side channel.
4. **That the conformance-vector safety net was carrying the components.**
   `b047` measured it: 4 of 13 inventoried machines exercise their real
   surface. `slider`'s vector has 3 cases and zero two-thumb coverage.
   `machines.json` has no `text` key at all.

## 5. Stop conditions

None of spec 063's six stop conditions was formally hit. Two deserve
qualification:

- *"the IR needs arbitrary executable Rust to describe cross-runtime
  behavior"* — not hit, but only because the IR describes **no** behaviour.
  The condition was avoided by scope, not satisfied by capability.
- *"a runtime needs an untyped side channel to render or interact"* — not hit
  for rendering; **effectively hit for capability ownership**, which is prose.

## 6. Recommendation

**Revise**, narrowly — and **reject is defensible** on the same evidence. Both
readings are consistent with what was measured; the choice is about whether the
drift machinery earns its keep independent of the codegen ambition.

The case for **revise**: the propagation and drift-gating machinery works, is
proven four ways, and caught real divergence. The web artifacts are small
(~400 lines total). If the IR is re-scoped to *"one source for cross-runtime
vocabulary, with drift gating"* and the behavioural ambition is dropped
explicitly, the useful half survives at a fraction of the cost — `expr.rs`, the
unused expression vocabulary, and most of the emitter surface would go.

The case for **reject**: ≈31,400 lines to remove zero duplication is a very
expensive way to hold constants, and the existing drift gates
(`docs:contract-drift`, `docs:value-domain-drift`, `docs:spec-drift`) already
cover much of the same ground without a compiler.

What the evidence does **not** support is **adopt**. Pass condition 5 fails
outright, and rolling `009`–`016` across ~165 components would multiply a
26% size increase and ≈31,400 lines of machinery across the whole corpus.

If **revise** is recorded, the two concrete amendments the evidence names are:
per-runtime capability expression (including absence), and repeated anatomy
with per-item identity.

## 7. Verdict

Not yet recorded. The maintainer records **adopt**, **revise** or **reject**
here, and `009`–`016` are recompiled or closed to match. They remain locked.
