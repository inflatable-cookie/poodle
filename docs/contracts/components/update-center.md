# UpdateCenter

Status: active contract
Updated: 2026-08-13

## 1. Purpose

- Component name: `UpdateCenter`
- Layer: composite
- Summary: the update counterpart to `MessageCenter` and `HistoryCenter` — a
  compact titlebar-grade `IconButton` trigger with a popover that renders
  `UpdateStatus`. It owns presence and attention, `UpdateStatus` owns the content.
- Composes: `IconButton`, `Popover`, `UpdateStatus`
- In scope: presence-driven visibility, attention indication, the popover, and
  pass-through of every update read and command
- Out of scope: the update information itself (that is `UpdateStatus`), channel
  configuration, and any Longhorn knowledge

`UpdateCenter` adds exactly three things over `UpdateStatus`: **visibility** (it
collapses to nothing unless there is something to show), **attention** (the icon
draws the eye on `attention`, is unremarkable on `quiet`), and **the popover**
(`UpdateStatus` doing its job). It never derives presence — the authority
supplies it as `presence`.

## 2. Anatomy

```text
UpdateCenter            (absent entirely when presence is "hidden")
├── Popover
│   ├── Trigger (IconButton, icon `download`)
│   │   ├── Circular progress ring (only while `progress` is `downloading`;
│   │   │   determinate from `fraction`, indeterminate from `fraction: null`)
│   │   └── Indicator dot (only when presence is "attention")
│   └── Surface
│       ├── Header: title
│       └── Body: UpdateStatus
```

## 3. Data Shapes

See `update-status.md` §3. `UpdateCenter` passes the same reads through without
interpretation. `presence` is the authority's predicate:

- `hidden` — **the operator cannot act.** `withheldByRollout` (not staged to
  them) and `aheadOfChannel` (nothing to install). The icon collapses; the
  states still render inside `UpdateStatus`.
- `quiet` — something exists but no press is expected now: work in flight, an
  offer the operator already postponed, or `managedElsewhere`.
- `attention` — an offer, or an artifact downloaded and waiting.

### Why `managedElsewhere` is `quiet`, not `hidden`

The discriminator is **"can the operator act?"**, not "can this application
perform the install?". A Homebrew or App Store install has a real newer version
and a real thing the operator can do — just not through us. Hiding it withholds
news the operator wants and can only otherwise reach by opening settings.

The objection — that an icon promises a button that installs — describes
`attention`, not `quiet`. `quiet` is the unremarkable trigger with no indicator
dot, and it already covers "an offer the operator already postponed", which is
likewise a state where no press is expected. `managedElsewhere` fits that
meaning exactly.

Nothing needs building for it: `updateStatusView` already returns an
install-free variant for this state — `tone: "info"`, "Version X is available",
"Managed by <manager>", and **`actions: []`**. Hosts should pair it with
`DeferralCause.externallyManaged`, whose notice carries the upgrade command
("Upgrade with: `brew upgrade finch`"). Without that pairing the popover still
names the version and the manager, but the operator has to know the command
themselves.

