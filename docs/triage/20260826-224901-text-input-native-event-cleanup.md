# TextInput Native Event Cleanup

Status: promoted — resolved into ready card `g16.008`
Captured: 2026-08-26
Source: PR #81 review and `g16.007` execution log

## Findings

`g16.007` proved core controlled TextInput editing and fixed the defects inside
that envelope. It also exposed two adjacent GPUI backend issues that were not
safe to repair without checking their wider consumers:

- `packages/gpui/node-backend/src/interaction.rs` maps `tab` to submit beside
  `enter`, while the TextInput contract assigns Tab to focus traversal.
  CodeInput and DurationInput may currently rely on the shared mapping.
- blur calls `input_text::forget` with the field-root id, while measured text,
  scroll, blink, and marked-text state use the value-node id. The documented
  blur-time reset therefore misses the state it intends to clear.

## Disposition

The post-`g16.007` checkpoint confirmed one bounded generic repair. TextInput
and DurationInput require Tab traversal; CodeInput has no submit contract; and
EditableLabel's Tab commit belongs to its documented blur path. Composite and
childless inputs also need one backend-owned painted-state-key rule. Execute
`../roadmaps/g16/008-native-text-event-routing-cleanup.md`.

## Guardrail

Keep this separate from NumberInput's unresolved value model, TextInput
multiline/slug closure, accessibility promotion, visual comparison, and
Jetstream admission.
