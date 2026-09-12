# g18.022 — block-first Slider family

Status: complete — merged as `02ab7f7ec9122d85364beca77d05d681fa4d0124` (PR #254) on 2026-09-12 after exact-head independent review (PR comment `5644429883`, `ready_to_merge`) at `7ed0daf093789177b8dbd67268f95c0a7151bed2` with green rust/web checks
Date: 2026-09-12
Branch: `ns-28f7942c-6586-497a-8d18-045602f654df`
Card: `docs/roadmaps/g18/022-block-first-slider-family.md`
Handoff: `docs/handoffs/20260911-g18-022-block-first-slider-family.md`
Governing refs: `docs/contracts/components/slider.md`,
`docs/contracts/components/range-slider.md`,
`docs/architecture/012-feedback-motion-and-state-change.md`
Base: `origin/main` at `d29415bc5bff0ef481a2e1d14decb11a7d3c444d`

## Outcome

`Slider` and `RangeSlider` ship the rounded-square block presentation as the
default public surface in every runtime. Both controls expose only
`variant="block" | "embedded"` with block the default; `appearance`,
`standard`, `track`, RangeSlider's combined visible-range formatter, and the
RangeSlider fallback text are removed without aliases. RangeSlider renders one
stable whole-capsule text layout (lower value at the logical start, label
centered, upper value at the logical end, upright in both orientations);
vertical block has full parity; block Slider keeps shared unipolar/bipolar
geometry. Embedded remains the sole quiet track alternative with its behavior
unchanged. Full execution detail lives in the card's worker execution and
review-fix logs.

## Review rounds

- Round 1 at `3ed46d03b15040d214ae6066f1e8a4ca335d1df7` (PR comment
  `5644254076`): **changes required** — `SliderVariant` missing from the
  packed `@inflatable-cookie/poodle-svelte/types` subpath
  (`test:web-pack-install` red) and vertical block Slider text not anchored
  to the whole capsule. Fixed by exporting `SliderVariant` from both public
  `types.ts` paths and anchoring the vertical inline row to the capsule.
- Round 2 at `c2ff85142b9ddaf88516ea7b1fe7b3d14081b49a` (PR comment
  `5644331256`): **ready to merge**; both blockers re-verified fixed.
- Verification then failed the `web` gate: Nucleus receipts were still pinned
  to the g18.017 commit `93d08f31266dd77bf2a1d6b550e3c584ccb39e8f` while the
  branch had moved slider sources. Repinned at `5c5c2df402bd2cd905d46e4cf2b36e091bb31de7`
  (Nucleus + GPUI census evidence to the block-first head).
- Round 3 at `5c5c2df402bd2cd905d46e4cf2b36e091bb31de7` (PR comment
  `5644412062`): **changes required** — the repin left the record-state oracle
  in `scripts/gpui-functionality-census.test.ts` hardcoding the g18.001 pin.
  Fixed at `7ed0daf093789177b8dbd67268f95c0a7151bed2` with an identity-free
  oracle (40-hex commit, record/census pin agreement, ancestry/lockfile/sha
  validation).
- Round 4 at `7ed0daf093789177b8dbd67268f95c0a7151bed2` (PR comment
  `5644429883`): **ready to merge**. Only `scripts/gpui-functionality-census.test.ts`
  changed since `c2ff85142`, so the round-2 functional verification stands.

## Validation at the merged head

- `effigy test:gpui-census` — 17/0; `check:gpui-census` — pass;
  `check:parity-evidence-ledger` — pass (176 rows);
  `test:parity-evidence-ledger` — 9/0; `test:nucleus-parity-receipts` — 17/0.
- `effigy docs:check` — pass end to end; `docs:lint` — pass;
  `git diff --check` — clean; worktree clean.
- Carried-over functional proof: `test:web-pack-install` green;
  block-slider inline probe 82/0 on Chromium and WebKit; hit probe 48/0;
  core 1376; GPUI `headless_regressions` 238; `poodle-render` 645 with the
  same 2 pre-existing unrelated failures; svelte-check 0 errors;
  Svelte/React `component-docs.json` in sync.
- Merge gate: PR #254 merged as `02ab7f7ec9122d85364beca77d05d681fa4d0124`
  with green rust/web checks.

## Explicitly not done

- `packages/react/components/test/BlockSliderLifecycle.test.tsx` still passes
  the removed `appearance="block"` prop (inert; block is the default).
- `docs/architecture/007-appearance-recipe-contract.md` still says
  `appearance="block"` and lists a Fallback text role.
- `packages/svelte/preview/scripts/contract-spec-drift.ts:167` still lists
  `formatVisibleRange` in `WEB_ONLY_BY_SLUG["range-slider"]`.
- RangeSlider label centering is exact only for equal endpoint advances
  (`space-between`); web and native agree.
- The synthetic `censusDoc()` fixture in the census test still uses the
  original `d8e174fb…` constant (harmless ancestor).
- Pre-existing on main, untouched: `poodle-render` context a11y wrapper test
  and segmented-control icon-only label test (unrelated controls).

## Continuation

g18.022 is merged. The retained g18.006 release-candidate task resumes after
operator acceptance of merged g18.011 plus merged g18.022/g18.023 and any
blocking repairs; recompute the full `0.4.0` source identity there and
classify this breaking pre-v1 migration in its release evidence. g18.023
still awaits operator dispatch approval; g18.009 stays dependency-queued
behind g18.006. Further planning direction needs the operator.
