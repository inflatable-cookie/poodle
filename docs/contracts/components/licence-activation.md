# LicenceActivation

Status: approved web reference; active native/conformance pending g14.017
Updated: 2026-08-14

## 1. Purpose

- Component name: `LicenceActivation`
- Layer: composite
- Summary: renders one host-selected activation model: licence-key entry, or
  account activation with licence-file fallback
- Composes: `Field`, `TextInput`, `EditableLabel`, `FileUpload`, `Button`
- In scope: activation-model selection by prop, local key-format feedback
  through an injected helper, an optional host-owned account-content region,
  invoking an injected account-token provider, switching account activation to
  offline file import, opt-in inline machine naming, file-to-base64 browser
  plumbing, pending/disabled state
- Out of scope: choosing an activation model for the application, account login
  implementation, licence evaluation, persistence, rejection policy,
  entitlement enforcement, Longhorn imports

The host chooses its product model once. A customer is never presented with
key, account, and file activation as three peer tabs.

## 2. Data Shapes

```ts
type LicenceActivationMode = "key" | "account";

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

`LicenceKeyFormat` is supplied only in key mode:

```ts
<LicenceActivation
  mode="key"
  keyFormat={{ parse: parseLicenceKey, isProbablyATypo }}
/>
```

Poodle neither imports nor reimplements those helpers. The raw typed key is
emitted after the helper accepts it; Poodle does not normalize again.

`LicenceAccountTokenProvider` is supplied only in account mode. It is an
activation adapter, not a browser-flow decision. It may open browser OAuth,
complete an embedded credential flow from host-owned state, run a device-code
flow, or use another host policy. It returns the resulting token; `null` means
the journey ended without activation. Poodle never asks the customer to paste
a token.

## 3. Anatomy

### Key mode

```text
Form
├── Heading
├── Licence-key TextInput + local format message
└── Actions
    ├── Machine-name EditableLabel (left; only when opted in)
    └── Activate Button (right)
```

### Account mode

```text
Form
├── Header
│   ├── Heading
│   └── Activate offline | Use account activation ghost Button
├── Current view
│   ├── Account: host account content or default explanation
│   └── Offline: single FileUpload
└── Actions
    ├── Machine-name EditableLabel (left; only when opted in)
    └── Continue with account | Activate submit Button (right)
