# EditableLabel Editing-Model Decision Proposal

Status: delegate proposal awaiting operator acceptance or revision
Captured: 2026-09-01
Owner: Poodle Northstar orchestrator
Scope: EditableLabel activation, draft, commit, cancel, blur, and focus
  ownership across the active cohort
Promotion authority: orchestrator after operator acceptance; merge is intake only

This packet proposes exact resolutions for the choices named by
`docs/handoffs/20260901-230406-editable-label-decision-planning.md`. It is not
contract, roadmap, implementation, or merge authority.

## Settled Decisions Preserved

- Plan the decision. Do not keep the lane blocked and do not accept current
  runtime behavior by default.
- TextInput's accepted normalization, max-length, composition, and focus
  boundaries remain authoritative where this control reuses them.
- Current Enter, Escape, and blur/Tab evidence is input, not contract
  authority, where runtimes disagree.
- Optimistic persistence, conflict resolution, rich text, and multiline stay
  out of the primitive.
- Svelte is the reference implementation for what the control can do. Native
  mechanism may differ; observable result must match.
- Jetstream stays deferred. Pre-1.0: no aliases, shims, or silent fallbacks.

## Delegate Authority

No operator conversation occurred in this delegate thread. The handoff said
not to re-ask the settled choices and to surface a reviewable recommendation.
That authorizes an exact delegate proposal; it does not make the proposal an
operator decision.

Every exact name and law below is a delegate recommendation. The operator must
accept, revise, or reject it before canonical promotion. PR merge is intake
for that review, not acceptance of the API or behavior.

## Audit Summary

Active cohort today: Svelte, React, shared Rust composition, GPUI. Jetstream
is compile-only.

| Surface | What is true now |
| --- | --- |
| Contract `editable-label.md` | Host-owned committed `value`; internal `isEditing`/`draftValue`; `onCommit({ value, previousValue })`; Enter/blur commit; Escape cancel; Tab commits because blur commits; focus restoration claimed for every commit/cancel |
| Shared web machine `packages/core/src/edit.ts` | `editLabelTransition`: start seeds draft from committed value; commit trims and always emits; cancel restores; start blocked when `disabled` or `canStartEdit` is false |
| Svelte / React | Internal editing and draft; raw `<input>`, not TextInput; HTML `maxlength`; no `startEditing`; programmatic mode has no start API; no focus restore after Enter/Escape; default `activationMode="doubleClick"` has no keyboard entry |
| Rust spec | Host-owned `is_editing` and `value`; `value` is the painted string in both modes |
| Native render | `on_change` streams live text; `on_commit(&str)` has no `previousValue`; commit does not trim; DoubleClick and EnterOrSpace both use `on_activate` (single click); caret-at-end subset; `max_length` via `poodle_headless::text_input`; Tab commits through blur (g16.008) |
| GPUI mounted tests | Enter commits once, Escape cancels, Tab blurs then advances; activation, select-on-focus, and focus restoration explicitly unclaimed |
| In-repo composites | LicenceActivation and LicenceSeats: web uses internal editing + `onCommit`; native drives `machine_label_editing` / `editing_machine_id` and live `on_change` |
| Downstream Svelte | `loophole` inspector: default doubleClick, async persist, skips unchanged; `nucleus` threads: doubleClick, extra trim, skips empty/unchanged, async rename; `soundcheck-library`: enterOrSpace, extra trim, skips empty/unchanged |
| Stale `docs/parity/editable-label.md` | Historical g12-era GPUI/Jetstream gap list. Not authority |

The decision gate exists because the contract, web adapters, and native path
already disagree on the same laws. g16.008 proved Tab-via-blur routing. It
did not choose the public model.

## Proposed Public Shape

### Committed value

Host-owned `value: string`. Always a string. Empty is a valid committed
unnamed state.

- No `defaultValue` and no uncontrolled committed store. This is a rename
  widget, not a form field that owns persistence.
- Omitted web `value` renders `""`. The contract may keep `value` required in
  meaning (there is always a committed string) while adapters default the
  omitted prop to `""`.
