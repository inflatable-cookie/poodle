# CommandPalette

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `CommandPalette`
- Layer: `workstation`
- Summary: a richer workstation command-discovery surface built on top of the shell-level palette posture
- In scope: grouped action results, active-result movement, host-owned ordered ranking, empty/loading/error result posture, and command commit semantics
- Out of scope: command registry persistence, fuzzy-search algorithm internals, telemetry, or app-specific command namespaces

## 2. Accessibility

- the query field remains the primary focus target when the palette opens
- focus stays trapped inside the palette while it is open
- focus restores to the invoking control when the palette closes
- active-result movement must stay keyboard reachable and visually explicit
- `Home` and `End` move to result boundaries when command results are present
- loading, no-results, empty, and error states must remain textual and distinct
- result and active-command changes should remain announceable without moving focus
- GPUI-native accessibility mapping notes: GPUI must preserve grouped results, active-result meaning, and command commit semantics instead of treating the palette as a generic modal list

## 3. Next Task

Use `CommandPalette` for workstation command launchers and keep registry ownership, ranking heuristics, and domain-specific commands outside the shared contract.
