# UpdateStatus

Status: active contract
Updated: 2026-08-13

## 1. Purpose

- Component name: `UpdateStatus`
- Layer: composite
- Summary: the update mechanism — the information, the download, and
  install-and-restart. One component, props in and events out, embeddable
  anywhere: it renders inside `UpdateCenter`'s popover and drops into a settings
  panel without a wrapper.
- Composes: `Button`, `Progress`, `Spinner`, `AlertDialog`
- In scope: the five availability states, the progress states that supersede
  them, deferral and rejection notices, and the install confirmation
- Out of scope: channel selection and configuration (a consumer composition),
  any update authority logic, and any Longhorn knowledge

`UpdateStatus` is **authority-agnostic**. The update authority's reads arrive as
props; commands leave as events. It performs nothing itself — no fetch, no
download, no install. `MessageCenter` is the precedent: data in, callbacks out.

There is no Longhorn dependency, and none is possible. The prop shapes below are
**structural mirrors** of the authority's projections so the bridge is a plain
mapping; Longhorn's generated field maps (`UPDATE_FIELDS`,
`UPDATE_VARIANT_FIELDS`) can be asserted against these shapes and fail on drift.
The component never imports a Longhorn type.

## 2. Anatomy

```text
UpdateStatus
├── Head (state is the dominant element)
│   ├── Spinner (only while checking / verifying / installing)
│   └── Title
├── Body (supporting copy, when the state has any)
├── Progress (only while downloading)
├── Notice (deferral, neutral — or rejection, danger)
│   └── Retry action (when the cause/code permits one)
├── Actions (install + defer on an offer; check on idle/failure)
└── AlertDialog (install confirmation; closed otherwise)
```

## 3. Data Shapes

Structural mirrors of the authority's projections. Do not add or remove fields —
the drift assertion is the bridge's job, not a contract of tolerance.

```ts
type Channel = "production" | "beta" | "nightly";

type OfferReason = "staged" | "below-minimum-version" | "user-initiated";

type InstallManager =
  | "macAppStore" | "homebrewCask" | "flatpak"
  | "snap" | "appImage" | "nix" | "linuxDistribution";

type UpdateAvailabilityProjection =
  | { state: "offer"; version: string; reason: OfferReason; notes: string | null }
  | { state: "upToDate" }
  | { state: "aheadOfChannel"; installed: string; channel: string }
  | { state: "withheldByRollout"; version: string }
  | { state: "managedElsewhere"; version: string; manager: InstallManager };

type UpdateProgressProjection =
  | { state: "idle" }
  | { state: "downloading"; fraction: number | null }
  | { state: "verifying" }
  | { state: "readyToInstall"; version: string }
  | { state: "installing"; version: string };

type DeferralCause =
  | { cause: "userPostponed" }
  | { cause: "workInFlight"; detail: string }
  | { cause: "installationNotWritable"; detail: string }
  | { cause: "externallyManaged"; manager: InstallManager; command: string | null }
  | { cause: "installFailed"; detail: string };

type UpdateRejectionCode =
  | "staleAuthority" | "noOffer" | "unavailable" | "unreachable"
  | "signatureRejected" | "notWritable" | "installFailed";
```

## 4. Public Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `status` | `UpdateControllerStatus` | `{ kind: "idle" }` | The check lifecycle: `idle`, `loading`, `ready`, or `failed` with an `error`. |
| `availability` | `UpdateAvailabilityProjection \| undefined` | `undefined` | The resolved availability, when the check is `ready`. |
| `progress` | `UpdateProgressProjection \| undefined` | `undefined` | The install progress; non-`idle` progress supersedes the availability line. |
| `channel` | `Channel \| undefined` | `undefined` | The channel the install follows; shown with the installed version when up to date. |
| `installedVersion` | `string \| undefined` | `undefined` | The running version; shown when up to date. |
| `deferral` | `UpdateDeferral \| undefined` | `undefined` | A postponed install and its reason. Not a fault — rendered neutrally. |
| `lastRejection` | `UpdateRejectionCode \| undefined` | `undefined` | The last rejection code. The only fault; rendered in danger styling. |
| `aheadOfChannel` | `UpdateAheadOfChannel \| undefined` | `undefined` | The preferred source for the ahead-of-channel message (both versions). |
| `pending` | `boolean` | `false` | Disables the actions while a command is in flight. |
| `observe` | `((observer: () => void) => () => void) \| null` | `null` | The authority's subscription; the component re-derives on each notification. |
| `size` | `ControlSize \| null` | `null` | Explicit semantic size override. |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | Semantic size role used when inheriting presentation scale. |
| `density` | `ControlDensity \| null` | `null` | Explicit density override. |
| `installLabel` | `string` | `"Install and restart"` | Install action label. |
| `deferLabel` | `string` | `"Later"` | Defer action label. |
| `checkLabel` | `string` | `"Check for updates"` | Check action label. |
| `retryLabel` | `string` | `"Try again"` | Retry action label (deferral or rejection notices). |
| `confirmInstall` | `boolean` | `true` | Whether install-and-restart confirms before emitting. It closes the application. |