- Svelte `$bindable` remains host-echo convenience, not a second value model.
- The host applies or ignores `onCommit`. If it ignores the callback, view
  mode shows the unchanged `value`.

Do not add a live public draft channel on web (`onChange`, `draftValue`).
Draft is session-private until commit or cancel.

### Editing ownership

| Runtime | Default owner | Host override |
| --- | --- | --- |
| Svelte / React | Adapter owns `isEditing` and draft | None in this proposal. Programmatic entry is `startEditing()` / `cancelEditing()` |
| Native spec | `is_editing` is resolved render state | Composites may keep driving it (LicenceActivation, LicenceSeats) |
| GPUI standalone | Session/host wrapper owns editing, draft, selection, and focus between rebuilds | Same wrapper exposes start/cancel; it writes `is_editing` into the spec |

This is the NumberInput pattern: declarative spec carries resolved state;
the wrapper retains the session. It is not a public controlled `editing` prop
on web.

### Callbacks

Keep the current web names. Make native match them.

| Callback | When | Payload |
| --- | --- | --- |
| `onEditStart` | view → editing | none |
| `onCommit` | editing → view by Enter, Tab/blur, or pointer/window blur | `{ value: string; previousValue: string }` |
| `onCancel` | editing → view by Escape or `cancelEditing()` | none |

Native `on_commit: Fn(&str)` is a clean pre-1.0 break. Replace it with the
same `{ value, previousValue }` meaning. No alias.

No public `onChange` for keystrokes. Native `on_change` may remain as a
render-layer channel for the wrapper or a composite that already stores a
draft; it is not the portable commit contract.

### Programmatic API

`activationMode="programmatic"` blocks every built-in pointer and keyboard
gesture. It does not mean the control cannot enter edit.

Web methods (Svelte instance / React handle), native wrapper methods:

- `startEditing()` — allowed unless disabled or already editing. Ignores
  `activationMode`.
- `cancelEditing()` — no-op unless editing; same as Escape.

The machine keeps `canStartEdit` as a **gesture** guard. API start is a
distinct event that still checks disabled and view state.

## Draft Law

- Draft exists only while editing.
- `START_EDIT` seeds draft from the current committed `value`, never from a
  stale buffer.
- Draft is not trimmed while typing.
- `COMMIT` trims leading and trailing Unicode whitespace, then reports the
  trimmed string. Interior spaces stay.
- Empty after trim is a valid committed value. Mapping `""` → `null` for a
  domain label remains the host's job (LicenceSeats and loophole already do).
- `maxLength` uses TextInput / `poodle_headless::text_input` character-count
  enforcement (`chars().count()`). Over-long insert truncates to fit;
  a keystroke into a full field is consumed and reports nothing.
- Unchanged accepted keystrokes are silent on any live native channel
  (g16.007 `report_edit` rule).
- External committed `value` replacement while editing discards the draft and
  returns to view with the new value. No `onCommit`. A host echo of the value
  just emitted by this session must not restart or cancel the session.
- Becoming disabled while editing cancels, restores the last committed
  display, and emits `onCancel`.

Do not compose the TextInput component. Anatomy, chrome, validation, and
affixes are wrong for a label. Reuse the headless text transitions and
TextInput's composition, caret, undo, and max-length ownership.

## Activation Law

Keep `activationMode: "doubleClick" | "enterOrSpace" | "programmatic"`.
Default remains `"doubleClick"` so list and sidebar rows can use single-click
for selection (nucleus threads, loophole inspector).

| Mode | Pointer | Keyboard on the display | Native channels |
| --- | --- | --- | --- |
| `doubleClick` | Double-click only | Enter or Space starts edit | `on_double_activate`; not `on_activate` |
| `enterOrSpace` | Single click | Enter or Space starts edit | `on_activate` |
| `programmatic` | None | None | Neither; API / host `is_editing` only |

Keyboard Enter/Space on the display in `doubleClick` is an intentional
behavior change. Today's web default is pointer-only, which fails keyboard
use. Native today maps both non-programmatic modes to `on_activate`, so
default GPUI is click-to-edit. That is evidence of drift, not the law.

