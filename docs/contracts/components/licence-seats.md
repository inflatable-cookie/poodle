# LicenceSeats

Status: approved for web reference; native/conformance pending g14.016
Updated: 2026-08-14

## 1. Purpose

- Component name: `LicenceSeats`
- Layer: composite
- Summary: lists authority-reported activation seats and lets the operator
  request release of seats other than this machine
- Composes: `Button`/`ConfirmAction`
- In scope: current-machine marker, honest unnamed rows, release requests,
  empty-authority behaviour
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
        ├── Label: supplied label | Unnamed machine
        ├── This machine marker (when true)
        └── Release ConfirmAction (when false)
```

## 4. Public Props And Events

| Prop | Type | Default | Required | Notes |
| --- | --- | --- | --- | --- |
| `seats` | `LicenceSeat[]` | `[]` | no | Empty renders nothing, not `1 seat` |
| `pendingMachineId` | `string \| null` | `null` | no | Disables that row's duplicate release |
| `title` | `string` | `"Activated machines"` | no | Section heading/name |
| `releaseLabel` | `string` | `"Release"` | no | Row action |
| `confirmRelease` | `boolean` | `true` | no | Uses warning confirmation before emitting |
| `size` | `ControlSize \| null` | `null` | no | Shared semantic size |
| `density` | `ControlDensity \| null` | `null` | no | Shared density |

| Callback | Payload | When |
| --- | --- | --- |
| `onRelease` | `{ machineId: string }` | Other-seat release confirmed/requested |

The component derives `otherSeats` only as `seats.filter(!thisMachine)` for
action availability. It does not derive identity or policy.

## 5. States And Behaviour

| State | Result |
| --- | --- |
| no seats | render nothing; authority does not account for seats |
| labelled | show label verbatim |
| unnamed | show `Unnamed machine` |
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

### Behavior Machine

Behavior classification: machine-backed via composed ConfirmAction. The host
owns the release command and resulting seat refresh.

## 6. Accessibility

- Labelled `<section>` containing a semantic list.
- Current machine text is visible and available to assistive technology.
- Each release control belongs to its row. Accessible name is
  `Release {label}` or `Release unnamed machine`.
- Confirmation delegates focus trap/restore and announcement to ConfirmAction.
- Pending state is exposed on the action and does not disable the list.

## 7. Layout And Tokens

- Compact settings list. Rows align label/marker left and action right.
- Unnamed text uses normal primary text, not placeholder/error styling.
- Current marker is quiet supporting text or neutral Pill.
- Release action is secondary until confirmation; no feature-lock iconography.
- Recipe hooks use `--poodle-recipe-licence-seats-*` names.

## 8. Framework And Runtime Parity

Svelte and React share types, row derivation, copy, CSS, and cases. The web
reference is incomplete until g14.016 adds native composition and full
four-runtime conformance.

## 9. Acceptance Cases

- empty seats renders no section and no synthetic seat count
- labelled and unnamed rows render honestly
- this machine cannot be released
- every other seat can emit its exact machine ID after confirmation
- raw/shortened machine ID never appears in rendered or accessible text
- pending affects only the matching row
- no hostname, platform, last-seen, limit, or entitlement policy appears

## 10. Known Deltas

| Delta | Status | Follow-up |
| --- | --- | --- |
| no native implementation in web-reference PR | incomplete, not accepted parity | g14.016 |
