<!-- parity consv=? gpui=? jetstream=? specimen=? -->
# Parity: <ComponentName>

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/<c>.md`
- Svelte (authoritative): `packages/svelte/components/src/<C>.svelte`
- GPUI: `packages/gpui/components/src/{composites|primitives}/<c>.rs`
- Jetstream: `packages/jetstream/components/src/<c>.rs`
- Specimens: svelte `…Specimen.svelte` · gpui `specimens/<c>.rs` · jetstream `specimens/<c>.rs`

## Contract ↔ Svelte

Prop/anatomy/state divergences. For each: what differs, which side is right
(Svelte unless it's missing contract-specified functionality), and the action.

- …

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] …
- accepted: no ARIA (gpui has no accessibility API)

## Jetstream gap (vs Svelte + contract)

- [ ] …

## Specimen parity

- Svelte covers: …
- GPUI covers: … — missing: …
- Jetstream covers: … — missing: …

## Notes

Accepted deltas, ambiguities, follow-ups.