Disabled blocks every start path, including `startEditing()`.

`selectOnFocus` default stays `true`: on edit entry, select the whole draft
using TextInput character indices. When `false`, place a caret at the end.
Native caret-at-end-only editing is a gap to port, not an accepted delta.

## Commit, Cancel, And Blur Law

```text
[view] --allowed activation or startEditing--> [editing]
[editing] --Enter--> [view] commit + restore focus to display
[editing] --Escape or cancelEditing--> [view] cancel + restore focus to display
[editing] --Tab / Shift+Tab--> [view] commit, focus continues
[editing] --pointer or window blur--> [view] commit, no restore
```

- Enter commits and prevents ancestor form submit. Direct commit, not a
  simulated blur.
- Escape cancels. Restore the committed display. No `onCommit`.
- Tab and Shift+Tab are sequential traversal. Focus leaves first; the
  resulting blur commits once. Do not restore to the display. This is the
  g16.008 observable result and the only law that lets Tab leave the control.
- Pointer blur and window blur commit once and do not restore.
- After cancel, a trailing blur must not emit commit. The machine already
  no-ops `COMMIT` in view; React adapters must hold mode in a ref so a
  same-tick unmount blur cannot see stale `isEditing`.
- Enter then blur of the unmounting input must not emit a second commit.
  Same guard.
- Teardown of the whole control while editing does not invent a second
  commit. If the runtime delivers a real blur, the blur law applies once.

The current contract line "commit or cancel returns focus to the display
label" is too broad. Restore only for Enter and Escape / `cancelEditing()`.

## Unchanged-Result Law

- Keystroke or insert that does not change accepted text: silent.
- Commit where trimmed draft equals `previousValue`: still fire `onCommit`.
  The session completed. Hosts already skip persistence (loophole, nucleus,
  soundcheck-library). The machine must not hide the boundary.
- Cancel: never `onCommit`.

## Async Policy

The primitive does not persist, validate, retry, or roll back.

- `onCommit` is synchronous notification of an accepted session.
- Hosts apply immediately, persist asynchronously, and pass `disabled` while
  saving if they need to freeze the row (nucleus, soundcheck-library).
- Failure to persist is host-owned: keep or revert `value`, show host error.
- No `validate`, pending, or error props on EditableLabel.
- NumberInput's async `validate` is not reused here.

## Focus And Selection Restoration

- Entering edit moves focus into the input. `selectOnFocus` then selects all
  or leaves a caret at the end.
- Enter / Escape / `cancelEditing()` restore focus to the display control.
- Tab / pointer / window blur do not restore.
- Display remains the only tab stop in view mode. The input exists only while
  editing.
- Web `focus()` on the component focuses the display in view mode and the
  input while editing. Native focus stays backend-owned, as in TextInput.
- Do not claim a new focus manager.

## Accessibility

- View: button-like, keyboard-reachable, except when disabled.
- Edit: standard single-line text input.
- Accessible name, in order: explicit `ariaLabel` if provided; else the
  visible value; else `emptyText`; else `"Edit label"`.
- Change the prop default from `"Edit label"` to `null`. Today's default
  overwrites the visible name, so AT hears "Edit label" instead of the row
  title. Hosts that pass `ariaLabel="Rename …"` keep current behavior.
- Apply the same resolved name to the display and the input.
- Edit icon stays `aria-hidden`.
- No live region.
- Do not claim GPUI assistive-technology proof in the first implementation
  card. Node name, role, disabled, and value exposure follow existing native
  a11y metadata; contract 003 remains the evidence boundary.

## Migration

Pre-1.0 clean break. No compatibility aliases.

