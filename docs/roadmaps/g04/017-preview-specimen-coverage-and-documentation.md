# g04.017 Preview Specimen Coverage And Documentation

Status: planned
Owner: Poodle Core
Updated: 2026-03-14
Depends on: g04.002 through g04.016
Primary repos: `poodle`

## Goals

- [ ] ensure every new g04 component has a complete specimen in the preview
  catalogue
- [ ] update component-registry.ts with all new component entries
- [ ] update the specimen registry with all new specimen mappings
- [ ] verify the preview catalogue renders all new components correctly

## Execution Checklist

- [ ] audit component-registry.ts against the g04 gap register for completeness
- [ ] audit specimen registry against component-registry.ts for completeness
- [ ] verify every new component page renders in the preview app
- [ ] verify sidebar navigation includes all new components in correct sections
- [ ] verify search/filtering works with new component names
- [ ] review specimen quality: each should demonstrate key variants, states, and
  interactions
- [ ] update any specimens that are placeholder-quality to full demo quality
- [ ] run full build verification

## Acceptance Criteria

- [ ] every g04 component has a component-registry entry with accurate metadata
- [ ] every g04 component has a specimen in the specimen registry
- [ ] every specimen renders correctly in the preview catalogue
- [ ] no build errors or console warnings from new components
- [ ] component count in preview header reflects the updated total

## Next Task

Open `g04.018` and close the generation.
