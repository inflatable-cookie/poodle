# g13.019 Vocabulary Coverage Across The Corpus

Status: **closed by the g13.020 verdict** (2026-08-13) — the first tranche
was measured and both numbers argue against the sweep; the maintainer closed
it rather than sweeping. See "Measured, 2026-08-13" below.
Owner: Poodle core
Depends on: `g13.018`

## Objective

Bring the rest of the corpus into the single vocabulary authority, family by
family, without rewriting components.

## Deliverables

- Vocabulary definitions for each family: part names, `data-*` names, value
  domains, axes, recipe hooks, capability declarations.
- `ir:check` coverage for each family as it lands.
- A running count of components covered, and of drift the coverage catches.

## Constraints

- **Additive only.** This is not component migration; that is closed
  (`g13.009`–`016`). No component is rewritten to consume a definition unless
  doing so is smaller than not doing so, measured per component.
- The pilot measured +965 lines across nine files for full consumption. If a
  family's consumption cost looks similar, declare the vocabulary and stop
  there — the authority and the drift gate are the deliverable, not the wiring.

## Acceptance

- Coverage is reported as a number, not a claim.
- Every family's vocabulary is drift-gated.
- Corpus LOC does not grow materially. If it does, stop and report.

## Measured, 2026-08-13 (card 052, first tranche)

The card was bounded to five display components precisely so the cost could be
measured before committing to 165. Both numbers came back against continuing.

**Cost — the definitions are ~9x the components they describe.**

| Component | Source | Definition | Ratio |
|---|---|---|---|
| Callout | 121 | 849 | 7.0x |
| EmptyState | 66 | 539 | 8.2x |
| Avatar | 44 | 505 | **11.5x** |
| Pill | 72 | 785 | 10.9x |
| Spinner | 76 | 684 | 9.0x |
| **Total** | **379** | **3,362** | **8.9x** |

**112 authored lines per prop, against the pilot's ~51.** Simple components are
*worse*, not better: the definition's fixed overhead dominates when there is
little surface to describe. Projected across the corpus this is ~113,000 lines,
above even the ~95,000 the card projected from the pilot rate.

**Marginal catch — zero on the canonical class.** Two cross-runtime vocabulary
drifts were planted at review and run against the existing gates:

| Plant | Caught by |
|---|---|
| Svelte-only `data-tone` -> `data-tone-drifted` | `drift:roles` |
| React-only `data-tone` -> `data-tone-x` | `drift:roles` |

`docs:contract-drift`, `docs:value-domain-drift`, `test:parity` and `docs:lint`
missed both — so vocabulary drift is thinly covered — but `drift:roles`, an
existing gate, caught both. The IR would be a **second** mechanism catching
what one already catches.

The verdict's §6 asked exactly this ("the existing drift gates already cover
much of the same ground without a compiler"). On this class, answered: they do.

**Status of the work.** Card 052's worker died mid-run (model stream closed)
before committing, logging, or measuring. Its five definitions are preserved
unmerged on `thread/g13-052-vocabulary-coverage-first-tranche`; the two numbers
above were computed at review instead. Nothing from it is on `main`.

## Next

The corpus sweep is **not recommended**. The maintainer decides between:

1. **Hand to `g13.020` now.** Reassess from the three pilot definitions plus
   this measurement rather than from a covered corpus. Cheapest, and the
   evidence for it is already in hand.
2. **A narrow subset.** Cover only components where cross-runtime vocabulary
   genuinely disagrees today, if any are found by a survey rather than assumed.
3. **Continue the sweep.** Requires accepting ~113,000 authored lines for a
   catch that `drift:roles` already provides.

`g13.020` consolidates and reassesses from whatever authority exists.
