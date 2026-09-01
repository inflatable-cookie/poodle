# Dedicated Conformance Lab Architecture

Status: planning packet — recommendation ready; promotion remains orchestrator-owned
Created: 2026-09-01
Source handoff: `docs/handoffs/20260901-230407-conformance-lab-architecture-planning.md`
Target repository: dedicated internal Poodle conformance-lab repository
Promotion route: orchestrator review, then a separate bootstrap/implementation card

## Outcome

Create a dedicated internal lab repository that can compare named Poodle
fixtures across Svelte, React, and GPUI without adding a reverse dependency to
Poodle or turning the lab into a second component system.

The lab has two distinct jobs:

- Longhorn controls and observes opted-in Tauri webviews without taking focus.
- A short-lived GPUI capture process renders one named native fixture, opens one
  non-activating window, captures only that window, writes a typed receipt, and
  exits.

The lab is diagnostic evidence. Poodle contracts, runtime implementations,
focused tests, current parity reports, and Poodle-owned visual policies remain
the authorities. This packet creates no repository, process, window, capture,
dependency, or Poodle runtime change.

## Settled decisions

These decisions come from the committed handoff and are not reopened here:

- The lab belongs in a dedicated repository, not a Longhorn example.
- Native GPUI capture uses one operator-approved non-activating process per
  fixture, not a long-running sidecar.
- Poodle packages never depend on the lab.
- Fixture identities are bounded adapters, not a universal scene or component
  schema.
- Default Poodle QA and CI remain headless.

## Authority and evidence

Poodle authority:

- [System shape](../architecture/001-poodle-system-shape.md) — contract,
  renderer, application, and Jetstream boundaries.
- [Cross-runtime conformance record](../architecture/009-cross-runtime-component-conformance.md)
  — rejected universal corpus/runner architecture and retained parity rules.
- [Native presentation context](../architecture/010-native-presentation-construction-context.md)
  — explicit native construction inputs; no universal native scene tree.
- [Semantic motion policy](../architecture/012-semantic-motion-policy.md) —
  explicit `frozen` capture policy and restriction-only propagation.
- [Working rules](../contracts/001-working-rules.md) — contract-first ownership,
  active-runtime posture, and headless/default validation boundaries.
- [Parity automation boundary](../specs/025-parity-automation-and-harness-boundary.md)
  — automated evidence versus named manual review.
- [Primitive visual lane](../roadmaps/g15/012-visual-conformance-lane.md) and
  [GPUI crates.io recovery](../roadmaps/g15/059-gpui-cratesio-recovery.md) —
  current visual and native capture limits.
- [Active-cohort evidence ledger](../roadmaps/g16/001-active-cohort-parity-evidence-ledger.md)
  — Button-only GPUI visual comparison and the current missing-evidence
  boundary.
- [Existing lab triage](20260821-165500-longhorn-conformance-lab.md) —
  Longhorn/webview assessment and the prior unresolved ownership question.

External control authority inspected read-only:

- Longhorn contract 022 at
  `/Users/tom/Dev/projects/longhorn/docs/contracts/022-agent-app-control.md`.
  It owns the stateless MCP protocol, loopback/token/origin boundary, discovery
  lifecycle, opted-in child-webview semantics, and window-composed webview
  screenshots. It explicitly excludes genuinely native surfaces from semantic
  control and screenshot composition.
- Longhorn host package notes at
  `/Users/tom/Dev/projects/longhorn/crates/longhorn-agent-control/README.md`
  and
  `/Users/tom/Dev/projects/longhorn/crates/longhorn-tauri-agent-control/README.md`.

## 1. Repository ownership and bootstrap authority

### Ownership split

| Surface | Owner | Lab rule |
| --- | --- | --- |
| Component meaning, public inputs, states, events, accessibility, tokens | Poodle | Read Poodle contracts and pinned artifacts. Never redefine them in the lab. |
| Web/native runtime implementations | Poodle and its runtime backends | Invoke real published or pinned Poodle surfaces. Do not add lab-only component behavior. |
| Webview MCP control, discovery, bearer token, origin checks, window operations | Longhorn | Consume contract 022 and its host adapter. Do not fork the protocol or token implementation. |
| Tauri controller, fixture registry, runtime adapters, orchestration, comparison output | Lab repository | Own the diagnostic workflow only. No Poodle package imports the lab. |
| GPUI capture process and native receipt adapter | Lab repository, consuming Poodle native crates | One process per fixture. No daemon, native webview bridge, or Longhorn native provider. |
| Visual threshold and parity meaning | Poodle planning/contract authority | The lab consumes an already-settled policy. It cannot change a threshold or turn a finding into an accepted delta. |
| Artifact storage and operator review bundle | Lab repository maintainers | Retain provenance and review state. Never silently promote a bundle into Poodle evidence. |

