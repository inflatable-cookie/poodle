# 026 `dismissOnOutsideInteract` Across The Overlay Family

Status: merged (`4418eb58` → `c468b434`)
Milestone: side-quest (component API, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-026-dismiss-outside-interact`
Depends on: nothing in flight. Avoid `HistoryCenter.*` (owned by `b024`).
Governing refs: `docs/contracts/components/popover.md`,
`docs/contracts/002-anchored-overlays.md`,
`docs/contracts/001-working-rules.md` §Runtime Parity Authority,
`packages/core/src/dom/dismiss.ts`

## Goal

Fourteen components register a dismiss layer with
`dismissOnOutsideInteract: true` **hardcoded**. Only `Popover` exposes it as a
prop. Make outside-dismissal a consumer decision everywhere it is a decision at
all.

## Evidence

Measured on `761f81d8`. Of the fifteen components that register a dismiss
layer, exactly one declares `dismissOnOutsideInteract?: boolean`:

| Declares the prop | Hardcodes `true` in the layer |
|---|---|
| `Popover` | `Select`, `Menu`, `Dialog`, `Drawer`, `ContextMenu`, `Menubar`, `NavigationMenu`, `SplitButton`, `ThemeSelect`, `RefSelect`, `ModelPicker`, `OrderBy`, `ListCard`, `FilterBuilder` |

`Popover` is the precedent and already documents the prop in its contract
(`popover.md:43`, `:109`, `:135`) — including its interaction with the machine,
which is the shape to copy.

## Fixed By Ruling (do not re-decide)

- **Default is `true`.** Every one of these currently dismisses on outside
  interaction, and that is right for the overwhelming majority of uses. This
  card makes the behaviour *refusable*, not different. A default flip would be
  a silent behavioural change across fourteen components — the exact failure
  mode the Tabs `bordered` work already cost us.
- **`Dialog` and `Drawer` are in scope but their default is `false`.** A modal
  that vanishes on an outside click loses work. Check each one's current
  registration before assuming: if a component already registers `false`,
  preserve that as its default and say so in the contract.
- **Escape is not in scope.** `resolveDismiss` deliberately splits the two:
  escape dismisses the innermost layer only, outside-interaction dismisses all
  non-containing layers. This card touches the outside axis alone. Do not add
  an escape prop.
- **Svelte is the reference**, then React mirrors it exactly
  (Runtime Parity Authority). Prop name, type, and default identical.

## Scope

### In scope

- `dismissOnOutsideInteract?: boolean` on all fourteen, in both web runtimes,
  plumbed to the layer registration rather than merely declared.
- Contract updates for all fourteen, matching `popover.md`'s treatment: the
  prop table row, and the behaviour-machine row showing the guard on the
  outside-interaction transition.
- A test per component in both runtimes proving `false` actually suppresses
  outside dismissal — not just that the prop is accepted. A prop that
  type-checks and does nothing is the defect this card exists to prevent.
- Native parity: `SplitButton`, `Menu`, `ContextMenu`, `Menubar`,
  `NavigationMenu`, `Select` and friends have Rust specs. Where a spec models
  dismissal, add the field; where it does not, note that in the log rather
  than inventing one.

### Out of scope — stop conditions if reached

- `HistoryCenter` — `b024` owns those files.
- Escape-key behaviour, focus trapping, or `resolveDismiss` itself.
- Changing any current default. Read each registration; preserve what is
  there.
- The `contract-prop-drift` reverse-direction gap (below). Recorded, not
  fixed here.

## A Gate Hole Found While Scoping — do not fix here

`contract-prop-drift` checks one direction only: *every documented prop is
implemented*. A prop that is implemented but undocumented never fails it, so
this whole gap was invisible. The reverse direction exists behind
`DRIFT_REPORT=1` as information that never exits non-zero, and it reports 54
components — much of it snippets (`children`, `footer`) rather than props.

It also has a real parser bug: it lists `and` and `time` as props of
`date-time-zone-picker`, both extracted from inside
`placeholder = "Select date, time, and zone"` and
`defaultValue = { date: null, time: null, timeZone: null }` — despite the
comment at `contract-prop-drift.ts:51` claiming default values and object
literals are skipped by depth.

Recorded in `PAPERCUTS.md`. Enforcing the reverse direction is worth its own
card and needs the snippet/prop distinction and that bug fixed first.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `Popover.svelte` and `popover.md` first — both are the template.
- Read each component's existing layer registration before changing it. The
  default you write must equal the behaviour that ships today.
- Do not touch `packages/core/src/dom/dismiss.ts`.
- Do not touch any `HistoryCenter` file.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-026-dismiss-outside-interact`. Do not merge.

## Writable Paths

- The fourteen components in `packages/svelte/components/src/`
- Their counterparts in `packages/react/components/src/`
- Their test files in both runtimes
- Their contracts in `docs/contracts/components/`
- `packages/contracts/components/src/` where a spec models dismissal
- `docs/logs/2026-08/<DD>-g13-026-dismiss-outside-interact.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:components`, `effigy test:parity`,
   `effigy docs:lint`, `effigy check:svelte`, `git diff --check`. Record exit
   states.
2. Read `Popover.svelte`, `popover.md`, and `packages/core/src/dom/dismiss.ts`.
3. Per component: read its current registration, add the prop with that
   behaviour as the default, plumb it, test both truthy and falsy.
4. Mirror into React.
5. Contracts, matching `popover.md`'s structure.
6. Validate:
   ```sh
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:contract-drift
   effigy svelte:surface-audit
   cargo test --manifest-path packages/contracts/components/Cargo.toml
   git diff --check
   ```

## Acceptance Criteria

- [ ] All fourteen expose `dismissOnOutsideInteract` in both web runtimes,
  plumbed to the layer.
- [ ] Every default equals the behaviour that shipped before this card, proven
  by reading each registration; deviations named in the log.
- [ ] Each has a test in both runtimes proving `false` suppresses outside
  dismissal.
- [ ] Fourteen contracts document the prop and its machine guard.
- [ ] `HistoryCenter` untouched; `dismiss.ts` untouched.
- [ ] All step-6 commands exit 0.

## Stop Conditions

- A component's dismissal is not expressible as a single boolean.
- Making the prop refusable breaks focus return or leaves a layer stuck open.
- A Rust spec models dismissal in a way the boolean cannot express.

Stop with exact paths, commands, and the smallest unresolved question.
