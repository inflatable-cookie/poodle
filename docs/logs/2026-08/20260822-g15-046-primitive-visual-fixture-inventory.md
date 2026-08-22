# g15.046 Primitive Visual Fixture Inventory

Date: 2026-08-22
Card: `../../roadmaps/g15/046-primitive-visual-fixture-inventory.md`
Parent: `../../roadmaps/g15/012-visual-conformance-lane.md`
Handoff: `../../handoffs/20260822-083804-g15-046-primitive-visual-fixture-inventory.md`
Worker branch: `t3code/primitive-visual-fixture-inventory`
Worker worktree: `/Users/tom/.t3/worktrees/poodle/t3code-f9be325c` (launcher-provided,
registered, clean, non-`main`; accepted without creating another)
Planning base: `2eb65afdc51c19fd74ded9dd9b9ff5171a71382d`, an ancestor of the
branch point `04cb16da5819787df775b0c61561703f89dff28e` (= `origin/main` at
dispatch)

## Outcome

One checked-in file,
`test/visual/fixtures/button-visual-inventory.json`, now names exactly the 18
Button visual cases the `g15.047` comparator will ask every renderer for. A
TypeScript loader and a Rust loader parse the same bytes, with no code
generation and no bindings between them.

Nothing renders. No capture, baseline, image, geometry receipt, pixel diff,
threshold, or comparison report was produced or committed by this card.

Every row carries all nine resolved values — theme, size, density, viewport,
scale, content, variant, tone, visual state — plus its derived landmark set.
There is no base object rows inherit from. Both loaders reject `null`, `""`,
and the markers `inherit`, `default-value`, `__default__` in any required
field, so a future capture cannot be produced from a value nobody wrote down.

The fixed environment is Eclipse, `md`, default density, 240×80 logical pixels,
2× scale, label `Run`. Only `button/theme-iceberg` (theme), the four size stops,
and the two density stops depart from it, and each still stores every value.
`scale` is pinned to 2 across the batch and rejected otherwise: `g15.045`
measured the adopted GPUI revision's headless window at a hardcoded 2× factor,
so any other scale is uncapturable in one of the three runtimes.

## Denominator

18 identities, matching the card's table exactly and in its order: three
resting variants, three secondary status tones, four non-`md` size stops, two
non-default densities, three visual states, two content shapes, one Iceberg
reference case. 18 × three renderers is 54 future captures — reviewable by one
person, which is the reason for the number.

The batch samples status tones on the secondary variant only. It does not claim
the variant × tone cross-product.

## Source Cost

| file | lines | note |
| --- | --- | --- |
| `test/visual/fixtures/button-visual-inventory.json` | 261 | canonical data, 18 rows |
| `test/visual/fixtures/button-visual-inventory.ts` | 507 | TypeScript loader/validator |
| `test/visual/fixtures/button-visual-inventory.test.ts` | 476 | 43 focused tests |
| `packages/gpui/preview/tests/visual_fixture_inventory.rs` | 1,063 | Rust loader/validator + 15 focused tests |
| `test/visual/fixtures/README.md` | 170 | boundary documentation |

Two mechanisms, ~2,477 lines, to name 18 cases. That ratio is the honest price
of refusing code generation, and it is why the card's stop conditions matter: a
second component must not double it before the comparator earns the cost.

## Duplicated Registry Count

Held twice, once per language, because no generator connects them:

1. the 18-name fixture roster
2. the schema discriminator `poodle.button-visual-inventory.v1`
3. variants (3)
4. tones (4)
5. visual states (4)
6. content kinds (3)
7. report roles (5)
8. supported capture scales (1)
9. unresolved-default markers (4)
10. the root key set (6)
11. the fixture key set (12)
12. the per-content-kind key sets (3)

Plus one duplicated rule: the landmark derivation (`root` + `content` always,
`icon` when the content carries one, `spinner` when the state is `loading`).

**12 duplicated lists + 1 duplicated rule.**

What `effigy test:visual-fixtures` actually guarantees: both loaders run over
the same canonical file, so a change to *the data* that only one language
accepts fails the selector. That is the whole of it.

What it does not do — and an earlier draft of this log wrongly claimed it did —
is compare the two validators against each other. The selector executes no
cross-language logic comparison. If one loader is loosened or tightened and the
canonical file is not changed to exercise the difference, both commands still
pass and the drift ships silently.

That is not hypothetical. Review of PR #65 found it twice.

First, in declared arrays: TypeScript compared `reportRoles` and `landmarks`
with `join(" ")`, which accepts a collapsed element, while Rust built the same
arrays with `filter_map(Value::as_str)`, which silently discards a non-string.
Each language accepted bytes the other rejected. Both now compare element by
element, and both suites plant the collapsed and non-string cases.

Second, in numbers. Rust used `Value::as_u64()` for `captureScales`, a row's
`scale`, and viewport sides, so it rejected `2.0` where TypeScript accepted it
— and TypeScript accepted integral values beyond 2^53 that Rust could not
represent. JSON has one number type, so a shared file cannot have two numeric
domains. The rule is now stated once and applied identically:

> A fixture number is accepted when it is finite, non-negative, mathematically
> integral, and no larger than 2^53 − 1 (`Number.MAX_SAFE_INTEGER`, the largest
> integer both languages represent exactly).

Each language has one helper for it (`integralNumber` / `integral_number`) used
by all three numeric paths. Fractional, negative, zero-viewport, numeric-string,
unsupported-scale, and beyond-exact-range cases stay rejected. The accepted
decimal-spelling evidence is planted on the canonical *text* rather than a
parsed clone, because the spelling only exists before parsing.

The green selector said nothing about either defect.

