# Severe Duplicate Audit

Poodle remains `strict-ready`. The release sweep found no contract or roadmap
gap, but it did find one shared generator and one preview catalogue seam that
need tighter ownership.

## Changed

- Moved parity-report validation and assembly into
  `packages/svelte/preview/scripts/parity-report.ts`.
- Kept the Svelte and React entry scripts as thin artifact-specific wrappers.
- Preserved both generated reports byte-for-byte.

## Classified

The initial scan reported 109 duplicate blocks: 2 critical, 11 high, and 96
warning. The shared generator extraction removed one high finding.

One remaining critical finding is actionable:

- `packages/react/preview/src/gallery/registry.ts` duplicates the common
  catalogue metadata in `packages/svelte/preview/src/component-registry.ts`.
  Its claim that the registries have the same slugs and order is stale. React
  exports `Stepper`, `AgentQuestion`, and `AgentQuestionRecord` without listing
  them in its gallery; `AgentPlan` and `AgentPlanRecord` remain Svelte-only.
  This needs a bounded catalogue and specimen audit, not blind deduplication.

The other 11 severe findings are intentional independent evidence or runtime
declarations:

- GPUI and Jetstream adapter manifests declare their own supported surfaces;
  the lists overlap but already contain real runtime differences.
- GPUI and Jetstream specimens repeat contract-shaped fixtures so each backend
  can be reviewed and run independently.
- React and Svelte `ModelPicker` tests repeat the same behavioral cases across
  different framework implementations.
- Icon names recur across support declarations, drift checks, and specimen
  galleries for different purposes; one shared list would erase those
  independent checks.

## Current State

- duplicate blocks: 108
- critical: 2
- high: 10
- warning: 96
- next actionable seam: web preview catalogue ownership and React coverage

## Validated

- `effigy parity:report`
- generated React report SHA-256:
  `f3413886f5d94db6ace1787af282eeff7618c64be1131b0dd9b91762c977f4fd`
- generated Svelte report SHA-256:
  `8a518ff0db670efb61ca5709def449300c7085d1fe4727eb02cf4a3a19810499`
- `effigy scan duplicate-blocks`
