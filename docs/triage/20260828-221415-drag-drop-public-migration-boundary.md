# Drag-And-Drop Public Migration Boundary

Status: open — operator decision required before component or DockRegion migration
Captured: 2026-08-28
Source: g16 post-audit runway compilation

## Finding

The existing bespoke drag machinery is not wholly private. The root
`@inflatable-cookie/poodle-core` export surface includes the Tabs reorder
helpers from `tabs-reorder.ts` and the complete DockRegion external-drag
controller/types from `dock-external-drag.ts`. Svelte and React also re-export
the DockRegion types.

Architecture 011/spec 069 require component-local controllers and global side
channels to disappear after their replacement passes. Keeping the old exports
as aliases or silent adapters would violate the pre-1.0 no-shim rule. Removing
or replacing them is a breaking public migration and cannot be inferred from
approval of the new substrate alone.

## Decision Needed

Before the Tabs/EditableList or DockRegion migration cards become ready,
choose one explicit pre-1.0 boundary:

1. remove the obsolete root exports and migrate in-repo callers in the same
   release tranche; or
2. retain any helper that remains a genuine first-class API, give it a defined
   role on top of the new substrate, and do not describe it as compatibility.

The recommendation is option 1 for the event-shaped Tabs helpers. Reassess the
DockRegion types against the new host bridge before deciding whether any of
their concepts remain first-class under new names.

## Guardrail

Do not add aliases, deprecation wrappers, dual controllers, or silent fallback
from the new substrate to the old global session. The semantic-kernel and
custom-surface foundation cards may proceed because they do not remove these
exports.

## Promotion Route

Keep this note open through the first semantic-kernel card. Resolve it before
marking the first component migration or cross-window DockRegion card ready,
then record the choice in those roadmap cards and the relevant public docs.
