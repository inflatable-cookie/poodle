# g14 Conformance Estate

Status: active baseline
Measured: 2026-08-14
Owner: Poodle orchestrator

## Problem Baseline

Poodle has no component-level gate that proves the same portable interface,
executable fixture, behaviour, and semantic output across the active Svelte,
React, and GPUI cohort while preserving a renderer-neutral Rust boundary.
Catalogue specimen structure is a separate human-facing documentation concern.

The previous frozen baseline measured:

- 168 Svelte component files
- 16 GPUI and 17 Jetstream registration gaps; 18-component union
- 21 name-mapped TS/Rust vector pairs, but only four canonical transition
  machines in both languages
- approximately 86k lines of hand-written specimen code across four runtimes
- 14 drift gates, each covering one projection rather than component
  completion

Those figures are point-in-time evidence, not a completion denominator. The
g14.001 executable roster is now established from source: the Button corpus
(`packages/core/src/conformance/button-cases.ts`) enumerates 20 cases across
the three active runtimes, executed by the opt-in
`effigy conformance:complete` selector.

## Current Gates And Holes

| Claim | Current evidence | Silent hole |
| --- | --- | --- |
| Svelte props match contracts | `docs:contract-drift` | React, Rust, composition regions, portable methods |
| value domains align | `docs:value-domain-drift` | report-only by default; currently 19 disagreements and 10 unresolved types under enforcement |
| callbacks match contracts | `docs:callback-drift` | React and native event shape/timing |
| Rust specs match contracts | `docs:spec-drift` | behaviour, defaults interpreted by renderer, web implementation |
| native roles appear | `drift:roles` | placement, state, accessible name, GPUI evidence |
| native has a handler | `drift:events` / `drift:handlers` | correct handler, payload, timing, actual backend action |
| machine names have vectors | `docs:machine-shape-drift` | vector depth and equal output; currently fails after generated imports |
| React specimen registered | `docs:react-specimen-drift` | specimen content and GPUI |
| capability declaration has a trace | `docs:capability-drift` | undeclared capability vocabulary and actual parity |
| visual baselines exist | web and native snapshot tools | shared fixture identity; stale GPUI captures; Jetstream overwrite workflow |
| **Button completes the conformance kernel** | `ci:conformance` (20 cases × Svelte, React, GPUI, all headless) | profile pilots 2–6 |

Operator ruling, 2026-08-14: the foreground GPUI proof is not acceptable as a
normal worker validation path. Local invocation is now hard-guarded; g14.023
migrates the same backend execution and observations onto GPUI's in-memory test
platform before the remaining profiles continue. Landed 2026-08-15:
`conformance:complete` and `ci:conformance` execute the full cohort headless
in any local worktree, and the AppKit activation/calibration/retry path is
deleted.

`docs:check` currently stays green while the machine-shape selector is red.
`check:svelte` currently has three `AppHeaderCenterHarness.svelte` Snippet
identity errors. These failures predate the redesigned runway; g14.001
recorded them as baseline (they are untouched by this card).

## Conformance Kernel Status (g14.001)

One portable interface + one typed case corpus → Svelte / React / GPUI
execution → normalized observations → one failing completion gate (spec 066,
architecture 009, active-cohort working rules). Curated catalogue specimens
remain a separate documentation surface; an exhaustive projection may later
appear in a dedicated `Conformance` tab.
Delivered for Button:

- **Portable interface module** — `packages/core/src/conformance/button.ts`.
  The single authority: `defineComponentInterface` takes a `const` generic
  and mapped types derive portable props, events, part ids, states, token
  roles, and axes — no hand-written type mirror exists anywhere. Svelte and
  React bind their shells to the derived types (`satisfies`-checked carrier
  names, `PortableEventsOf` key access), so a rename fails the shells.
- **Typed case corpus** — `packages/core/src/conformance/button-cases.ts`
  (20 cases), authored through `componentCase(buttonInterface, ...)`:
  fixture props, regions, parts, states, events, token roles, axes, and
  enum values are closed over the interface at authoring time, re-validated
  by the serializer, and validated again by the Rust codegen against the
  interface JSON. Unknown names are errors, never ignored. Nullable props
  stay `Option` in generated Rust — absence is `None`/`null` on both
  surfaces, pinned by the `default-pressed-toggle` case.
