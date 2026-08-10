# Public Front Door Reconciliation

Date: 2026-08-10

## Scope

Check the repository front door, contributor setup, support routes, and
changelog history against the actual public-release posture.

## Findings

- The repository and remote have no release tags, but the changelog linked the
  internal `0.1.0` source baseline to a nonexistent GitHub release.
- Setup documentation required Effigy without listing it as a prerequisite.
- CI pins Bun 1.3.14, while the root package manifest and contributor setup did
  not.
- The security contact was clear, but the front door had no general support
  route.

## Changes

- Described `0.1.0` as the source/version baseline it was and replaced broken
  release links with the baseline commit comparison and local release notes.
- Added Effigy and exact Bun requirements to contributor setup.
- Declared `bun@1.3.14` in the root package manifest.
- Added GitHub issue and email support routes without weakening the private
  security-reporting boundary.

## Validation

- `effigy docs:check`
- `effigy release status`
- `bun install --frozen-lockfile`
- `git diff --check`
