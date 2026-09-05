# g16.120 — A1 Landmarks And Content Roles

Status: held — ready when `g16.118` merges (needs `Heading` and `Banner`)
Type: native projection repair — `poodle-render`; A1 receipts for four rows
Opened: 2026-09-05
Depends on: merged `g16.118`
Governing refs: `../../contracts/components/{app-header,split-view,agent-chat-input,agent-plan}.md`,
`nucleus-parity-receipts/a1-divergences/{np1,agent-chat-input,agent-plan}/`
Dispatch manifest: `../dispatch.md`

## Rows and recorded causes

| Row | GPUI | Svelte (reference) |
| --- | --- | --- |
| AppHeader | role-less root | `banner` landmark |
| SplitView | separator has no current `value`; toggle named "Collapse" | separator `value` 0; toggle "Collapse primary" |
| AgentChatInput | textbox and send button have no roles (identity gap from NP-3) | `textbox` and `button` |
| AgentPlan | no title heading; buttons in a different order and names shifted | heading, then "Release plan", "Accept plan", "Revise" in order |

## Fixed Boundary

- Project `Banner` on AppHeader's root and `Heading` with `level` on
  AgentPlan's title (roles land in `g16.118`); give AgentChatInput's editor
  and action their `TextInput`/`Button` roles and backend identity; give
  SplitView's separator its current value and the toggle its full name.
- Svelte unchanged unless the contract contradicts it.
- Proof: empty-diff A1 receipts for the four rows; divergence stores deleted;
  cohort repin and re-emit; ledger regenerated.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Roles come from the vocabulary | a string role | compile error |
| Order matches | buttons right, order wrong | positional diff non-empty |

## Validation

`effigy regressions:native`, `cargo test -p poodle-render`,
`effigy check:parity-evidence-ledger`, `effigy docs:check`, `git diff --check
origin/main...HEAD`.

## Owned Paths

`packages/render/src/{app_header,split_view,agent_chat_input,agent_plan}.rs`
and tests, backend identity for the chat-input action, the rows' A1 tests,
scenarios, receipts and divergence stores, manifest `resolution`, ledger,
execution log, `PAPERCUTS.md` (append).

## Stop Conditions

Stop when a row needs a vocabulary addition beyond `118`'s two roles.
Escalation owner: Chatterbox.
