# Native Presentation Explicitness

Status: closed — promoted into architecture 010 and `g15.043`
Captured: 2026-08-23
Found while auditing `UiPresentationProvider`'s native cascade.

## Observation

Native presentation inputs are not represented consistently enough for an
ambient cascade. Seven component spec files retain `Option<ControlSize>` or
`Option<ControlDensity>`, so their renderer can distinguish an omitted value
from an explicit one. Most of the roster has already collapsed omission into
concrete `md` / `default` values.

Current measured surface:

- 125 component spec files expose semantic `ControlSize` or `ControlDensity`;
- 107 expose concrete `ControlSize` and 117 expose concrete
  `ControlDensity`;
- 103 shared-render modules read one or both fields;
- 168 shared-render modules accept `ThemeProvider` directly.

A provider cannot preserve “explicit child props win” after that distinction
has been erased. Treating `md` / `default` as inheritance would make an
explicit reset impossible. Treating them as explicit would make the provider
ineffective.

## Promotion

The operator approved the clean pre-v1 break on 2026-08-23. Architecture 010
now fixes the shared Rust render context, optional presentation inputs, scoped
construction, and backend boundary. `g15.043` owns implementation. This note is
closed evidence, not a separate work item.
