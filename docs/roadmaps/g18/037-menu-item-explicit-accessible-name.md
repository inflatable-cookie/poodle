# g18.037 — Menu item explicit accessible names

Owner: Poodle web MenuSurface adapters
Created: 2026-09-14
Governing refs: `../../contracts/components/menu.md`,
`../../contracts/components/context-menu.md`,
`../../../packages/svelte/components/src/MenuSurface.svelte`,
`../../../packages/react/components/src/MenuSurface.tsx`
Depends on: g18.036 complete
UI classification: refinement — expose the existing required item label as an
explicit accessible name; no visual, copy, focus, keyboard, or workflow change

## Outcome

Every non-separator item rendered by the shared web MenuSurface exposes its
required `item.label` as an exact explicit accessible name. Menu and ContextMenu
inherit the rule in Svelte and React without a new prop or consumer adapter.

## Ready-State Rubric

- [x] Both web MenuSurface implementations render visible `item.label` text and
  the menuitem role family without an item `aria-label`.
- [x] `MenuItem.label` is required and already owns visible text, navigation
  identity, and the corresponding shared-Rust accessibility label.
- [x] Menu and ContextMenu reuse MenuSurface; no consumer-specific surface is
  needed.
- [x] Browser-computed naming already resolves from visible text, so the change
  preserves ordinary assistive-technology behavior while making the semantic
  label explicit for bounded serializers.
- [x] g18.036 is merged and its hook-owned closeout is complete; no Poodle Queue
  task or PR owns these paths.
- [x] Scope, evidence, validation, and stop conditions are fixed.

## Decisions

- Set the item button's `aria-label` to exactly `item.label` in both web
  MenuSurface adapters.
- Apply the rule to action, checkbox, radio, disabled, and Svelte submenu-parent
  rows. Separators remain unnamed.
- Keep visible labels, shortcut/check/submenu metadata, roles, checked state,
  focus movement, activation, dismissal, and submenu behavior unchanged.
- Add no public naming override. Do not broaden this lane to Menubar,
  SplitButton, ListCard, or a general ARIA-role audit.
- Shared Rust Menu already assigns the entry label to accessibility metadata;
  no native or GPUI implementation change is required.

## UI Design Brief

Classification: accessibility refinement. The rendered interface is visually
identical. Each non-separator row announces the same text users see, now from
an explicit attribute whose value is the existing required label. Metadata
continues to be supplemental and hidden from the accessible name. All input,
focus, checked-state, dismissal, and submenu interactions remain unchanged.

## Dispatch manifest

- **State:** ready; sole active Poodle product task; follows completed g18.036.
- **Completion:** exact-label regressions pass across both Menu and ContextMenu
  wrappers, existing interactions remain green, one bounded web board passes,
  and an independently reviewed PR merges through Queue closeout.
- **Owned mutable paths:**
  `packages/svelte/components/src/MenuSurface.svelte`,
  `packages/react/components/src/MenuSurface.tsx`, paired Svelte/React Menu and
  ContextMenu tests, and the Menu/ContextMenu component contracts.
- **Reserved closeout surfaces:** this task and submitted handoff, g18 README,
  roadmap root/index/dispatch, lifecycle state/projections, `PAPERCUTS.md`, and
  any execution log.
- **Worker:** automatic general pool; this is a paired attribute projection with
  settled semantics and focused test seams.
- **Excluded:** Longhorn, Figmatic, their serializers or dependency pins;
  Menubar, SplitButton, ListCard, other role-bearing components; public APIs;
  core DOM machinery; native/GPUI behavior; workflows; release preparation or
  publication.
- **Escalation:** return to Chatterbox if an exact `item.label` cannot name a
  current MenuSurface row without changing public API or visible behavior.

## Work

1. Add focused regressions covering action, checkbox, radio, and disabled rows
   in both Menu wrappers. Assert each non-separator item's `aria-label` equals
   its visible required label and separators have no accessible-name attribute.
2. Cover Svelte's submenu-parent row and prove the shared surface rule through
   both framework ContextMenu wrappers.
3. Add `aria-label={item.label}` to each web MenuSurface item button and update
   the Menu and ContextMenu accessibility contracts.
4. Re-run the paired focused Menu/ContextMenu tests, then one final
   `effigy ci:web`, the Impeccable detector over the changed UI targets, and
   `git diff --check`. Do not stack `test:components`, `docs:check`, or `qa`
   around the green web board.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Exact explicit names | a menuitem relies only on descendant text or derives a name containing shortcut/check metadata | action, checkbox, and radio rows each expose `aria-label === item.label` in Svelte and React |
| Disabled rows stay named | the native disabled state drops the row's explicit semantic label | disabled-row regression retains the exact label and existing disabled behavior |
| Separators stay structural | the blanket render loop assigns a label to `role=separator` | separator regression finds no `aria-label` |
| Submenu parents follow the rule | Svelte's wrapped submenu branch misses the shared item attribute | submenu-parent regression proves its exact label while `aria-haspopup` and expansion behavior remain intact |
| Both public wrappers inherit | Menu passes while ContextMenu renders through a divergent path | paired Menu and ContextMenu tests pass in both frameworks |
| Behavior and scope stay fixed | the patch adds a naming prop, changes visible copy/interactions, or edits a consumer/native surface | exact diff is confined to the two adapters, paired tests, and two contracts; existing focused interactions and `ci:web` pass |

## Stop conditions

- Stop if the fix needs a new public prop, changes visible text or interaction
  behavior, or exposes a broader accessible-name policy decision.
- Stop if another Poodle worker or PR begins owning MenuSurface naming.
- Stop after one failed final web board with the named failing leaf; repair and
  rerun only when the candidate changes.
- Do not edit Longhorn or Figmatic, change dependency pins, or prepare/publish
  an npm patch from this task.

## Evidence

- Both web MenuSurface item buttons set role and state but omit `aria-label`;
  their label spans render the required `item.label`.
- Menu glyphs, shortcuts, checks, and submenu indicators are already
  `aria-hidden`, so the proposed explicit value matches the current computed
  browser name rather than replacing it with new copy.
- Shared Rust Menu already assigns `entry.label` to `item.a11y.label`, making
  this a web representation repair.
- Poodle Tree items use the same exact-label projection, established before
  both `v0.3.0` and `v0.4.1`.
- Longhorn's serializer omits menuitem/treeitem text fallback and currently
  pins Poodle `0.3.0`; Longhorn adoption therefore waits for a later exact
  published Poodle patch and stays outside this lane.

## Next task

Return to Chatterbox after merge. If no further sweep findings are accepted,
g18.036 and g18.037 can enter one separately operator-approved npm patch lane.
