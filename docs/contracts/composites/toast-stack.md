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

## 3. Next Task

Use `ToastStack` for transient confirmations and recoverable warnings while keeping long-lived or blocking conditions on persistent inline surfaces.
