# g16.014 — Acowtancy Release Boundary Closeout

Date: 2026-08-25
Status: complete
PRs: [Acowtancy #54](https://github.com/acowtancy/market/pull/54),
[Acowtancy #55](https://github.com/acowtancy/market/pull/55),
[Acowtancy #56](https://github.com/acowtancy/market/pull/56)
Final merge: `1ab977f8e1e0edbcb032073ec290dadb676388ad`

## Result

Cream and Dairy use exact registry Poodle core/Svelte 0.2.2, Froyo advertises
the 0.2.x peer line, and the active web/Rust graph resolves Underlay tag
`v0.9.2` at `ddba26400f480638829917cf72eecc62be4b978d`.

PR 56 removed the remaining SvelteKit, Vite, and TypeScript mappings that
compiled application code from the sibling Underlay checkout despite correct
manifests and locks. Cream and Dairy's config generators now use Underlay's
published config-stack export. Explicit sibling references remain only in
workspace conformance and guardrail QA.

## Validation

The corrective worker recorded green frozen workspace preparation, Cream,
Dairy, Froyo, and Cattle Grid validation, plus clean diff checks. The root
validation/QA failure on Farmyard API route-name audit reproduced unchanged
from baseline; the branch touched none of that scan's source paths. No lock
change or application exception was introduced.

## Continuation

The five-repository coupled Underlay product wave is complete. Continue through
the Underlay Reference follow-up and the independent Longhorn product lanes.
