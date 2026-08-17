# g15.010 — Display, Workstation & Agent GPUI Specimens (August batch log)

Date: 2026-08-17
Card: `docs/roadmaps/g15/010-display-workstation-agent-gpui-specimens.md`
Handoff: `docs/handoffs/20260817-184625-g15-010-gpui-specimen-closure.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-ad7fb2e9`
Branch: `t3code/gpui-specimen-closure`

## Summary

Named GPUI catalogue pages for the 18 components that already had Rust specs
and `poodle-render` implementations but fell through to `missing_specimen`.
Pages compose `poodle-render` through the node backend. Interactive examples
keep state in the preview host and rebuild the next spec. Jetstream is
program-deferred.

## Pages

### Batch A — display and status

| Component | File | Live host proof |
| --- | --- | --- |
| Avatar | `packages/gpui/preview/src/specimens/avatar.rs` | static |
| Callout | `.../callout.rs` | dismiss via `poodle-callout-dismiss` |
| RemediationBanner | `.../remediation_banner.rs` | action `retry` then dismiss |
| MetaItem | `.../meta_item.rs` | static (split out of MetaBar) |
| Pill | `.../pill.rs` | static; no remove example |
| Spinner | `.../spinner.rs` | static |
| EmptyState | `.../empty_state.rs` | static |
| StateTile | `.../state_tile.rs` | static; sparkline slot empty |

### Batch B — workstation

| Component | File | Live host proof |
| --- | --- | --- |
| ActionDiscoveryPanel | `.../action_discovery_panel.rs` | selection of `open-file` |
| DockRegion | `.../dock_region.rs` | tab `search` then `dock-collapse` |

Replaced the previous `action_discovery.rs` / `dock_split.rs` catalogue
routes. SplitView stays on its own page. Dock drag/reorder/panel-drop is a
documented native vocabulary gap.

### Batch C — agent surfaces

| Component | File | Live host proof |
| --- | --- | --- |
| AgentMessage | `.../agent_message.rs` | static display projection |
| AgentPlan | `.../agent_plan.rs` | accept / revise / dismiss |
| AgentPlanRecord | `.../agent_plan_record.rs` | disclosure |
| AgentQuestionRecord | `.../agent_question_record.rs` | static display projection |
| AgentSubagent | `.../agent_subagent.rs` | disclosure |
| ChangedFiles | `.../changed_files.rs` | group disclosure then file select |
| ToolCall | `.../tool_call.rs` | output disclosure |
| ToolCallGroup | `.../tool_call_group.rs` | run disclosure |

## Intentional native differences

- **Avatar:** native spec has no density; image uses `NodeKind::Image`.
- **Callout:** web `message` / `content` both map to `CallOutSpec.content`.
- **Pill:** removable chips are contract out of scope; no remove example.
- **StateTile sparkline:** reserved empty slot; the host owns the chart.
- **ActionDiscoveryPanel:** loading is shown as a representative posture; the
  Svelte teaching page does not put loading in Examples.
- **DockRegion:** no native drag, reorder, or panel-drop. Tabs and collapse
  use scoped `runtime_id` so several docks can share tab values.
- **AgentMessage / AgentQuestionRecord:** display projections. No web-only
  link click or answer ownership.
- **AgentPlan:** accept/revise/dismiss keep semantic ids; hosts that mount
  more than one pending plan pass `instance_id` so backend focus stays
  distinct.

## Renderer a11y for mounted input

`tracks_focus` needs `focusable` **and** `style.focus`. Named semantic ids
plus a focus ring were added on Callout/RemediationBanner dismiss,
ActionDiscovery rows, DockRegion tabs/collapse, AgentPlan actions,
AgentPlanRecord and AgentSubagent toggles, ChangedFiles header/file/chip,
ToolCall rows with output, and ToolCallGroup run toggle.

Backend focus is keyed by `runtime_id` then `id`. Those newly tracked
controls now take a host `instance_id` and set a scoped `runtime_id`,
keeping the semantic `id` readable. Identity never includes mutable
presentation state: AgentPlanRecord's toggle is `agent-plan-record-toggle`
whether the record is open or shut. Two records with the same status and
no `decided_at` stay distinct when the host supplies instance ids.

## Public-intent Rust API

- Package: `poodle-render`.
- Classification: operator-approved breaking pre-v0.2 source cleanup.
- `callout`, `action_discovery_panel`, and `tool_call` now consistently take
  typed handler structs. `CalloutHandlers`, `ActionDiscoveryPanelHandlers`,
  and `ToolCallHandlers` are the canonical callback and instance-scope inputs;
  no compatibility entry point remains.
- `RemediationBannerHandlers`, `DockRegionHandlers`, `AgentPlanHandlers`,
  `AgentPlanRecordHandlers`, `AgentSubagentHandlers`, `ChangedFilesHandlers`,
  and `ToolCallGroupHandlers` gain `instance_id` for stable backend focus
  scope. The new semantic focus-id helpers and constants are additive.
- A read-only scan across `/Users/tom/Dev/projects` found no direct external
  Rust call sites. Downstream native consumers must still recompile and should
  supply stable `instance_id` values wherever repeated interactive instances
  can share semantic ids.

## Validation run

- focused `poodle-gpui-preview` headless cases for Batches A–C
- `effigy check:gpui`
- `effigy regressions:native` — 40 passed
- `effigy docs:check`
- `effigy qa` (headless)
- `git diff --check origin/main...HEAD`

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector was run.
