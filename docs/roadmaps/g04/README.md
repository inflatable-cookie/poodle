# g04 GPUI Native Parity, Runtime Proof, And Public Docs Promotion

Status: active
Updated: 2026-03-13

## Context

`g04` begins after `g03` made the hardening, adoption-boundary, accessibility,
release, acceptance, and onboarding baselines explicit. The next bottleneck is
no longer whether Pug has a credible Svelte surface and adoption posture. It is
whether GPUI becomes a real implementation surface rather than mostly token
bindings and matrix-only validation, and whether the shared demo target itself
is coherent enough that side-by-side parity actually means something.

This generation should treat the Svelte implementation as the strongest current
reference, but not as a DOM-specific template to copy mechanically. GPUI work
should imitate Svelte semantics, state models, token usage, and contract
meaning as closely as possible while documenting native-runtime deltas
explicitly where one-to-one imitation would be wrong.

For Loophole-facing shared surfaces, the target is not merely "similar enough"
cross-runtime components. The target is the same UI implemented in Svelte and
GPUI against one contract, with explicit deltas only where the runtime makes a
real difference unavoidable. That requires the Svelte demo surface to become a
deliberate contract-owned target app, not just a dense preview page with broad
section coverage.

## Starting State

- `g03` is complete and explicit
- Svelte tokens, primitives, composites, and workstation shells exist as the
  strongest implementation surface
- GPUI currently has generated token bindings and a multi-app validation matrix
  but not comparable component packages
- Underlay and Loophole adoption boundaries are explicit
- release operations, ecosystem acceptance, and onboarding baselines are
  explicit
- the preview is strong internal docs evidence but not yet a published public
  docs platform

## Exit State

- GPUI implementation order and native-delta posture are explicit
- a GPUI preview or review app exists that mirrors the Svelte preview's review
  coverage closely enough for side-by-side comparison
- GPUI primitives, composites, and workstation-shell foundations exist against
  the same contracts where they are worth sharing
- cross-runtime parity evidence is stronger and less aspirational
- native accessibility, focus, and keyboard posture are documented with real
  GPUI evidence rather than browser inference
- the Svelte demo app is coherent, contract-owned, and broad enough to serve as
  the same target UI GPUI should implement
- at least one meaningful GPUI downstream or reference-app proof exists beyond
  token-only evidence
- docs and onboarding surfaces are strong enough to support external evaluator
  review rather than only internal preview usage

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | GPUI contract audit, parity priority matrix, and implementation order | g03.014 | Foundation | Completed |
| 002 | GPUI theme runtime, token application, and native preview app baseline | 001 | Foundation | Completed |
| 003 | GPUI layout, surface, scrolling, and structural primitives | 001, 002 | Core build | Completed |
| 004 | GPUI action, text-entry, and field primitives | 001-003 | Core build | Completed |
| 005 | GPUI selection, value, feedback, and date or time primitives | 001-004 | Core build | Completed |
| 006 | GPUI overlay, disclosure, navigation, and menu primitives | 001-005 | Core build | Completed |
| 007 | GPUI form, validation, and remediation composite parity | 004-006 | Depth | Completed |
| 008 | GPUI data, browse, detail, picker, and media composite parity | 003-007 | Depth | Completed |
| 009 | GPUI workstation shell, command discovery, and layout orchestration parity | 003, 006, 008 | Workstation | Completed |
| 010 | GPUI native accessibility, focus, keyboard, and assistive-technology proof | 003-009 | Hardening | Completed |
| 011 | Cross-runtime parity report, intentional delta register, and acceptance-harness expansion | 007-010 | Hardening | Completed |
| 012 | Shared demo-app audit, gap register, and target-shape freeze | 009-011 | Alignment | Completed |
| 013 | Cross-runtime demo-app contract, section model, and parity checklist | 012 | Alignment | Completed |
| 014 | Svelte demo-app rebuild, component adoption, and coverage upgrade | 012, 013 | Alignment | Completed |
| 015 | GPUI demo-app parity implementation and side-by-side review | 013, 014 | Alignment | Planned |
| 016 | GPUI downstream reference app and multi-app implementation proof | 015 | Adoption | Planned |
| 017 | Published docs platform, evaluator onboarding, and external release evidence | 015, 016 | Adoption | Planned |
| 018 | Generation closeout and `g05` cutover plan | 016, 017 | Closure | Planned |

## Dependency Shape

```text
001 GPUI Contract Audit / Priority Matrix
  -> 002 Theme Runtime / Native Preview App
      -> 003 Structural GPUI Primitives
          -> 004 Action / Text-Entry / Field Primitives
              -> 005 Selection / Value / Feedback / Date-Time Primitives
                  -> 006 Overlay / Disclosure / Navigation / Menu Primitives
                      -> 007 Form / Validation / Remediation Composites
                          -> 008 Data / Browse / Detail / Picker / Media Composites
                              -> 009 Workstation Shell / Command / Layout Orchestration
                                  -> 010 Native Accessibility / Focus / Keyboard Proof
                                      -> 011 Cross-Runtime Parity Report / Delta Register
                                          -> 012 Demo App Audit / Gap Register
                                              -> 013 Demo App Contract / Parity Checklist
                                                  -> 014 Svelte Demo Rebuild / Coverage Upgrade
                                                      -> 015 GPUI Demo Parity / Side-By-Side Review
                                                          -> 016 Downstream GPUI Reference App Proof
                                                              -> 017 Published Docs / External Onboarding
                                                                  -> 018 Closeout / g05 Cutover
```

## Execution Lanes

### Lane A: GPUI Foundation And Harness

`001 -> 002 -> 003`

### Lane B: GPUI Shared Component Surface

`004 -> 005 -> 006 -> 007 -> 008`

### Lane C: GPUI Runtime Proof

`009 -> 010 -> 011`

### Lane D: Demo Surface Alignment

`012 -> 013 -> 014 -> 015`

### Lane E: Adoption And External-Facing Promotion

`016 -> 017 -> 018`

## Next Task

Open `g04.015` and implement the same shared demo app in GPUI, using the
rebuilt Svelte target for side-by-side review and explicit delta handling.
