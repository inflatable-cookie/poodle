# g15.069 — Compli Me Tagged Underlay and Poodle 0.2.2 Closeout

Date: 2026-08-25
Status: complete
PRs: [Compli Me #1](https://github.com/double-dip/compli-me/pull/1),
[Compli Me #2](https://github.com/double-dip/compli-me/pull/2),
[Compli Me #3](https://github.com/double-dip/compli-me/pull/3)
Final merge: `db5741f63dfb1cc82d9b49436370edfe66366bb2`

## Result

Admin and Front use exact registry Poodle core/Svelte 0.2.2. Compli Me's active
Underlay graph has advanced to release `v0.9.4`: Rust and the Admin, Front, UI,
and API-client web packages all name that tag. Web and Rust locks resolve its
peeled commit `7004af5b3461b6c89a7faa646575ff69576c73b8`.

PR 3 began as the final `v0.9.2` web-manifest tag correction. Main advanced to
`v0.9.4` while it was under review, so the reviewed branch was rebased without
downgrading the application. The final eight-line diff replaces the peeled
`7004af5b` source in four manifests and four Bun requested-spec entries with
`#v0.9.4`; resolved code is unchanged. No Rust or application source changed.

## Validation

Independent closeout passed:

- frozen Bun installs in API Client, UI, Admin, and Front;
- `effigy api-client/validate`;
- `effigy ui/validate`;
- `effigy admin/validate`;
- `effigy front/validate`;
- `git diff --check`.

The existing Vite native-config and Lightning CSS `:global` warnings remain
visible but do not fail the package boards and were not introduced here.

## Continuation

Compli Me is complete. Continue the coupled Underlay wave only through
Acowtancy's bounded `g15.068` correction; the remaining published handoffs are
`g15.079` and product cards `073`–`078`.