| Change | Who feels it | Host action |
| --- | --- | --- |
| Keyboard Enter/Space starts edit in `doubleClick` | loophole, nucleus, default specimens | Usually desired. Single-click still does not rename |
| Enter/Escape restore focus to display | All web hosts | Tab order after Enter stays on the label |
| Default `ariaLabel` becomes `null` | Hosts that omit `ariaLabel` | Visible text becomes the name; pass `ariaLabel` to keep an action name |
| Native `on_commit` payload | In-repo Rust callers | Switch to `{ value, previousValue }` |
| Native DoubleClick uses `on_double_activate` | GPUI default specimens, any host relying on click-to-edit under DoubleClick | Use `enterOrSpace` if they wanted click |
| Native commit trims | Native hosts that stored raw spaces | Align with web |
| Native live `on_change` leaves the public contract | LicenceActivation native draft | Keep as composite/wrapper plumbing, not portable API |
| `startEditing` / `cancelEditing` | New | Additive |

In-repository LicenceActivation and LicenceSeats migrate in the
implementation card. Downstream loophole, nucleus, and soundcheck-library are
inspected, not edited.

`docs/parity/editable-label.md` is historical. Do not repair it in this
packet. Implementation may delete or archive it if docs policy requires;
that is not a second decision.

## Parity Deltas

Accepted:

| Delta | Why |
| --- | --- |
| DOM swap vs native entity-state swap | Renderer architecture; same visual and semantic result |
| Platform double-click timing | Pointer ergonomics; keyboard entry stays strict |
| Web `focus()` / `startEditing()` vs native wrapper / `is_editing` | Imperative vs declarative hosts; same observable session |
| Web focus event objects | Not portable value semantics |
| HTML `maxlength` UTF-16 vs headless Unicode scalar count | Web platform attribute; accepted text still goes through the shared character-count rule where the adapter owns insertion |
| Caret, IME, undo, and composition mechanism | TextInput adapter-owned; observable accepted text and selection match |
| Jetstream editing evidence | Program-deferred |

Rejected as deltas (must port):

- Native `on_activate` for `doubleClick`
- Native `spec.value` as the live draft with public `on_change` as the
  commit model
- Native no-trim commit
- Native caret-at-end as the only editor
- Missing `selectOnFocus`
- Missing keyboard entry in `doubleClick`
- Missing Enter/Escape focus restoration
- Missing programmatic start API
- Treating g16.008 Tab-commit evidence as a Tab-is-submit contract

## Required Review Oracles

| Invariant | Smallest adversarial counterexample | Expected failure or stop | Required proof |
| --- | --- | --- | --- |
| Committed value is host-owned | `onCommit` ignored after rename | Display shows previous `value` | Svelte/React focused test |
| Draft is session-private | Host has no `draftValue` and a keystroke emits a public change callback | Public surface audit failure | Prop/callback audit plus web tests |
| Start seeds from committed value | Edit, type, Escape, edit again | Second session shows original, not abandoned draft | Machine vector + web/native |
| Commit trims once | Draft `"  Take  "` | `onCommit` value `"Take"`; `previousValue` original | Shared machine + all active adapters |
| Unchanged commit still emits | Enter with no edit | One `onCommit` with equal value and previousValue | Machine + web; hosts may skip persist |
| Unchanged keystroke is silent | Native insert with no room under `maxLength` | No `on_change` | Headless text_input + native render |
| Escape then blur emits no commit | Escape, then unmount blur in React | `onCancel` once; `onCommit` absent | React focused test with ref-guard |
| Enter then blur emits one commit | Enter, input unmounts | One `onCommit` | Svelte, React, GPUI mounted |
| Tab commits via blur and advances | Tab after typing | Commit then next tab stop focused; display not refocused | GPUI mounted (extend g16.008) + web |
| Enter restores display focus | Enter after typing | Display button/node is `activeElement` / focused id | Web focused + GPUI mounted |
| DoubleClick pointer does not start on single click | One click in default mode | Stays view | Svelte, React, GPUI pointer |
| DoubleClick keyboard can start | Enter on focused display in default mode | Enters editing | Svelte, React, GPUI key |
| Native DoubleClick is not `on_activate` | GPUI click on default specimen | Stays view unless second click | Render channel assertion + mounted |
| enterOrSpace click starts | One click | Enters editing | All active adapters |
| programmatic ignores gestures | Click, double-click, Enter, Space | Stays view | All active adapters |
| `startEditing` works in programmatic | API call while enabled | Enters editing, `onEditStart` | Web methods + native wrapper |
| Disabled blocks start and cancels an open session | Disable while typing | `onCancel`; committed value unchanged | All active adapters |
| External value replacement while editing | Host sets a new `value` mid-edit | View mode, new value, no `onCommit` | Web + native wrapper |
| Echo of just-committed value does not restart | Host writes `onCommit.value` back | Stays view | Web + native wrapper |
| `selectOnFocus` true selects all | Start edit `"Studio"` | Selection covers the draft | Web `select()`; native selection range |
| `selectOnFocus` false caret at end | Same, flag false | Caret after last character | Web + native |
| `maxLength` is character-count | Value length 6, max 6, insert `"x"` | No accepted change | Shared text_input + adapters |
| Accessible name prefers visible text | Omit `ariaLabel`, value `"Kick"` | Name is `"Kick"`, not `"Edit label"` | Web a11y assertion |
| Same name on input | Start edit with `ariaLabel="Rename Kick"` | Input name `"Rename Kick"` | Web + native label |
| LicenceSeats empty commit | Flush row, commit `""` | Host still maps to `null`; primitive emits `""` | In-repo composite regression |
| Jetstream stays deferred | Mounted Jetstream claim or passing ledger cell | Scope failure | Changed-file and ledger review |

