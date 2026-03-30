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
