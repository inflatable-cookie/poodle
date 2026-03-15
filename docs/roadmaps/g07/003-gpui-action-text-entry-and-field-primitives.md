# g07.003 — GPUI Action, Text-Entry, and Field Primitives

Status: Completed
Updated: 2026-03-14

## Objective

Implement RenderComponent for action, text-entry, and field primitives.

## Components (12)

ButtonSpec, IconButtonSpec, FieldSpec, TextInputSpec, TextAreaSpec, SearchFieldSpec,
FormActionsSpec, TimeFieldSpec, EditableLabelSpec, NumberEntrySpec, PinInputSpec, ToolbarSpec

## Implementation

New module `render_action.rs` with 12 `RenderComponent<Spec>` implementations. Button and
TextInput resolve fill, border, radius, and disabled opacity tokens. Other components resolve
their spec-specific tokens.

## Tests

12 new tests (40 total).

## Verification

- [x] All 12 action/field primitives have RenderComponent implementations
- [x] AdapterManifest updated with 12 new supported component names
- [x] 40 tests passing
