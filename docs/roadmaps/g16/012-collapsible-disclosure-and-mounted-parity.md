# g16.012 — Collapsible Disclosure And Mounted Parity

Status: ready
Opened: 2026-08-27
Depends on: merged `g16.011` / PR #85
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/collapsible.md`,
`parity-evidence-ledger.md`,
`../../triage/20260827-141543-post-g16-011-native-lane-decision.md`

## Goal

Make shared Rust Collapsible project one coherent disclosure target, preserve
host-owned controlled and default-open state through GPUI rebuilds, and prove
pointer plus keyboard behavior through real mounted dispatch.

The generated ledger may move exactly one cell: Collapsible GPUI mounted
behavior from `missing` to `mounted`, taking totals from 40 to 41 mounted and
134 to 133 missing. Known-delta totals stay 115 present / 60 not-applicable.

## Current Evidence

- Svelte and React agree: `open` is controlled, `defaultOpen` seeds
  uncontrolled state, accepted activation reports the state the disclosure is
  moving to, and disabled triggers are inert.
- `CollapsibleSpec::current_open()` already resolves `open` over
  `default_open`, and the renderer uses it for paint, content, gap, chevron,
  and callback payload.
- Native semantics read a different value: the outer shell is a `Region` with
  `expanded = spec.open.unwrap_or(false)`. Default-open therefore paints open
  while announcing closed, and the actual trigger is an unnamed focusable
  container rather than a button.
- Open content has no region role or trigger relationship. The node vocabulary
  already carries button/region roles, `expanded`, `controls`, `labelled_by`,
  tab position, runtime ids, disabled state, and structured focus rings.
- Disabled Collapsible suppresses the callback but leaves its trigger focusable
  and pointer-styled.
- The GPUI compatibility wrapper stores no instance id: `with_id(...)` returns
  `self` unchanged. Its specimen calls that method on four instances and
  rebuilds host state after callbacks.
- The evidence ledger records focused web behavior and GPUI construction, but
  no named mounted Collapsible regression.

## Fixed Behavior Envelope

### Open state and host ownership

- Use `spec.current_open()` as the one effective open value for paint,
  content presence, trigger `expanded` state, and next callback value.
- Controlled `open` wins over `default_open`. `default_open` is a seed, not
  renderer-owned mutable state. After `on_open_change`, the host owns the
  resulting value and rebuilds the spec.
- Pointer, Enter, and Space activation each report exactly one inverse boolean
  when available. Disabled activation reports nothing.

### Disclosure target and relationships

- The trigger, not the outer shell, owns explicit button role, accessible
  label, expanded state, content control relationship, activation, sequential
  focus, and the standard structured focus ring.
- A title is the trigger's accessible label. `aria_label` supplies the name
  when title is absent. Do not duplicate the visible title as extra painted
  content.
- The outer shell is layout/presentation only. When open, the content wrapper
  owns `Region` role and is labelled by the trigger.
- Give the trigger and conditional content an instance-scoped identity pair so
  two same-titled Collapsibles do not collide. Make the GPUI wrapper's existing
  `with_id` meaningful and preserve identity across host rebuilds.
- Keep the existing `collapsible(...)` renderer entry point non-breaking. An
  additive identity-aware entry point or handler bundle is allowed. Do not
  derive identity from render order, open state, title text, or a process-global
  counter.
- Disabled triggers are disabled, non-focusable, absent from sequential
  traversal, carry no activation/key handler, and use the not-allowed cursor.
  Root opacity remains the visual disabled treatment.

### Specimen and mounted proof

- Keep the current human-centered specimen groups. Make "Default open" use the
  real default-open seed and host rebuild path; keep feedback compact and do
  not add a conformance matrix.
- Add one named mounted regression through the production renderer, GPUI node
  backend, hit testing, focus chain, and keyboard dispatch. It proves:
  - controlled closed → open and open → closed pointer activation;
  - Enter and Space report the next value and host rebuilds content/expanded
    state;
  - `default_open=true` begins open, announces open, and first activation
    reports false;
  - title and aria-label naming, button/region ownership, controls/labelled-by,
    tab position, focus ring, and stable scoped identity;
  - two same-titled instances keep independent trigger identity; and
  - disabled targets never emit and are skipped by sequential focus.

## Explicit Non-Claims

- This card does not change public Svelte or React behavior or props.
- It does not change `CollapsibleSpec`, break the existing renderer signature,
  or add compatibility aliases/fallback identity.
- It does not implement trigger snippets in native Rust, hidden renderer state,
  content height animation, or exact web transition timing.
- It does not claim broad native assistive-technology coverage. The mounted
  regression proves declared node/backend semantics only.
- It does not change Accordion, TriStateSwitch, NumberInput, EditableLabel,
  Select, Drawer, or other disclosure consumers. ModelCatalogueEditor may
  receive only the smallest mechanical call-site repair if an additive shared
  renderer entry point requires it; its ledger cell must not move.
- It does not admit Jetstream, promote visual comparison, or touch release,
  package, workflow, downstream, or sibling-repository surfaces.
- It does not move any ledger row except Collapsible or any evidence column
  except GPUI mounted behavior.

## Delivery

### 1. Reconcile renderer state and semantics

- Make effective open state single-source and move disclosure semantics from
  the outer shell onto the trigger/content pair.
- Add stable instance-scoped identity through the smallest additive renderer
  boundary while preserving the current helper for composition callers.
- Add focused Rust tests for controlled precedence, default-open projection,
  next-value reporting, semantic ownership/relationships, instance isolation,
  focusability, and disabled suppression.
- Update the Collapsible contract only where GPUI notes or Known Deltas need to
  describe host-owned state and runtime identity. Do not weaken strict parity.

### 2. Wire GPUI and mounted evidence

- Make the compatibility wrapper retain and consume `with_id`.
- Route the interactive specimen through the production identity/callback path
  and make its default-open example honest.
- Add the named mounted regression using real pointer and keyboard dispatch.
  Direct handler calls, spec inspection, or fixture-only state changes do not
  satisfy the card.

### 3. Prove and close

- Regenerate the evidence ledger and verify only Collapsible changes: 41
  mounted / 133 missing. Known-delta totals remain 115 / 60.
- Mark the source triage note resolved, add one August execution log, close
  this card, and return g16 to an orchestrator evidence checkpoint. Do not
  compile or start `g16.013` in the worker thread.

## Acceptance

- [ ] Controlled and default-open state use one effective value for paint,
      content, trigger semantics, and next callback payload.
- [ ] Pointer, Enter, and Space each emit the next open value once; host
      rebuilds project the new content and expanded state.
- [ ] The trigger owns button role, label, expanded/controls state, stable
      scoped identity, sequential focus, and the standard focus ring.
- [ ] Open content owns region role and is labelled by its trigger; the outer
      shell does not falsely claim either disclosure role.
- [ ] Two same-titled instances keep independent identities through rebuilds.
- [ ] Disabled triggers emit nothing, have no interactive handlers, and are
      skipped by sequential focus.
- [ ] The existing specimen stays human-centered and demonstrates a real
      default-open seed through the production wrapper path.
- [ ] One named mounted regression proves the behavior envelope through real
      GPUI production dispatch.
- [ ] Focused Svelte and React Collapsible tests stay green without behavior
      changes.
- [ ] The ledger changes only Collapsible from missing to mounted: 41 mounted /
      133 missing; known-delta totals remain 115 / 60.
- [ ] One August log records the defect, repair, evidence, validation, exact
      non-claims, and next orchestrator checkpoint.

## Writable Scope

- `docs/contracts/components/collapsible.md`
- `packages/render/src/collapsible.rs` and its focused tests
- the smallest export change in `packages/render/src/lib.rs` if an additive
  identity-aware entry point is introduced
- Collapsible-only compatibility/specimen state under `packages/gpui/preview/`
- the smallest Collapsible mounted regression changes in
  `packages/gpui/preview/tests/headless_regressions.rs`
- `packages/render/src/model_catalogue_editor.rs` only for mechanical
  compilation/identity plumbing caused by the additive renderer path; do not
  change ModelCatalogueEditor behavior or evidence
- focused Svelte/React Collapsible tests only if a test-only correction is
  required; do not change web implementation behavior
- generated parity ledger/check surfaces only for the one Collapsible mounted
  cell
- this card, its source triage note, one August log, g16/front-door status,
  and `PAPERCUTS.md` only for new execution friction

Do not edit other component contracts or implementations, `CollapsibleSpec`,
shared node/backend APIs, theme/token definitions, visual fixtures,
accessibility reports, versions, release metadata, workflows, downstream
repositories, or sibling runtime repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-render` Collapsible tests;
- focused Svelte and React Collapsible tests;
- the named mounted Collapsible regression;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy drift:handlers`, `effigy drift:events`, and relevant contract/spec
  drift selectors;
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:native`;
- `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, or publication selectors.

## Stop Conditions

- Svelte and React disagree on controlled/default-open behavior, naming, or
  callback timing.
- Honest identity or disclosure semantics require a breaking renderer/spec
  migration, hidden mutable renderer state, or a new node/backend capability.
- Correct disabled behavior or keyboard activation cannot reuse existing node
  interaction and focus channels.
- Mounted proof requires direct handler invocation, fixture-only behavior, or
  bypassing production focus/key dispatch.
- The ledger generator changes another row or promotes accessibility/visual
  evidence not proved by this card.
- Validation exposes Accordion, TriStateSwitch, NumberInput, EditableLabel,
  Jetstream, release, or downstream work outside this runway.
