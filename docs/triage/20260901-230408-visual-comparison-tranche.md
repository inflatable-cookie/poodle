# Cross-Runtime Visual Comparison Tranche

Status: accepted tranche — blocked on the operational dedicated lab
Disposition: preserve six components / 24 fixtures; do not dispatch locally
Captured: 2026-09-01
Owner: Poodle planning delegate
Source: `docs/handoffs/20260901-230408-visual-comparison-tranche-planning.md`
Promotion authority: orchestrator
Scope: one finite visual-comparison tranche for the active three-runtime cohort

This packet is planning evidence. It is not a contract, architecture decision,
roadmap card, fixture implementation, comparator implementation, ledger edit,
lab implementation, or merge authority. The PR is intake for orchestrator
review.

No operator questions are reopened here. The handoff's settled choices are
preserved below; the remaining exact tranche shape is this delegate's bounded
recommendation.

## Settled Decisions Preserved

- Visual comparison comes before a broad GPUI accessibility programme because
  stock GPUI 0.2.2 still blocks real assistive-technology automation.
- Component-specific semantic, mounted, and accessibility evidence remains
  authoritative for those dimensions. Pixels cannot upgrade or replace it.
- The tranche is finite and named. It does not revive specimen snapshots as an
  exhaustive conformance corpus.
- The tranche consumes the dedicated conformance-lab boundary. It does not
  silently decide lab ownership, bootstrap, IPC, lifecycle, or process reuse.
- Default QA and CI remain headless. No local windowed selector is part of this
  packet.

## Tranche Recommendation

### Family

Select the six-component selection-control family:

1. `Checkbox`
2. `Switch`
3. `RadioGroup`
4. `SegmentedControl`
5. `ToggleGroup`
6. `TriStateSwitch`

This is the highest-leverage finite family for the next visual comparison
because it combines reusable form, filter, settings, and toolbar primitives
with explicit visual contracts and recently closed mounted paths. Every member
already has contract, Svelte, React, shared Rust, and GPUI construction
evidence; every member has bounded GPUI mounted evidence in the current
ledger. Their visual ledger cells are still bounded by the Button-only
comparison boundary.

The family also exercises distinct but related visual laws: binary and ternary
states, selected and unselected indicators, grouped options, disabled group
and item states, orientation, equal-width and content-width layouts, density,
and theme-resolved semantic colors. It gives useful leverage over shared
tokens, control geometry, labels, and state paint without requiring overlay
capture, drag, motion, text entry, or platform-host behavior.

The first tranche does not add text inputs, sliders, overlays, media, motion,
or workstation composites. Those families either carry interaction or host
questions that would make a visual result hard to attribute. It does not add a
seventh selection control without a new bounded decision.

### Scope boundary

In scope:

- 24 closed fixture identities, four per named component;
- direct, controlled visual states rendered by the real Svelte, React, and GPUI
  components;
- explicit theme, size, density, viewport, scale, and fixture inputs;
- exact Svelte-to-React comparison and renderer-aware Svelte-to-GPUI
  comparison;
- deterministic repeat capture, typed receipts, geometry and paint-role
  evidence, pixel comparison, and mandatory manual review;
- a bounded evidence report that can update only the six named visual ledger
  rows after the run is valid.

Out of scope:

- behavior, keyboard, focus management, semantics, or assistive-technology
  acceptance; those remain separate evidence tracks;
- hover, focus-visible, read-only, transition, loading, or action-replayed
  frames in this first static tranche;
- specimen snapshots, a universal scene, a component IR, an observation model,
  a generic fixture registry, or a baseline updater;
- GPUI offscreen rendering, a local windowed selector, a long-running sidecar,
  or any change to the GPUI dependency;
- Jetstream, the broader GPUI accessibility programme, package publication,
  release work, and workflow changes;
- changing a contract, token, API, behavior, architecture boundary, or
  existing Button fixture/comparator.

## Closed Fixture Set

The fixture set is a selection-control inventory, not a reusable cross-runtime
schema. Each row is a hand-written identity with component-local inputs. The
future implementation must reject unknown identities and fields rather than
silently applying defaults. Adding another component, state, theme, size, or
density is tranche expansion and stops the work.

Every row uses a controlled value, an explicit accessible label where the
contract requires one, an explicit public `size`, `sizeRole="control"`, and an
explicit density. No row uses a custom color override.

