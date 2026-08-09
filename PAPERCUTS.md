# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

<!-- Keep entries short. Append newest entries at the top. Do not include secrets. -->

- 2026-08-10 — `cargo fmt --manifest-path` follows Poodle's linked Rust
  workspaces and reformats unrelated crates, creating broad release-sweep
  churn. Add an Effigy formatter that accepts an explicit changed-file set.

- 2026-08-09 — Vite JSON named exports omit kebab-case `lucide-static`
  keys, so bundle-safe imports cannot express most icon names. This blocks a
  plain `createIconSet({ ... })` helper over `icon-nodes.json`; add a build-time
  extractor or a per-icon node package surface. Affects web icon adoption.

- 2026-08-06 — `effigy doctor` reports the repo's `isolation` manifest key as
  unsupported, so routine health checks cannot go green on the checked-in
  manifest. Align the manifest schema or update Effigy's accepted config keys.