## 4. Public Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `presence` | `UpdatePresence` | — | Required. `hidden` collapses the component entirely. |
| `status` | `UpdateControllerStatus` | `{ kind: "idle" }` | Passed to `UpdateStatus`. |
| `availability` | `UpdateAvailabilityProjection \| undefined` | `undefined` | Passed to `UpdateStatus`. |
| `progress` | `UpdateProgressProjection \| undefined` | `undefined` | Passed to `UpdateStatus`. |
| `channel` | `Channel \| undefined` | `undefined` | Passed to `UpdateStatus`. |
| `installedVersion` | `string \| undefined` | `undefined` | Passed to `UpdateStatus`. |
| `deferral` | `UpdateDeferral \| undefined` | `undefined` | Passed to `UpdateStatus`. |
| `lastRejection` | `UpdateRejectionCode \| undefined` | `undefined` | Passed to `UpdateStatus`. |
| `aheadOfChannel` | `UpdateAheadOfChannel \| undefined` | `undefined` | Passed to `UpdateStatus`. |
| `pending` | `boolean` | `false` | Passed to `UpdateStatus`. |
| `observe` | `((observer: () => void) => () => void) \| null` | `null` | The authority's subscription, re-derived here and passed through. |
| `open` | `boolean \| null` | `null` | Controlled popover open state; bindable. |
| `defaultOpen` | `boolean` | `false` | Initial uncontrolled open state. |
| `placement` | `OverlayPlacement` | `"bottom-end"` | Popover placement hint. |
| `title` | `string` | `"Updates"` | Surface heading, trigger accessible name and tooltip. |
| `ariaLabel` | `string \| null` | `null` | Overrides the surface label. |
| `triggerLabel` | `string \| null` | `null` | Overrides the trigger accessible name. |
| `size` | `ControlSize \| null` | `null` | Explicit semantic size override. |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | Semantic size role used when inheriting presentation scale. |
| `density` | `ControlDensity \| null` | `null` | Explicit density override. |
| `installLabel` | `string` | `"Install and restart"` | Passed to `UpdateStatus`. |
| `deferLabel` | `string` | `"Later"` | Passed to `UpdateStatus`. |
| `checkLabel` | `string` | `"Check for updates"` | Passed to `UpdateStatus`. |
| `retryLabel` | `string` | `"Try again"` | Passed to `UpdateStatus`. |
| `confirmInstall` | `boolean` | `true` | Passed to `UpdateStatus`. |

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
| `onCheck` | `UpdateStatus` emits a check | — | Pass-through. |
| `onInstall` | `UpdateStatus` emits an install | — | Pass-through. |
| `onDefer` | `UpdateStatus` emits a defer | — | Pass-through. |
| `onOpenChange` | The popover opens or closes | `boolean` | The next open state. |

## 6. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| hidden | `presence === "hidden"` | Nothing renders — no button, no reserved space. |
| quiet | `presence === "quiet"` | The icon renders, unremarkable. |
| attention | `presence === "attention"` | The icon renders with an accent indicator dot. |
| downloading | `progress.state === "downloading"` | The trigger glyph becomes a circular progress ring. `fraction` fills the ring; `fraction: null` spins it (indeterminate, never a zero ring). The trigger's accessible name carries the fraction (`Downloading update, 42%`). |
| open | trigger click | Popover opens; `UpdateStatus` renders the current state. |

The popover stays open through an install: progress can run for a while, and the
icon stays `quiet` throughout. `UpdateStatus` owns the progress display; the
centre only hosts it.

## 7. Accessibility

- The trigger is an `IconButton` with an accessible name (`triggerLabel ?? title`)
  and `aria-expanded` while open.
- The popover surface is a labelled dialog.
- The indicator dot is decorative (`aria-hidden`).
- `hidden` presence removes the trigger from the accessibility tree entirely.

## 8. Token Usage

| Part | Recipe Hook | Fallback Token |
|------|-------------|----------------|
| attention indicator | `--poodle-recipe-update-center-indicator-fill` | `--poodle-color-accent-base` |
| progress ring fill | `--poodle-recipe-update-center-ring-fill` | `--poodle-color-accent-base` |

See `update-status.md` §8 for the rejection-notice hooks.

## 9. Framework Parity

Svelte and React share the same prop surface, presence semantics, anatomy, and
token usage. There is no native counterpart in this tranche.

## 10. Usage

```svelte
<script lang="ts">
  import { UpdateCenter } from "@inflatable-cookie/poodle-svelte";
  import type { UpdatePresence } from "@inflatable-cookie/poodle-core";

  let presence: UpdatePresence = "attention";
</script>

<UpdateCenter
  {presence}
  status={{ kind: "ready" }}
  availability={{ state: "offer", version: "1.4.0", reason: "staged", notes: null }}
  onInstall={startInstall}
/>
```