The lab's working repository name is `poodle-conformance-lab`. Naming the
repository does not grant bootstrap authority. The Poodle orchestrator owns the
promotion decision and the first bootstrap card; lab maintainers own the
repository after that card is accepted.

### Bootstrap authority

Bootstrap must start from a clean, pinned input set recorded in the lab
repository:

- Poodle tag or commit for web assets and native crates;
- Longhorn tag or commit providing contract-022 control support;
- GPUI source identity, currently crates.io `gpui = "0.2.2"` for the shipped
  native boundary;
- Rust/Bun/toolchain versions, target OS, architecture, font files and hashes;
- the initial manifest, adapter revision, receipt schemas, and comparison policy
  revision.

The lab may consume Poodle packages or pinned source required by the runtime,
but it must not use a floating branch, a repository-adjacent workspace path, or
an unrecorded sibling checkout as a committed dependency. Its lockfiles own its
resolution. Poodle's package graph must contain no edge back to the lab.

The bootstrap card must also record whether each input is published, source-only,
or internal. React is currently source-only in Poodle; that status must not be
hidden by pretending the lab is a published React consumer. A failed or
unavailable input stops bootstrap rather than causing a local fork or silent
fallback.

## 2. Longhorn control boundary

The controller is a dev-only Tauri application. It mounts Longhorn's existing
agent-control surface and opts in only the webviews it owns, using stable labels
such as `svelte` and `react`.

Longhorn may provide:

- semantic snapshots and ref-addressed click/type/press/scroll/drag for an
  explicitly opted-in webview;
- DOM-relative waits and page-event resources;
- explicit window sizing and targeting;
- a fresh screenshot of the Tauri window's hosted webview surfaces while the
  app is unfocused, occluded, or minimized.

The lab must preserve these boundaries:

- Longhorn controls the Tauri webviews only. It does not control a GPUI process,
  a native window, native menus, or the operator's desktop.
- A genuinely native GPUI surface is not expected in a Longhorn screenshot. The
  lab obtains it from the GPUI process's own receipt/PNG pair.
- The controller opts in only content it owns. A child webview remains closed to
  semantic targeting unless explicitly labelled at mount.
- The lab uses semantic tools for bounded review. It does not make `evaluate`
  the normal fixture path, and it mounts no command registry for the MVP; native
  command/menu authority stays outside the lab.
- Svelte and React assets are precompiled. Bun is a build tool, not an embedded
  runtime or a new package dependency for the capture process.
- The lab does not add a native-surface provider to Longhorn contract 022. That
  is a separate Longhorn contract decision if a future need appears.

For an isolated web capture, the controller shows one named target webview at a
time in a fixed target rectangle and hides the other target. If a later layout
shows multiple webviews, the run must record their physical bounds, z-order, and
crop before accepting a capture. Unlabelled chrome, hidden surfaces, or a
composite screenshot that cannot be partitioned is a failed artifact, not a
comparison tolerance.

## 3. Fixture manifest

### Manifest role

The manifest is a closed registry of named diagnostic inputs. It is not a public
Poodle schema and does not describe component anatomy, arbitrary props, slots,
children, actions, expected behavior, or a render tree.

The common envelope is intentionally small:

```json
{
  "schema": "poodle.conformance-lab.fixture-manifest.v1",
  "manifestId": "button-mvp",
  "poodleRef": "<immutable-tag-or-commit>",
  "runtimes": ["svelte", "react", "gpui"],
  "capture": {
    "logicalViewport": { "width": 240, "height": 80 },
    "scale": 2,
    "motion": "frozen",
    "repeat": 2
  },
  "fixtures": [
    {
      "id": "button/rest-secondary",
      "adapter": "button",
      "group": "resting-variants",
      "theme": "eclipse",
      "size": "md",
      "density": "default"
    }
  ]
}
```

Rules:

- `schema`, `manifestId`, `poodleRef`, runtime set, viewport, scale, motion,
  repeat count, fixture IDs, adapter keys, theme, size, density, and group are
  explicit and closed.
- The adapter owns component-specific input data and validation. The
  controller never interprets a generic `props`, `children`, `nodes`, or
  callback field. A Button adapter may keep its closed tone/variant/content/state
  table beside the adapter, keyed only by the declared fixture ID.
