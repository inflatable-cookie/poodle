# TextInput Native Event Cleanup

Status: open — bounded follow-up decision after `g16.007`
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

## Decision Needed

Inspect every consumer of the shared key-submit mapping and input-text state
keys. Decide whether these are one small generic backend repair or need
separate cards. Do not patch TextInput alone or change CodeInput/DurationInput
behaviour by implication.

## Guardrail

Keep this separate from NumberInput's unresolved value model, TextInput
multiline/slug closure, accessibility promotion, visual comparison, and
Jetstream admission.
