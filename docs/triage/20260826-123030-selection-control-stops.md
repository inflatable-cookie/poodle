# Selection-Control Stops After g16.002

Status: open — RadioGroup landed in PR #77; ToggleGroup is ready for contract
promotion and separate card compilation
Captured: 2026-08-26
Source: `g16.002` partial close and PR #76 review

## RadioGroup

The observable contract is coherent: one selected option, same-value inertia,
native-style roving focus, orientation-aware arrow movement, wrapping, and
disabled-option skipping. The stop is lifecycle identity, not selection
semantics.

Svelte and React generate a stable group identity per mounted instance when
`name` is absent. Shared Rust rendering is stateless. It cannot generate a new
identity during each rebuild without losing focus, and option-value-only ids
collide when two groups contain the same values.

Recommended boundary: keep web `name` and semantic component props unchanged.
Give the shared native renderer a host-supplied stable interaction scope,
separate from form naming and visual data. Require it for mounted interactive
use; do not silently fall back to option values or render order. This follows
the existing native handler-scope pattern and keeps runtime-owned focus
mechanism out of the renderer-neutral behavior contract.

Decision: carry the required scope in a RadioGroup handler bundle. Do not widen
the generic render context in this lane. The operator accepted this boundary on
2026-08-26; it is promoted into the RadioGroup contract and `g16.003`.

## ToggleGroup

The current authority contradicts itself:

- the callback contract and both web runtimes emit the resulting selection:
  `string | string[] | null`;
- shared Rust owns the same transition machine but the renderer callback emits
  only the activated option as `Fn(&str)`;
- the Jetstream contract note calls activated-option payload intentional, which
  contradicts the active-cohort contract and predates Jetstream deferral;
- the keyboard table requires single-mode roving focus, while the behavior
  section says there is no roving machinery and Svelte/React currently leave
  every item tabbable;
- option-value-only native ids collide across mounted instances.

Recommended semantic direction:

1. keep resulting-selection payloads as the public rule;
2. make the native callback carry a typed single/multiple selection result and
   derive it through the shared Rust transition, not in tests or hosts;
3. keep single mode as a radiogroup and implement roving focus in the shared
   web substrate, both web shells, and shared Rust;
4. use the same required host-supplied native interaction scope as RadioGroup;
5. correct the stale Jetstream note while leaving the deferred backend out of
   execution scope.

The alternative is to demote single ToggleGroup to ordinary pressed buttons
with no roving focus. That is simpler but changes its radiogroup semantics and
would make it less distinct from a row of toggle Buttons. It should happen only
as an explicit product decision.

Decision: the operator accepted the recommendation on 2026-08-26. Promotion
and execution remain sequenced after the RadioGroup identity pattern lands.

## Sequencing

Do not dispatch both lanes in parallel. They overlap the component contracts,
native interaction identity, the GPUI mounted regression file, parity-ledger
generator, and generation front doors.

After the accepted decisions:

1. land the RadioGroup identity/mounted-parity card — complete in PR #77;
2. promote the accepted ToggleGroup decision into its contract;
3. compile a separate ToggleGroup semantic/API/mounted-parity card using the
   landed identity pattern.
