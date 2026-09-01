# MotionPolicyProvider

Status: detailed contract
Updated: 2026-09-01
Governing architecture: `../../architecture/012-semantic-motion-policy.md`

## 1. Purpose

- Component name: `MotionPolicyProvider`
- Layer: foundation
- Summary: scopes the effective host motion policy for descendants without
  adding layout, paint, semantics, focus, or ambient preference discovery
- In scope: full/reduced/frozen propagation, restriction-only nesting, web
  context and style hook, native `RenderContext` projection
- Out of scope: system-preference discovery, semantic timers, component state,
  animation recipes, capture clock control, and presentation size/density

## 2. Anatomy

```text
[Provider boundary; no painted part]
  └── [Children]
```

The web shell may use a `display: contents` host only where framework context
requires one. It contributes no accessibility node or layout box. Native
composition returns the child built with the restricted `RenderContext`.

## 3. Props And Inputs

### Public props

| Prop | Type | Default | Required | Notes |
| --- | --- | --- | --- | --- |
| `policy` | `MotionPolicy` | `"full"` | no | requested local restriction: `"full"`, `"reduced"`, or `"frozen"` |
| `children` | framework child content | — | yes | descendants built under the effective policy |

`MotionPolicy` is one shared closed semantic type. No alias, boolean
`reducedMotion`, arbitrary recipe, or host message enters this provider.

### Resolution

Policies are ordered by restrictiveness:

```text
full < reduced < frozen
```

The effective value is the more restrictive ancestor or requested child value.
An omitted root resolves to `full`. A child `full` under `reduced` remains
`reduced`; a child `reduced` under `frozen` remains `frozen`.

Host integrations may convert a real system preference into the root value.
Poodle does not perform the lookup. Capture and test hosts request `frozen`
explicitly.

### Web context and hook

Svelte and React expose the effective value through framework context. Shared
web styles receive one stable inherited hook:

```text
data-poodle-motion-policy="full|reduced|frozen"
```

The hook carries only the effective value. It is not a second state store and
does not authorize component CSS to bypass the shared policy laws.

### Native construction context

`poodle-render::RenderContext` carries `motion_policy` beside theme, size scale,
and density. A provider derives a restricted child context, invokes a child
builder immediately, and returns the resulting node unchanged. Resolved nodes
do not carry provider metadata.

## 4. States

### Effective states

| State | Visual clocks | One-shot result | Continuous result |
| --- | --- | --- | --- |
| `full` | role-defined | role-defined transition | accepted role loop |
| `reduced` | short opacity only where allowed | immediate semantics; optional opacity continuity | readable static frame |
| `frozen` | none | settled endpoint | canonical static frame |

The provider owns no transition itself. It supplies the constraint components
use when producing their role-specific visual declarations.

### Behavior classification

Framework-free policy machine. The same TypeScript and Rust traces prove root
default, restriction-only nesting, policy tightening, and effective output.

## 5. Events

No component event is emitted. Changing `policy` rebuilds descendants under the
new effective value. It does not fire semantic callbacks.

## 6. Accessibility

- The provider is accessibility-neutral.
- It adds no landmark, grouping role, label, live region, or focus target.
- Policy changes never delay semantic state, focus, keyboard behavior, labels,
  announcements, or removal from the accessibility tree.
- `reduced` is a user-facing motion constraint; `frozen` is deterministic
  evidence policy. Frozen output is not reduced-motion evidence.

## 7. Layout And Composition

- The provider adds no dimensions, padding, margin, gap, overflow, hit target,
  or stacking context.
- `UiPresentationProvider` nesting preserves the current motion policy.
- Motion providers may nest only to restrict further.
- Existing component controls such as `Skeleton.animated=false` remain
  stricter than the provider and win.

## 8. Token Usage

No new token is owned by the provider. Pilot roles reuse current 120ms, 180ms,
and 260ms motion durations and standard/emphasized easing unless implementation
evidence proves a missing semantic role. The policy constrains whether a role
may schedule a clock; it does not redefine token values.

## 9. Svelte Notes

- One scoped context store contains the effective `MotionPolicy`.
- The shell computes `max(ancestor, requested)` before setting context.
- A stable inherited data hook exposes the same effective value to shared CSS.
- Effects and observers clean up on provider teardown; no global media-query
  listener is owned here.

## 10. React Notes

- One context value contains the effective `MotionPolicy`.
- The provider memoizes the resolved value and does not create a second policy
  when only presentation context changes.
- No ambient `matchMedia` lookup lives in the component.

## 11. GPUI Notes

- Expected shared surface: `poodle_render::MotionPolicy` and
  `RenderContext.motion_policy`.
- Expected provider surface: a construction helper that restricts the child
  context and returns the built node unchanged.
- GPUI interprets resolved animation declarations. It does not discover user
  preference or relax the policy.
- Generic opacity and SVG rotation are supported. Translation and scale remain
  named approximations under architecture 012.

## 12. Parity Checklist

- [ ] one closed `MotionPolicy` union/enum exists in TypeScript and Rust
- [ ] missing root preference resolves to full
- [ ] nesting can only move toward reduced or frozen
- [ ] Svelte, React, and Rust composition expose the same effective value
- [ ] presentation scopes do not reset motion
- [ ] frozen descendants schedule no clocks
- [ ] policy changes emit no semantic callback
- [ ] provider adds no layout, accessibility, or focus surface
- [ ] Jetstream remains deferred

## 13. Specimen Definitions

No standalone visual specimen is required. The pilot evidence mounts nested
providers around real disclosure, notification, Tabs, discrete-state, and
loading components and shows the effective value plus scheduled-clock receipt.

## 14. Approval And Adoption Notes

- contract status: promoted from the operator-approved PR #121 packet
- first adopter: `g16.034` five-family motion pilot
- downstream icon morphing and shimmer decisions consume this policy only after
  the pilot lands
