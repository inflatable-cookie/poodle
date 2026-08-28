# g16.020 — Component Continuation Audit

Status: complete — merged in PR #95
Date: 2026-08-28
Card: `docs/roadmaps/g16/020-component-continuation-audit.md`
Register: `docs/roadmaps/g16/component-continuation-register.md`

## Outcome

The audit accounts for the live public component roster once and returns seven
bounded continuation lanes. It keeps implementation work, evidence-only gaps,
operator decisions, and the architecture 011/spec 069 drag programme distinct.
No component, contract, specimen, test, runtime, token, ledger, generated
report, workflow, release, or sibling-repository file was changed.

## Inventory method and counts

- The one-off Bun check used `deriveLiveRoster()` from
  `scripts/parity-evidence-ledger.ts`, the generated
  `packages/codegen/fixtures/preview-catalogue.json`, and the live ledger
  section in `docs/roadmaps/g16/parity-evidence-ledger.md`.
- Result: 175 Svelte exports, 174 portable native components, and one explicit
  native exclusion: MeterSurface. Ledger identity returned 175 rows with no
  duplicate, missing, or extra names.
- Current live mounted-behaviour total after merged PR #94: 47 mounted / 127
  missing / 1 native not-applicable. Select is closed on main.
- Roadmap index: 79 g15 cards and 20 g16 cards, with the generation READMEs,
  release material, and live ledger included in the sorted file inventory.
- August log index: `rg --files docs/logs/2026-08 -g '*.md' | rg -v
  '^docs/logs/2026-08/20260828-g16-020-component-continuation-audit\.md$' |
  sort` contains 219 pre-audit Markdown logs. The audit log is excluded so it
  cannot count itself.
- Triage index: `rg --files docs/triage -g '*.md' | sort` returns 18 files.

Register classification counts:

| Class | Count | Interpretation |
| --- | ---: | --- |
| closed | 93 | No component-specific continuation repair identified. |
| evidence-only | 69 | Evidence remains unfilled without a named implementation defect. |
| known repair | 0 | No current known-repair class remains after merged g16.019. |
| decision-blocked | 3 | NumberInput, EditableLabel, and TimeInput need a contract/API or native interaction decision. |
| programme-owned | 7 | Tabs, EditableList, Tree, ModelCatalogueEditor, OrderBy, BlockEditor, and DockRegion belong to one drag programme. |
| unknown | 3 | Fader, Knob, and XYPad are triage candidates without component-level authority for a repair. |

## Historical reconciliation

g15's focused Svelte/React baseline, native declaration/render/specimen
boundary, named native repairs, specimen review, focus-ring parity, Button
visual comparison, release recovery, and consumer adoption through g15.079
are complete historical inputs. g15.034, .037–.043, .052, and .053 remain
closed and are not reissued as continuation work. g16.002–g16.018 closed the
recorded selection, slider, Tabs lifecycle, text event, DurationInput,
Breadcrumbs, IconButton, disclosure, TriStateSwitch, Accordion, Pagination,
Rating, and Select batches. g16.019 merged as PR #94 and moved Select to the
current 47 mounted / 127 missing ledger totals.

Still-valid non-claims are recorded in the register: TextInput-family
multiline/slug/validation/IME/native-a11y/visual breadth; CodeInput and
DurationInput focus-handle breadth; NumberInput raw-draft and native value
model; EditableLabel activation/draft/commit/focus; TimeInput segment and
bounds semantics; broad GPUI accessibility; visual comparison; motion research;
and Jetstream admission. Select's mounted overlay work is closed; its broader
accessibility and visual evidence remain separate.

The open triage inputs are motion/transition learning, the Longhorn
conformance lab, and NumberInput native value model. Selection, text routing,
post-g16 lane notes, and the old Tabs lifecycle stop are resolved. The
dependable drag/drop triage is promoted into architecture 011 and spec 069.
The g15.055–g15.079 consumer/release sequence is complete; no unresolved
consumer report was promoted into a new component repair.

## Candidate lanes

The register contains seven lanes, ranked by dependency and leverage. Select is
closed by merged g16.019 and is not a candidate lane:

1. TimeInput native entry — next independent foundation candidate, blocked on
   the committed-value/raw-draft and native editing decision.
2. NumberInput and EditableLabel editing value models — decision-blocked and
   separate from TimeInput and drag/drop.
3. One architecture 011/spec 069 drag programme for the seven named
   components; no second drag design and no scattered component cards.
4. Fader, Knob, and XYPad audio interaction candidate; continuous gestures
   stay separate from payload drag/drop.
5. GPUI accessibility evidence as a separate programme choice over the
   portable native roster.
6. Cross-runtime visual comparison as a separate programme choice; Button's
   existing fixture does not imply broad GPUI visual coverage.
7. No current component implementation lane for the remaining 162 rows;
   missing evidence is not promoted to a defect.

The register explains each lane's exact gap, authority, operator or breaking
decision, evidence gain, overlap boundary, batch shape, and readiness.

## Execution friction and unknowns

The handoff declares base `69118d83173d3d69b284b5ecf6d7315dc43ae5a8`, which is
not a Git object in this checkout. Short `69118d831` resolves to actual commit
`69118d83122e976d256af6033e57d1c8e6b1a9de`, and HEAD is its child. The audit
method and denominator were unchanged; the friction is recorded in the root
`PAPERCUTS.md`.

The only intentionally unknown component-level family is Fader, Knob, and
XYPad. Their candidate status comes from triage and the current visual-state
Rust surface, not from an unbounded audio audit. The static and evidence-only
rows do not claim defects where the ledger only lacks mounted, accessibility,
or visual evidence.

## Validation

Validation was run after the coherent docs batch and the locked dependency
bootstrap:

- deterministic roster/accounting proof: pass — 175 Svelte exports, 174
  portable native components, MeterSurface as the only exclusion, and 175
  unique ledger rows with no duplicate, missing, or extra names;
- `effigy check:parity-evidence-ledger`: pass — 175 component evidence rows;
- `effigy docs:lint`: pass;
- `effigy docs:check`: pass after `effigy bootstrap:deps` installed the locked
  dependencies; the React preview package-alias and full docs/build checks
  completed successfully;
- `effigy qa`: pass after the same bootstrap, including the dependency-sensitive
  icon check, the full headless test board, downstream consumer checks, and
  final build/smoke checks;
- `git diff --check origin/main...HEAD`: pass after the worker commit.

All validation stays headless. No windowed, native visual, Jetstream,
release, tag, publication, workflow, or sibling-repository selector was run.

## Merge closeout

PR #95 merged on 2026-08-28 as commit `6b5dea505`. The live ledger remains 47
mounted / 127 missing; the audit changed no implementation or evidence cell.
The next planning checkpoint compiles separate component-continuation and
dependable drag-and-drop runways. Decision-blocked editing candidates remain
unready until their operator choices are resolved.
