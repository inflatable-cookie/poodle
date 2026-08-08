# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

<!-- Keep entries short. Append newest entries at the top. Do not include secrets. -->

- 2026-08-08 — `poodle_render::calendar` reads `SystemTime::now()` directly to
  draw the today border, so every pixel baseline containing a Calendar expires
  at midnight. Both native gates now skip it, which costs the whole specimen's
  coverage for one border. Svelte reads the clock the same way
  (`todayIsoDate()`), so making `today` injectable is a contract decision
  across all targets, not a local fix.

- 2026-08-06 — `effigy doctor` reports the repo's `isolation` manifest key as
  unsupported, so routine health checks cannot go green on the checked-in
  manifest. Align the manifest schema or update Effigy's accepted config keys.

- 2026-08-06 — `effigy graph explore` can panic at `snippets.rs:210` while
  truncating a Unicode-bearing result. **Fixed in the effigy working tree**
  (`crates/effigy-codegraph/src/query/snippets.rs`) on 2026-08-08: the cut point
  now walks back to a char boundary, and the ellipsis is only appended when it
  fits inside the limit. Two tests added; 55 crate tests and clippy pass.
  Uncommitted there, and the installed binary at `.local-install/bin/effigy` is
  **not** rebuilt — this stays open until effigy releases and the install is
  refreshed.
