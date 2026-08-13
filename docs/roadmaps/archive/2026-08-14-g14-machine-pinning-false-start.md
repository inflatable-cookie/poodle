# g14 Machine-pinning False Start

Status: archived
Archived: 2026-08-14
Superseded by: `../g14/README.md`

## Record

The first g14 runway promoted machine pinning and Rust-authored specimen
scenes too early. Five batches merged before the runway was reset:

| Batch | Result | Disposition |
| --- | --- | --- |
| doctrine (`192c3819`) | specs 064/065 and architecture amendments | superseded by architecture 009 and spec 066 |
| RangeSlider role (`371b0c7e`) | real native accessibility fix | keep |
| baseline (`35392712`) | 18 native registration gaps; gate/specimen inventory | evidence carried into the new estate inventory |
| machine interfaces (`fae9efaf`) | generated hover/menu/modal/popover types | experimental; classify in new g14.001 |
| display specimens (`3ea0cf40`) | five generated four-runtime specimen scenes | experimental; salvage fixture data only if it fits shared cases |

The dispatched differential batch never landed and is withdrawn. Historical
cards and the frozen manifest remain in Git history and August logs; they do
not remain in the active roadmap.

## Why It Stopped

- The machine-interface change made `docs:machine-shape-drift` fail because
  the audit did not follow generated imports.
- `docs:check` did not call that failing gate.
- Machine equality covered four canonical duplicated machines, not component
  interface, composition, renderer, specimen, or backend parity.
- The display-specimen cost claim excluded generator, schema, interpreter,
  generated, test, and wiring code.
- Its Rust model declared component interfaces again to type fixture bindings,
  partially reviving the g13 component IR that had just been retired.
- A proposed `known-divergence` pass state would have made recorded drift
  compatible with completion.

## Durable Evidence

- Keep two implementation substrates: `poodle-core` for web and
  `poodle-render`/`poodle-node` for native.
- Author specimen structure once.
- Use executed evidence, not registration or prose, to prove parity.
- Count the whole mechanism, not only the authored fixture file.
- Missing capability must remain incomplete even when declared.
