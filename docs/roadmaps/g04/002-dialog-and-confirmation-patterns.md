# g04.002 Dialog And Confirmation Patterns

Status: completed
Owner: Flint Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `flint`

## Goals

- [ ] implement AlertDialog as a focused primitive for destructive-action
  confirmation
- [ ] implement FormDialog as a composite combining Dialog with form layout and
  validation
- [ ] implement ConfirmAction as a composite providing inline confirmation flows
  for dangerous operations

## Execution Checklist

- [ ] write contract for AlertDialog: title, description, confirm/cancel labels,
  tone (danger/warning), async confirm handler
- [ ] implement AlertDialog primitive in `@flint/svelte-primitives`
- [ ] write contract for FormDialog: extends Dialog with form props, field
  layout, submit/cancel, validation state
- [ ] implement FormDialog composite in `@flint/svelte-composites`
- [ ] write contract for ConfirmAction: trigger element, confirmation popover or
  inline prompt, confirm/cancel, loading state
- [ ] implement ConfirmAction composite in `@flint/svelte-composites`
- [ ] create specimens for all three components
- [ ] register in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] AlertDialog renders with title, message, and tone-colored confirm button
- [ ] AlertDialog traps focus and supports keyboard dismiss
- [ ] FormDialog renders a modal with form fields, validates on submit, and
  reports errors inline
- [ ] ConfirmAction wraps any trigger element and shows confirmation before
  executing the action
- [ ] all three components pass build and render in the preview catalogue

## Next Task

Open `g04.003` and implement file upload, media input, and embed patterns.
