---
title: g13 batch 027 — dismiss-native parity, and the drift gate's missing direction
status: complete
milestone: side-quest (parity + gate integrity, outside the g13 IR lane)
owner: Poodle core
updated: 2026-08-12
tags: [log, g13, dismissal, dismissal-on-outside-interact, drift-gate, side-quest]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/027-dismiss-native-parity-and-drift-direction.md`
on branch `thread/g13-027-dismiss-native-and-drift-direction`, in two separate
commits so either can be reverted alone:

- Part 1 (`1331b5e5`): retired `OPEN_GAPS` — the twelve specs now carry
  `dismiss_on_outside_interact` and the register is `{}` again.
- Part 2 (`c04a9cdc`): made `contract-prop-drift` bidirectional — parser fixed
  with a regression test, snippets separated from props, reverse direction
  enforced with the backlog burned down or registered with reasons.

## Part 1 — the twelve specs carry the field, the renderers resolve it

The ruling (made in `befc8aac`, not re-decided): PopoverSpec already carries
`dismiss_on_outside_interact`, defaulted `true`, with a builder, so the twelve
follow it. Each spec's default was read from its Svelte component, not assumed
— all twelve destructure `dismissOnOutsideInteract = true`.

Added to `context_menu.rs`, `filter_builder.rs`, `list_card.rs`, `menu.rs`,
`menubar.rs`, `model_picker.rs`, `navigation_menu.rs`, `order_by.rs`,
`ref_select.rs`, `select.rs`, `split_button.rs`, `theme_select.rs`: the field,
the `true` default, and `with_dismiss_on_outside_interact`.

### Resolution (the field is read, not dead)

Every one of the twelve renderers reads the field:

- `context_menu()` and `menubar()` forward their spec's value into the
  `MenuSpec` they compose — the `alert_dialog.rs:139-140` pattern (the
  renderer resolves the composed spec's dismissal from its own spec state);
  the composed renderers' tests prove the forwarding.
- `menu()`, `select()`, `split_button()`, `ref_select()`, `model_picker()`,
  `theme_select()`, `navigation_menu()`, `order_by()`, `filter_builder()`,
  `list_card()` resolve it onto the open overlay's surface node.

The node vocabulary (`poodle-node`, out of writable scope) has no
outside-interact channel for non-modal overlays — no backdrop to wire the way
`dialog.rs` wires `dismiss_on_backdrop`. So the refusal rides the interaction
model, the same mechanism `dialog.rs` uses to protect a surface from a
dismissal route: when `dismiss_on_outside_interact` is `false`, the open
overlay surface carries an inert `interaction.on_activate` marker. The host
contract, stated in each renderer's comment: a native host implementing
outside-dismissal must not dismiss a surface carrying this marker — the
node-tree form of the web layer's `dismissOnOutsideInteract: false`. Default
`true` changes nothing (platform-standard outside dismissal, as b026 ruled).

One documented limit: `list_card()`'s root is the only node it has, and an
interactive card's click handler owns the single `on_activate` slot, so the
marker yields to it. The renderer comment and the `list_card` test state this.

Twelve Rust tests prove the resolution: ten assert the marker appears with
`with_dismiss_on_outside_interact(false)` and is absent at the default (which
also pins the web default), two assert the composed-spec forwarding.

`OPEN_GAPS` in `contract-spec-drift.ts` is `{}`; `effigy docs:spec-drift`
passes with no exemptions.

### Clippy debt cleared so step 4 can exit 0

`cargo clippy --all-targets -- -D warnings` failed on pre-existing debt:
40 lints in `poodle-specs` (derivable `Default` impls, nine
`new_without_default`, manual `div_ceil`/checked-division, a `match`→`?`) and
the three in `poodle-render` the card warned survived two codegen cards
(`agent_subagent.rs` redundant binding, `audio.rs` too-many-arguments,
`app_header.rs` test `manual_contains`). Cleared in the Part 1 commit; the
contracts crate keeps its 241 tests green.

## Part 2 — the drift gate is bidirectional

### Parser: a comma inside a string literal is not a prop boundary

`contract-prop-drift` reported `and` and `time` as props of
`date-time-zone-picker`, lifted from `placeholder = "Select date, time, and
zone"` and `defaultValue = { date: null, time: null, timeZone: null }`
(`DateTimeZonePicker.svelte:42,45`): the depth rules skipped object literals
but never string literals. The destructure split loop now tracks string state
(quote + escape), and the close-brace finder too. Regression test
(`packages/svelte/components/test/ContractPropDrift.test.ts`) uses the exact
real line as the fixture, plus the real component file.

