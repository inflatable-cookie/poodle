# LicenceActivation

Status: approved web reference; active native/conformance pending g14.017
Updated: 2026-08-14

## 1. Purpose

- Component name: `LicenceActivation`
- Layer: composite
- Summary: presents key, account, and licence-file activation as equal routes
  and emits one structural credential plus an optional machine label
- Composes: `Tabs`, `Field`, `TextInput`, `FileUpload`, `Button`, `Callout`
- In scope: route selection, local key-format feedback through injected
  Longhorn helpers, invoking an injected account-token provider,
  file-to-base64 browser plumbing, pending/disabled state
- Out of scope: account login implementation, licence evaluation, persistence,
  rejection policy, entitlement enforcement, Longhorn imports

## 2. Data Shapes

```ts
type LicenceCredential =
  | { kind: "key"; key: string }
  | { kind: "accountToken"; token: string }
  | { kind: "licenceFile"; contentsBase64: string };

type LicenceKeyProblem =
  | { kind: "unexpectedSymbol"; symbol: string }
  | { kind: "tooShort"; minimum: number; actual: number }
  | { kind: "checkFailed" };

type LicenceKeyResult =
  | { ok: true; key: string; grouped: string }
  | { ok: false; problem: LicenceKeyProblem };

interface LicenceKeyFormat {
  parse(input: string): LicenceKeyResult;
  isProbablyATypo(problem: LicenceKeyProblem): boolean;
}

interface LicenceAccountTokenProvider {
  acquire(): Promise<string | null>;
}
```

`LicenceKeyFormat` is injected by the host:

```ts
keyFormat={{ parse: parseLicenceKey, isProbablyATypo }}
```

Poodle neither imports nor reimplements those helpers. The raw typed key is
emitted after the helper accepts it; Poodle does not normalize again.

`LicenceAccountTokenProvider` is also injected. It owns the host's browser or
account flow and returns the resulting token. `null` means the customer
cancelled. Poodle never asks the customer to paste an account token.

## 3. Anatomy

```text
Form
├── Route Tabs (Key | Account | Licence file; equal weight)
├── Route panel
│   ├── Key: TextInput + local format message
│   ├── Account: explanation
│   └── Licence file: single FileUpload
├── Machine label TextInput (shared by all routes; optional)
└── Activate Button
```

All three route triggers remain visible. File import is never under an
advanced/overflow disclosure.

## 4. Props And Events

### Public Props

| Prop | Type | Default | Required | Notes |
| --- | --- | --- | --- | --- |
| `keyFormat` | `LicenceKeyFormat` | — | yes | Host-supplied Longhorn behaviour |
| `accountTokenProvider` | `LicenceAccountTokenProvider` | — | yes | Host-supplied account flow; returns a token or cancellation |
| `defaultRoute` | `"key" \| "accountToken" \| "licenceFile"` | `"key"` | no | Initial selection only; no route is styled primary |
| `pending` | `boolean` | `false` | no | Disables submission while host command runs |
| `disabled` | `boolean` | `false` | no | Disables all fields/routes |
| `title` | `string` | `"Activate licence"` | no | Form heading |
| `machineLabelLabel` | `string` | `"Name this machine (optional)"` | no | Label field copy |
| `activateLabel` | `string` | `"Activate"` | no | Submit button |
| `fileAccept` | `string \| null` | `null` | no | Host may narrow accepted file types |
| `size` | `ControlSize \| null` | `null` | no | Shared semantic size |
| `density` | `ControlDensity \| null` | `null` | no | Shared density |

### Callbacks

| Callback | Payload | When |
| --- | --- | --- |
| `onActivate` | `{ credential: LicenceCredential; label: string \| null }` | Valid route form submitted; file bytes have been converted to base64 |

The label is trimmed; empty becomes `null`. Credential contents are never
logged, rendered back after submit, or placed in `data-*` attributes.

## 5. States And Behaviour

Internal state: selected route, key draft, optional label, selected file, key
result, account acquisition pending, file-read failure.

### Key route

1. Submit calls `keyFormat.parse(rawKey)` synchronously.
2. `ok: true` emits the raw key.
3. `ok: false` does not emit or round-trip.
4. `keyFormat.isProbablyATypo(problem) === true` renders:
   `Check the key for a typing mistake.` Never `invalid`, `fake`, or
   `not recognised`.
5. `tooShort` renders: `This key is too short.` It is distinct from typo copy.

Lowercase, dashes, whitespace, and I/L/O confusions are the injected parser's
job. Poodle must not pre-normalize them.

### Account route

- Submit calls `accountTokenProvider.acquire()`.
- The shared Activate button is the only account submit action.
- A returned token emits `{ kind: "accountToken", token }` immediately.
- `null` is a quiet cancellation and does not emit.
- Provider failure uses a polite generic account-flow error. It never exposes
  token or credential contents.
- While acquisition is pending, routes and fields are frozen and the form is
  busy. The submitted route and label are the values captured when acquisition
  began, so an async completion cannot drift between renderers.
- Poodle renders no token field and performs no token-format inference.

### File route

- Exactly one file.
- Browser adapter reads bytes and emits base64 without a data-URL prefix.
- File name may render; contents never do.
- Read failure is a local polite error and does not emit.
- Leaving the file route or removing the file cancels/invalidates any pending
  read and clears its bytes. Returning to the route requires a new selection.

### Behavior Machine

Behavior classification: machine-backed via composed Tabs/TextInput/FileUpload
machinery plus a small pure core submit resolver. No licence transition is
implemented in Poodle.

## 6. Accessibility

- Root is a `<form>` with visible heading.
- Routes use Tabs' tablist/tab/tabpanel semantics and keyboard behaviour.
- Every credential field has a visible label; placeholders are not names.
- Key/file errors link through `aria-describedby` and use polite status
  announcement, not alert.
- Account activation uses the form's named Activate button, not a token control.
- Pending state sets `aria-busy` on the form and disables submission without
  changing route visibility.
- Focus moves to the first field when a route activates. Invalid submit focuses
  the relevant field/message.

## 7. Layout And Tokens

- Route triggers are one equal-width row where space allows; vertical stacking
  preserves equal prominence at narrow widths.
- Machine label and submit action sit outside the route panels.
- No route uses muted, advanced, secondary, or overflow-only treatment.
- Validation uses shared Field/TextInput/Callout token roles.
- Recipe hooks use `--poodle-recipe-licence-activation-*` names.

## 8. Framework And Runtime Parity

Svelte and React share data types, submit resolver, validation copy, CSS, and
test cases. File reading is thin framework/browser plumbing. The web reference
is incomplete until g14.017 supplies native file-selection semantics and full
active-cohort conformance. Jetstream remains a separate deferred backend
admission.

## 9. Acceptance Cases

- valid raw key emits once
- check-failed and unexpected-symbol problems render typo copy and do not emit
- too-short renders distinct copy and does not emit
- lowercase/dashes/whitespace/confusions reach the injected parser unchanged
- key, account token, and file routes are visible and equally reachable
- each route emits the exact structural credential and shared optional label
- file base64 excludes the data-URL prefix
- account activation invokes the injected provider; no token-paste field exists
- async account acquisition freezes interaction and emits the captured label
- leaving/removing a file invalidates pending and completed file bytes
- pending/disabled blocks duplicate submit but never hides a route
- no Longhorn import or package dependency

## 10. Known Deltas

| Delta | Status | Follow-up |
| --- | --- | --- |
| no native implementation in web-reference PR | incomplete, not accepted parity | g14.017 |
