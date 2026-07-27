# 003 - Native Accessibility

Status: active
Owner: Poodle core
Applies to: every contract with ARIA requirements, on the GPUI and Jetstream targets

## The Fact

**Neither native runtime exposes an accessibility API.** Not partially, not
awkwardly — there is nothing to call.

Verified 2026-07-27 by source search, not by reputation:

| Runtime | Evidence |
|---------|----------|
| gpui 0.2.2 | No `accesskit` dependency in its `Cargo.toml`. No accessibility node type, no role or label API anywhere in `src/`. The only matches for "accessible" are doc-comment prose about unrelated things ("the window is no longer accessible at this point"). |
| Jetstream | No `accesskit` in any crate of the workspace. No aria or accessibility API in `jetstream-ui`. |

## What This Means For `aria_label`

**101 `poodle-specs` structs carry `aria_label`, and on native targets it reaches
nothing.** Both runtimes store it and forward it between components — 95 reads in
GPUI, 8 in Jetstream — and every one terminates in a struct field.

That is not a bug to fix and not a gap to close. The field is still correct to
carry:

- the **web** targets consume it, and the spec is the shared surface
- it records the component's accessible name as *intent*, which is what a
  contract is for
- the day either runtime gains an accessibility API, the name is already
  threaded to the point where it would be handed over

What must not happen is treating it as *done*. A native component with
`aria_label` set is not accessible; it is ready to be.

## Consequences For Planning

- **Do not schedule native accessibility work.** There is no API to build
  against. Effort spent on accessible naming, roles, live regions or focus
  announcement in GPUI or Jetstream is effort spent on nothing until the
  runtime moves first.
- **Do not read the accessibility artifacts as runtime proof.**
  `packages/gpui/native-accessibility-proof.json` is explicit about this in its
  own non-goals — it forbids claiming "mounted assistive-technology proof for
  sections that still only have spec-level or crate-test evidence". Its evidence
  is spec-level and crate-test level. That is the correct reading.
- **The web targets carry the accessibility story.** Svelte and React are held
  to it by the axe sweep, and that is real. Contracts should keep specifying
  ARIA fully; the native deltas are recorded, not designed around.

## What Would Change This

gpui shipping accesskit support is the single upstream event that unblocks the
GPUI half — Zed has the same need, so it is plausible rather than theoretical.
Jetstream is a first-party engine, so its half is a decision rather than a wait.

Revisit when either happens. Until then this is a **forced acceptance**, in the
sense the Tree contract already uses: not a debt anyone can pay down.

## The 48 Contracts This Governs

**48 component contracts carry ARIA requirements inside their GPUI Notes
section** — requirements that cannot be met on gpui 0.2.2. `checkbox.md` is
representative: it requires the indeterminate state to be "accessible to
assistive technology as `aria-checked="mixed"`", and requires exposing "state,
and accessible name through the native accessibility tree". There is no native
accessibility tree to expose anything through.

Those requirements are not deleted or softened. They describe what the component
must do when the runtime can, and they are the specification a future
implementation is measured against. **This document is what makes them
non-binding today**, per the cross-cutting rule convention: a contract
references the rule rather than restating it, and this rule says the native half
is blocked upstream.

A reviewer holding a GPUI component to its contract's ARIA section should read
that section together with this one.

## Prior Record

This was documented before, as one row in `components/tree.md`'s Known Deltas
table — accurate, but scoped to one component when it is a property of the whole
native surface. The initial draft of this document claimed "two other contracts"
were affected; checking rather than asserting turned up 48.
