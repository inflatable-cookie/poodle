# g13.020 Consolidate And Reassess

Status: verdict drafted — awaiting maintainer signature
Owner: Poodle maintainer
Depends on: `g13.019` (closed by this ruling), b052 measurement

## Objective

Use the single vocabulary authority as the vantage point for the reassessment
the `g13.008` verdict was recorded to enable.

Once every component's vocabulary lives in one place, the corpus can be judged
from one authority instead of four hand-maintained copies. That is the whole
reason `revise` was chosen over `reject`.

## Reassessment — evidence (b052)

The worker died mid-run; both required numbers were computed at review from
its five unmerged definitions.

### Cost — definitions are ~9× the components they describe

| Component | Source | Definition | Ratio |
|---|---|---|---|
| Callout | 121 | 849 | 7.0× |
| EmptyState | 66 | 539 | 8.2× |
| Avatar | 44 | 505 | 11.5× |
| Pill | 72 | 785 | 10.9× |
| Spinner | 76 | 684 | 9.0× |
| **Total** | **379** | **3,362** | **8.9×** |

112 authored lines per prop against the pilot's ~51. Simple components are
worse, not better: the definition's fixed overhead dominates when there is
little surface to describe. Corpus-wide that projects to ~113,000 lines.

### Marginal catch — zero on the canonical class

| Plant | Caught by |
|---|---|
| Svelte-only `data-tone` → `data-tone-drifted` | `drift:roles` |
| React-only `data-tone` → `data-tone-x` | `drift:roles` |

`docs:contract-drift`, `docs:value-domain-drift`, `test:parity` and
`docs:lint` missed both, so vocabulary drift *is* thinly covered — but
`drift:roles` caught both. The IR would be a second mechanism catching what
one already catches. `g13.008` §6's open question now has evidence.

## What the authority did not deliver

- No marginal catch over the gate estate on the class tested.
- The pilot's own ledger: +965 consumer lines across nine files, zero
  duplication removed, every targeted file grew.
- The surviving value claim — "one place to reassess the corpus from" —
  costs ~9× the source it describes and ~113k lines at corpus scale.

## Verdict

Retire the vocabulary authority as a corpus mechanism. The drift-gate estate
is the corpus-wide authority; the IR earns its keep nowhere beyond what is
already built.

1. **Unwind the three pilot slices** — Button, RangeSlider, TextInput return
   to hand-written surfaces; definitions and generated artifacts are removed
   (card `053`).
2. **`g13.019` is closed.** No sweep, no survey. The b052 measurement is the
   answer.
3. **The gate estate stands** — `drift:roles` included, now proven on the
   canonical class.
4. **The two `g13.017`/`018` lessons are rehomed, not discarded:**
   capability absence becomes a working-rules and `g14.006` requirement;
   repeated anatomy with per-item identity becomes a `g14.007` rendering
   requirement. They were worth finding; they are not worth the machinery
   that carried them.
5. **Shell scene — kept** (maintainer ruling, recorded). It is the one
   replacement case: four hand-written preview shells became one Rust
   source, proven across all four runtimes. `poodle-ir` and `poodle-codegen`
   survive slimmed to scene-only; specimen migration onto the scene system
   is scheduled into g14 (`g14.003`).

## Acceptance

- [ ] The reassessment names what the authority did **not** deliver, not
  only what it did. (Above.)
- [ ] Any further programme is compiled from measured evidence, not from
  the momentum of this one. (g14's fixed inputs record this.)

## Next

On signature: rule the `053` R2 destination (capability declaration table
rehomed into the gate itself, or `packages/contracts/headless/`), dispatch
card `053` (unwind), run the g13 closeout checklist in `../g14/README.md`,
then open g14 with `g14.001`.
