# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

<!-- Keep entries short. Append newest entries at the top. Do not include secrets. -->

- 2026-08-11 — A playbook-style detached `nohup omp -p` launch exited
  immediately with an empty log and untouched worktree under the Codex exec
  harness. Document a harness-safe detach or persistent-session recipe.
  Affects worker orchestration from agent threads.

- 2026-08-10 — `effigy doctor` runs a failing health check that reformats
  generated Rust token artifacts and unrelated Rust tests, leaving a dirty
  worktree. Make doctor diagnostics read-only or isolate formatter output.
  Affects routine repo-health checks.

- 2026-08-10 — `effigy react:build` runs Vite transpilation without a TypeScript
  check; direct `tsc` finds pre-existing `AgentChatStatus` errors the build
  reports as green. Add a React consumer type-check selector and CI lane.
  Affects React parity work.

- 2026-08-10 — A combined `git log -G` secret-pattern history scan ran for
  more than 90 seconds without progress or a result. Add a bounded, cached
  tracked-tree and history secret scanner. Affects public-release audits.

- 2026-08-10 — `effigy tasks` advertises a `distribution` built-in, but
  `effigy distribution --help` resolves it as an undefined catalog task and
  the general help omits it. Align discovery and command routing. Affects
  package-publication audits.

- 2026-08-10 — `effigy graph explore` can hang after a successful index refresh
  without progress or timeout output. Add a bounded query timeout and a clear
  fallback diagnostic. Affects agent code-navigation workflows.

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