## Proposed Implementation Card Split

One g16 implementation card after operator acceptance and canonical
promotion. Do not number it from this packet. Do not split web and native
into two cards: that is the current ownership split.

Batches inside that one card:

1. **Contract and paired machine.** Rewrite `editable-label.md`. Port
   `editLabelTransition` into idiomatic Rust headless beside the TypeScript
   machine. Add a bounded vector corpus for start, draft, trim-commit,
   unchanged commit, cancel, gesture vs API start, disabled cancel, and
   external replacement. Reuse `poodle_headless::text_input` for max-length
   and insert; do not fork it.
2. **Web adapters.** Svelte and React: ref-guarded commit-after-cancel,
   keyboard entry in `doubleClick`, Enter/Escape restore, `ariaLabel` default
   null, `startEditing` / `cancelEditing`, TextInput max-length rule.
   Focused tests. Curated specimens stay documentation, not a case corpus.
3. **Native spec, render, wrapper, in-repo composites.** `on_double_activate`
   vs `on_activate`; trim + `{ value, previousValue }`; session wrapper for
   standalone GPUI; migrate LicenceActivation / LicenceSeats Rust hosts.
   Mechanical Jetstream compile only.
4. **Mounted proof and closeout.** Named GPUI regressions for the oracle
   rows that native can drive. Move only EditableLabel's GPUI mounted-behavior
   ledger cell. Accessibility and visual-comparison cells do not move.

Stop the card if the operator changes commit-vs-restore, public draft
ownership, or activation mapping. Stop if IME, multiline, validation, or a
new focus architecture is required to land the envelope.

## Proposed Canonical Destinations

| Meaning | Destination after packet acceptance |
| --- | --- |
| Public props, callbacks, activation, commit/blur/focus, a11y, deltas | `docs/contracts/components/editable-label.md` |
| Pure view/editing machine | `packages/core/src/edit.ts` and a paired Rust headless module; contract Behavior Machine section |
| Text insert, max-length, selection, composition ownership | Reuse TextInput contract §6 and `poodle_headless::text_input`; project the reuse into EditableLabel notes |
| Native session ownership | Contract active-cohort notes plus the GPUI wrapper; NumberInput's wrapper note is the pattern, not a shared type |
| Implementation sequencing, scope, stops, validation | One g16 card after promotion |
| Register / runway / generation index | Clear the EditableLabel decision-blocked row after that card merges |
| Jetstream | Mechanical spec compile only until backend admission |

The orchestrator chooses the final promotion split. No implementation card is
ready from this packet alone.

## Alternatives Not Selected

