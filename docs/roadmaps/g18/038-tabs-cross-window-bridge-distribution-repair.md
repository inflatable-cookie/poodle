# g18.038 — Tabs cross-window bridge distribution repair

Owner: Poodle Svelte Tabs and npm web artifact
Created: 2026-09-14
Governing refs: `../../contracts/components/tabs.md`,
`../../../packages/svelte/components/src/Tabs.svelte`,
`../../../packages/svelte/components/src/tabs-parts/TabsItem.svelte`,
`../../../test/package-install/fixture/DockRegion.test.ts`,
`../../contracts/001-working-rules.md`
Depends on: g18.037 complete; public `@inflatable-cookie/poodle-svelte@0.4.1`
regression confirmed
UI classification: interaction repair — restore the existing public bridge
path with no visual, copy, focus, keyboard, or API change

## Outcome

Svelte Tabs forwards its public `crossWindowSourceBridge` to every eligible
TabsItem again. Source tests and a source-free packed-consumer test prove host
preparation during pointer pre-drag, closing the distribution gap that leaves
Longhorn transfer state idle on published `0.4.1`.

The repair lands on `main` without changing package versions, tags, workflows,
or published artifacts. A separately approved patch lane must publish it as
`0.4.2`; npm package version `0.4.1` is immutable.

## Ready-State Rubric

- [x] The published `0.4.1` bundle reads `crossWindowSourceBridge` inside
  TabsItem but the parent Tabs construction does not pass the prop.
- [x] Current source has the same omission; `v0.3.0` forwarded the prop.
- [x] The public Tabs contract already owns this bridge and its prepare-once
  lifecycle. No new API or consumer migration is required.
- [x] The source component test and installed-package fixture provide bounded
  behavioral seams for source and compiled-artifact proof.
- [x] Longhorn task `g02.039` remains the downstream adoption owner with its
  retained worker, workspace, branch, and edits intact.
- [x] Scope, acceptance, validation, release boundary, and stop conditions are
  fixed.

## Decisions

- Restore `{crossWindowSourceBridge}` on the parent Svelte TabsItem
  construction. Do not alter bridge semantics or add a fallback.
- Prove a pointer pre-drag crossing the existing threshold calls `prepare`
  exactly once with the selected tab source identity.
- Extend the source-free package-install fixture so the compiled archive, not
  repository source resolution, must deliver the same call.
- Keep React unchanged: its forwarding path is present and the defect is in
  the Svelte distribution.
- Preserve pinned tabs, local reorder, keyboard behavior, native drag policy,
  and ordinary Tabs selection.
- Treat `0.4.1` as a defective immutable artifact. The smallest compatible
  correction is `0.4.2`, after separate operator approval.
- Do not edit, replace, cancel, or shim Longhorn task `g02.039`. After `0.4.2`
  registry proof, Longhorn may revise the target version and resume the same
  retained task only under its own operator-authorized Queue action.

## UI Design Brief

Classification: interaction repair. There is no visual change. The existing
cross-window transfer gesture must move from idle into the host-prepared path
once pointer travel crosses the current pre-drag threshold. Local reorder,
pinned-tab constraints, selection, focus, and keyboard behavior stay exactly
as published.

## Dispatch manifest

- **State:** ready and operator-approved for Queue dispatch; no Queue task or
  PR existed when the handoff was compiled.
- **Completion:** source and packed-consumer regressions pass, the existing
  Tabs interaction suite stays green, one frozen source-free web artifact gate
  passes, and an independently reviewed PR merges through Queue closeout.
- **Owned mutable paths:** `packages/svelte/components/src/Tabs.svelte`,
  `packages/svelte/components/test/Tabs.test.ts`,
  `test/package-install/fixture/DockRegion.test.ts`, and
  `docs/contracts/components/tabs.md`.
- **Reserved closeout surfaces:** this task, g18 README, roadmap
  root/index/dispatch, lifecycle state/projections, `PAPERCUTS.md`, and any
  submitted handoff or execution log.
- **Worker:** automatic general pool; the source repair is a single Svelte prop
  projection with existing gesture and packed-install harnesses.
