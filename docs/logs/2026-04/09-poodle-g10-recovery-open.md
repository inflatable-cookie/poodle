# 2026-04-09 - Poodle g10 Recovery Open

Roadmap: `g10.002`

## Summary

Recovered Poodle's roadmap front doors so they match the real active queue.

`g09` had remained documented as active even though every on-disk `g09`
milestone was already complete and `g10.001` already existed. At the same
time, the live thread work had drifted into Jetstream and component-overhaul
activity that no longer mapped cleanly to the public roadmap entry points.

This batch closes `g09` as lineage, marks `g10` as the active generation, and
opens `g10.002` as a recovery/control lane so the real queue can be frozen from
evidence before more freeform work continues.

## Changes

- marked `g10` active in roadmap front doors
- marked `g09` complete as a closed generation README
- opened `g10.002` as the live recovery/control lane
- aligned the top-level roadmap index to the actual active generation

## Validation

- pending

## Next Task

Execute `g10.002` Batch 2.2: freeze the real active queue from current
evidence, then compile the first bounded `g10` milestone.
