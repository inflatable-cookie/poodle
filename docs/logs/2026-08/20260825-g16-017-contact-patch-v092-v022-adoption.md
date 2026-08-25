# g16.017 — Contact Patch Underlay 0.9.2 and Poodle 0.2.2 Adoption

Date: 2026-08-25
Status: complete
PR: [Contact Patch #1](https://github.com/contact-patch/contact-patch/pull/1)
Merge: `c497547bfb244f53b1f68f2f16e292103e9e756f`

## Result

Contact Patch's active application graph no longer resolves Underlay or Poodle
from sibling source trees. Admin, Front, UI, and client use Underlay tag
`v0.9.2` at `ddba26400f480638829917cf72eecc62be4b978d`. Admin and Front use
exact registry Poodle 0.2.2. The API's 26 Underlay crates resolve that same Git
revision.

The migration removed the client TypeScript sibling path map, the shared
config generator's sibling source import, Admin/Front Poodle overrides, and
all active web and Rust sibling dependency paths. Explicit sibling mounts and
scripts remain only for Effigy workspace QA.

## Validation

Independent review passed:

- `effigy health`
- `effigy cp-admin/validate`
- `effigy cp-front/validate`
- `effigy cp-ui/validate`
- `git diff --check`

All four Bun locks resolve one Underlay revision and registry Poodle 0.2.2.
The Cargo lock changes exactly the 26 Underlay packages from local 0.8.0 to
tagged 0.9.2; no unrelated Cargo package changed. The large Bun lock deletions
remove development dependencies inherited from the former local Underlay
checkout rather than upgrading unrelated application packages.

Database-backed API tests retain their no-Postgres baseline. One Admin
contract test searches the wrong client types file. Both are recorded in the
consumer's `PAPERCUTS.md`; no validation exception was added.

## Continuation

Contact Patch is complete. Continue the coupled product wave through the
remaining Acowtancy, Compli Me, Composer, and Songsprout lanes. Re-review the
requested Composer and Songsprout lock repairs before merge; keep each product
repository independent.