- **Generated Rust declaration** — `packages/contracts/components/src/generated/button/mod.rs`
  replaces the hand-written `ButtonSpec` struct/default/builders; the token
  recipes live in the extension module beside it.
- **Runtime harnesses** — web runners (Svelte + React, real DOM + real
  events + real CSS geometry), GPUI runner (GPUI 0.2.2's in-memory test
  platform: TestAppContext / VisualTestContext / TestWindow, real backend
  focus through the node-backend's focus registry, real Enter key
  activation through the window's dispatch tree).
- **Observation** — `component-observation.v1` per runtime, data-driven from
  the interface's part descriptors and observation rules. No component
  identifier, class name, icon name, or part list lives in shared runner or
  observer code. Token roles travel on the `poodle-node` `roles` channel
  the renderer stamps; roles are read from `a11y.role`; labels through
  `Node::intrinsic_text()`. Icon identity is observed and asserted on all
  three runtimes through the web `data-icon` channels and the native Icon
  nodes. The orchestrator compares the normalized observations
  field-for-field (shape + value), not just asserted fields.
- **Strict verdicts** — `pass`/`fail` only. A required field a runtime
  cannot observe fails that runtime's case, naming runtime/case/step/field
  and the reason; no cross-runtime "someone exercised it" vacuity exists.
- **Standing enforcement** — `docs:check` and `ci:web` carry read-only
  authority checks and web execution. A dedicated path-scoped macOS PR
  workflow runs `ci:conformance`, including full GPUI execution and
  normalized comparison. `qa`, `ci:conformance`, and `ci:native` are all
  headless.
- **Catalogue correction** — g14 initially replaced all three active Button
  specimen pages with corpus projections. Commit `8ac863b4` restored the last
  curated pages. The corpus remains executable harness authority, not the
  catalogue's teaching structure.
- **Completion** — `effigy conformance:complete` passes
  the active cohort and reports Jetstream program-deferred, never passing.
  Removing the GPUI registration fails completion; an inert backend binding
  fails the executed cases. Planted-failure tests cover an inert listener,
  a wrong focus target, a missing selected state, and broken drag/keyboard
  event order (g14.023).

### Defects the corpus caught (fixed, not waived)

- GPUI double-activation: the node-backend bound Enter/Space `on_key_down`
  while gpui itself synthesizes Enter/Space KeyUp → click on focused
  clickable elements — one Enter fired the handler twice. The redundant
  binding is removed; the click binding is the single activation path.
- Native never projected `aria-pressed`/`aria-expanded`/focus-visible:
  `poodle-render::button` now sets `a11y.toggled`, `a11y.expanded`, and a
  focus style, and stamps token roles.

### Recorded native gaps (not required Button cases)

- `fit`, `truncate`, `max_width` are declared portable but `poodle-render`
  does not consume them (web-only behaviour today). Tracked for `g14.014`.
- Keyboard activation on web: happy-dom implements no browser default
  actions, so the web harness performs the browser default (keydown then
  click); GPUI proves the real keyboard confirm path.

### Driver notes (recorded, not architecture)

- The headless GPUI driver (g14.023) runs on the in-memory test platform and
  takes no OS focus. Every draw invalidates the mount view so paint-time
  backend observations are deterministic, and all input goes through
  `TestWindow`'s real dispatch callback (hit testing, focus chain,
  listeners). The legacy AppKit activation, click calibration, first-click
  retry, and foreground opt-in plumbing are deleted.

### Cost ruling

The final exhaustive report records 4,522 source LOC plus 33,392 bytes of
generated JSON:

- generic kernel: 2,947 LOC
- Button pilot increment: 1,575 LOC, including 1,052 LOC of Button harness
- replaced Button declaration plus the temporary specimen projection delta:
  619 LOC. Curated specimen deletion no longer counts as pilot savings.

The stop condition is triggered. The orchestrator accepts `g14.001` as a
feasibility proof, not a rollout verdict. Cards `002`–`007` must reuse or
extract the pilot harness; `008` still decides adoption.

## Primitive Substrate Status (g14.002)

Finite typed roster → web / render-neutral / GPUI probes →
`primitive-capability-report.v1` → completion rejects missing owned evidence.

