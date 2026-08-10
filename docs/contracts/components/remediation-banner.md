# RemediationBanner

> **Implementation note**: Implemented in the shared Rust renderer, Svelte, and React.

Status: detailed contract
Updated: 2026-08-10

## 1. Purpose

- Component name: `RemediationBanner`
- Layer: `composites`
- Summary: a persistent, announcing recovery surface for form- or page-level
  error states that require one or two recovery actions. Distinct from
  `InlineRemediation` (field-adjacent) and `Callout` (passive messaging) by
  being announce-capable, action-primary, and dismissible.
- In scope: tone-styled background and border, required title and message,
  announcement mode (polite / assertive / none), up to two remediation actions,
  optional dismiss affordance, role derivation from announce mode
- Out of scope: field-scoped messaging (use `InlineRemediation`), toast
  notifications (use `ToastHost`), form-level status summary (use
  `FormShell.status_summary`)

## 2. Anatomy

```text
[Root .remediation-banner]  <section>
  ├── [Icon .remediation-banner__icon]  (tone-based default)
  ├── [Content .remediation-banner__content]
  │     ├── [Title]  <strong>
  │     └── [Message]  <p>
  ├── [Actions .remediation-banner__actions]
  │     ├── [Primary]  RemediationAction button
  │     └── [Secondary]  RemediationAction button (optional)
  └── [Dismiss .remediation-banner__dismiss]  <button> (optional)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | announcing banner surface | background, border, radius, padding |
| Icon | no | tone-based leading indicator | icon color per tone |
| Content | yes | title + message text container | typography, gap |
| Actions | yes | recovery action row | inline gap, button styling |
| Dismiss | no | close button | size, cursor |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `tone` | `StatusTone` | `"warning"` | no | banner fill, border, and icon color |
| `title` | `string` | — | yes | bold heading text |
| `message` | `string` | — | yes | body message |
| `announceMode` | `AnnouncementMode` | `"polite"` | no | `"polite"`, `"assertive"`, or `"none"` — derives role/aria-live |
| `primaryAction` | `RemediationAction \| null` | `null` | no | primary recovery action |
| `secondaryAction` | `RemediationAction \| null` | `null` | no | secondary recovery action |
| `isDismissible` | `boolean` | `false` | no | shows dismiss close button |
| `dismissLabel` | `string` | `"Dismiss"` | no | accessible label for the dismiss control |
| `onAction` | `(id: string) => void` | — | no | reports the selected action id |
| `onDismiss` | `() => void` | — | no | reports dismiss activation |

### Types

```ts
type RemediationAction = {
  id: string;
  label: string;
  variant: ButtonVariant;
  isDisabled: boolean;
};

type AnnouncementMode = "none" | "polite" | "assertive";
```

### Derived Helpers

- `action_count()` — number of remediation actions set (0, 1, or 2)
- `accessibility_role()` — `"status"` for polite, `"alert"` for assertive, none for "none"

### Controlled And Uncontrolled

- The parent owns visibility and handles `onDismiss`; the banner does not hide itself
- Actions are command-only; the component does not track which was clicked

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| warning | `tone="warning"` (default) | warning-tinted background and border |
| danger | `tone="danger"` | danger-tinted background and border |
| success | `tone="success"` | success-tinted treatment (recovery confirmation) |
| info | `tone="info"` | info-tinted treatment |
| pending | `tone="pending"` | accent-tinted treatment (in-flight recovery) |
| polite | `announceMode="polite"` | `role="status"`, `aria-live="polite"` |
| assertive | `announceMode="assertive"` | `role="alert"`, `aria-live="assertive"` |
| silent | `announceMode="none"` | no live-region semantics |

## 5. Accessibility

- Root element: `<section>` with `aria-labelledby` pointing to title id
- Role derived from `announceMode` via `accessibility_role()`
- Dismiss button has `aria-label="Dismiss"` (or contract-level `dismissLabel`)
- Actions must be keyboard-reachable via Tab; Enter/Space activates

## 6. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root bg | `color.background.panel` (from `background_token()`) | base surface |
| Root border | tone → `color.status.*` (from `border_token()`) | tone accent |
| Icon | tone → `color.status.*` | tone-matched leading indicator |
| Radius | `radius.surface` | banner shape |
| Padding | `space.panel.x`, `space.panel.y` | internal spacing |
| Title | `typography.label` | recovery heading |
| Message | `typography.body`, `color.text.secondary` | recovery guidance |

## 7. Rust Spec

- Rust type: `poodle_specs::RemediationBannerSpec`
- File: `packages/contracts/components/src/remediation_banner.rs`
- Introduced by baseline: `054-gpui-form-validation-and-remediation-composite-baseline`

## 8. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Overlaps with `Callout` visually | RemediationBanner is distinguished by required actions and announce-default | allowed | both exist as complementary shapes |
| GPUI does not emit aria-live | GPUI 0.2 has no ARIA surface | allowed | revisit with GPUI accessibility layer |
