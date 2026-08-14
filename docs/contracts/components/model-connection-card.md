# ModelConnectionCard

Status: approved
Updated: 2026-08-14
Governing spec: `../../specs/067-model-connection-management.md`

## 1. Purpose

- Component name: `ModelConnectionCard`
- Layer: `composites`
- Summary: a controlled disclosure card for one configured model connection
- In scope: safe summary, readiness, independent disclosure and enable switch,
  closed update accessory, open host details, multiple-instance labels
- Out of scope: readiness derivation, auth enforcement, updates, settings
  persistence, credential values, removal confirmation, or list ownership

## 2. Anatomy

```text
[Root] <section data-open data-readiness data-enabled>
  ├── [Summary row]
  │   ├── [Identity]
  │   │   ├── [Inline leading snippet + title + badges]
  │   │   └── [Route + version]
  │   ├── [StatusIndicator: active readiness or access summary]
  │   ├── [Closed accessory snippet] (closed only)
  │   ├── [Actions snippet]
  │   ├── [Disclosure IconButton]
  │   └── [Switch]
  └── [Details region] (open only)
      └── [Details snippet]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | quiet bordered connection surface | surface, border, radius |
| Status | yes | readiness tone and text | status tokens |
| Leading | no | host-rendered provider mark | icon size, tint |
| Identity | yes | instance/provider/route summary | text hierarchy |
| Closed accessory | no | intended for `UpdateCenter` | chrome sizing |
| Disclosure | yes | independent open control | focus ring |
| Enable switch | yes | independent host preference | Switch tokens |
| Details | open | host-owned settings region | separator, inset, stack gap |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `id` | `string` | — | yes | opaque configured-connection id |
| `title` | `string` | — | yes | instance label or provider label |
| `providerLabel` | `string` | — | yes | provider family display label |
| `routeLabel` | `string \| null` | `null` | no | exact route display label |
| `version` | `string \| null` | `null` | no | observed safe version text |
| `accessSummary` | `string \| null` | `null` | no | ready-state status label; sanitized auth/access summary |
| `readiness` | `ModelConnectionReadiness` | `"unknown"` | no | display posture only |
| `readinessLabel` | `string` | `"Status unknown"` | no | visible/accessibility meaning |
| `open` | `boolean \| undefined` | `undefined` | no | controlled disclosure |
| `defaultOpen` | `boolean` | `false` | no | uncontrolled initial disclosure |
| `isEnabled` | `boolean` | `true` | no | host preference; not readiness |
| `isEnableDisabled` | `boolean` | `false` | no | disables only the Switch |
| `isDisabled` | `boolean` | `false` | no | disables card controls |
| `ariaLabel` | `string \| null` | `null` | no | falls back to title |
| `onOpenChange` | `((open: boolean) => void) \| null` | `null` | no | disclosure request |
| `onEnabledChange` | `((enabled: boolean) => void) \| null` | `null` | no | preference request |

```ts
type ModelConnectionReadiness =
  | "ready" | "checking" | "attention"
  | "unavailable" | "unknown" | "error";
```

### Snippets / Render Props

| Name | Input | Purpose |
|------|-------|---------|
| `leading` | `{ id }` | provider mark |
| `badges` | `{ id }` | route maturity or host labels |
| `closedAccessory` | `{ id }` | closed-only `UpdateCenter` or equivalent |
| `actions` | `{ id, isOpen }` | optional host actions/menu |
| `details` | `{ id, isEnabled }` | forms, access actions, diagnostics, model editor |

### Controlled And Uncontrolled

Disclosure supports controlled/uncontrolled use. Enabled state is always
host-owned and changes only through `onEnabledChange`.

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| closed | default | compact summary and closed accessory |
| open | disclosure | details region; closed accessory omitted |
| enabled/off | `isEnabled` | off card is visually muted; Switch remains clear and readiness remains unchanged |
| checking | readiness checking | readiness label overrides the access summary |
| attention | readiness attention | warning indicator and supplied reason |
| unavailable/error | supplied readiness | honest status; details remain reachable |
| disabled | `isDisabled` | all controls inert, content readable |

### Behavior Machine

Behavior classification: styled-only. Disclosure and switch transitions are
delegated to shared Poodle machinery. The card must not combine their hit areas
or derive one state from the other.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOpenChange` | disclosure control activated | next boolean | no enable change |
| `onEnabledChange` | switch activated | next boolean | no readiness/auth side effect |

## 6. Accessibility

- Root is a labelled section.
- Disclosure button has `aria-expanded` and controls a labelled region.
- Switch has an explicit `Enable {title}` accessible name.
- Disclosure, closed accessory, actions, and Switch are separate tab stops.
- StatusIndicator receives the resolved access/readiness label; colour alone is
  insufficient.
- Opening does not move focus. Closing while focus is inside details restores
  focus to disclosure.
- Native implementation must expose equivalent expanded, enabled, and status
  meaning through its accessibility tree.

## 7. Layout

- Summary uses a flexible two-line identity, then a contextual status
  immediately before compact controls. The provider mark precedes the title
  inline. Ready connections show the access summary; other readiness states
  show their readiness label.
- Details span the full card below a separator.
- Provider marks and controls do not shrink; copy uses `min-width: 0`.
- Disabled connections mute card copy, status, and auxiliary controls without
  muting the enable Switch.
- Card-width container rules move status and controls below identity when the
  card runs narrow, independent of the page viewport. All copy and status share
  the same left edge; the provider mark stays inline before the name, while
  accessories, disclosure, and Switch remain adjacent and right-aligned.

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `color.background.panel`, `color.border.subtle`, `radius.surface` | card |
| Identity | `color.text.primary`, `color.text.secondary`, `color.text.muted` | hierarchy |
| Details | `color.border.subtle`, `space.panel.*`, `space.stack.*` | open region |
| Controls | semantic control-size and focus tokens | compact actions |
| Status | semantic status tokens | readiness |

## 9. Svelte Notes

- Use shared disclosure and switch transitions; do not nest controls inside a
  summary button.
- `closedAccessory` is conditionally mounted only while closed.
- Details use a snippet and retain no host form state.

## 10. GPUI Notes

- Deferred until g14 adoption.
- Native card must preserve independent hit regions and focus restoration.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] open and enabled events remain independent
- [ ] status text, focus order, restoration, and details reachability match

### Tier 2: Visual Parity

- [ ] closed hierarchy, wrapping, open separator, and status tones match

### Tier 3: Implementation Freedom

- [ ] provider marks and details remain consumer-owned composition

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Native implementation deferred | g14 pilot verdict not yet recorded | approved staging delta | active-runtime tranche |

## 13. Approval And Adoption Notes

- contract status: `approved`
- approver: operator, 2026-08-14
- downstream adopter: Nucleus
- future follow-up: shared g14 cases after adoption verdict
