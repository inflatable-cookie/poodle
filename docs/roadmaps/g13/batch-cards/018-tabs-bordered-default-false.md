# 018 Tabs `bordered` Defaults To False

Status: blocked — waiting on `g13-b016` to release the Tabs files
Milestone: side-quest (component API, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-018-tabs-bordered-default`
Depends on: `g13-b016` merged (it currently holds `Tabs.svelte`, `Tabs.tsx`,
`contracts/components/src/tabs.rs`, `render/src/tabs.rs`, both `types.ts`, and
`tabs.md`)
Governing refs: `docs/contracts/components/tabs.md`,
`docs/contracts/001-working-rules.md` §Runtime Parity Authority

## Ruling

`bordered` defaults to **`false`**.

## Why

`bordered` is currently the only Tabs decoration that is on by default.
`activeOutline` is `false`, `activeFill` is `"tint"`, and after this change
`card` becomes a genuinely plain baseline with three opt-in decorations. That
inconsistency is what produced the Loophole titlebar bug: the component assumed
"tabs above content", a use case it cannot see, and every other use paid for it
in dead space.

**The failure modes are asymmetric, and that is the decisive argument.** A tab
strip over content with no separating line is plainer but functional. A tab
strip in a titlebar with an unwanted border and its padding is visibly broken
layout. The milder failure belongs in the default.

Usage evidence was gathered and is genuinely close — roughly 60 bare strips to
45 panel-rendering usages across consumers, and closer still once duplicate
worktree copies are excluded. It does not decide the question either way, and
the ruling does not rest on it.

## Consumer Impact — read before starting

This is a **silent visual change**: ~45 panel-rendering consumer usages lose
their separating border with no type error and no build error. Unlike the
`text`/`underline` removal, nothing fails loudly.

Mitigation is **not** this card's job — the consumer sweep adds an explicit
`bordered` to panel-rendering usages so the change is mechanical rather than
silent. This card must not edit any consumer repository.

## Scope

### In scope

- Default flipped to `false` in: `Tabs.svelte`, `Tabs.tsx`, `TabsSpec`
  (`contracts/components/src/tabs.rs`), and any default asserted in
  `render/src/tabs.rs`.
- `tabs.md`: default column, and a short note on why the plain baseline is the
  default and when to opt in.
- Specimens in all four runtimes: any specimen that renders a panel should pass
  `bordered` explicitly, so the docs keep showing the bordered look where it is
  correct. Bare-strip specimens stay unbordered.

### Out of scope — stop conditions if reached

- Any consumer repository.
- Changing what `bordered` *does* — it already owns both the border and the
  padding that holds tabs off it (`7647a704`). Only the default changes.
- Any other prop default.
- Refreshing visual baselines.
- `poodle-ir`.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Confirm `g13-b016` is merged before starting; if `Tabs.svelte` still lacks
  the shared `ActiveFill` type reference, stop — you are on a stale base.
- Svelte, React and Rust defaults must match exactly. A default that differs
  per runtime is the drift this project exists to remove.
- Contract updates in the same commit.
- Do not refresh baselines; enumerate and classify diffs. Tabs diffs are
  expected here; anything else is a stop condition.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-018-tabs-bordered-default`. Do not merge.

## Writable Paths

- `packages/svelte/components/src/Tabs.svelte`
- `packages/react/components/src/Tabs.tsx`
- `packages/contracts/components/src/tabs.rs`
- `packages/render/src/tabs.rs`
- `docs/contracts/components/tabs.md`
- `packages/{svelte,react}/preview/src/**/TabsSpecimen.*`
- `packages/{gpui,jetstream}/preview/src/specimens/tabs.rs`
- Tests for Tabs in either web runtime
- `docs/logs/2026-08/<DD>-g13-018-tabs-bordered-default.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

## Steps

1. Verify base: `g13-b016` merged, `git log --oneline -3`, then
   `effigy test:components`, `effigy test:parity`, `effigy docs:lint`,
   `cargo test -p poodle-render`, `git diff --check`.
2. Flip the default in all three type surfaces plus any render-side default.
3. Update `tabs.md`: the default column, and a note that `card` is a plain
   baseline — `bordered` for tabs above content, `activeOutline` and
   `activeFill` for selection emphasis.
4. Specimens: add explicit `bordered` to panel-rendering specimens in all four
   runtimes; leave bare strips unbordered. Svelte and React labels identical.
5. Add a test asserting the default is `false` in both web runtimes, and a Rust
   test asserting `TabsSpec::default().bordered == false`.
6. Visual enumeration in report mode; classify. Refresh nothing.
7. Validate:
   ```sh
   effigy test:components
   effigy test:parity
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:spec-drift
   cargo test -p poodle-render
   cargo test -p poodle-specs
   effigy docs:check
   git checkout -- packages/tokens/artifacts/rust/
   git diff --check
   git status --porcelain
   ```

## Acceptance Criteria

- [ ] Default is `false` in Svelte, React, and `TabsSpec`, asserted by tests in
  all three.
- [ ] `tabs.md` records the new default and the reasoning.
- [ ] Panel-rendering specimens pass `bordered` explicitly in all four
  runtimes; bare strips do not.
- [ ] No consumer repository touched; no baseline refreshed.
- [ ] All step-7 commands exit 0.
- [ ] Batch log records commands, exit states, and the diff table.

## Stop Conditions

- `g13-b016` is not merged, or the Tabs files do not match its post-merge state.
- The three runtimes cannot be made to agree on the default.
- A visual diff appears on a component other than tabs.

Stop with exact paths, commands, and the smallest unresolved question.