- No unresolved default, `inherit`, expression, condition, loop, action list,
  expected-result field, tolerance, or runtime branch is accepted in the
  manifest.
- A fixture ID addresses one bounded adapter case. It is not an instruction to
  load an arbitrary component or construct an arbitrary scene.
- Every adapter maps the same ID to each requested runtime's real component
  implementation. Missing adapter/runtime support fails the run; it is not
  reported as parity or skipped silently.
- A new component family needs a new bounded adapter and a separate planning
  review. It does not widen this manifest into a universal schema.
- The manifest records its source path, source commit, and SHA-256 when a row is
  imported from Poodle. Later edits require a deliberate manifest revision;
  automatic live syncing is not evidence.

### MVP fixture set

The first lab batch reuses the current accepted Button identities from
`test/visual/fixtures/button-visual-inventory.json`. The adapter copies the
fully resolved rows and records that source commit; it does not make the Poodle
package import the lab manifest.

| Group | Fixture IDs |
| --- | --- |
| Resting variants | `button/rest-secondary`, `button/variant-primary`, `button/variant-ghost` |
| Secondary status tones | `button/tone-danger`, `button/tone-success`, `button/tone-warning` |
| Size ladder | `button/size-xs`, `button/size-sm`, `button/size-lg`, `button/size-xl` |
| Density ladder | `button/density-compact`, `button/density-comfortable` |
| Visual states | `button/state-disabled`, `button/state-loading`, `button/state-pressed` |
| Content shapes | `button/content-leading-icon`, `button/content-icon-only` |
| Reference theme | `button/theme-iceberg` |

The MVP keeps the current Button capture constraints: 240×80 logical pixels,
2× device scale, explicit theme/size/density, frozen visual state, and the
existing 18-fixture repeat/comparison policy. Focus, hover, pointer trust,
keyboard behavior, and component completion remain separate evidence surfaces;
the process focus guarantee below is not a Button focus fixture.

## 4. Short-lived GPUI process protocol

The controller owns orchestration. The GPUI child owns one native render and one
native capture. There is no persistent sidecar and no native MCP server.

### Request

For each fixture and each planned repeat, the controller:

1. creates a private run directory and a fixture subdirectory;
2. writes a closed `capture-request.v1` document containing `runId`, `attempt`,
   fixture ID, adapter key, fully resolved capture axes, output filenames,
   Poodle ref, expected GPUI source/version, and manifest hash;
3. freezes the request and records its SHA-256;
4. directly spawns the capture executable with the request path; no shell,
   interpolation, inherited Longhorn token, or arbitrary executable path is
   allowed.

The request contains identifiers and resolved fixture values only. It contains
no credentials, product data, arbitrary Rust/TypeScript, or behavior program.
The child rejects unknown keys, unknown fixture IDs, path escapes, unresolved
values, unsupported scale, and a source/version mismatch before creating a
window.

### Response

The child returns one closed `capture-result.v1` JSON record on stdout after it
has closed the window and atomically written the PNG and receipt. Diagnostics go
to stderr. The result names:

- `runId`, `attempt`, `fixtureId`, `runtime: "gpui"`;
- PNG and receipt paths relative to the private run directory;
- PNG SHA-256 and receipt schema;
- transport identity `macos-window-server-nonactivating`;
- logical/device dimensions and scale;
- GPUI source/version and environment;
- foreground verdict, sample count, baseline, and observed applications;
- process/window completion evidence.

The parent accepts a result only when the child exited successfully, both files
exist inside the run directory, the receipt's hash matches the PNG, the receipt
matches the request and manifest, and foreground evidence is `proved`.

Typed failure classes are sufficient; they must remain distinguishable in the
run summary:

- `invalid-request`;
- `unsupported-capture-environment` (including no WindowServer or missing
  Screen Recording permission);
- `focus-proof-failed`;
- `render-failed`;
- `artifact-write-failed`;
- `timeout-or-abnormal-exit`.

The parent does not publish partial output or retry a failed attempt under the
same identity. The planned second repeat is a new `attempt` with a new child
process and a new receipt. A timeout terminates the child, closes/removes only
that private unpublished run directory, and marks the fixture failed.

### Native render path

The child must render through the real Poodle path:

```text
bounded fixture adapter
  -> Poodle spec + RenderContext
  -> poodle-render
  -> poodle-node
  -> GPUI backend
  -> one non-activating GPUI window
  -> own-window PNG + typed receipt
```

