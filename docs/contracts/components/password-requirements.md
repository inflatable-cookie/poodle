# PasswordRequirements

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `PasswordRequirements`
- Layer: `foundation`
- Summary: a UI-only password-policy checklist that evaluates a password
  against caller-supplied requirements
- In scope: checklist rendering, live requirement evaluation, loading/error
  display, shared neutral wording for basic password rules
- Out of scope: fetching password policy, auth-policy fallback defaults,
  password-reset workflow orchestration

## 2. Public Props

| Prop | Type | Default |
|------|------|---------|
| `password` | `string` | `""` |
| `requirements` | `PasswordRequirementsPolicy \| null` | `null` |
| `loading` | `boolean` | `false` |
| `error` | `string \| null` | `null` |
| `title` | `string` | `"Password requirements"` |
| `hint` | `string \| null` | default hint text |
| `loadingLabel` | `string` | `"Loading requirements..."` |

## 3. Data Contract

```ts
type PasswordRequirementsPolicy = {
  minLength: number;
  requireMixedCase: boolean;
  requireDigit: boolean;
  requireSpecial: boolean;
  minStrengthScore?: number;
  description?: string | null;
};
```

## 4. Behavior

- renders a loading state when `loading` is true
- renders the checklist when `requirements` is present
- marks checklist items complete as `password` satisfies each rule
- may render `description` and `hint` below the checklist
- may render a simple inline `error` message when requirements are absent

## 5. Boundary

- callers own policy fetch and retry behavior
- callers own fallback defaults when policy fetch fails
- callers may wrap this primitive in auth-specific adapters or workflow shells

## 6. Accessibility

### Semantics

- Root element uses a descriptive heading or region so assistive technology
  identifies the checklist as a distinct landmark when appropriate
- Requirements list uses semantic `<ul>` with `<li>` items so screen readers
  announce list length and position
- Each requirement item conveys its pass/fail state via `aria-label` text (e.g.
  "Minimum 8 characters — met" / "Minimum 8 characters — not met") so status
  is available without vision
- Color alone must not convey pass/fail status — a checkmark or cross icon (or
  equivalent text indicator) must supplement the color change
- The component should be associated with its password input via
  `aria-describedby` on the input, so that screen readers announce the
  requirements when the password field receives focus
- Loading state uses `aria-live="polite"` or equivalent so the transition from
  loading to loaded is announced
- Error message, when present, uses `role="alert"` or `aria-live="assertive"`
  so it is announced immediately

### Keyboard

| Key | Behavior |
|-----|----------|
| none | not interactive — display-only checklist |

### Focus And Announcement

- Focus entry: not focusable by default (informational display)
- Live-region behavior: parent-owned; callers are responsible for associating
  this component with the password field via `aria-describedby`
- GPUI-native accessibility mapping notes: GPUI must expose list semantics and
  per-item pass/fail status through native accessibility APIs
