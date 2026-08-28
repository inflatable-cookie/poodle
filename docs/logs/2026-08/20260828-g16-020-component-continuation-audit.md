# g16.020 — Component Continuation Audit

Status: audit complete — validation recorded below
Date: 2026-08-28
Card: `docs/roadmaps/g16/020-component-continuation-audit.md`
Register: `docs/roadmaps/g16/component-continuation-register.md`

## Outcome

The audit accounts for the live public component roster once and returns eight
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
- Current live mounted-behaviour total: 46 mounted / 128 missing / 1 native
  not-applicable. PR #94's provisional 47 / 127 remains external and was not
  copied into current evidence.
- Roadmap index: 79 g15 cards and 20 g16 cards, with the generation READMEs,
  release material, and live ledger included in the sorted file inventory.
  August log index: 218 Markdown logs. Triage index: 19 files.

Register classification counts:

| Class | Count | Interpretation |
| --- | ---: | --- |
| closed | 92 | No component-specific continuation repair identified. |
| evidence-only | 69 | Evidence remains unfilled without a named implementation defect. |
| known repair | 1 | Select mounted overlay work. |
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
Rating, and Select semantic-machine batches.

Still-valid non-claims are recorded in the register: TextInput-family
multiline/slug/validation/IME/native-a11y/visual breadth; CodeInput and
DurationInput focus-handle breadth; NumberInput raw-draft and native value
model; EditableLabel activation/draft/commit/focus; TimeInput segment and
bounds semantics; Select mounted overlay behavior; broad GPUI accessibility;
visual comparison; motion research; and Jetstream admission.

The open triage inputs are motion/transition learning, the Longhorn
conformance lab, and NumberInput native value model. Selection, text routing,
post-g16 lane notes, and the old Tabs lifecycle stop are resolved. The
dependable drag/drop triage is promoted into architecture 011 and spec 069.
The g15.055–g15.079 consumer/release sequence is complete; no unresolved
consumer report was promoted into a new component repair.

## Candidate lanes

The register contains eight lanes, ranked by dependency and leverage:

1. Select mounted overlay — active external g16.019 / PR #94 lane; keep main's
   46 / 128 totals until review disposition.
2. TimeInput native entry — next independent foundation candidate, blocked on
   the committed-value/raw-draft and native editing decision.
3. NumberInput and EditableLabel editing value models — decision-blocked and
   separate from TimeInput and drag/drop.
4. One architecture 011/spec 069 drag programme for the seven named
   components; no second drag design and no scattered component cards.
5. Fader, Knob, and XYPad audio interaction candidate; continuous gestures
   stay separate from payload drag/drop.
6. GPUI accessibility evidence as a separate programme choice over the
   portable native roster.
7. Cross-runtime visual comparison as a separate programme choice; Button's
   existing fixture does not imply broad GPUI visual coverage.
8. No current component implementation lane for the remaining 161 rows;
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

Validation was run after the coherent docs batch:

- deterministic roster/accounting proof: pass — 175 Svelte exports, 174
  portable native components, MeterSurface as the only exclusion, and 175
  unique ledger rows with no duplicate, missing, or extra names;
- `effigy check:parity-evidence-ledger`: pass — 175 component evidence rows;
- `effigy docs:lint`: pass;
- `effigy docs:check`: failed at the existing React preview package-alias
  boundary: `@inflatable-cookie/poodle-core/tokens` cannot be resolved by
  `packages/react/preview` during `parity:report`. The earlier docs checks in
  the same selector passed. This is recorded in the existing root
  `PAPERCUTS.md` entries;
- `effigy qa`: failed at its first dependency-sensitive gate: the resolved
  `lucide-static` version is 1.35.0 while the manifest requires 1.31.0. This
  is also an existing root `PAPERCUTS.md` entry;
- `git diff --check origin/main...HEAD`: pass after the worker commit.

All validation stays headless. No windowed, native visual, Jetstream,
release, tag, publication, workflow, or sibling-repository selector was run.