All labels and option values are fixed as written. The adapters must not
generate copy, ids, options, or state from fixture names.

### Checkbox

| Fixture id | Direct component inputs | Theme | Size | Density |
| --- | --- | --- | --- | --- |
| `checkbox/unchecked-md` | `checked=false`, `mixed=false`, `label="Enable"` | `eclipse` | `md` | `default` |
| `checkbox/mixed-xs-compact` | `checked=false`, `mixed=true`, `label="Include inherited"` | `eclipse` | `xs` | `compact` |
| `checkbox/checked-xl-comfortable` | `checked=true`, `mixed=false`, `label="Remember choice"` | `eclipse` | `xl` | `comfortable` |
| `checkbox/disabled-checked-iceberg` | `checked=true`, `mixed=false`, `disabled=true`, `label="Unavailable"` | `iceberg` | `md` | `default` |

The mixed row is controlled because the contract defines `mixed` as a
controlled-only visual state. No toggle action is replayed.

### Switch

| Fixture id | Direct component inputs | Theme | Size | Density |
| --- | --- | --- | --- | --- |
| `switch/off-md` | `checked=false`, `label="Auto-save"` | `eclipse` | `md` | `default` |
| `switch/on-xs-compact` | `checked=true`, `label="Dark mode"` | `eclipse` | `xs` | `compact` |
| `switch/on-xl-comfortable` | `checked=true`, `label="Sync"` | `eclipse` | `xl` | `comfortable` |
| `switch/disabled-on-iceberg` | `checked=true`, `disabled=true`, `label="Locked"` | `iceberg` | `md` | `default` |

The tranche compares the public on/off and disabled paint. It does not infer
read-only behavior from a screenshot or exercise the native change path.

### RadioGroup

| Fixture id | Direct component inputs | Theme | Size | Density |
| --- | --- | --- | --- | --- |
| `radio-group/vertical-selected-md` | `value="pro"`, `orientation="vertical"`, `options=[Basic, Pro, Enterprise]`, `ariaLabel="Plan"` | `eclipse` | `md` | `default` |
| `radio-group/horizontal-selected-xs-compact` | `value="m"`, `orientation="horizontal"`, `options=[S, M, L]`, `ariaLabel="Size"` | `eclipse` | `xs` | `compact` |
| `radio-group/vertical-selected-xl-comfortable` | `value="enterprise"`, `orientation="vertical"`, `options=[Free, Pro, Enterprise]`, `ariaLabel="Plan"` | `eclipse` | `xl` | `comfortable` |
| `radio-group/disabled-option-iceberg` | `value="free"`, `orientation="vertical"`, `options=[Free, Pro, Enterprise (disabled)]`, `ariaLabel="Plan availability"` | `iceberg` | `md` | `default` |

`options` means the contract's option records with the displayed label and
value matching the names above. The disabled option remains in the group so
the adapter exercises muted item paint without changing option order.

### SegmentedControl

| Fixture id | Direct component inputs | Theme | Size | Density |
| --- | --- | --- | --- | --- |
| `segmented-control/equal-selected-md` | `value="grid"`, `equalWidth=true`, `options=[Grid, List, Table]`, `ariaLabel="View mode"` | `eclipse` | `md` | `default` |
| `segmented-control/unequal-selected-xs-compact` | `value="effects"`, `equalWidth=false`, `options=[Effects, Instruments]`, `ariaLabel="Plugin kind"` | `eclipse` | `xs` | `compact` |
| `segmented-control/disabled-option-xl-comfortable` | `value="all"`, `options=[All, Active, Archived, Draft (disabled)]`, `ariaLabel="Status filter"` | `eclipse` | `xl` | `comfortable` |
| `segmented-control/disabled-group-iceberg` | `value="list"`, `disabled=true`, `options=[Grid, List, Table]`, `ariaLabel="Disabled view mode"` | `iceberg` | `md` | `default` |

The unequal-width row uses text options only. Icon-only rendering is held for
a later fixture decision so icon geometry does not confound this first family
comparison.

### ToggleGroup

