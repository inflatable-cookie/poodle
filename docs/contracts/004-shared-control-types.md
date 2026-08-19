# 004 Shared Control Types

Status: active
Owner: Poodle core
Updated: 2026-08-11

Canonical definitions for control types that more than one component contract
uses. Per `README.md` "Cross-Cutting Rules", a component contract **references**
a type defined here and does not restate its members.

## Why This Exists

`ButtonTone` was shared in code by Button, IconButton, and SplitButton — one
TypeScript type, one Rust enum — but each contract declared its own narrower
union inline. The three unions disagreed, the CSS implemented a different subset
again, and nothing detected it. A `tone="success"` Button type-checked, resolved
correctly in the Rust renderer, and silently rendered as default on the web.

One definition, referenced everywhere, is the fix.

## Rules

- **T1 — Single definition.** A type used by two or more component contracts is
  defined here. Component contracts reference it by name.
- **T2 — No narrowing by restatement.** A contract must not restate a shared
  type with fewer members. If a component genuinely cannot support a member,
  that is a documented delta in its "Known Deltas" section with a rationale —
  not a silently narrower union.
- **T3 — Full delivery.** Every member of a shared type must be delivered by
  every consuming component in every runtime that implements the component:
  the web stylesheet, the Rust renderer, and both native adapters. A member that
  type-checks but has no visual treatment is a defect, not an accepted delta.
- **T4 — Derivation over duplication.** Where a family of members shares one
  structure, state the derivation rule once and let each contract's tone
  sections cite it, rather than duplicating derived token tables per member.

## ButtonTone

```
type ButtonTone = "default" | "danger" | "success" | "warning"
```

Default: `"default"`.

Consumers: `components/button.md`, `components/icon-button.md`,
`components/split-button.md`.

Rust: `ButtonTone` in `packages/contracts/components/src/types.rs`, resolved
through `ButtonVariant::{fill_token, border_token, text_token}` for every
`variant × tone` pair.

TypeScript: `ButtonTone` in `packages/svelte/components/src/types.ts` and
`packages/react/components/src/types.ts`.

### Status-tone derivation rule

`default` is the neutral baseline: the accent family for `primary`, the surface
and border families for `secondary`, transparent for `ghost`.

`danger`, `success`, and `warning` are **status tones** and share one structure.
Each component defines its danger treatment explicitly in its own "Token Usage —
Exact Values" section; `success` and `warning` mirror that structure with
`color.status.success` / `color.status.warning` substituted for
`color.status.danger` throughout — fill, border, and text, across idle, hover,
and active, for each of `primary`, `secondary`, and `ghost`.

Status tones stay inside their own color family. They do not fall back to the
generic hover variable system.

A component whose danger structure differs from the family baseline (for
example, IconButton's square control geometry) derives its success and warning
treatments from **its own** danger tables, not from another component's.

### Composition with variant

All four tones compose with all three variants. `variant × tone` is a 3 × 4
matrix and every cell has a defined treatment.

`ButtonVariant::Danger` is retained in Rust for backward compatibility only and
is equivalent to `Primary` + `Danger` tone. It is not part of the authored
vocabulary and must not be used in new specs, specimens, or IR definitions.

## ActiveFill

```
type ActiveFill = "none" | "tint" | "solid"
```

Default: `"tint"`.

Consumers: `components/tabs.md`, `components/navigation-menu.md`.

Rust: `ActiveFill` in `packages/contracts/components/src/tabs.rs`, re-exported
from `packages/contracts/components/src/lib.rs`; resolved by each component's
renderer against its own selection state (`TabsSpec::active_fill`,
`NavigationMenuSpec::active_fill`).

TypeScript: `ActiveFill` in `packages/svelte/components/src/types.ts` and
`packages/react/components/src/types.ts`.

### Semantics

`none` draws no selection fill — selection is carried by the edge and the
selected text colour alone. It is the off value of the fill axis, symmetric
with `ActiveEdge::none`: `block` + `activeFill="none"` +
`activeEdge="underline"` is exactly the deleted `strip` variant (underline
and no fill). `tint` is the accent-tinted selection fill a component defines
for its own active state (Tabs' variant fills, NavigationMenu's `accent-base`
16% open trigger). `solid` fills the active control fully with `accent-base`
and switches the foreground to `text-inverse` — the same token the primary
Button uses on `accent-base`, so the filled control keeps legible contrast
against every accent.

Members are added by ruling and recorded here — a contract restating this
type with fewer members is a T2 violation, but the type is not frozen by
that rule (the former "exactly two members" line predated `ActiveEdge` and
was wrong on both counts: T2 forbids a *contract* narrowing a shared type,
and a recorded ruling is exactly how a member legitimately arrives).

## ActiveEdge

```
type ActiveEdge = "none" | "outline" | "underline"
```

Default: `"none"`.

Consumers: `components/tabs.md`, `components/navigation-menu.md`.

Rust: `ActiveEdge` in `packages/contracts/components/src/tabs.rs`,
re-exported from `packages/contracts/components/src/lib.rs`; resolved by each
component's renderer against its own selection state (`TabsSpec::active_edge`,
`NavigationMenuSpec::active_edge`).

TypeScript: `ActiveEdge` in `packages/svelte/components/src/types.ts` and
`packages/react/components/src/types.ts`.

### Semantics

The border axis is a single enum, not booleans. `outline` and `underline` are
both borders on the active control and conflict on the same property, so they
cannot compose — a boolean pair would admit nonsense combinations and require
suppression rules. `none` draws no edge. `outline` draws the accent border
around the active control (`accent-base` 32% mixed with `border-subtle` for
Tabs, `accent-base` 42% mixed with `border-default` for NavigationMenu).
`underline` draws the accent edge along the inline-end side — bottom when
horizontal, right when vertical. There are exactly three members; a fourth
value is a contract violation (T2), not an extension.

`activeEdge` (the border axis) and `activeFill` (the fill axis) are orthogonal
and compose freely: any edge value combines with any fill value.

## ToneFill

```
type ToneFill = "tint" | "solid"
```

Default: `"tint"`.

Consumers: `components/callout.md`,
`components/remediation-banner.md`.

Rust: `ToneFill` in `packages/contracts/components/src/types.rs`, re-exported
from `packages/contracts/components/src/lib.rs`; each shared renderer resolves
the same solid surface recipe.

TypeScript: `ToneFill` in `packages/svelte/components/src/types.ts` and
`packages/react/components/src/types.ts`.

### Semantics

`tint` preserves each component's existing tone recipe. `solid` is a shared
foreground/background treatment: non-neutral backgrounds are an opaque sRGB
mix of 45% tone base and 55% `color.text.primary`, with the raw tone base as
the border and `color.text.inverse` as the foreground. Neutral solid surfaces
use `color.text.primary` as the background and `color.border.strong` as the
border. Components keep their existing typography and focus-ring behavior.
