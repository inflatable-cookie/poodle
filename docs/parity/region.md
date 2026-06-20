<!-- parity consv=ok gpui=1 jetstream=5 specimen=gap -->
# Parity: Region

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/region.md`
- Svelte (authoritative): `packages/svelte/components/src/Region.svelte`
- GPUI: `packages/gpui/components/src/primitives/region.rs`
- Jetstream: `packages/jetstream/components/src/region.rs`
- Spec: `packages/contracts/components/src/region.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/RegionSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/region.rs` · jetstream `packages/jetstream/preview/src/specimens/region.rs`

## Contract ↔ Svelte

Svelte matches the contract: `label`/`color`/`minHeight` props with matching defaults, `role="presentation"`, dashed border, uppercase muted label, no child content. No divergence.

- Contract §7 names the border token `--poodle-color-border-default`; Svelte uses exactly that. OK.
- Contract §3 explicitly states "Region does not accept child content." Svelte honours this (renders only the label). **Jetstream violates this** — see below.

## GPUI gap (vs Svelte + contract)

GPUI is close: dashed border via `.border_2().border_dashed()`, padding/radius/label-size from tokens, custom color overrides both border and label, uppercases the label.

- [ ] `line_height(relative(1.5))` and `FontWeight::SEMIBOLD` are hardcoded styling (`region.rs:92-94`). Minor — contract has no line-height token and Svelte uses `font-weight: 600`; acceptable but note. Letter-spacing `0.05em` from Svelte is omitted (no token); accepted delta.
- accepted: no ARIA (`role="presentation"` not emitted; gpui has no accessibility API). Decorative-only behavior preserved.

## Jetstream gap (vs Svelte + contract)

- [ ] **Border is solid 1px, not dashed 0.125rem** — `.border(1.0)` (`region.rs:19`); contract §2/Svelte require a `dashed` border at `0.125rem` (2px). No dashed style applied. Primary visual gap.
- [ ] **Accepts `children: Vec<JsEl>`** (`region.rs:10,33-35`) — contract §3 forbids child content; Region renders only the label. Drop the `children` param.
- [ ] Label size hardcodes `rem_to_px(0.75)` (`region.rs:29`) instead of resolving `spec.label_text_size_token()`. Use the token.
- [ ] Label is not uppercased — Svelte/contract §2 render uppercase placeholder copy; `js_region` passes `spec.label` verbatim (`region.rs:27`). Uppercase it (or apply text-transform).
- [ ] `spec.color` custom-color override is ignored — border/label always use default tokens (`region.rs:11-12`); contract §5 custom-color state requires border+label adopt the supplied color. GPUI handles this; Jetstream does not.
- accepted: interaction n/a (Region is non-interactive); no ARIA channel.

## Specimen parity

- Svelte covers: Default, **Custom colors** (4 colored regions), **Layout composition** (nav/toolbar/content grid).
- GPUI covers: Default, Custom colors (4), Layout composition. — full parity with Svelte.
- Jetstream covers: Labeled region (with child content — contradicts contract), Unlabeled region. — missing: **Custom colors**, **Layout composition**; and its specimens demonstrate the forbidden children path rather than the contract states.

## Notes

- Jetstream specimen passing text children into `js_region` (`specimens/region.rs:17-24`) bakes the contract violation into the integration test — rewrite to label-only once the `children` param is removed.
- `RegionSpec::min_height_px` is an `f32` px prop (default 64.0 = 4rem). It is a caller-supplied dimension, not a token target, so px storage is acceptable; the contract default (`4rem`) is preserved.
- Specimen `group()` helper hardcodes `text_size(11.0)` (jetstream `specimens/region.rs:31`) — specimen chrome.