### Snippets are not props

Snippet-typed props are detected from the component's `Props` interface (or
inline annotation) using the same `FRAMEWORK_TYPES = { Snippet: true }`
convention `contract-value-domain-drift.ts` uses, with the arrow-function
depth guard (an interface scan hit the same `=>`-breaks-depth bug the
destructure parser already guarded) and doc-comment stripping. Snippets count
as implementations for the contract-only direction (TextInput documents
`leading`/`trailing`), never as undocumented props. The gate's "checked 130"
now sees props, not slots.

### Enforcement and the backlog

Both directions now fail the gate (`contractDriftErrors()` and the standalone
script; `componentDrift()` is exported and unit-tested). Enforcing exposed 26
undocumented props across 7 components. Disposition:

- **Documented in the contract** (spec surface already excused on the native
  side): `card` `class`, `spinner` `class`/`style`, each marked
  `**Web targets only**` — the same marker the parser uses for
  "**Rust targets only**" props.
- **Parser artifacts, not backlog**: `xy-pad`'s thirteen props and
  `split-view`'s `primaryHidden`/`secondaryHidden` were documented all along in
  comma- or slash-joined table cells the prop parser could not read; the cell
  parser now reads both spellings.
- **Registered in `BASELINE` with reasons** (the gate's sanctioned, existing
  drift register — every entry carries a reason; no new empty escape list was
  created):

  | Component | Props | Reason |
  |---|---|---|
  | `dialog` | `closeButtonSize`, `overlayStyle` | cross-target close-button size with no `DialogSpec` field (spec-surface tranche); web-only styling passthrough (`WEB_ONLY_PROPS` excuses `overlayClassName` but not this spelling; the register is out of this card's scope) |
  | `dock-region` | `showTabs`, `tabVariant` | cross-target strip controls; `DockRegionSpec` models `tabs_placement` only (spec-surface tranche) |
  | `popover` | `triggerIsInteractive` | DOM-only switch; documented in contract prose (§TriggerIsInteractive); native composes its trigger directly |
  | `split-view` | `minRatio`, `maxRatio` | cross-target ratio clamps; `SplitViewSpec` models `ratio` only (spec-surface tranche) |

  **What remains (tranche):** the six cross-target props above need
  `poodle-specs` fields (`DialogSpec::close_button_size`,
  `DockRegionSpec::show_tabs`/`tab_variant`, `SplitViewSpec::min_ratio`/
  `max_ratio`), renderer resolution, and tests before they can move from the
  register into the contracts' Public Props tables. That is a spec-surface
  batch of its own; the gate ships enforcing meanwhile. Popover's
  `triggerIsInteractive` and dialog's `overlayStyle` are web-only by nature and
  stay registered unless `WEB_ONLY_PROPS` is later extended (out of scope
  here).

## Validation

| Command | Exit state |
|---------|-----------|
| `effigy docs:lint` | 0 |
| `effigy docs:contract-drift` | 0 — both directions, no findings |
| `effigy docs:spec-drift` | 0 — checked 113, `OPEN_GAPS` `{}` |
| `effigy test:components` | 0 — 69 files / 974 tests (baseline 46 / 910; +64 includes the 10 drift-regression tests) |
| `effigy test:parity` | 0 — 164 tests |
| `effigy check:svelte` | 0 — 449 files, 0 errors (2 pre-existing TextInput autofocus warnings) |
| `cargo test --manifest-path packages/contracts/components/Cargo.toml` | 0 — 241 tests |
| `cargo clippy --manifest-path packages/contracts/components/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo clippy --manifest-path packages/render/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo test --manifest-path packages/render/Cargo.toml` | 0 — 179 tests (incl. the twelve refusal/forwarding tests) |
| `cargo build --manifest-path packages/gpui/preview/Cargo.toml` | 0 |
| `effigy ci:web` | 0 |
| `git diff --check` | 0 |

`cargo build --manifest-path packages/jetstream/preview/Cargo.toml` fails
identically on the clean base commit: the external `jetstream-poodle` crate
(path dep outside this repo) cannot resolve its `poodle-node` dependency. Not
introduced by this batch; unbuildable environmentally.

## Not done

- No dismissal *behaviour* change, no `resolveDismiss`/escape handling, no
  `WEB_ONLY_PROPS` change, no consumer-repo change (card out-of-scope list).
- No merge, no `git add -A` (staged by explicit path only).
- Six cross-target props remain registered in `BASELINE` with reasons, awaiting
  the spec-surface tranche (see above).
- The jetstream preview build failure is pre-existing and environmental
  (external crate resolution), recorded here rather than fixed.