- **Excluded:** React/native/GPUI changes; bridge API or lifecycle redesign;
  Longhorn source, task, workspace, or dependency edits; version bumps, tags,
  workflows, npm publication, or release certification.
- **Escalation:** return to Chatterbox if the packed fixture cannot exercise
  pointer pre-drag without changing public behavior, or if the correction needs
  anything beyond the existing bridge contract.

## Work

1. Add a focused Svelte Tabs regression that mounts a bridge, performs pointer
   pre-drag beyond the existing threshold, and proves `prepare` is called once
   for the selected tab.
2. Forward `{crossWindowSourceBridge}` from Tabs to TabsItem and retain a
   negative no-bridge case plus existing local reorder and pinned-tab proof.
3. Extend the installed-package DockRegion fixture with the same behavioral
   assertion so a source-free packed Svelte archive must forward the bridge.
4. Clarify the Tabs contract's parent-to-item forwarding and packed-artifact
   proof requirement.
5. Run the focused Svelte Tabs test during implementation, then one final
   `effigy release:web-certificate`, the Impeccable detector over the changed
   UI target, and `git diff --check`. Do not stack `ci:web`, `qa`, or another
   release board around the green artifact certificate.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Parent forwards the bridge | Tabs accepts the prop but a generated TabsItem never receives it | focused source pointer pre-drag calls the bridge `prepare` spy exactly once |
| Packed archive preserves behavior | repository-source tests pass while `dist/chunks/Tabs.client.js` drops the prop | source-free installed DockRegion fixture crosses the pointer threshold and observes one prepare call |
| Source identity is correct | host preparation fires for the wrong tab or without current metadata | prepare assertion matches the selected tab's declared source identity |
| Existing interaction stays fixed | the repair changes selection, pinned movement, keyboard reorder, or local drag behavior | focused Tabs suite remains green with existing gesture cases |
| Scope stays compatible | patch adds a fallback/API, edits React/native/Longhorn, or mutates release state | exact diff stays within owned paths and the frozen artifact certificate passes |

## Stop conditions

- Stop if the fix requires a public API change, fallback, core transfer-state
  redesign, or React/native behavior change.
- Stop if another Poodle worker or PR begins owning this forwarding seam.
- Stop after one failed final artifact certificate with the named failing leaf;
  repair and rerun only when the candidate changes.
- Do not change package versions, tags, workflows, or npm state in this task.
- Do not edit or supersede Longhorn task `g02.039`, workspace
  `wks_c6ea0088e9f696b9`, branch
  `ns-750ac957-830d-4224-a289-3770c5c3d582`, or its retained worker.

## Evidence

- Published `@inflatable-cookie/poodle-svelte@0.4.1` declares
  `crossWindowSourceBridge` in TabsItem source registration, while the parent
  TabsItem construction omits the prop.
- Current `Tabs.svelte` accepts the bridge but omits it from `<TabsItem>`;
  `v0.3.0` includes `{crossWindowSourceBridge}` at that call site.
- The omission entered beside the later pinned-value projection; it is not a
  deliberate contract change.
- Longhorn's exact retained-worker proof fails all five transfer assertions
  because host preparation never occurs, while its unrelated boundary proof
  and diff hygiene pass.
- npm serves `0.4.1` publicly and does not permit replacing that version's
  archive. A corrected publication therefore requires `0.4.2`.
- PR #274 merged as `e5a575358be949567eb9bb00d10004b3fad759b2`
  after exact-head review at `c4618cbecdd45529a3308f629fa346538673ccf8`.
  The source suite passed 16/16 and the source-free packed fixture passed
  26/26.
- `v0.4.2` dereferences to
  `d2438aef7d34df31b958d172c9c162e83063d83c`. Hosted candidate run
  `34838863944` passed, and both public registry archives byte-match its
  certified hashes. A fresh registry consumer passed DockRegion 4/4,
  including bridge preparation.

## Next task

Longhorn's retained `g02.039` worker resumed in its original workspace and
branch with a scoped instruction to adopt public `0.4.2`. Poodle has no active
follow-up from this repair; select the next bounded g18 runway item in
Chatterbox.
