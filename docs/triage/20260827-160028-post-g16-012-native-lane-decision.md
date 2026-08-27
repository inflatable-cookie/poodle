# Post-g16.012 Native Lane Decision

Status: resolved — implemented in g16.013 on `t3code/tri-state-switch`
Captured: 2026-08-27
Resolved: 2026-08-27
Source: orchestrator evidence checkpoint after PR #86

## Finding

The ledger is at 41 mounted / 133 missing. TriStateSwitch is the strongest
next bounded foundation lane, but correct closure requires a deliberate
pre-1.0 Rust contract break rather than a renderer-only patch.

### TriStateSwitch — recommended

The web contract and both TypeScript runtimes agree on one semantic surface:

- value is `excluded | default | included`;
- the default value is `default`;
- the fixed order is excluded, default, included;
- selecting the current value is inert through `singleSelectTransition`;
- one checked radio is the tab stop and arrow keys move selection and focus;
- disabled controls are inert and absent from focus traversal.

The shared Rust path does not yet preserve that contract:

- `TriStateSwitchSpec` stores legacy `CheckState` and exposes `with_state`,
  translating unchecked/mixed/checked into excluded/default/included;
- `TriStateSwitchSpec::default()` therefore resolves to excluded, not the web
  and documented default value;
- the spec contains an undocumented general `label` field;
- every native segment is focusable rather than exposing one roving tab stop;
- disabled segments remain focusable;
- GPUI has no host-supplied instance identity and stamps every instance with
  the same root id;
- the renderer emits when the selected segment is activated again, contrary to
  the shared single-select machine and contract behavior section;
- no named mounted regression proves pointer, keyboard, focus, disabled, or
  host-rebuild behavior.

The repair should replace `state: CheckState` with `value: TriStateValue`,
replace `with_state` with `with_value`, default to `TriStateValue::Default`, and
remove the undocumented `label`. It should then align renderer dispatch and
focus behavior with `singleSelectTransition`, require stable instance identity,
repair the GPUI specimen, and add one mounted behavior regression.

This is intentionally not a compatibility migration. Poodle's pre-1.0 rules
forbid aliases and silent fallbacks, and the affected in-repository call sites
are bounded to Poodle's GPUI and deferred Jetstream specimens/adapters.

### Deferred candidates

- **NumberInput:** still blocked on the raw draft / committed number decision in
  `20260826-213343-number-input-native-value-model.md`.
- **EditableLabel:** still needs a coherent activation, draft, commit payload,
  select-on-focus, and focus-restoration model.
- **Accordion:** still needs a semantic API repair around resulting selection
  versus activated item and overlapping selection fields.
- **Visual and accessibility programmes:** remain separate evidence lanes.

## Decision

The operator approved the breaking Rust migration:

1. `TriStateSwitchSpec.state: CheckState` → `value: TriStateValue`;
2. `with_state(...)` → `with_value(...)` with no alias;
3. default Excluded → Default;
4. remove the undocumented `label` field and `with_label(...)`;
5. update Poodle-owned call sites directly; do not preserve the legacy surface.

This is compiled as
`../roadmaps/g16/013-tri-state-switch-contract-and-mounted-parity.md`.
Expected ledger movement is 41 → 42 mounted and 133 → 132 missing.
Known-delta totals remain 115 present / 60 not-applicable unless card evidence
finds a contract-owned reason to change them.
