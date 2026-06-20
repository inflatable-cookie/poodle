<!-- parity consv=gap gpui=2 jetstream=2 specimen=gap -->
# Parity: PaginationSummary

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/pagination-summary.md`
- Svelte (authoritative): `packages/svelte/components/src/PaginationSummary.svelte`
- GPUI: `packages/gpui/components/src/primitives/pagination_summary.rs`
- Jetstream: `packages/jetstream/components/src/pagination_summary.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/PaginationSummarySpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/pagination_summary_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/pagination_summary.rs`

## Contract ↔ Svelte

- Copy `font-size`: contract §8 says `var(--poodle-typography-body-size)`; Svelte uses `var(--poodle-typography-label-size, 0.75rem)` (line 41) and `line-height: var(--poodle-typography-label-lineHeight, 1.4)` (line 42). **Two mismatches — body vs label typography.** Svelte authoritative → **fix contract §8 Copy table to label-size / label-lineHeight.**
- Empty-state text: contract §4 says `totalItems=0` shows `0-0 of 0`. Svelte computes `from=0, to=0` and renders "Showing 0-0 of 0" (lines 14–15, 24). OK — matches.
- ARIA label phrasing: contract §9 says `"Showing X-Y of Z across N pages"`; Svelte renders exactly `Showing ${fromItem}-${toItem} of ${totalItems} across ${totalPages} pages` (line 21). OK — matches.
- Props (`currentPage=1`, `totalPages=1`, `totalItems=0`, `pageSize=5`): contract §3 defaults match Svelte defaults (lines 3–6). OK.

## GPUI gap (vs Svelte + contract)

- [ ] Empty-state text wrong: Svelte renders "Showing 0-0 of 0" when `total==0`; GPUI prints `"No items"` (`pagination_summary.rs:92-93`). **Fix to "Showing 0–0 of 0".**
- [ ] Copy typography uses `typography.body.size` (`pagination_summary.rs:85`); Svelte uses `typography.label.size` (0.75rem). Resolve `typography.label.size` to match Svelte.
- accepted: no ARIA live region / `aria-label` with page count (gpui has no accessibility API). Spec carries no `total_pages` field, so the "across N pages" copy cannot be produced — note below.

## Jetstream gap (vs Svelte + contract)

- [ ] Text format spacing differs: Svelte renders `Showing {from}-{to} of {total}` (tight hyphen); Jetstream renders `"Showing {} – {} of {}"` with spaced en-dash (`pagination_summary.rs:16`). **Fix to tight `-` to match Svelte exactly.**
- [ ] No empty-state branch: when `total_items==0`, `start_index()` returns 0 and `end_index()` returns 0 (spec), so output is "Showing 0 – 0 of 0" — close, but the spaced dash still diverges from Svelte's "Showing 0-0 of 0". Folded into the format fix above; flagged separately because contract §4 calls out the `0-0 of 0` empty state explicitly.
- accepted: no ARIA live region (platform limit). Interaction n/a — component is read-only.

## Specimen parity

- Svelte covers: Default (1/8/156/20), Single page (1/1/12/20), Large dataset (5/50/1000/20) — matches contract §13 exactly.
- GPUI covers: Default (1/20/156), Single page (1/20/12), Large dataset (5/20/1000). — matches Svelte's three cases. OK (note Single-page uses pageSize 20 not the contract's implied range; visually equivalent).
- Jetstream covers: Page 1 (1/25/67), Page 3 (3/25/67) only. — missing: **Single page** and **Large dataset** cases; uses different data (67 vs 156/1000). Under-covers vs Svelte.

## Notes

- `PaginationSummarySpec` (`packages/contracts/components/src/pagination_summary.rs`) has no `total_pages` field — only `page`, `page_size`, `total_items`. So the Rust impls structurally **cannot** produce the contract/Svelte ARIA copy "…across N pages". Low impact (no ARIA channel anyway) but worth a spec field if accessibility is ever wired. Not counted as a todo since ARIA is an accepted platform delta.
- `consv=gap` driver: contract §8 Copy typography (body-size/body-lineHeight) is stale; Svelte uses label-size/label-lineHeight. Single clean fix to the contract.
- Range math (`start_index`/`end_index`) in the spec matches Svelte's `from`/`to` formulas exactly. Pure logic is correct in both Rust targets.
