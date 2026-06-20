<!-- parity consv=gap gpui=4 jetstream=2 specimen=gap -->
# Parity: MetaItem

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/meta-item.md`
- Svelte (authoritative): `packages/svelte/components/src/MetaItem.svelte`
- GPUI: `packages/gpui/components/src/primitives/meta_item.rs`
- Jetstream: `packages/jetstream/components/src/meta_item.rs`
- Specimens: svelte — **no standalone `MetaItemSpecimen.svelte`**; meta-item is exercised only inside `MetaBarSpecimen.svelte` · gpui — **no standalone specimen file**; `render_meta_item` lives inside `packages/gpui/preview/src/specimens/meta_bar.rs` · jetstream `packages/jetstream/preview/src/specimens/meta_item.rs` (standalone exists)

## Contract ↔ Svelte

Props mostly match (`label` default `null`, `ariaLabel` default `null`, `typography` default `"body"`). Two divergences: an undocumented prop and a wrong inherit ratio.

- Svelte adds `separator?: boolean` (default `true`) → emits `data-separator` on the root span (`MetaItem.svelte:8,16,24`). This is the signal MetaBar reads to decide whether to draw a leading separator dot. Not in contract §2 props or anatomy. **Fix: add `separator` (default `true`) to contract §2 + document the `data-separator` attribute in anatomy/accessibility.**
- Inherit-mode label font-size mismatch. Contract §7 ("When `typography=inherit`") says label `font-size: 0.7857em`, and `MetaItemSpec::label_font_size_rem()` returns `0.7857` for `Inherit` (`packages/contracts/components/src/meta_item.rs:54-58`). But Svelte (authoritative) sets `--poodle-meta-item-label-font-size: 0.6875em` in inherit mode (`MetaItem.svelte:71`). **Fix: contract §7 and the Rust spec are both wrong — change inherit label size to `0.6875em` / `0.6875` to match Svelte.**
- Inherit gap: Svelte uses `0.375em` (`MetaItem.svelte:70`); contract §7 says `0.4286em` and spec `gap_rem()` returns `0.4286` for `Inherit`. **Fix: reconcile — Svelte authoritative, change contract + spec inherit gap to `0.375em` / `0.375`.** (Value font-size `1em` matches across all three.)

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] No `separator` prop / `data-separator` emission — `MetaItemSpec` has no `separator` field, so GPUI cannot signal separator opt-out to MetaBar (`meta_item.rs` whole file). Add the field + builder once spec gains it.
- [ ] Label `font_weight` hardcoded `FontWeight::SEMIBOLD` at `meta_item.rs:74` — contract §7 label weight is `var(--poodle-typography-label-weight)`; resolve from a typography token, not a hardcoded weight.
- [ ] Inherit mode is approximated via the spec's `*_rem()` ratio methods (`meta_item.rs:66,81,95`) — fine per the runtime-note fallback, but it inherits the **wrong** label ratio (`0.7857`, see Contract↔Svelte). Will be correct once the spec ratio is fixed to `0.6875`.
- [ ] Missing typography fidelity: no `letter-spacing: 0.08em`, no uppercase `line-height: 1`, no label `font-family` (`typography.label.family`) — contract §7 specifies all three; GPUI only sets size + weight + color (`meta_item.rs:71-76`). Apply where GPUI text API allows.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored, not emitted.
- accepted: placeholder `"Value"` fallback when no value supplied (`meta_item.rs:79-85`) is preview-only convenience.

## Jetstream gap (vs Svelte + contract)

- [ ] No `separator` prop / signal — same structural gap as GPUI; `MetaItemSpec` lacks the field.
- [ ] Label weight hardcoded `text_weight(600)` at `meta_item.rs:45` — should resolve from `typography.label.weight` token rather than the literal `600`.
- accepted: label/value `font-family`, `letter-spacing`, `line-height` omitted — explicitly an approved Known Delta in contract §10 (Jetstream `JsEl` text surface lacks those controls). Documented in `meta_item.rs:30-32`.
- accepted: interaction is N/A (component is non-interactive; value content owns its own semantics).
- note: gap/size resolve from spec `gap_rem()`/`label_font_size_rem()`/`value_font_size_rem()` via `rem_to_px` (`meta_item.rs:26-28`) — token/spec-derived, no raw px literals (besides the weight `600`). Inherit label ratio inherits the same `0.7857` bug as GPUI; fixed once spec changes.

## Specimen parity

- Svelte covers (inside `MetaBarSpecimen.svelte`): labeled item, value-only item (no label), rich value (`Code` inline + copy), inherit typography. **No dedicated MetaItem specimen** — coverage is incidental to MetaBar.
- GPUI covers (`render_meta_item` in `meta_bar.rs`): Labeled, Rich Value (`Pill` + text), Inherit typography. — missing: label-only (no value) case; lives in shared file rather than its own.
- Jetstream covers (`specimens/meta_item.rs`): label+text value, value-only, label-only, multiple side-by-side, inherit typography — **most complete specimen of the three**. — missing: rich value (`Code`/`Pill`) child, present in Svelte/GPUI.

## Notes

- `specimen=gap`: Svelte has no standalone MetaItem specimen at all, and GPUI's lives inside `meta_bar.rs`; only Jetstream has a dedicated file. Jetstream's specimen is the broadest (5 groups) but skips rich-value children.
- The inherit label-size bug (`0.6875em` Svelte vs `0.7857` contract+spec) propagates into both Rust ports via `label_font_size_rem()` — fixing the one spec method fixes GPUI and Jetstream simultaneously. Same applies to the inherit gap (`0.375` vs `0.4286`).
- The `separator` prop is the cross-cutting gap: it is the contract↔Svelte miss AND the reason both Rust MetaBars cannot replicate Svelte's per-child separator suppression. The field belongs on `MetaItemSpec`, then surfaced through both builders.