```

Account activation is the initial and primary view. Offline activation is a
direct, named fallback, not an advanced setting. Key activation is a separate
product model and never appears in account mode.

## 4. Props And Events

### Mode-specific props

The public props form a discriminated union:

| Mode | Required | Rejected |
| --- | --- | --- |
| `"key"` | `keyFormat: LicenceKeyFormat` | `accountTokenProvider`, `accountContent`, `fileAccept` |
| `"account"` | `accountTokenProvider: LicenceAccountTokenProvider` | `keyFormat` |

Account mode additionally accepts:

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `accountContent` | React: `(disabled: boolean) => ReactNode`; Svelte: `Snippet<[boolean]>` | — | Host-owned content for an embedded account journey |
| `fileAccept` | `string \| null` | `null` | Narrows accepted offline licence files |

Key mode additionally accepts:

| Prop | Type | Default | Notes |
| --- | --- | --- | --- |
| `keyCodeInput` | `{ length: number; groups?: readonly number[] \| null; separator?: string \| null } \| null` | `null` | Opts into segmented CodeInput entry; omit for free-form TextInput entry |

`keyCodeInput.groups` is a list of group lengths, not separator positions or a
regular expression. `{ length: 20, groups: [5, 5, 5, 5], separator: "-" }`
produces four five-character visual groups with presentation-only hyphens while
the parser receives one joined string.
The pattern must be a complete positive-integer partition of `length`; an
invalid pattern renders the configured number of slots without visual breaks.

### Account content region

`accountContent(disabled)` renders inside Poodle's account-mode `<form>` and
replaces the default external-flow explanation. The host may supply labelled
login fields, device-code instructions, SSO choices, or other account UI. It
must supply a form fragment, not a nested `<form>`.

Poodle owns the submit button and passes `true` while the component is disabled
or account acquisition is running. The provider may close over the host-owned
field state; pressing Poodle's submit button then calls `acquire()`. Field
validation and authentication-specific errors remain host-owned.

### Shared props

| Prop | Type | Default | Required | Notes |
| --- | --- | --- | --- | --- |
| `mode` | `"key" \| "account"` | — | yes | Host-selected activation product model |
| `pending` | `boolean` | `false` | no | Disables submission while the host command runs |
| `disabled` | `boolean` | `false` | no | Disables fields and actions |
| `title` | `string` | `"Activate licence"` | no | Form heading |
| `machineLabel` | `string \| null` | — | no | Omit to hide machine naming; pass a hostname to seed it or `null` to opt in empty |
| `activateLabel` | `string \| null` | `null` | no | Submit override; otherwise mode/view copy is used |
| `size` | `ControlSize \| null` | `null` | no | Shared semantic size |
| `density` | `ControlDensity \| null` | `null` | no | Shared density |

Default submit copy is `Continue with account` in the account view and
`Activate` in key/offline views.

### Callbacks

| Callback | Payload | When |
| --- | --- | --- |
| `onActivate` | `{ credential: LicenceCredential; label: string \| null }` | Valid form submitted; file bytes have been converted to base64 |

The label is trimmed; empty becomes `null`. Credential contents are never
logged, rendered back after submit, or placed in `data-*` attributes.

`machineLabel` is an opt-in seed, not licence authority data. When omitted,
the form contains no machine-name control and activation emits `label: null`.
When supplied, Poodle renders an inline `EditableLabel`. A non-empty value is
the host-provided default, normally the hostname. `null` or a committed empty
edit displays `unnamed machine` as empty-state and input placeholder copy; that
copy is never stored or emitted as the label.

## 5. States And Behaviour

Internal state: account/offline view, key draft, optional opted-in machine-label
draft, selected file, key result, account acquisition pending, file-read
failure.

Changing `mode` resets account mode to its primary view and invalidates any
selected or pending file read.

### Key mode

1. Submit calls `keyFormat.parse(keyDraft)` synchronously.
2. `ok: true` emits the raw key.
3. `ok: false` does not emit or round-trip.
4. `keyFormat.isProbablyATypo(problem) === true` renders:
   `Check the key for a typing mistake.` Never `invalid`, `fake`, or
   `not recognised`.
5. `tooShort` renders: `This key is too short.` It is distinct from typo copy.

TextInput is the default and preserves lowercase, dashes, whitespace, and I/L/O
confusions exactly for the injected parser. Poodle must not pre-normalize them.

When `keyCodeInput` is supplied, CodeInput owns fixed-length segmented entry
with `numbersOnly=false`. The parser receives CodeInput's joined, length-capped
value; visual group gaps and `separator` text are never inserted into it. Hosts
that must preserve arbitrary separator or whitespace characters from pasted
input use the default TextInput route instead.

At full segmented length, CodeInput calls the injected parser for presentation
feedback: `ok: true` shows its success tick and `ok: false` shows its failure
cross. This check never emits activation and never replaces the submit-time
parse, whose problem kind still owns the distinct typo and too-short copy. The
accessible status says the check passed or failed; it does not call a mistyped
key invalid.

### Account view

- It is the initial account-mode view.
- Without `accountContent`, Poodle renders neutral external-flow copy. The
  provider may then open a browser or start another out-of-surface journey.
- With `accountContent`, Poodle renders that host-owned form fragment. Its state
  remains outside Poodle and may be captured by the provider.
- Submit calls `accountTokenProvider.acquire()`.
- A returned token emits `{ kind: "accountToken", token }` immediately.
- `null` is a quiet cancellation and does not emit.
- Provider failure uses a polite generic account-flow error. It never exposes
  token or credential contents.
- While acquisition is pending, the switch and fields are frozen and the form
  is busy. The label is captured when acquisition begins.
- `Activate offline` switches in place to the file view.
- Poodle renders no token field and performs no token-format inference.

### Offline file view

- It exists only inside account mode.
- Exactly one file.
- Browser adapter reads bytes and emits base64 without a data-URL prefix.
- File name may render; contents never do.
- Read failure is a local polite error and does not emit.
- Returning to account activation or changing mode cancels/invalidates any
  pending read and clears its bytes. Returning offline requires a new file.

### Behavior Machine

Behavior classification: machine-backed through composed TextInput/FileUpload
machinery plus a small pure core submit resolver. No licence transition is
implemented in Poodle.

## 6. Accessibility

- Root is a `<form>` with visible heading.
- Every credential field has a visible label; placeholders are not names.
- Segmented key entry remains one real named input; visual slots and group gaps
  do not become separate accessibility stops.
- The optional machine name is a named inline-edit control. Its empty-state
  copy is visually distinct and never substitutes for its value.
- Enter commits a machine-name edit without submitting activation. A separate
  submit action is still required.
- The host labels and validates controls supplied through `accountContent` and
  respects its `disabled` argument.
- Key/file errors link through `aria-describedby` and use polite status
  announcement, not alert.
- The account/offline switch is a named ghost Button and never submits the form.
- Switching offline focuses the file control; returning online focuses the
  account submit action.
- Invalid submit focuses the relevant field/message.
- Pending state sets `aria-busy` and freezes every action that could change the
  captured submission.

## 7. Layout And Tokens

- There is no route tab strip.
- Account/offline switching preserves the form frame and any opted-in machine
  name.
- The opted-in machine name and primary activation button share the footer row,
  aligned to opposite edges.
- The footer has one additional `stack-sm` separation from the active form
  view.
- The route switch sits at the top right opposite the title and uses an `xs`
  ghost Button with secondary text colour. `Activate offline` carries a
  decorative `cloud-off` icon; the return action carries `user`.
- The header has one additional `stack-sm` separation from the active login or
  file-import view.
- Validation uses shared Field/TextInput token roles.
- Optional segmented key entry uses CodeInput's slot, explicit group-end, and
  separator token roles; LicenceActivation adds no licence-specific styling.
- Recipe hooks use `--poodle-recipe-licence-activation-*` names.

## 8. Framework And Runtime Parity

Svelte and React share data types, submit resolver, validation copy, CSS, and
test cases. File reading is thin framework/browser plumbing. The web reference
is incomplete until g14.017 supplies native file-selection semantics and full
active-cohort conformance. Jetstream remains a separate deferred backend
admission.

## 9. Acceptance Cases

- key mode renders no account or offline route and requires only `keyFormat`
- account mode opens on account activation, renders no key route, and requires
  only `accountTokenProvider`
- account mode exposes a direct `Activate offline` switch and a route back
- no route tabs render
- valid raw key emits once
- key mode defaults to free-form TextInput and may opt into fixed-length
  CodeInput with explicit multi-group and separator presentation
- check-failed and unexpected-symbol problems render typo copy and do not emit
- too-short renders distinct copy and does not emit
- lowercase/dashes/whitespace/confusions reach the injected parser unchanged
- account activation invokes the injected provider; no token-paste field exists
- machine naming is absent unless `machineLabel` is supplied
- a supplied hostname is inline-editable and emits the committed trimmed label
- `machineLabel={null}` displays `unnamed machine` as placeholder copy while
  emitting `label: null` unless the customer enters a name
- account mode supports both default external activation and host-owned embedded
  account content without prescribing browser OAuth or login vocabulary
- submitting host-owned account content invokes the provider against host state
- each available route emits the exact structural credential and optional label
- file base64 excludes the data-URL prefix
- async account acquisition freezes interaction and emits the captured label
- leaving/removing a file invalidates pending and completed file bytes
- pending/disabled blocks duplicate submit
- no Longhorn import or package dependency

## 10. Known Deltas

| Delta | Status | Follow-up |
| --- | --- | --- |
| no native implementation in web-reference PR | incomplete, not accepted parity | g14.017 |
| grouped key entry depended on the new web CodeInput `groups`/`separator` props; Rust CodeInput had a legacy inferred 3+3 split | closed by g15.007 Batch A (§11, code-input §12) | none — explicit groups, separators, and the completion result are native-bound |

## 11. Rust Binding Notes (g15.007)

Status: **bound** — native implementation lands with `g15.007` (specs,
`poodle-render`, GPUI specimen, headless evidence).

- `LicenceActivationSpec` is cloneable data: the activation model, the
  account/offline route, the controlled key draft and its caret, the opt-in
  machine-label draft, local message fields, and the selected file's name and
  bare-base64 payload. Web-native props stay out of the portable spec.
- `LicenceActivationHandlers` (render crate) owns the callbacks: controlled
  key/label edits, the route switch, submit, the generic file browse/remove,
  and the injected key parser used for the segmented entry's presentation
  feedback. The pure submit decision runs through
  `poodle_headless::licence::resolve_licence_submit`; the host executes it on
  submit, updates the spec's local messages, and emits the exact structural
  credential plus the trimmed label. A rejected key shows the distinct
  typo/too-short copy and never emits.
- Key mode composes CodeInput when `keyCodeInput` is supplied (explicit
  groups/separator, `numbersOnly=false`, parser joined value). At full length
  the injected parser drives the tick/cross; this never activates and never
  replaces the submit-time parse. Free-form TextInput is the default.
- Account mode composes optional host-owned `poodle-node` account content
  beside the spec and emits an account-activation request; tokens never
  enter render state. Offline mode composes the generic FileUpload browse
  seam with the host's `fileAccept`; accept/size rules are enforced after
  selection and rejections are reported honestly (GPUI 0.2.2 has no dialog
  filter). File contents are base64 without a data-URL prefix and never
  render.
- `machineLabel` is an opt-in seed on the web; on Rust targets the host seeds
  `spec.machine_label` and drives the draft. `None` hides the control; the
  committed label is trimmed and `null` for blank — `unnamed machine` copy is
  never emitted.
- Data-state roles (`mode`, `route`, `busy`) mirror the web's `data-*`
  attributes; `pending` freezes the submit action (loading button).