| Fixture id | Direct component inputs | Theme | Size | Density |
| --- | --- | --- | --- | --- |
| `toggle-group/single-selected-md` | `selectionMode="single"`, `value="grid"`, `options=[Grid, List, Board]`, `ariaLabel="View mode"` | `eclipse` | `md` | `default` |
| `toggle-group/multiple-selected-xs-compact` | `selectionMode="multiple"`, `value=["design", "docs"]`, `options=[Design, Engineering, Docs]`, `ariaLabel="Filter tags"` | `eclipse` | `xs` | `compact` |
| `toggle-group/disabled-option-xl-comfortable` | `selectionMode="single"`, `value="all"`, `options=[All, Active, Archived (disabled)]`, `ariaLabel="Status filter"` | `eclipse` | `xl` | `comfortable` |
| `toggle-group/disabled-group-iceberg` | `selectionMode="single"`, `value="list"`, `disabled=true`, `options=[Grid, List, Board]`, `ariaLabel="Disabled view mode"` | `iceberg` | `md` | `default` |

The multiple row proves simultaneous selected paint. `allowDeactivation` is
not set because its interaction law is not a static visual input.

### TriStateSwitch

| Fixture id | Direct component inputs | Theme | Size | Density |
| --- | --- | --- | --- | --- |
| `tri-state-switch/default-md` | `value="default"`, `options={excluded:"Exclude", default:"Default", included:"Include"}`, `ariaLabel="Filter mode"` | `eclipse` | `md` | `default` |
| `tri-state-switch/excluded-xs-compact` | `value="excluded"`, `options={excluded:"Exclude", default:"Default", included:"Include"}`, `ariaLabel="Filter mode"` | `eclipse` | `xs` | `compact` |
| `tri-state-switch/included-xl-comfortable` | `value="included"`, `options={excluded:"Exclude", default:"Default", included:"Include"}`, `ariaLabel="Filter mode"` | `eclipse` | `xl` | `comfortable` |
| `tri-state-switch/disabled-custom-iceberg` | `value="included"`, `disabled=true`, `options={excluded:"Hide", default:"All", included:"Show"}`, `ariaLabel="Visibility filter"` | `iceberg` | `md` | `default` |

The three first rows cover each state color and capsule position. The fourth
row adds disabled opacity and the contract's custom-label input without
changing the fixed three-segment order.

## Runtime, Theme, Size, And Density Matrix

The matrix is intentionally sampled, not Cartesian. The 24 identities are the
whole denominator. It is not a claim to cover every combination of six
components, two themes, five sizes, and three densities.

| Axis | Fixed values | Coverage rule |
| --- | --- | --- |
| Runtime | Svelte, React, GPUI | Every fixture is captured in all three runtimes |
| Theme | `eclipse`, `iceberg` | 18 Eclipse rows and six Iceberg rows; the Iceberg rows are the fourth row for each component |
| Size | `xs`, `md`, `xl` | Each component has one boundary-small, one baseline, and one boundary-large row; the Iceberg row returns to `md` |
| Density | `compact`, `default`, `comfortable` | Each component has one row at each density; the Iceberg row uses `default` |
| Size role | `control` | Fixed for all components |
| Static state | Selected/unselected, mixed, on/off, disabled item/group, orientation, width mode, and ternary state | Only the direct states listed in the fixture tables are authoritative |
| Repeat | Two observations per runtime/fixture | Repeats must be byte-identical at the decoded image and typed-receipt level |

Capture frame:

- logical viewport: `360 x 128`;
- scale: `2x` for all runtimes, matching the current native capture seam;
- fixed canvas theme background from the selected theme;
- component host origin: `(16, 16)` logical pixels;
- component-local layout is hand-written for each adapter and follows the
  component contract; it is not a universal scene layout;
- labels and options are chosen to fit without wrapping or truncation;
- motion and clock-dependent paint are frozen before capture.

`sm` and `lg`, all focus/hover/read-only frames, custom color overrides, and
icon-only options are deliberately outside the denominator. The packet makes
no visual claim for them.

## Capture Authority And Provenance

| Concern | Authority | Boundary |
| --- | --- | --- |
| Meaning, states, parts, and token roles | The six component contracts under `docs/contracts/components/` | A screenshot cannot redefine a contract or token |
| Web visual reference | Svelte implementation in the same pinned headless Chromium environment | Svelte is the web reference for GPUI comparison; it does not donate behavior or native evidence |
| Web peer | React implementation in the same pinned headless Chromium environment | React must match Svelte exactly for this static visual set |
| Native visual source | Dedicated conformance lab's short-lived, operator-approved, non-activating GPUI capture process | The lab owns native process, permission, window-activation, provenance, and transport details |
| Evidence status | `docs/roadmaps/g16/parity-evidence-ledger.md` after the run | Only the six named visual rows may be updated by the future execution owner |

