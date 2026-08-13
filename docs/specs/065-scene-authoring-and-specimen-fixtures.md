# 065 Scene Authoring And Specimen Fixtures

Status: retired — superseded by spec 066
Updated: 2026-08-14
Owner: Poodle core

## Record

The first g14 runway kept the Rust scene model as a specimen-only authority.
Its first tranche generated five display specimens in four runtimes and proved
that fixture structure should be authored once.

The implementation also repeated component interface definitions inside the
Rust specimen model and separated specimen fixtures from executable component
cases. Its cost report excluded schema, generator, generated output,
interpreters, tests, and wiring.

The durable requirement moves to
[066 Executable Component Conformance](066-executable-component-conformance.md):
one typed case owns fixture data, actions/assertions, and specimen structure;
four thin preview adapters project it. Existing scene artifacts are
experimental until g14.001 classifies them.

Do not dispatch work from this document.
