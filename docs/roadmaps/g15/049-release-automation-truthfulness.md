# g15.049 — Release Automation Truthfulness

Status: **blocked — explicit operator approval required for workflow edits**
Depends on: none; may run in parallel once approved
Unblocks: `g15.050`
Governing refs: `../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../contracts/001-working-rules.md`, repository `AGENTS.md`,
`013-v020-release-certification.md`

## Problem

The release workflow is broadly sound, but the pre-tag native workflow still
calls the deleted `packages/gpui/components/Cargo.toml`. It therefore cannot
provide the native evidence its comments promise. Separately,
`effigy release gates` currently reports success with zero configured gates;
that result is vacuous and must not be cited as release proof.

## Goal

Make every advertised release/pre-tag path execute current Effigy-owned gates
and fail closed. Keep publication human-dispatched and preserve the current
decision about which npm packages actually publish.

## Scope Envelope

- Repair `ci-native.yml` to invoke current Effigy selectors rather than stale
  package paths, while keeping it manual and headless.
- Decide explicitly whether `effigy release gates` is configured to the real
  read-only board or removed from release guidance. Zero configured gates may
  never be described as green certification.
- Reconcile release workflow comments, tag examples, package list, packed
  artifact verification, and native pre-tag instructions with v0.2.0.
- Use current action/security guidance and keep OIDC trusted publishing. Do not
  add automatic tag publication or long-lived tokens.
- Validate workflows without publishing, tagging, or mutating a release.

## Acceptance Envelope

- [ ] No workflow references a deleted package or raw command that duplicates
      an Effigy selector.
- [ ] The manual native workflow exercises the same supported headless native
      board named by release documentation.
- [ ] A read-only release-gate command is non-vacuous or is explicitly excluded
      from the proof.
- [ ] Dry-run release evidence packs exactly the intended npm artifacts and
      preserves the no-publish default.
- [ ] No release, tag, or registry mutation occurs.

## Stop Conditions

- The operator has not explicitly approved `.github/workflows/` edits.
- A required release claim cannot be expressed through a supported Effigy
  selector.
- The fix would silently change the published package set.
