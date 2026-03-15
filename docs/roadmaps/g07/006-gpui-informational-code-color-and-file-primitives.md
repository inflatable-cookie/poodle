# g07.006 — GPUI Informational, Code, Color, and File Primitives

Status: Completed
Updated: 2026-03-14

## Objective

Implement RenderComponent for informational, code, color, file, and remaining
date/time primitives.

## Components (16)

CodeSpec, ColorPickerSpec, FileUploadSpec, EyebrowSpec, PillSpec, TimeAgoSpec,
DurationInputSpec, TimeZoneSelectSpec, ZonedDateTimePickerSpec, SplitButtonSpec,
CalendarSpec, RangeCalendarSpec, DatePickerSpec, DateRangePickerSpec,
DateTimePickerSpec, DateTimeRangePickerSpec

## Implementation

New module `render_informational.rs` with 16 `RenderComponent<Spec>` implementations.

## Tests

16 new tests (83 total).

## Primitive Coverage

All 64 primitive specs now have RenderComponent implementations across 5 render modules:
- render_structural.rs: 8 components (g07.002)
- render_action.rs: 12 components (g07.003)
- render_selection.rs: 14 components (g07.004)
- render_overlay.rs: 13 components (g07.005)
- render_informational.rs: 16 components (g07.006)
- SplitButton is included here as it combines action+overlay behavior
- Total: 63 unique specs + SplitButton = 64 primitive RenderComponent impls

## Verification

- [x] All 64 primitive specs have RenderComponent implementations
- [x] AdapterManifest lists all 64 supported primitive component names
- [x] 83 tests passing across all modules
