# g16.120 — A1 Landmarks And Content Roles

Status: implementation complete — pending fresh exact-head review
Type: native projection repair — `poodle-render`; A1 receipts for four rows
Opened: 2026-09-05
Depends on: merged `g16.118`
Base: `origin/main` at `cceb6646a2bf7776b670fb63f586bce037d0ee6e`
Runtime/evidence source pin: `54646ba2369959150a1b4953e06de5871b3ffe8f`
Lock digest: `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`
Governing refs: `../../contracts/components/{app-header,split-view,agent-chat-input,agent-plan}.md`,
`nucleus-parity-receipts/a1-divergences/README.md`
Dispatch manifest: `../dispatch.md`

## Rows and recorded causes

| Row | GPUI | Svelte (reference) |
| --- | --- | --- |
| AppHeader | role-less root | `banner` landmark |
| SplitView | separator has no current `value`; toggle named "Collapse" | separator `value` 0; toggle "Collapse primary" |
| AgentChatInput | textbox and send button have no roles (identity gap from NP-3) | `textbox` and `button` |
| AgentPlan | no title heading; buttons in a different order and names shifted | heading, then "Release plan", "Accept plan", "Revise" in order |

## Fixed Boundary

- Use the `g16.118` vocabulary to project `Banner` on AppHeader's root and
  `Heading` with `level` on AgentPlan's title; give AgentChatInput's editor
  and action their `TextInput`/`Button` roles and backend identity; give
  SplitView's separator its current value and the toggle its full name.
- Svelte unchanged unless the contract contradicts it.
- Proof: empty-diff A1 receipts for the four rows; divergence stores deleted;
  cohort repin and re-emit; ledger regenerated.

## Outcome

All four rows now match their paired Svelte accessibility snapshots at the
implementation source pin above. AppHeader projects `Banner`; SplitView
projects the separator's current value and full collapse-toggle name;
AgentChatInput projects `TextInput` and `Button` roles with stable action
identity; AgentPlan projects its first markdown heading as a levelled `Heading`.
The complete cohort was re-emitted at the same exact head. The four target A1
receipts are empty-diff receipts, and their superseded divergence stores were
deleted. The older active divergence stores remain unchanged.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Roles come from the vocabulary | a string role | compile error |
| Order matches | buttons right, order wrong | positional diff non-empty |

## Validation

`effigy regressions:native`, `effigy test:nucleus-a11y`,
`effigy test:nucleus-parity-receipts`, `effigy test:parity-evidence-ledger`,
`effigy check:parity-evidence-ledger`, `effigy docs:check`, `effigy ci:web`,
`effigy ci:rust`, and `git diff --check origin/main...HEAD`.

Full `poodle-render` tests were run through
`cargo test --manifest-path packages/render/Cargo.toml`: 642 passed and the
two known origin-main failures recorded in `PAPERCUTS.md` remain outside this
card's ownership.

## Owned Paths

`packages/render/src/{app_header,split_view,agent_chat_input,agent_plan}.rs`
and tests, backend identity for the chat-input action, the rows' A1 tests,
scenarios, receipts and divergence stores, manifest `resolution`, ledger,
execution log, `PAPERCUTS.md` (append).

## Stop Conditions

Stop when a row needs a vocabulary addition beyond `118`'s two roles.
Escalation owner: Chatterbox.
