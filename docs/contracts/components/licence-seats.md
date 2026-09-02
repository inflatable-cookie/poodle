# LicenceSeats

Status: approved web reference; active native/conformance pending g14.017
Updated: 2026-08-14

## 1. Purpose

- Component name: `LicenceSeats`
- Layer: composite
- Summary: lists authority-reported activation seats and lets the operator
  rename machines or request release of seats other than this machine
- Composes: `Icon`/`EditableLabel`/`IconButton`/`ConfirmAction`
- In scope: decorative machine glyph, inline machine naming, current-machine
  marker, honest unnamed rows, release requests, empty-authority behaviour
- Out of scope: host discovery, platform/hostname/last-seen inference,
  activation policy, seat limits, Longhorn imports

## 2. Data Shape

```ts
interface LicenceSeat {
  machineId: string;
  label: string | null;
  thisMachine: boolean;
}
```

Exact structural mirror. `machineId` is a random command identifier, not human
identity.

## 3. Anatomy

```text
Section (absent when seats=[])
├── Heading: Activated machines
└── List
    └── Row
        ├── Decorative computer icon
        ├── EditableLabel: supplied label | Unnamed machine
        ├── This machine marker (when true)
        └── Danger IconButton → Release ConfirmAction (when false)
```

## 4. Props And Events

### Public Props

| Prop | Type | Default | Required | Notes |
| --- | --- | --- | --- | --- |
| `seats` | `readonly LicenceSeat[]` | `[]` | no | Accepts authority getters directly; empty renders nothing, not `1 seat` |
| `pendingMachineId` | `string \| null` | `null` | no | Disables that row's duplicate release |
| `title` | `string` | `"Activated machines"` | no | Section heading/name |
| `releaseLabel` | `string` | `"Release"` | no | Row action |
| `confirmRelease` | `boolean` | `true` | no | Uses warning confirmation before emitting |
| `size` | `ControlSize \| null` | `null` | no | Shared semantic size |
| `density` | `ControlDensity \| null` | `null` | no | Shared density |

### Callbacks

| Callback | Payload | When |
| --- | --- | --- |
| `onRename` | `{ machineId: string; label: string \| null }` | A machine-name edit is committed; blank names normalize to `null` |
| `onRelease` | `{ machineId: string }` | Other-seat release confirmed/requested |

The component derives `otherSeats` only as `seats.filter(!thisMachine)` for
action availability. It does not derive identity or policy.

## 5. States And Behaviour

| State | Result |
| --- | --- |
| no seats | render nothing; authority does not account for seats |
| labelled | show label verbatim; click/keyboard activation starts inline editing |
| unnamed | show `Unnamed machine`; editing begins with an empty value |
| this machine | show marker; no Release action |
| other machine | show Release action |
| pending | row action disabled/busy; other rows remain available |

Raw `machineId` is never visible, shortened, copied, used as accessible copy,
or placed in a title/tooltip. It may exist in internal keys and callback data.
Several unnamed rows may look alike; that is more honest than inventing host
identity.

Release uses `ConfirmAction` with warning tone by default:

- title: `Release this seat?`
- body: labelled machine name or `Unnamed machine`
- confirm: `Release`

The visible release trigger is a ghost danger `IconButton` using the
`trash-2` icon. Its row-specific accessible name carries the action and
machine identity; the random machine ID remains callback-only.

Rename uses controlled `EditableLabel` composition. Poodle trims the committed
value and emits `null` for a blank name. The host owns persistence and supplies
the resulting `seats`; Poodle does not optimistically rewrite authority state.

### Behavior Machine

Behavior classification: machine-backed via composed EditableLabel and
ConfirmAction. The host owns rename persistence, the release command, and the
resulting seat refresh.

## 6. Accessibility

- Labelled `<section>` containing a semantic list.
- Every row's computer icon is decorative and excluded from the accessibility
  tree; the adjacent label owns machine identity.
- Current machine text is visible and available to assistive technology.
- Each label exposes a row-specific `Rename {label}` or
  `Rename unnamed machine` edit affordance.
- Each release control belongs to its row. Accessible name is
  `Release {label}` or `Release unnamed machine`.
- Confirmation delegates focus trap/restore and announcement to ConfirmAction.
- Pending state is exposed on the action and does not disable the list.

## 7. Layout And Tokens

- Compact settings list. Rows align label/marker left and action right.
- Every row begins with the standard `monitor` icon in secondary text colour.
- Unnamed text uses normal primary text, not placeholder/error styling.
- Current marker is quiet supporting text or neutral Pill.
- Release is a ghost danger `IconButton`; no feature-lock iconography.
- Recipe hooks use `--poodle-recipe-licence-seats-*` names.

## 8. Framework And Runtime Parity

Svelte and React share types, row derivation, copy, CSS, and cases. The web
reference is incomplete until g14.017 adds native composition and full
active-cohort conformance. Jetstream remains a separate deferred backend
admission.

## 9. Acceptance Cases

- empty seats renders no section and no synthetic seat count
- labelled and unnamed rows render honestly
- every rendered row carries one decorative computer icon
- every row can emit its exact machine ID and trimmed new label; blank emits
  `label: null`
- this machine cannot be released
- every other seat can emit its exact machine ID after confirmation
- raw/shortened machine ID never appears in rendered or accessible text
- pending affects only the matching row
- no hostname, platform, last-seen, limit, or entitlement policy appears

## 10. Known Deltas

| Delta | Status | Follow-up |
| --- | --- | --- |
| no native implementation in web-reference PR | incomplete, not accepted parity | g14.017 |

## 11. Rust Binding Notes (g15.007)

Status: **bound** — native implementation lands with `g15.007` (specs,
`poodle-render`, GPUI specimen, headless evidence).

- `LicenceSeatsSpec` is cloneable data: authority seats, pending machine id,
  copy, and the controlled per-row confirm/edit open state (Rust targets own
  overlay/edit open state, like every composed control).
- Rows derive once through `poodle_headless::licence::licence_seat_rows`.
  Raw `machine_id` values are callback data and list keys only: every visible
  and accessible identity is the supplied label or `Unnamed machine`, and two
  unnamed rows look alike on purpose. No hostname, platform, last-seen,
  limit, or entitlement policy appears.
- `LicenceSeatsHandlers` owns the callbacks: `on_rename(machine_id,
  trimmed_label | null)`, edit start, live draft/caret, `on_rename_cancel`,
  `on_rename_restore_display_focus`, `on_release(machine_id)`, and the
  release trigger/cancel. Enter and Escape restore the row's display focus:
  the handler names the machine, and the host's next view-mode paint sets
  `request_focus_machine_id` on that row. Tab/blur commit without that flag.
  Rename persistence, the release command, and the resulting seat refresh are
  host-owned; Poodle never optimistically rewrites authority state.
- Release composes the warning ConfirmAction by default (trigger: ghost
  danger trash IconButton whose accessible name carries the action and
  machine identity) or releases directly when `confirmRelease=false`. Pending
  affects only the matching row's action.
- The GPUI specimen is interactive through the preview host loop; mounted
  regressions drive rename and release through the real dispatch tree.
