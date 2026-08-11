# g13.011 Controls, Forms, And Audio Component Migration

Status: gated on g13.008 adopt verdict
Owner: Poodle core
Depends on: `g13.009`, `g13.010`

## Objective

Migrate machine-backed controls and audio components onto shared definitions
while preserving adapter-owned input, accessibility, and VisualState seams.

## Scope

- Selection, scalar, text/number, form, validation, date/time, and audio
  component families.
- Generate conformance vectors and capability declarations with definitions.
- Keep realtime meter feeds, text systems, pointer capture, and host effects
  in their existing runtime-owned layers.

## Acceptance

- Machine laws and public events retain golden-value coverage.
- Drawing reads serializable state only.
- Size, density, orientation, unipolar/bipolar, and accessibility axes remain
  complete across all four runtimes.

## Next

`g13.012` handles focus-, portal-, and navigation-heavy families.
