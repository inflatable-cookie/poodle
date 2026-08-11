# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

<!-- Keep entries short. Append newest entries at the top. Do not include secrets. -->

- 2026-08-11 — `effigy visual:report` (sweep tier) reports 53 failing
  Svelte↔React pairs at HEAD (308 compared; 46 size / 4 capture / 3 pixels,
  spread across both themes — e.g. `tree`, `xy-pad`, `tool-call`, `fader`,
  `agent-plan`) with no allowlist or debt inventory covering them, so report
  mode cannot distinguish pre-existing parity debt from a card's real
  regressions without a manual before/after comparison (g13.008 had to run
  the gate twice and diff the summaries). Track the failing-pair set (committed
  allowlist with reasons, or a parity-debt inventory) so cards can diff
  against it. Affects visual-gate triage.

- 2026-08-11 — A new public web prop cannot be documented in a contract
  without `effigy docs:lint` failing: the contract ↔ poodle-specs drift gate
  (`packages/svelte/preview/scripts/contract-spec-drift.ts`) requires every
  documented Public Prop to exist on the matching `*Spec` struct, so g13.009's
  `initialFocus` (a focus-behaviour semantic, not web plumbing) cannot ship
  without `DialogSpec`/`FormDialogSpec` gaining `initial_focus` — stopped the
  card; see `docs/logs/2026-08/11-g13-009-dialog-initial-focus.md` §5. Either
  the Specs need the field (native renderers own focus per dialog.md §10) or
  the gate needs a sanctioned web-only carve-out; a decision is required
  before the next prop of this kind lands.

- 2026-08-11 — Batch card 006 (button tone parity) lists `split-button.css`
  as "implements danger", but the contract amendment it depends on
  (`282ce489`) added a `--poodle-split-shadow` column to split-button.md §8
  "Tone: danger" that the stylesheet never implemented — primary danger
  resolves `none` against a mandated elevation shadow (stopped the card;
  see `docs/logs/2026-08/11-g13-006-button-tone-parity.md` §5). Card gap
  tables should be re-verified against amended contracts at dispatch time.

- 2026-08-11 — `docs/guides/svelte-developer-guide.md` §Types still defines
  `type ButtonTone = "default" | "danger"` (2 members) while
  `docs/contracts/004-shared-control-types.md` canonically defines all four
  (`default | danger | success | warning`); the same block restates
  `ControlSize`/`ControlDensity`/`SemanticControlSizeRole`/`ButtonVariant`/
  `ValidationState`/`StatusTone` with no link to 004. The guide contradicts the
  canonical contract and misleads docs-based type resolution. Refresh the block
  or replace it with a 004 reference. Affects docs-authority consistency.

- 2026-08-11 — The g13.002 generated-LOC measure globs
  `packages/core/src/tokens/generated/*` and `packages/tokens/artifacts/**`
  return 0 lines without bash `globstar`: the first matches only the
  `css/`/`ts/` subdirectories and `**` degenerates to `*`. Actual counts are
  1,813 lines / 25 files and 2,407 lines / 31 files. Document the recursion or
  use an explicit recursive glob. Affects reproducible measurement.

- 2026-08-11 — `pilot-expressiveness-corpus.md` §8 counts `EXT` as
  Button 5 / RangeSlider 3 / TextInput 2 (10 total), but the row-level
  classification marks only 3/2/1 (6 total). The manifest records both; the
  count table drifted from the rows. Recompute the §8 table from the rows.
  Affects corpus arithmetic.

- 2026-08-11 — Jetstream RangeSlider densities specimen
  (`packages/jetstream/preview/src/specimens/range_slider.rs`) renders the
  standard variant, while range-slider.md §13 / `RNG-25` specify density
  specimens use embedded bipolar (Svelte/React do). Align the Jetstream
  densities group. Affects cross-runtime specimen parity.

- 2026-08-11 — `docs/parity/text-input.md` status line reads `gpui=2
  jetstream=2` but its own gap sections enumerate 8 GPUI / 9 Jetstream open
  `[ ]` bullets. The pass-level count drifted from the bullets (distinct from
  the stale-path lag of `OBS-04`). Refresh the status line. Affects parity-doc
  readability.

- 2026-08-11 — `effigy docs:check` rewrites the committed
  `packages/tokens/artifacts/rust/*` via `report:parity` → `tokens:build`,
  leaving a dirty worktree, and `effigy audit:tokens` fails at HEAD: commit
  `45caae82` rustfmt-formatted the generated Rust artifacts without updating
  `packages/tokens/scripts/build-tokens.ts`, whose emitter writes 4-space
  override arrays. Align the generator with the committed formatting or
  regenerate the artifacts from it. Affects the docs gate and token drift
  audit.

- 2026-08-11 — React ports of Button and TextInput omit contract-listed
  web-native props: `Button.tsx` has no `formenctype`/`formmethod` passthrough
  and `TextInput.tsx` has no `autocorrect`, all three documented in the
  contracts and implemented in Svelte. Add the props to the React components.
  Affects Svelte↔React surface parity.

- 2026-08-11 — A playbook-style detached `nohup omp -p` launch exited
  immediately with an empty log and untouched worktree under the Codex exec
  harness. Document a harness-safe detach or persistent-session recipe.
  Affects worker orchestration from agent threads.

- 2026-08-10 — `effigy doctor` runs a failing health check that reformats
  generated Rust token artifacts and unrelated Rust tests, leaving a dirty
  worktree. Make doctor diagnostics read-only or isolate formatter output.
  Affects routine repo-health checks.

- 2026-08-10 — `effigy react:build` runs Vite transpilation without a TypeScript
  check; direct `tsc` finds pre-existing `AgentChatStatus` errors the build
  reports as green. Add a React consumer type-check selector and CI lane.
  Affects React parity work.

- 2026-08-10 — A combined `git log -G` secret-pattern history scan ran for
  more than 90 seconds without progress or a result. Add a bounded, cached
  tracked-tree and history secret scanner. Affects public-release audits.

- 2026-08-10 — `effigy tasks` advertises a `distribution` built-in, but
  `effigy distribution --help` resolves it as an undefined catalog task and
  the general help omits it. Align discovery and command routing. Affects
  package-publication audits.

- 2026-08-10 — `effigy graph explore` can hang after a successful index refresh
  without progress or timeout output. Add a bounded query timeout and a clear
  fallback diagnostic. Affects agent code-navigation workflows.

- 2026-08-10 — `cargo fmt --manifest-path` follows Poodle's linked Rust
  workspaces and reformats unrelated crates, creating broad release-sweep
  churn. Add an Effigy formatter that accepts an explicit changed-file set.

- 2026-08-09 — Vite JSON named exports omit kebab-case `lucide-static`
  keys, so bundle-safe imports cannot express most icon names. This blocks a
  plain `createIconSet({ ... })` helper over `icon-nodes.json`; add a build-time
  extractor or a per-icon node package surface. Affects web icon adoption.

- 2026-08-06 — `effigy doctor` reports the repo's `isolation` manifest key as
  unsupported, so routine health checks cannot go green on the checked-in
  manifest. Align the manifest schema or update Effigy's accepted config keys.

