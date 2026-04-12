# g10.006 GPUI Component Page Usage Docs And Shape Parity

Status: planned
Owner: Poodle core
Depends on: g10.005
Updated: 2026-04-12

## Context

After the shell gap, the biggest direct parity hole is the component page
itself. Svelte renders a cleaner page shape with generated usage docs when docs
exist. GPUI currently stops at specimen, import, and a lighter contract-doc
note.

This milestone moves GPUI component pages toward the same review value as the
Svelte `ComponentPage` without faking docs data that does not exist.

## Governing Refs

- `docs/specs/002-component-contract-template-and-parity-rules.md`
- `docs/specs/020-docs-site-example-and-component-discoverability-rules.md`
- `docs/specs/027-docs-completeness-contract-linting-and-publish-pipeline.md`
- `docs/specs/058-cross-runtime-parity-report-delta-register-and-acceptance-harness-expansion.md`
- `docs/contracts/components/`

## Goals

- build a real GPUI usage-doc surface from shared component contract docs
- align GPUI component page structure closer to the Svelte shape:
  hero, specimen, import, usage docs when data exists
- render props, states, slots, events, examples, and doc status where shared
  contract data supports them
- keep missing docs explicit instead of inventing generated output

## Non-Goals

- rewriting component contracts for design changes
- long-tail specimen upgrades across the registry
- broad preview-shell restructuring beyond what `g10.005` leaves behind

## Execution Plan

### Batch 6.1 - Contract Doc Extraction

- [ ] define one GPUI-side parser or mapper from shared contract docs into a
      usage-doc view model
- [ ] extract the sections that are consistently present enough to review:
      summary, props, states, events, anatomy, examples, and status metadata
- [ ] keep unsupported or absent fields explicit in the model rather than
      fabricating output

### Batch 6.2 - Component Page Integration

- [ ] replace the current contract-doc card with a usage-doc surface that sits
      where Svelte shows usage docs
- [ ] keep import guidance compact and subordinate to specimen plus docs
- [ ] ensure pages with no shared docs do not render a misleading full-doc shell

### Batch 6.3 - Review And Validation

- [ ] run a spot review across representative components from multiple tags
- [ ] correct any doc-shape mismatch between GPUI rendering and current
      contract structure
- [ ] validate with `cargo check --manifest-path packages/gpui/preview/Cargo.toml`
      and `git diff --check`

## Exit Criteria

- GPUI component pages surface real shared usage-doc data where docs exist
- page structure is closer to Svelte and less GPUI-specific
- missing docs remain visible as real gaps, not hidden or fabricated

## Next Task

After `g10.005` closes, execute Batch 6.1 in `g10.006`: map shared contract
docs into a GPUI usage-doc view model.