### `observe` and prop freshness — a real runtime difference

`observe` lets a host pass **plain reads off a non-reactive controller** and
have the component refresh when the authority notifies. That works in Svelte
and **cannot work in React**, and the difference is structural rather than a
defect in either component:

- **Svelte 5** compiles props to lazy getters. Re-reading a prop inside a
  `notify`-tracked `$derived` re-evaluates the host's expression, so the fresh
  value arrives. Every prop that can move under `observe` must therefore be
  read through that derived — a prop read straight in the template is captured
  once at first render and never updates.
- **React** passes props by value. A child re-render triggered by `observe`
  re-runs with the *same* props the parent last rendered, so no amount of
  internal forcing produces a fresh `presence`. A React host must drive its own
  re-render — `useSyncExternalStore` over the same authority is the direct
  equivalent — and `observe` there only serves the nested component.

Measured 2026-08-13: the identical scenario (mount hidden, authority notifies,
expect the trigger) passes in Svelte and fails in React.

## 5. Callbacks

| Callback | When It Fires | Payload | Notes |
|----------|---------------|---------|-------|
| `onCheck` | The check action is activated (idle, failure, or a rejection retry) | — | Re-runs the update check. |
| `onInstall` | The install action is activated and, when `confirmInstall`, confirmed | — | Installs and restarts. The host owns the operation. |
| `onDefer` | The defer action is activated on an offer | — | Postpones the offer ("not now", not "never"). |

## 6. States

### Availability

| State | Title | Body | Actions |
|-------|-------|------|---------|
| `offer` | `Version {version} is available` | notes, when present | install, defer |
| `upToDate` | `You're up to date` | `Version {installed} · {channel} channel`, when known | none |
| `aheadOfChannel` | `You're ahead of your channel` | `Installed {installed} · channel {channel}` | none |
| `withheldByRollout` | `Version {version} exists` | `Not staged to you yet.` | none |
| `managedElsewhere` | `Version {version} is available` | `Managed by {manager}.` | none |

### Progress

Supersedes the availability line while an install runs: `idle` (fall through),
`downloading` (a `Progress` bar — `fraction: null` renders indeterminate, never a
zero bar), `verifying`, `readyToInstall` (install action), `installing`.

### The Five Wrong Messages

Rendered once in core (`updateStatusView`), so the two renderers cannot drift:

1. **A deferral is not a failure.** The command succeeded; `deferral` carries the
   reason (`workInFlight` detail is shown, with a retry). Only `lastRejection` is
   a fault.
2. **`fraction: null` is not zero.** A source without a content length renders an
   indeterminate bar; `fraction: 0` renders a zero bar. The two look different.
3. **`aheadOfChannel` is not `upToDate`.** Both versions sit in the sentence, from
   the `aheadOfChannel` read. It is correct behaviour, not a broken updater.
4. **`managedElsewhere` is information, not an error.** The version and manager are
   shown; `DeferralCause.externallyManaged`'s optional `command` is the upgrade
   command to show.
5. **`signatureRejected` offers no retry.** The artifact did not come from the
   signing key. No retry, and never phrased as a network problem.

## 7. Accessibility

- The state is text, not a form: titles are calm; buttons trail the state.
- The download bar is `role="progressbar"` with an indeterminate form carrying no
  `aria-valuenow`.
- Notices are `role="status"` (polite) — deferral and rejection alike are never
  announced as alerts.
- The install confirmation is an `AlertDialog` (role `alertdialog`) naming that
  the application will close and restart.

## 8. Token Usage

| Part | Recipe Hook | Fallback Token |
|------|-------------|----------------|
| rejection notice border | `--poodle-recipe-update-status-rejection-border` | danger 45% mix |
| rejection notice fill | `--poodle-recipe-update-status-rejection-fill` | danger 10% mix |
| rejection notice text | `--poodle-recipe-update-status-rejection-text` | `--poodle-color-text-primary` |
| danger title | — | `--poodle-color-text-danger` |

## 9. Framework Parity

Svelte and React share the same prop surface, defaults, copy (via the shared
`updateStatusView` derivation), events, and token usage. `observe` re-derives in
both runtimes. There is no native counterpart in this tranche.

## 10. Usage

```svelte
<script lang="ts">
  import { UpdateStatus } from "@inflatable-cookie/poodle-svelte";
  import type { UpdateAvailabilityProjection, UpdateControllerStatus } from "@inflatable-cookie/poodle-core";

  let status: UpdateControllerStatus = { kind: "ready" };
  let availability: UpdateAvailabilityProjection | undefined = {
    state: "offer", version: "1.4.0", reason: "staged", notes: "Bug fixes and improvements.",
  };
</script>

<UpdateStatus
  {status}
  {availability}
  onInstall={startInstall}
  onDefer={postpone}
  onCheck={checkNow}
/>
```