The child may not replace the component with a hand-built GPUI scene, call a
Poodle handler directly, or use a native preview shortcut as evidence.

## 5. Capture lifecycle

1. **Plan, headlessly.** Validate the manifest, all adapter IDs, immutable
   source pins, toolchain, output root, target viewport, scale, motion policy,
   font hashes, and closed schemas. No window exists in this phase.
2. **Record approval.** An authorized operator approves the exact manifest hash,
   fixture list, source pins, target OS, and output destination. Approval is
   recorded in the run envelope and is not inferred from a branch, PR, or CLI
   flag alone.
3. **Start the controller.** Build/load the pinned static Svelte and React
   assets, mount only owned webviews, and establish the Longhorn discovery/token
   boundary. The controller remains a diagnostic host, not a source of component
   truth.
4. **Capture web targets.** For each fixture, select one visible named webview,
   set explicit theme/size/density and frozen motion, obtain semantic/geometry
   evidence, then request the Longhorn window screenshot. Verify target bounds,
   image dimensions, receipt, and hash before retaining it.
5. **Capture native target.** Spawn one GPUI child for that fixture. The child
   performs its own preflight, starts foreground monitoring before window
   creation, renders, settles, captures its own window, writes evidence, closes,
   and exits. The controller waits for the typed result before starting the next
   fixture.
6. **Verify repeats.** Run the declared repeat count as separate child/runtime
   attempts. For the MVP, each fixture/runtime pair is captured twice and the
   pair must be byte-identical. No averaging, frame picking, or hidden retry.
7. **Compare.** Verify every artifact before comparison. Preserve independent
   dimensions, geometry, token/role, and pixel channels. Svelte↔React uses the
   current exact policy where the same web capture environment justifies it;
   Svelte↔GPUI uses the current renderer-aware policy. The lab reads the policy
   from its pinned Poodle evidence/configuration; it does not invent tolerances.
8. **Review.** Produce a machine summary, diffs, and contact sheet. An operator
   reviews the named findings, known contract deltas, and provenance. A known
   delta stays visible and does not become a hidden pass.
9. **Close.** Stop the controller, confirm no GPUI child remains, finalize the
   run envelope, retain or expire artifacts by policy, and leave Poodle's
   package graph and default QA unchanged.

Any failure before artifact verification stops the affected run. The lab does
not continue with a partial denominator and does not treat omitted runtime
output as `not-applicable`.

## 6. Focus and window guarantees

### GPUI process

The native child must satisfy all of these conditions:

- create exactly one GPUI window with `focus: false` and `show: true`;
- never call `App::activate`, `Window::activate_window`,
  `makeKeyAndOrderFront`, System Events activation, or an equivalent focus-taking
  path;
- identify the window by this process's own PID and capture exactly that window
  ID with `screencapture -x -o -l <window-id>`;
- never use desktop, display, region, cursor, or frontmost-window capture;
- sample the frontmost application before window creation and through the full
  process run; publish only when the baseline is readable, the minimum sample
  count is met, and every observation matches the baseline;
- record `proved`, `changed`, and `unprovable` as separate states. Only `proved`
  is publishable;
- close the window and exit after one fixture. No process remains to receive a
  later fixture.

Static source checks must reject forbidden activation and broad-capture APIs.
Runtime foreground evidence must also pass; either proof alone is insufficient.
The window may be visible. Non-activating means the run does not claim to hide
the window or remove WindowServer/Screen Recording requirements.

### Web controller

Longhorn's synthetic semantic input does not move the OS pointer or require
focus. The controller must keep child labels explicit and must not click native
chrome, invoke native dialogs, or use computer-use/system input as a fallback.
Webview screenshot evidence is limited to what contract 022 proves: hosted
webviews, not genuinely native GPUI content.

## 7. Artifact and provenance retention

### Required run envelope

Every accepted or rejected run retains a small `lab-run.v1` envelope with:

- run ID, manifest ID/hash, fixture IDs, attempts, and operator approval record;
- lab commit, Poodle ref, Longhorn ref/contract revision, GPUI source/version,
  and lockfile/toolchain identities;
- runtime, adapter revision, viewport, scale, theme, density, motion policy,
  and capture policy revision;
- OS, architecture, GPU/display scale, font hashes, token/build hashes, and
  capture timestamps;
- artifact paths relative to the run bundle, PNG/receipt SHA-256 values, receipt
  schemas, comparison summary, diffs, and human review decision;