- **Authority** — `packages/core/src/conformance/primitives.ts`. Component
  capability names validate against the roster at interface authoring.
- **Report** — `effigy conformance:primitives-report` emits the machine gate
  and Markdown matrix. Jetstream is program-deferred outside rows. GPUI
  mounted accessibility is contract-003 forced-acceptance outside passing
  rows.
- **Probes** — shared headless GPUI driver; hand-built node fixtures (not a
  public dummy component); web fixtures + Button cases through the existing
  adapters.
- **Legacy capability tooling** — `capabilities.json` + `capability-drift.ts`
  adapted as non-passing debt evidence. `timers` retired from the primitive
  roster.
- **Native visual** — compare is read-only; refresh preserves
  `*.previous.png` and writes a refresh manifest; `--control-size` is live
  end to end.

See `docs/logs/2026-08/14-g14-002-primitive-substrate-certification.md`.

## Controlled-control Status (g14.003)

RangeSlider proves a controlled two-part value through the same kernel:

- **Authority** — `range-slider.ts` + 10 typed cases; scrub/key/value vocabulary
  stays generic (no RangeSlider-specific runner branches).
- **Execution** — Svelte / React / GPUI all green; compare covers button +
  range-slider corpora (30 cases × 3 runtimes).
- **Native** — dual identified thumbs, keyboard, scrub Release; GPUI scrub uses
  captured `on_drag_move` for real out-of-bounds drags plus the mouse-move
  fallback on the headless platform.
- **Specimens** — the three curated RangeSlider pages remain documentation;
  exhaustive cases execute through the conformance harness and may later gain
  a separate diagnostic projection under g14.026.
- **Disposition** — two-thumb claims live in RangeSlider cases; single-value
  `slider` vectors remain. Jetstream stays program-deferred.
- **Known boundary** — `RangeSliderSpec.law` is a structured
  `AudioValueLaw`, outside the pilot's scalar/number-pair interface vocabulary.
  The profile does not claim full interface replacement until g14.010 rules on
  structured portable values.

See `docs/logs/2026-08/14-g14-003-range-slider-controlled-control-proof.md`.

## Identified-collection Status (g14.004)

Tabs extends the kernel from fixed parts to ordered, semantically identified
collections:

- **Authority** — `tabs.ts` + 9 typed cases; `items` is a structured collection
  and repeated trigger/panel parts resolve as `<part>:<item.value>`, independent
  of fixture order.
- **Execution** — Svelte / React / GPUI run controlled selection, disabled
  items, horizontal/vertical arrows, wrap, Home/End, automatic/manual
  activation, focus, relationships, events, token roles, and bounded geometry.
- **Native focus boundary** — stable per-instance runtime ids isolate GPUI
  focus handles without changing portable semantic ids or relationships;
  renderer key effects return focus targets, GPUI executes them, and roving
  `tab_index` reaches real GPUI tab stops. The conformance driver no longer
  completes focus after key dispatch.
- **Generic vocabulary** — collection prop fields, repeated parts, keyed web
  resolution, native id templates, selected/tabbable/orientation/controls/
  labelled-by observations. The runners contain no Tabs component branch or
  second item corpus.
- **Specimens** — the three curated Tabs pages retain explicit examples for
  supported behavior. Web keeps overflow and close/reorder operator evidence;
  GPUI keeps observable close and residual presentation examples, without
  claiming unsupported native reorder or overflow behavior. Exhaustive cases
  stay in the conformance harness and may later gain a separate diagnostic
  projection under g14.026.
- **Cost** — 1,126 LOC Tabs pilot increment and 23,010 bytes of Tabs fixture JSON.
- **Residual surfaces** — `TabsSpec` remains because overflow, history,
  close/reorder, tooltips, and host actions are outside this bounded profile.
  Headless Tabs vectors remain for close/reorder claims not replaced here.
- **Deferred runtime** — Jetstream remains program-deferred.

See `docs/logs/2026-08/14-g14-004-tabs-collection-navigation-proof.md`.

## Experimental Surface Disposition

No experimental surface is architecture merely because it merged.

