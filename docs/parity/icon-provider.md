<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok | web uses provider-first IconSets plus a scoped default Lucide set; native hosts own name-to-asset resolution -->
# Parity: IconProvider

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/icon-provider.md`
- Svelte (authoritative): `packages/svelte/components/src/IconProvider.svelte`
- GPUI: `packages/gpui/node-backend/src/lib.rs` (`NodeKind::Icon` host asset path)
- Jetstream: host renderer consumes `poodle_node::NodeKind::Icon` names
- Specimens: svelte `packages/svelte/preview/src/specimens/IconProviderSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/icon_provider.rs` · jetstream `packages/jetstream/preview/src/specimens/icon_provider.rs`

## Contract ↔ Svelte

IconProvider is a pure context boundary with no DOM output, no tokens, no states. Props and anatomy match. The one documentation divergence is reconciled:

- [x] FIXED Contract §9 said "Uses `setIconRegistry(registry)`" / "Import `setIconRegistry`". Svelte actually uses `setIconSet` (`IconProvider.svelte:5,22`; defined in `icon-registry.ts`). Contract §9 repointed to `setIconSet` (and `import { setIconSet } from './icon-registry'`).
- [x] FIXED Svelte prop is `icons: IconSet` (already matched contract §3).
  Added §9 notes for the empty-set seed + `$effect` sync, provider-first
  resolution, the scoped default Lucide set, and loud missing-name behavior.
- [x] FIXED Removed the catalogue-wide namespace fallback. Svelte and React now
  resolve application sets before the same 54-icon default Lucide set.

## GPUI boundary

- accepted: GPUI has no npm dependency or catalogue import. The node backend
  maps `NodeKind::Icon { name, size }` to the host-owned
  `assets/icons/{name}.svg` path. Icon names cross the renderer boundary; the
  host owns asset registration.
- accepted: no visual output, no tokens — nothing to resolve (contract §8).
- accepted: no ARIA (no DOM element by design).

## Jetstream boundary

- accepted: Jetstream has no npm dependency or catalogue import. Poodle emits
  `NodeKind::Icon` names and the host renderer owns glyph resolution. No web
  icon package crosses this boundary.
- accepted: no visual output, no tokens.

## Specimen parity

- Svelte covers: registry provision + descendant Icon resolution (129 lines — demonstrates icons rendering under a provider).
- GPUI covers: **done (as far as the contract allows)** — `icon_provider.rs` wraps three real `Icon`s (`search`/`calendar`/`clock`) inside a real `IconProvider` boundary, labeled with an `Eyebrow`, plus an honest note that GPUI uses a shared/global icon registry today so the provider is a no-visual compatibility boundary. Contract §4/§7 say IconProvider produces **no visual output** — there is no renderable surface to demonstrate beyond "icons resolve under the provider", which this specimen does. No additional groups are meaningful; adding more would be invented visuals. Left as-is.
- Jetstream covers: real icons inside a labeled provider scope plus the explicit
  global-registry note. The provider remains non-visual by contract.

Both native specimens cover the full observable boundary: icon names resolve
inside a no-visual provider scope through host-owned registries.

## Notes

- This component produces no visual output and uses no tokens, so there are zero token-violation risks. Parity here is purely functional: does the provider make a registry available to descendant Icons.
- Both Rust targets use host-owned asset/glyph resolution rather than an npm
  icon set. The web catalogue deletion does not reach either runtime.