- native PID/window ID and foreground evidence where applicable, with no bearer
  token or absolute home-directory path.

Receipts are closed and versioned. Component-specific receipt fields stay with
the bounded adapter; the lab envelope supplies shared provenance without
turning into a generic component observation or scene schema. The existing
Button receipt identity `poodle.button-visual-capture.v2` remains evidence for
the current Poodle Chromium comparator. A Tauri/WebKit lab capture must record
its actual engine in a versioned lab/adapter receipt; wrapping it must not
masquerade as Chromium or authorize a generic receipt migration.

### Retention policy

- Scratch directories and failed, unaccepted PNG/diff payloads are private to
  the run and are removed at run close unless the operator explicitly retains
  the failure bundle for investigation.
- MVP accepted raw PNGs, diffs, and contact sheets remain in the restricted lab
  evidence store for 90 days. The manifest, source pins, receipts, hashes,
  summary, approval, and accept/reject decision remain indefinitely.
- An accepted bundle is immutable. A later run creates a new run ID; it never
  overwrites a prior baseline or rewrites its provenance.
- Promotion into a Poodle log, ledger, contract, or baseline requires a separate
  Poodle planning decision and review. The lab cannot make that promotion by
  updating its own manifest.
- If the evidence store cannot preserve the envelope and immutable hashes, the
  run stops before capture.

## 8. Security boundary

### Controller and Longhorn

- The controller is internal and dev-only. It binds loopback only and uses the
  Longhorn contract-022 per-instance bearer token and Origin validation.
- Discovery files and tokens are treated as credentials, are not copied into
  artifacts or logs, and are removed on clean shutdown. Stale discovery is
  handled by the Longhorn contract, not by a lab reimplementation.
- The controller serves only pinned local static assets with a restrictive CSP
  and no external network requirement. The lab does not load arbitrary fixture
  URLs or third-party content.
- Semantic child-webview opt-in is explicit. No child content is driven merely
  because it is present in the Tauri window.
- `evaluate` and command invocation are not part of the MVP run protocol. If the
  mounted Longhorn surface exposes them, the bearer token remains the full trust
  boundary and the lab must never pass untrusted manifest values to them.

### Native process and artifacts

- Spawn the fixed capture executable directly, with a sanitized environment,
  closed stdin, bounded runtime, private output directory, and no shell.
- Allowlist fixture IDs and adapter keys before render. Canonicalize every output
  path and reject traversal, symlink replacement, directory collisions, and
  writes outside the run root.
- Do not inherit Longhorn tokens, consumer application data, credentials, or
  arbitrary environment values into the GPUI child.
- Capture only the child-owned window. Any need for desktop-wide, region, cursor,
  or frontmost-window capture is a security and architecture stop.
- Keep fixtures synthetic and sanitized. Do not capture Longhorn, Loophole,
  Underlay, or operator application data as a fixture.
- Pin dependencies and record hashes. No local fork, floating Git revision, or
  unreviewed external asset enters a run.

## 9. Manual and windowed approval

Windowed capture is never an implicit consequence of running a test or opening a
PR.

The lab exposes separate operations:

- a headless plan/validate/verify path for manifest, adapters, receipts,
  security checks, and comparison logic;
- an explicitly named manual windowed capture path that requires a recorded
  approval for the exact run envelope.

The windowed path is excluded from Poodle `qa`, `ci:native`, release gates, and
ordinary worker validation. It requires a macOS WindowServer session and Screen
Recording permission. Missing permission, an unreadable foreground baseline, or
any focus change fails closed. It never falls back to a desktop or region shot.

Manual review must inspect:

- the full fixture denominator and repeat integrity;
- webview/native transport labels and source identities;
- focus evidence and window ownership;
- independent geometry, role/token, and pixel findings;
- contract-cited known deltas and any unexplained failure;
- artifact hashes and the immutable run envelope.

The reviewer may accept the diagnostic bundle as evidence for the named run.
That acceptance does not mark a component complete, alter a contract, waive a
missing active-cohort cell, or authorize a Poodle baseline update.

## 10. MVP tranche

The first implementation card should deliver one coherent batch:

1. Bootstrap the dedicated internal lab repository with its own instruction
   surface, Effigy tasks, lockfiles, security posture, artifact store contract,
   and pinned Poodle/Longhorn inputs.
2. Add the dev-only Tauri controller with two explicitly labelled webview
   targets, Longhorn contract-022 integration, fixed target bounds, and no native
   command bridge.
3. Add the closed Button adapter and the exact 18-fixture manifest above. Keep
   the adapter's current resolved axes and 2× viewport/scale rules.
