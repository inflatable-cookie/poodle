# Public Repository Security Hygiene

Poodle remains `strict-ready`. The public tree now has one repeatable gate for
credential hygiene, dependency advisories, and dependency-source policy.

## Findings

- The current tracked tree and full 1,555-commit patch history contain no
  private-key headers or major provider-token patterns.
- No credential-like files, first-party install lifecycle hooks, remote npm
  dependencies, or active Cargo Git dependencies are tracked.
- `bun audit` reports no known vulnerabilities, and `bun pm untrusted` reports
  no blocked dependency lifecycle scripts.
- Shared Rust, GPUI adapter, GPUI node-backend, and Jetstream adapter graphs
  contain no vulnerability, unsoundness, yanked-crate, or unapproved-source
  findings.
- GPUI 0.2.2 carries ten unmaintained transitive crate versions. RustSec marks
  these as maintenance notices rather than vulnerabilities, and no safe
  Poodle-side upgrade exists. Crates.io still reports GPUI 0.2.2 as the latest
  published release.
- `SECURITY.md` provides a private reporting address, requested report detail,
  disclosure guidance, and supported-version posture.

## Repaired

- Added `effigy audit:security` to scan repository files, npm advisories, Rust
  advisories, and Rust dependency sources.
- Added `deny.toml`: vulnerabilities and unsoundness fail at any depth;
  unmaintained direct dependencies fail; crates.io and local paths are the only
  accepted Rust sources.
- Expanded `.gitignore` to cover environment variants, private keys,
  keystores, and signing bundles while preserving explicit environment example
  files.
- Documented the gate for contributors. The unavailable bounded history
  scanner remains recorded in `PAPERCUTS.md`; this batch completed a one-time
  full-history scan directly.

## Validated

- `effigy audit:security`
- `effigy docs:check`
- `effigy doctor`
- `git diff --check`
