# React Preview Catalogue Repair

Poodle remains `strict-ready`. The duplicated React catalogue is gone, its
missing public exports are visible, and the composer now hosts question and
plan-review regions on both web runtimes.

## Changed

- Made the Svelte component registry canonical for shared web-preview names,
  tags, descriptions, ordering, and slugs.
- Kept `AgentPlan` and `AgentPlanRecord` explicitly Svelte-only.
- Kept React's four transcript-internal components explicitly embedded-only.
- Added React specimens for `Stepper`, `AgentQuestion`, and
  `AgentQuestionRecord`.
- Added a runtime assertion that rejects missing or unknown React specimen
  mappings during the preview build.
- Added React `AgentChatInput` question and plan regions, state-specific
  placeholders, and question-aware submit gating to match the component
  contract and Svelte implementation.
- Repaired the contract's public prop, slot, and `AgentChatStatus` tables.
- Extended the shared native spec with `questioning` and `reviewing-plan`
  states, state-specific placeholders, and question-aware submit gating.

## Measured

- React catalogue: 144 entries
- Svelte catalogue: 146 entries
- intentional difference: `AgentPlan` and `AgentPlanRecord`
- duplicate blocks: 105, down from 108
- critical: 1, down from 2
- high: 10
- warning: 94, down from 96

The remaining critical and high findings are the independent native adapter,
specimen, icon-inventory, and cross-framework test declarations classified in
`10-severe-duplicate-audit.md`.

Native renderers still compose question and plan surfaces outside the composer;
the contract now records the missing child-vector seam as an open delta.

## Validated

- `effigy test:components`
- `effigy react:build`
- `effigy docs:check`
- `effigy ci:rust`
- `effigy ci:native`
- `effigy scan duplicate-blocks`
- `git diff --check`
