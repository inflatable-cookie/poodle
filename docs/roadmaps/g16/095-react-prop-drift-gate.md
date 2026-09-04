# g16.095 — Svelte↔React Public Prop Drift Gate

Status: ready — in revision after review of PR #202 (head `1440aeb33`);
operator decision 2026-09-04: ratcheted baseline, gate stays in `docs:check`
Type: validation gate — no component API change
Opened: 2026-09-04
Depends on: none; independent of every other ready lane
Governing refs: `../../contracts/001-working-rules.md` (Svelte is the reference implementation; React must match), `../../architecture/001-poodle-system-shape.md`, `../../architecture/006-headless-core-and-machine-model.md`
Operator decision: 2026-09-02 — React stays source-only, gains a Svelte↔React public-prop drift gate, and is not published before a named consumer exists
Dispatch manifest: `../dispatch.md`

## Goal

Make Svelte↔React public prop divergence a failing check. Today
`packages/svelte/preview/scripts/contract-prop-drift.ts` compares each
contract's `### Public Props` table against the Svelte `interface Props` only.
Nothing compares React. The 2026-09-01 audit found 32 components whose React
`Props` interface diverges from Svelte: React-only uncontrolled defaults
(`defaultValue` on `Slider`, `RangeSlider`, `TriStateSwitch`, `SidebarNav`,
`defaultCollapsed` on `FilterToolbar`), Svelte props missing from React
(`Button` `formenctype`/`formmethod`; `DockRegion` `showCollapseToggle`,
`showTabs`), and a callback arity mismatch (`ToastStack.onDismiss`).

## Fixed Boundary

- Add one script, `packages/svelte/preview/scripts/react-prop-drift.ts`, that
  parses `packages/react/components/src/<Name>.tsx` `interface <Name>Props`
  (or the exported props type the shell actually uses) and compares its public
  prop name set against the Svelte `interface Props` for the same component,
  reusing `svelteProps`, `snippetProps`, and the exclusion rules already
  exported by `contract-prop-drift.ts`.
- Normalize framework idiom before comparing: React camelCase DOM attributes
  (`autoComplete`, `spellCheck`, `autoCapitalize`, `autoCorrect`) are the same
  prop as Svelte lowercase attributes; `children`/`render` props are the React
  form of Svelte snippets and are excluded like snippets; `on*` callbacks are
  compared by name only. Record every normalization rule in the script header.
- Fail on: a React-only prop the contract does not document; a Svelte prop
  absent from React; a documented default that differs where both sides
  declare a static literal default.
- Ship a `BASELINE` register in the script. Every entry carries a `kind` and
  a reason, and the script refuses to load an entry without both. Kinds:
  - `pending-port` — a Svelte prop not yet ported to React. The reason must
    name the card that will clear it (`g16.099`). Adding a new `pending-port`
    entry without a card reference fails the gate.
  - `framework-idiom` — React uncontrolled `default*` initializers and
    React-only change callbacks that mirror Svelte `$bindable` initial values
    (working rules, Runtime Parity Authority). Documented, not drift.
  - `needs-decision` — a divergence that requires a contract or API choice;
    the reason names the open question and the owning note or card.
  The register is a ratchet: it may only shrink except by a card-referenced
  `pending-port` entry. Seed it with the 29 current findings so `main` stays
  green and any new divergence fails.
- Add selector `docs:react-prop-drift` to `tasks/effigy.tasks.toml` and
  include it in `docs:check` beside `docs:contract-drift`. Do not add it to
  `docs:lint`'s in-process aggregation; a standalone selector is enough.
- Do not change any component API, default, or contract to make the gate pass.
  Report the finding set in the execution log grouped as: port to React,
  candidate for Svelte inclusion, framework idiom, needs decision. The
  revision after review (`1440aeb33`) adds only the seeded register, its
  `kind` validation and ratchet test, and the log grouping; the reviewer's
  five non-blocking hardening notes may be taken in the same revision when
  they do not widen scope.
- Do not touch `.github/workflows/`, release surfaces, or React publication
  metadata.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| React-only prop fails | plant `defaultValue?: number` on a React shell whose contract lacks it | selector exits 1 naming component and prop |
| Missing React port fails | remove one documented Svelte prop from a React shell | selector exits 1 naming component and prop |
| Attribute casing is not drift | `autocomplete` (Svelte) vs `autoComplete` (React) | no finding |
| Snippets and children are not drift | Svelte `children: Snippet` vs React `children: ReactNode` | no finding |
| Baseline is reasoned | add a baseline entry without a reason string or `kind` | script refuses to load the entry |
| Ratchet holds | add a `pending-port` entry whose reason names no card | gate exits 1 |
| Main is green | run `effigy docs:check` on the PR head against current main | pass |
| Board integration | run `effigy docs:check` on a planted drift | board is red at `docs:react-prop-drift` |

## Validation

Run `effigy docs:react-prop-drift`, `effigy docs:check`, `effigy ci:web`, and
`git diff --check origin/main...HEAD`. Commit the biting counterexample tests
before the script. Never run release, workflow, windowed, or native-visual
selectors.

## Owned Paths

`packages/svelte/preview/scripts/react-prop-drift.ts`,
`packages/svelte/preview/test/react-prop-drift.test.ts` (the vitest
`packages/svelte/preview/test/**` project), `tasks/effigy.tasks.toml`
(`docs:react-prop-drift` and the `docs:check` sequence only), this card's
execution log under `docs/logs/2026-09/`, and root `PAPERCUTS.md` (append
only).

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`.

## Stop Conditions

Stop and report when: the script cannot locate a React props type for a public
component; a divergence requires a contract or API decision (return it to
planning with the finding, do not decide it); `docs:check` or `ci:web` fails
for a reason unrelated to this card; the finding set exceeds what the baseline
register can honestly carry. Escalation owner: Chatterbox (planning).

## Continuation

The `pending-port` entries are cleared by `g16.099`. `needs-decision` entries
return to Chatterbox. Svelte-inclusion candidates (`Tree.onEditingChange`,
`OrderBy.onActiveSortChange`) are recorded, not decided, here.
