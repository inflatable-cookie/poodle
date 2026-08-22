# Button visual fixture inventory (g15.046)

One file names the Button visual cases that Svelte, React, and GPUI will each
be asked to render in `g15.047`. That is the whole mechanism: **one name for
the same visual case in every renderer.**

| file | role |
| --- | --- |
| `button-visual-inventory.json` | the canonical inventory — the only fixture-list authority |
| `button-visual-inventory.ts` | TypeScript loader/validator over that file |
| `button-visual-inventory.test.ts` | focused evidence, including planted negative cases |
| `../../../packages/gpui/preview/tests/visual_fixture_inventory.rs` | Rust loader/validator over the same bytes |

Run both loaders:

```sh
effigy test:visual-fixtures
```

## The denominator

Exactly **18** Button identities, frozen by the `g15.046` card:

| group | identities |
| --- | --- |
| resting variants | `rest-secondary`, `variant-primary`, `variant-ghost` |
| secondary status tones | `tone-danger`, `tone-success`, `tone-warning` |
| size ladder beyond `md` | `size-xs`, `size-sm`, `size-lg`, `size-xl` |
| density ladder beyond default | `density-compact`, `density-comfortable` |
| visual states | `state-disabled`, `state-loading`, `state-pressed` |
| content shapes | `content-leading-icon`, `content-icon-only` |
| reference light theme | `theme-iceberg` |

18 identities across three renderers is 54 future captures — small enough for
one person to look at every one. That is the point of the number, and the
reason the batch does not grow until the comparator is proved and reviewed.

The batch samples status tones on the **secondary** variant only. It does not
claim the variant × tone cross-product, and it is not a representative sample
of the component library.

## The fixed environment

Eclipse theme, `md`, default density, 240×80 logical pixels, 2× scale, label
`Run` — except where an identity's own name says it changes one of those.

Every row still stores **all** of those values explicitly. There is no base
object rows inherit from, no `null` meaning "ask the runtime", and no implied
default. Both loaders reject `null`, `""`, and the markers `inherit`,
`default-value`, `__default__` in any required field, precisely so that a
future capture cannot be produced with a value nobody wrote down.

`scale` is explicit and pinned to 2 because `g15.045` measured the adopted GPUI
revision's headless window at a hardcoded 2× factor. A fixture at any other
scale cannot be captured in every runtime, so it is rejected here rather than
approximated later.

## Authority boundary

This inventory names **observable inputs**. It is not, and must not become:

- a component API schema, a props registry, or a `Record<string, unknown>` bag
  — the format is Button-specific, the key set is closed, and an unknown field
  is an error;
- a scene, node tree, or normalized renderer output;
- an action script, event sequence, or behavior machine — `state` names a
  rendering input that is *already true of the frame* (`disabled`, `loading`,
  `pressed`), never the interaction that produces it, which is why `hover`,
  `active`, and `focus` are absent;
- a place for expected output — no bounds, colors, hashes, thresholds,
  tolerances, or baselines live here.

Value domains are checked against existing authority, not re-declared:
TypeScript reads themes, control sizes, and densities from
`packages/core/src/tokens`, Rust reads themes and control sizes from the GPUI
preview's `presentation_axes` module, and both check icon names against
`packages/core/src/icons/default-icons.json`. No component contract, public
package surface, specimen, or token changed to make this file parse.

Only the portable Button surface enters a fixture. HTML form-only props
(`type`, `form`, `formaction`, …) are excluded because native renderers use a
different model, and `ButtonVariant::Danger` — a Rust backward-compatibility
arm — is excluded because the contract names three variants.

The file is test tooling. No published Poodle package imports it, and a test
asserts that.

## Landmarks and report roles

The inventory names where a later receipt may measure and what it may report
beside pixels. Names only — never a position, size, or resolved value.

- Landmarks: `root` and `content` on every case, `icon` when the content shape
  carries one, `spinner` when the state is `loading`. The set is derived from
  the case, and both loaders reject a hand-written set that disagrees.
- Report roles: `fill`, `border`, `text`, `shadow`, `focus-ring`.

## Two loaders, one file

TypeScript and Rust parse the same bytes with no code generation and no
bindings between them. The cost is one duplicated registry: each loader holds
its own copy of the 18-name roster, so each can independently detect a missing,
extra, or duplicate identity. That duplication is deliberate and recorded
rather than generated away — a generator would make one language the authority
for the other, which is the failure mode that killed the previous two
cross-runtime mechanisms.

Every negative case in both suites is planted on an in-memory clone at run
time, so a broken inventory is never committed, and every rejection names its
exact offender.

## What this cannot prove

- Nothing about pixels. No image is captured, compared, or stored by this card.
- Nothing about whether Button is correct, complete, or at parity. The
  inventory is diagnostic; it cannot mark a component done.
- Nothing about renderers that do not yet consume it — Svelte, React, and GPUI
  adapters arrive in `g15.047`, and Jetstream is out of scope.
- Nothing about coverage. 18 sampled cases of one component say nothing about
  the other components or the untested variant × tone combinations.

## Continuation

`g15.047` builds the comparator: per-renderer adapters that take a fixture
name, render the real component under that fixture's resolved values, capture
at the stated viewport and scale, and report geometry landmarks and token roles
beside the pixels. Baselines, tolerance policy, and any expansion of this batch
belong to that card and the operator review that follows it.