The dedicated-lab architecture handoff at
`docs/handoffs/20260901-230407-conformance-lab-architecture-planning.md` is a
transport dependency. Its settled boundary is compatible with this tranche:
the lab is a separate internal repository, Poodle packages do not depend on
it, native capture is operator-approved and non-activating, and default QA/CI
is headless. This packet does not choose the lab's ownership, bootstrap,
Longhorn boundary, IPC, lifecycle, or process-reuse details.

If that lab boundary is not available, the tranche stops. It must not fall
back to `test:visual-button-comparison-windowed`, a GPUI preview screenshot,
the historical offscreen fork, or a locally opened window.

Every accepted capture has a typed, family-specific receipt with a small common
provenance envelope:

- fixture identity and component;
- runtime and runtime revision;
- resolved theme, size, density, size role, logical viewport, and scale;
- decoded image dimensions and content hash;
- component-local paint-part landmarks and expected token roles;
- pinned Chromium version for web or GPUI source/version plus OS/architecture
  for native;
- lab/capture protocol revision and capture timestamp.

Receipts must not contain hostname, username, credentials, customer data, or
absolute local paths. The common envelope is only provenance. Paint landmarks
remain component-local; the implementation must not turn the envelope into a
generic observation model.

## Comparison Method

### Real component adapters

Use six hand-written adapters per runtime. Each adapter takes only the direct
inputs of its named fixture and renders the real component. No adapter may
reconstruct the component from a screenshot, use a specimen snapshot, replay
an action, or infer omitted props from the fixture id.

The component-local paint parts are:

| Component | Required visual parts in the receipt |
| --- | --- |
| Checkbox | root, indicator, mark when present, label |
| Switch | root, track, thumb, visible label when present |
| RadioGroup | root, each option, each indicator, each dot when present, each label |
| SegmentedControl | root, each segment, each selected surface, each label |
| ToggleGroup | root, each item, selected and disabled item surfaces |
| TriStateSwitch | root, selection capsule, each segment/control, each label |

These are six closed adapter contracts, not a shared component schema. A
missing, extra, duplicated, stale, or unknown part is an integrity failure.

### Runtime comparison

1. Compare decoded Svelte and React pixels in the pinned web environment. They
   must have equal dimensions, equal component-local receipt geometry and paint
   roles, and zero differing decoded pixels. A PNG metadata difference is not
   a visual difference; a decoded pixel difference is.
2. Compare decoded Svelte and GPUI pixels using the fixed renderer-aware policy
   below. Svelte remains the reference image; GPUI receives no pass by
   comparing only to React.
3. Run geometry, paint-role/color, and pixel channels independently. A pixel
   result cannot hide a missing part, wrong state, token-role drift, or
   geometry failure.
4. Require two identical observations for every runtime/fixture pair before
   either comparison result is accepted. A repeat mismatch invalidates the
   pair; do not average frames, choose a favorable frame, or widen a threshold.

### Fixed tolerance policy

The policy starts from the accepted Button comparison policy and is applied
uniformly to this family. It has no per-fixture allowlist and no known-delta
exception.

| Channel | Rule |
| --- | --- |
| Root/group/track/segment/option edges | At most `0.5` logical pixel per corresponding edge |
| Indicator, mark, dot, thumb, and selection-capsule centers and extents | At most `1.0` logical pixel per corresponding center or extent |
| Label and option-content extents | At most `2.0` logical pixels |
| Expected fill, border, and text-role colors | At most one 8-bit sRGB channel |
| Focus-visible paint | N/A for this static tranche; no focus frames are captured or compared, so absent focus paint cannot fail a comparison or create a focus claim |
| Border width | At most `0.5` logical pixel |
| Shadow layers and inset | Layer count and inset exact; geometry at most `0.5` logical pixel |
| Full viewport pixels for Svelte-to-GPUI | Pixelmatch threshold `0.1`, `includeAA=false`, differing pixels no more than `3%` of the full viewport |

The web pair has zero differing decoded pixels. The native policy is not a
license for structural drift: geometry and paint-role checks remain blocking,
and the 3% full-viewport rule is only one channel. Existing or newly observed
renderer differences are annotations, not passes. For example, missing native
shadow, altered selected or disabled treatment, or a color-mix approximation
still produces a finding when it exceeds the fixed policy. Focus-visible paint
is the explicit N/A exception defined in the policy table, not a hidden pass.

