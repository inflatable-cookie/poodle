# 027 Dismiss Native Parity, And The Drift Gate's Missing Direction

Status: ready
Milestone: side-quest (parity + gate integrity, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-027-dismiss-native-and-drift-direction`
Depends on: `g13-b026` merged (`c468b434`)
Governing refs: `docs/contracts/001-working-rules.md` §Runtime Parity Authority,
`packages/svelte/preview/scripts/contract-spec-drift.ts`,
`packages/svelte/preview/scripts/contract-prop-drift.ts`

## Goal

Two debts, both incurred by discovering that a gate was not looking where we
assumed. They share a root cause and belong together.

1. `dismissOnOutsideInteract` reached the web on fifteen components and the
   shared spec surface on none, parked in `OPEN_GAPS`.
2. `contract-prop-drift` never checks that an implemented prop is documented,
   which is why `b026`'s whole gap went unseen for as long as it did.

## Part 1 — Retire the `OPEN_GAPS` entries

`b026` added twelve entries to `OPEN_GAPS` in `contract-spec-drift.ts`. That
list was **empty** before, and its own doc comment says an entry "means a prop
shipped to the web without reaching the shared spec surface, which is the thing
this gate exists to stop."

The register is the right one — `WEB_ONLY_PROPS` would have excused it
permanently, and refusing outside dismissal is a genuine behavioural capability
a native overlay could want, not a web-platform artifact like `as` or
`scrollTarget`. But debt recorded is not debt discharged.

### The decision this card must make and record

`b026`'s reasoning was that native platforms have standard outside-dismissal,
so a spec field always reading `true` would be invented data. Test that against
the specs as they actually are:

- `Dialog` and `Drawer` already model dismissal (`dismiss_on_*`), and their
  defaults are `false`. So the spec surface *does* model this concept where the
  default is interesting — the twelve are excused only because their default is
  uniform.
- A uniform default is not the same as an inexpressible one. If a native host
  ever needs to refuse dismissal on a `Menu`, the field's absence is a wall.

Rule it one way and write the ruling into `contract-spec-drift.ts` beside the
register:

- **Add the field** to the twelve specs and empty `OPEN_GAPS` — or
- **Keep the carve-out**, and then say precisely what distinguishes these
  twelve from `Dialog`/`Drawer`, in the code, not in a log.

Do not leave twelve entries with a "revisit if" comment. That is the state this
card exists to end.

### If the ruling is "add the field"

Field, renderer resolution, and a Rust test per spec. Native adapters consume
`poodle-render` output, so the field must reach a real decision, not just the
struct.

## Part 2 — Make the drift gate bidirectional

`contract-prop-drift` reports `OK — every documented public prop is implemented
in Svelte` and exits 0 while fifteen components carried an undocumented public
prop. The reverse direction exists behind `DRIFT_REPORT=1`, never exits
non-zero, and is unusable as a gate for two reasons.

**It cannot tell a snippet from a prop.** It reports 54 components, most of it
`children`, `footer`, `trailing`, `icon`, `actions` — snippet slots typed as
props, which contracts legitimately document differently. `WEB_ONLY_PROPS`
already carries a "snippet slots typed as props: `leading`, `trailing`" note,
so the distinction exists in the codebase and needs applying here.

**It has a parser bug.** It lists `and` and `time` as props of
`date-time-zone-picker`. Both come from inside
`placeholder = "Select date, time, and zone"` and
`defaultValue = { date: null, time: null, timeZone: null }` —
`DateTimeZonePicker.svelte:42,45`. The comment at `contract-prop-drift.ts:51`
claims commas inside default values and object literals are skipped by depth;
for string literals containing commas, they are not.

### Required

- Fix the parser. A comma inside a string literal is not a prop boundary. Add a
  test with `placeholder = "Select date, time, and zone"` as the fixture — the
  real line that exposed it.
- Separate snippets from props, reusing the existing convention rather than a
  new list.
- Then enforce the reverse direction, exiting non-zero.
- Expect a real backlog when you turn it on. Document every remaining
  undocumented prop in its contract, or register it with a reason. **Do not
  create a second empty-list-shaped escape hatch to make the gate pass.** If
  the backlog is too large for this card, enforce it and land the
  documentation in tranches, saying in the log exactly what remains — but the
  gate ships enforcing.

## Out Of Scope — stop conditions if reached

- Changing any component's dismissal *behaviour*. `b026` settled the defaults
  and they are correct; this card moves the concept to native and fixes a gate.
- `resolveDismiss` or escape-key handling.
- `contract-spec-drift`'s `WEB_ONLY_PROPS` contents.
- Any consumer repository.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `b026`'s batch log and `contract-spec-drift.ts`'s register comments
  before ruling Part 1.
- Part 1 and Part 2 are independent — commit them separately so either can be
  reverted alone.
- Run `cargo clippy --all-targets -- -D warnings`, not bare `cargo clippy`.
  Bare clippy does not lint test targets, which is how three lints survived
  two codegen cards.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-027-dismiss-native-and-drift-direction`. Do
  not merge.

## Writable Paths

- `packages/contracts/components/src/**`
- `packages/render/src/**` where a dismissal field needs resolving
- `packages/svelte/preview/scripts/contract-prop-drift.ts`
- `packages/svelte/preview/scripts/contract-spec-drift.ts`
- `docs/contracts/components/**`
- Tests for the above
- `docs/logs/2026-08/<DD>-g13-027-dismiss-native-and-drift-direction.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy docs:lint`, `docs:contract-drift`, `docs:spec-drift`,
   `test:components`, `test:parity`,
   `cargo test --manifest-path packages/contracts/components/Cargo.toml`,
   `git diff --check`. Record exit states.
2. Part 1: rule, implement, empty or justify `OPEN_GAPS`. Commit.
3. Part 2: fix the parser with its regression test, separate snippets, enforce,
   burn down or tranche the backlog. Commit separately.
4. Validate:
   ```sh
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:spec-drift
   effigy test:components
   effigy test:parity
   effigy check:svelte
   cargo test --manifest-path packages/contracts/components/Cargo.toml
   cargo clippy --manifest-path packages/contracts/components/Cargo.toml --all-targets -- -D warnings
   effigy ci:web
   git diff --check
   ```

## Acceptance Criteria

- [ ] `OPEN_GAPS` is either empty, or carries a ruling in code stating what
  distinguishes these twelve from `Dialog`/`Drawer`.
- [ ] If the field was added: every one of the twelve specs has it, resolved by
  the renderer, with a Rust test.
- [ ] `contract-prop-drift` no longer reports `and`/`time` for
  `date-time-zone-picker`, proven by a test using that exact line.
- [ ] The gate enforces the reverse direction and exits non-zero on an
  undocumented prop.
- [ ] No new permanently-empty escape list.
- [ ] All step-4 commands exit 0.

## Stop Conditions

- The twelve specs cannot carry the field without inventing native behaviour —
  say which spec and why, and take the carve-out branch instead.
- Enforcing the reverse direction reveals a backlog too large to document here.
  Enforce anyway, tranche the documentation, and say exactly what remains.

Stop with exact paths, commands, and the smallest unresolved question.