| Alternative | Reason |
| --- | --- |
| Accept current behavior as the contract | Handoff forbids treating disagreeing Enter/Escape/blur evidence as authority |
| Public controlled `draftValue` like NumberInput | Draft is a rename session, not a standing raw field. Live draft would force every host to echo keystrokes |
| Uncontrolled committed `defaultValue` | Persistence is host-owned; an internal committed store hides failed applies |
| Controlled web `editing` prop | Extra value model. Native already has resolved `is_editing`. Web programmatic needs methods, not a second source of truth |
| Compose TextInput | Wrong anatomy, chrome, validation, and size. Reuse headless text rules instead |
| Blur cancels | Current web, contract, Licence*, and g16.008 all commit on blur. Cancelling would drop inspector and sidebar renames on click-away |
| Skip `onCommit` when unchanged | Hosts already filter. Hiding the session boundary makes "Enter to finish" indistinguishable from a swallowed key |
| Restore focus on every commit, including Tab | Tab could not leave the control. Contradicts g16.008 |
| Keep default `ariaLabel="Edit label"` | Overwrites the visible name. Breaking the default is the a11y fix |
| Change default activation to `enterOrSpace` | Steals single-click in nucleus/loophole rows. Keep double-click pointer; add keyboard |
| Native stays fully host-driven with public `on_change` | Makes GPUI the source of the value model and leaves standalone use unusable |
| Two implementation cards (web then native) | Recreates today's split and delays the mounted cell that unblocks the register |
| Add async validate / pending | Contract already assigns optimism to the app. Downstream hosts already pass `disabled` while saving |

## Explicit Non-Goals

- Implementing code, tokens, specimens, tests, or ledger movement in this
  packet.
- Promoting contracts, architecture, specs, roadmaps, or a ready card.
- Rich text, multiline, slug, conflict resolution, collaborative cursors.
- Optimistic-update policy inside the primitive.
- Visual comparison, broad GPUI assistive-technology evidence, IME
  certification beyond TextInput's current boundary.
- Jetstream behavior, evidence, or admission.
- Editing sibling repositories.
- Repairing the historical `docs/parity/editable-label.md` in this PR.
- A public live `onChange` or controlled draft on web.

## Unresolved Questions

Operator acceptance or revision of this exact proposal remains open. That
gate includes committed-value ownership, internal draft, activation mapping,
commit/cancel/blur/restore split, unchanged `onCommit`, async-out-of-component,
`ariaLabel` default, native payload break, and the one-card split.

Merge is intake for that review, not acceptance. Promotion may expose
integration drift against newer `main`; the orchestrator owns that
reconciliation after the operator gate.

## Evidence Used

- `docs/handoffs/20260901-230406-editable-label-decision-planning.md`
- `docs/contracts/001-working-rules.md`
- `docs/contracts/components/editable-label.md`
- `docs/contracts/components/text-input.md`
- `docs/contracts/components/number-input.md`
- `docs/architecture/006-headless-core-and-machine-model.md`
- `docs/roadmaps/g16/007-text-input-controlled-editing-and-mounted-evidence.md`
- `docs/roadmaps/g16/008-native-text-event-routing-cleanup.md`
- `docs/roadmaps/g16/030-number-input-value-draft-and-mounted-parity.md`
- `docs/roadmaps/g16/component-continuation-register.md`
- `docs/roadmaps/g16/component-continuation-runway.md`
- `packages/core/src/edit.ts`, `packages/core/test/edit-code-token.test.ts`
- `packages/svelte/components/src/EditableLabel.svelte` and focused test
- `packages/react/components/src/EditableLabel.tsx` and focused test
- `packages/contracts/components/src/editable_label.rs`
- `packages/render/src/editable_label.rs`
- `packages/render/src/licence_activation.rs`, `packages/render/src/licence_seats.rs`
- `packages/svelte/components/src/LicenceActivation.svelte`, `LicenceSeats.svelte`
- `packages/gpui/preview/src/specimens/editable_label.rs`
- `packages/gpui/preview/tests/headless_regressions.rs` EditableLabel routing
- Inspected, not edited: loophole inspector track rename; nucleus thread
  titles; soundcheck-library organization rename
