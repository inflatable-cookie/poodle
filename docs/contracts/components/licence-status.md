# LicenceStatus

Status: approved web reference; active native/conformance pending g14.017
Updated: 2026-08-14

## 1. Purpose

- Component name: `LicenceStatus`
- Layer: composite
- Summary: reports the held licence's usability, trust basis, use window, and
  update window without enforcing entitlement
- Composes: `StatusIndicator`, `TimeAgo`
- In scope: five usability states, supplied attention, supplied usability,
  trust basis, both windows, calm and accurate copy
- Out of scope: entitlement enforcement, feature availability, activation,
  seat management, purchase/renewal workflow, Longhorn imports

The component is a window onto licence state. `usable` and `attention` are
authority reads. It never derives whether a feature should render, enable, or
show a padlock.

## 2. Data Shapes

Structural mirrors of the Longhorn controller reads. Poodle declares these
locally and exports mirror field maps; it never imports Longhorn.

```ts
type LicenceUsability =
  | { state: "active" }
  | { state: "inGrace"; until: number }
  | { state: "useWindowExpired"; at: number }
  | { state: "leaseLapsed"; at: number }
  | { state: "clockRefused" };

type LicenceTrustBasis =
  | { kind: "offlineSignature" }
  | { kind: "remoteAssertion"; checked: number };

type LicenceAttention = "none" | "informational" | "actionable";
```

All authority timestamps (`until`, `at`, `checked`, `useUntil`, and
`updateUntil`) are integer Unix seconds. Poodle core converts them to
milliseconds once at the shared view boundary before passing them to
`TimeAgo`. Callers and renderers must not convert them.

## 3. Anatomy

```text
Section
├── Head
│   ├── StatusIndicator
│   └── State title
├── State body
├── Definition list
│   ├── Use coverage
│   ├── Update coverage
│   └── Trust basis
└── Quiet detail (inGrace only)
```

No action lives in this component.

## 4. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
| --- | --- | --- | --- | --- |
| `usability` | `LicenceUsability` | — | yes | Authority projection; every state renders distinctly |
| `trustBasis` | `LicenceTrustBasis` | — | yes | Shown quietly; contains no credential |
| `useUntil` | `number \| null` | — | yes | Unix seconds; always rendered as its own row |
| `updateUntil` | `number \| null` | — | yes | Unix seconds; always rendered as its own row |
| `usable` | `boolean` | — | yes | Reported through copy/data state only; never gates child controls |
| `attention` | `LicenceAttention` | — | yes | Authority emphasis; not re-derived |
| `title` | `string` | `"Licence"` | no | Section accessible name |
| `size` | `ControlSize \| null` | `null` | no | Shared semantic size |
| `density` | `ControlDensity \| null` | `null` | no | Shared density |

### Callbacks

None. No action lives in this component, so there is nothing to report upwards.

## 5. States And Copy

Copy is resolved once in `poodle-core`; Svelte and React consume the same view.

| State | Title | Treatment | Required meaning |
| --- | --- | --- | --- |
| `active` | `Licence active` | neutral/success | use is currently covered |
| `inGrace` | `Licence active` | neutral; never warning/danger | renewal is pending but use continues until `until` |
| `useWindowExpired` | `Use coverage ended` | actionable | use window ended at `at`; do not mention update coverage here |
| `leaseLapsed` | `Licence confirmation required` | actionable | lease lapsed at `at`; do not call the licence expired |
| `clockRefused` | `Check this machine's clock` | actionable warning | clock moved backwards; never expiry or purchase copy |

The use and update rows are always separate:

- `useUntil === null` → `Use coverage: No end date`
- timestamp → `Use covered until: <time>`
- `updateUntil === null` → `Update coverage: No end date`
- timestamp → `Updates covered until: <time>`

An update window never uses error styling. `informational` is visible where the
operator is already looking and never interrupts.

Trust copy:

- `offlineSignature` → `Verified on this machine`
- `remoteAssertion` → `Confirmed with the server <time>`

### Behavior Machine

Behavior classification: styled-only. The component renders supplied state and
pure core view data. It owns no entitlement or licence transition.

## 6. Accessibility

- Root is a labelled `<section>`.
- State title is a heading under the host's heading context.
- Coverage and trust values use a semantic definition list.
- Timestamps use `<time>`/`TimeAgo` with absolute text available.
- Initial content is not a live region. Hosts announce a material state change
  if their workflow requires it.
- Colour is never the only distinction between states.

## 7. Layout And Tokens

- Compact settings-panel block; no modal, popover, or feature control.
- Status head and definition list stack with semantic spacing.
- `inGrace` uses normal surface/text tokens.
- `informational` may use info text/border tokens.
- `actionable` uses warning or danger only where the state meaning requires it;
  `clockRefused` is warning, not expired-danger.
- Recipe hooks use `--poodle-recipe-licence-status-*` names.

## 8. Framework And Runtime Parity

Svelte and React share types, view derivation, copy, CSS, and tests. The web
reference is incomplete until g14.017 adds the portable interface, shared
cases/specimens, Rust spec/renderer, and GPUI evidence. Jetstream remains a
separate deferred backend admission and is not claimed by that milestone.

## 9. Acceptance Cases

- one case per usability state
- `inGrace` has no warning/danger token role
- `clockRefused` contains clock remedy and no expiry/purchase wording
- use/update windows are two visible rows in all null/value combinations
- both trust bases render distinctly
- 10-digit authority seconds render as their intended modern date, not 1970
- changing `usable` changes reported state only and never disables/hides an
  unrelated action

## 10. Known Deltas

| Delta | Status | Follow-up |
| --- | --- | --- |
| no native implementation in web-reference PR | incomplete, not accepted parity | g14.017 |