| Surface | Provisional disposition | Deciding milestone |
| --- | --- | --- |
| RangeSlider native slider role | keep | done |
| Rust component/scene IR shell artifacts | retire or isolate from active component path | g14.001 |
| five generated display specimens | adapt fixture content into cases or retire | g14.001 / g14.009 |
| Rust display specimen component definitions | retire; must not duplicate portable interface authority | g14.001 |
| generated machine interfaces | adapt only if the kernel replaces declarations and fixes its standing gate | g14.001 |
| machine vectors | adapt as case inputs where they prove component behaviour | profile pilots |
| capability registry | adapt as debt/evidence; execution authority is primitive-capability-report.v1 | g14.002 |
| prop/callback/spec drift scripts | consolidate behind component completion | g14.014 |
| native registration and snapshot tooling | repair and feed completion evidence | g14.002 done / g14.014 |
| stale specs 063–065 and old roadmap | archived/retired | done |
| **g14.001 conformance kernel (typed interface, corpus, observers, GPUI runner)** | keep — the replacement-pass proof; profile pilots 2–6 reuse it | g14.010 |
| **g14.002 primitive capability roster + report** | keep — substrate certification beneath profile pilots | g14.010 |
| **g14.003 RangeSlider controlled-control proof** | keep — second profile pilot; reuses Button harness path | g14.010 |
| **hand-written ButtonSpec declaration surface** | replaced by `generated/button/mod.rs` + extension module | done |
| **curated Button specimens (3 active runtimes)** | restored; keep as documentation, audit under g14.026 | g14.026 |
| **curated RangeSlider specimens (3 active runtimes)** | restored; keep as documentation, audit under g14.026 | g14.026 |
| **g14.004 Tabs identified-collection proof** | keep — stable keyed repeated anatomy and navigation profile | g14.010 |
| **curated Tabs specimens (3 active runtimes)** | restored; retain useful overflow/reorder, close, decoration, panel, and scale examples, then audit under the human-centred rubric | g14.026 |
| **hand-written TabsSpec + Tabs machine vectors** | retain residual overflow/history/close/reorder claims not replaced by g14.004 | g14.010 / g14.011 |
| **generated specimen scenes (specimen-ts/rust targets)** | shell/nav support only; do not promote into a universal catalogue scene tree | g14.021 / g14.026 |

## Staged Licence Intake

The licence surface enters through a bounded web-reference tranche while the
conformance kernel is still under proof. Web delivery is not completion.

| Component | Web reference | Review | Native/shared cases | Completion state |
| --- | --- | --- | --- | --- |
| `LicenceStatus` | g14.015 landed | g14.016 | g14.017 after adopt | incomplete |
| `LicenceActivation` | g14.015 landed | g14.016 | g14.017 after adopt | incomplete |
| `LicenceSeats` | g14.015 landed | g14.016 | g14.017 after adopt | incomplete |

`LicenceCentre` is an explicit non-goal. The downstream comparison between
Poodle's structural field mirrors and Longhorn's generated field maps remains
Longhorn-owned; g14.015 exports the Poodle side but cannot claim that gate.
Jetstream is program-deferred rather than a per-component known delta.

## Staged Model-Connection Intake

The model-connection suite enters through the same bounded web-reference
posture. Poodle owns presentation and interaction only; Nucleus and Swallowtail
remain external authorities.

| Component | Web reference | Review | Native/shared cases | Completion state |
| --- | --- | --- | --- | --- |
| `ModelConnectionPicker` | g14.018 | g14.019 | g14.020 after adopt | incomplete |
| `ModelConnectionSetup` | g14.018 | g14.019 | g14.020 after adopt | incomplete |
| `ModelConnectionCard` | g14.018 | g14.019 | g14.020 after adopt | incomplete |
| `ModelCatalogueEditor` | g14.018 | g14.019 | g14.020 after adopt | incomplete |

The existing `ModelPicker` stays the per-thread model/options control. No
provider registry, route fallback, credential authority, provider schema, or
model-default policy enters this intake. Jetstream remains program-deferred.

## Cleanup Rule

Every active claim ends with one canonical gate. A legacy gate may stay while
coverage migrates, but it needs an owner and retirement condition. Generated
artifacts stay out of hand-edited source roots where possible (the poodle-specs
`generated/` module is generated and gated byte-exact). Known generated-source
and god-file health findings are owned by g14.021 rather than normalized as
permanent warnings.
