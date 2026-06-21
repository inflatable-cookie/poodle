<!-- parity consv=ok gpui=2 jetstream=1 specimen=gap --><!-- pass: GPUI specimen done; Jetstream pending engine recovery -->
# Parity: ErrorBoundary

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/error-boundary.md`
- Svelte (authoritative): `packages/svelte/components/src/ErrorBoundary.svelte`
- GPUI: `packages/gpui/components/src/composites/error_boundary.rs`
- Jetstream: **absent** (no `error_boundary.rs` in `packages/jetstream/components/src/`)
- Specimens: svelte `packages/svelte/preview/src/specimens/ErrorBoundarySpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/error_boundary_specimen.rs` · jetstream **absent**

## Contract ↔ Svelte

Svelte implements the full contract — props `children` (required Snippet), `title` (default `"Something went wrong"`), `retryLabel` (default `"Try again"`) (`ErrorBoundary.svelte:7-17`). Both states (normal / error) present: `svelte:boundary onerror` catches child errors (`:37`), error branch renders `EmptyState` (title + `currentError.message`) with a secondary retry Button whose `onclick={reset}` clears `currentError` (`:30-34`). Composition (EmptyState + Button), internal error handling (no public onError/onReset), and a11y (delegated to EmptyState; no `role="alert"` required) all match. consv=ok.

## GPUI gap (vs Svelte + contract)

- [ ] Retry is non-functional — error branch emits a static `RemediationAction::new("retry", retry_label)` (`error_boundary.rs:40-43`) with no reset closure; clicking does not clear the error. Svelte's retry actually resets `currentError`.
- [ ] Error state is forced via `with_error_message` (`error_boundary.rs:36`), not caught — GPUI cannot catch render panics, so the boundary is a presentational shell. (Accepted runtime limit, but the retry should still be wireable; flag as a follow-up.)
- accepted: no error-catching runtime (GPUI has no `svelte:boundary` equivalent) — boundary is presentational only.
- Component file is literal-free (grep found zero px/color literals). The one `gap(px(16.0))` is in the specimen (`error_boundary_specimen.rs:10`), not the component.

## Jetstream gap (vs Svelte + contract)

- [x] DONE: `js_error_boundary(spec, theme, child)` created (`error_boundary.rs`), composing `js_empty_state` (title + message + retry action) on error, else the child. Registered in lib.rs, probe-tested.
- [ ] Add the Jetstream specimen (error + non-error states).

## Specimen parity

- Svelte covers: Normal children (stable content), Caught render error (interactive "Throw again" button + real throw + working reset to "Recovered child content."). Both states, interactive.
- GPUI covers: labeled groups — "Normal children" (real `Surface` border=subtle + padding=md wrapping a `Text` child, passed through), "Caught render error" (`EmptyState` fallback with custom title + retry, forced via `with_error_message`), "Default fallback" (default title/retry from the spec). **GPUI specimen complete** — both contract states plus a default-title variant, real EmptyState fallback (icon + title + message + retry action), no fakes. Error remains props-driven (GPUI cannot catch render panics) and retry is a static action (accepted runtime delta) — statically rendered, no real throw / working reset.
- Jetstream covers: **nothing** (no specimen).

## Notes

- This is the lightest component in the set: no token-usage section in the contract (all visuals delegate to EmptyState + Button), so there are no token violations to flag — GPUI's component file is literal-free.
- The two real gaps are (1) GPUI retry is a dead label, and (2) Jetstream has no implementation at all. The Jetstream gap is the headline.
- Because GPUI/Jetstream cannot catch runtime errors, the "error" state is necessarily props-driven (`error_message`) rather than caught — this is an accepted runtime delta, but the retry action should still reset the props-driven error in a host-wired way.