### Manual review

Machine comparison does not provide final visual acceptance. A named visual
reviewer must inspect one contact sheet containing all 24 fixture rows and the
three primary runtime captures for each row, then inspect every diff and every
near-threshold result. The review record must state, per fixture:

- selected/unselected or ternary state placement;
- disabled item/group opacity and state paint;
- option order, orientation, equal-width/content-width behavior, and grouping;
- label legibility and absence of clipping or wrapping;
- token-role and renderer-delta findings;
- whether the result is accepted, accepted with a recorded finding, or
  rejected.

The reviewer may not turn a mismatch into an acceptance by classifying it as a
known delta. A mismatch that requires a contract, token, API, behavior, or
renderer-architecture change is routed out of this tranche.

## Artifact Retention

Working capture output is disposable by default. On a valid completed run, the
future execution card retains a sanitized evidence bundle with its execution
log under:

`docs/logs/2026-09/assets/visual-selection-controls/`

Retain only the first end-to-end run's reviewed evidence:

- 72 primary capture images and typed receipts: 24 fixtures × three runtimes;
- 48 comparison result records and diff images: Svelte-to-React and
  Svelte-to-GPUI for each fixture;
- one contact sheet, one environment/provenance summary, one machine summary,
  and the manual-review record;
- the second end-to-end run's hashes and aggregate metrics, not its duplicate
  images.

The second run is a determinism check, not a second corpus. Invalid captures,
temporary screenshots, repeat images, and raw host paths stay outside the
canonical tree and are removed after review. There is no committed baseline,
baseline updater, golden-image refresh command, or silent replacement of an
earlier evidence bundle. The durable log records findings and the exact
fixture/runtime/axis provenance instead.

## Ledger Claims

This packet does not edit the ledger. The current six rows are already
`present`/focused or mounted through the nonvisual tracks, with Web visual
`focused` and GPUI visual `missing`. The future evidence owner may make the
following row-local update only after all 24 identities have valid receipts,
both runtime comparisons have run, and manual review is recorded:

- `Checkbox`, `Switch`, `RadioGroup`, `SegmentedControl`, `ToggleGroup`, and
  `TriStateSwitch`: Web visual `focused` → `compared`;
- the same six rows: GPUI visual `missing` → `compared`;
- all other interface, Svelte, React, Rust, GPUI construction, GPUI mounted,
  Web accessibility, GPUI accessibility, and known-delta cells remain
  unchanged.

`compared` means that the named comparison evidence exists. It does not mean
that every channel passed or that the component is complete. Pixel, geometry,
token-role, or manual findings remain explicit in the execution log. If the
batch is partial, the lab is unavailable, receipt integrity fails, or the
comparison never completes, no visual ledger status moves. No claim is made
for the other GPUI visual gaps, for Jetstream, or for an exhaustive visual
conformance denominator.

## False-Positive Controls

- Keep the 24 identities closed, component-local, and explicit. Reject unknown
  keys, omitted required values, duplicate ids, duplicate option values,
  invalid sizes/densities/themes, and implicit ambient defaults.
- Set theme, resolved size, density, viewport, scale, font, and motion/time
  policy before first paint. Use the same pinned Inter/font assets and pinned
  Chromium build for both web runtimes.
- Render controlled state directly. Do not use pointer, keyboard, focus, or
  animation replay to reach a static frame, and do not treat a behavior test as
  a visual receipt.
- Require decoded-image and typed-receipt identity across the two observations
  for every runtime/fixture pair. Integrity failure stops the run.
- Compare Svelte-to-React exactly and Svelte-to-GPUI separately. No runtime may
  borrow another runtime's image, receipt, or ledger status.
- Use geometry, paint-role, color, and pixel verdicts independently. A large
  empty canvas cannot mask a component-local geometry or role failure.
- Keep the fixed tolerances global. Do not add per-fixture thresholds,
  threshold widening, frame selection, pixel averaging, or a known-delta
  suppression list.
- Require the native lab receipt to prove the approved non-activating capture
  boundary and to include sanitized provenance. Window activation/frontmost
  change, permission drift, missing provenance, or a long-running sidecar
  invalidates the run.
- Keep semantic, mounted, accessibility, and visual evidence separate. A
  visual match cannot close a GPUI AT gap; an axe or mounted result cannot serve
  as a pixel comparison.
