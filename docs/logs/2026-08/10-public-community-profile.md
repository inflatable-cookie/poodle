# Public Community Profile

Date: 2026-08-10

## Scope

Check Poodle's repository front door through GitHub's community-profile API.

## Findings

- GitHub reported 85% community-profile health.
- README, MIT license, contribution guide, and pull-request template were
  present.
- Poodle had no code of conduct.
- External repository settings still show private visibility, no homepage, and
  the short description `Shared contract UI components`. Those settings remain
  publication-operator work.

## Changes

- Added a concise project-owned code of conduct covering expected behavior,
  unacceptable conduct, private reporting, scope, and proportionate
  enforcement.
- Linked the policy from the README and contributor guide.

## Validation

- `effigy docs:check`
- `git diff --check`
