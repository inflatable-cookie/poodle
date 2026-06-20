<!-- parity consv=gap gpui=1 jetstream=1 specimen=gap -->
# Parity: IconProvider

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/icon-provider.md`
- Svelte (authoritative): `packages/svelte/components/src/IconProvider.svelte`
- GPUI: `packages/gpui/components/src/primitives/icon_provider.rs`
- Jetstream: **missing** — no `packages/jetstream/components/src/icon_provider.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/IconProviderSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/icon_provider.rs` · jetstream — none

## Contract ↔ Svelte

IconProvider is a pure context boundary with no DOM output, no tokens, no states. Props and anatomy match. One documentation divergence:

- Contract §9 states "Uses `setIconRegistry(registry)`" and "Import `setIconRegistry`". Svelte actually uses `setIconSet` (`IconProvider.svelte:5,22`; defined in `icon-registry.ts:32`). The function name in the contract is wrong. **Fix: update contract §9 to `setIconSet` (and reference `icon-registry.ts`).**
- Svelte prop is `icons: IconSet` (matches contract §3). Svelte additionally seeds an empty set via `setIconSet({})` then syncs the prop in an `$effect`, and notes string-based lazy auto-import from `@poodle/icons-lucide` — an implementation detail beyond the contract, acceptable but worth a one-line mention in §9.

## GPUI gap (vs Svelte + contract)

- [ ] No registry wiring — `IconProvider::into_element` (`icon_provider.rs:27-29`) just returns its child; it never sets any icon-set context or `Arc<IconRegistry>`. Contract §10 allows a global registry instead of scoped context, but the provider must actually make a registry available to descendant Icons. Right now it is a no-op pass-through. Either wire it to the GPUI registry or document that GPUI uses a global registry and IconProvider is intentionally inert (contract §10 / §12 Known Delta already permit this — pick one and make it explicit).
- accepted: no visual output, no tokens — nothing to resolve (contract §8).
- accepted: no ARIA (no DOM element by design).

## Jetstream gap (vs Svelte + contract)

- [ ] **No implementation.** There is no `icon_provider.rs` in `packages/jetstream/components/src/` and no `set_icon_set` / registry-context mechanism in Jetstream source. Top-priority gap: add `js_icon_provider` (or document that Jetstream resolves icons from a global registry and the provider is intentionally omitted per contract §12 Known Delta). Jetstream's `ui_element::icon(name)` resolves names somewhere — confirm whether a registry context is even needed before implementing.
- accepted: no visual output, no tokens.

## Specimen parity

- Svelte covers: registry provision + descendant Icon resolution (129 lines — demonstrates icons rendering under a provider).
- GPUI covers: 35-line specimen — but since the provider is a pass-through, it cannot demonstrate registry scoping; verify it actually shows provided icons resolving.
- Jetstream covers: **none** — no specimen, matching the missing component.

## Notes

- This component produces no visual output and uses no tokens, so there are zero token-violation risks. Parity here is purely functional: does the provider make a registry available to descendant Icons.
- Both Rust targets effectively rely on a global/implicit registry rather than scoped context. Contract §10/§12 explicitly allow this — so the correct resolution may be to (a) update the contract to state Rust targets use a global registry, and (b) make GPUI's IconProvider either wire context or be documented as inert. The Jetstream gap is "no component and no documented stance", which is the real open item.
- consv=gap is driven solely by the wrong function name (`setIconRegistry` vs `setIconSet`) in contract §9.