- Review every primary capture and every diff/near-threshold result manually.
  Do not silently drop crowded, failed, or inconvenient rows.

## Cost Bound

The fixed batch has:

- 24 fixture identities;
- three runtimes;
- two observations per runtime/fixture pair;
- 144 capture observations per end-to-end run;
- two end-to-end runs for determinism, capped at 288 observations total;
- 72 retained primary capture artifacts, 48 retained comparison result/diff
  sets, and one 24-row contact sheet;
- no cross-product expansion, extra runtime, extra theme, extra size, extra
  density, or unbounded retry.

The native process topology is lab-owned. The cap counts requested observations
and excludes no row because a native capture is inconvenient. A transport or
receipt failure invalidates the run and stops it; it is not repaired by
silently retrying or substituting a local selector. A repair-driven rerun is a
new orchestrator decision, not hidden tranche work.

Implementation effort is bounded to one selection-control-specific inventory,
six hand-written adapters per runtime, a closed comparator and receipts, and
the evidence report. A generic fixture engine, code generator, scene system,
or shared observation abstraction is outside the bound.

## Validation Plan

### Packet validation

This delegate validates only the named packet:

- `effigy docs:lint`;
- `git diff --check origin/main...HEAD`;
- a range-file assertion that the diff contains exactly
  `docs/triage/20260901-230408-visual-comparison-tranche.md`.

No local `*-windowed` selector is run.

### Future execution validation

Before capture, the future implementation card must run the narrow headless
checks for the closed inventory and comparator, including planted failures for
unknown, missing, extra, duplicate, stale, hash-mismatched, and implicitly
defaulted inputs. It must also run the relevant existing Svelte, React, Rust,
and GPUI mounted checks without turning their results into visual evidence.

The dedicated lab owns native capture-process and window-activation/permission
checks. Poodle-side execution uses headless visual fixture/comparator selectors
plus `effigy docs:lint`; it does not invoke a local windowed or native visual
selector. The complete 24-row batch is run twice through the lab boundary,
then receives the manual contact-sheet review before any ledger update.

## Stop Conditions

Stop and return to the orchestrator when any of the following occurs:

- the work needs a universal fixture schema, scene tree, component IR,
  normalized observation model, action language, generic registry, codegen, or
  specimen snapshot corpus;
- a fixture needs an uncontrolled default, ambient context, generated copy,
  unapproved custom token, or non-contract input;
- any named component cannot render its real implementation in Svelte, React,
  or GPUI, or a runtime would borrow a capture or receipt from another runtime;
- the dedicated lab is not ready, cannot provide sanitized provenance, activates
  or changes frontmost window state, requires unapproved permission, uses a
  local windowed selector, or crosses its own process/lifecycle boundary;
- either repeat differs, an image or receipt hash is wrong, a part is missing or
  duplicated, dimensions differ, or the input manifest is stale;
- fixed tolerances reject antialiasing that needs a policy decision, or appear
  to permit structural, token, or state drift; thresholds are not widened;
- a result passes pixels but fails geometry, paint roles, contract state, or
  manual review; channels remain independent;
- a mismatch calls for a contract, public API, token, behavior, accessibility,
  renderer, package, workflow, or architecture change;
- the batch needs a seventh component, a new fixture state, another theme,
  `sm`/`lg`, focus or interaction frames, icon-only cases, or any matrix
  expansion;
- a reviewer cannot inspect all 24 rows and their diffs within the fixed
  review bound;
- the work begins to define the conformance lab, revive the rejected
  conformance/specimen architecture, broaden GPUI accessibility, or involve
  Jetstream, release, or CI changes.

## Promotion Route

1. The orchestrator reviews this packet against the dedicated lab architecture
   decision and either promotes this exact six-component/24-fixture boundary
   or rejects it as a whole. The delegate does not promote it.
2. A separately owned execution card implements the closed inventory, six
   runtime adapters, receipt/comparator checks, and disposable capture path.
3. The execution owner runs the headless checks, requests the two bounded lab
   observations per runtime/fixture, runs both end-to-end passes, and records
   manual findings.
4. Only valid completed evidence may update the twelve visual cells named in
   the ledger section. Findings remain findings; no visual result closes a
   semantic, mounted, accessibility, or architecture decision.

The next move after this PR is orchestrator review and, if accepted, a
separate execution-card dispatch. This packet itself is complete and contains
no implementation or merge action.