4. Add the one-shot GPUI executable and request/result protocol. Adapt the
   current non-activating window transport; do not move a daemon into the lab
   and do not change Poodle's public GPUI dependency boundary.
5. Add receipt/hash verification, repeat-capture integrity, independent channel
   comparison, summary/diff/contact-sheet output, and the run envelope.
6. Add headless negative tests for unknown IDs, extra keys, path escapes,
   mismatched hashes, missing artifacts, unsupported scale, changed foreground
   verdict, forbidden activation APIs, and partial batches.
7. Run one manually approved native fixture and one full approved Button batch
   only after the headless gates pass. Retain the resulting bundle as lab
   evidence; do not add it to Poodle's default board.

MVP completion means the lab can re-run the exact Button batch from immutable
   inputs, prove every runtime/fixture pair or fail closed, retain complete
   provenance, and leave Poodle's package graph and default QA unchanged.

Out of MVP:

- any second component family;
- a universal component/scene/case schema;
- behavior completion or accessibility certification;
- a persistent GPUI sidecar or native Longhorn provider;
- Jetstream admission;
- Poodle workflow, package, contract, or default-QA changes;
- automatic visual baseline update or merge authority.

## 11. Validation

### Packet validation

This packet is valid when the named sections are present, the links resolve in
Poodle, the Longhorn authority is recorded, and the change remains one file.
The delegate run uses:

- `effigy docs:lint`;
- `git diff --check origin/main...HEAD`.

No windowed selector, native visual selector, release mutation, or sibling-repo
write is part of packet validation.

### Future lab validation

The bootstrap/implementation card must validate in this order:

1. Headless schema/manifest/adapter and receipt tests, including all negative
   cases and exact-denominator checks.
2. Static security/source checks for forbidden activation, broad capture,
   token leakage, shell spawning, path escape, and unpinned inputs.
3. Headless web control/observation and comparison tests using synthetic fixtures;
   no claim that these prove GPUI or assistive-technology parity.
4. A manually approved single-fixture GPUI run proving own-window selection,
   non-activation, foreground samples, receipt/PNG hash integrity, clean exit,
   and no leftover process.
5. The manually approved full MVP batch, including two independent captures per
   runtime/fixture pair and the operator contact sheet review.
6. Lab headless QA from a clean checkout, plus a proof that Poodle's default
   `effigy qa`/CI path remains free of the lab and windowed capture.

Any validation result that needs a new contract, public API, Longhorn provider,
threshold, dependency edge, or default-board exception returns to planning.

## 12. Stop conditions

Stop bootstrap or implementation if any of these occur:

- the lab needs to edit or depend on Poodle package code in the reverse
  direction, or Poodle packages would import the lab;
- fixture metadata grows into a universal component interface, scene tree,
  behavior/action language, expected-result authority, or generic runtime
  interpreter;
- Longhorn must control a genuinely native surface, click native chrome, bind
  remotely, weaken token/origin security, or add a provider not admitted by
  contract 022;
- GPUI pixels require activation, a focus-taking fallback, desktop/region
  capture, a modified or floating GPUI source, a long-running sidecar, or a
  silent fallback from missing WindowServer/permission;
- the child cannot prove it owns the captured window or the foreground monitor
  cannot produce `proved` evidence;
- an artifact is missing, mutable, hash-mismatched, path-escaping, or stripped
  of source/provenance fields;
- a runtime is absent and the run would call it `not-applicable`, `deferred`, or
  passing without a separate authority decision;
- an operator requests the windowed path in default QA, CI, release gates, or
  unattended worker validation;
- a threshold, known delta, component contract, runtime behavior, public API,
  or completion meaning must change to make the comparison green;
- a new component family, new host repository, new control surface, or new
  retention/security policy is needed outside the Button MVP;
- Longhorn or Poodle dependency availability cannot be pinned and reproduced
  from the recorded bootstrap inputs.

These are implementation stops, not prompts to reopen the settled decisions in
this packet. The next move after a stop is a new bounded planning decision owned
by the relevant repository/orchestrator.

## Recommendation

Promote this packet as the architecture boundary for a dedicated internal lab,
then compile one bootstrap/implementation card for the Button-only MVP. Keep
the current Poodle Button comparator and evidence ledger unchanged until the
lab proves equivalent artifact integrity and focus/provenance behavior. After
that proof, decide separately whether any lab bundle should be promoted into a
Poodle execution log or visual evidence surface.
