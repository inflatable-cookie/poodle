# g13.013 Data, Composite, And Workstation Migration

Status: closed — superseded by the `g13.008` **revise** verdict
(`docs/roadmaps/g13/pilot-verdict-evidence.md` §7–8). This milestone describes
family-by-family migration to a generative model the verdict declines. It is
retained as evidence and is **not executable**. The replacement runway is
`g13.017`–`g13.020`.
Owner: Poodle core
Depends on: `g13.009`, `g13.010`, `g13.011`, `g13.012`

## Objective

Move reusable data, media, agent, form-shell, and workstation compositions to
the shared definition and scene model.

## Scope

- Tables, lists, detail views, filters, media, agent surfaces, application
  chrome, docks, panels, and reusable workstation shells.
- Keep routing, fetching, persistence, authorization, product workflows, and
  DAW-specific models in host applications.
- Use explicit runtime extensions for compositions beyond scene IR.

## Acceptance

- Component ownership remains inside Poodle guardrails.
- Slots, callbacks, layout intent, and accessibility retain contract parity.
- Old composition copies are removed only after consumer and preview proof.

## Next

`g13.014` removes remaining native implementation forks.
