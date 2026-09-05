# Holistic Posture Assessment — Open Remainder

Status: open — pruned 2026-09-05; remaining: repository settings and web pair architecture
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

### Validation hygiene — promoted as `g16.107` (2026-09-05)

### Repository settings (operator-owned)

- 98 merged remote branches were never deleted; enable delete-on-merge.

### Docs compaction — promoted as `g16.108` (2026-09-05)

Active roadmap front doors (`roadmaps/README.md`, `generation-index.md`,
`g16/README.md`) stay narrative until generation rollover.

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
