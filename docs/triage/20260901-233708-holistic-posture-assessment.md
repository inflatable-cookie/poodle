# Holistic Posture Assessment — Open Remainder

Status: open — pruned 2026-09-04 to the items not yet promoted or rejected
Captured: 2026-09-01; revised 2026-09-02; pruned 2026-09-04
Owner: Chatterbox (planning)
Provenance: the full advisory, its verified-facts table, and the operator's
2026-09-02 decisions are in git history of this file (through `409f1ee93`) and
in the surfaces they were promoted to. Do not reconstruct them here.

## Promoted (removed from this note)

- Parity goal, Nucleus as the switch target, execution-backed ledger:
  `../roadmaps/g16/nucleus-gpui-parity-programme.md`, `g16.062`–`g16.093`.
- Release truth and scanner: `g16.053`, `g16.054`.
- Compiled web distribution, `sideEffects`, `marked` as optional peer:
  `g16.056`–`g16.061`, architecture 014.
- Native source-of-truth repairs: `g16.063`–`g16.066`.
- Jetstream hold: `20260902-000959-jetstream-admission-hold.md`.
- React retain + drift gate: `g16.095` (ready).
- Linux web + Rust PR/main board: `g16.096` (ready).
- Underlay direct-import rule: `AGENTS.md`, product guardrails, vision §Underlay.
- Consumer defect intake lane: `../roadmaps/README.md` rules.

## Still open

### Validation hygiene (candidates, each assessed separately)

- `test:web-pack-install` leaves two tarballs in the checkout; doctor then
  reads them as invalid UTF-8. Move pack output to a temp dir.
- `scripts/gate-tree-guard.ts:27` keys its snapshot on one global
  `/tmp/poodle-gate-tree-guard.json`; concurrent worktrees can consume each
  other's state. Key by worktree.
- Doctor scan config (`quality/effigy.scan.toml:72-87`) excludes only
  token/icon generated roots, not the committed catalogue/specimen roots
  `tasks/effigy.tasks.toml:14-25` calls intentional inputs; every `#[allow(`
  scores high. A permanently red doctor is ignored.
- `docs:machine-shape-drift` exits 1 with 20 findings and sits in no board;
  `docs:value-domain-drift` has 20 findings but is report-only without
  `VALUE_DOMAIN_ENFORCE=1`; `lint-docs.ts:3093` adds only contract-only prop
  errors so Svelte-only drift is green in the composed gate.
- `test:contracts` omits `packages/contracts/node`; no board typechecks
  React (`g16.095` adds a prop gate, not a typecheck).
- Two GPUI harness flake causes: `window_capture.rs:820-836` temp-dir
  collision under parallel tests and a stderr-dropping smoke wrapper;
  `specimen_probe.rs:295-346` asserts a 120 s wall clock while four shards
  share one global registry lock.
- The component denominator lives in seven places (`specimen_probe.rs:41`,
  the census test, the ledger script, `parity.ts:754`, two JSON reports, the
  demo audit). One `public-surface.json` manifest would derive all of them.

Next check: bundle into one "validation hygiene" card only after the operator
confirms it is worth a lane; otherwise leave as papercut-class debt.

### Repository settings (operator-owned)

- 98 merged remote branches were never deleted; enable delete-on-merge.

### Docs compaction (docs-only; suits a low-cost model)

- `g16/README.md` and `roadmaps/README.md` are PR narratives; reduce to
  status tables and leave narrative to logs.
- 121 of 125 handoffs are uncited; `docs/parity/` (141 files) is marked
  historical yet g16 cards instruct editing it; 28 specs are unreferenced and
  spec 001 is still `draft`; the working rules' rollover purge never ran.
- Guides teach retired APIs: `Tabs variant="underline"`
  (`svelte-developer-guide.md:903,908`), two-member `ButtonTone` (`:1226`),
  pre-state Popover trigger (`:820`), HistoryCenter v1 props in
  `component-docs.ts:5135`. Add a guide-snippet compile check.
- `docs/contracts/components/README.md` lists `token-input.md` twice.
- `packages/jetstream/adapter/README.md` says the adapter implements no
  components; it implements 108. Quarantine/delete stays a gated
  architecture decision; the README line is a plain defect.

Next check: propose one docs-compaction card to the operator once the two
ready lanes are dispatched.

### Web pair architecture (needs a decision before any card)

- Large composites duplicate logic per shell (DataTable 632/604 lines with 0
  in core; Button 264/185 with 0 in core; Select, Tabs, Tree, MarkdownEditor
  duplicate loader, overflow, rename, and cursor logic). Extraction into core
  is one card per composite, and only worth it while React is retained.
- `packages/core/src/index.ts` exports ~751 symbols, ~45% with no shell
  consumer, including test fixtures (`MODEL_*_FIXTURES`) and clip/motion
  internals. `file-upload.ts:102` uses `document` outside `dom/`.

Next check: raise with the operator when React's first consumer is named, or
when a composite is next touched for another reason.

## Promotion Route

Each "Still open" block becomes at most one card after operator confirmation
in a Chatterbox conversation. Remove this note when every block is promoted or
rejected.
