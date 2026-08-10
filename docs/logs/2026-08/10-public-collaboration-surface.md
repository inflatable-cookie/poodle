# Public Collaboration Surface

Date: 2026-08-10

## Scope

Check whether public issue and pull-request entry points enforce Poodle's
documented contribution and security boundaries.

## Findings

- The repository had no issue forms or pull-request template.
- Public issue creation did not warn reporters away from disclosing security
  vulnerabilities.
- Pull requests were not prompted for contract ownership, affected runtimes,
  cross-runtime differences, packed-package proof, or release impact.

## Changes

- Added structured bug and feature-request forms.
- Routed security reports to the private policy and general questions to the
  project contact.
- Added a compact pull-request template aligned with `CONTRIBUTING.md`.

## Validation

- Parsed all issue configuration as YAML.
- `effigy docs:check`
- `git diff --check`
