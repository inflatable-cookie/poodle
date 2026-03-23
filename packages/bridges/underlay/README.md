# Underlay Bridge

Underlay bridge scaffolding for token aliases, theme translation, and
wrapper-preservation rules that ingest Poodle artifacts without exposing Poodle
directly to Underlay app code.

## Current Purpose

In `g03`, this package is the zero-leak adapter layer between Poodle and
Underlay-owned public APIs.

It exists to let Underlay adopt Poodle internally while keeping:

- Underlay-owned import paths
- Underlay-owned component names
- Underlay-owned prop naming
- Underlay-owned rollout and deprecation posture

It is not:

- a public app-facing UI kit
- a second canonical token source
- a place where Poodle naming becomes the public Underlay contract

## Package Shape

```text
packages/bridges/underlay/
  README.md
  package.json
  css/
    poodle-to-underlay.css
  ts/
    index.ts
    token-map.ts
    theme-map.ts
    component-wrappers.ts
    zero-leak-proof.ts
```

## Ownership Rule

- Poodle owns canonical token meaning and component contracts.
- The bridge owns alias maps and wrapper-preservation guidance.
- Underlay owns app-facing APIs and rollout.

## Zero-Leak Goal

Underlay apps should not need:

- direct Poodle imports
- Poodle token variable names
- Poodle component names
- Poodle-specific prop names

## Current Public Bridge Surface

The bridge currently exports:

- `token-map`
- `theme-map`
- `component-wrappers`
- `zero-leak-proof`

These exports are bridge-owned adoption artifacts. They describe how Underlay
may consume Poodle, but they do not redefine Poodle canonically.

## Current Zero-Leak Proof

The current proof artifact lives in:

- `ts/zero-leak-proof.ts`

It makes the bridge posture concrete by recording:

- the zero-leak rules
- the current wrapper-backed adoption surfaces
- the canonical dependency shape (`bridge-owned`)
- and the remaining adoption friction that later milestones must resolve

The current proof surfaces are:

- `@underlay/ui/Button`
- `@underlay/ui/SearchField`
- `@underlay/ui/Panel`

Each of these surfaces assumes:

- app-facing imports stay Underlay-owned
- Poodle internals may sit underneath wrappers
- bridge-local token aliases and theme maps remain internal implementation detail

## Remaining Adoption Friction

The bridge baseline still leaves these items explicit:

- Underlay theme IDs are placeholders until real Underlay canonical names are supplied
- wrapper prop translation is policy-first until a downstream Underlay wrapper package consumes the bridge
- accessibility parity still needs real downstream wrapper evidence
- the alias map should widen only when concrete Underlay adoption requires it

## Next Task

Use this bridge package while executing
`docs/roadmaps/g03/007-underlay-bridge-hardening-and-zero-leak-adoption-proof.md`
so the first real Underlay adoption proof inherits an explicit zero-leak
artifact instead of relying on README prose alone.
