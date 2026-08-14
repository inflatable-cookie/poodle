# g14 Specimen Ownership Correction

Date: 2026-08-14
Status: complete

## Change

The Button, RangeSlider, and Tabs pilots projected exhaustive conformance cases
into the default catalogue `Examples` views. Size and density matrices were
duplicated despite dedicated tabs, and the pages stopped teaching component
use.

Commit `8ac863b4` restored the last curated Svelte, React, and GPUI specimens.
The executable cases, adapters, observations, and headless conformance gates
remain intact.

## Corrected Ownership

- conformance cases own exhaustive fixtures, actions, assertions, and
  diagnostic projection metadata
- catalogue specimens own human-facing examples and use guidance
- `Sizes` and `Densities` own their axes
- a later `Conformance` tab may expose every case after the pilot verdict
- shared catalogue structure must remain metadata, not another scene tree

## Planning Effect

Architecture 009, spec 066, the g14 index, pending profile cards, and the
conformance estate now use the corrected boundary. `g14.026` schedules a full
catalogue audit after the pilot verdict; it will pilot the curation contract
and compile bounded rollout tranches.

## Evidence

- `effigy test:components` — 1,332 passing
- `effigy test:parity` — 174 passing
- `effigy ci:conformance` — headless board passing
- Svelte and React live previews show curated Button, RangeSlider, and Tabs
  groups without corpus captions
