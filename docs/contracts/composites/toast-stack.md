# ToastStack

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `ToastStack`
- Layer: `composites`
- Summary: a transient notification stack for low-interruption confirmations, warnings, and recoverable failures
- In scope: toast ordering, title/message copy, optional action affordance, dismissal, and polite live-region posture
- Out of scope: long-lived inline status, blocking errors, background queue persistence, or system notification integration

## 2. Accessibility

- toasts must remain textual and independently dismissible
- transient notifications should announce politely and avoid stealing focus
- higher-severity failure toasts may escalate announcement urgency when the host
  treats them as materially disruptive
- optional actions need explicit names and must not replace the core message
- GPUI-native accessibility mapping notes: GPUI must preserve transient notification meaning and dismiss/action reachability even where there is no web-style live region

## 3. Specimen Definitions

### Interactive Stack

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Success toast | `title="Changes saved"`, `message="Your settings have been updated."`, `tone="success"` | Toast with success styling, dismiss affordance |
| Info toast with action | `title="New version available"`, `message="Update to v2.1 for the latest features."`, `tone="info"`, `actionLabel="Update"` | Toast with info styling, action button, dismiss affordance |
| Warning toast | `title="Rate limit warning"`, `message="You are approaching your API limit."`, `tone="warning"` | Toast with warning styling, dismiss affordance |

The specimen includes an "Add toast" button that appends new toasts cycling through info, success, warning, and danger tones. Dismiss and action handlers remove toasts from the stack. Toasts are rendered in a stacked layout within a positioned container.

## 4. Next Task

Use `ToastStack` for transient confirmations and recoverable warnings while keeping long-lived or blocking conditions on persistent inline surfaces.
