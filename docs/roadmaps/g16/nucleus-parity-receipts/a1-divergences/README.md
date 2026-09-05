# A1 divergence records

Executed A1 runs whose paired snapshots did not agree. A record here is not a
receipt and moves no ledger cell; it is the honest output of both extractors
for one shared scenario. The validator ignores this directory; only top-level
`*.json` receipts are evidence.

## Active records

Each active row directory contains `diff.json`, `gpui.json`, `svelte.json`, and
`attributes.json` from the same executed run. The records preserve the
remaining differences rather than silently accepting them.

### g16.114 NP-3 rows

- `agent-chat-input/` — the rendered action has no backend identity, so the
  scenario action cannot be resolved. No renderer projection fix was applied.
- `agent-plan/` — the title text is available, but the native node has no
  contract heading role/level. No projection fix was applied.

### g16.119 focus and state semantics

`AgentQuestion` is repaired: the option group is labelled by the prompt node,
and the A1 replay keeps one stable instance scope so post-action focus lands on
the answered radio. Its empty-diff receipt is
`agentquestion--nucleus-agent-agent-question--a1.json`.

The remaining four owned rows retain honest divergences:

| Store | Remaining attributes | GPUI | Svelte |
| --- | --- | --- | --- |
| `agent-transcript/` | node 0 `focused` | `null` | `true` |
| `menu/` | node 5 `focus_order` | Delete `null` | Delete `3` |
| `radio-group/` | nodes 1–2 `focus_order` | Pro `null`, Free `0` | Pro `0`, Free `1` |
| `segmented-control/` | nodes 1–2 `focus_order` | Grid `null`, List `0` | Grid `0`, List `1` |

The RadioGroup and SegmentedControl state projections are repaired. Their
remaining focus-order difference is the happy-dom extractor counting every
enabled native radio as a sequential stop; a real browser gives the group one
roving stop, which GPUI projects. Teaching the extractor that behavior is
outside this card.

Menu focus entry is repaired: opening moves focus to the first enabled item.
The remaining tab-stop question is unresolved because Svelte `MenuSurface`
renders enabled items as plain buttons while `menu.md` §6 specifies focus entry
and highlighted-item movement but no tab-stop rule. It is returned to
Chatterbox rather than guessed.

AgentTranscript remains unfocused because
`agent-transcript.md` §"Focus And Announcement" says the transcript never takes
focus on append. The Svelte value is happy-dom focusing a clicked `role="log"`
container; making the log a tab stop would contradict the contract.

## NP-1 records

These older records remain outside g16.119's boundary:

- `np1/app-header.*` — Svelte has an implicit `banner` role; `poodle-node` has
  no Banner role.
- `np1/split-view.*` — the native separator has no host-owned current value,
  and its collapse toggle is named `Collapse` rather than Svelte's
  `Collapse primary`.

## Consumed g16.118 overlay stores

The five g16.118 focus-only stores were consumed after the structure projection
and initial-focus repair reached empty diffs. They were replaced by A1 receipts
at the final runtime source pin:

| Overlay | Initial-focus record | Receipt |
| --- | --- | --- |
| Dialog | open dialog panel | `dialog--nucleus-navigation-dialog--a1.json` |
| Popover | open Content surface | `popover--nucleus-navigation-popover--a1.json` |
| ConfirmAction | inherited open dialog panel | `confirmaction--nucleus-settings-confirm-action--a1.json` |
| MessageCenter | open message surface | `messagecenter--nucleus-attention-message-center--a1.json` |
| ModelPicker | selected enabled model row, or first enabled row | `modelpicker--nucleus-agent-model-picker--a1.json` |

`poodle-node` now carries one `initial_focus: bool` accessibility-record
field. `poodle-render` sets it on exactly one node in each open overlay, and
the backend consumes that claim through the existing mount-time focus request.
The claim is one-time per runtime identity, so a later frame cannot steal focus
from user navigation.

Reproduce the owned cohort with:

```sh
POODLE_NUCLEUS_RECEIPT_DIR=$PWD/target/nucleus-receipts \
  cargo test --manifest-path packages/gpui/preview/Cargo.toml \
  --test headless_regressions nucleus_a11y
```