The residual risk stands for every one of the 13 duplicated items above. It is
mitigated by planted negative cases on both sides using identical fixtures and
identical expected message text — not by the selector. Any change to one loader
requires the matching change and the matching planted case in the other.

Four domains are *not* duplicated — they are read from existing authority:

| domain | TypeScript reads | Rust reads |
| --- | --- | --- |
| themes | `packages/core/src/tokens` | `presentation_axes::ThemePreset` |
| control sizes | `packages/core/src/tokens` | `presentation_axes::ControlSize` |
| densities | `packages/core/src/tokens` | `poodle_tokens::density` definitions |
| icon names | `packages/core/src/icons/default-icons.json` | the same JSON file |

Both theme/size/density paths trace to the same generated token build, and Rust
reuses exactly the domain authority the offscreen capture target already parses
its CLI against.

## Negative Evidence

Every planted fault is applied to an in-memory clone at run time, so no broken
canonical inventory is committed. The one exception is the accepted
decimal-spelling case, which is planted on the canonical *text* because a
number's spelling stops existing once it is parsed. Both suites assert the
offender is named exactly:

- **missing** — `missing fixture name 'button/size-lg'`
- **extra** — `unknown fixture name 'button/tone-info'`
- **duplicate** — `duplicate fixture name 'button/variant-ghost'`
- **unknown domain** — tone `info`, variant `danger` (the legacy Rust
  compatibility arm), theme `iceberg-light`, size `xxl`, state `hover`, icon
  `rocket-ship`
- **unresolved default** — `density: null`, `theme: "inherit"`, absent `tone`
- **invalid viewport** — width `0`, fractional height, stray `dpr` key
- **invalid scale** — row scale `1`, inventory `captureScales: [3]`
- **malformed / not Button** — non-object root, wrong schema, `component:
  "icon-button"`, a generic `props` bag as an unknown field
- **content and landmark shape** — `icon-only` missing `ariaLabel`, a stray
  `icon` on a label-only case, `loading` without `spinner`, leading-icon
  without `icon`
- **declared array shape** — a collapsed element (`["root content"]`,
  `["fill border", ...]`), a non-string element in place, a non-string element
  inserted, and a bare string where an array is required, for both
  `reportRoles` and fixture `landmarks`; every landmark failure names its
  fixture
- **numeric domain** — fractional `scale` and `captureScales`, negative
  viewport side and scale, a numeric string, and an integral value beyond
  2^53 - 1; paired with positive evidence that integral decimal spellings
  (`2.0`, `240.0`, `80.0`) are accepted on all three numeric paths

A test also asserts the canonical JSON contains none of `expected`, `baseline`,
`threshold`, `tolerance`, `sha256`, or `#`, so expected renderer output cannot
drift into the file.

## Authority Boundary

The inventory names observable inputs and nothing else. Unknown fields are
errors and the key set is closed, so it cannot become a props registry or a
generic component schema. `state` names a rendering input already true of the
frame (`disabled`, `loading`, `pressed`); `hover`, `active`, and `focus` are
absent because they are interactions, not fixture data. Landmarks and report
roles are names only — no bounds, colors, hashes, or thresholds.

Only the portable Button surface enters a fixture. HTML form-only props are
excluded, and `ButtonVariant::Danger` is excluded because the contract names
three variants. No component contract, public package surface, specimen, theme,
token, generated adapter, workflow, or release artifact changed. A test asserts
no published package source imports the inventory.

## Dependency And Task Impact

- **New dependencies:** none, in either language. Rust uses `serde_json`, which
  `poodle-gpui-preview` already declares; the test target also pulls
  `poodle_tokens` through the existing graph.
- **Package metadata:** unchanged. No version bumps.
- **Cargo:** one new `[[test]]` target, `visual_fixture_inventory`, in
  `packages/gpui/preview/Cargo.toml`. It builds no GPUI context and opens no
  window.
- **Effigy:** one new narrow selector, `test:visual-fixtures`, running both
  loaders. Existing routing could not reach either: `test:components` is
  vitest, and `regressions:native` names a different `--test` target. The
  selector is **not** composed into `ci:web`, `ci:native`, or `qa` — gate
  composition is left to the orchestrator alongside `g15.047`.
- **`PAPERCUTS.md`:** unchanged; no new execution friction found.

## Validation

| check | result |
| --- | --- |
| `bun test test/visual/fixtures/button-visual-inventory.test.ts` | 43 pass, 0 fail, 539 assertions |
| `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test visual_fixture_inventory` | 15 pass, 0 fail |
| `effigy docs:check` | pass |
| `git diff --check origin/main...HEAD` | clean |

No `*-windowed` selector, `test:native-visual`, GPUI preview, Jetstream
selector, release mutation, or workflow edit ran.

## What This Cannot Prove

- Nothing about pixels. No image was captured or compared.
- Nothing about whether Button is correct, complete, or at parity in any
  renderer. The inventory is diagnostic and cannot mark a component done.
- Nothing about the renderers themselves: no Svelte, React, or GPUI adapter
  consumes the file yet, and Jetstream is out of scope.
- Nothing about coverage. 18 sampled cases of one component say nothing about
  the rest of the library, or about the variant × tone combinations this batch
  deliberately skips.
- Nothing about tolerance. Whether these cases can be compared within a useful
  threshold is exactly the question `g15.047` exists to answer.

## Continuation

`g15.047` builds the comparator: per-renderer adapters that take a fixture
name, render the real component under that fixture's resolved values, capture
at the stated viewport and scale, and report geometry landmarks and token roles
beside the pixels. Baselines, tolerance policy, gate composition, and any
expansion beyond these 18 cases belong to that card and the operator review
that follows it.
