# Native Composer Attention Regions

Poodle remains `strict-ready`. The native composer now owns the same question
and plan composition boundary as the web implementations.

## Changed

- Added question and plan child vectors to the shared `poodle-render`
  `agent_chat_input` function.
- Made composer status select the active region. Question content renders only
  while questioning; plan content renders only while reviewing a plan.
- Kept both runtime adapters thin: GPUI and Jetstream pass host-rendered nodes
  into the shared renderer.
- Added questioning and plan-review specimens to both native previews.
- Added a renderer test proving the selected region stays inside the composer
  field and inactive regions do not leak into other states.
- Removed the open native composition delta from the component contract.

## Validated

- `cargo test --manifest-path packages/render/Cargo.toml agent_chat_input`
- `effigy gpui:build`
- `effigy jetstream:build`
- `effigy docs:check`
- `effigy scan duplicate-blocks`
- `git diff --check`

`effigy ci:native` reached the role census after its Poodle checks and tests,
then stopped because the concurrently modified sibling Jetstream renderer did
not compile. Both Poodle native preview builds pass against the same checkout;
the external renderer worktree remains outside this batch.
